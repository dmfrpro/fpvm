use std::fs;
use std::path::{Path, PathBuf};

pub fn collect_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for entry in
        fs::read_dir(dir).unwrap_or_else(|e| panic!("cannot read dir {}: {e}", dir.display()))
    {
        let path = entry
            .unwrap_or_else(|e| panic!("cannot read dir entry {e}"))
            .path();

        if path.is_file() {
            files.push(path);
        }
    }

    files.sort();
    files
}

pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

pub fn yellow(s: &str) -> String {
    format!("\x1b[33m{}\x1b[0m", s)
}