#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use test_case::test_case;

    use crate::installer::error::InstallError;
    use crate::installer::file::{is_supported, FileInfo, FileType, SupportedFileInfo};
    use crate::installer::TarKind;

    #[test_case("deb", FileType::Debian)]
    #[test_case("gz", FileType::TarArchive(TarKind::Gz))]
    #[test_case("bz2", FileType::TarArchive(TarKind::Bz2))]
    #[test_case("xz", FileType::TarArchive(TarKind::Xz))]
    #[test_case("zip", FileType::ZipArchive)]
    fn supported_file(file_extension: &str, expected_file_type: FileType) {
        let file_info = any_file_info(Some(file_extension));
        let result = is_supported(file_info);

        assert_ok_equal(expected_file_type, result);
    }

    #[test_case("txt")]
    fn not_supported(file_extension: &str) {
        let file_info = any_file_info(Some(file_extension));
        let result = is_supported(file_info);

        assert_not_supported(result);
    }

    #[test]
    fn no_file_extension() {
        let file_info = any_file_info(None);
        let result = is_supported(file_info);

        assert_not_supported(result);
    }

    fn any_file_info(extension: Option<&str>) -> FileInfo {
        FileInfo {
            path: PathBuf::new(),
            name: "ANY".into(),
            extension: extension.map(|x| x.into()),
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
