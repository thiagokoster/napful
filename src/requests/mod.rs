use std::{env, io::Error};

use crate::file_system::FileSystem;

pub fn list_requests(fs: &dyn FileSystem) -> Result<Vec<String>, Error> {
    // Get current directory
    let cwd = env::current_dir().expect("Failed to determine current directory");
    let requests_path = cwd.join("requests");

    match fs.read_dir(&requests_path) {
        Ok(entries) => {
            println!("Listing requests in: {}", requests_path.display());
            let mut files: Vec<String> = vec![];
            for entry in entries {
                let file_content = fs.read_file(entry.as_path());
                println!("File content:");
                println!("{}", file_content.unwrap());
                files.push(entry.display().to_string());
            }
            Ok(files)
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
        mock_fs.expect_read_dir().returning(|_| Ok(vec![PathBuf::from("path1")]));
        mock_fs.expect_read_file().returning(|_| Ok(String::from("Hello world")));

        let out = list_requests(&mock_fs).unwrap();

        assert_eq!(out[0], "path1");
    }
}
