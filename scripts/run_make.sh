#!/usr/bin/env bash
# Prepare stores before Make/platform dispatch starts; derive every runtime path from this checkout.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
exec python3 -I -B "$ROOT/scripts/local_environment.py" -- make -C "$ROOT" "$@"
