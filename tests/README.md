# Integration tests

## Normal Tests

These are rust tests that can run on your machine that does not affect your system.

Example: [integration_tests.rs](integration_tests.rs)

## Sandbox Tests

:warning: It currently only works on linux

When your tests affect your system, like installing a `.deb` release asset, you should use a sandbox environment.

An [ubuntu docker image](../devtools/Dockerfile.ubuntu) is provided as test environment with `dra` executable.

### Test architecture

Tests are written in rust. It's possible to interact with docker api through a custom wrapper called [Docker](docker/mod.rs).

### How to write a test

1. Start docker container `dra-ubuntu` with `Docker::run()`
2. Execute `dra` command to be tested and wait for its result using `Docker::exec()`.

   You need to use [devmatteini/dra-tests](https://github.com/devmatteini/dra-tests) repository.
3. Do assertions on command result (you can find helpers methods in [assertions](assertions/mod.rs) module)

**Note**: when the docker container started in step 1 goes out of scope, is then stopped in background.

Example:

```rust
use crate::assertions::{assert_contains, assert_success};
use crate::docker::{images, Docker, ExecArgs};

#[test]
fn print_right_version() {
    let container = Docker::run(images::UBUNTU);

    let result = container.exec("dra --version", ExecArgs::Default);

    let output = assert_success(result);
    assert_contains("0.2.3", &output);
}
```
