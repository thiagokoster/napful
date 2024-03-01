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
    use std::{cell::RefCell, io, path::PathBuf};

    struct MockFileSystem {
        pub read_dir_behavior: RefCell<Vec<io::Result<Vec<PathBuf>>>>,
    }

    impl MockFileSystem {
        fn new() -> Self {
            MockFileSystem {
                read_dir_behavior: RefCell::new(vec![]),
            }
        }
    }

    impl FileSystem for MockFileSystem {
        fn read_dir(&self, _path: &std::path::Path) -> io::Result<Vec<PathBuf>> {
            self.read_dir_behavior
                .borrow_mut()
                .pop()
                .unwrap_or(Ok(vec![]))
        }
    }

    #[test]
    fn test_list_request() {
        let mock_fs = MockFileSystem::new();
        mock_fs
            .read_dir_behavior
            .borrow_mut()
            .push(Ok(vec![PathBuf::from("path1")]));
        let out = list_requests(&mock_fs).unwrap();

        assert_eq!(out[0], "path1");
    }
}
