use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AssetId(pub u64);

#[derive(Deserialize, Debug)]
pub struct Tag(pub String);

impl Tag {
    pub fn version(&self) -> String {
        self.0.replace("v", "")
    }
}

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub id: AssetId,
    pub name: String,
    #[serde(rename(deserialize = "browser_download_url"))]
    pub download_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Release {
    #[serde(rename(deserialize = "tag_name"))]
    pub tag: Tag,
    pub assets: Vec<Asset>,
}
