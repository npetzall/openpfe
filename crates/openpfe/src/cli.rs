use std::time::Duration;

use clap::{Parser, Subcommand};

/// `openpfe` — project-local server and CLI client.
#[derive(Parser, Debug)]
#[command(name = "openpfe", about = "OpenPFE project server and CLI")]
pub struct Cli {
    /// Verbose logging to stderr (lock/echo retries, spawn milestones).
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Max wait for IPC echo when the pid lock is held (seconds).
    #[arg(long, global = true, env = "OPENPFE_TIMEOUT", default_value = "5")]
    pub timeout: u64,

    /// Skip opening a browser on the default command.
    #[arg(long, global = true, env = "OPENPFE_NO_BROWSER")]
    pub no_browser: bool,

    /// Internal: run the long-lived server (wired in plan 005).
    #[arg(long, hide = true)]
    pub server: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Stdio MCP bridge to the project server.
    Mcp,
    /// Graceful server shutdown (idempotent when not running).
    Stop,
}

impl Cli {
    pub fn timeout_duration(&self) -> Duration {
        Duration::from_secs(self.timeout)
    }

    pub fn skip_browser(&self) -> bool {
        self.no_browser
            || std::env::var_os("OPENPFE_NO_BROWSER").is_some_and(|v| !v.is_empty() && v != "0")
    }
}
