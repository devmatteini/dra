mod assertions;
mod docker;

#[cfg(test)]
mod debian {
    use crate::assertions::{assert_contains, assert_error, assert_success};
    use crate::docker::{images, users, Docker, ExecArgs};

    #[test]
    fn installed_successfully() {
        let container = Docker::run(images::UBUNTU);

        let result = container.exec(
            "dra devmatteini/dra-tests download -s helloworld.deb -i",
            ExecArgs::Default,
        );

        let output = assert_success(result);
        assert_contains("Installation completed", &output);
    }

    #[test]
    fn wrong_privileges() {
        let container = Docker::run(images::UBUNTU);

        let result = container.exec(
            "dra devmatteini/dra-tests download -s helloworld.deb -i",
            ExecArgs::User(users::TESTER.into()),
        );

        let output = assert_error(result);
        assert_contains("dpkg", &output);
        assert_contains("requires superuser privilege", &output);
    }
}

#[cfg(test)]
mod tar_archive {
    use crate::assertions::{assert_contains, assert_error, assert_success};
    use crate::docker::{images, Docker, ExecArgs};

    #[test]
    fn installed_successfully() {
        let container = Docker::run(images::UBUNTU);

        let result = container.exec(
            "dra devmatteini/dra-tests download -s helloworld.tar.gz -i",
            ExecArgs::Default,
        );

        let output = assert_success(result);
        assert_contains("Installation completed", &output);
    }

    #[test]
    fn no_executable() {
        let container = Docker::run(images::UBUNTU);

        let result = container.exec(
            "dra devmatteini/dra-tests download -s no_executable.tar.gz -i",
            ExecArgs::Default,
        );

        let output = assert_error(result);
        assert_contains("No executable found", &output);
    }
}
