use crate::github::release::{Asset, Tag};

pub struct TaggedAsset;

impl TaggedAsset {
    fn placeholder() -> &'static str {
        "{tag}"
    }

    pub fn tag(tag: &Tag, untagged: String) -> String {
        untagged.replace(Self::placeholder(), &tag.version())
    }

    pub fn untag(tag: &Tag, asset: &Asset) -> String {
        asset.name.replace(&tag.version(), Self::placeholder())
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

    #[test]
    fn tag() {
        let result = TaggedAsset::tag(&tag_for("1.5.3"), "file-{tag}-linux.deb".to_string());

        assert_eq!("file-1.5.3-linux.deb".to_string(), result);
    }

    #[test]
    fn tag_with_vtag() {
        let result = TaggedAsset::tag(&tag_for("v1.5.3"), "file-v{tag}-linux.deb".to_string());

        assert_eq!("file-v1.5.3-linux.deb".to_string(), result);
    }

    #[test]
    fn tag_vtag_without_v_in_file() {
        let result = TaggedAsset::tag(&tag_for("v1.5.3"), "file-{tag}-linux.deb".to_string());

        assert_eq!("file-1.5.3-linux.deb".to_string(), result);
    }

    #[test]
    fn tag_no_tag_in_asset_name() {
        let result = TaggedAsset::tag(&tag_for("v1.5.3"), "file-linux.deb".to_string());

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
