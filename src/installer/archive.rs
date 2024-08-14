use std::ffi::OsString;
#[cfg(target_family = "unix")]
use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::installer::error::InstallErrorMapErr;
use crate::installer::InstallerResult;

use super::error::InstallError;
use super::Executable;

pub struct ArchiveInstaller;

impl ArchiveInstaller {
    pub fn run<F>(
        extract_files: F,
        source: &Path,
        destination_dir: &Path,
        executable: &Executable,
    ) -> InstallerResult
    where
        F: FnOnce(&Path, &Path) -> Result<(), InstallError>,
    {
        let temp_dir = Self::create_temp_dir()?;
        extract_files(source, &temp_dir)?;

        let executable = Self::find_executable(&temp_dir, executable.name())?;

        Self::copy_executable_to_destination_dir(executable, destination_dir)?;
        Self::cleanup(&temp_dir)?;

        Ok(())
    }

    fn create_temp_dir() -> Result<PathBuf, InstallError> {
        let temp_dir = crate::cli::temp_file::temp_dir();
        std::fs::create_dir(&temp_dir)
            .map(|_| temp_dir)
            .map_fatal_err("Error creating temp dir".into())
    }

    fn find_executable(
        directory: &Path,
        executable_name: &str,
    ) -> Result<ExecutableFile, InstallError> {
        let ignore_error = |result: walkdir::Result<walkdir::DirEntry>| result.ok();

        let executables: Vec<_> = WalkDir::new(directory)
            .max_depth(3)
            .into_iter()
            .filter_map(ignore_error)
            .filter(Self::is_executable)
            .map(ExecutableFile::from_file)
            .collect();

        let preferred = executables.iter().find(|x| x.name == executable_name);
        if let Some(executable) = preferred {
            return Ok(executable.clone());
        }

        match executables.as_slice() {
            [] => Err(InstallError::NoExecutable),
            [x] => Ok(x.clone()),
            candidates => Err(too_many_executable_candidates(candidates, directory)),
        }
    }

    fn is_executable(x: &walkdir::DirEntry) -> bool {
        let path = x.path();

        path.metadata()
            .map(|metadata| path.is_file() && is_executable_file(path, metadata))
            .unwrap_or(false)
    }

    fn copy_executable_to_destination_dir(
        executable: ExecutableFile,
        destination_dir: &Path,
    ) -> Result<(), InstallError> {
        let mut to = PathBuf::from(destination_dir);
        to.push(executable.name);
        std::fs::copy(&executable.path, &to)
            .map(|_| ())
            .map_fatal_err(format!(
                "Error copying {} to {}",
                &executable.path.display(),
                to.display(),
            ))
    }

    fn cleanup(temp_dir: &Path) -> Result<(), InstallError> {
        std::fs::remove_dir_all(temp_dir).map_fatal_err("Error deleting temp dir".into())
    }
}

#[cfg(target_family = "unix")]
fn is_executable_file(_: &Path, metadata: std::fs::Metadata) -> bool {
    metadata.permissions().mode() & 0o111 != 0
}

#[cfg(target_os = "windows")]
fn is_executable_file(path: &Path, _: std::fs::Metadata) -> bool {
    path.extension().map(|x| x == "exe").unwrap_or(false)
}

#[derive(Clone)]
struct ExecutableFile {
    pub path: PathBuf,
    pub name: OsString,
}

impl ExecutableFile {
    fn from_file(x: walkdir::DirEntry) -> Self {
        Self {
            name: x.file_name().to_os_string(),
            path: x.path().to_path_buf(),
        }
    }
}

fn too_many_executable_candidates(
    candidates: &[ExecutableFile],
    base_directory: &Path,
) -> InstallError {
    let errors: Vec<_> = candidates
        .iter()
        .map(|x| {
            let name = x.name.to_str().unwrap_or("Unknown candidate name");
            let file_path = x.path.strip_prefix(base_directory).unwrap_or(&x.path);
            format!("{} ({})", name, file_path.display())
        })
        .collect();

    InstallError::TooManyExecutableCandidates(errors)
}

#[cfg(test)]
mod tests {
    #[cfg(target_family = "unix")]
    use std::os::unix::fs::PermissionsExt;
    use std::path::{Path, PathBuf};

    use crate::installer::{error::InstallError, Executable, InstallerResult};

    use super::ArchiveInstaller;

    #[test]
    fn single_executable() {
        let destination_dir = temp_dir("single_executable");

        let result = ArchiveInstaller::run(
            |_, temp_dir| {
                create_file(temp_dir, "README.md");
                create_file(temp_dir, "LICENSE");
                create_executable_file(temp_dir, "my-executable");
                Ok(())
            },
            &any_directory_path(),
            &destination_dir,
            &any_executable_name(),
        );

        assert_ok(result);
        assert_file_exists(executable_path(&destination_dir, "my-executable"))
    }

    #[test]
    fn no_executable() {
        let destination_dir = temp_dir("no_executable");

        let result = ArchiveInstaller::run(
            |_, temp_dir| {
                create_file(temp_dir, "README.md");
                create_file(temp_dir, "LICENSE");
                Ok(())
            },
            &any_directory_path(),
            &destination_dir,
            &any_executable_name(),
        );

        assert_no_executable(result);
    }

    #[test]
    fn executable_inside_nested_directory() {
        let destination_dir = temp_dir("executable_inside_nested_directory");

        let result = ArchiveInstaller::run(
            |_, temp_dir| {
                let nested_dir = create_dir(temp_dir, "nested");
                create_file(&nested_dir, "README.md");
                create_file(&nested_dir, "LICENSE");
                create_executable_file(&nested_dir, "my-executable");

                Ok(())
            },
            &any_directory_path(),
            &destination_dir,
            &any_executable_name(),
        );

        assert_ok(result);
        assert_file_exists(executable_path(&destination_dir, "my-executable"))
    }

    #[test]
    fn many_executable_select_preferred() {
        let destination_dir = temp_dir("many_executable_select_preferred");

        let executable_name = Executable::Selected(executable_name("mytool"));

        let result = ArchiveInstaller::run(
            |_, temp_dir| {
                create_file(temp_dir, "README.md");
                create_file(temp_dir, "LICENSE");
                create_executable_file(temp_dir, "some-random-script");
                create_executable_file(temp_dir, executable_name.name());
                create_executable_file(temp_dir, "mytool-v2");
                Ok(())
            },
            &any_directory_path(),
            &destination_dir,
            &executable_name,
        );

        assert_ok(result);
        assert_file_exists(executable_path(&destination_dir, executable_name.name()))
    }

    #[test]
    fn many_executable_no_matches() {
        let destination_dir = temp_dir("many_executable_no_matches");

        let result = ArchiveInstaller::run(
            |_, temp_dir| {
                create_file(temp_dir, "README.md");
                create_file(temp_dir, "LICENSE");
                create_executable_file(temp_dir, "some-random-script");
                create_executable_file(temp_dir, "mytool");
                create_executable_file(temp_dir, "install.sh");
                Ok(())
            },
            &any_directory_path(),
            &destination_dir,
            &any_executable_name(),
        );

        assert_too_many_candidates(vec!["some-random-script", "mytool", "install.sh"], result)
    }

    fn any_executable_name() -> Executable {
        Executable::Default(executable_name("ANY_EXECUTABLE_NAME"))
    }

    #[cfg(target_family = "unix")]
    fn create_executable_file(directory: &Path, name: &str) -> PathBuf {
        let path = executable_path(directory, name);
        std::fs::File::create(&path).unwrap();
        std::fs::set_permissions(&path, PermissionsExt::from_mode(0o755)).unwrap();
        path
    }

    #[cfg(target_os = "windows")]
    fn create_executable_file(directory: &Path, name: &str) -> PathBuf {
        let path = executable_path(directory, name);
        std::fs::File::create(&path).unwrap();
        path
    }

    fn executable_path(directory: &Path, name: &str) -> PathBuf {
        directory.join(executable_name(name))
    }

    fn executable_name(name: &str) -> String {
        if cfg!(target_os = "windows") {
            if name.ends_with(".exe") {
                name.to_string()
            } else {
                format!("{}.exe", name)
            }
        } else {
            name.to_string()
        }
    }

    fn create_file(directory: &Path, name: &str) -> PathBuf {
        let path = PathBuf::from(directory).join(name);
        std::fs::File::create(&path).unwrap();
        path
    }

    fn temp_dir(name: &str) -> PathBuf {
        let path = std::env::temp_dir().join("dra-tests").join(name);
        std::fs::create_dir_all(&path).unwrap();
        path
    }

    fn create_dir(parent: &Path, name: &str) -> PathBuf {
        let path = parent.join(name);
        std::fs::create_dir_all(&path).unwrap();
        path
    }

    fn any_directory_path() -> PathBuf {
        std::env::temp_dir().join("dra-any")
    }

    fn assert_file_exists(path: PathBuf) {
        match path.try_exists() {
            Ok(does_exists) => {
                assert!(does_exists, "File not exists: {}", path.display());
            }
            Err(e) => {
                panic!("Error checking if file '{}' exists: {}", path.display(), e);
            }
        }
    }

    fn assert_ok(result: InstallerResult) {
        assert!(result.is_ok(), "Result is Err: {:?}", result);
    }

    fn assert_no_executable(result: InstallerResult) {
        match result {
            Ok(_) => {
                panic!("No installer error");
            }
            Err(e) => match e {
                InstallError::NoExecutable => {}
                _ => {
                    panic!("Installer error is not NoExecutable: {:?}", e);
                }
            },
        }
    }

    fn assert_too_many_candidates(expected_candidates: Vec<&str>, result: InstallerResult) {
        match result {
            Ok(_) => {
                panic!("No installer error");
            }
            Err(e) => match e {
                InstallError::TooManyExecutableCandidates(candidates) => {
                    let contains_all_candidates = expected_candidates.iter().all(|expected| {
                        candidates
                            .iter()
                            .any(|candidate| candidate.contains(expected))
                    });
                    assert!(
                        contains_all_candidates,
                        "Not all expected candidates found: {:?}",
                        candidates
                    );
                }
                _ => {
                    panic!(
                        "Installer error is not TooManyExecutableCandidates: {:?}",
                        e
                    );
                }
            },
        }
    }
}
