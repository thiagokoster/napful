use std::{env, path::PathBuf};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

use crate::errors::NapfulError;

pub struct FileSource {
    pub path: PathBuf,
    pub content: String,
    pub hash: String,
}

pub fn get_file_sources() -> Result<Vec<FileSource>, NapfulError> {
    const EXTENSIONS: [&str; 2] = ["txt", "md"];
    let mut files = Vec::new();

    let requests_dir = env::current_dir()?.join("requests");
    let wd = WalkDir::new(requests_dir);
    for entry in wd.into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| 
            e.path().extension().and_then(|ext| ext.to_str())
                .map_or(false, |ext| EXTENSIONS.contains(&ext))
        )
    {
        let content = std::fs::read_to_string(&entry.path())?;
        files.push(FileSource {
            path: entry.path().to_path_buf(),
            content: content.clone(),
            hash: hash_file_content(&content)
        });
    }

    Ok(files)
}

pub fn hash_file_content(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&content);
    let hash = hasher.finalize();
    format!("{:x}", hash)
}
