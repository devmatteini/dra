use std::path::Path;

use crate::installer::error::InstallError;

pub trait InstallCleanup {
    fn cleanup(self, path: &Path) -> Result<(), InstallError>;
}

impl InstallCleanup for Result<(), InstallError> {
    /// If the installation succeeds then the asset can be deleted from the filesystem
    fn cleanup(self, path: &Path) -> Result<(), InstallError> {
        self.and_then(|_| {
            std::fs::remove_file(path).map_err(|x| {
                InstallError::Fatal(format!("Unable to delete installed asset: {}", x))
            })
        })
    }
}
