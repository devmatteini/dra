use std::fmt::Formatter;

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
        if !src.contains('/') {
            return Err("Invalid repository. Use {owner}/{repo} format".to_string());
        }
        let parts = src
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
