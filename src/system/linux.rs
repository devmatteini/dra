use crate::github::release::Asset;
use crate::system::core::{Arch, System, OS};

pub struct LinuxAmd64;
impl LinuxAmd64 {
    const OS: OS = OS::Linux;
    const ARCH: Arch = Arch::X86_64;
}

impl System for LinuxAmd64 {
    fn os(&self) -> OS {
        Self::OS
    }
    fn arch(&self) -> Arch {
        Self::ARCH
    }
    fn matches(&self, asset: &Asset) -> bool {
        matches(Self::OS, Self::ARCH, asset)
    }
    fn by_asset_priority(&self, asset: &Asset) -> i32 {
        asset_priority(asset)
    }
}

pub struct LinuxArmV6;
impl LinuxArmV6 {
    const OS: OS = OS::Linux;
    const ARCH: Arch = Arch::ArmV6;
}

impl System for LinuxArmV6 {
    fn os(&self) -> OS {
        Self::OS
    }
    fn arch(&self) -> Arch {
        Self::ARCH
    }
    fn matches(&self, asset: &Asset) -> bool {
        matches(Self::OS, Self::ARCH, asset)
    }
    fn by_asset_priority(&self, asset: &Asset) -> i32 {
        asset_priority(asset)
    }
}

pub struct LinuxArm64;
impl LinuxArm64 {
    const OS: OS = OS::Linux;
    const ARCH: Arch = Arch::Arm64;
}

impl System for LinuxArm64 {
    fn os(&self) -> OS {
        Self::OS
    }
    fn arch(&self) -> Arch {
        Self::ARCH
    }
    fn matches(&self, asset: &Asset) -> bool {
        matches(Self::OS, Self::ARCH, asset)
    }
    fn by_asset_priority(&self, asset: &Asset) -> i32 {
        asset_priority(asset)
    }
}

fn matches(os: OS, arch: Arch, asset: &Asset) -> bool {
    let asset_name = asset.name.to_lowercase();
    let same_arch = is_same_arch(arch, &asset_name);
    let is_same_system = is_same_os(os, &asset_name) && same_arch;
    let is_same_arch_and_extension = same_arch && contains_extension(&asset_name);
    is_same_system || is_same_arch_and_extension
}

fn is_same_os(os: OS, asset_name: &str) -> bool {
    asset_name.contains(os.as_str())
}

fn is_same_arch(arch: Arch, asset_name: &str) -> bool {
    let aliases: Vec<&str> = match arch {
        Arch::X86_64 => vec!["x86_64", "amd64", "x64"],
        Arch::Arm64 => vec!["aarch64", "arm64"],
        Arch::ArmV6 => vec!["arm", "armv6", "armv7"],
    };
    aliases.into_iter().any(|alias| asset_name.contains(alias))
}

fn contains_extension(asset_name: &str) -> bool {
    let extensions = vec![".appimage"];
    extensions
        .into_iter()
        .any(|extension| asset_name.ends_with(extension))
}

const ARCHIVES: [&str; 7] = [".gz", ".tgz", ".bz2", ".tbz", ".xz", ".txz", ".zip"];

fn asset_priority(a: &Asset) -> i32 {
    let is_archive = ARCHIVES.iter().any(|x| a.name.ends_with(x));
    let is_musl = a.name.contains("musl");

    if is_musl && is_archive {
        1
    } else if is_musl {
        2
    } else if is_archive {
        3
    } else {
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asset_found() {
        let asset = any_asset("mypackage-x86_64-unknown-linux-musl.tar.gz");

        let result = matches(LinuxAmd64::OS, LinuxAmd64::ARCH, &asset);

        assert!(result)
    }

    #[test]
    fn found_by_arch_alias() {
        let asset = any_asset("mypackage-linux-amd64.tar.gz");

        let result = matches(LinuxAmd64::OS, LinuxAmd64::ARCH, &asset);

        assert!(result)
    }

    #[test]
    fn not_matching() {
        let asset = any_asset("mypackage-x86_64-unknown-linux-musl.tar.gz");

        let result = matches(LinuxArmV6::OS, LinuxArmV6::ARCH, &asset);

        assert!(!result)
    }

    #[test]
    fn find_asset_case_insensitive() {
        let asset = any_asset("mypackage-X86_64-unknown-LiNuX-musl.tar.gz");

        let result = matches(LinuxAmd64::OS, LinuxAmd64::ARCH, &asset);

        assert!(result)
    }

    #[test]
    fn order_assets_by_priority() {
        let mut assets = vec![
            any_asset("mypackage-linux-amd64.deb"),
            any_asset("mypackage-linux-amd64-musl.deb"),
            any_asset("mypackage-linux-gnu.zip"),
            any_asset("mypackage-linux-x86_64.rpm"),
            any_asset("mypackage-linux-musl.tar.gz"),
            any_asset("mypackage-linux-musl"),
        ];

        assets.sort_by_key(asset_priority);

        let actual_names: Vec<_> = assets.into_iter().map(|x| x.name).collect();

        assert_eq!(
            vec![
                "mypackage-linux-musl.tar.gz",
                "mypackage-linux-amd64-musl.deb",
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
        let asset = any_asset("mypackage-amd64.AppImage");

        let result = matches(LinuxAmd64::OS, LinuxAmd64::ARCH, &asset);

        assert!(result)
    }

    // TODO: this use case could be improved since most of the time when the arch is missing is implicit to be x86_64
    #[test]
    fn not_found_by_asset_extension_without_arch() {
        let asset = any_asset("mypackage.AppImage");

        let result = matches(LinuxAmd64::OS, LinuxAmd64::ARCH, &asset);

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
