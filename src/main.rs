// NOTE: this rule is not supported by rust-analyzer or JetBrains Rust plugin go to definition/refactoring tools so disable it until it's supported properly
#![allow(clippy::uninlined_format_args)]

use crate::cli::color::Color;
use crate::cli::handlers::completion::CompletionHandler;
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
    init_ctrl_c_handler();
    handle(run(cli));
}

// NOTE: this is needed to restore the cursor if CTRL+C is
// pressed during the asset selection (https://github.com/mitsuhiko/dialoguer/issues/77)
fn init_ctrl_c_handler() {
    ctrlc::set_handler(move || {
        let term = dialoguer::console::Term::stderr();
        let _ = term.show_cursor();
        exit(1);
    })
    .expect("Error initializing CTRL+C handler")
}

fn run(cli: Cli) -> HandlerResult {
    match cli.cmd {
        Command::Download {
            repo,
            select,
            automatic,
            tag,
            output,
            install,
        } => DownloadHandler::new(repo, select, tag, output, install).run(),
        Command::Untag { repo } => UntagHandler::new(repo).run(),
        Command::Completion { shell } => CompletionHandler::new(shell).run(),
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
