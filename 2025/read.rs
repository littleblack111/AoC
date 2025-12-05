use std::{fs, path::Path, str::pattern::Pattern};

pub fn read(file: impl AsRef<Path>, p: impl Pattern) -> Vec<String> {
    let mut ret: Vec<String> = fs::read_to_string(file)
        .expect("Failed to read file")
        .split(p)
        .map(|l| l.to_string())
        .collect();
    if ret
        .last()
        .unwrap()
        .is_empty()
    {
        ret.pop();
    }
    ret
}
