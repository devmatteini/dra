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

function target_by_os() {
  if [[ "$1" == "$LINUX" ]]; then
    echo "x86_64-unknown-linux-gnu"
  elif [[ "$1" == "$MACOS" ]]; then
    echo "x86_64-apple-darwin"
  elif [[ "$1" == "$WINDOWS" ]]; then
    echo "x86_64-pc-windows-msvc"
  else
    echo "Error: cannot create target because no os was provided"
    exit 1
  fi
}

if [[ -z $1 || -z $2 ]]; then usage; fi
if ! is_supported_os "$2"; then usage; fi

version="$1"
os="$2"

target=$(target_by_os "$os")
output="dra-$version-$target"
extension=$([[ "$os" == "$WINDOWS" ]] && echo "zip" || echo "tar.gz")
archive="${output}.${extension}"

mkdir -p "$output"

if [[ "$os" == "$WINDOWS" ]]; then
  cp target/release/dra.exe "$output"
else
  strip target/release/dra
  cp target/release/dra "$output"
fi
cp README.md "$output"
cp LICENSE "$output"

if [[ "$os" == "$WINDOWS" ]]; then
  7z -y a "$archive" "$output" > /dev/null
else
  tar czf "$archive" "$output"
fi

echo "$archive"
