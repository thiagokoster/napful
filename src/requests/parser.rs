use std::{io, str::FromStr};

use super::model::{HttpMethod, Request};

const NAME_DELIMITER: &str = "#";

pub fn requests(content: &str) -> io::Result<Vec<Request>> {
    let mut lines = content.lines();
    let mut requests = Vec::new();

    while let Some(line) = lines.next() {
        // Read request
        if line.starts_with(NAME_DELIMITER) {
            let name = line.trim_start_matches(NAME_DELIMITER).trim().to_string();
            let mut method: Option<String> = None;
            let mut url: Option<String> = None;

            // Read request params
            while let Some(next_line) = lines.next() {
                if next_line.is_empty() {
                    break;
                }

                let parts: Vec<&str> = next_line.split_whitespace().collect();
                if parts.len() == 2 {
                    method = Some(parts[0].to_string());
                    url = Some(parts[1].to_string());
                }
            }
            // Add request
            requests.push(Request {
                name,
                method: HttpMethod::from_str(&method.unwrap())?,
                url: url.clone().expect("request must have an url").clone(),
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
}
