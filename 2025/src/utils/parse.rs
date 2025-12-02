use std::fs;
use std::path::Path;

pub fn load_file(path: impl AsRef<Path>) -> String {
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {:?}: {}", path.as_ref(), e))
}

pub fn load_lines(path: impl AsRef<Path>) -> Vec<String> {
    load_file(path).lines().map(|l| l.to_string()).collect()
}
