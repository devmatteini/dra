use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::installer::debian::DebianInstaller;
use crate::installer::error::InstallError;

pub mod cleanup;
mod command;
mod debian;
pub mod error;

pub fn install(path: &Path) -> Result<(), InstallError> {
    let file_info = file_info_from(path).and_then(is_supported)?;
    let installer = find_installer_for(&file_info.file_type);

    installer(&file_info.path).map_err(InstallError::Fatal)?;

    Ok(())
}

type InstallerResult = Result<(), String>;

#[derive(Debug, Eq, PartialEq)]
enum FileType {
    Debian,
}

#[derive(Debug)]
struct FileInfo {
    path: PathBuf,
    extension: Option<OsString>,
}

#[derive(Debug)]
struct SupportedFileInfo {
    path: PathBuf,
    file_type: FileType,
}

fn file_info_from(path: &Path) -> Result<FileInfo, InstallError> {
    if !path.is_file() {
        return Err(InstallError::not_a_file(path));
    }

    Ok(FileInfo {
        path: PathBuf::from(path),
        extension: path.extension().map(OsString::from),
    })
}

fn is_supported(file: FileInfo) -> Result<SupportedFileInfo, InstallError> {
    file.extension
        .and_then(file_type_for)
        .map(|file_type| SupportedFileInfo {
            path: PathBuf::from(&file.path),
            file_type,
        })
        .ok_or_else(|| InstallError::not_supported(&file.path))
}

fn file_type_for(extension: OsString) -> Option<FileType> {
    if extension == "deb" {
        return Some(FileType::Debian);
    }

    None
}

fn find_installer_for(file_type: &FileType) -> fn(&Path) -> InstallerResult {
    match file_type {
        FileType::Debian => DebianInstaller::run,
    }
}
