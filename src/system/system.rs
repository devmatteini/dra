use crate::github::release::Asset;

pub trait System {
    fn matches(&self, asset: &Asset) -> bool;
    fn by_asset_priority(&self, asset: &Asset) -> i32;
}
