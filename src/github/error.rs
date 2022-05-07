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
            GithubError::RateLimitExceeded => f.write_str(
                "GitHub API rate limit exceeded.
Export GITHUB_TOKEN environment variable to avoid this error.
More information can be found at https://github.com/devmatteini/dra#usage",
            ),
            GithubError::Unauthorized => f.write_str(
                "Invalid GitHub credentials.
Make sure GITHUB_TOKEN is valid",
            ),
        }
    }
}
