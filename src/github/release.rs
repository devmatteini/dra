use crate::github::response::{AssetResponse, ReleaseResponse};
use crate::github::Repository;

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
        let tarball =
            tarball_to_asset(release.tarball_url, source_code(repository, &tag, "tar.gz"));
        let zipball = zipball_to_asset(release.zipball_url, source_code(repository, &tag, "zip"));

        let assets = release
            .assets
            .into_iter()
            .map(Asset::from)
            .chain([tarball, zipball])
            .collect();

        Self { tag, assets }
    }
}

fn tarball_to_asset(url: String, name: String) -> Asset {
    Asset {
        name,
        download_url: url,
        display_name: Some("Source code (tar.gz)".to_string()),
    }
}

fn zipball_to_asset(url: String, name: String) -> Asset {
    Asset {
        name,
        download_url: url,
        display_name: Some("Source code (zip)".to_string()),
    }
}

fn source_code(repository: &Repository, tag: &Tag, extension: &str) -> String {
    format!(
        "{}-{}-source-code.{}",
        repository.repo,
        tag.version(),
        extension
    )
}
