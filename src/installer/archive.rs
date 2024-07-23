use std::ffi::OsString;
#[cfg(target_family = "unix")]
use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::installer::error::MapErrWithMessage;
use crate::installer::InstallerResult;

pub struct ArchiveInstaller;

impl ArchiveInstaller {
    pub fn run<F>(
        extract_files: F,
        source: &Path,
        destination_dir: &Path,
        executable_name: &str,
    ) -> InstallerResult
    where
        F: FnOnce(&Path, &Path) -> Result<(), String>,
    {
        let temp_dir = Self::create_temp_dir()?;
        extract_files(source, &temp_dir)?;

        let executable = Self::find_executable(&temp_dir, executable_name)?;

        Self::copy_executable_to_destination_dir(executable, destination_dir)?;
        Self::cleanup(&temp_dir)?;

        Ok(())
    }

    fn create_temp_dir() -> Result<PathBuf, String> {
        let temp_dir = crate::cli::temp_file::temp_dir();
        std::fs::create_dir(&temp_dir)
            .map(|_| temp_dir)
            .map_err_with("Error creating temp dir".into())
    }

    fn find_executable(directory: &Path, executable_name: &str) -> Result<ExecutableFile, String> {
        let ignore_error = |result: walkdir::Result<walkdir::DirEntry>| result.ok();

        let executables: Vec<_> = WalkDir::new(directory)
            .max_depth(2)
            .into_iter()
            .filter_map(ignore_error)
            .filter(Self::is_executable)
            .map(ExecutableFile::from_file)
            .collect();

        let preferred = executables.iter().find(|x| x.name == executable_name);
        if let Some(executable) = preferred {
            return Ok(executable.clone());
        }

        executables
            .into_iter()
            .next()
            .ok_or_else(|| String::from("No executable found"))
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
    ) -> Result<(), String> {
        let mut to = PathBuf::from(destination_dir);
        to.push(executable.name);
        std::fs::copy(&executable.path, &to)
            .map(|_| ())
            .map_err_with(format!(
                "Error copying {} to {}",
                &executable.path.display(),
                to.display(),
            ))
    }

    fn cleanup(temp_dir: &Path) -> Result<(), String> {
        std::fs::remove_dir_all(temp_dir).map_err_with("Error deleting temp dir".into())
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

#[cfg(test)]
mod tests {
    #[cfg(target_family = "unix")]
    use std::os::unix::fs::PermissionsExt;
    use std::path::{Path, PathBuf};

    use crate::installer::InstallerResult;

    use super::ArchiveInstaller;

    #[test]
    fn executable_found() {
        let destination_dir = temp_dir("executable_found");

        let result = ArchiveInstaller::run(
            |_, temp_dir| {
                create_file(temp_dir, "README.md");
                create_file(temp_dir, "LICENSE");
                create_executable_file(temp_dir, "my-executable");
                Ok(())
            },
            &any_directory_path(),
            &destination_dir,
            ANY_EXECUTABLE_NAME,
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
            ANY_EXECUTABLE_NAME,
        );

        assert_err_equal("No executable found", result);
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
            ANY_EXECUTABLE_NAME,
        );

        assert_ok(result);
        assert_file_exists(executable_path(&destination_dir, "my-executable"))
    }

    #[test]
    fn many_executable_select_preferred() {
        let destination_dir = temp_dir("many_executable_select_preferred");

        let executable_name = "mytool";

        let result = ArchiveInstaller::run(
            |_, temp_dir| {
                create_file(temp_dir, "README.md");
                create_file(temp_dir, "LICENSE");
                create_executable_file(temp_dir, "some-random-script");
                create_executable_file(temp_dir, executable_name);
                create_executable_file(temp_dir, "mytool-v2");
                Ok(())
            },
            &any_directory_path(),
            &destination_dir,
            executable_name,
        );

        assert_ok(result);
        assert_file_exists(executable_path(&destination_dir, executable_name))
    }

    const ANY_EXECUTABLE_NAME: &str = "ANY_EXECUTABLE_NAME";

    #[cfg(target_family = "unix")]
    fn create_executable_file(directory: &Path, name_without_extension: &str) -> PathBuf {
        let path = executable_path(directory, name_without_extension);
        std::fs::File::create(&path).unwrap();
        std::fs::set_permissions(&path, PermissionsExt::from_mode(0o755)).unwrap();
        path
    }

    #[cfg(target_os = "windows")]
    fn create_executable_file(directory: &Path, name_without_extension: &str) -> PathBuf {
        let path = executable_path(directory, name_without_extension);
        std::fs::File::create(&path).unwrap();
        path
    }

    fn executable_path(directory: &Path, name_without_extension: &str) -> PathBuf {
        if cfg!(target_os = "windows") {
            directory.join(format!("{}.exe", name_without_extension))
        } else {
            directory.join(name_without_extension)
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
        assert_eq!(Ok(()), result);
    }

    fn assert_err_equal(expected: &str, result: InstallerResult) {
        match result {
            Ok(_) => {
                panic!("No installer error");
            }
            Err(e) => {
                assert_eq!(expected, e);
            }
        }
    }
}
