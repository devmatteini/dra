use crate::github::error::GithubError;
use crate::github::release::{Asset, Release, Tag};
use crate::github::repository::Repository;
use crate::github::response::ReleaseResponse;
use std::io::Read;
use std::time::Duration;

const GITHUB_TOKEN: &str = "GITHUB_TOKEN";

pub struct GithubClient {
    pub token: Option<String>,
}

impl GithubClient {
    pub fn new(token: Option<String>) -> Self {
        Self { token }
    }

    pub fn from_environment() -> Self {
        Self::new(std::env::var(GITHUB_TOKEN).ok())
    }

    fn get(&self, url: &str) -> ureq::Request {
        self.token
            .as_ref()
            .map(|x| ureq::get(url).set("Authorization", &format!("token {}", x)))
            .unwrap_or_else(|| ureq::get(url))
    }

    // DOCS:
    // - https://docs.github.com/en/rest/releases/releases#get-the-latest-release
    // - https://docs.github.com/en/rest/releases/releases#get-a-release-by-tag-name
    pub fn get_release(
        &self,
        repository: &Repository,
        tag: Option<&Tag>,
    ) -> Result<Release, GithubError> {
        let url = get_release_url(repository, tag);
        self.get(&url)
            .timeout(Duration::from_secs(5))
            .call()
            .map_err(GithubError::from)
            .and_then(deserialize)
            .map(to_release(repository))
    }

    // DOCS: https://docs.github.com/en/rest/releases/assets#get-a-release-asset
    pub fn download_asset_stream(
        &self,
        asset: &Asset,
    ) -> Result<(impl Read + Send, Option<u64>), GithubError> {
        let response = self
            .get(&asset.download_url)
            .set("Accept", "application/vnd.github.raw")
            .call()
            .map_err(GithubError::from)?;
        let content_length = response
            .header("Content-Length")
            .and_then(|v| v.parse().ok());
        Ok((response.into_reader(), content_length))
    }
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
