# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.1] - 2022-11-10

### Added

- Download release source code archives (interactive/non-interactive), by @tranzystorek-io ([#52](https://github.com/devmatteini/dra/issues/52))

### Development

After the cross-platform release (0.4.0), a lot of works was done to improve `dra` test suite to make sure all supported os works.
We are now able to run integration tests on macOS and Windows as well!

### Updated dependencies

- `indicatif` from 0.17.1 to 0.17.2
- `predicates` from 2.1.1 to 2.1.2
- `serde` from 1.0.145 to 1.0.147
- `clap` from 4.0.15 to 4.0.22
- `clap_complete` from 4.0.2 to 4.0.5

## [0.4.0] - 2022-10-19

`dra` is now a cross-platform command line that works on Linux, macOS and Windows 🎉!

### Changed

The `install` feature for tar and zip archives no longer rely on `tar` and `unzip` command lines.
This change was needed to ensure that it behaves the same on all supported platforms ([commits](https://github.com/devmatteini/dra/compare/3d0e189cf000b15d11c97760199012ed15f69ef4...38fef0b936931c33ddd3841b6862847131915cc5)).

### Updated dependencies

- `clap` migration to v4, by @tranzystorek-io
- `ctrlc` from 3.2.2 to 3.2.3
- `uuid` from 1.1.2 to 1.2.1
- `dependabot/fetch-metadata` from 1.3.3 to 1.3.4
- `test-case` from 2.2.1 to 2.2.2
- `serde` from 1.0.144 to 1.0.145
- `clap_complete` from 3.2.4 to 3.2.5
- `indicatif` from 0.17.0 to 0.17.1

## [0.3.8] - 2022-09-17

This is a maintenance release that updates our dependencies.

### Updated dependencies

- `serde` from 1.0.138 to 1.0.144
- `ureq` from 2.4.0 to 2.5.0
- `clap` from 3.2.8 to 3.2.20
- `clap_complete` from 3.2.3 to 3.2.4
- `indicatif` from 0.16.2 to 0.17.0
- `dialoguer` from 0.10.1 to 0.10.2

## [0.3.7] - 2022-07-09

### Development

- Fix clap deprecation warnings for upcoming v4 release ([e7d6997](https://github.com/devmatteini/dra/commit/e7d6997b0ba803aa1e5f5df6ef920bc2ea965135))
- Add github action to auto merge dependabot pull requests of patch updates

### Updated dependencies

- `test-case` from 1.2.3 to 2.1.0
- `uuid` from 0.8.2 to 1.1.2
- `clap_complete` from 3.1.4 to 3.2.3
- `serde` from 1.0.137 to 1.0.138
- `clap` from 3.1.18 to 3.2.8

## [0.3.6] - 2022-06-07

### Security

[CVE-2022-24713](https://github.com/advisories/GHSA-m5pq-gvj9-9vr8) - Updated `regex` crate to 1.5.6 ([#23](https://github.com/devmatteini/dra/pull/23))

### Updated dependencies

- `clap` from 3.1.17 to 3.1.18

## [0.3.5] - 2022-05-12

### Added

- Show download progress, by @orhun ([#17](https://github.com/devmatteini/dra/issues/17))

### Updated dependencies

- `clap_complete` from 3.1.3 to 3.1.4
- `clap` from 3.1.14 to 3.1.17
- `dialoguer` from 0.10.0 to 0.10.1

## [0.3.4] - 2022-05-07

### Added

- More useful error message on rate limit exceeded and unauthorized errors from GitHub API
- `dra --help` and `dra help` now display examples of the most common commands used.

### Fixed

Previously when installing a tar/zip archive without a root directory it would result in an error.

Now this type of structures inside tar/zip archives are supported.

More info on commit [5f73077](https://github.com/devmatteini/dra/commit/5f7307753ea87701a2b8180698a68d86278ee0f8)

### Development

In order to speed up integration tests on CI it was created a custom docker image with all
runtime dependencies already installed ([devmatteini/dra-ubuntu-base](https://hub.docker.com/r/devmatteini/dra-ubuntu-base)).

### Updated dependencies

- `clap` from 3.1.12 to 3.1.14
- `clap_complete` from 3.1.2 to 3.1.3
- `serde` from 1.0.136 to 1.0.137

## [0.3.3] - 2022-04-25

### Changed

- The [release workflow](./.github/workflows/release.yml) is now using [github cli](https://cli.github.com/) to create github release and upload
  assets, since `actions/create-release` and `actions/upload-release-asset` are not maintained anymore.

### Development

- [Dependabot](https://github.com/dependabot) is now used to weekly update cargo crates and github actions
- Integration tests are now faster to run on both host machine and CI (see 1f36ffc12e4bb2da07be3106bc982d76419c7bf0 for more details)

### Updated dependencies

- `test-case` from 1.2.1 to 1.2.3
- `dialoguer` from 0.9.0 to 0.10.0
- `ctrlc` from 3.2.1 to 3.2.2
- `clap` from 3.1.6 to 3.1.12
- `clap_complete` from 3.1.1 to 3.1.2

## [0.3.2] - 2022-03-31

### :warning: Breaking Changes :warning:

The command line interface has changed to `dra <SUBCOMMAND>`.

The `<REPO>` positional argument must be passed after choosing `download` or `untag` subcommand (e.g `dra download <REPO>`)

This change was needed in order to add subcommands/flags that didn't require `<REPO>`, like the newly added `completion`.

### Added

- Generate shell completion with `dra completion <SHELL>`

### Updated dependencies

- clap 3.0.13 -> 3.1.6

## [0.3.1] - 2022-03-19

### Changed

- Improve `dra-download` and `dra-untag` UX by showing a spinner while fetching the release information
- `dra-download` and `dra-untag` now print which release tag is currently used

### Fixed

- GitHub releases with no assets are now handled properly
- If CTRL+C was pressed during the asset selection the cursor would not be restored (in both `dra-download` and `dra-untag` sub commands).

  The issue and solution that we implemented is described here [mitsuhiko/dialoguer/issues/77](https://github.com/mitsuhiko/dialoguer/issues/77).

## [0.3.0] - 2022-03-15

### Added

- Download and install some supported assets (`dra <repo> download --[i]nstall`).

  The supported assets are:

  - Debian packages (`.deb`)
  - Tar archive with executable inside (`.tar.[gz|bz2|xz]`)
  - Zip file with executable inside (`.zip`)

### Internals

Integration tests have been added to test the installation methods for the various supported assets.
For more information on how this tests works, read the [docs](./tests/README.md).

## [0.2.3] - 2022-02-05

This release update some of our dependencies and migrates to clap v3 as args parser.

### Changed

- Migration to clap v3 instead of using structopt (fe132c1).

### Updated dependencies

- serde 1.0.130 -> 1.0.136
- ureq 2.3.1 -> 2.4.0

## [0.2.2] - 2022-01-08

### Added

- Download assets from a specific release, by @orhun (see [issue 3](https://github.com/devmatteini/dra/issues/3)).
  If none is specified, the latest release is used.

  e.g: `dra <repo> download --tag <tag>`

- Download assets from private repositories by exporting `GITHUB_TOKEN` environment variable

## [0.2.1] - 2022-01-06

### Added

- Created `CONTRIBUTING.md` guidelines
- Instructions for installing on Arch Linux from AUR, by
  @orhun ([See pull request](https://github.com/devmatteini/dra/pull/2))
- Instructions for installing on Debian-based distributions.

## [0.2.0] - 2021-12-27

Change application name from `dag` to `dra`.

`dag` is widely used as the acronym of [Directed Acyclic Graph](https://en.wikipedia.org/wiki/Directed_acyclic_graph).

To avoid any confusion I decided to rename this application to `dra`.
The new name is the acronym of `Download Release Asset`.

The repository url has changed as well: https://github.com/devmatteini/dra

## [0.1.0] - 2021-12-21

Initial release of `dag`.

### Added

- Select and download an asset from a repository (interactive)
- Save a downloaded asset to custom file path (`dag <repo> download --output <path>`)
- Generate a pattern to auto select an asset to download (`dag <repo> untag`)
- Auto select and download an asset (`dag <repo> download --select <pattern>`).

  The `--select` value is the pattern generated by `dag-untag` command
