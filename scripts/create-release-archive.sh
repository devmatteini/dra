#!/usr/bin/env bash

set -eo pipefail

LINUX="linux"
LINUX_MUSL="linux-musl"
LINUX_ARMV6="linux-armv6"
LINUX_ARM64="linux-arm64"
MACOS="macos"
WINDOWS="windows"

function usage() {
  echo
  echo "Usage: $(basename "$0") <version> <os> <target>"
  echo
  echo "ARGS:"
  echo "    <version>     version using format x.y.z"
  echo "    <os>          supported os: $LINUX, $MACOS, $WINDOWS, $LINUX_ARMV6, $LINUX_ARM64, $LINUX_MUSL"
  echo "    <target>      build target name (https://doc.rust-lang.org/rustc/platform-support.html)"
  exit 1
}

function is_supported_os() {
  if [[ "$1" != "$LINUX" && "$1" != "$MACOS" && "$1" != "$WINDOWS" && "$1" != "$LINUX_ARMV6" && "$1" != "$LINUX_ARM64" && "$1" != "$LINUX_MUSL" ]]; then
    echo "Error: '$1' is not valid os"
    return 1
  fi
}

# Arguments:
#   $1 = os; $2 = file to strip
function strip_executable() {
  if [[ "$1" == "$LINUX" || "$1" == "$LINUX_MUSL" || "$1" == "$MACOS" ]]; then
    strip "$2"
  elif [[ "$1" == "$LINUX_ARMV6" ]]; then
    arm-linux-gnueabihf-strip "$2"
  elif [[ "$1" == "$LINUX_ARM64" ]]; then
    aarch64-linux-gnu-strip "$2"
  fi
}

if [[ -z $1 || -z $2 || -z $3 ]]; then usage; fi
if ! is_supported_os "$2"; then usage; fi

version="$1"
os="$2"
target="$3"

output="dra-$version-$target"
extension=$([[ "$os" == "$WINDOWS" ]] && echo "zip" || echo "tar.gz")
archive="${output}.${extension}"

target_dir="target/$target/release"

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
  7z -y a "$archive" "$output" >/dev/null
else
  tar czf "$archive" "$output"
fi

echo "$archive"
