#!/usr/bin/env bash
# SPINE.19.2 — isolated real-input mutations, stdout/reconstruction and bounds.
set -euo pipefail
ROOT="$(git rev-parse --show-toplevel)"
exec python3 -B "$ROOT/docs/tasks/artifacts/history_archive/history_archive_probes.py"
