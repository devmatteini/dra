use std::io::Write;

use clap::CommandFactory;

use crate::Cli;
use crate::cli::result::HandlerResult;

pub struct CompletionHandler {
    shell: clap_complete::Shell,
}

impl CompletionHandler {
    pub fn new(shell: clap_complete::Shell) -> Self {
        Self { shell }
    }

    pub fn run(&self) -> HandlerResult {
        let mut command = Cli::command();
        Self::generate_completion(self.shell, &mut command, &mut std::io::stdout());
        Ok(())
    }

    fn generate_completion<G, W>(generator: G, command: &mut clap::Command, buffer: &mut W)
    where
        G: clap_complete::Generator,
        W: Write,
    {
        clap_complete::generate(generator, command, command.get_name().to_string(), buffer);
    }
}
