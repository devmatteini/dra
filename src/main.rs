use crate::cli::color::Color;
use crate::cli::handlers::download::DownloadHandler;
use crate::cli::handlers::untag::UntagHandler;
use crate::cli::handlers::{HandlerError, HandlerResult};
use crate::cli::root_command::{Cli, Command};
use std::process::exit;
use structopt::StructOpt;

mod cli;
mod github;

fn main() {
    let cli: Cli = Cli::from_args();
    match cli.cmd {
        Command::Download { select, output } => {
            handle(DownloadHandler::new(cli.repo, select, output).run())
        }
        Command::Untag => handle(UntagHandler::new(cli.repo).run()),
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
