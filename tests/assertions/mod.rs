use predicates::Predicate;

use crate::docker::ExecResult;

pub fn assert_success(result: ExecResult) -> String {
    match result {
        ExecResult::Success(x) => x,
        ExecResult::Error(x) => {
            panic!("exec failed with: {}", x)
        }
    }
}

pub fn assert_contains(expected: &str, actual: String) {
    assert!(
        predicates::str::contains(expected).eval(&actual),
        "actual '{}' does not contains '{}'",
        actual,
        expected
    )
}
