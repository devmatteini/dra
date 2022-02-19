use std::path::PathBuf;

use uuid::Uuid;

pub fn temp_file() -> PathBuf {
    let mut temp_dir = std::env::temp_dir();
    temp_dir.push(format!("dra-{}", Uuid::new_v4().to_simple()));
    temp_dir
}

pub fn temp_dir() -> PathBuf {
    let mut temp_dir = std::env::temp_dir();
    temp_dir.push(format!("dra-{}/", Uuid::new_v4().to_simple()));
    temp_dir
}
