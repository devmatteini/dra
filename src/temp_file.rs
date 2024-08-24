use std::path::PathBuf;

use uuid::Uuid;

pub fn temp_file() -> PathBuf {
    let mut temp_dir = std::env::temp_dir();
    temp_dir.push(format!("dra-{}", Uuid::new_v4().simple()));
    temp_dir
}

pub fn make_temp_dir() -> Result<PathBuf, std::io::Error> {
    let temp_dir = temp_file();
    std::fs::create_dir(&temp_dir)?;
    Ok(temp_dir)
}
