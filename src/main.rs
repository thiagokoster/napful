mod cli;
mod file_system;
mod requests;

use clap::Parser;
use cli::{Cli, Commands};
use file_system::StandardFileSystem;

use crate::requests::list_requests;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            let fs = StandardFileSystem;
            let _ = list_requests(&fs);
        }
        Commands::Run { request_name } => {
            println!("Executing request: {} ...", request_name);
        }
    }
}
