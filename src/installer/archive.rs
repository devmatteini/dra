use std::ffi::OsString;
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
        let is_executable =
            |metadata: std::fs::Metadata| (metadata.permissions().mode() & 0o111) != 0;

        path.metadata()
            .map(|metadata| path.is_file() && is_executable(metadata))
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
