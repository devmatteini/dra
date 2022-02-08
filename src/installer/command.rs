use std::process::{Command, Output};

pub fn exec_command(command: &mut Command, name: &str) -> Result<Output, String> {
    command
        .output()
        .map_err(|x| format!("An error occurred executing '{}': {}", name, x))
}
