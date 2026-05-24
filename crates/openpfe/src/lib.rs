//! `openpfe` CLI binary library — ports, client flow, MCP stdio bridge.

pub mod adapters;
pub mod bridge;
pub mod cli;
pub mod client;
pub mod diagnostics;
pub mod error;
pub mod paths;
pub mod ports;
pub mod spawn;

use std::sync::Arc;

use adapters::{IpcProjectControl, ReExecSpawn};
use cli::{Cli, Command};
use error::ClientError;
use paths::ProjectPaths;

/// Run the CLI with production IPC and detached spawn adapters.
pub async fn run(cli: Cli) -> Result<(), ClientError> {
    let paths = ProjectPaths::from_cwd()?;
    let timeout = cli.timeout_duration();
    let verbose = cli.verbose;

    let control = Arc::new(IpcProjectControl::from_paths(&paths));
    let spawner = ReExecSpawn::for_openpfe_binary()?;

    match cli.command {
        None => match client::run_default(
            &paths,
            control.as_ref(),
            &spawner,
            timeout,
            verbose,
            cli.skip_browser(),
        )
        .await
        {
            Ok(()) => Ok(()),
            Err(e @ ClientError::EchoTimeout { .. }) => client::exit_on_echo_timeout(&paths, e),
            Err(e) => Err(e),
        },
        Some(Command::Stop) => client::run_stop(control.as_ref()).await,
        Some(Command::Mcp) => {
            let echo =
                match client::ensure_server(&paths, control.as_ref(), &spawner, timeout, verbose)
                    .await
                {
                    Ok(info) => info,
                    Err(e @ ClientError::EchoTimeout { .. }) => {
                        client::exit_on_echo_timeout(&paths, e)
                    }
                    Err(e) => return Err(e),
                };
            if verbose {
                eprintln!(
                    "openpfe: mcp bridge attached (server at {})",
                    echo.http_base_url
                );
            }
            bridge::run_stdio_bridge(control.as_ref(), verbose).await
        }
    }
}
