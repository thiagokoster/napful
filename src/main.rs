mod cli;
mod file_system;
mod requests;

use clap::Parser;
use cli::{Cli, Commands};
use file_system::StandardFileSystem;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            let fs = StandardFileSystem;
            let requests = requests::get_all(&fs).unwrap();
            
            println!("  Requests:");
            for request in requests {
                println!("  - Name: {}", request.1.name);
            }
        }
        Commands::Run { request_name } => {
            let fs = StandardFileSystem;
            let requests = requests::get_all(&fs).unwrap();

            match requests.get(&request_name) {
                Some(r) => println!("Executing request: {} ...", r.name),
                None => println!("Request '{}' not found", request_name)

            }

        }
    }
}
