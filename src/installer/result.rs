use crate::installer::error::InstallError;

#[derive(Debug)]
pub struct InstallOutput(String);

impl InstallOutput {
    pub fn new(message: String) -> Self {
        Self(message)
    }
}

pub type InstallerResult = Result<InstallOutput, InstallError>;
