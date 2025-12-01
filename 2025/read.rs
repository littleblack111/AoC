use std::{fs, path::Path};

pub fn read(file: impl AsRef<Path>) -> Vec<String> {
    fs::read_to_string(file)
        .expect("Failed to read file")
        .lines()
        .map(|l| l.to_string())
        .collect()
}
