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
    mode: DownloadMode,
    tag: Option<Tag>,
    output: Option<PathBuf>,
    install: bool,
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

impl DownloadHandler {
    pub fn new(
        repository: Repository,
        select: Option<String>,
        automatic: bool,
        tag: Option<String>,
        output: Option<PathBuf>,
        install: bool,
    ) -> Self {
        DownloadHandler {
            repository,
            mode: DownloadMode::new(select.clone(), automatic),
            tag: tag.map(Tag),
            output,
            install,
        }
    }

    pub fn run(&self) -> HandlerResult {
        let client = GithubClient::new(get_env(GITHUB_TOKEN));
        let release = self.fetch_release(&client)?;
        let selected_asset = self.select_asset(release)?;
        let output_path = self.choose_output_path(&selected_asset.name);
        Self::download_asset(&client, &selected_asset, &output_path)?;
        self.maybe_install(&selected_asset.name, &output_path)?;
        Ok(())
    }

    fn select_asset(&self, release: Release) -> Result<Asset, HandlerError> {
        match &self.mode {
            DownloadMode::Interactive => Self::ask_select_asset(release.assets),
            DownloadMode::Selection(untagged) => Self::autoselect_asset(release, untagged),
            DownloadMode::Automatic => {
                let os = std::env::consts::OS;
                let arch = std::env::consts::ARCH;
                find_asset_by_os_arch(os, arch, release.assets).ok_or_else(|| {
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
        choose_output_path_from(self.output.as_ref(), self.install, asset_name, Path::is_dir)
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
        return crate::cli::temp_file::temp_file();
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
        "x86_64" => vec!["amd64", "x64"],
        "aarch64" => vec!["arm64"],
        "arm" => vec!["armv6", "armv7"],
        _ => return false,
    };
    aliases.into_iter().any(|alias| asset_name.contains(alias))
}

fn contains_extension(os: &str, asset_name: &str) -> bool {
    let extensions: Vec<&str> = match os {
        "linux" => vec![".appimage"],
        "macos" => vec![".dmg"],
        "windows" => vec![".exe"],
        _ => return false,
    };
    extensions
        .into_iter()
        .any(|extension| asset_name.ends_with(extension))
}

fn find_asset_by_os_arch(os: &str, arch: &str, assets: Vec<Asset>) -> Option<Asset> {
    let mut matches: Vec<_> = assets
        .into_iter()
        .filter(|asset| {
            let asset_name = asset.name.to_lowercase();
            let same_arch = is_same_arch(arch, &asset_name);
            let is_same_system = is_same_os(os, &asset_name) && same_arch;
            let is_same_arch_and_extension = same_arch && contains_extension(os, &asset_name);
            is_same_system || is_same_arch_and_extension
        })
        .collect();
    matches.sort_by_key(asset_priority);
    matches.into_iter().next()
}

const ARCHIVES: [&str; 5] = [".gz", ".tgz", ".bz2", ".xz", ".zip"];

fn asset_priority(a: &Asset) -> i32 {
    if a.name.contains("musl") {
        1
    } else if ARCHIVES.iter().any(|x| a.name.ends_with(x)) {
        2
    } else {
        3
    }
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
            asset("mypackage-linux-musl"),
        ];

        assets.sort_by_key(asset_priority);

        let actual_names: Vec<_> = assets.into_iter().map(|x| x.name).collect();

        assert_eq!(
            vec![
                "mypackage-linux-musl.tar.gz",
                "mypackage-linux-musl",
                "mypackage-linux-gnu.zip",
                "mypackage-linux-amd64.deb",
                "mypackage-linux-x86_64.rpm",
            ],
            actual_names
        )
    }

    #[test]
    fn found_by_asset_extension_and_arch() {
        let assets = vec![
            asset("mypackage-arm64.AppImage"),
            asset("mypackage-amd64.AppImage"),
        ];

        let result = find_asset_by_os_arch("linux", "x86_64", assets);

        assert_eq_asset("mypackage-amd64.AppImage", result)
    }

    // TODO: this use case could be improved since most of the time when the arch is missing is implicit to be x86_64
    #[test]
    fn not_found_by_asset_extension_without_arch() {
        let assets = vec![
            asset("mypackage-arm64.AppImage"),
            asset("mypackage.AppImage"),
        ];

        let result = find_asset_by_os_arch("linux", "x86_64", assets);

        assert!(result.is_none());
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
