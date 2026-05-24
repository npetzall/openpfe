//! Server startup options (phase 1 subset).

use std::time::Duration;

/// Options for [`crate::run_server_with_opts`].
#[derive(Debug, Clone)]
pub struct ServerOptions {
    /// Log to stderr instead of `./.openpfe/server/openpfe.log`.
    pub foreground: bool,
    /// Graceful drain bound before force-cancel (default **5s**).
    pub shutdown_timeout: Duration,
}

impl Default for ServerOptions {
    fn default() -> Self {
        Self {
            foreground: false,
            shutdown_timeout: Duration::from_secs(5),
        }
    }
}
