use std::path::Path;

use crate::installer::debian::DebianInstaller;
use crate::installer::error::InstallError;
use crate::installer::tar_archive::TarArchiveInstaller;
use crate::installer::zip_archive::ZipArchiveInstaller;

use crate::installer::file::{validate_file, FileInfo, FileType, TarKind};

mod archive;
pub mod cleanup;
mod command;
mod debian;
pub mod error;
mod file;
mod tar_archive;
mod zip_archive;

pub fn install(
    asset_name: String,
    source: &Path,
    destination_dir: &Path,
    executable_name: &str,
) -> Result<(), InstallError> {
    let file_info = file_info_from(&asset_name, source).and_then(validate_file)?;
    let installer = find_installer_for(&file_info.file_type);

    installer(&file_info.path, destination_dir, executable_name)
}

type InstallerResult = Result<(), InstallError>;

fn file_info_from(name: &str, path: &Path) -> Result<FileInfo, InstallError> {
    if !path.is_file() {
        return Err(InstallError::not_a_file(path));
    }
    Ok(FileInfo::new(name, path))
}

fn find_installer_for(file_type: &FileType) -> fn(&Path, &Path, &str) -> InstallerResult {
    match file_type {
        FileType::Debian => DebianInstaller::run,
        FileType::TarArchive(TarKind::Gz) => TarArchiveInstaller::gz,
        FileType::TarArchive(TarKind::Xz) => TarArchiveInstaller::xz,
        FileType::TarArchive(TarKind::Bz2) => TarArchiveInstaller::bz2,
        FileType::ZipArchive => ZipArchiveInstaller::run,
    }
}
