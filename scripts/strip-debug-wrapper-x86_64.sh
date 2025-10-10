#!/usr/bin/env bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
real_linker="${RUSTICK_X86_64_REAL_LINKER:-}"
if [ -z "$real_linker" ]; then
	if command -v x86_64-unknown-linux-gnu-gcc >/dev/null 2>&1; then
		real_linker=x86_64-unknown-linux-gnu-gcc
	elif command -v gcc >/dev/null 2>&1; then
		real_linker=gcc
	elif command -v cc >/dev/null 2>&1; then
		real_linker=cc
	else
		echo "no linker found for x86_64 target" >&2
		exit 1
	fi
fi
exec "${SCRIPT_DIR}/strip-debug.sh" "$real_linker" "$@"
