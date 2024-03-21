use std::{str::FromStr};

use super::model::{HttpMethod, ParseError, Request};

const NAME_DELIMITER: &str = "#";

enum ParseState {
    Unknown,
    Name,
    MethodAndUrl,
    Headers,
    Body,
}

pub fn requests(content: &str) -> Result<Vec<Request>, ParseError> {
    let mut lines = content.lines().peekable();
    let mut requests = Vec::new();
    let mut state = ParseState::Unknown;
    let mut current_request = Request::new();
    let mut body_lines: Vec<String> = vec![];

    while let Some(line) = lines.peek() {

        match state {
            ParseState::Unknown => {
                if line.starts_with(NAME_DELIMITER){
                    state = ParseState::Name
                } else {
                    lines.next();
                }
            },
            ParseState::Name => {
                println!("Parsing name: {}", line);
                // Start a new request
                current_request = Request::new();
                current_request.name = line.trim_start_matches(NAME_DELIMITER).trim().to_string();
                state = ParseState::MethodAndUrl;
                lines.next();
            },
            ParseState::MethodAndUrl => {
                println!("Parsing method and url: {}", line);
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() == 2 {
                    current_request.method = HttpMethod::from_str(parts[0])?;
                    current_request.url = parts[1].to_string();
                }
                state = ParseState::Headers;
                lines.next();
            },
            ParseState::Headers => {
                println!("Parsing headers: {}", line);
                if line.trim().is_empty() || line.eq(&"{") {
                    state = ParseState::Body;
                } else { 
                    //TODO: Parse headers
                    lines.next();
                }
            },
            ParseState::Body => {
                println!("Parsing body: {}", line);
                if line.starts_with(NAME_DELIMITER){
                    // End of current request and start of a new one
                    validate_body(&body_lines, &mut current_request);
                    requests.push(current_request);

                    current_request = Request::new();
                    body_lines.clear();
                    state = ParseState::Unknown;
                } else {
                    if !line.is_empty(){
                        body_lines.push(line.to_string());
                    }
                    lines.next();
                }
            },
        }
    }

    // Add request
    if !body_lines.is_empty() {
        // TODO: Validate if is a valid json and set error property
        validate_body(&body_lines, &mut current_request);
        body_lines.clear();
    }
    requests.push(current_request);

    Ok(requests)
}

fn validate_body(body_lines : &Vec<String>, current_request : &mut Request) {
    if !body_lines.is_empty() {
        let body = body_lines.join("\n");
        println!("Validating body: {}", body);
        match serde_json::from_str::<serde::de::IgnoredAny>(body.as_str()) {
            Ok(_) => current_request.error = None,
            Err(e) => current_request.error = Some(ParseError::new(format!("invalid request body: {}", e).as_str()))
        }

        current_request.body = Some(body);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requests_when_multiple_test() {
        let file_content = "
# Get authors
GET http://localhost:3000/authors


# Get books
GET http://localhost:3000/authors/1/books
";

        let requests = requests(file_content);

        assert!(requests.is_ok());
        let requests = requests.unwrap();
        assert_eq!(requests.len(), 2);

        let authors = requests.first().unwrap();

        assert_eq!(authors.name, "Get authors");
        assert_eq!(authors.method, HttpMethod::Get);
        assert_eq!(authors.url, "http://localhost:3000/authors");

        let books = requests.last().unwrap();
        assert_eq!(books.name, "Get books");
        assert_eq!(books.method, HttpMethod::Get);
        assert_eq!(books.url, "http://localhost:3000/authors/1/books");
    }

    #[test]
    fn requests_when_invalid_method() {
        let file_content = "
# Get authors
GET23 http://localhost:3000/authors
";
        let requests = requests(file_content);
        assert!(requests.is_err());
    }

    #[test]
    fn requests_with_body() {
        let file_content = r#"
# Create author
POST http://localhost:3000/authors
{
  "name": "Isaac",
  "last_name": "Asimov"
}
"#;
        let expected_body = String::from(
            r#"{
  "name": "Isaac",
  "last_name": "Asimov"
}"#,
        );

        let requests = requests(file_content).expect("Should work with no errors");
        let request = requests.first().unwrap();
        let request_body = &request.body;
        assert!(request_body.is_some());
        assert_eq!(*request_body, Some(expected_body));
    }
}
