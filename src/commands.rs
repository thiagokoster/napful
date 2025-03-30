use crate::cli::Commands;
use crate::errors::NapfulError;
use crate::index;
use crate::storage::{database, repositories};

pub async fn dispatch(command: Commands) -> Result<(), NapfulError> {
    match command {
        Commands::List => list().await,
        Commands::Run { request_name, headers, formatted } => todo!(),
    }
}

async fn list() -> Result<(), NapfulError> {
    let conn = database::get_connection(database::get_db_path()?)?;

    index::update(&conn)?;

    let requests = repositories::request::get_all(&conn)?;

    println!("Listing requests");
    for request in requests {
        println!(
            "{:<5} {:<30}",
            request.id,
            request.name,
        );
    }

    Ok(())
}
