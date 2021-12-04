use std::fmt::Formatter;
use std::time::Duration;

use crate::github::release::Release;

pub mod release;

#[derive(Debug)]
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
    Http(String, ureq::Error),
    Json(String),
}

impl ReleaseError {
    pub fn http_error(error: ureq::Error) -> Self {
        // TODO: should check status code to understand if the repository does not exists
        Self::Http(error.to_string(), error)
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
            ReleaseError::Http(message, _) => f.write_str(message),
            ReleaseError::Json(e) => f.write_str(e),
        }
    }
}
