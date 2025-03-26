mod assertions;
mod docker;

use crate::assertions::{assert_contains, assert_error, assert_success};
use crate::docker::{Docker, ExecArgs, images, users};

#[test]
fn installed_successfully() {
    let container = Docker::run(images::UBUNTU);

    let result = container.exec(
        "dra download -i -s helloworld.deb devmatteini/dra-tests",
        ExecArgs::Default,
    );

    let output = assert_success(result);
    assert_contains("Installation completed", &output);
}

#[test]
fn wrong_privileges() {
    let container = Docker::run(images::UBUNTU);

    let result = container.exec(
        "dra download -i -s helloworld.deb devmatteini/dra-tests",
        ExecArgs::User(users::TESTER.into()),
    );

    let output = assert_error(result);
    assert_contains("dpkg", &output);
    assert_contains("requires superuser privilege", &output);
}
