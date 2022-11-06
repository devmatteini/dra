use crate::cli::spinner::Spinner;
use crate::github::client::GithubClient;
use crate::github::error::GithubError;
use crate::github::release_new::{ReleaseNew, TagNew};
use crate::github::Repository;
use crate::{github, Color, HandlerError};

pub fn fetch_release_for(
    client: &GithubClient,
    repository: &Repository,
    tag: Option<&TagNew>,
) -> Result<ReleaseNew, HandlerError> {
    let spinner = Spinner::no_messages();
    spinner.start();

    let release = github::get_release(client, repository, tag).map_err(release_error)?;

    let message = format!("Release tag is {}", Color::new(&release.tag.0).bold());
    spinner.stop_with_message(&message);
    Ok(release)
}

fn release_error(e: GithubError) -> HandlerError {
    HandlerError::new(format!("Error fetching the release: {}", e))
}

pub fn check_has_assets(release: &ReleaseNew) -> Result<(), HandlerError> {
    if release.assets.is_empty() {
        Err(HandlerError::new("No assets found for this release".into()))
    } else {
        Ok(())
    }
}
