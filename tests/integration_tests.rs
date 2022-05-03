mod assertions;
mod docker;

mod debian {
    use crate::assertions::{assert_contains, assert_error, assert_success};
    use crate::docker::{images, users, Docker, ExecArgs};

    #[test]
    fn installed_successfully() {
        let container = Docker::run(images::UBUNTU);

        let result = container.exec(
            "dra download -s helloworld.deb -i devmatteini/dra-tests",
            ExecArgs::Default,
        );

        let output = assert_success(result);
        assert_contains("Installation completed", &output);
    }

    #[test]
    fn wrong_privileges() {
        let container = Docker::run(images::UBUNTU);

        let result = container.exec(
            "dra download -s helloworld.deb -i devmatteini/dra-tests",
            ExecArgs::User(users::TESTER.into()),
        );

        let output = assert_error(result);
        assert_contains("dpkg", &output);
        assert_contains("requires superuser privilege", &output);
    }
}

mod archives {
    use crate::assertions::{assert_contains, assert_error, assert_success};
    use crate::docker::{images, Docker, ExecArgs};
    use test_case::test_case;

    #[test_case("helloworld.tar.gz"; "tar gzip")]
    #[test_case("helloworld.tar.bz2"; "tar bzip2")]
    #[test_case("helloworld.tar.xz"; "tar xz")]
    #[test_case("helloworld.zip"; "zip")]
    fn installed_successfully(asset: &str) {
        let container = Docker::run(images::UBUNTU);

        let result = container.exec(
            &format!("dra download -s {} -i devmatteini/dra-tests", asset),
            ExecArgs::Default,
        );

        let output = assert_success(result);
        assert_contains("Installation completed", &output);
    }

    #[test]
    fn no_executable() {
        let container = Docker::run(images::UBUNTU);

        let result = container.exec(
            "dra download -s no_executable.tar.gz -i devmatteini/dra-tests",
            ExecArgs::Default,
        );

        let output = assert_error(result);
        assert_contains("No executable found", &output);
    }

    #[test]
    fn no_root_directory() {
        let container = Docker::run(images::UBUNTU);

        let result = container.exec(
            "dra download -s no_root_directory.tar.gz -i devmatteini/dra-tests",
            ExecArgs::Default,
        );

        let output = assert_success(result);
        assert_contains("Installation completed", &output);
    }
}
