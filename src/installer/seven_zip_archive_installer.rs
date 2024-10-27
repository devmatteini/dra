use crate::installer::archive_installer::ArchiveInstaller;
use crate::installer::command::exec_command;
use crate::installer::destination::Destination;
use crate::installer::error::InstallError;
use crate::installer::executable::Executable;
use crate::installer::file::SupportedFileInfo;
use crate::installer::result::InstallerResult;
use std::path::Path;
use std::process::Command;

const _7Z: &str = "7z";

pub struct SevenZipArchiveInstaller;

impl SevenZipArchiveInstaller {
    pub fn run(
        file_info: SupportedFileInfo,
        destination: Destination,
        executable: &Executable,
    ) -> InstallerResult {
        ArchiveInstaller::run(Self::extract_archive, file_info, destination, executable)
    }

    fn extract_archive(source: &Path, temp_dir: &Path) -> Result<(), InstallError> {
        exec_command(
            _7Z,
            Command::new(_7Z)
                .arg("x")
                .arg(source)
                .arg(format!("-o{}", temp_dir.display())),
        )
    }
}
