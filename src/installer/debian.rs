use std::path::Path;
use std::process::Command;

use crate::installer::command::exec_command;
use crate::installer::InstallerResult;

use super::file::SupportedFileInfo;
use super::Executable;

const DPKG: &str = "dpkg";

pub struct DebianInstaller;

impl DebianInstaller {
    pub fn run(
        file_info: SupportedFileInfo,
        _destination_dir: &Path,
        _executable: &Executable,
    ) -> InstallerResult {
        exec_command(
            DPKG,
            Command::new(DPKG).arg("--install").arg(file_info.path),
        )
    }
}
