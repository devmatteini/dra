use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use crate::installer::InstallerResult;

use super::error::InstallError;

pub struct CompressedFileInstaller;

impl CompressedFileInstaller {
    pub fn gz(source: &Path, destination_dir: &Path, executable_name: &str) -> InstallerResult {
        Self::decompress_and_move(
            |file| Box::new(flate2::read::GzDecoder::new(file)),
            source,
            destination_dir,
            executable_name,
        )
    }

    pub fn xz(source: &Path, destination_dir: &Path, executable_name: &str) -> InstallerResult {
        Self::decompress_and_move(
            |file| Box::new(xz2::read::XzDecoder::new(file)),
            source,
            destination_dir,
            executable_name,
        )
    }

    pub fn bz2(source: &Path, destination_dir: &Path, executable_name: &str) -> InstallerResult {
        Self::decompress_and_move(
            |file| Box::new(bzip2::read::BzDecoder::new(file)),
            source,
            destination_dir,
            executable_name,
        )
    }

    fn decompress_and_move<D>(
        decode: D,
        source: &Path,
        destination_dir: &Path,
        executable_name: &str,
    ) -> Result<(), InstallError>
    where
        D: FnOnce(File) -> Box<dyn Read>,
    {
        let compressed_file = File::open(source).map_err(|x| {
            InstallError::Fatal(format!("Error opening {}: {}", source.display(), x))
        })?;

        let mut stream = decode(compressed_file);

        let executable_path = destination_dir.join(executable_name);
        let mut destination_file = File::create(&executable_path).map_err(|x| {
            InstallError::Fatal(format!(
                "Error creating {}: {}",
                executable_path.display(),
                x
            ))
        })?;

        let mut buffer = [0; 1024];
        while let Ok(bytes) = stream.read(&mut buffer) {
            if bytes == 0 {
                break;
            }

            destination_file.write(&buffer[..bytes]).map_err(|e| {
                InstallError::Fatal(format!("Error saving {}: {}", executable_path.display(), e))
            })?;
        }

        set_executable_permissions(&executable_path)?;

        Ok(())
    }
}

#[cfg(target_family = "unix")]
fn set_executable_permissions(path: &Path) -> Result<(), InstallError> {
    use std::os::unix::fs::PermissionsExt;

    std::fs::set_permissions(&path, PermissionsExt::from_mode(0o755)).map_err(|e| {
        InstallError::Fatal(format!(
            "Cannot set executable permissions on {}: {}",
            path.display(),
            e
        ))
    })
}

#[cfg(target_os = "windows")]
fn set_executable_permissions(path: &Path) -> Result<(), InstallError> {
    Ok(())
}
