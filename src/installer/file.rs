use std::{ffi::OsString, path::PathBuf};

use crate::installer::error::InstallError;
use crate::installer::tar_archive::TarKind;

#[derive(Debug, Eq, PartialEq)]
pub enum FileType {
    Debian,
    TarArchive(TarKind),
    ZipArchive,
}

#[derive(Debug)]
pub struct FileInfo {
    pub path: PathBuf,
    pub name: String,
    pub extension: Option<OsString>,
}

#[derive(Debug)]
pub struct SupportedFileInfo {
    pub path: PathBuf,
    pub file_type: FileType,
}

pub fn is_supported(file: FileInfo) -> Result<SupportedFileInfo, InstallError> {
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
    if extension == "gz" {
        return Some(FileType::TarArchive(TarKind::Gz));
    }
    if extension == "bz2" {
        return Some(FileType::TarArchive(TarKind::Bz2));
    }
    if extension == "xz" {
        return Some(FileType::TarArchive(TarKind::Xz));
    }
    if extension == "zip" {
        return Some(FileType::ZipArchive);
    }

    None
}
