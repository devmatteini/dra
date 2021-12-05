use crate::cli::download_spinner::DownloadSpinner;
use crate::cli::handlers::{HandlerError, HandlerResult};
use crate::github;
use crate::github::release::{Asset, Release};
use crate::github::{DownloadAssetError, ReleaseError, Repository};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::fs::File;

pub struct DownloadHandler {
    repository: Repository,
}

impl DownloadHandler {
    pub fn new(repository: Repository) -> Self {
        DownloadHandler { repository }
    }

    pub fn run(&self) -> HandlerResult {
        let release = self.fetch_latest_release()?;
        let selected_asset = Self::ask_select_asset(release.assets)?;
        Self::download_asset(&selected_asset)?;
        Ok(())
    }

    fn fetch_latest_release(&self) -> Result<Release, HandlerError> {
        github::latest_release(&self.repository).map_err(Self::release_error)
    }

    fn ask_select_asset(assets: Vec<Asset>) -> Result<Asset, HandlerError> {
        let items = Self::assets_names(&assets);
        let index = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick the asset to download")
            .default(0)
            .items(&items)
            .interact_opt()
            .map_err(|e| HandlerError::new(e.to_string()))?;
        if index.is_none() {
            return Err(HandlerError::op_cancelled("No asset selected"));
        }
        let selected_name = &items[index.unwrap()];
        Ok(Self::find_asset_by_name(selected_name, assets))
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

    fn assets_names(assets: &[Asset]) -> Vec<String> {
        assets
            .iter()
            .map(|x| x.name.clone())
            .collect::<Vec<String>>()
    }

    fn find_asset_by_name(name: &str, assets: Vec<Asset>) -> Asset {
        assets.into_iter().find(|x| x.name == name).unwrap()
    }

    fn release_error(e: ReleaseError) -> HandlerError {
        HandlerError::new(e.to_string())
    }

    fn download_error(e: DownloadAssetError) -> HandlerError {
        HandlerError::new(e.to_string())
    }
}
