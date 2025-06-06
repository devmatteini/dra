// NOTE: this rule is not supported by rust-analyzer or JetBrains Rust plugin go to definition/refactoring tools so disable it until it's supported properly
#![allow(clippy::uninlined_format_args)]
// NOTE: this is needed because clippy gives false positives when compiling each integration test in different crates
#![allow(dead_code)]

use crate::docker::ExecResult;
use std::path::Path;

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

pub fn assert_file_exists(path: &Path) {
    if !path.exists() {
        panic!("File does not exist: {:?}", path)
    }
}

pub fn assert_file_not_exists(path: &Path) {
    if path.exists() {
        panic!("File exist: {:?}", path)
    }
}
