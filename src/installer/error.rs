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
    NoExecutables,
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
            InstallError::NoExecutables => {
                f.write_str("No executables found")?;
                let hint = if cfg!(target_family = "unix") {
                    Some("The archive may be empty or files in it may lack executable permissions")
                } else if cfg!(target_os = "windows") {
                    Some("The archive may be empty or may not have any executables (.exe)")
                } else {
                    None
                };
                match hint {
                    Some(hint) => f.write_str(&format!("\n{}", hint)),
                    None => Ok(()),
                }
            }
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

        let failures_title_options = if self.successes.is_empty() {
            ("", "all")
        } else {
            ("\n\n", "some")
        };
        let show_failures_title = !self.successes.is_empty() || self.failures.len() > 1;
        if show_failures_title {
            f.write_str(&format!(
                "{}Failed to install {} executables:\n",
                failures_title_options.0, failures_title_options.1
            ))?;
        }

        let failures = self
            .failures
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("\n")
            .to_string();
        f.write_str(&failures)?;

        Ok(())
    }
}

impl std::fmt::Display for ArchiveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ArchiveError(executable, error) = self;
        match error {
            ArchiveErrorType::ExecutableNotFound => {
                f.write_str(&format!("Executable {} not found", executable))
            }
            ArchiveErrorType::TooManyExecutableCandidates(candidates) => {
                let mut message = String::new();
                message.push_str("Many executable candidates found, you must select one:\n");
                for candidate in candidates {
                    let x = format!("- {}\n", candidate);
                    message.push_str(&x);
                }
                message.push_str("\nYou can use --install-file <INSTALL_FILE> instead");
                f.write_str(&message)
            }
            ArchiveErrorType::CopyExecutable(from, to, error) => f.write_str(&format!(
                "Unable to copy {} to {} ({})",
                from.display(),
                to.display(),
                error
            )),
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

#[cfg(test)]
mod archive_installer_error_tests {
    use super::*;

    #[test]
    fn successes_and_failures() {
        let error = ArchiveInstallerError {
            successes: vec!["mytool".to_string(), "mytool2".to_string()],
            failures: vec![
                ArchiveError("mytool3".to_string(), ArchiveErrorType::ExecutableNotFound),
                ArchiveError("mytool4".to_string(), ArchiveErrorType::ExecutableNotFound),
            ],
        };

        let result = error.to_string();

        assert_eq!(
            "mytool
mytool2

Failed to install some executables:
Executable mytool3 not found
Executable mytool4 not found",
            result
        );
    }

    #[test]
    fn only_failures() {
        let error = ArchiveInstallerError {
            successes: vec![],
            failures: vec![
                ArchiveError("mytool".to_string(), ArchiveErrorType::ExecutableNotFound),
                ArchiveError("mytool2".to_string(), ArchiveErrorType::ExecutableNotFound),
            ],
        };

        let result = error.to_string();

        assert_eq!(
            "Failed to install all executables:
Executable mytool not found
Executable mytool2 not found",
            result
        );
    }

    #[test]
    fn only_one_failure() {
        let error = ArchiveInstallerError {
            successes: vec![],
            failures: vec![ArchiveError(
                "mytool".to_string(),
                ArchiveErrorType::ExecutableNotFound,
            )],
        };

        let result = error.to_string();

        assert_eq!("Executable mytool not found", result);
    }
}
