use std::path::Path;
use std::time::Duration;

use crate::paths::ProjectPaths;

/// Collected fields for lock-contention stderr diagnostics (FR-1.8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockContentionReport {
    pub pid_file_display: String,
    pub recorded_pid: Option<u32>,
    pub recorded_pid_alive: Option<bool>,
    pub recorded_comm: String,
    pub lock_holder_pid: Option<u32>,
    pub lock_holder_comm: String,
    pub socket_display: String,
    pub elapsed: Duration,
}

impl LockContentionReport {
    pub fn gather(paths: &ProjectPaths, elapsed: Duration) -> Self {
        let recorded_pid = read_pid_file(&paths.pid_file);
        let recorded_pid_alive = recorded_pid.map(is_process_alive);
        let recorded_comm = recorded_pid
            .map(process_comm)
            .unwrap_or_else(|| "not running".into());
        let (lock_holder_pid, lock_holder_comm) = lock_holder_info(&paths.pid_file);
        Self {
            pid_file_display: display_path(&paths.pid_file),
            recorded_pid,
            recorded_pid_alive,
            recorded_comm,
            lock_holder_pid,
            lock_holder_comm,
            socket_display: display_path(&paths.socket_path),
            elapsed,
        }
    }
}

/// Format the illustrative diagnostic block from [design.md](https://github.com/npetzall/openpfe).
pub fn format_lock_contention(report: &LockContentionReport) -> String {
    let mut lines = vec![
        "error: openpfe server for this project did not respond on IPC".into(),
        format!(
            "  pid file: {} (locked by another process)",
            report.pid_file_display
        ),
    ];
    match (report.recorded_pid, report.recorded_pid_alive) {
        (Some(pid), Some(true)) => {
            lines.push(format!(
                "  recorded pid: {pid} (alive, comm: {})",
                report.recorded_comm
            ));
        }
        (Some(pid), Some(false)) => {
            lines.push(format!("  recorded pid: {pid} (not running)"));
        }
        (Some(pid), None) => lines.push(format!("  recorded pid: {pid}")),
        (None, _) => lines.push("  recorded pid: (missing or invalid)".into()),
    }
    if let Some(holder) = report.lock_holder_pid {
        if report.lock_holder_comm.is_empty() {
            lines.push(format!("  lock holder pid: {holder}"));
        } else {
            lines.push(format!(
                "  lock holder pid: {holder} (comm: {})",
                report.lock_holder_comm
            ));
        }
    }
    lines.push(format!(
        "  socket: {} (no echo after {:.1}s)",
        report.socket_display,
        report.elapsed.as_secs_f64()
    ));
    lines.push(
        "hint: wait and retry, run `openpfe stop`, or remove stale socket if no openpfe process is running"
            .into(),
    );
    lines.join("\n")
}

fn display_path(path: &Path) -> String {
    path.strip_prefix(std::env::current_dir().unwrap_or_default())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| path.display().to_string())
}

fn read_pid_file(path: &Path) -> Option<u32> {
    let contents = std::fs::read_to_string(path).ok()?;
    contents.trim().parse().ok()
}

fn is_process_alive(pid: u32) -> bool {
    #[cfg(unix)]
    {
        unsafe { libc::kill(pid as i32, 0) == 0 }
    }
    #[cfg(not(unix))]
    {
        let _ = pid;
        false
    }
}

fn process_comm(pid: u32) -> String {
    #[cfg(target_os = "linux")]
    {
        let path = format!("/proc/{pid}/comm");
        return std::fs::read_to_string(path)
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "unknown".into());
    }
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let output = Command::new("ps")
            .args(["-p", &pid.to_string(), "-o", "comm="])
            .output();
        match output {
            Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
            _ => "unknown".into(),
        }
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        let _ = pid;
        "unknown".into()
    }
}

fn lock_holder_info(pid_file: &Path) -> (Option<u32>, String) {
    #[cfg(unix)]
    {
        use std::process::Command;
        let output = Command::new("lsof").arg(pid_file).output();
        if let Ok(o) = output
            && o.status.success()
            && let Some((pid, comm)) = parse_lsof_output(&String::from_utf8_lossy(&o.stdout))
        {
            return (Some(pid), comm);
        }
    }
    (None, String::new())
}

#[cfg(unix)]
fn parse_lsof_output(stdout: &str) -> Option<(u32, String)> {
    let line = stdout.lines().nth(1)?;
    let cols: Vec<&str> = line.split_whitespace().collect();
    let pid = cols.get(1)?.parse().ok()?;
    let comm = cols.first().unwrap_or(&"").trim_matches('"').to_string();
    Some((pid, comm))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn format_includes_hint_and_socket() {
        let report = LockContentionReport {
            pid_file_display: ".openpfe/server/pid".into(),
            recorded_pid: Some(12345),
            recorded_pid_alive: Some(true),
            recorded_comm: "openpfe".into(),
            lock_holder_pid: Some(12345),
            lock_holder_comm: "openpfe".into(),
            socket_display: ".openpfe/server/socket".into(),
            elapsed: Duration::from_secs(5),
        };
        let text = format_lock_contention(&report);
        assert!(text.contains("did not respond on IPC"));
        assert!(text.contains("recorded pid: 12345"));
        assert!(text.contains("lock holder pid: 12345"));
        assert!(text.contains("no echo after 5.0s"));
        assert!(text.contains("openpfe stop"));
    }
}
