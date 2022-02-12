pub mod color;
pub mod handlers;
mod parse_repository;
pub mod root_command;
mod select;
pub mod spinner;
mod temp_file;

pub fn get_env(name: &str) -> Option<String> {
    std::env::var(name).ok()
}
