use crate::github::release_response::{AssetResponse, ReleaseResponse};
use crate::github::repository::Repository;

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

impl Release {
    pub fn from_response(release: ReleaseResponse, repository: &Repository) -> Self {
        let tag = Tag(release.tag_name);

        let source_code_base = source_code(repository, &tag);
        let tarball = tarball_asset(release.tarball_url, &source_code_base);
        let zipball = zipball_asset(release.zipball_url, &source_code_base);

        let assets = release
            .assets
            .into_iter()
            .map(Asset::from)
            .chain([tarball, zipball])
            .collect();

        Self { tag, assets }
    }
}

impl Asset {
    pub fn show_name(&self) -> &str {
        self.display_name.as_ref().unwrap_or(&self.name)
    }

    pub fn is_same_name(&self, name: &str) -> bool {
        self.display_name
            .as_deref()
            .filter(|&n| n == name)
            .is_some()
            || self.name == name
    }
}

fn tarball_asset(url: String, base_name: &str) -> Asset {
    Asset {
        name: format!("{}.tar.gz", base_name),
        download_url: url,
        display_name: Some("Source code (tar.gz)".to_string()),
    }
}

fn zipball_asset(url: String, base_name: &str) -> Asset {
    Asset {
        name: format!("{}.zip", base_name),
        download_url: url,
        display_name: Some("Source code (zip)".to_string()),
    }
}

fn source_code(repository: &Repository, tag: &Tag) -> String {
    format!("{}-{}-source-code", repository.repo, tag.version(),)
}
