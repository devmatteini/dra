use std::fs::File;
use std::path::Path;

use crate::installer::archive_installer::ArchiveInstaller;
use crate::installer::destination::Destination;
use crate::installer::error::InstallError;
use crate::installer::executable::Executable;
use crate::installer::file::SupportedFileInfo;
use crate::installer::result::InstallerResult;

pub struct ZipArchiveInstaller;

impl ZipArchiveInstaller {
    pub fn run(
        file_info: SupportedFileInfo,
        destination: Destination,
        executables: Vec<Executable>,
    ) -> InstallerResult {
        ArchiveInstaller::run(Self::extract_archive, file_info, destination, executables)
    }

    fn extract_archive(source: &Path, temp_dir: &Path) -> Result<(), InstallError> {
        let zip_archive = File::open(source).map_err(|x| {
            InstallError::Fatal(format!(
                "Error opening zip archive {}: {}",
                source.display(),
                x
            ))
        })?;

        let mut archive = zip::ZipArchive::new(zip_archive)
            .map_err(|x| InstallError::Fatal(format!("Error reading zip archive: {}", x)))?;

        archive
            .extract(temp_dir)
            .map_err(|x| InstallError::Fatal(format!("Error extracting zip archive: {}", x)))
    }
}
