# DRA - Download Release Assets from GitHub

[![CI](https://github.com/devmatteini/dra/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/devmatteini/dra/actions/workflows/ci.yml)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/devmatteini/dra)

A command line tool to download release assets from GitHub.

[Why should I use dra?](#why-should-i-use-dra) •
[Installation](#installation) •
[Usage](#usage) •
[Contributing](#contributing) •
[License](#license)

![dra demo](./assets/demo.gif)

## Why should I use dra?

You can do everything `dra` does with the official [GitHub cli](https://cli.github.com/).

`dra` helps you download release assets more easily:

- No authentication for public repository (you cannot use `gh` without authentication)
- [Built-in generation of pattern](#non-interactive-download) to select an asset to download
  (with `gh` you need to provide [glob pattern](https://cli.github.com/manual/gh_release_download) that you need to
  create manually).
- [Automatically select and download](#automatic) an asset based on your operating system and architecture

## Installation

`dra` is available on Linux (x86_64, armv6, arm64), macOS (x86_64, arm64) and Windows.

### Prebuilt binaries

Download the prebuilt versions of `dra` for all supported platforms from
the [latest release](https://github.com/devmatteini/dra/releases/latest).

You can use this `bash` script to automatically download the latest release across all supported platforms.
Replace `<DESTINATION>` with the path where you want to place dra (e.g `~/.local/bin`).
If you omit `--to` option, the default value is the current working directory.

```shell
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/devmatteini/dra/refs/heads/main/install.sh | bash -s -- --to <DESTINATION>
```

### Debian-based distributions

Download the latest `.deb` package from the [release page](https://github.com/devmatteini/dra/releases/latest) and
install it via:

```shell
sudo dpkg -i dra_x.y.z_amd64.deb # adapt version number
```

### Arch Linux

`dra` can be installed from the [community repository](https://archlinux.org/packages/extra/x86_64/dra/):

```shell
pacman -S dra
```

### macOS/Linux with Homebrew

`dra` can be installed from [Homebrew](https://formulae.brew.sh/formula/dra#default):

```shell
brew install dra
```

### From source

```shell
git clone https://github.com/devmatteini/dra && cd dra
make release
./target/release/dra --version
```

### Update dra

The method to update `dra` depends on how you initially installed it.

**If you used a package manager** (e.g [Homebrew](#macoslinux-with-homebrew) or [pacman](#arch-linux)), use the
corresponding package manager commands to update `dra`.

**If you downloaded a prebuilt binary** from GitHub Releases, you have two options:

#### Option 1: Use dra to update itself

- Linux (_Note that you can't replace a binary while it's executing_)
  ```shell
  dra download -a -i -o dra-new devmatteini/dra && mv dra-new /path/to/dra
  ```
- macOS
  ```shell
  dra download -a -i -o /path/to/dra devmatteini/dra
  ```
- Windows (_Note that you can't replace a binary while it's executing_)
  ```shell
  dra download -a -i -o dra-new.exe devmatteini/dra && mv dra-new.exe /path/to/dra.exe
  ```

#### Option 2: Use the automated bash script

Follow the installation instructions on how to use the [automated bash script](#prebuilt-binaries)

## Usage

- [Download assets with interactive mode](#interactive-download)
- [Download assets with non-interactive mode](#non-interactive-download)
- [Download options](#download-options)
- [Install assets](#install-assets)
- [Authentication](#authentication)
- [Shell completion](#shell-completion)
- [Examples](#examples)

### Interactive download

Manually select and download an asset from a repository

```shell
dra download devmatteini/dra-tests
```

### Non-Interactive download

This mode is useful to be used in automated scripts.

There are two modes to download assets: [automatic](#automatic) and [selection](#selection).

#### Automatic

Automatically select and download an asset based on your operating system and architecture

```shell
# you can use -a or --automatic
dra download -a devmatteini/dra-tests
```

> [!IMPORTANT]
> Since there is no naming convention for release assets,
> be aware that this mode may fail if no asset matches your system based on `dra` rules for recognizing an asset.

#### Selection

First, you need to generate an untagged asset name:

```shell
dra untag devmatteini/dra-tests
# output: helloworld_{tag}.tar.gz
```

Copy the output and run:

```shell
# use this command in your scripts
dra download --select "helloworld_{tag}.tar.gz" devmatteini/dra-tests
```

### Download options

All `dra-download` options works with both interactive and non-interactive modes.

Select and download an asset to custom path

```shell
dra download --output /tmp/dra-example devmatteini/dra-tests

# or save to custom directory path
dra download --output ~/Downloads devmatteini/dra-tests
```

Select and download an asset from a specific release

```shell
dra download --tag 0.1.1 devmatteini/dra-tests
```

Select and download source code archives

```shell
dra download devmatteini/dra-tests
Release tag is 0.1.5
? Pick the asset to download ›
  helloworld_0.1.5.tar.gz
❯ Source code (tar.gz)
  Source code (zip)
```

### Install assets

Download and install an asset (on both interactive and non-interactive modes)

```shell
dra download --install devmatteini/dra-tests
```

Supported assets that can be installed are:

- Debian packages (requires elevated privileges)
- Tar archives with executable(s)
- Zip files with executable(s)
- 7-Zip files with executable(s) (requires `7z` cli to be installed and in your `PATH`)
- Compressed executable files
- Executable files
- AppImage files

You can use `-I/--install-file <INSTALL_FILE>` option when a tar/zip archive contains many executables or
when `dra` can't automatically detect which one to install:

```shell
dra download -s helloworld-many-executables-unix.tar.gz -I helloworld-v2 devmatteini/dra-tests
```

You can also specify this option multiple times to install multiples executables

```shell
dra download -s helloworld-many-executables-unix.tar.gz -I helloworld-v2 -I random-script devmatteini/dra-tests
```

### Authentication

In order to download assets from private repositories and avoid rate limit
issues ([60 requests per hour](https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api?apiVersion=2022-11-28#primary-rate-limit-for-unauthenticated-users)
is the default for unauthenticated users), `dra` must make authenticated requests to GitHub.

You can create
a [personal access token](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens)
and then export of the following environment variables:

1. `DRA_GITHUB_TOKEN`
2. `GITHUB_TOKEN`
3. `GH_TOKEN`

If none of the above environment variables are set,
the [GitHub cli token](https://cli.github.com/manual/gh_auth_token) (if available) will be used as default value.
You need to install [GitHub cli](https://cli.github.com/) and then run `gh auth login`.

#### Disable authentication

If you would like to disable GitHub authentication, you can export the environment variable
`DRA_DISABLE_GITHUB_AUTHENTICATION=true`

### Shell completion

Generate shell completion

```shell
dra completion bash > dra-completion
source dra-completion
```

See all supported shell with `dra completion -h`

### Examples

Install an executable from a tar archive

```shell
dra download -s helloworld.tar.gz -i devmatteini/dra-tests
./helloworld
```

Install and move the executable to a custom directory

```shell
dra download -a -i -o ~/.local/bin/ devmatteini/dra-tests
~/.local/bin/helloworld
```

Install an executable file

```shell
dra download -s helloworld-unix -i devmatteini/dra-tests
./helloworld-unix
```

Install an executable from a compressed file

```shell
dra download -s helloworld-compressed-unix.bz2 -i devmatteini/dra-tests
./helloworld-compressed-unix
```

Install and rename the executable (useful when downloading an executable or compressed file)

```shell
dra download -s helloworld-unix -i -o helloworld devmatteini/dra-tests
./helloworld
```

Install a specific executable when many are available

```shell
dra download -s helloworld-many-executables-unix.tar.gz -I helloworld-v2 devmatteini/dra-tests
./helloworld-v2
```

Install multiple executables from a tar/zip archive

```shell
dra download -s helloworld-many-executables-unix.tar.gz -I helloworld-v2 -I random-script devmatteini/dra-tests
./helloworld-v2
./random-script
```

---

For more information on args/flags/options/commands run:

```shell
dra --help
dra <command> --help
```

## Contributing

Take a look at the [CONTRIBUTING.md](CONTRIBUTING.md) guidelines.

### Found a Bug?

If you find a bug in the source code, you can help us
by [submitting an issue](https://github.com/devmatteini/dra/issues/new) to our GitHub Repository. Even
better, you can submit a Pull Request with a fix.

### Missing a Feature?

You can request a new feature
by [submitting a discussion](https://github.com/devmatteini/dra/discussions/new/choose) to
our GitHub Repository.
If you would like to implement a new feature, please consider the size of the change and reach out to
better coordinate our efforts and prevent duplication of work.

## License

`dra` is made available under the terms of the MIT License.

See the [LICENSE](LICENSE) file for license details.
