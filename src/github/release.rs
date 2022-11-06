use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub name: String,
    pub browser_download_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Release {
    pub tag_name: String,
    pub tarball_url: String,
    pub zipball_url: String,
    pub assets: Vec<Asset>,
}
