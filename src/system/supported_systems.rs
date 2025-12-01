use crate::github::release::Asset;
use crate::system::core::{Arch, System, OS};
use crate::system::{linux, macos, windows};
use linux::{LinuxArm64, LinuxArmV6, LinuxX86_64};
use macos::{MacOSArm64, MacOSX86_64};
use std::fmt::{Display, Formatter};
use windows::WindowsX86_64;

pub enum SupportedSystem {
    LinuxX86_64(LinuxX86_64),
    LinuxArmV6(LinuxArmV6),
    LinuxArm64(LinuxArm64),
    MacOSX86_64(MacOSX86_64),
    MacOSArm64(MacOSArm64),
    WindowsX86_64(WindowsX86_64),
}

impl System for SupportedSystem {
    fn os(&self) -> OS {
        match self {
            SupportedSystem::LinuxX86_64(system) => system.os(),
            SupportedSystem::LinuxArmV6(system) => system.os(),
            SupportedSystem::LinuxArm64(system) => system.os(),
            SupportedSystem::MacOSX86_64(system) => system.os(),
            SupportedSystem::MacOSArm64(system) => system.os(),
            SupportedSystem::WindowsX86_64(system) => system.os(),
        }
    }

    fn arch(&self) -> Arch {
        match self {
            SupportedSystem::LinuxX86_64(system) => system.arch(),
            SupportedSystem::LinuxArmV6(system) => system.arch(),
            SupportedSystem::LinuxArm64(system) => system.arch(),
            SupportedSystem::MacOSX86_64(system) => system.arch(),
            SupportedSystem::MacOSArm64(system) => system.arch(),
            SupportedSystem::WindowsX86_64(system) => system.arch(),
        }
    }

    fn matches(&self, asset: &Asset) -> bool {
        match self {
            SupportedSystem::LinuxX86_64(system) => system.matches(asset),
            SupportedSystem::LinuxArmV6(system) => system.matches(asset),
            SupportedSystem::LinuxArm64(system) => system.matches(asset),
            SupportedSystem::MacOSX86_64(system) => system.matches(asset),
            SupportedSystem::MacOSArm64(system) => system.matches(asset),
            SupportedSystem::WindowsX86_64(system) => system.matches(asset),
        }
    }

    fn by_asset_priority(&self, asset: &Asset) -> i32 {
        match self {
            SupportedSystem::LinuxX86_64(system) => system.by_asset_priority(asset),
            SupportedSystem::LinuxArmV6(system) => system.by_asset_priority(asset),
            SupportedSystem::LinuxArm64(system) => system.by_asset_priority(asset),
            SupportedSystem::MacOSX86_64(system) => system.by_asset_priority(asset),
            SupportedSystem::MacOSArm64(system) => system.by_asset_priority(asset),
            SupportedSystem::WindowsX86_64(system) => system.by_asset_priority(asset),
        }
    }
}

pub enum SystemError {
    UnknownSystem(String),
}

impl Display for SystemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemError::UnknownSystem(system) => {
                write!(f, "Unknown operating system or architecture: {}", system)
            }
        }
    }
}

pub fn from_environment() -> Result<SupportedSystem, SystemError> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    match (os, arch) {
        ("linux", "x86_64") => Ok(SupportedSystem::LinuxX86_64(LinuxX86_64)),
        ("linux", "arm") => Ok(SupportedSystem::LinuxArmV6(LinuxArmV6)),
        ("linux", "aarch64") => Ok(SupportedSystem::LinuxArm64(LinuxArm64)),
        ("macos", "x86_64") => Ok(SupportedSystem::MacOSX86_64(MacOSX86_64)),
        ("macos", "aarch64") => Ok(SupportedSystem::MacOSArm64(MacOSArm64)),
        ("windows", "x86_64") => Ok(SupportedSystem::WindowsX86_64(WindowsX86_64)),
        _ => Err(SystemError::UnknownSystem(format!("{} {}", os, arch))),
    }
}
