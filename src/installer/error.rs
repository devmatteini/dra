use std::fmt::Formatter;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum InstallError {
    NotAFile(String),
    NotSupported(String),
    Fatal(String),
    NoExecutable,
    TooManyExecutableCandidates(Vec<String>),
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
            InstallError::NoExecutable => f.write_str("No executable found"),
            InstallError::TooManyExecutableCandidates(candidates) => {
                f.write_str("Many executable candidates found, you must select one:\n")?;
                for candidate in candidates {
                    let message = format!("- {}\n", candidate);
                    f.write_str(&message)?;
                }
                Ok(())
            }
        }
    }
}

pub trait InstallErrorMapErr<T> {
    fn map_fatal_err(self, message: String) -> Result<T, InstallError>;
}

impl<T> InstallErrorMapErr<T> for std::io::Result<T> {
    fn map_fatal_err(self, message: String) -> Result<T, InstallError> {
        self.map_err(|e| InstallError::Fatal(format!("{}:\n  {}", &message, e)))
    }
}
