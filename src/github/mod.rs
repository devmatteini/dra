use std::fmt::Formatter;
use std::io::Read;
use std::time::Duration;

use crate::github::release::{Asset, Release};

pub mod release;

#[derive(Debug, Eq, PartialEq)]
pub struct Repository {
    pub owner: String,
    pub repo: String,
}

pub fn latest_release(repository: &Repository) -> Result<Release, ReleaseError> {
    let url = format!(
        "https://api.github.com/repos/{owner}/{repo}/releases/latest",
        owner = &repository.owner,
        repo = &repository.repo
    );

    ureq::get(&url)
        .timeout(Duration::from_secs(5))
        .call()
        .map_err(ReleaseError::http_error)
        .and_then(deserialize)
}

fn deserialize(response: ureq::Response) -> Result<Release, ReleaseError> {
    response.into_json::<Release>().map_err(ReleaseError::json)
}

#[derive(Debug)]
pub enum ReleaseError {
    Http(ureq::Error),
    Json(String),
}

impl ReleaseError {
    pub fn http_error(error: ureq::Error) -> Self {
        // TODO: should check status code to understand if the repository does not exists
        Self::Http(error)
    }

    pub fn json(error: std::io::Error) -> Self {
        Self::Json(format!(
            "Error deserializing response: {}",
            error.to_string()
        ))
    }
}

impl std::fmt::Display for ReleaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReleaseError::Http(e) => {
                f.write_str(&format!("Error fetching latest release. {}", e.to_string()))
            }
            ReleaseError::Json(e) => f.write_str(e),
        }
    }
}

pub fn download_asset(asset: &Asset) -> Result<impl Read + Send, DownloadAssetError> {
    ureq::get(&asset.download_url)
        .call()
        .map_err(DownloadAssetError::http_error)
        .map(|response| response.into_reader())
}

#[derive(Debug)]
pub enum DownloadAssetError {
    Http(ureq::Error),
}

impl DownloadAssetError {
    pub fn http_error(error: ureq::Error) -> Self {
        Self::Http(error)
    }
}

impl std::fmt::Display for DownloadAssetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DownloadAssetError::Http(e) => {
                f.write_str(&format!("Error downloading asset. {}", e.to_string()))
            }
        }
    }
}
