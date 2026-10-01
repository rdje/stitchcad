#!/usr/bin/env bash
# Repository-relative, same-volume archive reader; Python >=3.9 standard library.
set -euo pipefail
ROOT="$(git rev-parse --show-toplevel)"
exec python3 -B "$ROOT/scripts/history_archive.py" --root "${HISTORY_ARCHIVE_ROOT:-$ROOT}" "$@"
