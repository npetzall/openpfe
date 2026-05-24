//! Exclusive `pid` flock for singleton server process.

use fd_lock::{RwLock, RwLockWriteGuard};
use std::fs::OpenOptions;
use std::io::Write;

use crate::error::ServerError;
use crate::runtime::{PID_FILE, ensure_runtime_dir};

/// Held for process lifetime; releases flock when dropped.
pub struct PidLock {
    _guard: RwLockWriteGuard<'static, std::fs::File>,
}

/// Create runtime dir, acquire non-blocking exclusive flock, write ASCII PID.
pub fn acquire_pid_lock() -> Result<PidLock, ServerError> {
    ensure_runtime_dir()?;

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(PID_FILE)?;

    let rw: &'static mut RwLock<std::fs::File> = Box::leak(Box::new(RwLock::new(file)));
    let mut guard = rw.try_write().map_err(map_lock_error)?;
    guard.set_len(0)?;
    writeln!(guard, "{}", std::process::id())?;
    guard.sync_data()?;

    Ok(PidLock { _guard: guard })
}

fn map_lock_error(err: std::io::Error) -> ServerError {
    if err.kind() == std::io::ErrorKind::WouldBlock {
        ServerError::AlreadyRunning
    } else {
        ServerError::Io(err)
    }
}
