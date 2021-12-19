# DAG - Download Asset from GitHub releases

Download an asset from the latest GitHub release.

## Table of contents

- [Installation](#installation)
- [Usage](#usage)
- [License](#license)

## Installation

### Recommended

Download from the [latest release](https://github.com/devmatteini/dag/releases/latest).

### From source

```bash
git clone https://github.com/devmatteini/dag && cd dag
make release
./target/release/dag --version
```

## Usage

### Interactive

Select and download an asset from a repository

```
$ dag devmatteini/dag-example download
```

Select and download an asset to custom path

```
$ dag devmatteini/dag-example download --output /tmp/dag-example
```

### Non-Interactive

This mode is useful to be used in automated scripts.

First you need to generate an untagged asset name:

```
$ dag devmatteini/dag-example untag
dag-example-{tag}-amd64
```

Copy the output and run:

```
$ dag devmatteini/dag-example download --select "dag-example-{tag}-amd64"
```

This last command can be used in automated scripts without human interaction.

---

For more information on args/flags/options/commands run:

```bash
$ dag --help
$ dag <command> --help
```

## License

`dag` is made available under the terms of the MIT License.

See the [LICENSE](LICENSE) file for license details.