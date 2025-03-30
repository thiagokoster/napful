use rusqlite::Connection;

use crate::{models::request::Request, storage::database::DatabaseError};

pub fn get_all(conn: &Connection) -> Result<Vec<Request>, DatabaseError> {
    let mut query = conn.prepare(
        "SELECT id, file_id, name, method, url, headers, body
         FROM request"
    )?;

    let rows = query.query_map([], Request::from_row)?;
    let requests = rows.collect::<Result<Vec<_>, _>>();
    Ok(requests?)
}
