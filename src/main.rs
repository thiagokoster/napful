use clap::Parser;
use cli::Cli;
use commands::dispatch;

mod cli;
mod commands;
mod errors;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if let Err(e) = dispatch(cli.command).await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }

}
