use crate::env_var;
use crate::github::constants::{
    DRA_DISABLE_GITHUB_AUTHENTICATION, DRA_GITHUB_TOKEN, GH_TOKEN, GITHUB_TOKEN,
};
use crate::github::error::GithubError;
use crate::github::release::{Asset, Release, Tag};
use crate::github::release_response::ReleaseResponse;
use crate::github::repository::Repository;
use std::io::Read;
use std::process::Command;
use std::time::Duration;

pub struct GithubClient {
    pub token: Option<String>,
}

impl GithubClient {
    pub fn new(token: Option<String>) -> Self {
        Self { token }
    }

    pub fn from_environment() -> Self {
        let is_auth_disabled = env_var::boolean(DRA_DISABLE_GITHUB_AUTHENTICATION);
        if is_auth_disabled {
            return Self::new(None);
        }

        let token = env_var::string(DRA_GITHUB_TOKEN)
            .or_else(|| env_var::string(GITHUB_TOKEN))
            .or_else(|| env_var::string(GH_TOKEN))
            .or_else(github_cli_token);

        Self::new(token)
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

fn github_cli_token() -> Option<String> {
    Command::new("gh")
        .args(["auth", "token"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })
        .map(|x| x.trim().to_string())
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
