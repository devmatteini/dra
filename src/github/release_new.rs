use crate::github::release::{Asset, Release};

#[derive(Debug)]
pub struct TagNew(pub String);

impl TagNew {
    pub fn version(&self) -> String {
        self.0.replace('v', "")
    }
}

#[derive(Debug)]
pub struct ReleaseNew {
    pub tag: TagNew,
    pub tarball: String,
    pub zipball: String,
    pub assets: Vec<AssetNew>,
}

#[derive(Debug)]
pub struct AssetNew {
    pub name: String,
    pub display_name: Option<String>,
    pub download_url: String,
}

impl From<Asset> for AssetNew {
    fn from(asset: Asset) -> Self {
        Self {
            name: asset.name,
            download_url: asset.browser_download_url,
            display_name: None,
        }
    }
}

impl From<Release> for ReleaseNew {
    fn from(release: Release) -> Self {
        let assets = release.assets.into_iter().map(AssetNew::from).collect();
        Self {
            tag: TagNew(release.tag_name),
            tarball: release.tarball_url,
            zipball: release.zipball_url,
            assets,
        }
    }
}
