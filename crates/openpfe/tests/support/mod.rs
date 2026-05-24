//! Shared harness for `openpfe` integration tests.

#![allow(dead_code)]

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use fd_lock::RwLock;
use openpfe::adapters::{IpcProjectControl, ReExecSpawn};
use openpfe::client::{run_default, wait_for_echo};
use openpfe::error::ClientError;
use openpfe::paths::ProjectPaths;
use openpfe::ports::{ProjectControl, ServerSpawn};
use openpfe_server::{ServerOptions, run_server_with_opts};
use tempfile::TempDir;
use tokio::time::{sleep, timeout};

pub struct TestProject {
    pub _dir: TempDir,
    pub paths: ProjectPaths,
}

impl TestProject {
    pub fn new() -> Self {
        let dir = tempfile::tempdir().expect("tempdir");
        setup_layout(dir.path());
        let paths = ProjectPaths::from_root(dir.path()).expect("paths");
        Self { _dir: dir, paths }
    }

    pub fn chdir(&self) {
        std::env::set_current_dir(self._dir.path()).expect("chdir");
    }
}

pub fn setup_layout(root: &Path) {
    std::fs::create_dir_all(root.join(".openpfe").join("server")).expect("server dir");
}

pub async fn start_background_server(
    foreground: bool,
) -> tokio::task::JoinHandle<Result<(), openpfe_server::ServerError>> {
    let opts = ServerOptions {
        foreground,
        shutdown_timeout: Duration::from_secs(1),
    };
    tokio::spawn(async move { run_server_with_opts(opts).await })
}

pub async fn wait_for_echo_ready(paths: &ProjectPaths, secs: u64) {
    let control = IpcProjectControl::from_paths(paths);
    timeout(
        Duration::from_secs(secs),
        wait_for_echo(&control, Duration::from_secs(secs), false),
    )
    .await
    .expect("echo within timeout")
    .expect("server ready");
}

/// [`ServerSpawn`] wrapper that counts spawn attempts (integration tests).
pub struct CountingSpawn {
    inner: ReExecSpawn,
    pub count: Arc<AtomicUsize>,
}

impl CountingSpawn {
    pub fn new() -> Result<Self, ClientError> {
        Ok(Self {
            inner: ReExecSpawn::current_exe()?,
            count: Arc::new(AtomicUsize::new(0)),
        })
    }

    pub fn spawns(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }
}

impl ServerSpawn for CountingSpawn {
    fn spawn_detached_server(&self) -> Result<(), ClientError> {
        self.count.fetch_add(1, Ordering::SeqCst);
        self.inner.spawn_detached_server()
    }
}

pub async fn run_default_no_browser(
    paths: &ProjectPaths,
    control: &IpcProjectControl,
    spawn: &dyn ServerSpawn,
) -> Result<(), ClientError> {
    run_default(paths, control, spawn, Duration::from_secs(5), false, true).await
}

pub async fn stop_server(control: &IpcProjectControl) {
    let _ = control.shutdown().await;
}

/// Hold an exclusive flock on `pid` for the lifetime of the returned guard.
pub fn hold_pid_lock(paths: &ProjectPaths) -> fd_lock::RwLockWriteGuard<'static, std::fs::File> {
    use std::fs::OpenOptions;

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&paths.pid_file)
        .expect("open pid");
    let rw: &'static mut RwLock<std::fs::File> = Box::leak(Box::new(RwLock::new(file)));
    rw.try_write().expect("exclusive pid flock")
}

pub async fn pause(ms: u64) {
    sleep(Duration::from_millis(ms)).await;
}
