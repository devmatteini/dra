use arboard::Clipboard;

use crate::cli::get_env;
use crate::cli::handlers::common::{check_has_assets, fetch_release_for};
use crate::cli::handlers::{HandlerError, HandlerResult};
use crate::cli::select;
use crate::github::client::GithubClient;
use crate::github::release::{Asset, Release};
use crate::github::tagged_asset::TaggedAsset;
use crate::github::{Repository, GITHUB_TOKEN};

pub struct UntagHandler {
    repository: Repository,
    copy: bool,
}

impl UntagHandler {
    pub fn new(repository: Repository, copy: bool) -> Self {
        UntagHandler { repository, copy }
    }

    pub fn run(&self) -> HandlerResult {
        let client = GithubClient::new(get_env(GITHUB_TOKEN));
        let release = Self::fetch_latest_release(&client, &self.repository)?;
        check_has_assets(&release)?;
        let selected_asset = Self::ask_select_asset(release.assets)?;
        let untagged = TaggedAsset::untag(&release.tag, &selected_asset);
        println!("{}", &untagged);
        self.copy_to_clipboard(&untagged)?;
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

    fn copy_to_clipboard(&self, value: &str) -> Result<(), HandlerError> {
        if !self.copy {
            return Ok(());
        }

        // TODO: should we fail the execution if cannot copy to clipboard?
        let to_handler_error = |e: arboard::Error| HandlerError::new(format!("clipboard: {}", e));
        let mut clipboard = Clipboard::new().map_err(to_handler_error)?;
        clipboard.set_text(value).map_err(to_handler_error)?;
        Ok(())
    }
}
