use clap::Parser;
use openpfe::cli::Cli;
use openpfe_server::{ServerOptions, run_server_with_opts};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if cli.server {
        if let Err(e) = run_server_with_opts(ServerOptions::default()).await {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
        return;
    }

    if let Err(e) = openpfe::run(cli).await {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
