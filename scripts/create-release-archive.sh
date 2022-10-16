#!/usr/bin/env bash

set -eo pipefail

LINUX="linux"
MACOS="macos"
WINDOWS="windows"

function usage() {
  echo
  echo "Usage: $(basename "$0") <version> <os>"
  echo
  echo "ARGS:"
  echo "    <version> version using format x.y.z"
  echo "    <os>      supported os: linux, macos, windows"
  exit 1
}

function is_supported_os() {
  if [[ "$1" != "$LINUX" && "$1" != "$MACOS" && "$1" != "$WINDOWS" ]]; then
    echo "Error: '$1' is not valid os"
    return 1
  fi
}

if [[ -z $1 || -z $2 ]]; then usage; fi
if ! is_supported_os "$2"; then usage; fi

version="$1"
os="$2"

output="dra-$version"
archive="${output}.tar.gz"

mkdir -p "$output"

cp target/release/dra "$output"
cp README.md "$output"
cp LICENSE "$output"

tar czf "$archive" "$output"
echo "$archive"
