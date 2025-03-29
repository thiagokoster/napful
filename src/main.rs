mod cli;
mod executor;
mod file_system;
mod requests;

use std::env;

use clap::Parser;
use cli::{Cli, Commands};
use file_system::StandardFileSystem;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Get current directory
    let cwd = env::current_dir().expect("Failed to determine current directory");
    let requests_path = cwd.join("requests");

    let fs = StandardFileSystem;

    //TODO: Move printing to the commandline to another place. It should not be in main.cs
    match cli.command {
        Commands::List => {
            let requests = requests::get_all(&fs, requests_path.as_path()).unwrap();

            println!("  Requests:");
            for request in requests {
                match request.1.error {
                    Some(e) => println!("  - Name: {} -> ERR: {}", request.1.name, e),
                    None => println!("  - Name: {}", request.1.name),
                }
            }
        }
        Commands::Run {
            request_name,
            headers,
            formatted
        } => {
            let fs = StandardFileSystem;
            let requests_result = requests::get_all(&fs, requests_path.as_path());
            let Ok(requests) = requests_result else {
                let err = requests_result.err().unwrap();
                eprintln!("Request failed: {err}");
                return;
            };

            match requests.get(&request_name) {
                Some(request) => {
                    if let Some(err) = &request.error {
                        println!("Invalid request: {}", err);
                        return;
                    }
                    let result = executor::execute_request(request, formatted).await;
                    let Ok(response) = result else {
                        let err = result.err().unwrap();
                        eprintln!("Request failed: {err}");
                        return;
                    };

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
                }
                None => println!("Request '{}' not found", request_name),
            }
        }
    }
}
