# DRA - Download Release Assets from GitHub

[![CI](https://github.com/devmatteini/dra/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/devmatteini/dra/actions/workflows/ci.yml)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/devmatteini/dra)

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

### Recommended

Download from the [latest release](https://github.com/devmatteini/dra/releases/latest).

### Debian-based distributions

Download the latest `.deb` package from the [release page](https://github.com/devmatteini/dra/releases/latest) and
install it via:

```bash
sudo dpkg -i dra_x.y.z_amd64.deb # adapt version number 
```

### From AUR

Arch Linux users can
install [dra](https://aur.archlinux.org/packages/?O=0&SeB=nd&K=download+assets+from+GitHub+release&outdated=&SB=n&SO=a&PP=50&do_Search=Go)
from the AUR using an [AUR helper](https://wiki.archlinux.org/index.php/AUR_helpers). For example:

```bash
paru -S dra
```

### From source

```bash
git clone https://github.com/devmatteini/dra && cd dra
make release
./target/release/dra --version
```

## Usage

In order to download assets from private repositories export an environment variable `GITHUB_TOKEN=<token>`.

Follow the official guide to create
a [personal access token](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token).
The minimum set of scopes needed is: `Full control of private repositories `.

### Interactive

Select and download an asset from a repository

```
$ dra devmatteini/dra-tests download
```

Select and download an asset to custom path

```
$ dra devmatteini/dra-tests download --output /tmp/dra-example
```

Select and download an asset from a specific release

```
$ dra devmatteini/dra-tests download --tag 0.1.1
```

### Install assets

Download and install an asset (on both interactive and non-interactive modes)

```
$ dra devmatteini/dra-tests download --install
```

Supported assets that can be installed are:

- Debian packages (`.deb`)
- Tar archive with executable (`.tar.[gz|bz2|xz]`)
- Zip file with executable (`.zip`)

### Non-Interactive

This mode is useful to be used in automated scripts.

First you need to generate an untagged asset name:

```
$ dra devmatteini/dra-tests untag
helloworld_{tag}.tar.gz
```

Copy the output and run:

```
$ dra devmatteini/dra-tests download --select "helloworld_{tag}.tar.gz"
```

This last command can be used in automated scripts without human interaction.

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
