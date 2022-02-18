use std::path::Path;
use std::process::Command;

use crate::installer::archive::ArchiveInstaller;
use crate::installer::command::exec_command;
use crate::installer::InstallerResult;

const TAR: &str = "tar";

pub struct TarArchiveInstaller;

impl TarArchiveInstaller {
    pub fn run(source: &Path, destination_dir: &Path) -> InstallerResult {
        ArchiveInstaller::run(Self::extract_archive, source, destination_dir)
    }

    fn extract_archive(source: &Path, temp_dir: &Path) -> Result<(), String> {
        exec_command(
            TAR,
            Command::new(TAR)
                .arg("zxf")
                .arg(source)
                .arg("-C")
                .arg(temp_dir)
                // Remove the root dir inside the tar archive. See https://askubuntu.com/a/470266
                .arg("--strip-components=1"),
        )
    }
}
