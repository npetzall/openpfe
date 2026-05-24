use std::time::{Duration, Instant};

use crate::diagnostics::{LockContentionReport, format_lock_contention};
use crate::error::ClientError;
use crate::paths::ProjectPaths;
use crate::ports::{EchoInfo, ProjectControl, ServerSpawn};

/// Backoff between echo retries when the pid lock is held: 50ms → 200ms → 500ms (repeat).
pub fn echo_backoff_steps() -> impl Iterator<Item = Duration> {
    [
        Duration::from_millis(50),
        Duration::from_millis(200),
        Duration::from_millis(500),
    ]
    .into_iter()
    .chain(std::iter::repeat(Duration::from_millis(500)))
}

/// Retry echo until success or `timeout` elapses (FR-1.7).
pub async fn wait_for_echo(
    control: &dyn ProjectControl,
    timeout: Duration,
    verbose: bool,
) -> Result<EchoInfo, ClientError> {
    let started = Instant::now();
    let deadline = started + timeout;
    let mut steps = echo_backoff_steps();
    while Instant::now() < deadline {
        match control.echo().await {
            Ok(info) => return Ok(info),
            Err(e) => {
                if verbose {
                    eprintln!("openpfe: echo retry: {e}");
                }
            }
        }
        let step = steps.next().unwrap_or(Duration::from_millis(500));
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            break;
        }
        let sleep_for = step.min(remaining);
        tokio::time::sleep(sleep_for).await;
    }
    Err(ClientError::EchoTimeout {
        elapsed: started.elapsed(),
    })
}

/// Ensure the project server responds to echo (spawn + lock-held wait when needed).
///
/// Echo-first; if the socket exists but echo fails, retry until timeout (lock-held wait).
pub async fn ensure_server(
    paths: &ProjectPaths,
    control: &dyn ProjectControl,
    spawn: &dyn ServerSpawn,
    timeout: Duration,
    verbose: bool,
) -> Result<EchoInfo, ClientError> {
    if let Ok(info) = control.echo().await {
        if verbose {
            eprintln!("openpfe: server already up");
        }
        return Ok(info);
    }

    if paths.socket_path.exists() {
        if verbose {
            eprintln!("openpfe: socket present, waiting for echo");
        }
        return wait_for_echo(control, timeout, verbose).await;
    }

    if verbose {
        eprintln!("openpfe: spawning detached server");
    }
    spawn.spawn_detached_server()?;
    wait_for_echo(control, timeout, verbose).await
}

/// Default command: ensure server, optionally open browser (FR-2).
pub async fn run_default(
    paths: &ProjectPaths,
    control: &dyn ProjectControl,
    spawn: &dyn ServerSpawn,
    timeout: Duration,
    verbose: bool,
    skip_browser: bool,
) -> Result<(), ClientError> {
    let info = ensure_server(paths, control, spawn, timeout, verbose).await?;
    if skip_browser {
        return Ok(());
    }
    match webbrowser::open(&info.http_base_url) {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("openpfe: warning: could not open browser: {e}");
            Ok(())
        }
    }
}

/// `openpfe stop` — idempotent when echo fails (FR-4.2).
pub async fn run_stop(control: &dyn ProjectControl) -> Result<(), ClientError> {
    match control.echo().await {
        Ok(_) => control.shutdown().await,
        Err(_) => {
            eprintln!("server not running");
            Ok(())
        }
    }
}

/// Exit code 1 with lock contention diagnostics on stderr.
pub fn fail_lock_contention(paths: &ProjectPaths, elapsed: Duration) -> ! {
    let report = LockContentionReport::gather(paths, elapsed);
    eprintln!("{}", format_lock_contention(&report));
    std::process::exit(1);
}

/// Map echo timeout to diagnostics exit (used when callers want FR-1.8 output).
pub fn exit_on_echo_timeout(paths: &ProjectPaths, err: ClientError) -> ! {
    match err {
        ClientError::EchoTimeout { elapsed } => fail_lock_contention(paths, elapsed),
        other => {
            eprintln!("error: {other}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use crate::adapters::mock::{MockProjectControl, MockServerSpawn};
    use crate::diagnostics::{LockContentionReport, format_lock_contention};
    use crate::paths::ProjectPaths;

    fn temp_paths() -> (tempfile::TempDir, ProjectPaths) {
        let dir = tempfile::tempdir().expect("tempdir");
        let openpfe = dir.path().join(".openpfe");
        std::fs::create_dir_all(openpfe.join("server")).expect("server dir");
        let paths = ProjectPaths::from_root(dir.path()).expect("paths");
        (dir, paths)
    }

    #[tokio::test]
    async fn wait_for_echo_succeeds_after_transient_failures() {
        let control = MockProjectControl::with_url("http://127.0.0.1:8080/");
        control.set_echo_failures_remaining(2);
        let info = wait_for_echo(&control, Duration::from_secs(2), false)
            .await
            .expect("echo");
        assert_eq!(info.http_base_url, "http://127.0.0.1:8080/");
    }

    #[tokio::test]
    async fn stop_is_idempotent_when_server_down() {
        let control = MockProjectControl::new();
        control.set_server_up(false);
        run_stop(&control).await.expect("stop ok");
    }

    #[tokio::test]
    async fn second_default_does_not_double_spawn() {
        let (_dir, paths) = temp_paths();
        let control = Arc::new(MockProjectControl::with_url("http://127.0.0.1:8080/"));
        let spawn = MockServerSpawn::new();
        run_default(
            &paths,
            control.as_ref(),
            &spawn,
            Duration::from_secs(2),
            false,
            true,
        )
        .await
        .expect("first default");
        assert_eq!(spawn.spawns(), 0);
        run_default(
            &paths,
            control.as_ref(),
            &spawn,
            Duration::from_secs(2),
            false,
            true,
        )
        .await
        .expect("second default");
        assert_eq!(spawn.spawns(), 0);
    }

    #[test]
    fn diagnostics_text_contains_required_fields() {
        let (_dir, paths) = temp_paths();
        let report = LockContentionReport::gather(&paths, Duration::from_secs(5));
        let text = format_lock_contention(&report);
        assert!(text.contains("socket:"));
        assert!(text.contains("hint:"));
    }
}
