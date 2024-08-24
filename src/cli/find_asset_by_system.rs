use crate::github::release::Asset;

pub fn find_asset_by_system(os: &str, arch: &str, assets: Vec<Asset>) -> Option<Asset> {
    let mut matches: Vec<_> = assets
        .into_iter()
        .filter(skip_ignored_asset)
        .filter(|asset| {
            let asset_name = asset.name.to_lowercase();
            let same_arch = is_same_arch(arch, &asset_name);
            let is_same_system = is_same_os(os, &asset_name) && same_arch;
            let is_same_arch_and_extension = same_arch && contains_extension(os, &asset_name);
            is_same_system || is_same_arch_and_extension
        })
        .collect();
    matches.sort_by_key(asset_priority);
    matches.into_iter().next()
}

const IGNORED_ASSETS: [&str; 3] = ["sha256", "sha512", "checksums"];

fn skip_ignored_asset(asset: &Asset) -> bool {
    !IGNORED_ASSETS
        .iter()
        .any(|ignored| asset.name.contains(ignored))
}

fn is_same_os(os: &str, asset_name: &str) -> bool {
    if asset_name.contains(os) {
        return true;
    }
    let aliases: Vec<&str> = match os {
        "macos" => vec!["darwin", "apple", "osx"],
        "windows" => vec!["win64"],
        _ => return false,
    };
    aliases.into_iter().any(|alias| asset_name.contains(alias))
}

fn is_same_arch(arch: &str, asset_name: &str) -> bool {
    if asset_name.contains(arch) {
        return true;
    }
    let aliases: Vec<&str> = match arch {
        "x86_64" => vec!["amd64", "x64"],
        "aarch64" => vec!["arm64"],
        "arm" => vec!["armv6", "armv7"],
        _ => return false,
    };
    aliases.into_iter().any(|alias| asset_name.contains(alias))
}

fn contains_extension(os: &str, asset_name: &str) -> bool {
    let extensions: Vec<&str> = match os {
        "linux" => vec![".appimage"],
        "macos" => vec![".dmg"],
        "windows" => vec![".exe"],
        _ => return false,
    };
    extensions
        .into_iter()
        .any(|extension| asset_name.ends_with(extension))
}

const ARCHIVES: [&str; 7] = [".gz", ".tgz", ".bz2", ".tbz", ".xz", ".txz", ".zip"];

fn asset_priority(a: &Asset) -> i32 {
    if a.name.contains("musl") {
        1
    } else if ARCHIVES.iter().any(|x| a.name.ends_with(x)) {
        2
    } else {
        3
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn asset_found() {
        let assets = vec![
            asset("mypackage-arm-unknown-linux-gnueabihf.tar.gz"),
            asset("mypackage-x86_64-apple-darwin.tar.gz"),
            asset("mypackage-x86_64-unknown-linux-musl.tar.gz"),
        ];

        let result = find_asset_by_system("linux", "x86_64", assets);

        assert_eq_asset("mypackage-x86_64-unknown-linux-musl.tar.gz", result)
    }

    #[test]
    fn found_by_os_alias() {
        let assets = vec![
            asset("mypackage-arm-unknown-linux-gnueabihf.tar.gz"),
            asset("mypackage-x86_64-apple-darwin.tar.gz"),
            asset("mypackage-x86_64-unknown-linux-musl.tar.gz"),
        ];

        let result = find_asset_by_system("macos", "x86_64", assets);

        assert_eq_asset("mypackage-x86_64-apple-darwin.tar.gz", result)
    }

    #[test]
    fn found_by_arch_alias() {
        let assets = vec![
            asset("mypackage-x86_64-apple-darwin.tar.gz"),
            asset("mypackage-linux-amd64.tar.gz"),
        ];

        let result = find_asset_by_system("linux", "x86_64", assets);

        assert_eq_asset("mypackage-linux-amd64.tar.gz", result)
    }

    #[test]
    fn no_matching_asset() {
        let assets = vec![
            asset("mypackage-x86_64-apple-darwin.tar.gz"),
            asset("mypackage-x86_64-unknown-linux-musl.tar.gz"),
        ];

        let result = find_asset_by_system("linux", "arm", assets);

        assert!(result.is_none())
    }

    #[test]
    fn find_asset_case_insensitive() {
        let assets = vec![
            asset("mypackage-x86_64-apple-darwin.tar.gz"),
            asset("mypackage-X86_64-unknown-LiNuX-musl.tar.gz"),
        ];

        let result = find_asset_by_system("linux", "x86_64", assets);

        assert_eq_asset("mypackage-X86_64-unknown-LiNuX-musl.tar.gz", result)
    }

    #[test]
    fn order_assets_by_priority() {
        let mut assets = vec![
            asset("mypackage-linux-amd64.deb"),
            asset("mypackage-linux-gnu.zip"),
            asset("mypackage-linux-x86_64.rpm"),
            asset("mypackage-linux-musl.tar.gz"),
            asset("mypackage-linux-musl"),
        ];

        assets.sort_by_key(asset_priority);

        let actual_names: Vec<_> = assets.into_iter().map(|x| x.name).collect();

        assert_eq!(
            vec![
                "mypackage-linux-musl.tar.gz",
                "mypackage-linux-musl",
                "mypackage-linux-gnu.zip",
                "mypackage-linux-amd64.deb",
                "mypackage-linux-x86_64.rpm",
            ],
            actual_names
        )
    }

    #[test]
    fn found_by_asset_extension_and_arch() {
        let assets = vec![
            asset("mypackage-arm64.AppImage"),
            asset("mypackage-amd64.AppImage"),
        ];

        let result = find_asset_by_system("linux", "x86_64", assets);

        assert_eq_asset("mypackage-amd64.AppImage", result)
    }

    // TODO: this use case could be improved since most of the time when the arch is missing is implicit to be x86_64
    #[test]
    fn not_found_by_asset_extension_without_arch() {
        let assets = vec![
            asset("mypackage-arm64.AppImage"),
            asset("mypackage.AppImage"),
        ];

        let result = find_asset_by_system("linux", "x86_64", assets);

        assert!(result.is_none());
    }

    #[test_case("mypackage-x86_64-linux-musl.sha256sum")]
    #[test_case("mypackage-x86_64-linux-musl.sha256")]
    #[test_case("mypackage-x86_64-linux-musl.sha512")]
    #[test_case("mypackage_checksums.txt")]
    #[test_case("sha256sum.txt")]
    fn skip_ignored_file(ignored_file: &str) {
        let assets = vec![
            asset(ignored_file),
            asset("mypackage-x86_64-linux-musl.tar.gz"),
        ];

        let result = find_asset_by_system("linux", "x86_64", assets);

        assert_eq_asset("mypackage-x86_64-linux-musl.tar.gz", result)
    }

    fn assert_eq_asset(expected_name: &str, actual: Option<Asset>) {
        match actual {
            None => {
                panic!("Asset is None, expected {}", expected_name)
            }
            Some(asset) => assert_eq!(expected_name, asset.name),
        }
    }

    fn asset(name: &str) -> Asset {
        Asset {
            name: name.into(),
            display_name: None,
            download_url: "ANY_DOWNLOAD_URL".into(),
        }
    }
}
