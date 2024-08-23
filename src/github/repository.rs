use std::fmt::Formatter;

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
