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
    use test_case::test_case;

    #[test_case("1.5.3", "file-1.5.3-linux.deb", "file-{tag}-linux.deb"; "only version")]
    #[test_case("v1.5.3", "file-v1.5.3-linux.deb", "file-v{tag}-linux.deb"; "v-tag")]
    #[test_case("v1.5.3", "file-1.5.3-linux.deb", "file-{tag}-linux.deb"; "v-tag but asset only version")]
    fn untag(tag: &str, asset_name: &str, expected: &str) {
        let result = TaggedAsset::untag(&tag_for(tag), &asset_for(asset_name));

        assert_eq!(expected.to_string(), result);
    }

    #[test]
    fn untag_no_tag_in_asset_name() {
        let result = TaggedAsset::untag(&tag_for("v1.5.3"), &asset_for("file-linux.deb"));

        assert_eq!("file-linux.deb".to_string(), result);
    }

    #[test_case("1.5.3", "file-{tag}-linux.deb", "file-1.5.3-linux.deb"; "only version")]
    #[test_case("v1.5.3", "file-v{tag}-linux.deb", "file-v1.5.3-linux.deb"; "v-tag")]
    #[test_case("v1.5.3", "file-{tag}-linux.deb", "file-1.5.3-linux.deb"; "v-tag but asset only version")]
    fn tag(tag: &str, untagged: &str, expected: &str) {
        let result = TaggedAsset::tag(&tag_for(tag), untagged.to_string());

        assert_eq!(expected.to_string(), result);
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
