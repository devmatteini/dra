use std::fmt::{Display, Formatter};
use std::path::Path;
use std::{ffi::OsString, path::PathBuf};

use crate::installer::error::InstallError;

#[derive(Debug, Eq, PartialEq)]
pub enum Compression {
    Gz,
    Xz,
    Bz2,
}

#[derive(Debug, Eq, PartialEq)]
pub enum FileType {
    Debian,
    TarArchive(Compression),
    ZipArchive,
    CompressedFile(Compression),
}

#[derive(Debug)]
pub struct FileInfo {
    pub path: PathBuf,
    pub name: String,
    extension: Option<OsString>,
}

#[derive(Debug)]
pub struct SupportedFileInfo {
    pub path: PathBuf,
    pub file_type: FileType,
}

pub fn validate_file(file: FileInfo) -> Result<SupportedFileInfo, InstallError> {
    file_type_for(&file)
        .map(|file_type| SupportedFileInfo {
            path: PathBuf::from(&file.path),
            file_type,
        })
        .ok_or_else(|| InstallError::not_supported(&file.name))
}

fn file_type_for(file: &FileInfo) -> Option<FileType> {
    let file_name = file.name.to_lowercase();

    if file_name.ends_with(".deb") {
        return Some(FileType::Debian);
    }
    if file_name.ends_with(".tar.gz") || file_name.ends_with(".tgz") {
        return Some(FileType::TarArchive(Compression::Gz));
    }
    if file_name.ends_with(".gz") {
        return Some(FileType::CompressedFile(Compression::Gz));
    }
    if file_name.ends_with(".tar.bz2") || file_name.ends_with(".tbz") {
        return Some(FileType::TarArchive(Compression::Bz2));
    }
    if file_name.ends_with(".bz2") {
        return Some(FileType::CompressedFile(Compression::Bz2));
    }
    if file_name.ends_with(".tar.xz") || file_name.ends_with(".txz") {
        return Some(FileType::TarArchive(Compression::Xz));
    }
    if file_name.ends_with(".xz") || file_name.ends_with(".txz") {
        return Some(FileType::CompressedFile(Compression::Xz));
    }
    if file_name.ends_with(".zip") {
        return Some(FileType::ZipArchive);
    }

    None
}

impl Display for Compression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Compression::Gz => f.write_str("gz"),
            Compression::Xz => f.write_str("xz"),
            Compression::Bz2 => f.write_str("bz2"),
        }
    }
}

impl FileInfo {
    pub fn new(name: &str, path: &Path) -> Self {
        FileInfo {
            path: PathBuf::from(path),
            name: String::from(name),
            extension: Path::new(name).extension().map(OsString::from),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use test_case::test_case;

    use super::{validate_file, Compression, FileInfo, FileType, SupportedFileInfo};
    use crate::installer::error::InstallError;

    #[test_case("file.deb", FileType::Debian)]
    #[test_case("file.tar.gz", FileType::TarArchive(Compression::Gz))]
    #[test_case("file.tgz", FileType::TarArchive(Compression::Gz))]
    #[test_case("file.gz", FileType::CompressedFile(Compression::Gz))]
    #[test_case("file.tar.bz2", FileType::TarArchive(Compression::Bz2))]
    #[test_case("file.tbz", FileType::TarArchive(Compression::Bz2))]
    #[test_case("file.bz2", FileType::CompressedFile(Compression::Bz2))]
    #[test_case("file.tar.xz", FileType::TarArchive(Compression::Xz))]
    #[test_case("file.txz", FileType::TarArchive(Compression::Xz))]
    #[test_case("file.xz", FileType::CompressedFile(Compression::Xz))]
    #[test_case("file.zip", FileType::ZipArchive)]
    fn supported_file(file_name: &str, expected_file_type: FileType) {
        let file_info = any_file_info(file_name);
        let result = validate_file(file_info);

        assert_ok_equal(expected_file_type, result);
    }

    #[test_case("file.txt")]
    fn not_supported(file_name: &str) {
        let file_info = any_file_info(file_name);
        let result = validate_file(file_info);

        assert_not_supported(result);
    }

    #[test]
    fn no_file_extension() {
        let file_info = any_file_info("any_file");
        let result = validate_file(file_info);

        assert_not_supported(result);
    }

    fn any_file_info(file_name: &str) -> FileInfo {
        let path = PathBuf::from(file_name);
        let extension = path.extension().map(|x| x.into());

        FileInfo {
            path,
            name: file_name.to_string(),
            extension,
        }
    }

    fn assert_ok_equal(expected: FileType, actual: Result<SupportedFileInfo, InstallError>) {
        if let Ok(x) = actual {
            assert_eq!(expected, x.file_type);
        } else {
            panic!("Result is Err: {:?}", actual);
        }
    }

    fn assert_not_supported(actual: Result<SupportedFileInfo, InstallError>) {
        if let Err(e) = actual {
            match e {
                InstallError::NotSupported(_) => {}
                _ => panic!("expected InstallError::NotSupported. Got {}", e),
            }
        } else {
            panic!("Result is ok: {:?}", actual);
        }
    }
}
