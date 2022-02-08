use crate::cli::color::Color;
use crate::cli::handlers::download::DownloadHandler;
use crate::cli::handlers::untag::UntagHandler;
use crate::cli::handlers::{HandlerError, HandlerResult};
use crate::cli::root_command::{Cli, Command};
use clap::Parser;
use std::process::exit;

mod cli;
mod github;
mod installer;

fn main() {
    let cli: Cli = Cli::parse();
    handle(run(cli));
}

fn run(cli: Cli) -> HandlerResult {
    match cli.cmd {
        Command::Download {
            select,
            tag,
            output,
            install,
        } => DownloadHandler::new(cli.repo, select, tag, output, install).run(),
        Command::Untag => UntagHandler::new(cli.repo).run(),
    }
}

fn handle(result: HandlerResult) {
    if let Err(error) = result {
        match error {
            HandlerError::Default(msg) => {
                eprintln!("{}", Color::new(&msg).red().bold());
                exit(1)
            }
            HandlerError::OperationCancelled(msg) => {
                println!("Operation cancelled: {}", Color::new(&msg).bold());
            }
        }
    }
}
