use crate::github::release::Asset;

pub trait System {
    const OS: &'static str;
    const ARCH: &'static str;

    fn matches(&self, asset: &Asset) -> bool;
    fn by_asset_priority(&self, asset: &Asset) -> i32;
}
