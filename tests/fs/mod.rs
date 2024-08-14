// NOTE: this is needed because clippy gives false positives when compiling each integration test in different crates
#![allow(dead_code)]

use std::path::PathBuf;

pub fn any_temp_dir() -> PathBuf {
    let name = uuid::Uuid::new_v4().simple().to_string();
    let path = std::env::temp_dir()
        .join("dra-integration-tests")
        .join(name);
    std::fs::create_dir_all(&path).unwrap();
    path
}

pub fn any_temp_file(name: &str) -> PathBuf {
    let dir = any_temp_dir();
    dir.join(name)
}

pub fn path_to_string(path: PathBuf) -> String {
    path.to_str().unwrap().to_owned()
}
