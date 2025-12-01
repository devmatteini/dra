use crate::github::release::Asset;
use std::fmt::Formatter;

pub enum OS {
    Linux,
    Mac,
    Windows,
}

pub enum Arch {
    X86_64,
    ArmV6,
    Arm64,
}

pub trait System {
    fn os(&self) -> OS;
    fn arch(&self) -> Arch;
    fn matches(&self, asset: &Asset) -> bool;
    fn by_asset_priority(&self, asset: &Asset) -> i32;
}

impl OS {
    pub fn as_str(&self) -> &str {
        match self {
            OS::Linux => "linux",
            OS::Mac => "macos",
            OS::Windows => "windows",
        }
    }
}

impl std::fmt::Display for OS {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Arch {
    pub fn as_str(&self) -> &str {
        match self {
            Arch::X86_64 => "x86_64",
            Arch::ArmV6 => "arm",
            Arch::Arm64 => "aarch64",
        }
    }
}

impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
