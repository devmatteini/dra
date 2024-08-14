use crate::installer::InstallerResult;
use std::path::Path;

use super::{
    error::{InstallError, InstallErrorMapErr},
    Executable,
};

pub struct ExecutableFileInstaller;

impl ExecutableFileInstaller {
    pub fn run(source: &Path, destination_dir: &Path, executable: &Executable) -> InstallerResult {
        let executable_path = destination_dir.join(executable.name());

        std::fs::copy(source, &executable_path)
            .map_fatal_err(format!("Error copying {}", executable_path.display()))?;

        set_executable_permissions(&executable_path)?;

        Ok(())
    }
}

#[cfg(target_family = "unix")]
fn set_executable_permissions(path: &Path) -> Result<(), InstallError> {
    use std::os::unix::fs::PermissionsExt;

    std::fs::set_permissions(path, PermissionsExt::from_mode(0o755)).map_fatal_err(format!(
        "Cannot set executable permissions on {}",
        path.display(),
    ))
}

#[cfg(target_os = "windows")]
fn set_executable_permissions(path: &Path) -> Result<(), InstallError> {
    Ok(())
}
