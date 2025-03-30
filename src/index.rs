use rusqlite::Connection;

use crate::{errors::NapfulError, models::file::RequestFile, storage::{file, repositories}};

pub fn update(conn: &Connection) -> Result<(), NapfulError> {
    let file_sources = file::get_file_sources()?;
    let files = repositories::file::get_all(&conn)?;

    for source in file_sources {
        if let Some(existing) = files.get(&source.path) {
            
            // File exists but changed
            if existing.hash != source.hash {
            }
        } else {  // new file
           repositories::file::insert(conn, &RequestFile {
                id: None,
                path: source.path,
                hash: file::hash_file_content(&source.content),
                content: source.content
            })?;
        }
    }

    Ok(())
}
