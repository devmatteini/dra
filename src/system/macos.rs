use crate::github::release::Asset;
use crate::system::system::System;

pub struct MacOSAmd64;
impl MacOSAmd64 {
    const OS: &'static str = "macos";
    const ARCH: &'static str = "x86_64";
}

impl System for MacOSAmd64 {
    fn os(&self) -> &str {
        Self::OS
    }
    fn arch(&self) -> &str {
        Self::ARCH
    }
    fn matches(&self, asset: &Asset) -> bool {
        matches(Self::OS, Self::ARCH, asset)
    }
    fn by_asset_priority(&self, asset: &Asset) -> i32 {
        asset_priority(asset)
    }
}

pub struct MacOSArm64;
impl MacOSArm64 {
    const OS: &'static str = "macos";
    const ARCH: &'static str = "aarch64";
}

impl System for MacOSArm64 {
    fn os(&self) -> &str {
        Self::OS
    }
    fn arch(&self) -> &str {
        Self::ARCH
    }
    fn matches(&self, asset: &Asset) -> bool {
        matches(Self::OS, Self::ARCH, asset)
    }
    fn by_asset_priority(&self, asset: &Asset) -> i32 {
        asset_priority(asset)
    }
}

fn matches(os: &str, arch: &str, asset: &Asset) -> bool {
    let asset_name = asset.name.to_lowercase();
    let same_arch = is_same_arch(arch, &asset_name);
    let is_same_system = is_same_os(os, &asset_name) && same_arch;
    let is_same_arch_and_extension = same_arch && contains_extension(&asset_name);
    is_same_system || is_same_arch_and_extension
}

fn is_same_os(os: &str, asset_name: &str) -> bool {
    if asset_name.contains(os) {
        return true;
    }
    let aliases = vec!["darwin", "apple", "osx"];
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

fn contains_extension(asset_name: &str) -> bool {
    let extensions = vec![".dmg"];
    extensions
        .into_iter()
        .any(|extension| asset_name.ends_with(extension))
}

const ARCHIVES: [&str; 7] = [".gz", ".tgz", ".bz2", ".tbz", ".xz", ".txz", ".zip"];

fn asset_priority(a: &Asset) -> i32 {
    let is_archive = ARCHIVES.iter().any(|x| a.name.ends_with(x));

    if is_archive { 1 } else { 2 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asset_found() {
        let asset = any_asset("mypackage-x86_64-apple-darwin.tar.gz");

        let result = matches("macos", "x86_64", &asset);

        assert!(result)
    }

    #[test]
    fn found_by_os_alias() {
        let asset = any_asset("mypackage-x86_64-apple-darwin.tar.gz");

        let result = matches("macos", "x86_64", &asset);

        assert!(result)
    }

    #[test]
    fn found_by_arch_alias() {
        let asset = any_asset("mypackage-amd64-apple-darwin.tar.gz");

        let result = matches("macos", "x86_64", &asset);

        assert!(result)
    }

    #[test]
    fn not_matching() {
        let asset = any_asset("mypackage-x86_64-apple-darwin.tar.gz");

        let result = matches("macos", "arm", &asset);

        assert!(!result)
    }

    #[test]
    fn find_asset_case_insensitive() {
        let asset = any_asset("mypackage-x86_64-dArWIn.tar.gz");

        let result = matches("macos", "x86_64", &asset);

        assert!(result)
    }

    #[test]
    fn order_assets_by_priority() {
        let mut assets = vec![
            any_asset("mypackage-macos-amd64.dmg"),
            any_asset("mypackage-macos.zip"),
            any_asset("mypackage-macos.tar.gz"),
            any_asset("mypackage-macos"),
        ];

        assets.sort_by_key(asset_priority);

        let actual_names: Vec<_> = assets.into_iter().map(|x| x.name).collect();

        assert_eq!(
            vec![
                "mypackage-macos.zip",
                "mypackage-macos.tar.gz",
                "mypackage-macos-amd64.dmg",
                "mypackage-macos",
            ],
            actual_names
        )
    }

    #[test]
    fn found_by_asset_extension_and_arch() {
        let asset = any_asset("mypackage-x86_64.dmg");

        let result = matches("macos", "x86_64", &asset);

        assert!(result)
    }

    // TODO: this use case could be improved since most of the time when the arch is missing is implicit to be x86_64
    #[test]
    fn not_found_by_asset_extension_without_arch() {
        let asset = any_asset("mypackage.dmg");

        let result = matches("macos", "x86_64", &asset);

        assert!(!result);
    }

    fn any_asset(name: &str) -> Asset {
        Asset {
            name: name.into(),
            display_name: None,
            download_url: "ANY_DOWNLOAD_URL".into(),
        }
    }
}
