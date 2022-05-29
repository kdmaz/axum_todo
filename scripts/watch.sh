#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v cargo)" ]; then
  echo >&2 "Error: `cargo` is not installed."
  exit 1
fi

if ! [ -x "$(command -v cargo-watch)" ]; then
  echo >&2 "Error: `cargo-watch` is not installed."
  exit 1
fi

if ! [ -x "$(command -v bunyan)" ]; then
  echo >&2 "Error: `bunyan` is not installed."
  exit 1
fi

cargo watch -x fmt -x check -x clippy -x "run | bunyan"