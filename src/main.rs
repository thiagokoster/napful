mod cli;
mod requests;

use clap::Parser;
use cli::{Cli, Commands};

use crate::requests::list_requests;


fn main() {
    println!("Hello, world!");

    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            list_requests()
        },
        Commands::Run {request_name} => {
            println!("Executing request: {} ...", request_name);
        }
    }
}

