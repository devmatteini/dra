use std::process::Command;

use crate::installer::command::exec_command;
use crate::installer::destination::Destination;
use crate::installer::executable::Executable;
use crate::installer::file::SupportedFileInfo;
use crate::installer::result::{InstallOutput, InstallerResult};

const RPM: &str = "rpm";

pub struct RpmInstaller;

impl RpmInstaller {
    pub fn run(
        file_info: SupportedFileInfo,
        _destination: Destination,
        _executables: Vec<Executable>,
    ) -> InstallerResult {
        exec_command(
            RPM,
            Command::new(RPM)
                .arg("--install")
                .arg("--replacepkgs")
                .arg(file_info.path),
        )
        .map(|_| InstallOutput::new(format!("RPM package '{}' installed", file_info.name)))
    }
}
