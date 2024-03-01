use std::fs::File;
use std::io::{Read, Result};
use std::path::{Path, PathBuf};

use mockall::automock;
use walkdir::WalkDir;

#[automock]
pub trait FileSystem {
    fn read_dir(&self, path: &Path) -> Result<Vec<PathBuf>>;
    fn read_file(&self, path: &Path) -> Result<String>;
}

pub struct StandardFileSystem;

impl FileSystem for StandardFileSystem {
    fn read_dir(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let wd = WalkDir::new(path);

        let mut files = Vec::new();
        for entry in wd
            .into_iter()
            .filter_map(|e| e.ok()) // ignore errors
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "txt"))
        {
            files.push(entry.path().to_path_buf());
        }
        println!("Found {} files", files.len());
        Ok(files)
    }

    fn read_file(&self, path: &Path) -> Result<String> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        let _ = file.read_to_string(&mut content);
        Ok(content)
    }
}
