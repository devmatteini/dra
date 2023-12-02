use crate::github::client::GithubClient;
use crate::github::release::{Asset, Release, Tag};
use crate::github::response::ReleaseResponse;
use error::GithubError;
use std::fmt::Formatter;
use std::io::Read;
use std::time::Duration;

pub mod client;
pub mod error;
pub mod release;
mod response;
pub mod tagged_asset;

pub const GITHUB_TOKEN: &str = "GITHUB_TOKEN";

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Repository {
    pub owner: String,
    pub repo: String,
}

impl std::fmt::Display for Repository {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", &self.owner, &self.repo)
    }
}

// DOCS:
// - https://docs.github.com/en/rest/releases/releases#get-the-latest-release
// - https://docs.github.com/en/rest/releases/releases#get-a-release-by-tag-name
pub fn get_release(
    client: &GithubClient,
    repository: &Repository,
    tag: Option<&Tag>,
) -> Result<Release, GithubError> {
    let url = get_release_url(repository, tag);
    client
        .get(&url)
        .timeout(Duration::from_secs(5))
        .call()
        .map_err(GithubError::from)
        .and_then(deserialize)
        .map(to_release(repository))
}

// DOCS: https://docs.github.com/en/rest/releases/assets#get-a-release-asset
pub fn download_asset(
    client: &GithubClient,
    asset: &Asset,
) -> Result<(impl Read + Send, Option<u64>), GithubError> {
    let response = client
        .get(&asset.download_url)
        .set("Accept", "application/vnd.github.raw")
        .call()
        .map_err(GithubError::from)?;
    let content_length = response
        .header("Content-Length")
        .and_then(|v| v.parse().ok());
    Ok((response.into_reader(), content_length))
}

fn get_release_url(repository: &Repository, tag: Option<&Tag>) -> String {
    format!(
        "https://api.github.com/repos/{owner}/{repo}/releases/{release}",
        owner = &repository.owner,
        repo = &repository.repo,
        release = tag
            .map(|t| format!("tags/{}", t.0))
            .unwrap_or_else(|| String::from("latest"))
    )
}

fn deserialize(response: ureq::Response) -> Result<ReleaseResponse, GithubError> {
    response
        .into_json::<ReleaseResponse>()
        .map_err(GithubError::JsonDeserialization)
}

fn to_release(repository: &Repository) -> impl Fn(ReleaseResponse) -> Release + '_ {
    |response| Release::from_response(response, repository)
}
