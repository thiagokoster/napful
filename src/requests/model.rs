use std::{io, str::FromStr, time::Duration};

use reqwest::{header::HeaderMap, StatusCode};

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete
}


impl FromStr for HttpMethod {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binding = s.to_lowercase();
        let s = binding.as_str();
        match s {
            "get" => Ok(HttpMethod::Get),
            "post" => Ok(HttpMethod::Post),
            "put" => Ok(HttpMethod::Put),
            "patch" => Ok(HttpMethod::Patch),
            "delete" => Ok(HttpMethod::Delete),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid HttpMethod. Use Get, Post, Put, Patch or Delete"))
        } 
    }
}


pub struct Request {
    pub name: String,
    pub method: HttpMethod,
    pub url: String,
}

pub struct Response {
    pub status: StatusCode,
    pub duration: Duration, 
    pub headers: HeaderMap,
    pub body: String
}
