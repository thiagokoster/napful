use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    List,
    Run {
        #[clap(value_parser)]
        request_name: String

    }
}
