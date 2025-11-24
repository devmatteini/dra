use crate::github::release::Asset;

pub trait System {
    fn os(&self) -> &str;
    fn arch(&self) -> &str;
    fn matches(&self, asset: &Asset) -> bool;
    fn by_asset_priority(&self, asset: &Asset) -> i32;
}
