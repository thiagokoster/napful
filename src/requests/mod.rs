mod model;
mod parser;
use std::{collections::HashMap, env, io::Error};

use crate::file_system::FileSystem;

use self::model::Request;

pub fn get_all(fs: &dyn FileSystem) -> Result<HashMap<String, Vec<Request>>, Error> {
    // Get current directory
    let cwd = env::current_dir().expect("Failed to determine current directory");
    let requests_path = cwd.join("requests");

    match fs.read_dir(&requests_path) {
        Ok(files) => {
            let mut requests: HashMap<String, Vec<Request>> = HashMap::new();
            for file in files {
                let file_content = fs.read_file(file.as_path()).unwrap();
                let file_requests = parser::requests(&file_content)?;
                requests.insert(file.file_name().unwrap().to_str().unwrap().to_string(), file_requests);
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

        let out = get_all(&mock_fs).unwrap();
        assert_eq!(out.len(), 1);

        let requests = out["path1"];
        let request = requests.first().unwrap();

        assert_eq!(request.name, "Get contacts");


    }

}
