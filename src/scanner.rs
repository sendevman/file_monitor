use std::fs;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;
use chrono::{DateTime, Local};
use anyhow::Result;

pub fn scan_and_print_files(inbox_path: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(inbox_path).into_iter().filter_map(Result::ok).filter(|e| e.file_type().is_file()) {
        let path = entry.path().to_path_buf();
        let metadata = fs::metadata(&path)?;
        let modified = metadata.modified()?;
        let datetime: DateTime<Local> = modified.into();
        let relative = path.strip_prefix(inbox_path).unwrap();

        println!("[{}] {}", datetime.format("%Y-%m-%d %H:%M:%S"), relative.display());

        files.push(relative.to_path_buf());
    }

    Ok(files)
}
