use std::collections::HashMap;

use rusqlite::Row;

pub struct Request {
    pub id: u64,
    pub file_id: u64,
    pub name: String,
    pub method: Option<String>,
    pub url: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
}

impl Request {
    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        let headers = parse_header(row.get("headers")?);
        
        Ok(Self {
            id: row.get("id")?,
            file_id: row.get("file_id")?,
            name: row.get("name")?,
            method: row.get("method")?,
            url: row.get("url")?,
            headers,
            body: row.get("body")?,
        })
    }
}

fn parse_header(raw: Option<String>) -> Option<HashMap<String, String>> {
    if let Some(headers) = raw {
        let mut map : HashMap<String, String> = HashMap::new();
        for part in headers.split(',') {
            if let Some((k, v)) = part.split_once(':') {
                map.insert(k.trim().to_string(), v.trim().to_string());
            }
        }

        return Some(map);
    }
    return None;
}
