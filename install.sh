#!/usr/bin/env bash

set -euo pipefail

readonly GITHUB_REPOSITORY="devmatteini/dra"

info(){
  echo -e "$1" >&2
}

error(){
  echo -e "$1" >&2
  exit 1
}

check_command(){
  if ! command -v "$1" &>/dev/null; then
    return 1
  fi
}

check_dependencies(){
  local os=$1

  if ! check_command curl && ! check_command wget; then
    error "Missing 'curl' and 'wget'"
  fi

  check_command grep || error "Missing 'grep'"
  check_command cut || error "Missing 'cut'"

  if [[ $os == "Windows" ]]; then
    check_command unzip || error "Missing 'unzip'"
  else
    check_command tar || error "Missing 'tar'"
  fi
}

http_get(){
  local url="$1"
  local output_path="$2"

  if command -v curl &>/dev/null; then
    curl --proto =https --tlsv1.2 -sSfL -o "$output_path" "$url"
  else
    wget --https-only --secure-protocol=TLSv1_2 --quiet -O "$output_path" "$url"
  fi
}

load_latest_release(){
  local stdout="-"

  http_get "https://api.github.com/repos/$GITHUB_REPOSITORY/releases/latest" "$stdout" |
    grep tag_name |
    cut -d'"' -f4
}

get_os(){
  local os
  os=$(uname -s)
  case "$os" in
    MINGW* | Win*) echo "Windows" ;;
    *) echo "$os" ;;
  esac
}

get_arch(){
  uname -m
}

get_target(){
  local os=$1
  local arch=$2
  local system="$arch-$os"

  case "$system" in
    arm64-Darwin) echo "aarch64-apple-darwin" ;;
    aarch64-Linux) echo "aarch64-unknown-linux-gnu" ;;
    armv6l-Linux) echo "arm-unknown-linux-gnueabihf" ;;
    armv7l-Linux) echo "arm-unknown-linux-gnueabihf" ;;
    x86_64-Darwin) echo "x86_64-apple-darwin" ;;
    x86_64-Windows) echo "x86_64-pc-windows-msvc" ;;
    x86_64-Linux) echo "x86_64-unknown-linux-musl" ;;
    *) error "Unsupported system: $system" ;;
  esac
}

get_archive_extension(){
  local target=$1

  case "$target" in
    *windows*) echo "zip" ;;
    *) echo "tar.gz" ;;
  esac
}

download_asset(){
  local version=$1
  local asset=$2
  local temp_dir=$3
  local output_path="$temp_dir/$asset"

  http_get "https://github.com/$GITHUB_REPOSITORY/releases/download/$version/$asset" "$output_path"
  echo "$output_path"
}

extract_archive(){
  local asset_path=$1
  local output_dir=$2

  case "$asset_path" in
    *zip) unzip -q -j -d "$output_dir" "$asset_path" ;;
    *tar.gz) tar xf "$asset_path" --strip-components=1 -C "$output_dir" ;;
    *) error "Unknown archive $asset_path" ;;
  esac
}

copy_executable(){
  local asset_dir=$1
  local destination=$2
  local os=$3

  if [[ "$os" == "Windows" ]]; then
    cp "$asset_dir"/dra.exe "$destination"
  else
    cp "$asset_dir"/dra "$destination"
  fi
}

help(){
  cat <<'EOF'
Install latest release of dra from GitHub Releases

USAGE:
    install.sh [options]

FLAGS:
    -h, --help      Display this message

OPTIONS:
    --to <DESTINATION>   Save dra to custom path [default: current working directory]
EOF
}

installation_completed(){
  cat <<'EOF'

Thanks for installing dra!

You can run `dra --help` to get started and see useful examples.

More examples can be found in the documentation:
- https://github.com/devmatteini/dra#usage
- https://github.com/devmatteini/dra#examples
EOF
}

main(){
  local destination="$PWD"

  while [[ $# -gt 0 ]]; do
    case $1 in
      -h|--help)
        help
        exit 0
        ;;
      --to)
        destination="$2"
        shift
        shift
        ;;
      *)
        error "Unknown option $1"
        ;;
    esac
  done

  local os arch
  os=$(get_os)
  arch=$(get_arch)
  check_dependencies "$os"

  local target archive_extension version asset
  target=$(get_target "$os" "$arch")
  archive_extension=$(get_archive_extension "$target")
  version=$(load_latest_release)
  asset="dra-$version-$target.$archive_extension"

  info "OS:           $arch-$os"
  info "Repository:   $GITHUB_REPOSITORY"
  info "Release:      $version"
  info "Asset:        $asset"

  info "\nDownloading $asset"
  local temp_dir asset_path
  temp_dir=$(mktemp -d)
  asset_path=$(download_asset "$version" "$asset" "$temp_dir")

  info "Extracting archive $asset_path"
  extract_archive "$asset_path" "$temp_dir"
  copy_executable "$temp_dir" "$destination" "$os"

  info "dra saved to $destination"
  installation_completed
}

main "$@"
