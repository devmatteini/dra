use crate::github::constants::{DRA_GITHUB_TOKEN, GH_TOKEN, GITHUB_TOKEN};
use std::fmt::Formatter;

#[derive(Debug)]
pub enum GithubError {
    Http(Box<ureq::Error>),
    JsonDeserialization(std::io::Error),
    RepositoryOrReleaseNotFound,
    RateLimitExceeded,
    Unauthorized,
}

impl GithubError {
    pub fn from(error: ureq::Error) -> Self {
        match error {
            ureq::Error::Status(401, _) => Self::Unauthorized,
            ureq::Error::Status(403, _) => Self::RateLimitExceeded,
            ureq::Error::Status(404, _) => Self::RepositoryOrReleaseNotFound,
            ureq::Error::Status(_, _) => Self::Http(Box::new(error)),
            ureq::Error::Transport(_) => Self::Http(Box::new(error)),
        }
    }
}

fn authentication_tokens() -> String {
    format!("{} / {} / {}", DRA_GITHUB_TOKEN, GITHUB_TOKEN, GH_TOKEN)
}

impl std::fmt::Display for GithubError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GithubError::Http(e) => f.write_str(&e.to_string()),
            GithubError::JsonDeserialization(e) => {
                f.write_str(&format!("Error deserializing response: {}", e))
            }
            GithubError::RepositoryOrReleaseNotFound => {
                f.write_str("Repository or release not found")
            }
            GithubError::RateLimitExceeded => {
                let message = format!(
                    "GitHub API rate limit exceeded.
Export one of {} environment variable to avoid this error.
More information can be found at https://github.com/devmatteini/dra#usage",
                    authentication_tokens()
                );
                f.write_str(&message)
            }
            GithubError::Unauthorized => {
                let message = format!(
                    "Invalid GitHub credentials.
Make sure one of {} is valid",
                    authentication_tokens()
                );
                f.write_str(&message)
            }
        }
    }
}
