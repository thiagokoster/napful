use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};

pub trait FileSystem {
    fn read_dir(&self, path: &Path) -> Result<Vec<PathBuf>>;
}

pub struct StandardFileSystem;

impl FileSystem for StandardFileSystem {
    fn read_dir(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let mut paths = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            paths.push(entry.path());
        }
        Ok(paths)
    }
}
