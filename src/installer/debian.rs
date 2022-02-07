use crate::installer::InstallerResult;
use std::path::Path;

pub struct DebianInstaller;

impl DebianInstaller {
    pub fn run(path: &Path) -> InstallerResult {
        let result = std::process::Command::new("dpkg")
            .arg("--install")
            .arg(path)
            .output()
            .map_err(|x| format!("An error occurred while installing: {}", x))?;

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
