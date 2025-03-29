use crate::{cli::Commands, errors::NapfulError};

pub async fn dispatch(command: Commands) -> Result<(), NapfulError> {
    match command {
        Commands::List => todo!(),
        Commands::Run { request_name, headers, formatted } => todo!(),
    }
}
