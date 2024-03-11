mod cli;
mod file_system;
mod requests;
mod executor;

use clap::Parser;
use cli::{Cli, Commands};
use file_system::StandardFileSystem;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    //TODO: Move printing to the commandline to another place. It should not be in main.cs
    match cli.command {
        Commands::List => {
            let fs = StandardFileSystem;
            let requests = requests::get_all(&fs).unwrap();
            
            println!("  Requests:");
            for request in requests {
                println!("  - Name: {}", request.1.name);
            }
        }
        Commands::Run { request_name, headers } => {
            let fs = StandardFileSystem;
            let requests = requests::get_all(&fs).unwrap();

            match requests.get(&request_name) {
                Some(request) => {
                    let response = executor::execute_request(request).await;
                    println!("Status Code: {}", response.status);
                    println!("Duration: {:?}", response.duration);
                    if headers {
                        println!("Headers:");
                        for (key, value) in response.headers {
                            println!("{}: {:?}", key.unwrap(), value);
                        }
                    }
                    println!("Body:");
                    println!("{}", response.body);
                },
                None => println!("Request '{}' not found", request_name)

            }
        }
    }
}
