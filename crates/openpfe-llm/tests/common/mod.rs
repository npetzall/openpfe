use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;

use sha2::{Digest, Sha256};
use tempfile::TempDir;

pub struct TestDirs {
    pub _project: TempDir,
    pub _home: TempDir,
    pub project: PathBuf,
    pub home: PathBuf,
}

impl TestDirs {
    pub fn new() -> Self {
        let project = tempfile::tempdir().expect("project tempdir");
        let home = tempfile::tempdir().expect("home tempdir");
        Self {
            project: project.path().to_path_buf(),
            home: home.path().to_path_buf(),
            _project: project,
            _home: home,
        }
    }
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

/// Minimal HTTP/1.1 server; returns `http://127.0.0.1:PORT/path` (test-only http allowance).
pub fn spawn_http_server(body: Vec<u8>) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock server");
    let addr = listener.local_addr().expect("local addr");
    thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut buf = [0_u8; 1024];
            let _ = stream.read(&mut buf);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            );
            let _ = stream.write_all(response.as_bytes());
            let _ = stream.write_all(&body);
        }
    });
    format!("http://127.0.0.1:{}/model.gguf", addr.port())
}

pub fn wait_for<F: FnMut() -> bool>(mut f: F, timeout: Duration) {
    let start = std::time::Instant::now();
    while start.elapsed() < timeout {
        if f() {
            return;
        }
        thread::sleep(Duration::from_millis(20));
    }
    panic!("condition not met within {:?}", timeout);
}

static BACKEND: OnceLock<()> = OnceLock::new();

/// Serialize tests that initialize the llama backend (global init).
pub fn llama_test_lock() -> std::sync::MutexGuard<'static, ()> {
    BACKEND.get_or_init(|| ());
    static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    LOCK.lock().expect("llama test lock")
}

pub fn touch_file(path: &Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent");
    }
    std::fs::write(path, contents).expect("write file");
}
