use crate::installer::compressed_file_installer::CompressedFileInstaller;
use crate::installer::debian_installer::DebianInstaller;
use crate::installer::destination::Destination;
use crate::installer::error::InstallError;
use crate::installer::executable::Executable;
use crate::installer::executable_file_installer::ExecutableFileInstaller;
use crate::installer::file::{validate_file, Compression, FileInfo, FileType, SupportedFileInfo};
use crate::installer::result::InstallerResult;
use crate::installer::seven_zip_archive_installer::SevenZipArchiveInstaller;
use crate::installer::tar_archive_installer::TarArchiveInstaller;
use crate::installer::zip_archive_installer::ZipArchiveInstaller;
use std::path::Path;

pub fn install(
    asset_name: String,
    source: &Path,
    destination: Destination,
    executables: Vec<Executable>,
) -> InstallerResult {
    let file_info = file_info_from(&asset_name, source).and_then(validate_file)?;
    let installer = find_installer_for(&file_info.file_type);

    installer(file_info, destination, executables)
}

fn file_info_from(name: &str, path: &Path) -> Result<FileInfo, InstallError> {
    if !path.is_file() {
        return Err(InstallError::not_a_file(path));
    }
    Ok(FileInfo::new(name, path))
}

fn find_installer_for(
    file_type: &FileType,
) -> fn(SupportedFileInfo, Destination, Vec<Executable>) -> InstallerResult {
    match file_type {
        FileType::Debian => DebianInstaller::run,
        FileType::TarArchive(Compression::Gz) => TarArchiveInstaller::gz,
        FileType::TarArchive(Compression::Xz) => TarArchiveInstaller::xz,
        FileType::TarArchive(Compression::Bz2) => TarArchiveInstaller::bz2,
        FileType::ZipArchive => ZipArchiveInstaller::run,
        FileType::SevenZipArchive => SevenZipArchiveInstaller::run,
        FileType::CompressedFile(Compression::Gz) => CompressedFileInstaller::gz,
        FileType::CompressedFile(Compression::Xz) => CompressedFileInstaller::xz,
        FileType::CompressedFile(Compression::Bz2) => CompressedFileInstaller::bz2,
        FileType::ExecutableFile => ExecutableFileInstaller::run,
    }
}
