use std::process::Command;

use crate::installer::command::exec_command;
use crate::installer::destination::Destination;
use crate::installer::executable::Executable;
use crate::installer::file::SupportedFileInfo;
use crate::installer::result::{InstallOutput, InstallerResult};

const DPKG: &str = "dpkg";

pub struct DebianInstaller;

impl DebianInstaller {
    pub fn run(
        file_info: SupportedFileInfo,
        _destination: Destination,
        _executables: Vec<Executable>,
    ) -> InstallerResult {
        exec_command(
            DPKG,
            Command::new(DPKG).arg("--install").arg(file_info.path),
        )
        .map(|_| InstallOutput::new(format!("Debian package '{}' installed", file_info.name)))
    }
}
