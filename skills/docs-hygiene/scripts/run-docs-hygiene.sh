#!/bin/sh
set -eu

if command -v docs-hygiene >/dev/null 2>&1; then
  exec docs-hygiene "$@"
fi

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
plugin_root=$(CDPATH= cd -- "$script_dir/../../.." && pwd)

if [ -f "$plugin_root/Cargo.toml" ] && command -v cargo >/dev/null 2>&1; then
  exec cargo run --quiet --manifest-path "$plugin_root/Cargo.toml" -- "$@"
fi

printf '%s\n' 'docs-hygiene is not on PATH and no buildable plugin source was found.' >&2
exit 127
