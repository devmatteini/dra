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

    pub fn get(&self, url: &str) -> ureq::Request {
        self.token
            .as_ref()
            .map(|x| ureq::get(url).set("Authorization", &format!("token {}", x)))
            .unwrap_or_else(|| ureq::get(url))
    }
}
