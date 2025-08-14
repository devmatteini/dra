use std::fmt::Formatter;
use url::Url;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Repository {
    pub owner: String,
    pub repo: String,
}

impl Repository {
    pub fn try_parse(src: &str) -> Result<Repository, String> {
        if src.is_empty() {
            return Err("Invalid repository. Cannot be empty".to_string());
        }

        if src.starts_with("http://github.com") || src.starts_with("https://github.com") {
            Self::parse_url(src)
        } else {
            Self::parse(src)
        }
    }

    fn parse(input: &str) -> Result<Repository, String> {
        if !input.contains('/') {
            return Err("Invalid repository. Use {owner}/{repo} format".to_string());
        }
        let parts = input
            .split('/')
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();
        if parts.len() < 2 {
            return Err("Invalid repository. Missing owner or repo".to_string());
        }

        Ok(Repository {
            owner: parts[0].to_string(),
            repo: parts[1].to_string(),
        })
    }

    fn parse_url(input: &str) -> Result<Repository, String> {
        let github_url = Url::parse(input).map_err(|x| format!("Invalid repository URL: {}", x))?;
        let parts = github_url
            .path()
            .split('/')
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();
        if parts.len() < 2 {
            return Err("Invalid repository URL. Missing owner or repo".to_string());
        }

        Ok(Repository {
            owner: parts[0].to_string(),
            repo: parts[1].to_string(),
        })
    }
}

impl std::fmt::Display for Repository {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", &self.owner, &self.repo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_repository() {
        let input = "foo/bar";

        let result = Repository::try_parse(input);

        assert_eq!(
            Ok(Repository {
                owner: "foo".to_string(),
                repo: "bar".to_string()
            }),
            result
        );
    }

    #[test]
    fn valid_repository_from_url() {
        let input = "https://github.com/foo/bar?tab=readme-ov-file";

        let result = Repository::try_parse(input);

        assert_eq!(
            Ok(Repository {
                owner: "foo".to_string(),
                repo: "bar".to_string()
            }),
            result
        );
    }

    #[test]
    fn valid_repository_url_from_any_page() {
        let input = "https://github.com/foo/bar/actions/runs/16966254957";

        let result = Repository::try_parse(input);

        assert_eq!(
            Ok(Repository {
                owner: "foo".to_string(),
                repo: "bar".to_string()
            }),
            result
        );
    }

    #[test]
    fn missing_owner() {
        let input = "/bar";

        let result = Repository::try_parse(input);

        assert_error(|e| assert_contains("Missing owner or repo", e), result);
    }

    #[test]
    fn missing_repo() {
        let input = "foo/";

        let result = Repository::try_parse(input);

        assert_error(|e| assert_contains("Missing owner or repo", e), result);
    }

    #[test]
    fn missing_owner_in_url() {
        let input = "https://github.com";

        let result = Repository::try_parse(input);

        assert_error(|e| assert_contains("Missing owner or repo", e), result);
    }

    #[test]
    fn missing_repo_in_url() {
        let input = "https://github.com/foo/";

        let result = Repository::try_parse(input);

        assert_error(|e| assert_contains("Missing owner or repo", e), result);
    }

    #[test]
    fn empty_repository() {
        let input = "";

        let result = Repository::try_parse(input);

        assert_error(|e| assert_contains("Cannot be empty", e), result);
    }

    fn assert_error<F>(assert: F, actual: Result<Repository, String>)
    where
        F: FnOnce(&str),
    {
        if actual.is_ok() {
            panic!("actual is ok: {:#?}", actual.unwrap())
        }
        let error = actual.err().unwrap();
        assert(&error);
    }

    fn assert_contains(expected: &str, actual: &str) {
        if !actual.contains(expected) {
            panic!(
                "'{actual}' not contains '{expected}'",
                expected = expected,
                actual = actual
            )
        }
    }
}
