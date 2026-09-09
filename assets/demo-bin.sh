#!/usr/bin/env bash
# Print this checkout's release binary.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN="$ROOT/target/release/diple"

test -x "$BIN"
printf '%s\n' "$BIN"
