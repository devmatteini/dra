use crate::cli::environment::get_env;
use crate::cli::handlers::common::{check_has_assets, fetch_release_for};
use crate::cli::handlers::result::{HandlerError, HandlerResult};
use crate::cli::select;
use crate::github::client::GithubClient;
use crate::github::release::{Asset, Release};
use crate::github::repository::Repository;
use crate::github::tagged_asset::TaggedAsset;
use crate::github::GITHUB_TOKEN;

pub struct UntagHandler {
    repository: Repository,
}

impl UntagHandler {
    pub fn new(repository: Repository) -> Self {
        UntagHandler { repository }
    }

    pub fn run(&self) -> HandlerResult {
        let client = GithubClient::new(get_env(GITHUB_TOKEN));
        let release = Self::fetch_latest_release(&client, &self.repository)?;
        check_has_assets(&release)?;
        let selected_asset = Self::ask_select_asset(release.assets)?;
        let untagged = TaggedAsset::untag(&release.tag, &selected_asset);
        println!("{}", untagged);
        Ok(())
    }

    fn fetch_latest_release(
        client: &GithubClient,
        repository: &Repository,
    ) -> Result<Release, HandlerError> {
        fetch_release_for(client, repository, None)
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
}
