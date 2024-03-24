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

    let env_file_path = requests_path.join(".env");
    let fs = StandardFileSystem;
    if let Err(e) = dotenvy::from_path(env_file_path.as_path()) {
        println!("Error while loading environment file: {}", e);
    }

    println!("BASE_URL: {:?}", env::var("BASE_URL"));

    //TODO: Move printing to the commandline to another place. It should not be in main.cs
    match cli.command {
        Commands::List => {
            let requests = requests::get_all(&fs, requests_path.as_path()).unwrap();

            println!("  Requests:");
            for request in requests {
                match request.1.error {
                    Some(e) => println!("  - Name: {} -> ERR: {}", request.1.name, e.message),
                    None => println!("  - Name: {}", request.1.name),
                }
            }
        }
        Commands::Run {
            request_name,
            headers,
        } => {
            let fs = StandardFileSystem;
            let requests = requests::get_all(&fs, requests_path.as_path()).unwrap();

            match requests.get(&request_name) {
                Some(request) => {
                    if let Some(err) = &request.error {
                        println!("Invalid request: {}", err.message);
                        return;
                    }
                    let response = executor::execute_request(request).await.unwrap();
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
