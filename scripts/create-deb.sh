#!/usr/bin/env bash

set -eo pipefail

if ! command -V cargo-deb >/dev/null; then
  echo "cargo-deb command missing" >&2
  exit 1
fi

if [[ -z $1 ]]; then
  echo "Missing target argument"
  exit 1
fi

target="$1"

output="$(cargo deb --target "$target" --no-build --output .)"

# The output is like './dra_x.y.z_amd64.deb'.
# We need to remove './' with bash replace so we can use it
# as the asset name when uploading to github release
echo "${output/.\//}"
