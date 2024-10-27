mod archive_installer;
mod command;
mod compressed_file_installer;
mod debian_installer;
pub mod destination;
pub mod error;
pub mod executable;
mod executable_file_installer;
mod file;
mod install;
mod result;
mod seven_zip_archive_installer;
mod tar_archive_installer;
mod zip_archive_installer;

pub use install::install;
