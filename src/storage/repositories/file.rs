use std::{collections::HashMap, path::PathBuf};

use rusqlite::{params, Connection};

use crate::{models::file::RequestFile, storage::database::DatabaseError};

pub fn get_all(conn: &Connection) -> Result<HashMap<PathBuf, RequestFile>, DatabaseError> {
    let mut sql = conn.prepare(
        "SELECT id, path, hash, content
         FROM file"
    )?;
    let mut rows = sql.query([])?;

    let mut map = HashMap::new();
    while let Some(row) = rows.next()? {
        let file = RequestFile::from_row(row)?;
        map.insert(file.path.clone(), file);
    }
    Ok(map)
}

pub fn insert(conn: &Connection, file: &RequestFile) -> Result<(), DatabaseError> {
    conn.execute(
        "INSERT INTO file(path, hash, content) VALUES (?1, ?2, ?3)",
        params![
            file.path.to_string_lossy(),
            file.hash,
            file.content
        ]
    )?;

    Ok(())
}
