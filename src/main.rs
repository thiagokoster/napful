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
            let request_files = requests::get_all(&fs).unwrap();
            
            println!("Found {} files", request_files.len());
            for request_file in request_files {
                println!(" File: {}", request_file.0);
                println!("  Requests:");
                for request in request_file.1 {
                    println!("  - Name: {}", request.name);
                }
            }
        }
        Commands::Run { request_name } => {
            println!("Executing request: {} ...", request_name);
        }
    }
}
