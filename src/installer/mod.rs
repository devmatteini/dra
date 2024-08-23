mod archive;
mod command;
mod compressed_file;
mod debian;
pub mod destination;
pub mod error;
pub mod executable;
mod executable_file;
mod file;
mod install;
mod result;
mod tar_archive;
mod zip_archive;

pub use install::install;
