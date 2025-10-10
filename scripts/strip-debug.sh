#!/usr/bin/env bash
set -euo pipefail

# Minimal linker wrapper stripper
#   strip-debug.sh <real-linker> [linker-args...]
real_linker=$1; shift
"$real_linker" "$@"
output=""
# Scan all linker args for -o or -o<file>
args=("$@")
while getopts ":o:" opt; do
    case $opt in
        o) output="$OPTARG";;
    esac
done
[ -n "$output" ] || exit 0
[ -f "$output" ] || exit 0

# Only run split/strip for release builds, not debug.
if [[ "$output" != */release/* && "$output" != *"\\release\\"* ]]; then
  exit 0
fi

objcopy=${RUSTICK_OBJCOPY:-objcopy}
strip=${RUSTICK_STRIP:-strip}

out_dir=$(dirname -- "$output")
if [ "$(basename -- "$out_dir")" = "deps" ]; then
  parent_dir=$(dirname -- "$out_dir")
  base=$(basename -- "$output")
  target="$parent_dir/$base"
  dbg_dir="$parent_dir"
  [ -f "$target" ] || exit 0
else
  target="$output"
  dbg_dir="$out_dir"
fi

dbg_file="$dbg_dir/$(basename -- "$target").dbg"

workdir=$(dirname -- "$target")
(
  cd "$workdir"
  "$objcopy" --only-keep-debug "$(basename -- "$target")" "$(basename -- "$dbg_file")"
  "$strip" --strip-debug "$(basename -- "$target")"
  "$objcopy" --add-gnu-debuglink="$(basename -- "$dbg_file")" "$(basename -- "$target")"
)
