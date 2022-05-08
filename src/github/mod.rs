use crate::github::client::GithubClient;
use crate::github::release::{Asset, Release, Tag};
use error::GithubError;
use std::io::Read;
use std::time::Duration;

pub mod client;
pub mod error;
pub mod release;
pub mod tagged_asset;

pub const GITHUB_TOKEN: &str = "GITHUB_TOKEN";

#[derive(Debug, Eq, PartialEq)]
pub struct Repository {
    pub owner: String,
    pub repo: String,
}

pub fn get_release(
    client: &GithubClient,
    repository: &Repository,
    tag: Option<&Tag>,
) -> Result<Release, GithubError> {
    let url = format!(
        "https://api.github.com/repos/{owner}/{repo}/releases/{release}",
        owner = &repository.owner,
        repo = &repository.repo,
        release = tag
            .map(|t| format!("tags/{}", t.0))
            .unwrap_or_else(|| String::from("latest"))
    );

    client
        .get(&url)
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

// DOCS: https://docs.github.com/en/rest/reference/releases#get-a-release-asset
pub fn download_asset(
    client: &GithubClient,
    asset: &Asset,
) -> Result<(impl Read + Send, u64), GithubError> {
    let response = client
        .get(&asset.download_url)
        .set("Accept", "application/octet-stream")
        .call()
        .map_err(GithubError::from)?;
    let content_length = response
        .header("Content-Length")
        .and_then(|v| v.parse().ok())
        .ok_or(GithubError::InvalidContentLength)?;
    Ok((response.into_reader(), content_length))
}
