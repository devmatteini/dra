use crate::github::response::{AssetResponse, ReleaseResponse};

#[derive(Debug)]
pub struct Tag(pub String);

impl Tag {
    pub fn version(&self) -> String {
        self.0.replace('v', "")
    }
}

#[derive(Debug)]
pub struct Release {
    pub tag: Tag,
    pub tarball: String,
    pub zipball: String,
    pub assets: Vec<Asset>,
}

#[derive(Debug)]
pub struct Asset {
    pub name: String,
    pub display_name: Option<String>,
    pub download_url: String,
}

impl From<AssetResponse> for Asset {
    fn from(asset: AssetResponse) -> Self {
        Self {
            name: asset.name,
            download_url: asset.browser_download_url,
            display_name: None,
        }
    }
}

impl From<ReleaseResponse> for Release {
    fn from(release: ReleaseResponse) -> Self {
        let assets = release.assets.into_iter().map(Asset::from).collect();
        Self {
            tag: Tag(release.tag_name),
            tarball: release.tarball_url,
            zipball: release.zipball_url,
            assets,
        }
    }
}
