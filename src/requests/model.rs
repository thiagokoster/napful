use core::fmt;
use std::{str::FromStr, time::Duration};

use reqwest::{header::HeaderMap, StatusCode};

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

impl ParseError {
    pub fn new(message: &str) -> ParseError {
        ParseError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse Error: {}", self.message)
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl FromStr for HttpMethod {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binding = s.to_lowercase();
        let s = binding.as_str();
        match s {
            "get" => Ok(HttpMethod::Get),
            "post" => Ok(HttpMethod::Post),
            "put" => Ok(HttpMethod::Put),
            "patch" => Ok(HttpMethod::Patch),
            "delete" => Ok(HttpMethod::Delete),
            _ => Err(ParseError::new(
                format!(
                    "Invalid HttpMethod '{}'. Use Get, Post, Put, Patch or Delete",
                    s
                )
                .as_str(),
            )),
        }
    }
}

pub struct Request {
    pub name: String,
    pub method: HttpMethod,
    pub url: String,
    pub body: Option<String>,
    pub error: Option<ParseError>,
}

pub struct Response {
    pub status: StatusCode,
    pub duration: Duration,
    pub headers: HeaderMap,
    pub body: String,
}
