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

pub fn assert_error(result: ExecResult) -> String {
    match result {
        ExecResult::Success(x) => {
            panic!("exec succeeded with: {}", x)
        }
        ExecResult::Error(x) => x,
    }
}

pub fn assert_contains(expected: &str, actual: &str) {
    assert!(
        predicates::str::contains(expected).eval(actual),
        "actual '{}' does not contains '{}'",
        actual,
        expected
    )
}
