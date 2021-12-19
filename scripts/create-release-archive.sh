#!/usr/bin/env bash

set -eo pipefail

function usage() {
  echo "Usage: $(basename "$0") <version>"
  echo
  echo "ARGS:"
  echo "    <version> dag version using format x.y.z"
  exit 1
}

if [[ -z $1 ]]; then usage; fi

version="$1"
output="dag-$version"
archive="${output}.tar.gz"

mkdir -p "$output"

cp target/release/dag "$output"
cp README.md "$output"
cp LICENSE "$output"

tar czf "$archive" "$output"
echo "$archive"
