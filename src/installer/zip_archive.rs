use std::fs::File;
use std::path::Path;

use crate::installer::archive::ArchiveInstaller;
use crate::installer::InstallerResult;

pub struct ZipArchiveInstaller;

impl ZipArchiveInstaller {
    pub fn run(source: &Path, destination_dir: &Path, executable_name: &str) -> InstallerResult {
        ArchiveInstaller::run(
            Self::extract_archive,
            source,
            destination_dir,
            executable_name,
        )
    }

    fn extract_archive(source: &Path, temp_dir: &Path) -> Result<(), String> {
        let zip_archive =
            File::open(source).map_err(|x| format!("Error opening {}: {}", source.display(), x))?;

        let mut archive = zip::ZipArchive::new(zip_archive)
            .map_err(|x| format!("Error opening zip archive: {}", x))?;

        archive
            .extract(temp_dir)
            .map_err(|x| format!("Error extracting the zip archive: {}", x))
    }
}
