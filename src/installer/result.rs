use crate::installer::error::InstallError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct InstallOutput(String);

impl InstallOutput {
    pub fn new(message: String) -> Self {
        Self(message)
    }
}

impl Display for InstallOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

pub type InstallerResult = Result<InstallOutput, InstallError>;
