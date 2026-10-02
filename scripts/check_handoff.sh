#!/usr/bin/env bash
# Project-owned handoff entry point; inherited neutral checker remains unchanged.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
exec python3 -I -B "$ROOT/scripts/handoff_census.py" "$@"
