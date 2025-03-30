use std::path::PathBuf;

use rusqlite::Row;

pub struct RequestFile {
    pub id: Option<u64>,
    pub path: PathBuf,
    pub hash: String,
    pub content: String,
}
impl RequestFile {
    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            path: PathBuf::from(row.get::<_, String>("path")?),
            hash: row.get("hash")?,
            content: row.get("content")?,
        })
    }
}
 
