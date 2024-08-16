use std::path::Path;

use compressed_file::CompressedFileInstaller;
use executable_file::ExecutableFileInstaller;
use file::SupportedFileInfo;

use crate::installer::debian::DebianInstaller;
use crate::installer::error::InstallError;
use crate::installer::tar_archive::TarArchiveInstaller;
use crate::installer::zip_archive::ZipArchiveInstaller;

use crate::installer::file::{validate_file, Compression, FileInfo, FileType};

mod archive;
pub mod cleanup;
mod command;
mod compressed_file;
mod debian;
pub mod error;
mod executable_file;
mod file;
mod tar_archive;
mod zip_archive;

#[derive(Debug)]
pub enum Executable {
    Default(String),
    Selected(String),
}

impl Executable {
    pub fn name(&self) -> &str {
        match self {
            Executable::Default(name) => name,
            Executable::Selected(name) => name,
        }
    }
}

pub fn install(
    asset_name: String,
    source: &Path,
    destination_dir: &Path,
    executable: &Executable,
) -> Result<(), InstallError> {
    let file_info = file_info_from(&asset_name, source).and_then(validate_file)?;
    let installer = find_installer_for(&file_info.file_type);

    installer(file_info, destination_dir, executable)
}

type InstallerResult = Result<(), InstallError>;

fn file_info_from(name: &str, path: &Path) -> Result<FileInfo, InstallError> {
    if !path.is_file() {
        return Err(InstallError::not_a_file(path));
    }
    Ok(FileInfo::new(name, path))
}

fn find_installer_for(
    file_type: &FileType,
) -> fn(SupportedFileInfo, &Path, &Executable) -> InstallerResult {
    match file_type {
        FileType::Debian => DebianInstaller::run,
        FileType::TarArchive(Compression::Gz) => TarArchiveInstaller::gz,
        FileType::TarArchive(Compression::Xz) => TarArchiveInstaller::xz,
        FileType::TarArchive(Compression::Bz2) => TarArchiveInstaller::bz2,
        FileType::ZipArchive => ZipArchiveInstaller::run,
        FileType::CompressedFile(Compression::Gz) => CompressedFileInstaller::gz,
        FileType::CompressedFile(Compression::Xz) => CompressedFileInstaller::xz,
        FileType::CompressedFile(Compression::Bz2) => CompressedFileInstaller::bz2,
        FileType::ExecutableFile => ExecutableFileInstaller::run,
    }
}
