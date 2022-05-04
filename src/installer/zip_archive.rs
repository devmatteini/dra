use std::path::Path;
use std::process::Command;

use crate::installer::archive::ArchiveInstaller;
use crate::installer::command::exec_command;
use crate::installer::InstallerResult;

const UNZIP: &str = "unzip";

pub struct ZipArchiveInstaller;

impl ZipArchiveInstaller {
    pub fn run(source: &Path, destination_dir: &Path) -> InstallerResult {
        ArchiveInstaller::run(Self::extract_archive, source, destination_dir)
    }

    fn extract_archive(source: &Path, temp_dir: &Path) -> Result<(), String> {
        exec_command(
            UNZIP,
            Command::new(UNZIP).arg(source).arg("-d").arg(temp_dir),
        )
    }
}
