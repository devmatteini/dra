use crate::cli::color::Color;
use crate::cli::result::HandlerError;
use crate::cli::spinner::Spinner;
use crate::github::client::GithubClient;
use crate::github::error::GithubError;
use crate::github::release::{Release, Tag};
use crate::github::repository::Repository;

pub fn fetch_release_for(
    github: &GithubClient,
    repository: &Repository,
    tag: Option<&Tag>,
) -> Result<Release, HandlerError> {
    let spinner = Spinner::empty_layout();
    spinner.show();

    let release = github.get_release(repository, tag).map_err(release_error)?;

    let message = format!("Release tag is {}", Color::new(&release.tag.0).bold());
    spinner.finish_with_message(&message);
    Ok(release)
}

fn release_error(e: GithubError) -> HandlerError {
    HandlerError::new(format!("Error fetching the release: {}", e))
}

pub fn check_has_assets(release: &Release) -> Result<(), HandlerError> {
    if release.assets.is_empty() {
        Err(HandlerError::new("No assets found for this release".into()))
    } else {
        Ok(())
    }
}
