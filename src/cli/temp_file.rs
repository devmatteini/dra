use std::path::PathBuf;

use uuid::Uuid;

pub fn temp_file() -> PathBuf {
    let mut temp_dir = std::env::temp_dir();
    temp_dir.push(format!("dra-{}", Uuid::new_v4()));
    temp_dir
}
