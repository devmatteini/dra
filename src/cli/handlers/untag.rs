use crate::cli::handlers::select;
use crate::cli::handlers::{HandlerError, HandlerResult};
use crate::github;
use crate::github::release::{Asset, Release};
use crate::github::{ReleaseError, Repository};

pub struct UntagHandler {
    repository: Repository,
}

impl UntagHandler {
    pub fn new(repository: Repository) -> Self {
        UntagHandler { repository }
    }

    pub fn run(&self) -> HandlerResult {
        let release = Self::fetch_latest_release(&self.repository)?;
        let selected_asset = Self::ask_select_asset(release.assets)?;
        let untagged = github::untag_asset(&release.tag, &selected_asset);
        println!("Untagged asset: {}", untagged);
        Ok(())
    }

    fn fetch_latest_release(repository: &Repository) -> Result<Release, HandlerError> {
        github::latest_release(repository).map_err(Self::release_error)
    }

    fn ask_select_asset(assets: Vec<Asset>) -> select::AskSelectAssetResult {
        select::ask_select_asset(
            assets,
            select::Messages {
                select_prompt: "Pick the asset to untag",
                quit_select: "No asset selected",
            },
        )
    }

    fn release_error(e: ReleaseError) -> HandlerError {
        HandlerError::new(e.to_string())
    }
}
