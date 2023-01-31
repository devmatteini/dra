// NOTE: this rule is not supported by rust-analyzer or JetBrains Rust plugin go to definition/refactoring tools so disable it until it's supported properly
#![allow(clippy::uninlined_format_args)]

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
        actual.contains(expected),
        "actual '{}' does not contains '{}'",
        actual,
        expected
    )
}
