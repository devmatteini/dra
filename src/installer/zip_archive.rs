use std::fs::File;
use std::path::Path;

use crate::installer::archive::ArchiveInstaller;
use crate::installer::InstallerResult;

use super::error::InstallError;
use super::file::SupportedFileInfo;
use super::{Destination, Executable};

pub struct ZipArchiveInstaller;

impl ZipArchiveInstaller {
    pub fn run(
        file_info: SupportedFileInfo,
        destination_dir: &Path,
        executable: &Executable,
        destination: Destination,
    ) -> InstallerResult {
        ArchiveInstaller::run(
            Self::extract_archive,
            file_info,
            destination_dir,
            executable,
            destination,
        )
    }

    fn extract_archive(source: &Path, temp_dir: &Path) -> Result<(), InstallError> {
        let zip_archive = File::open(source).map_err(|x| {
            InstallError::Fatal(format!("Error opening {}: {}", source.display(), x))
        })?;

        let mut archive = zip::ZipArchive::new(zip_archive)
            .map_err(|x| InstallError::Fatal(format!("Error opening zip archive: {}", x)))?;

        archive
            .extract(temp_dir)
            .map_err(|x| InstallError::Fatal(format!("Error extracting the zip archive: {}", x)))
    }
}
