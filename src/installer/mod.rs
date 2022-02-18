use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::installer::debian::DebianInstaller;
use crate::installer::error::InstallError;
use crate::installer::tar_archive::TarArchiveInstaller;
use crate::installer::zip_archive::ZipArchiveInstaller;

pub mod cleanup;
mod command;
mod debian;
pub mod error;
mod tar_archive;
mod zip_archive;

pub fn install(
    asset_name: String,
    source: &Path,
    destination_dir: &Path,
) -> Result<(), InstallError> {
    let file_info = file_info_from(&asset_name, source).and_then(is_supported)?;
    let installer = find_installer_for(&file_info.file_type);

    installer(&file_info.path, destination_dir).map_err(InstallError::Fatal)?;

    Ok(())
}

type InstallerResult = Result<(), String>;

#[derive(Debug, Eq, PartialEq)]
enum FileType {
    Debian,
    TarArchive,
    ZipArchive,
}

#[derive(Debug)]
struct FileInfo {
    path: PathBuf,
    name: String,
    extension: Option<OsString>,
}

#[derive(Debug)]
struct SupportedFileInfo {
    path: PathBuf,
    file_type: FileType,
}

fn file_info_from(name: &str, path: &Path) -> Result<FileInfo, InstallError> {
    if !path.is_file() {
        return Err(InstallError::not_a_file(path));
    }

    Ok(FileInfo {
        path: PathBuf::from(path),
        name: String::from(name),
        extension: Path::new(name).extension().map(OsString::from),
    })
}

fn is_supported(file: FileInfo) -> Result<SupportedFileInfo, InstallError> {
    file.extension
        .and_then(file_type_for)
        .map(|file_type| SupportedFileInfo {
            path: PathBuf::from(&file.path),
            file_type,
        })
        .ok_or_else(|| InstallError::not_supported(&file.name))
}

fn file_type_for(extension: OsString) -> Option<FileType> {
    if extension == "deb" {
        return Some(FileType::Debian);
    }
    // TODO: add support for this archives as well: tar, bz2, xz
    if extension == "gz" {
        return Some(FileType::TarArchive);
    }
    if extension == "zip" {
        return Some(FileType::ZipArchive);
    }

    None
}

fn find_installer_for(file_type: &FileType) -> fn(&Path, &Path) -> InstallerResult {
    match file_type {
        FileType::Debian => DebianInstaller::run,
        FileType::TarArchive => TarArchiveInstaller::run,
        FileType::ZipArchive => ZipArchiveInstaller::run,
    }
}
