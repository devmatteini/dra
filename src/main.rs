use crate::cli::handlers::download::DownloadHandler;
use crate::cli::handlers::untag::UntagHandler;
use crate::cli::handlers::HandlerError;
use crate::cli::root_command::{Cli, Command};
use std::process::exit;
use structopt::StructOpt;

mod cli;
mod github;

fn main() {
    let cli: Cli = Cli::from_args();
    match cli.cmd {
        Command::Download => {
            if let Err(e) = DownloadHandler::new(cli.repo).run() {
                handle_error(e)
            }
        }
        Command::Untag => {
            if let Err(e) = UntagHandler::new(cli.repo).run() {
                handle_error(e)
            }
        }
    }
}

fn handle_error(error: HandlerError) {
    match error {
        HandlerError::Default(msg) => {
            eprintln!("{}", msg);
            exit(1)
        }
        HandlerError::OperationCancelled(msg) => {
            println!("Operation cancelled: {}", msg);
        }
    }
}
