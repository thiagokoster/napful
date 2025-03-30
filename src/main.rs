use clap::Parser;
use cli::Cli;
use commands::dispatch;

mod cli;
mod commands;
mod errors;
mod index;

mod storage {
    pub mod database;
    pub mod file;
    pub mod repositories {
        pub mod file;
        pub mod request;
    }
}

pub mod models {
    pub mod file;
    pub mod request;
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if let Err(e) = dispatch(cli.command).await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }

}
