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
  (with `gh` you need to provide [glob pattern](https://cli.github.com/manual/gh_release_download) that you need to create manually).

## Installation

### Recommended

Download from the [latest release](https://github.com/devmatteini/dra/releases/latest).

### From source

```bash
git clone https://github.com/devmatteini/dra && cd dra
make release
./target/release/dra --version
```

### From AUR

Arch Linux users can install [dra](https://aur.archlinux.org/packages/?O=0&SeB=nd&K=download+assets+from+GitHub+release&outdated=&SB=n&SO=a&PP=50&do_Search=Go) from the AUR using an [AUR helper](https://wiki.archlinux.org/index.php/AUR_helpers). For example:

```bash
paru -S dra
```

## Usage

### Interactive

Select and download an asset from a repository

```
$ dra devmatteini/dra-example download
```

Select and download an asset to custom path

```
$ dra devmatteini/dra-example download --output /tmp/dra-example
```

### Non-Interactive

This mode is useful to be used in automated scripts.

First you need to generate an untagged asset name:

```
$ dra devmatteini/dra-example untag
dra-example-{tag}-amd64
```

Copy the output and run:

```
$ dra devmatteini/dra-example download --select "dra-example-{tag}-amd64"
```

This last command can be used in automated scripts without human interaction.

---

For more information on args/flags/options/commands run:

```bash
$ dra --help
$ dra <command> --help
```

## License

`dra` is made available under the terms of the MIT License.

See the [LICENSE](LICENSE) file for license details.