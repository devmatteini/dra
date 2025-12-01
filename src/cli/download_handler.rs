use crate::cli::color::Color;
use crate::cli::github_release::fetch_release_for;
use crate::cli::progress_bar::ProgressBar;
use crate::cli::result::{HandlerError, HandlerResult};
use crate::cli::select_assets;
use crate::cli::spinner::Spinner;
use crate::github::client::GithubClient;
use crate::github::error::GithubError;
use crate::github::release::{Asset, Release, Tag};
use crate::github::repository::Repository;
use crate::github::tagged_asset::TaggedAsset;
use crate::installer::destination::Destination;
use crate::installer::executable::Executable;
use crate::installer::install;
use crate::{system, vector};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub struct DownloadHandler {
    repository: Repository,
    mode: DownloadMode,
    tag: Option<Tag>,
    output: Option<PathBuf>,
    install: Install,
}

enum DownloadMode {
    Interactive,
    Selection(String),
    Automatic,
}

impl DownloadMode {
    fn new(select: Option<String>, automatic: bool) -> Self {
        match (select, automatic) {
            (Some(x), _) => Self::Selection(x),
            (_, true) => Self::Automatic,
            (None, false) => Self::Interactive,
        }
    }
}

enum Install {
    No,
    Yes(Vec<Executable>),
}

impl Install {
    fn new(install: bool, install_file: Option<Vec<String>>, repository: &Repository) -> Self {
        match (install_file, install) {
            (Some(executable_names), _) => Self::Yes(
                vector::unique(executable_names)
                    .into_iter()
                    .map(Executable::Selected)
                    .collect(),
            ),
            (_, true) => Self::Yes(vec![Executable::Automatic(repository.repo.clone())]),
            (None, false) => Self::No,
        }
    }

    fn as_bool(&self) -> bool {
        match self {
            Self::No => false,
            Self::Yes(_) => true,
        }
    }

    fn is_more_than_one(&self) -> bool {
        match self {
            Self::No => false,
            Self::Yes(x) => x.len() > 1,
        }
    }
}

impl DownloadHandler {
    pub fn new(
        repository: Repository,
        select: Option<String>,
        automatic: bool,
        tag: Option<String>,
        output: Option<PathBuf>,
        install: bool,
        install_file: Option<Vec<String>>,
    ) -> Self {
        let install = Install::new(install, install_file, &repository);
        DownloadHandler {
            repository,
            mode: DownloadMode::new(select.clone(), automatic),
            tag: tag.map(Tag),
            output,
            install,
        }
    }

    pub fn run(&self) -> HandlerResult {
        let github = GithubClient::from_environment();
        let release = self.fetch_release(&github)?;
        let selected_asset = self.select_asset(release)?;
        let output_path = self.choose_output_path(&selected_asset.name);
        Self::download_asset(&github, &selected_asset, &output_path)?;
        self.maybe_install(&selected_asset.name, &output_path)?;
        Ok(())
    }

    fn fetch_release(&self, github: &GithubClient) -> Result<Release, HandlerError> {
        fetch_release_for(github, &self.repository, self.tag.as_ref())
    }

    fn select_asset(&self, release: Release) -> Result<Asset, HandlerError> {
        match &self.mode {
            DownloadMode::Interactive => ask_select_asset(release.assets),
            DownloadMode::Selection(untagged) => autoselect_asset(release, untagged),
            DownloadMode::Automatic => {
                let system = system::from_environment().map_err(|e| {
                    automatic_download_system_error(&self.repository, &release.tag, e)
                })?;
                system::find_asset_by_system(&system, release.assets).ok_or_else(|| {
                    automatic_download_error(&self.repository, &release.tag, &system)
                })
            }
        }
    }

    fn choose_output_path(&self, asset_name: &str) -> PathBuf {
        choose_output_path_from(
            self.output.as_ref(),
            self.install.as_bool(),
            asset_name,
            Path::is_dir,
        )
    }

    fn download_asset(
        github: &GithubClient,
        selected_asset: &Asset,
        output_path: &Path,
    ) -> Result<(), HandlerError> {
        let progress_bar = ProgressBar::download_layout(&selected_asset.name, output_path);
        progress_bar.show();
        let (mut stream, maybe_content_length) = github
            .download_asset_stream(selected_asset)
            .map_err(download_error)?;
        progress_bar.set_length(maybe_content_length);

        let mut destination = create_file(output_path)?;
        let mut total_bytes = 0;
        let mut buffer = [0; 1024];
        while let Ok(bytes) = stream.read(&mut buffer) {
            if bytes == 0 {
                break;
            }

            destination
                .write(&buffer[..bytes])
                .map_err(|x| save_to_file_error(&selected_asset.name, output_path, x))?;

            total_bytes += bytes as u64;
            progress_bar.update_progress(total_bytes);
        }
        progress_bar.finish();
        Ok(())
    }

    fn maybe_install(&self, asset_name: &str, path: &Path) -> Result<(), HandlerError> {
        match &self.install {
            Install::No => Ok(()),
            Install::Yes(executables) => {
                let cwd = cwd()?;
                let destination = match self.output.as_ref() {
                    Some(output) if output.is_dir() => Destination::Directory(output.clone()),
                    Some(output) => Destination::File(output.clone()),
                    None => Destination::Directory(cwd),
                };
                self.check_destination_invariants(&destination)?;

                let spinner = Spinner::install_layout();
                spinner.show();

                let output = install(
                    asset_name.to_string(),
                    path,
                    destination,
                    executables.clone(),
                )
                .map_err(|x| HandlerError::new(x.to_string()))?;

                remove_temporary_file(path)?;

                let message = format!(
                    "{}\n{}",
                    output,
                    Color::new("Installation completed!").green(),
                );
                spinner.finish_with_message(&message);
                Ok(())
            }
        }
    }

    fn check_destination_invariants(&self, destination: &Destination) -> Result<(), HandlerError> {
        if !self.install.is_more_than_one() {
            return Ok(());
        }
        match destination {
            Destination::File(x) => {
                let message = format!(
                    "{} is not a directory. When you specify multiple executables to install, you must provide a directory path",
                    x.display()
                );
                Err(HandlerError::new(message))
            }
            Destination::Directory(_) => Ok(()),
        }
    }
}

fn ask_select_asset(assets: Vec<Asset>) -> select_assets::AskSelectAssetResult {
    select_assets::ask_select_asset(
        assets,
        select_assets::Messages {
            select_prompt: "Pick the asset to download",
            quit_select: "No asset selected",
        },
    )
}

fn autoselect_asset(release: Release, untagged: &str) -> Result<Asset, HandlerError> {
    let asset_name = TaggedAsset::tag(&release.tag, untagged);
    release
        .assets
        .into_iter()
        .find(|x| x.name == asset_name)
        .ok_or_else(|| HandlerError::new(format!("No asset found for {}", untagged)))
}

fn automatic_download_system_error(
    repository: &Repository,
    release: &Tag,
    error: system::SystemError,
) -> HandlerError {
    let title = urlencoding::encode("Error: System error automatic download of asset");
    let body = format!(
        "## dra version\n{}\n## Bug report\nRepository: https://github.com/{}\nRelease: {}\nError: {}\n\n---",
        env!("CARGO_PKG_VERSION"),
        repository,
        release.0,
        error
    );
    let body = urlencoding::encode(&body);
    let issue_url = format!(
        "https://github.com/devmatteini/dra/issues/new?title={}&body={}",
        title, body
    );
    HandlerError::new(format!(
        "There was an error determining your system configuration for automatic download: {}\nPlease report the issue: {}\n",
        error, issue_url
    ))
}

fn automatic_download_error(
    repository: &Repository,
    release: &Tag,
    system: &impl system::System,
) -> HandlerError {
    let title = urlencoding::encode("Error: automatic download of asset");
    let body = format!(
        "## dra version\n{}\n## Bug report\nRepository: https://github.com/{}\nRelease: {}\nOS: {}\nARCH: {}\n\n---",
        env!("CARGO_PKG_VERSION"),
        repository,
        release.0,
        system.os(),
        system.arch()
    );
    let body = urlencoding::encode(&body);
    let issue_url = format!(
        "https://github.com/devmatteini/dra/issues/new?title={}&body={}",
        title, body
    );
    HandlerError::new(format!(
        "Cannot find asset that matches your system {} {}\nIf you think this is a bug, please report the issue: {}",
        system.os(),
        system.arch(),
        issue_url
    ))
}

fn choose_output_path_from<IsDir>(
    output: Option<&PathBuf>,
    install: bool,
    asset_name: &str,
    is_dir: IsDir,
) -> PathBuf
where
    IsDir: FnOnce(&Path) -> bool,
{
    if install {
        return crate::temp_file::temp_file();
    }

    output
        .map(|path| {
            if is_dir(path) {
                path.join(asset_name)
            } else {
                path.to_path_buf()
            }
        })
        .unwrap_or_else(|| PathBuf::from(asset_name))
}

fn download_error(e: GithubError) -> HandlerError {
    HandlerError::new(format!("Error downloading asset: {}", e))
}

fn create_file(path: &Path) -> Result<File, HandlerError> {
    File::create(path)
        .map_err(|e| HandlerError::new(format!("Failed to create file {}: {}", path.display(), e)))
}

fn save_to_file_error(asset_name: &str, output_path: &Path, error: std::io::Error) -> HandlerError {
    HandlerError::new(format!(
        "Error saving {} to {}: {}",
        asset_name,
        output_path.display(),
        error
    ))
}

fn cwd() -> Result<PathBuf, HandlerError> {
    std::env::current_dir()
        .map_err(|x| HandlerError::new(format!("Error retrieving current directory: {}", x)))
}

fn remove_temporary_file(path: &Path) -> Result<(), HandlerError> {
    std::fs::remove_file(path)
        .map_err(|x| HandlerError::new(format!("Unable to delete temporary file: {}", x)))
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    const INSTALL: bool = true;
    const NO_INSTALL: bool = false;
    const ANY_ASSET_NAME: &str = "ANY_ASSET_NAME";

    /// CLI command:
    /// dra download -i -o /some/path <REPO> or dra download -i <REPO>
    #[test_case(Some(PathBuf::from("/some/path")); "any_custom_output")]
    #[test_case(None; "no_output")]
    fn choose_output_install(output: Option<PathBuf>) {
        let result = choose_output_path_from(output.as_ref(), INSTALL, ANY_ASSET_NAME, not_dir);

        assert!(
            result
                .to_str()
                .expect("Error: no path available")
                .contains("dra-")
        )
    }

    /// CLI command:
    /// dra download -s my_asset.deb <REPO>
    /// output: $PWD/my_asset.deb
    #[test]
    fn choose_output_nothing_chosen() {
        let result = choose_output_path_from(None, NO_INSTALL, "my_asset.deb", not_dir);

        assert_eq!(PathBuf::from("my_asset.deb"), result)
    }

    /// CLI command:
    /// dra download -o /some/path.zip <REPO>
    /// output: /some/path.zip
    #[test]
    fn choose_output_custom_file_path() {
        let output = PathBuf::from("/some/path.zip");

        let result = choose_output_path_from(Some(&output), NO_INSTALL, ANY_ASSET_NAME, not_dir);

        assert_eq!(output, result)
    }

    /// CLI command:
    /// dra download -s my_asset.tar.gz -o /my/custom-dir/ <REPO>
    /// output: /my/custom-dir/my_asset.tar.gz
    #[test]
    fn choose_output_custom_directory_path() {
        let output = PathBuf::from("/my/custom-dir/");
        let asset_name = "my_asset.tar.gz";

        let result = choose_output_path_from(Some(&output), NO_INSTALL, asset_name, is_dir);

        let expected = output.join(asset_name);
        assert_eq!(expected, result);
    }

    fn is_dir(_: &Path) -> bool {
        true
    }

    fn not_dir(_: &Path) -> bool {
        false
    }
}
