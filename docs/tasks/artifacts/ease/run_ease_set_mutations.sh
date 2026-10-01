#!/usr/bin/env bash
# Deliberate guard mutation; never overlap another build/gate or handoff.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"
python3 -I -B docs/tasks/artifacts/ease/ease_set_mutations.py
