use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::installer::archive::ArchiveInstaller;
use crate::installer::InstallerResult;

#[derive(Debug, Eq, PartialEq)]
pub enum TarKind {
    Gz,
    Xz,
    Bz2,
}

impl Display for TarKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            TarKind::Gz => f.write_str("gz"),
            TarKind::Xz => f.write_str("xz"),
            TarKind::Bz2 => f.write_str("bz2"),
        }
    }
}

pub struct TarArchiveInstaller;

impl TarArchiveInstaller {
    pub fn gz(source: &Path, destination_dir: &Path) -> InstallerResult {
        ArchiveInstaller::run(Self::extract_gz, source, destination_dir)
    }

    pub fn xz(source: &Path, destination_dir: &Path) -> InstallerResult {
        ArchiveInstaller::run(Self::extract_xz, source, destination_dir)
    }

    pub fn bz2(source: &Path, destination_dir: &Path) -> InstallerResult {
        ArchiveInstaller::run(Self::extract_bz2, source, destination_dir)
    }

    fn extract_gz(source: &Path, temp_dir: &Path) -> Result<(), String> {
        Self::extract_archive(TarKind::Gz, source, temp_dir)
    }

    fn extract_xz(source: &Path, temp_dir: &Path) -> Result<(), String> {
        Self::extract_archive(TarKind::Xz, source, temp_dir)
    }

    fn extract_bz2(source: &Path, temp_dir: &Path) -> Result<(), String> {
        Self::extract_archive(TarKind::Bz2, source, temp_dir)
    }

    fn extract_archive(kind: TarKind, source: &Path, temp_dir: &Path) -> Result<(), String> {
        let archive =
            File::open(source).map_err(|x| format!("Error opening {}: {}", source.display(), x))?;

        let stream = Self::decode(&kind, archive);
        let mut archive = tar::Archive::new(stream);

        archive
            .unpack(temp_dir)
            .map_err(|x| format!("Error extracting the {kind} archive: {}", x))
    }

    fn decode(extension: &TarKind, file: File) -> Box<dyn Read> {
        match extension {
            TarKind::Gz => Box::new(flate2::read::GzDecoder::new(file)),
            TarKind::Xz => Box::new(xz2::read::XzDecoder::new(file)),
            TarKind::Bz2 => Box::new(bzip2::read::BzDecoder::new(file)),
        }
    }
}
