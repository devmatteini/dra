use std::path::Path;
use std::process::Command;

use crate::installer::command::exec_command;
use crate::installer::InstallerResult;

const DPKG: &str = "dpkg";

pub struct DebianInstaller;

impl DebianInstaller {
    pub fn run(path: &Path) -> InstallerResult {
        let result = exec_command(Command::new(DPKG).arg("--install").arg(path), DPKG)?;

        if result.status.success() {
            Ok(())
        } else {
            // TODO: create some helper function to do error reporting from a Command result
            Err(format!(
                "An error occurred while installing (status: {}):\n{}",
                result
                    .status
                    .code()
                    .map(|x| x.to_string())
                    .unwrap_or_else(|| "Unknown".into()),
                String::from_utf8(result.stderr).unwrap_or_else(|_| "Unknown dpkg error".into())
            ))
        }
    }
}
