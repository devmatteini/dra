#!/usr/bin/env bash

if ! command -v cross >/dev/null; then
  cargo install cross
fi
