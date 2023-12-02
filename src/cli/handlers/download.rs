use std::cmp;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::cli::get_env;
use crate::cli::handlers::common::fetch_release_for;
use crate::cli::handlers::{HandlerError, HandlerResult};
use crate::cli::progress_bar::ProgressBar;
use crate::cli::select;
use crate::cli::spinner::Spinner;
use crate::github::client::GithubClient;
use crate::github::error::GithubError;
use crate::github::release::{Asset, Release, Tag};
use crate::github::tagged_asset::TaggedAsset;
use crate::github::{Repository, GITHUB_TOKEN};
use crate::installer::cleanup::InstallCleanup;
use crate::{github, installer};

pub struct DownloadHandler {
    repository: Repository,
    select: Option<String>,
    mode: DownloadMode,
    tag: Option<Tag>,
    output: Option<PathBuf>,
    install: bool,
}

enum DownloadMode {
    Interactive,
    Selection(String),
}

impl DownloadMode {
    fn new(select: Option<String>) -> Self {
        if let Some(value) = select {
            Self::Selection(value)
        } else {
            Self::Interactive
        }
    }
}

impl DownloadHandler {
    pub fn new(
        repository: Repository,
        select: Option<String>,
        tag: Option<String>,
        output: Option<PathBuf>,
        install: bool,
    ) -> Self {
        DownloadHandler {
            repository,
            mode: DownloadMode::new(select.clone()),
            select,
            tag: tag.map(Tag),
            output,
            install,
        }
    }

    pub fn run(&self) -> HandlerResult {
        let client = GithubClient::new(get_env(GITHUB_TOKEN));
        let release = self.fetch_release(&client)?;
        let selected_asset = self.autoselect_or_ask_asset(release)?;
        let output_path = self.choose_output_path(&selected_asset.name);
        Self::download_asset(&client, &selected_asset, &output_path)?;
        self.maybe_install(&selected_asset.name, &output_path)?;
        Ok(())
    }

    fn autoselect_or_ask_asset(&self, release: Release) -> Result<Asset, HandlerError> {
        match &self.mode {
            DownloadMode::Interactive => Self::ask_select_asset(release.assets),
            DownloadMode::Selection(untagged) => Self::autoselect_asset(release, untagged),
        }
    }

    fn maybe_install(&self, asset_name: &str, path: &Path) -> Result<(), HandlerError> {
        if self.install {
            let destination_dir = self.output_dir_or_cwd()?;
            return Self::install_asset(String::from(asset_name), path, &destination_dir);
        }
        Ok(())
    }

    fn output_dir_or_cwd(&self) -> Result<PathBuf, HandlerError> {
        self.output
            .as_ref()
            .map(|x| Self::dir_or_error(x))
            .unwrap_or_else(|| {
                std::env::current_dir().map_err(|x| {
                    HandlerError::new(format!("Error retrieving current directory: {}", x))
                })
            })
    }

    fn autoselect_asset(release: Release, untagged: &str) -> Result<Asset, HandlerError> {
        let asset_name = TaggedAsset::tag(&release.tag, untagged);
        release
            .assets
            .into_iter()
            .find(|x| x.name == asset_name)
            .ok_or_else(|| HandlerError::new(format!("No asset found for {}", untagged)))
    }

    fn fetch_release(&self, client: &GithubClient) -> Result<Release, HandlerError> {
        fetch_release_for(client, &self.repository, self.tag.as_ref())
    }

    fn ask_select_asset(assets: Vec<Asset>) -> select::AskSelectAssetResult {
        select::ask_select_asset(
            assets,
            select::Messages {
                select_prompt: "Pick the asset to download",
                quit_select: "No asset selected",
            },
        )
    }

    fn download_asset(
        client: &GithubClient,
        selected_asset: &Asset,
        output_path: &Path,
    ) -> Result<(), HandlerError> {
        let progress_bar = ProgressBar::download(&selected_asset.name, output_path);
        progress_bar.start();
        let (mut stream, maybe_content_length) =
            github::download_asset(client, selected_asset).map_err(Self::download_error)?;
        if let Some(cl) = maybe_content_length {
            progress_bar.set_max_progress(cl);
        } else {
            progress_bar.progress_unknown();
        }
        let mut destination = Self::create_file(output_path)?;
        let mut downloaded = 0;
        let mut buffer = [0; 1024];
        while let Ok(bytes_read) = stream.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }
            destination
                .write(&buffer[..bytes_read])
                .map_err(|x| Self::write_err(&selected_asset.name, output_path, x))?;
            if let Some(cl) = maybe_content_length {
                downloaded = cmp::min(downloaded + bytes_read as u64, cl);
            } else {
                downloaded += bytes_read as u64;
            }
            progress_bar.update_progress(downloaded);
        }
        progress_bar.stop();
        Ok(())
    }

    fn install_asset(
        asset_name: String,
        asset_path: &Path,
        destination_dir: &Path,
    ) -> Result<(), HandlerError> {
        let spinner = Spinner::install();
        spinner.start();
        installer::install(asset_name, asset_path, destination_dir)
            .cleanup(asset_path)
            .map_err(|x| HandlerError::new(x.to_string()))?;
        spinner.stop();
        Ok(())
    }

    pub fn choose_output_path(&self, asset_name: &str) -> PathBuf {
        if self.install {
            return crate::cli::temp_file::temp_file();
        }

        self.output
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(asset_name))
    }

    fn create_file(path: &Path) -> Result<File, HandlerError> {
        File::create(path).map_err(|e| {
            HandlerError::new(format!(
                "Failed to create the file {}: {}",
                path.display(),
                e
            ))
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

    fn dir_or_error(path: &Path) -> Result<PathBuf, HandlerError> {
        if path.is_dir() {
            Ok(PathBuf::from(path))
        } else {
            Err(HandlerError::new(format!(
                "{} is not a directory",
                path.display()
            )))
        }
    }

    fn download_error(e: GithubError) -> HandlerError {
        HandlerError::new(format!("Error downloading asset: {}", e))
    }
}

fn is_same_os(os: &str, asset_name: &str) -> bool {
    if asset_name.contains(os) {
        return true;
    }
    let aliases: Vec<&str> = match os {
        "macos" => vec!["darwin", "apple", "osx"],
        "windows" => vec!["win64"],
        _ => return false,
    };
    aliases.into_iter().any(|alias| asset_name.contains(alias))
}

fn is_same_arch(arch: &str, asset_name: &str) -> bool {
    if asset_name.contains(arch) {
        return true;
    }
    let aliases: Vec<&str> = match arch {
        "x86_64" => vec!["amd64"],
        "aarch64" => vec!["arm64"],
        "arm" => vec!["armv6", "armv7"],
        _ => return false,
    };
    aliases.into_iter().any(|alias| asset_name.contains(alias))
}

fn find_asset_by_os_arch(os: &str, arch: &str, assets: Vec<Asset>) -> Option<Asset> {
    let mut matches: Vec<_> = assets
        .into_iter()
        .filter(|asset| {
            let asset_name = asset.name.to_lowercase();
            is_same_os(os, &asset_name) && is_same_arch(arch, &asset_name)
        })
        .collect();
    matches.sort_by_key(asset_priority);
    matches.into_iter().next()
}

const ARCHIVES: [&str; 5] = [".gz", ".tgz", ".bz2", ".xz", ".zip"];

fn asset_priority(a: &Asset) -> i32 {
    if ARCHIVES.iter().any(|x| a.name.ends_with(x)) {
        if a.name.contains("musl") {
            1
        } else {
            2
        }
    } else {
        3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INSTALL: bool = true;
    const NO_INSTALL: bool = false;

    #[test]
    fn temp_output_when_installing() {
        let output = PathBuf::from("/some/path");
        let handler = handler_for(Some(output), INSTALL);

        let result = handler.choose_output_path("ANY");

        assert!(result
            .to_str()
            .expect("Error: no path available")
            .contains("dra-"))
    }

    #[test]
    fn selected_output() {
        let output = PathBuf::from("/some/path");
        let handler = handler_for(Some(output.clone()), NO_INSTALL);

        let result = handler.choose_output_path("ANY");

        assert_eq!(output, result)
    }

    #[test]
    fn no_output() {
        let handler = handler_for(None, NO_INSTALL);

        let result = handler.choose_output_path("my_asset.deb");

        assert_eq!(PathBuf::from("my_asset.deb"), result)
    }

    fn handler_for(output: Option<PathBuf>, install: bool) -> DownloadHandler {
        DownloadHandler {
            select: None,
            mode: DownloadMode::Interactive,
            output,
            install,
            tag: None,
            repository: Repository {
                owner: "ANY".into(),
                repo: "ANY".into(),
            },
        }
    }
}

#[cfg(test)]
mod find_asset_by_os_arch_tests {
    use super::*;

    #[test]
    fn asset_found() {
        let assets = vec![
            asset("mypackage-arm-unknown-linux-gnueabihf.tar.gz"),
            asset("mypackage-x86_64-apple-darwin.tar.gz"),
            asset("mypackage-x86_64-unknown-linux-musl.tar.gz"),
        ];

        let result = find_asset_by_os_arch("linux", "x86_64", assets);

        assert_eq_asset("mypackage-x86_64-unknown-linux-musl.tar.gz", result)
    }

    #[test]
    fn found_by_os_alias() {
        let assets = vec![
            asset("mypackage-arm-unknown-linux-gnueabihf.tar.gz"),
            asset("mypackage-x86_64-apple-darwin.tar.gz"),
            asset("mypackage-x86_64-unknown-linux-musl.tar.gz"),
        ];

        let result = find_asset_by_os_arch("macos", "x86_64", assets);

        assert_eq_asset("mypackage-x86_64-apple-darwin.tar.gz", result)
    }

    #[test]
    fn found_by_arch_alias() {
        let assets = vec![
            asset("mypackage-x86_64-apple-darwin.tar.gz"),
            asset("mypackage-linux-amd64.tar.gz"),
        ];

        let result = find_asset_by_os_arch("linux", "x86_64", assets);

        assert_eq_asset("mypackage-linux-amd64.tar.gz", result)
    }

    #[test]
    fn no_matching_asset() {
        let assets = vec![
            asset("mypackage-x86_64-apple-darwin.tar.gz"),
            asset("mypackage-x86_64-unknown-linux-musl.tar.gz"),
        ];

        let result = find_asset_by_os_arch("linux", "arm", assets);

        assert!(result.is_none())
    }

    #[test]
    fn find_asset_case_insensitive() {
        let assets = vec![
            asset("mypackage-x86_64-apple-darwin.tar.gz"),
            asset("mypackage-X86_64-unknown-LiNuX-musl.tar.gz"),
        ];

        let result = find_asset_by_os_arch("linux", "x86_64", assets);

        assert_eq_asset("mypackage-X86_64-unknown-LiNuX-musl.tar.gz", result)
    }

    #[test]
    fn order_assets_by_priority() {
        let mut assets = vec![
            asset("mypackage-linux-amd64.deb"),
            asset("mypackage-linux-gnu.zip"),
            asset("mypackage-linux-x86_64.rpm"),
            asset("mypackage-linux-musl.tar.gz"),
            asset("mypackage-rand-linux-file-with-musl"),
        ];

        assets.sort_by_key(asset_priority);

        let actual_names: Vec<_> = assets.into_iter().map(|x| x.name).collect();

        assert_eq!(
            vec![
                "mypackage-linux-musl.tar.gz",
                "mypackage-linux-gnu.zip",
                "mypackage-linux-amd64.deb",
                "mypackage-linux-x86_64.rpm",
                "mypackage-rand-linux-file-with-musl"
            ],
            actual_names
        )
    }

    fn assert_eq_asset(expected_name: &str, actual: Option<Asset>) {
        match actual {
            None => {
                panic!("Asset is None, expected {}", expected_name)
            }
            Some(asset) => assert_eq!(expected_name, asset.name),
        }
    }

    fn asset(name: &str) -> Asset {
        Asset {
            name: name.into(),
            display_name: None,
            download_url: "ANY_DOWNLOAD_URL".into(),
        }
    }
}
