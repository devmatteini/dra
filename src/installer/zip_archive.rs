use std::ffi::OsString;
use std::fs::DirEntry;
use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::installer::command::exec_command;
use crate::installer::InstallerResult;

const UNZIP: &str = "unzip";

pub struct ZipArchiveInstaller;

impl ZipArchiveInstaller {
    pub fn run(source: &Path, destination_dir: &Path) -> InstallerResult {
        let temp_dir = Self::create_temp_dir()?;
        Self::extract_archive(source, &temp_dir)?;

        let executable = Self::find_executable(&temp_dir)?;

        Self::copy_executable_to_destination_dir(executable, destination_dir)?;
        Self::cleanup(&temp_dir)?;

        Ok(())
    }

    fn create_temp_dir() -> Result<PathBuf, String> {
        let temp_dir = crate::cli::temp_file::temp_dir();
        std::fs::create_dir(&temp_dir)
            .map(|_| temp_dir)
            .map_err(|x| format!("Error creating temp dir: {}", x))
    }

    fn extract_archive(source: &Path, temp_dir: &Path) -> Result<(), String> {
        exec_command(
            UNZIP,
            Command::new(UNZIP)
                // Remove the root dir inside the zip archive. see man unzip
                .arg("-j")
                .arg(source)
                .arg("-d")
                .arg(temp_dir),
        )
    }

    fn find_executable(directory: &Path) -> Result<ExecutableFile, String> {
        std::fs::read_dir(directory)
            .map_err(|x| format!("Error reading files in {}: {}", directory.display(), x))?
            .find(Self::is_executable)
            .ok_or_else(|| String::from("No executable found"))?
            .map(ExecutableFile::from_file)
            .map_err(|e| format!("Cannot read file information: {}", e))
    }

    fn is_executable(entry: &std::io::Result<DirEntry>) -> bool {
        entry
            .as_ref()
            .map(|x| {
                let path = x.path();
                path.metadata()
                    .map(|metadata| path.is_file() && (metadata.permissions().mode() & 0o111) != 0)
                    .unwrap_or(false)
            })
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
            .map_err(|x| {
                format!(
                    "Error copying {} to {}: {}",
                    &executable.path.display(),
                    to.display(),
                    x
                )
            })
    }

    fn cleanup(temp_dir: &Path) -> Result<(), String> {
        std::fs::remove_dir_all(temp_dir).map_err(|x| format!("Error deleting temp dir: {}", x))
    }
}

struct ExecutableFile {
    pub path: PathBuf,
    pub name: OsString,
}

impl ExecutableFile {
    fn from_file(x: DirEntry) -> Self {
        Self {
            path: x.path(),
            name: x.file_name(),
        }
    }
}
