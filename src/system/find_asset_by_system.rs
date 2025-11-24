use crate::github::release::Asset;
use crate::system::system::System;

pub fn find_asset_by_system(system: &impl System, assets: Vec<Asset>) -> Option<Asset> {
    let mut matches: Vec<_> = assets
        .into_iter()
        .filter(skip_ignored_asset)
        .filter(|asset| system.matches(asset))
        .collect();
    matches.sort_by_key(|asset| system.by_asset_priority(asset));
    matches.into_iter().next()
}

const IGNORED_ASSETS: [&str; 3] = ["sha256", "sha512", "checksums"];

fn skip_ignored_asset(asset: &Asset) -> bool {
    !IGNORED_ASSETS
        .iter()
        .any(|ignored| asset.name.contains(ignored))
}

#[cfg(test)]
mod acceptance_tests {
    use super::*;

    #[test]
    fn asset_found() {
        let system = FixedAssetSystem {
            asset: "mypackage-x86_64-unknown-linux-musl.tar.gz".to_string(),
        };
        let assets = vec![
            asset("mypackage-arm-unknown-linux-gnueabihf.tar.gz"),
            asset("mypackage-x86_64-apple-darwin.tar.gz"),
            asset("mypackage-x86_64-unknown-linux-musl.tar.gz"),
        ];

        let result = find_asset_by_system(&system, assets);

        assert_eq_asset("mypackage-x86_64-unknown-linux-musl.tar.gz", result)
    }

    #[test]
    fn no_matching_asset() {
        let system = FixedAssetSystem {
            asset: "mypackage-arm-unknown-linux-gnueabihf.tar.gz".to_string(),
        };
        let assets = vec![
            asset("mypackage-x86_64-apple-darwin.tar.gz"),
            asset("mypackage-x86_64-unknown-linux-musl.tar.gz"),
        ];

        let result = find_asset_by_system(&system, assets);

        assert!(result.is_none())
    }

    struct FixedAssetSystem {
        asset: String,
    }
    impl System for FixedAssetSystem {
        fn os(&self) -> &str {
            "any"
        }
        fn arch(&self) -> &str {
            "any"
        }
        fn matches(&self, asset: &Asset) -> bool {
            self.asset == asset.name
        }

        fn by_asset_priority(&self, _asset: &Asset) -> i32 {
            1
        }
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
