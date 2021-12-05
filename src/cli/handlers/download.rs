use crate::cli::download_spinner::DownloadSpinner;
use crate::cli::handlers::HandlerResult;
use crate::github;
use crate::github::release::{Asset, Release};
use crate::github::Repository;
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
        let selected_asset = Self::ask_select_asset(release.assets);
        Self::download_asset(&selected_asset)?;
        Ok(())
    }

    fn fetch_latest_release(&self) -> Result<Release, String> {
        github::latest_release(&self.repository).map_err(|e| e.to_string())
    }

    fn ask_select_asset(assets: Vec<Asset>) -> Asset {
        let items = Self::assets_names(&assets);
        let index = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick the asset to download")
            .default(0)
            .items(&items)
            // TODO: check out interact_out() to allow user to exit selection and stop download
            .interact()
            .unwrap();
        let selected_name = &items[index];
        Self::find_asset_by_name(selected_name, assets)
    }

    fn download_asset(selected_asset: &Asset) -> Result<(), String> {
        let spinner = DownloadSpinner::new(&selected_asset.name);
        spinner.start();
        let mut stream = github::download_asset(&selected_asset).map_err(|e| e.to_string())?;
        let mut destination = Self::create_file(&selected_asset.name)?;
        std::io::copy(&mut stream, &mut destination).unwrap();
        spinner.stop();
        Ok(())
    }

    fn create_file(selected_name: &String) -> Result<File, String> {
        File::create(&selected_name).map_err(|e| e.to_string())
    }

    fn assets_names(assets: &Vec<Asset>) -> Vec<String> {
        assets
            .iter()
            .map(|x| x.name.clone())
            .collect::<Vec<String>>()
    }

    fn find_asset_by_name(name: &String, assets: Vec<Asset>) -> Asset {
        assets.into_iter().find(|x| &x.name == name).unwrap()
    }
}
