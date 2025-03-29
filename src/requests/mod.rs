pub mod model;
mod parser;
use std::{collections::HashMap, error::Error, path::Path};

use crate::file_system::FileSystem;

use self::model::Request;

pub fn get_all(fs: &dyn FileSystem, requests_path: &Path) -> Result<HashMap<String, Request>, Box<dyn Error>> {

    match fs.read_dir(requests_path) {
        Ok(files) => {
            let mut requests: HashMap<String, Request> = HashMap::new();
            for file in files {
                let file_content = fs.read_file(file.as_path()).unwrap();
                match parser::requests(&file_content) {
                    Ok(file_requests) => {
                        for request in file_requests {
                            requests.insert(request.name.clone(), request);
                        }
                    }
                    Err(err) => {
                        return Err(Box::new(err));
                    },
                }
            }
            Ok(requests)
        }
        Err(e) => Err(Box::new(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_system::MockFileSystem;
    use std::{path::PathBuf, vec};

    #[test]
    fn test_list_request() {
        let mut mock_fs = MockFileSystem::new();
        mock_fs
            .expect_read_dir()
            .returning(|_| Ok(vec![PathBuf::from("path1")]));
        mock_fs
            .expect_read_file()
            .returning(|_| Ok(String::from("# Get contacts\nGET url.example.com")));

        let requests = get_all(&mock_fs, Path::new("anypath")).unwrap();
        assert_eq!(requests.len(), 1);

        let request = &requests["Get contacts"];

        assert_eq!(request.name, "Get contacts");
    }
}
