#!/usr/bin/env bash

set -eo pipefail

LINUX="linux"
LINUX_ARMV6="linux-armv6"
MACOS="macos"
WINDOWS="windows"

function usage() {
  echo
  echo "Usage: $(basename "$0") <version> <os>"
  echo
  echo "ARGS:"
  echo "    <version> version using format x.y.z"
  echo "    <os>      supported os: $LINUX, $MACOS, $WINDOWS, $LINUX_ARMV6"
  exit 1
}

function is_supported_os() {
  if [[ "$1" != "$LINUX" && "$1" != "$MACOS" && "$1" != "$WINDOWS" && "$1" != "$LINUX_ARMV6" ]]; then
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
  elif [[ "$1" == "$LINUX_ARMV6" ]]; then
    echo "arm-unknown-linux-gnueabihf"
  else
    echo "Error: cannot create target because no os was provided"
    exit 1
  fi
}

# Arguments:
#   $1 = os; $2 = file to strip
function strip_executable() {
  if [[ "$1" == "$LINUX" || "$1" == "$MACOS" ]]; then
    strip "$2"
  elif [[ "$1" == "$LINUX_ARMV6" ]]; then
    arm-linux-gnueabihf-strip "$2"
  fi
}

# Arguments:
#   $1 = os
function target_dir() {
  if [[ "$1" == "$LINUX_ARMV6" ]]; then
    echo "target/arm-unknown-linux-gnueabihf/release"
    return
  fi
  echo "target/release"
}

if [[ -z $1 || -z $2 ]]; then usage; fi
if ! is_supported_os "$2"; then usage; fi

version="$1"
os="$2"

target=$(target_by_os "$os")
output="dra-$version-$target"
extension=$([[ "$os" == "$WINDOWS" ]] && echo "zip" || echo "tar.gz")
archive="${output}.${extension}"

target_dir=$(target_dir "$os")

mkdir -p "$output"

if [[ "$os" == "$WINDOWS" ]]; then
  cp "$target_dir"/dra.exe "$output"
else
  strip_executable "$os" "$target_dir"/dra
  cp "$target_dir"/dra "$output"
fi
cp README.md "$output"
cp LICENSE "$output"

if [[ "$os" == "$WINDOWS" ]]; then
  7z -y a "$archive" "$output" > /dev/null
else
  tar czf "$archive" "$output"
fi

echo "$archive"
