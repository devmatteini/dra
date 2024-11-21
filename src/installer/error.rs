use std::fmt::Formatter;
use std::path::{Path, PathBuf};

pub type ExecutableName = String;

#[derive(Debug, PartialEq)]
pub enum ArchiveErrorType {
    ExecutableNotFound,
    TooManyExecutableCandidates(Vec<String>),
    CopyExecutable(PathBuf, PathBuf, String),
}

#[derive(Debug, PartialEq)]
pub struct ArchiveError(pub ExecutableName, pub ArchiveErrorType);

#[derive(Debug, PartialEq)]
pub struct ArchiveInstallerError {
    pub successes: Vec<ExecutableName>,
    // TODO: it would be nice to have a NonEmptyVec
    pub failures: Vec<ArchiveError>,
}

#[derive(Debug, PartialEq)]
pub enum InstallError {
    NotAFile(String),
    NotSupported(String),
    Fatal(String),
    NoExecutable,
    Archive(ArchiveInstallerError),
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
            InstallError::Archive(error) => {
                let message = format!("{}", error);
                f.write_str(&message)
            }
        }
    }
}

impl std::fmt::Display for ArchiveInstallerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let successes = self.successes.join("\n").to_string();
        f.write_str(&successes)?;

        let some_or_all = if self.successes.is_empty() {
            "all"
        } else {
            "some"
        };
        let failures = format!("Failed to install {} executables:\n", some_or_all);
        f.write_str(&failures)?;
        for ArchiveError(executable, error) in self.failures.iter() {
            match error {
                ArchiveErrorType::ExecutableNotFound => {
                    let message = format!("Executable {} not found", executable);
                    f.write_str(&message)?;
                }
                ArchiveErrorType::TooManyExecutableCandidates(candidates) => {
                    f.write_str("Many executable candidates found, you must select one:\n")?;
                    for candidate in candidates {
                        let message = format!("- {}\n", candidate);
                        f.write_str(&message)?;
                    }
                    f.write_str("\nYou can use --install-file <INSTALL_FILE> instead")?;
                }
                ArchiveErrorType::CopyExecutable(from, to, error) => {
                    let message = format!(
                        "Unable to copy {} to {} ({})",
                        from.display(),
                        to.display(),
                        error
                    );
                    f.write_str(&message)?;
                }
            }
        }

        Ok(())
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
