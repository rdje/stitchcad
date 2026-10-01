#!/usr/bin/env bash
# Deliberately mutates production guards sequentially; never overlap another build or gate.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"
python3 -I -B docs/tasks/artifacts/ease/ease_mutations.py
