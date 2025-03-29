use core::fmt;
use std::{collections::HashMap, str::FromStr, time::Duration};

use reqwest::{header::HeaderMap, StatusCode};

#[derive(Debug)]
pub enum ParseError {
    InvalidHttpMethod(String),
    InvalidBody(String),
    InvalidRequestLine(String)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidHttpMethod(method) => write!(f, "Invalid http method '{method}'"),
            ParseError::InvalidBody(request) => write!(f, "Invalid body for request '{request}'"),
            ParseError::InvalidRequestLine(line) => write!(f, "Invalid request line: '{line}'"),
        }
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, PartialEq, Default)]
pub enum HttpMethod {
    #[default]
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
            _ => Err(ParseError::InvalidHttpMethod(s.to_string())),
        }
    }
}

#[derive(Default)]
pub struct Request {
    pub name: String,
    pub method: HttpMethod,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub error: Option<ParseError>,
}

impl Request {
    pub fn new() -> Request {
        Request::default()
    }
}

pub struct Response {
    pub status: StatusCode,
    pub duration: Duration,
    pub headers: HeaderMap,
    pub body: String,
}
