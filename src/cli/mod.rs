pub mod color;
pub mod download_spinner;
pub mod handlers;
mod parse_repository;
pub mod root_command;

pub fn get_env(name: &str) -> Option<String> {
    std::env::var(name).ok()
}
