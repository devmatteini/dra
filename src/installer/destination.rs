use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Destination {
    Directory(PathBuf),
    File(PathBuf),
}
