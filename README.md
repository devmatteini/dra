# DRA - Download Release Assets from GitHub

[![CI](https://github.com/devmatteini/dra/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/devmatteini/dra/actions/workflows/ci.yml)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/devmatteini/dra)
![GitHub downloads all releases](https://img.shields.io/github/downloads/devmatteini/dra/total)

A command line tool to download release assets from GitHub.

[Why should I use dra?](#why-should-i-use-dra) •
[Installation](#installation) •
[Usage](#usage) •
[License](#license)

![dra demo](./assets/demo.gif)

## Why should I use dra?

You can do everything `dra` does with the official [GitHub cli](https://cli.github.com/).

`dra` helps you download release assets more easily:

- no authentication for public repository (you cannot use `gh` without authentication)
- [Built-in generation of pattern](#non-interactive) to select an asset to download
  (with `gh` you need to provide [glob pattern](https://cli.github.com/manual/gh_release_download) that you need to
  create manually).

## Installation

`dra` is available on Linux, macOS and Windows.

It's also available on linux for `armv6` and `arm64`.

### Recommended

Download the prebuilt versions of `dra` for supported platforms from the [latest release](https://github.com/devmatteini/dra/releases/latest).

### Debian-based distributions

Download the latest `.deb` package from the [release page](https://github.com/devmatteini/dra/releases/latest) and
install it via:

```bash
sudo dpkg -i dra_x.y.z_amd64.deb # adapt version number
```

### On Arch Linux

Arch Linux users can install `dra` from the [community repository](https://archlinux.org/packages/community/x86_64/dra/) using [pacman](https://wiki.archlinux.org/title/Pacman):

```bash
pacman -S dra
```

### From source

```bash
git clone https://github.com/devmatteini/dra && cd dra
make release
./target/release/dra --version
```

## Usage

In order to download assets from private repositories (and avoid rate limit issues) export an environment variable `GITHUB_TOKEN=<token>`.

Follow the official guide to create
a [personal access token](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token).
The minimum set of scopes needed is: `Full control of private repositories `.

### Interactive

Select and download an asset from a repository

```
$ dra download devmatteini/dra-tests
```

Select and download an asset to custom path

```
$ dra download --output /tmp/dra-example devmatteini/dra-tests
```

Select and download an asset from a specific release

```
$ dra download --tag 0.1.1 devmatteini/dra-tests
```

Select and download source code archives

```
$ dra download devmatteini/dra-tests
Release tag is 0.1.5
? Pick the asset to download ›
  helloworld_0.1.5.tar.gz
❯ Source code (tar.gz)
  Source code (zip)
```

### Non-Interactive

This mode is useful to be used in automated scripts.

There are two non-interactive mode to download assets: [selection](#selection) and [automatic](#automatic).

#### Selection

First you need to generate an untagged asset name:

```
$ dra untag devmatteini/dra-tests
helloworld_{tag}.tar.gz
```

Copy the output and run:

```shell
# use this command in your scripts
$ dra download --select "helloworld_{tag}.tar.gz" devmatteini/dra-tests
```

#### Automatic

Automatically download an asset based on your operating system and architecture

```shell
# you can use -a or --automatic
dra download -a devmatteini/dra-tests
```

> [!IMPORTANT]
> Since there is no naming convention for release assets,
> be aware that this mode may fail if no asset matches your system based on `dra` rules for recognizing an asset.

### Install assets

Download and install an asset (on both interactive and non-interactive modes)

```
$ dra download --install devmatteini/dra-tests
```

Supported assets that can be installed are:

- Debian packages (`.deb`)
- Tar archive with executable (`.tar.[gz|bz2|xz]`, `.tgz`)
- Zip file with executable (`.zip`)

### Shell completion

Generate shell completion

```
$ dra completion bash > dra-completion
$ source dra-completion
```

See all supported shell with `dra completion -h`

---

For more information on args/flags/options/commands run:

```bash
$ dra --help
$ dra <command> --help
```

## Contributing

Take a look at the [CONTRIBUTING.md](CONTRIBUTING.md) guide.

## License

`dra` is made available under the terms of the MIT License.

See the [LICENSE](LICENSE) file for license details.
