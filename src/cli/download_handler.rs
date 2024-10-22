use crate::cli::color::Color;
use crate::cli::find_asset_by_system::find_asset_by_system;
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
use crate::installer::file::is_supported_archive;
use crate::installer::install;
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
    Yes(Executable),
}

impl Install {
    fn new(install: bool, install_file: Option<String>, repository: &Repository) -> Self {
        match (install_file, install) {
            (Some(executable_name), _) => Self::Yes(Executable::Selected(executable_name)),
            (_, true) => Self::Yes(Executable::Automatic(repository.repo.clone())),
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
        false
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
        install_file: Option<String>,
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
        /*
           dra download -s archive.tar -o /any/fle -I exec1 -I exec2 any/repo
           This command should failed as install destination is a file
        */
        let destination = self.selection_destination_for_install()?;
        let github = GithubClient::from_environment();
        let release = self.fetch_release(&github)?;
        let selected_asset = self.select_asset(release)?;
        /*
           dra download -s not.archive -I exec1 -I exec2 any/repo
           dra download -a -I exec1 -I exec2 any/repo
           This command should failed if selected asset is not an archive
        */
        self.asset_may_not_be_archive(&selected_asset.name)?;
        let output_path = self.choose_output_path(&selected_asset.name);
        Self::download_asset(&github, &selected_asset, &output_path)?;
        self.maybe_install(&selected_asset.name, &output_path, destination)?;
        Ok(())
    }

    fn selection_destination_for_install(&self) -> Result<Destination, HandlerError> {
        let cwd = Self::cwd()?;
        match self.install {
            Install::No => Ok(Destination::Directory(cwd)),
            Install::Yes(_) => match self.output.as_ref() {
                Some(output) if output.is_dir() => Ok(Destination::Directory(output.clone())),
                Some(output) => Ok(Destination::File(output.clone())),
                None => Ok(Destination::Directory(cwd)),
            },
        }
    }

    fn asset_may_not_be_archive(&self, asset_name: &str) -> Result<(), HandlerError> {
        if self.install.is_more_than_one() {
            match is_supported_archive(asset_name) {
                Ok(is_archive) => {
                    if !is_archive {
                        return Err(HandlerError::new(format!(
                            "Selected asset {} is not an archive",
                            asset_name
                        )));
                    }
                    Ok(())
                }
                Err(e) => return Err(HandlerError::new(e.to_string())),
            }
        } else {
            Ok(())
        }
    }

    fn select_asset(&self, release: Release) -> Result<Asset, HandlerError> {
        match &self.mode {
            DownloadMode::Interactive => Self::ask_select_asset(release.assets),
            DownloadMode::Selection(untagged) => Self::autoselect_asset(release, untagged),
            DownloadMode::Automatic => {
                let os = std::env::consts::OS;
                let arch = std::env::consts::ARCH;
                find_asset_by_system(os, arch, release.assets).ok_or_else(|| {
                    Self::automatic_download_error(&self.repository, &release.tag, os, arch)
                })
            }
        }
    }

    fn automatic_download_error(
        repository: &Repository,
        release: &Tag,
        os: &str,
        arch: &str,
    ) -> HandlerError {
        let title = urlencoding::encode("Error: automatic download of asset");
        let body = format!(
            "## dra version\n{}\n## Bug report\nRepository: {}\nRelease: {}\nOS: {}\nARCH: {}",
            env!("CARGO_PKG_VERSION"),
            repository,
            release.0,
            os,
            arch
        );
        let body = urlencoding::encode(&body);
        let issue_url = format!(
            "https://github.com/devmatteini/dra/issues/new?title={}&body={}",
            title, body
        );
        HandlerError::new(format!(
            "Cannot find asset that matches your system {} {}\nIf you think this is a bug, please report the issue: {}",
            os, arch, issue_url
        ))
    }

    fn maybe_install(
        &self,
        asset_name: &str,
        path: &Path,
        destination: Destination,
    ) -> Result<(), HandlerError> {
        match &self.install {
            Install::No => Ok(()),
            Install::Yes(executable) => {
                let spinner = Spinner::install_layout();
                spinner.show();

                let output = install(asset_name.to_string(), path, executable, destination)
                    .map_err(|x| HandlerError::new(x.to_string()))?;
                std::fs::remove_file(path).map_err(|x| {
                    HandlerError::new(format!(
                        "Unable to delete temporary file after installation: {}",
                        x
                    ))
                })?;

                let message = format!(
                    "{}\n{}",
                    Color::new("Installation completed!").green(),
                    output
                );
                spinner.finish_with_message(&message);
                Ok(())
            }
        }
    }

    fn cwd() -> Result<PathBuf, HandlerError> {
        std::env::current_dir()
            .map_err(|x| HandlerError::new(format!("Error retrieving current directory: {}", x)))
    }

    fn autoselect_asset(release: Release, untagged: &str) -> Result<Asset, HandlerError> {
        let asset_name = TaggedAsset::tag(&release.tag, untagged);
        release
            .assets
            .into_iter()
            .find(|x| x.name == asset_name)
            .ok_or_else(|| HandlerError::new(format!("No asset found for {}", untagged)))
    }

    fn fetch_release(&self, github: &GithubClient) -> Result<Release, HandlerError> {
        fetch_release_for(github, &self.repository, self.tag.as_ref())
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

    fn download_asset(
        github: &GithubClient,
        selected_asset: &Asset,
        output_path: &Path,
    ) -> Result<(), HandlerError> {
        let progress_bar = ProgressBar::download_layout(&selected_asset.name, output_path);
        progress_bar.show();
        let (mut stream, maybe_content_length) = github
            .download_asset_stream(selected_asset)
            .map_err(Self::download_error)?;
        progress_bar.set_length(maybe_content_length);

        let mut destination = Self::create_file(output_path)?;
        let mut total_bytes = 0;
        let mut buffer = [0; 1024];
        while let Ok(bytes) = stream.read(&mut buffer) {
            if bytes == 0 {
                break;
            }

            destination
                .write(&buffer[..bytes])
                .map_err(|x| Self::write_err(&selected_asset.name, output_path, x))?;

            total_bytes += bytes as u64;
            progress_bar.update_progress(total_bytes);
        }
        progress_bar.finish();
        Ok(())
    }

    pub fn choose_output_path(&self, asset_name: &str) -> PathBuf {
        choose_output_path_from(
            self.output.as_ref(),
            self.install.as_bool(),
            asset_name,
            Path::is_dir,
        )
    }

    fn create_file(path: &Path) -> Result<File, HandlerError> {
        File::create(path).map_err(|e| {
            HandlerError::new(format!("Failed to create file {}: {}", path.display(), e))
        })
    }

    pub fn write_err(asset_name: &str, output_path: &Path, error: std::io::Error) -> HandlerError {
        HandlerError::new(format!(
            "Error saving {} to {}: {}",
            asset_name,
            output_path.display(),
            error
        ))
    }

    fn download_error(e: GithubError) -> HandlerError {
        HandlerError::new(format!("Error downloading asset: {}", e))
    }
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

        assert!(result
            .to_str()
            .expect("Error: no path available")
            .contains("dra-"))
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
