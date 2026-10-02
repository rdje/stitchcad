#!/usr/bin/env bash
# Deliberate production mutations; do not overlap any other build/gate or handoff.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"
python3 -I -B docs/tasks/artifacts/size_chart_collection/size_chart_collection_mutations.py
