mod model;
mod parser;
use std::{collections::HashMap, env, io::Error};

use crate::file_system::FileSystem;

use self::model::Request;

pub fn get_all(fs: &dyn FileSystem) -> Result<HashMap<String, Request>, Error> {
    // Get current directory
    let cwd = env::current_dir().expect("Failed to determine current directory");
    let requests_path = cwd.join("requests");

    match fs.read_dir(&requests_path) {
        Ok(files) => {
            let mut requests: HashMap<String, Request> = HashMap::new();
            for file in files {
                let file_content = fs.read_file(file.as_path()).unwrap();
                let file_requests = parser::requests(&file_content)?;
                for request in file_requests {
                    requests.insert(request.name.clone(), request);
                }
            }
            Ok(requests)
        }
        Err(e) => Err(e),
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
            .returning(|_| Ok(String::from("# Get contacts")));

        let requests = get_all(&mock_fs).unwrap();
        assert_eq!(requests.len(), 1);

        let request = &requests["Get contacts"];

        assert_eq!(request.name, "Get contacts");


    }

}
