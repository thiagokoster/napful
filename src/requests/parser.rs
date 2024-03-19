use std::str::FromStr;

use super::model::{HttpMethod, ParseError, Request};

const NAME_DELIMITER: &str = "#";

pub fn requests(content: &str) -> Result<Vec<Request>, ParseError> {
    let mut lines = content.lines().peekable();
    let mut requests = Vec::new();

    while let Some(line) = lines.next() {
        // Read request
        if line.starts_with(NAME_DELIMITER) {
            let name = line.trim_start_matches(NAME_DELIMITER).trim().to_string();
            let mut method = HttpMethod::Get;
            let mut url: Option<String> = None;
            let mut body: Option<String> = None;
            let mut error: Option<ParseError> = None;


            // Read request method and URL
            if let Some(next_line) = lines.next() {
                let parts: Vec<&str> = next_line.split_whitespace().collect();
                if parts.len() == 2 {
                    method = HttpMethod::from_str(parts[0])?;
                    url = Some(parts[1].to_string());
                }
            }


            if let Some(next_line) = lines.next() {
                // Request has body
                if next_line.starts_with("{") {
                    // Parse body to content
                    let mut content = String::from("{\n");
                    while let Some(&l) = lines.peek() {
                        if l == "}" {
                            lines.next();
                            content.push_str(l);
                            content.push('\n');
                            break;
                        }
                        if l.starts_with("#") {
                           break; 
                        }

                        lines.next();
                        content.push_str(l);
                        content.push('\n');
                    }
                    match serde_json::from_str::<serde::de::IgnoredAny>(content.as_str()) {
                        Ok(_) => body = Some(content),
                        Err(_) => error = Some(ParseError::new("Invalid body")),
                    };
                }
            }

            // Add request
            requests.push(Request {
                name,
                method,
                url: url.clone().expect("request must have an url").clone(),
                body,
                error,
            });
        }
    }

    Ok(requests)
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
}
"#,
        );

        let requests = requests(file_content).expect("Should work with no errors");
        let request = requests.first().unwrap();
        let request_body = &request.body;
        assert!(request_body.is_some());
        assert_eq!(*request_body, Some(expected_body));
    }
}
