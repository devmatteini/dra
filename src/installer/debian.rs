use std::path::Path;
use std::process::Command;

use crate::installer::command::exec_command;
use crate::installer::InstallerResult;

use super::error::InstallError;

const DPKG: &str = "dpkg";

pub struct DebianInstaller;

impl DebianInstaller {
    pub fn run(source: &Path, _destination_dir: &Path, _executable_name: &str) -> InstallerResult {
        exec_command(DPKG, Command::new(DPKG).arg("--install").arg(source))
            .map_err(InstallError::Fatal)
    }
}
