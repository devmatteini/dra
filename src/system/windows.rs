use crate::github::release::Asset;
use crate::system::core::System;

pub struct WindowsAmd64;
impl WindowsAmd64 {
    const OS: &'static str = "windows";
    const ARCH: &'static str = "x86_64";
}

impl System for WindowsAmd64 {
    fn os(&self) -> &str {
        Self::OS
    }
    fn arch(&self) -> &str {
        Self::ARCH
    }
    fn matches(&self, asset: &Asset) -> bool {
        let asset_name = asset.name.to_lowercase();
        let same_arch = is_same_arch(Self::ARCH, &asset_name);
        let is_same_system = is_same_os(Self::OS, &asset_name) && same_arch;
        let is_same_arch_and_extension = same_arch && contains_extension(&asset_name);
        is_same_system || is_same_arch_and_extension
    }
    fn by_asset_priority(&self, asset: &Asset) -> i32 {
        asset_priority(asset)
    }
}

fn is_same_os(os: &str, asset_name: &str) -> bool {
    if asset_name.contains(os) {
        return true;
    }
    let aliases = vec!["win64", "win-64bit"];
    aliases.into_iter().any(|alias| asset_name.contains(alias))
}

fn is_same_arch(arch: &str, asset_name: &str) -> bool {
    if asset_name.contains(arch) {
        return true;
    }
    let aliases: Vec<&str> = match arch {
        "x86_64" => vec!["amd64", "x64", "win64", "win-64bit"],
        _ => return false,
    };
    aliases.into_iter().any(|alias| asset_name.contains(alias))
}

fn contains_extension(asset_name: &str) -> bool {
    let extensions = vec![".exe", ".msi"];
    extensions
        .into_iter()
        .any(|extension| asset_name.ends_with(extension))
}

const ARCHIVES: [&str; 7] = [".gz", ".tgz", ".bz2", ".tbz", ".xz", ".txz", ".zip"];

fn asset_priority(a: &Asset) -> i32 {
    let is_archive = ARCHIVES.iter().any(|x| a.name.ends_with(x));
    let is_exe = a.name.ends_with(".exe");

    if is_archive {
        1
    } else if is_exe {
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
        let asset = any_asset("mypacakge-x86_64-pc-windows-msvc.zip");

        let result = WINDOWS_X86_64.matches(&asset);

        assert!(result)
    }

    #[test]
    fn found_by_os_alias() {
        let asset = any_asset("mypackage-x86_64-win64.zip");

        let result = WINDOWS_X86_64.matches(&asset);

        assert!(result)
    }

    #[test]
    fn found_by_arch_alias() {
        let asset = any_asset("mypackage-windows-amd64.tar.gz");

        let result = WINDOWS_X86_64.matches(&asset);

        assert!(result)
    }

    #[test]
    fn not_matching() {
        let asset = any_asset("mypackage-arm64-windows.zip");

        let result = WINDOWS_X86_64.matches(&asset);

        assert!(!result)
    }

    #[test]
    fn find_asset_case_insensitive() {
        let asset = any_asset("mypackage-X86_64-wINdoWs.zip");

        let result = WINDOWS_X86_64.matches(&asset);

        assert!(result)
    }

    #[test]
    fn order_assets_by_priority() {
        let mut assets = vec![
            any_asset("mypackage-windows-amd64.exe"),
            any_asset("mypackage-windows-x86_64.zip"),
            any_asset("mypackage-windows-amd64.msi"),
            any_asset("mypackage-windows-amd64.tar.gz"),
        ];

        assets.sort_by_key(asset_priority);

        let actual_names: Vec<_> = assets.into_iter().map(|x| x.name).collect();

        assert_eq!(
            vec![
                "mypackage-windows-x86_64.zip",
                "mypackage-windows-amd64.tar.gz",
                "mypackage-windows-amd64.exe",
                "mypackage-windows-amd64.msi"
            ],
            actual_names
        )
    }

    #[test]
    fn found_by_asset_extension_and_arch() {
        let asset = any_asset("mypackage-x86_64.exe");

        let result = WINDOWS_X86_64.matches(&asset);

        assert!(result)
    }

    // TODO: this use case could be improved since most of the time when the arch is missing is implicit to be x86_64
    #[test]
    fn not_found_by_asset_extension_without_arch() {
        let asset = any_asset("mypackage.exe");

        let result = WINDOWS_X86_64.matches(&asset);

        assert!(!result);
    }

    #[test_case("mypackage-win64.zip")]
    #[test_case("mypackage-win-64bit.zip")]
    fn os_and_arch_aliases(asset_name: &str) {
        let asset = any_asset(asset_name);

        let result = WINDOWS_X86_64.matches(&asset);

        assert!(result)
    }

    fn any_asset(name: &str) -> Asset {
        Asset {
            name: name.into(),
            display_name: None,
            download_url: "ANY_DOWNLOAD_URL".into(),
        }
    }

    const WINDOWS_X86_64: WindowsAmd64 = WindowsAmd64 {};
}
