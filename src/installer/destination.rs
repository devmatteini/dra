use std::path::PathBuf;

#[derive(Debug)]
pub enum Destination {
    Directory(PathBuf),
    File(PathBuf),
}
