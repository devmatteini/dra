use crate::installer::destination::Destination;
use crate::installer::error::{
    ArchiveError, ArchiveErrorType, ArchiveInstallerError, InstallError, InstallErrorMapErr,
};
use crate::installer::executable::Executable;
use crate::installer::file::SupportedFileInfo;
use crate::installer::result::{InstallOutput, InstallerResult};
use itertools::{Either, Itertools};
use std::ffi::OsString;
#[cfg(target_family = "unix")]
use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct ArchiveInstaller;

impl ArchiveInstaller {
    pub fn run<F>(
        extract_files: F,
        file_info: SupportedFileInfo,
        destination: Destination,
        executables_to_install: Vec<Executable>,
    ) -> InstallerResult
    where
        F: FnOnce(&Path, &Path) -> Result<(), InstallError>,
    {
        let temp_dir = Self::create_temp_dir()?;
        extract_files(&file_info.path, &temp_dir)?;

        let all_executables = Self::find_all_executables(&temp_dir)?;

        let results = executables_to_install
            .into_iter()
            .map(|executable| {
                Self::find_executable(&temp_dir, &executable, &all_executables)
                    .and_then(|executable| {
                        Self::copy_executable_to_destination(executable, &destination)
                    })
                    .map_err(|error| ArchiveError(executable.name(), error))
                    .map(|destination_path| {
                        format!(
                            "Extracted archive executable to '{}'",
                            destination_path.display()
                        )
                    })
            })
            .collect::<Vec<_>>();

        let (successes, failures): (Vec<_>, Vec<_>) =
            results.into_iter().partition_map(|result| match result {
                Ok(x) => Either::Left(x),
                Err(x) => Either::Right(x),
            });

        if !failures.is_empty() {
            return Err(InstallError::Archive(ArchiveInstallerError {
                successes,
                failures,
            }));
        }

        Self::cleanup(&temp_dir)?;

        Ok(InstallOutput::new(successes.join("\n").to_string()))
    }

    fn create_temp_dir() -> Result<PathBuf, InstallError> {
        crate::temp_file::make_temp_dir().map_fatal_err("Error creating temp dir".into())
    }

    fn find_all_executables(directory: &Path) -> Result<Vec<ExecutableFile>, InstallError> {
        let ignore_error = |result: walkdir::Result<walkdir::DirEntry>| result.ok();

        let executables: Vec<_> = WalkDir::new(directory)
            .max_depth(3)
            .into_iter()
            .filter_map(ignore_error)
            .filter(Self::is_executable)
            .map(ExecutableFile::from_file)
            .collect();

        if executables.is_empty() {
            return Err(InstallError::NoExecutable);
        }

        // TODO: it would be nice to have a NonEmptyVec
        Ok(executables)
    }

    fn find_executable(
        directory: &Path,
        executable: &Executable,
        executables: &[ExecutableFile],
    ) -> Result<ExecutableFile, ArchiveErrorType> {
        match executable {
            Executable::Automatic(name) => Self::discover_executable(executables, name, directory),
            Executable::Selected(name) => Self::find_selected_executable(executables, name),
        }
    }

    fn discover_executable(
        executables: &[ExecutableFile],
        executable_name: &str,
        directory: &Path,
    ) -> Result<ExecutableFile, ArchiveErrorType> {
        let default_executable = executables.iter().find(|x| x.name == executable_name);
        if let Some(executable) = default_executable {
            return Ok(executable.clone());
        }

        match executables {
            [x] => Ok(x.clone()),
            candidates => Err(too_many_executable_candidates(candidates, directory)),
        }
    }

    fn find_selected_executable(
        executables: &[ExecutableFile],
        executable_name: &str,
    ) -> Result<ExecutableFile, ArchiveErrorType> {
        executables
            .into_iter()
            .find(|x| x.name == executable_name)
            .map(|x| x.clone())
            .ok_or_else(|| ArchiveErrorType::ExecutableNotFound)
    }

    fn is_executable(x: &walkdir::DirEntry) -> bool {
        let path = x.path();

        path.metadata()
            .map(|metadata| path.is_file() && is_executable_file(path, metadata))
            .unwrap_or(false)
    }

    fn copy_executable_to_destination(
        executable: ExecutableFile,
        destination: &Destination,
    ) -> Result<PathBuf, ArchiveErrorType> {
        let to = match destination {
            Destination::Directory(dir) => dir.join(executable.name),
            Destination::File(file) => file.clone(),
        };

        std::fs::copy(&executable.path, &to)
            .map(|_| ())
            .map_err(|e| {
                ArchiveErrorType::CopyExecutable(executable.path.clone(), to.clone(), e.to_string())
            })?;

        Ok(to)
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
) -> ArchiveErrorType {
    let errors: Vec<_> = candidates
        .iter()
        .map(|x| {
            let name = x.name.to_str().unwrap_or("Unknown candidate name");
            let file_path = x.path.strip_prefix(base_directory).unwrap_or(&x.path);
            format!("{} ({})", name, file_path.display())
        })
        .collect();

    ArchiveErrorType::TooManyExecutableCandidates(errors)
}

#[cfg(test)]
mod tests {
    #[cfg(target_family = "unix")]
    use std::os::unix::fs::PermissionsExt;
    use std::path::{Path, PathBuf};

    use super::ArchiveInstaller;
    use crate::installer::destination::Destination;
    use crate::installer::error::{ArchiveError, ArchiveErrorType, ArchiveInstallerError};
    use crate::installer::executable::Executable;
    use crate::installer::result::InstallerResult;
    use crate::installer::{
        error::InstallError,
        file::{FileType, SupportedFileInfo},
    };

    #[test]
    fn automatic_executable_with_default_name() {
        let destination_dir = temp_dir("automatic_executable_with_default_name");
        let destination = Destination::Directory(destination_dir.clone());
        let executable = Executable::Automatic(executable_name("my-tool"));

        let result = ArchiveInstaller::run(
            |_, temp_dir| {
                create_file(temp_dir, "README.md");
                create_file(temp_dir, "LICENSE");
                create_executable_file(temp_dir, "another-tool");
                create_executable_file(temp_dir, "my-tool");
                Ok(())
            },
            any_file_info(),
            destination,
            vec![executable],
        );

        assert_ok(result);
        assert_file_exists(executable_path(&destination_dir, "my-tool"))
    }

    #[test]
    fn automatic_executable_with_single_executable() {
        let destination_dir = temp_dir("automatic_executable_with_single_executable");
        let destination = Destination::Directory(destination_dir.clone());
        let executable = Executable::Automatic(executable_name("long-tool-name"));

        let result = ArchiveInstaller::run(
            |_, temp_dir| {
                create_file(temp_dir, "README.md");
                create_file(temp_dir, "LICENSE");
                create_executable_file(temp_dir, "ltn");
                Ok(())
            },
            any_file_info(),
            destination,
            vec![executable],
        );

        assert_ok(result);
        assert_file_exists(executable_path(&destination_dir, "ltn"))
    }

    #[test]
    fn archive_with_no_executable() {
        let destination_dir = temp_dir("automatic_executable_with_no_executable");
        let destination = Destination::Directory(destination_dir.clone());
        let executable = Executable::Automatic(executable_name("my-tool"));

        let result = ArchiveInstaller::run(
            |_, temp_dir| {
                create_file(temp_dir, "README.md");
                create_file(temp_dir, "LICENSE");
                Ok(())
            },
            any_file_info(),
            destination,
            vec![executable],
        );

        assert_no_executable(result);
    }

    #[test]
    fn automatic_executable_with_many_executable_candidates() {
        let destination_dir = temp_dir("automatic_executable_with_many_executable_candidates");
        let destination = Destination::Directory(destination_dir.clone());
        let executable = Executable::Automatic(executable_name("my-tool"));

        let result = ArchiveInstaller::run(
            |_, temp_dir| {
                create_file(temp_dir, "README.md");
                create_file(temp_dir, "LICENSE");
                create_executable_file(temp_dir, "some-random-script");
                create_executable_file(temp_dir, "mytool");
                create_executable_file(temp_dir, "install.sh");
                Ok(())
            },
            any_file_info(),
            destination,
            vec![executable],
        );

        assert_too_many_candidates(vec!["some-random-script", "mytool", "install.sh"], result)
    }

    #[test]
    fn all_selected_executables_found() {
        let destination_dir = temp_dir("all_selected_executables_found");
        let destination = Destination::Directory(destination_dir.clone());
        let mytool = executable_name("mytool");
        let mytool2 = executable_name("mytool2");
        let mytool3 = executable_name("mytool3");

        let result = ArchiveInstaller::run(
            |_, temp_dir| {
                create_file(temp_dir, "README.md");
                create_file(temp_dir, "LICENSE");
                create_executable_file(temp_dir, "some-random-script");
                create_executable_file(temp_dir, "mytool");
                create_executable_file(temp_dir, "mytool2");
                create_executable_file(temp_dir, "mytool3");
                Ok(())
            },
            any_file_info(),
            destination,
            vec![
                Executable::Selected(mytool.clone()),
                Executable::Selected(mytool2.clone()),
                Executable::Selected(mytool3.clone()),
            ],
        );

        assert_ok(result);
        assert_file_exists(executable_path(&destination_dir, &mytool));
        assert_file_exists(executable_path(&destination_dir, &mytool2));
        assert_file_exists(executable_path(&destination_dir, &mytool3));
    }

    #[test]
    fn all_selected_executables_not_found() {
        let destination_dir = temp_dir("all_selected_executables_not_found");
        let destination = Destination::Directory(destination_dir.clone());
        let mytool = executable_name("mytool");
        let mytool2 = executable_name("mytool2");
        let mytool3 = executable_name("mytool3");

        let result = ArchiveInstaller::run(
            |_, temp_dir| {
                create_file(temp_dir, "README.md");
                create_file(temp_dir, "LICENSE");
                create_executable_file(temp_dir, "some-random-script");
                create_executable_file(temp_dir, "my-tool");
                create_executable_file(temp_dir, "my-tool-v2");
                Ok(())
            },
            any_file_info(),
            destination,
            vec![
                Executable::Selected(mytool.clone()),
                Executable::Selected(mytool2.clone()),
                Executable::Selected(mytool3.clone()),
            ],
        );

        let error = assert_archive_error(result);

        let expected = ArchiveInstallerError {
            successes: vec![],
            failures: vec![
                ArchiveError(mytool, ArchiveErrorType::ExecutableNotFound),
                ArchiveError(mytool2, ArchiveErrorType::ExecutableNotFound),
                ArchiveError(mytool3, ArchiveErrorType::ExecutableNotFound),
            ],
        };
        assert_eq!(expected, error);
    }

    #[test]
    fn some_selected_executables_not_found() {
        let destination_dir = temp_dir("some_selected_executables_not_found");
        let destination = Destination::Directory(destination_dir.clone());
        let mytool = executable_name("mytool");
        let mytool2 = executable_name("mytool2");
        let mytool3 = executable_name("mytool3");
        let mytool4 = executable_name("mytool4");

        let result = ArchiveInstaller::run(
            |_, temp_dir| {
                create_file(temp_dir, "README.md");
                create_file(temp_dir, "LICENSE");
                create_executable_file(temp_dir, "mytool");
                create_executable_file(temp_dir, "mytool3");
                Ok(())
            },
            any_file_info(),
            destination,
            vec![
                Executable::Selected(mytool.clone()),
                Executable::Selected(mytool2.clone()),
                Executable::Selected(mytool3.clone()),
                Executable::Selected(mytool4.clone()),
            ],
        );

        let error = assert_archive_error(result);
        // TODO: maybe we can find a way to check that mytool and mytool3 are present
        assert_eq!(2, error.successes.len());
        assert_eq!(
            vec![
                ArchiveError(mytool2, ArchiveErrorType::ExecutableNotFound),
                ArchiveError(mytool4, ArchiveErrorType::ExecutableNotFound)
            ],
            error.failures
        );
    }

    #[test]
    fn executable_inside_nested_directory() {
        let destination_dir = temp_dir("executable_inside_nested_directory");
        let destination = Destination::Directory(destination_dir.clone());

        let result = ArchiveInstaller::run(
            |_, temp_dir| {
                let nested_dir = create_dir(temp_dir, "nested");
                create_file(&nested_dir, "README.md");
                create_file(&nested_dir, "LICENSE");
                create_executable_file(&nested_dir, "my-executable");
                Ok(())
            },
            any_file_info(),
            destination,
            vec![any_automatic_executable_name()],
        );

        assert_ok(result);
        assert_file_exists(executable_path(&destination_dir, "my-executable"))
    }

    fn any_automatic_executable_name() -> Executable {
        Executable::Automatic(executable_name("ANY_EXECUTABLE_NAME"))
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

    fn any_file_info() -> SupportedFileInfo {
        SupportedFileInfo {
            name: "any-name".to_string(),
            path: any_directory_path(),
            file_type: FileType::TarArchive(crate::installer::file::Compression::Gz),
        }
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
        let error = assert_archive_error(result);

        assert_eq!(
            1,
            error.failures.len(),
            "More than one failure, only expected one"
        );
        let archive_error = error.failures.into_iter().next().unwrap();

        match archive_error.1 {
            ArchiveErrorType::TooManyExecutableCandidates(candidates) => {
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
            x => panic!("Expected TooManyExecutableCandidates, got {:?}", x),
        }
    }

    fn assert_archive_error(result: InstallerResult) -> ArchiveInstallerError {
        if result.is_ok() {
            panic!("No installer error");
        }

        match result.err().unwrap() {
            InstallError::Archive(error) => error,
            x => panic!("Expected InstallError::Archive, got {:?}", x),
        }
    }
}
