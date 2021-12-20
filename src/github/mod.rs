use crate::github::release::{Asset, Release};
use error::GithubError;
use std::io::Read;
use std::time::Duration;

pub mod error;
pub mod release;
pub mod tagged_asset;

#[derive(Debug, Eq, PartialEq)]
pub struct Repository {
    pub owner: String,
    pub repo: String,
}

pub fn latest_release(repository: &Repository) -> Result<Release, GithubError> {
    let url = format!(
        "https://api.github.com/repos/{owner}/{repo}/releases/latest",
        owner = &repository.owner,
        repo = &repository.repo
    );

    ureq::get(&url)
        .timeout(Duration::from_secs(5))
        .call()
        .map_err(GithubError::from)
        .and_then(deserialize)
}

fn deserialize(response: ureq::Response) -> Result<Release, GithubError> {
    response
        .into_json::<Release>()
        .map_err(GithubError::JsonDeserialization)
}

pub fn download_asset(asset: &Asset) -> Result<impl Read + Send, GithubError> {
    ureq::get(&asset.download_url)
        .call()
        .map_err(GithubError::from)
        .map(|response| response.into_reader())
}
