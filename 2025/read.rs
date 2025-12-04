use std::{fs, path::Path, str::pattern::Pattern};

pub fn read(file: impl AsRef<Path>, p: impl Pattern) -> Vec<String> {
    fs::read_to_string(file)
        .expect("Failed to read file")
        .split(p)
        .map(|l| l.to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
