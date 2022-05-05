#!/usr/bin/env bash

set -eo pipefail

if ! command -V docker >/dev/null; then
  echo "docker command missing" >&2
  exit 1
fi

if [[ -z "$DOCKER_PASSWORD" ]]; then
  echo "No DOCKER_PASSWORD env var" >&2
  exit 1
fi


make build-base
make tag-base

echo "$DOCKER_PASSWORD" | docker login -u devmatteini --password-stdin
make push-base
