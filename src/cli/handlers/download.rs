use crate::cli::download_spinner::DownloadSpinner;
use crate::cli::handlers::HandlerResult;
use crate::github;
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
        let release = github::latest_release(&self.repository).map_err(|e| e.to_string())?;
        let items = release
            .assets
            .iter()
            .map(|x| x.name.clone())
            .collect::<Vec<String>>();
        let selected = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick the asset to download")
            .default(0)
            .items(&items)
            .interact()
            .unwrap();
        let selected_name = &items[selected];
        let selected_asset = release
            .assets
            .into_iter()
            .find(|x| &x.name == selected_name)
            .unwrap();
        let spinner = DownloadSpinner::new(selected_name);
        spinner.start();
        let mut stream = github::download_asset(&selected_asset).map_err(|e| e.to_string())?;
        let mut destination = File::create(&selected_name).unwrap();
        std::io::copy(&mut stream, &mut destination).unwrap();
        spinner.finish();

        Ok(())
    }
}
