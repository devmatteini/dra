mod assertions;
mod docker;
mod fs;

mod install {
    use std::process::Command;
    use test_case::test_case;

    use crate::fs::{any_temp_dir, any_temp_file, path_to_string};
    use assert_cmd::assert::OutputAssertExt;
    use assert_cmd::prelude::CommandCargoExt;

    use crate::assertions::assert_file_exists;

    #[test]
    fn destination_is_not_dir() {
        let output = any_temp_file("exec");
        let mut cmd = Command::cargo_bin("dra").unwrap();

        let result = cmd
            .arg("download")
            .args(["-o", &path_to_string(output.clone())])
            .args(["-I", "exec1"])
            .args(["-I", "exec2"])
            .arg("devmatteini/dra-tests")
            .assert();

        result.failure().stderr(predicates::str::contains(
            "Multiple install target (-I,--install-file) are selected \
                but output (-o,--output) is not a directory",
        ));
    }

    #[cfg(target_family = "unix")]
    #[test_case("helloworld-unix", "helloworld", "helloworld-v2"; "executable")]
    #[test_case("helloworld.deb", "helloworld", "helloworld-v2"; "deb")]
    fn selected_asset_is_not_archive(selected_asset: &str, exec1: &str, exec2: &str) {
        let output_dir = any_temp_dir();

        let mut cmd = Command::cargo_bin("dra").unwrap();

        let result = cmd
            .arg("download")
            .args(["-s", selected_asset])
            .args(["-o", &path_to_string(output_dir.clone())])
            .args(["-I", exec1])
            .args(["-I", exec2])
            .arg("devmatteini/dra-tests")
            .assert();

        result.failure().stderr(predicates::str::contains(format!(
            "Selected asset {} is not an archive",
            selected_asset
        )));
    }

    #[cfg(target_family = "unix")]
    #[test_case("helloworld-many-executables-unix.tar.gz", "random-script", "helloworld-v2"; "install 2 executables")]
    fn installed_multiple_successfully(selected_asset: &str, exec1: &str, exec2: &str) {
        let output_dir = any_temp_dir();

        let mut cmd = Command::cargo_bin("dra").unwrap();

        let result = cmd
            .arg("download")
            .args(["-s", selected_asset])
            .args(["-o", &path_to_string(output_dir.clone())])
            .args(["-I", exec1])
            .args(["-I", exec2])
            .arg("devmatteini/dra-tests")
            .assert();

        result
            .success()
            .stdout(predicates::str::contains("Installation completed"));

        assert_file_exists(output_dir.join(exec1).as_path());
        assert_file_exists(output_dir.join(exec2).as_path());
    }

    #[cfg(target_family = "windows")]
    #[test_case("helloworld-many-executables-windows.zip", "random-script.exe", "helloworld-v2.exe"; "install 2 executables")]
    fn installed_multiple_successfully(selected_asset: &str, exec1: &str, exec2: &str) {
        let output_dir = any_temp_dir();

        let mut cmd = Command::cargo_bin("dra").unwrap();

        let result = cmd
            .arg("download")
            .args(["-s", selected_asset])
            .args(["-o", &path_to_string(output_dir.clone())])
            .args(["-I", exec1])
            .args(["-I", exec2])
            .arg("devmatteini/dra-tests")
            .assert();

        result
            .success()
            .stdout(predicates::str::contains("Installation completed"));

        assert_file_exists(output_dir.join(exec1).as_path());
        assert_file_exists(output_dir.join(exec2).as_path());
    }

    #[cfg(target_family = "windows")]
    #[test_case("helloworld.exe", "helloworld.exe", "helloworld-v2.exe"; "executable")]
    fn selected_asset_is_not_archive(selected_asset: &str, exec1: &str, exec2: &str) {
        let output_dir = any_temp_dir();

        let mut cmd = Command::cargo_bin("dra").unwrap();

        let result = cmd
            .arg("download")
            .args(["-s", selected_asset])
            .args(["-o", &path_to_string(output_dir.clone())])
            .args(["-I", exec1])
            .args(["-I", exec2])
            .arg("devmatteini/dra-tests")
            .assert();

        result.failure().stderr(predicates::str::contains(format!(
            "Selected asset {} is not an archive",
            selected_asset
        )));
    }

    #[cfg(target_family = "unix")]
    #[test_case("helloworld.tar.gz", "helloworld"; "tar gzip")]
    #[test_case("helloworld.tgz", "helloworld"; "tar tgz")]
    #[test_case("helloworld.tar.bz2", "helloworld"; "tar bzip2")]
    #[test_case("helloworld.tar.xz", "helloworld"; "tar xz")]
    #[test_case("helloworld.zip", "helloworld"; "zip")]
    #[test_case("helloworld.7z", "helloworld"; "7zip")]
    #[test_case("helloworld-compressed-unix.gz", "helloworld-compressed-unix"; "gzip")]
    #[test_case("helloworld-compressed-unix.bz2", "helloworld-compressed-unix"; "bzip2")]
    #[test_case("helloworld-compressed-unix.xz", "helloworld-compressed-unix"; "xz")]
    #[test_case("helloworld-unix", "helloworld-unix"; "executable")]
    fn installed_successfully(asset: &str, expected_executable: &str) {
        let output_dir = any_temp_dir();

        let mut cmd = Command::cargo_bin("dra").unwrap();

        let result = cmd
            .arg("download")
            .arg("-i")
            .args(["-s", asset])
            .args(["-o", &path_to_string(output_dir.clone())])
            .arg("devmatteini/dra-tests")
            .assert();

        result
            .success()
            .stdout(predicates::str::contains("Installation completed"));

        assert_file_exists(output_dir.join(expected_executable).as_path());
    }

    #[cfg(target_os = "windows")]
    #[test_case("helloworld-windows.tar.gz", "helloworld.exe"; "tar gzip")]
    #[test_case("helloworld-windows.zip", "helloworld.exe"; "zip")]
    #[test_case("helloworld-windows.7z", "helloworld.exe"; "7zip")]
    #[test_case("helloworld-compressed-windows.gz", "helloworld-compressed-windows"; "gzip")]
    #[test_case("helloworld-compressed-windows.bz2", "helloworld-compressed-windows"; "bzip2")]
    #[test_case("helloworld-compressed-windows.xz", "helloworld-compressed-windows"; "xz")]
    #[test_case("helloworld-windows.exe", "helloworld-windows.exe"; "executable")]
    fn installed_successfully(asset: &str, expected_executable: &str) {
        let output_dir = any_temp_dir();

        let mut cmd = Command::cargo_bin("dra").unwrap();

        let result = cmd
            .arg("download")
            .arg("-i")
            .args(["-s", asset])
            .args(["-o", &path_to_string(output_dir.clone())])
            .arg("devmatteini/dra-tests")
            .assert();

        result
            .success()
            .stdout(predicates::str::contains("Installation completed"));

        assert_file_exists(output_dir.join(expected_executable).as_path());
    }

    #[test]
    fn no_executable() {
        let output_dir = path_to_string(any_temp_dir());

        let mut cmd = Command::cargo_bin("dra").unwrap();

        let result = cmd
            .arg("download")
            .arg("-i")
            .args(["-s", "no_executable.tar.gz"])
            .args(["-o", &output_dir])
            .arg("devmatteini/dra-tests")
            .assert();

        result
            .failure()
            .stderr(predicates::str::contains("No executable found"));
    }

    #[cfg(target_family = "unix")]
    #[test_case("helloworld-many-executables-unix.tar.gz", "helloworld-v2"; "install helloworld-v2")]
    fn install_file_successfully(asset: &str, file: &str) {
        let temp_dir = any_temp_dir();
        let expected_installed_file = temp_dir.join(file);
        let output_dir = path_to_string(temp_dir);

        let mut cmd = Command::cargo_bin("dra").unwrap();

        let result = cmd
            .arg("download")
            .args(["-I", file])
            .args(["-s", asset])
            .args(["-o", &output_dir])
            .arg("devmatteini/dra-tests")
            .assert();

        result
            .success()
            .stdout(predicates::str::contains("Installation completed"));
        assert_file_exists(&expected_installed_file);
    }

    #[cfg(target_os = "windows")]
    #[test_case("helloworld-many-executables-windows.zip", "helloworld-v2.exe"; "install helloworld-v2.exe")]
    fn install_file_successfully(asset: &str, file: &str) {
        let temp_dir = any_temp_dir();
        let expected_installed_file = temp_dir.join(file);
        let output_dir = path_to_string(temp_dir);

        let mut cmd = Command::cargo_bin("dra").unwrap();

        let result = cmd
            .arg("download")
            .args(["-I", file])
            .args(["-s", asset])
            .args(["-o", &output_dir])
            .arg("devmatteini/dra-tests")
            .assert();

        result
            .success()
            .stdout(predicates::str::contains("Installation completed"));
        assert_file_exists(&expected_installed_file);
    }
}

mod download {
    use crate::fs::{any_temp_dir, any_temp_file, path_to_string};
    use assert_cmd::Command;

    #[test]
    fn download_source_code_successfully() {
        let output_file = path_to_string(any_temp_file("dra-tests-src"));

        let mut cmd = Command::cargo_bin("dra").unwrap();

        let result = cmd
            .arg("download")
            .args(["-s", "dra-tests-{tag}-source-code.tar.gz"])
            .args(["-o", &output_file])
            .arg("devmatteini/dra-tests")
            .assert();

        let expected = format!("Saved to: {}", &output_file);
        result.success().stdout(predicates::str::contains(expected));
    }

    #[test]
    fn cannot_use_display_name_to_select_asset() {
        let output_file = path_to_string(any_temp_file("dra-tests-any"));

        let mut cmd = Command::cargo_bin("dra").unwrap();

        let result = cmd
            .arg("download")
            .args(["-s", "Source code (tar.gz)"])
            .args(["-o", &output_file])
            .arg("devmatteini/dra-tests")
            .assert();

        result.failure().stderr(predicates::str::contains(
            "No asset found for Source code (tar.gz)",
        ));
    }

    #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
    #[test]
    fn automatic_download() {
        let temp_dir = any_temp_dir();

        let mut cmd = Command::cargo_bin("dra").unwrap();

        cmd.current_dir(&temp_dir)
            .arg("download")
            .arg("-a")
            .arg("devmatteini/dra-tests")
            .assert()
            .success();

        let expected_asset = if cfg!(target_os = "linux") {
            "helloworld-x86_64-linux.tar.gz"
        } else if cfg!(target_os = "windows") {
            "helloworld-x86_64-windows.tar.gz"
        } else if cfg!(target_os = "macos") {
            "helloworld-aarch64-apple-darwin.tar.gz"
        } else {
            panic!("This test should only run on linux, macOS and windows")
        };

        assert!(
            temp_dir.join(expected_asset).exists(),
            "Expected asset '{}' not exists in {}",
            expected_asset,
            temp_dir.display()
        );
    }
}
