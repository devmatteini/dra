use crate::github::release::{Asset, Tag};

pub struct TaggedAsset;

impl TaggedAsset {
    pub fn untag(tag: &Tag, asset: &Asset) -> String {
        asset.name.replace(&tag.version(), "{tag}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn untag() {
        let result = TaggedAsset::untag(&tag_for("1.5.3"), &asset_for("file-1.5.3-linux.deb"));

        assert_eq!("file-{tag}-linux.deb".to_string(), result);
    }

    #[test]
    fn untag_with_vtag() {
        let result = TaggedAsset::untag(&tag_for("v1.5.3"), &asset_for("file-v1.5.3-linux.deb"));

        assert_eq!("file-v{tag}-linux.deb".to_string(), result);
    }

    #[test]
    fn untag_vtag_without_v_in_file() {
        let result = TaggedAsset::untag(&tag_for("v1.5.3"), &asset_for("file-1.5.3-linux.deb"));

        assert_eq!("file-{tag}-linux.deb".to_string(), result);
    }

    #[test]
    fn no_tag_in_asset_name() {
        let result = TaggedAsset::untag(&tag_for("v1.5.3"), &asset_for("file-linux.deb"));

        assert_eq!("file-linux.deb".to_string(), result);
    }

    fn tag_for(value: &str) -> Tag {
        Tag(value.to_string())
    }

    fn asset_for(name: &str) -> Asset {
        Asset {
            name: name.to_string(),
            download_url: "ANY_DOWNLOAD_URL".to_string(),
        }
    }
}
