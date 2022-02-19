use std::fmt::Formatter;
use std::path::Path;

#[derive(Debug)]
pub enum InstallError {
    NotAFile(String),
    NotSupported(String),
    Fatal(String),
}

impl InstallError {
    pub fn not_a_file(path: &Path) -> InstallError {
        InstallError::NotAFile(format!("{} is not a file", path.display()))
    }

    pub fn not_supported(name: &str) -> InstallError {
        InstallError::NotSupported(format!("{} is not supported", name,))
    }
}

impl std::fmt::Display for InstallError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InstallError::NotAFile(msg) => f.write_str(msg),
            InstallError::NotSupported(msg) => f.write_str(msg),
            InstallError::Fatal(msg) => f.write_str(msg),
        }
    }
}

pub trait MapErrWithMessage<T> {
    fn map_err_with(self, message: String) -> Result<T, String>;
}

impl<T> MapErrWithMessage<T> for std::io::Result<T> {
    fn map_err_with(self, message: String) -> Result<T, String> {
        self.map_err(|e| format!("{}:\n  {}", &message, e))
    }
}
