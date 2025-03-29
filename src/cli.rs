use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    List,
    Run {
        #[clap(value_parser)]
        request_name: String,
        /// Pass this flag to show all headers
        #[clap(long, action=clap::ArgAction::SetTrue)]
        headers: bool,
        /// Pass this flag to format reponse body
        #[clap(long, action=clap::ArgAction::SetTrue)]
        formatted: bool,
    },
}


