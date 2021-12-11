use crate::cli::download_spinner::DownloadSpinner;
use crate::cli::handlers::select;
use crate::cli::handlers::{HandlerError, HandlerResult};
use crate::github;
use crate::github::error::GithubError;
use crate::github::release::{Asset, Release};
use crate::github::tagged_asset::TaggedAsset;
use crate::github::Repository;
use std::fs::File;

pub struct DownloadHandler {
    repository: Repository,
    select: Option<String>,
}

impl DownloadHandler {
    pub fn new(repository: Repository, select: Option<String>) -> Self {
        DownloadHandler { repository, select }
    }

    pub fn run(&self) -> HandlerResult {
        let release = self.fetch_latest_release()?;
        let selected_asset = self.autoselect_or_ask_asset(release)?;
        Self::download_asset(&selected_asset)?;
        Ok(())
    }

    fn autoselect_or_ask_asset(&self, release: Release) -> Result<Asset, HandlerError> {
        if let Some(untagged) = self.select.as_ref() {
            Self::autoselect_asset(release, untagged)
        } else {
            Self::ask_select_asset(release.assets)
        }
    }

    fn autoselect_asset(release: Release, untagged: &str) -> Result<Asset, HandlerError> {
        let asset_name = TaggedAsset::tag(&release.tag, untagged);
        release
            .assets
            .into_iter()
            .find(|x| x.name == asset_name)
            .ok_or_else(|| HandlerError::new(format!("No asset found for {}", untagged)))
    }

    fn fetch_latest_release(&self) -> Result<Release, HandlerError> {
        github::latest_release(&self.repository).map_err(Self::release_error)
    }

    fn ask_select_asset(assets: Vec<Asset>) -> select::AskSelectAssetResult {
        select::ask_select_asset(
            assets,
            select::Messages {
                select_prompt: "Pick the asset to download",
                quit_select: "No asset selected",
            },
        )
    }

    fn download_asset(selected_asset: &Asset) -> Result<(), HandlerError> {
        let spinner = DownloadSpinner::new(&selected_asset.name);
        spinner.start();
        let mut stream = github::download_asset(selected_asset).map_err(Self::download_error)?;
        let mut destination = Self::create_file(&selected_asset.name)?;
        std::io::copy(&mut stream, &mut destination).unwrap();
        spinner.stop();
        Ok(())
    }

    fn create_file(selected_name: &str) -> Result<File, HandlerError> {
        File::create(selected_name).map_err(|e| HandlerError::new(e.to_string()))
    }

    fn release_error(e: GithubError) -> HandlerError {
        HandlerError::new(format!("Error fetching latest release. {}", e))
    }

    fn download_error(e: GithubError) -> HandlerError {
        HandlerError::new(format!("Error downloading asset. {}", e))
    }
}
