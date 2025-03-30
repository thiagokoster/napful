use std::path::PathBuf;

use rusqlite::Connection;

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("missing file path")]
    MissingFilePath,

    #[error("I/O error while creating database directory: {0}")]
    Io(#[from] std::io::Error),

    #[error("Sqlite error: {0}")]
    Rusqlite(#[from] rusqlite::Error)
}

pub fn get_db_path() -> Result<PathBuf, DatabaseError> {
    let mut path = dirs::data_local_dir().ok_or(DatabaseError::MissingFilePath)?;
    path.push("napful");
    std::fs::create_dir_all(&path)?;
    path.push("napful.db");
    Ok(path)
}

pub fn get_connection(db_path: PathBuf) -> Result<Connection, DatabaseError> {
    println!("Connecting to db at: {}", db_path.to_string_lossy());
    let conn = Connection::open(db_path)?;
    initialize_schema(&conn)?;
    Ok(conn)
}

pub fn initialize_schema(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(
r#"
    CREATE TABLE IF NOT EXISTS file (
        id INTEGER PRIMARY KEY,
        path TEXT NOT NULL UNIQUE,
        hash TEXT NOT NULL,
        content TEXT NOT NULL
    );
    
    CREATE TABLE IF NOT EXISTS request (
        id INTEGER PRIMARY KEY,
        file_id INTEGER NOT NULL,
        name TEXT NOT NULL,
        method TEXT,
        url TEXT,
        headers TEXT,
        body TEXT,
        FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE
    );
"#
    )?;
    Ok(())
}
