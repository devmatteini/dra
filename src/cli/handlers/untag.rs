use crate::cli::handlers::select;
use crate::cli::handlers::{HandlerError, HandlerResult};
use crate::github;
use crate::github::client::GithubClient;
use crate::github::error::GithubError;
use crate::github::release::{Asset, Release};
use crate::github::tagged_asset::TaggedAsset;
use crate::github::Repository;

pub struct UntagHandler {
    repository: Repository,
}

impl UntagHandler {
    pub fn new(repository: Repository) -> Self {
        UntagHandler { repository }
    }

    pub fn run(&self) -> HandlerResult {
        let client = GithubClient::new(None);
        let release = Self::fetch_latest_release(&client, &self.repository)?;
        let selected_asset = Self::ask_select_asset(release.assets)?;
        let untagged = TaggedAsset::untag(&release.tag, &selected_asset);
        println!("{}", untagged);
        Ok(())
    }

    fn fetch_latest_release(
        client: &GithubClient,
        repository: &Repository,
    ) -> Result<Release, HandlerError> {
        github::get_release(client, repository, None).map_err(Self::release_error)
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

    fn release_error(e: GithubError) -> HandlerError {
        HandlerError::new(format!("Error fetching latest release: {}", e))
    }
}
