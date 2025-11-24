use crate::github::release::Asset;
use crate::system::core::System;
use crate::system::{linux, macos, windows};
use linux::{LinuxAmd64, LinuxArm64, LinuxArmV6};
use macos::{MacOSAmd64, MacOSArm64};
use std::fmt::{Display, Formatter};
use windows::WindowsAmd64;

pub enum SupportedSystem {
    LinuxAmd64(LinuxAmd64),
    LinuxArmV6(LinuxArmV6),
    LinuxArm64(LinuxArm64),
    MacOSAmd64(MacOSAmd64),
    MacOSArm64(MacOSArm64),
    WindowsAmd64(WindowsAmd64),
}

impl System for SupportedSystem {
    fn os(&self) -> &str {
        match self {
            SupportedSystem::LinuxAmd64(system) => system.os(),
            SupportedSystem::LinuxArmV6(system) => system.os(),
            SupportedSystem::LinuxArm64(system) => system.os(),
            SupportedSystem::MacOSAmd64(system) => system.os(),
            SupportedSystem::MacOSArm64(system) => system.os(),
            SupportedSystem::WindowsAmd64(system) => system.os(),
        }
    }

    fn arch(&self) -> &str {
        match self {
            SupportedSystem::LinuxAmd64(system) => system.arch(),
            SupportedSystem::LinuxArmV6(system) => system.arch(),
            SupportedSystem::LinuxArm64(system) => system.arch(),
            SupportedSystem::MacOSAmd64(system) => system.arch(),
            SupportedSystem::MacOSArm64(system) => system.arch(),
            SupportedSystem::WindowsAmd64(system) => system.arch(),
        }
    }

    fn matches(&self, asset: &Asset) -> bool {
        match self {
            SupportedSystem::LinuxAmd64(system) => system.matches(asset),
            SupportedSystem::LinuxArmV6(system) => system.matches(asset),
            SupportedSystem::LinuxArm64(system) => system.matches(asset),
            SupportedSystem::MacOSAmd64(system) => system.matches(asset),
            SupportedSystem::MacOSArm64(system) => system.matches(asset),
            SupportedSystem::WindowsAmd64(system) => system.matches(asset),
        }
    }

    fn by_asset_priority(&self, asset: &Asset) -> i32 {
        match self {
            SupportedSystem::LinuxAmd64(system) => system.by_asset_priority(asset),
            SupportedSystem::LinuxArmV6(system) => system.by_asset_priority(asset),
            SupportedSystem::LinuxArm64(system) => system.by_asset_priority(asset),
            SupportedSystem::MacOSAmd64(system) => system.by_asset_priority(asset),
            SupportedSystem::MacOSArm64(system) => system.by_asset_priority(asset),
            SupportedSystem::WindowsAmd64(system) => system.by_asset_priority(asset),
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
        ("linux", "x86_64") => Ok(SupportedSystem::LinuxAmd64(LinuxAmd64)),
        ("linux", "arm") => Ok(SupportedSystem::LinuxArmV6(LinuxArmV6)),
        ("linux", "aarch64") => Ok(SupportedSystem::LinuxArm64(LinuxArm64)),
        ("macos", "x86_64") => Ok(SupportedSystem::MacOSAmd64(MacOSAmd64)),
        ("macos", "aarch64") => Ok(SupportedSystem::MacOSArm64(MacOSArm64)),
        ("windows", "x86_64") => Ok(SupportedSystem::WindowsAmd64(WindowsAmd64)),
        _ => Err(SystemError::UnknownSystem(format!("{} {}", os, arch))),
    }
}
