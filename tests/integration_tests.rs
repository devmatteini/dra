mod archives {
    use std::path::PathBuf;
    use std::process::Command;
    use test_case::test_case;

    use assert_cmd::assert::OutputAssertExt;
    use assert_cmd::prelude::CommandCargoExt;

    #[cfg(target_family = "unix")]
    #[test_case("helloworld.tar.gz"; "tar gzip")]
    #[test_case("helloworld.tar.bz2"; "tar bzip2")]
    #[test_case("helloworld.tar.xz"; "tar xz")]
    #[test_case("helloworld.zip"; "zip")]
    fn installed_successfully(asset: &str) {
        let output_dir = path_to_string(any_temp_dir());

        let mut cmd = Command::cargo_bin("dra").unwrap();

        let result = cmd
            .arg("download")
            .args(["-s", asset])
            .args(["-o", &output_dir])
            .arg("-i")
            .arg("devmatteini/dra-tests")
            .assert();

        result
            .success()
            .stdout(predicates::str::contains("Installation completed"));
    }

    #[cfg(target_os = "windows")]
    #[test_case("helloworld-windows.tar.gz"; "tar gzip")]
    #[test_case("helloworld-windows.zip"; "zip")]
    fn installed_successfully(asset: &str) {
        let output_dir = path_to_string(any_temp_dir());

        let mut cmd = Command::cargo_bin("dra").unwrap();

        let result = cmd
            .arg("download")
            .args(["-s", asset])
            .args(["-o", &output_dir])
            .arg("-i")
            .arg("devmatteini/dra-tests")
            .assert();

        result
            .success()
            .stdout(predicates::str::contains("Installation completed"));
    }

    #[test]
    fn no_executable() {
        let output_dir = path_to_string(any_temp_dir());

        let mut cmd = Command::cargo_bin("dra").unwrap();

        let result = cmd
            .arg("download")
            .args(["-s", "no_executable.tar.gz"])
            .args(["-o", &output_dir])
            .arg("-i")
            .arg("devmatteini/dra-tests")
            .assert();

        result
            .failure()
            .stderr(predicates::str::contains("No executable found"));
    }

    fn any_temp_dir() -> PathBuf {
        let name = uuid::Uuid::new_v4().simple().to_string();
        let path = std::env::temp_dir()
            .join("dra-integration-tests")
            .join(name);
        std::fs::create_dir_all(&path).unwrap();
        path
    }

    fn path_to_string(path: PathBuf) -> String {
        path.to_str().unwrap().to_owned()
    }
}
