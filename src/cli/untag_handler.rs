use crate::cli::github_release::{check_has_assets, fetch_release_for};
use crate::cli::result::{HandlerError, HandlerResult};
use crate::cli::select_assets;
use crate::github::client::GithubClient;
use crate::github::release::{Asset, Release};
use crate::github::repository::Repository;
use crate::github::tagged_asset::TaggedAsset;

pub struct UntagHandler {
    repository: Repository,
}

impl UntagHandler {
    pub fn new(repository: Repository) -> Self {
        UntagHandler { repository }
    }

    pub fn run(&self) -> HandlerResult {
        let github = GithubClient::from_environment();
        let release = Self::fetch_latest_release(&github, &self.repository)?;
        check_has_assets(&release)?;
        let selected_asset = Self::ask_select_asset(release.assets)?;
        let untagged = TaggedAsset::untag(&release.tag, &selected_asset);
        println!("{}", untagged);
        Ok(())
    }

    fn fetch_latest_release(
        github: &GithubClient,
        repository: &Repository,
    ) -> Result<Release, HandlerError> {
        fetch_release_for(github, repository, None)
    }

    fn ask_select_asset(assets: Vec<Asset>) -> select_assets::AskSelectAssetResult {
        select_assets::ask_select_asset(
            assets,
            select_assets::Messages {
                select_prompt: "Pick the asset to untag",
                quit_select: "No asset selected",
            },
        )
    }
}
