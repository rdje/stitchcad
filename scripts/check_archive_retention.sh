#!/usr/bin/env bash
# ARCHIVE-RETENTION: unconditional exact retained bytes/membership/navigation and
# independent decoded/resident pressure; immutable committed windows stay retained.
# --self-test runs isolated calibrated refusal fixtures without touching originals.
set -euo pipefail
ROOT="$(git rev-parse --show-toplevel)"
if [ "${1:-}" = --self-test ]; then
  exec bash "$ROOT/docs/tasks/artifacts/history_archive/run_history_archive_probes.sh"
fi
exec bash "$ROOT/scripts/history_archive.sh" verify-retention
