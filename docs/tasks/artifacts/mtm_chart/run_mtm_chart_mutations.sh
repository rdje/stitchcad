#!/usr/bin/env bash
# Deliberate production mutations; do not overlap any other build/gate or handoff.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"
python3 -I -B docs/tasks/artifacts/mtm_chart/mtm_chart_mutations.py
