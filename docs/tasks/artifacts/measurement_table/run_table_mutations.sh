#!/usr/bin/env bash
# G1-SLICE.4a.3 — deliberately disable production guards; restore before returning.
# Explicit diagnostic only: it mutates tracked source temporarily, so never run
# alongside Cargo checks, another mutation diagnostic, or a handoff.
set -euo pipefail
ROOT="$(git rev-parse --show-toplevel)"
exec python3 -I -B "$ROOT/docs/tasks/artifacts/measurement_table/table_mutations.py"
