use crate::installer::destination::Destination;
use crate::installer::error::InstallErrorMapErr;
use crate::installer::executable::{set_executable_permissions, Executable};
use crate::installer::file::SupportedFileInfo;
use crate::installer::result::{InstallOutput, InstallerResult};

pub struct ExecutableFileInstaller;

impl ExecutableFileInstaller {
    pub fn run(
        file_info: SupportedFileInfo,
        destination: Destination,
        _executable: &Executable,
    ) -> InstallerResult {
        let executable_path = match destination {
            Destination::Directory(dir) => dir.join(file_info.name),
            Destination::File(file) => file,
        };

        std::fs::copy(file_info.path, &executable_path)
            .map_fatal_err(format!("Error copying {}", executable_path.display()))?;

        set_executable_permissions(&executable_path)?;

        Ok(InstallOutput::new(format!(
            "Extracted executable '{}'",
            executable_path.display()
        )))
    }
}
