use crate::github::release::Release;
use crate::HandlerError;

pub fn check_has_assets(release: &Release) -> Result<(), HandlerError> {
    if release.assets.is_empty() {
        Err(HandlerError::new("No assets found for this release".into()))
    } else {
        Ok(())
    }
}
