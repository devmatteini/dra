use std::fmt::Formatter;

#[derive(Debug)]
pub enum GithubError {
    Http(ureq::Error),
    JsonDeserialization(std::io::Error),
}

impl std::fmt::Display for GithubError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GithubError::Http(e) => f.write_str(&e.to_string()),
            GithubError::JsonDeserialization(e) => {
                f.write_str(&format!("Error deserializing response: {}", e))
            }
        }
    }
}
