use std::ffi::OsString;
#[cfg(target_family = "unix")]
use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::installer::error::MapErrWithMessage;
use crate::installer::InstallerResult;

pub struct ArchiveInstaller;

impl ArchiveInstaller {
    pub fn run<F>(extract_files: F, source: &Path, destination_dir: &Path) -> InstallerResult
    where
        F: FnOnce(&Path, &Path) -> Result<(), String>,
    {
        let temp_dir = Self::create_temp_dir()?;
        extract_files(source, &temp_dir)?;

        let executable = Self::find_executable(&temp_dir)?;

        Self::move_executable_to_destination_dir(executable, destination_dir)?;
        Self::cleanup(&temp_dir)?;

        Ok(())
    }

    fn create_temp_dir() -> Result<PathBuf, String> {
        let temp_dir = crate::cli::temp_file::temp_dir();
        std::fs::create_dir(&temp_dir)
            .map(|_| temp_dir)
            .map_err_with("Error creating temp dir".into())
    }

    fn find_executable(directory: &Path) -> Result<ExecutableFile, String> {
        let ignore_error = |result: walkdir::Result<walkdir::DirEntry>| result.ok();

        WalkDir::new(directory)
            .max_depth(2)
            .into_iter()
            .filter_map(ignore_error)
            .find(Self::is_executable)
            .ok_or_else(|| String::from("No executable found"))
            .map(ExecutableFile::from_file)
    }

    fn is_executable(x: &walkdir::DirEntry) -> bool {
        let path = x.path();

        path.metadata()
            .map(|metadata| path.is_file() && is_executable_file(path, metadata))
            .unwrap_or(false)
    }

    fn move_executable_to_destination_dir(
        executable: ExecutableFile,
        destination_dir: &Path,
    ) -> Result<(), String> {
        let mut to = PathBuf::from(destination_dir);
        to.push(executable.name);
        std::fs::rename(&executable.path, &to)
            .map(|_| ())
            .map_err_with(format!(
                "Error moving {} to {}",
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
                create_file("README.md", temp_dir);
                create_file("LICENSE", temp_dir);
                create_executable_file("my-executable", temp_dir);
                Ok(())
            },
            &any_directory_path(),
            &destination_dir,
        );

        assert_ok(result);
        // TODO: should we check if the executable has been moved to destination_dir?
    }

    #[test]
    fn no_executable() {
        let destination_dir = temp_dir("no_executable");

        let result = ArchiveInstaller::run(
            |_, temp_dir| {
                create_file("README.md", temp_dir);
                create_file("LICENSE", temp_dir);
                Ok(())
            },
            &any_directory_path(),
            &destination_dir,
        );

        assert_err_equal("No executable found", result);
    }

    #[test]
    fn executable_inside_nested_directory() {
        let destination_dir = temp_dir("no_executable");

        let result = ArchiveInstaller::run(
            |_, temp_dir| {
                let nested_dir = create_dir("nested", temp_dir);
                create_file("README.md", &nested_dir);
                create_file("LICENSE", &nested_dir);
                create_executable_file("my-executable", &nested_dir);

                Ok(())
            },
            &any_directory_path(),
            &destination_dir,
        );

        assert_ok(result);
        // TODO: should we check if the executable has been moved to destination_dir?
    }

    #[cfg(target_family = "unix")]
    fn create_executable_file(name_without_extension: &str, directory: &Path) -> PathBuf {
        let path = PathBuf::from(directory).join(name_without_extension);
        std::fs::File::create(&path).unwrap();
        std::fs::set_permissions(&path, PermissionsExt::from_mode(0o755)).unwrap();
        path
    }

    #[cfg(target_os = "windows")]
    fn create_executable_file(name_without_extension: &str, directory: &Path) -> PathBuf {
        let path = PathBuf::from(directory).join(format!("{}.exe", name_without_extension));
        std::fs::File::create(&path).unwrap();
        path
    }

    fn create_file(name: &str, directory: &Path) -> PathBuf {
        let path = PathBuf::from(directory).join(name);
        std::fs::File::create(&path).unwrap();
        path
    }

    fn temp_dir(name: &str) -> PathBuf {
        let path = std::env::temp_dir().join("dra-tests").join(name);
        std::fs::create_dir_all(&path).unwrap();
        path
    }

    fn create_dir(name: &str, parent: &Path) -> PathBuf {
        let path = parent.join(name);
        std::fs::create_dir_all(&path).unwrap();
        path
    }

    fn any_directory_path() -> PathBuf {
        std::env::temp_dir().join("dra-any")
    }

    fn assert_ok(result: InstallerResult) {
        assert_eq!(Ok(()), result);
    }

    fn assert_err_equal(to_contain: &str, result: InstallerResult) {
        match result {
            Ok(_) => {
                panic!("No installer error");
            }
            Err(e) => {
                assert_eq!(e, to_contain);
            }
        }
    }
}
