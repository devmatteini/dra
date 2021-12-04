use structopt::StructOpt;

use crate::cli::parse_repository::try_parse_repository;
use crate::github::Repository;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "dag",
    about = "A command line tool to download assets from Github"
)]
pub struct Cli {
    #[structopt(subcommand)]
    pub cmd: Command,

    /// Github repository using format {owner}/{repo}
    #[structopt(parse(try_from_str = try_parse_repository))]
    pub repo: Repository,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Select and download an asset
    Download,
}
