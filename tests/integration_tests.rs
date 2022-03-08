mod assertions;
mod docker;

#[cfg(test)]
mod debian {
    use crate::assertions::{assert_contains, assert_error, assert_success};
    use crate::docker::Docker;

    #[test]
    fn installed_successfully() {
        let container = Docker::run("dra-ubuntu");

        // FIXME: create repo instead of using external repos :)
        let result = container.exec(
            "dra sharkdp/bat download -s bat_{tag}_amd64.deb -i",
            Docker::NO_ARGS,
        );

        let output = assert_success(result);
        assert_contains("Installation completed", &output);

        container.stop();
    }

    #[test]
    fn wrong_privileges() {
        let container = Docker::run("dra-ubuntu");

        // FIXME: create repo instead of using external repos :)
        let result = container.exec(
            "dra sharkdp/bat download -s bat_{tag}_amd64.deb -i",
            &["--user", "tester"],
        );

        let output = assert_error(result);
        assert_contains("dpkg", &output);
        assert_contains("requires superuser privilege", &output);

        container.stop();
    }
}
