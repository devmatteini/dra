use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::installer::archive::ArchiveInstaller;
use crate::installer::InstallerResult;

pub struct TarArchiveInstaller;

impl TarArchiveInstaller {
    pub fn gz(source: &Path, destination_dir: &Path, executable_name: &str) -> InstallerResult {
        ArchiveInstaller::run(Self::extract_gz, source, destination_dir, executable_name)
    }

    pub fn xz(source: &Path, destination_dir: &Path, executable_name: &str) -> InstallerResult {
        ArchiveInstaller::run(Self::extract_xz, source, destination_dir, executable_name)
    }

    pub fn bz2(source: &Path, destination_dir: &Path, executable_name: &str) -> InstallerResult {
        ArchiveInstaller::run(Self::extract_bz2, source, destination_dir, executable_name)
    }

    fn extract_gz(source: &Path, temp_dir: &Path) -> Result<(), String> {
        Self::extract_archive(
            |file| Box::new(flate2::read::GzDecoder::new(file)),
            source,
            temp_dir,
        )
    }

    fn extract_xz(source: &Path, temp_dir: &Path) -> Result<(), String> {
        Self::extract_archive(
            |file| Box::new(xz2::read::XzDecoder::new(file)),
            source,
            temp_dir,
        )
    }

    fn extract_bz2(source: &Path, temp_dir: &Path) -> Result<(), String> {
        Self::extract_archive(
            |file| Box::new(bzip2::read::BzDecoder::new(file)),
            source,
            temp_dir,
        )
    }

    fn extract_archive<D>(decode: D, source: &Path, temp_dir: &Path) -> Result<(), String>
    where
        D: FnOnce(File) -> Box<dyn Read>,
    {
        let archive =
            File::open(source).map_err(|x| format!("Error opening {}: {}", source.display(), x))?;

        let stream = decode(archive);
        let mut archive = tar::Archive::new(stream);

        archive
            .unpack(temp_dir)
            .map_err(|x| format!("Error extracting the archive: {}", x))
    }
}
