use std::path::PathBuf;

use crate::error::ClientError;
use crate::ports::ServerSpawn;

/// Re-exec the current binary with hidden `--server` and detach on Unix.
#[derive(Debug, Clone)]
pub struct ReExecSpawn {
    exe: PathBuf,
}

impl ReExecSpawn {
    pub fn current_exe() -> Result<Self, ClientError> {
        Ok(Self {
            exe: std::env::current_exe()?,
        })
    }

    pub fn with_exe(exe: impl Into<PathBuf>) -> Self {
        Self { exe: exe.into() }
    }

    /// Resolve the `openpfe` executable (production: current exe; tests: target dir).
    pub fn for_openpfe_binary() -> Result<Self, ClientError> {
        if let Ok(path) = std::env::var("CARGO_BIN_EXE_openpfe") {
            return Ok(Self::with_exe(path));
        }
        if let Ok(target_dir) = std::env::var("CARGO_TARGET_DIR") {
            let candidate = PathBuf::from(target_dir).join("debug/openpfe");
            if candidate.is_file() {
                return Ok(Self::with_exe(candidate));
            }
        }
        Self::current_exe()
    }
}

impl ServerSpawn for ReExecSpawn {
    fn spawn_detached_server(&self) -> Result<(), ClientError> {
        let mut cmd = std::process::Command::new(&self.exe);
        cmd.arg("--server")
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());

        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            unsafe {
                cmd.pre_exec(|| {
                    if libc::setsid() == -1 {
                        return Err(std::io::Error::last_os_error());
                    }
                    Ok(())
                });
            }
        }

        cmd.spawn()
            .map(|_| ())
            .map_err(|e| ClientError::Spawn(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::time::Duration;

    use super::*;

    #[test]
    fn re_exec_spawn_resolves_current_exe() {
        let spawn = ReExecSpawn::current_exe().expect("current_exe");
        assert!(!spawn.exe.as_os_str().is_empty());
    }

    #[tokio::test]
    async fn detached_spawn_creates_socket_under_temp_project() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(dir.path().join(".openpfe").join("server")).expect("layout");
        std::env::set_current_dir(dir.path()).expect("chdir");

        let spawn = ReExecSpawn::for_openpfe_binary().expect("bin");
        spawn.spawn_detached_server().expect("spawn");

        let socket = Path::new(".openpfe/server/socket");
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        while std::time::Instant::now() < deadline {
            if socket.exists() {
                let mut client = openpfe_ipc::IpcClient::connect(socket)
                    .await
                    .expect("connect");
                if client.echo().await.is_ok() {
                    let _ = client.shutdown().await;
                    return;
                }
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        panic!("server socket never became ready");
    }
}
