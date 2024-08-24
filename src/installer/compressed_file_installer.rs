use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::installer::destination::Destination;
use crate::installer::error::InstallErrorMapErr;
use crate::installer::executable::{set_executable_permissions, Executable};
use crate::installer::file::SupportedFileInfo;
use crate::installer::result::{InstallOutput, InstallerResult};

pub struct CompressedFileInstaller;

impl CompressedFileInstaller {
    pub fn gz(
        file_info: SupportedFileInfo,
        destination: Destination,
        _executable: &Executable,
    ) -> InstallerResult {
        Self::decompress_and_move(
            |file| Box::new(flate2::read::GzDecoder::new(file)),
            file_info,
            destination,
        )
    }

    pub fn xz(
        file_info: SupportedFileInfo,
        destination: Destination,
        _executable: &Executable,
    ) -> InstallerResult {
        Self::decompress_and_move(
            |file| Box::new(xz2::read::XzDecoder::new(file)),
            file_info,
            destination,
        )
    }

    pub fn bz2(
        file_info: SupportedFileInfo,
        destination: Destination,
        _executable: &Executable,
    ) -> InstallerResult {
        Self::decompress_and_move(
            |file| Box::new(bzip2::read::BzDecoder::new(file)),
            file_info,
            destination,
        )
    }

    fn decompress_and_move<D>(
        decode: D,
        file_info: SupportedFileInfo,
        destination: Destination,
    ) -> InstallerResult
    where
        D: FnOnce(File) -> Box<dyn Read>,
    {
        let compressed_file = File::open(&file_info.path)
            .map_fatal_err(format!("Error opening {}", file_info.path.display()))?;

        let mut stream = decode(compressed_file);

        let executable_path = match destination {
            Destination::Directory(dir) => dir.join(executable_name(&file_info)),
            Destination::File(file) => file,
        };
        let mut destination_file = File::create(&executable_path)
            .map_fatal_err(format!("Error creating {}", executable_path.display()))?;

        std::io::copy(&mut stream, &mut destination_file)
            .map_fatal_err(format!("Error saving {}", executable_path.display()))?;

        set_executable_permissions(&executable_path)?;

        Ok(InstallOutput::new(format!(
            "Extracted compressed executable to '{}'",
            executable_path.display()
        )))
    }
}

/// This follows the same behavior of bzip2, gzip, and xz when decompressing a file.
fn executable_name(file_info: &SupportedFileInfo) -> PathBuf {
    let default_name = PathBuf::from(&file_info.name);

    default_name
        .as_path()
        .file_stem()
        .map(PathBuf::from)
        .unwrap_or_else(|| default_name)
}
