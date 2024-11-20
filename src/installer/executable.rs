use crate::installer::error::InstallError;
use std::path::Path;

#[derive(Debug, Clone)]
pub enum Executable {
    Automatic(String),
    Selected(String),
}

impl Executable {
    pub fn name(&self) -> String {
        match self {
            Executable::Automatic(name) => name.clone(),
            Executable::Selected(name) => name.clone(),
        }
    }
}

#[cfg(target_family = "unix")]
pub fn set_executable_permissions(path: &Path) -> Result<(), InstallError> {
    use crate::installer::error::InstallErrorMapErr;
    use std::os::unix::fs::PermissionsExt;

    std::fs::set_permissions(path, PermissionsExt::from_mode(0o755)).map_fatal_err(format!(
        "Cannot set executable permissions on {}",
        path.display(),
    ))
}

#[cfg(target_os = "windows")]
pub fn set_executable_permissions(_path: &Path) -> Result<(), InstallError> {
    Ok(())
}
