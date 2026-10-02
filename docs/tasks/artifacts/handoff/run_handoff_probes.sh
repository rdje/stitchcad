#!/usr/bin/env bash
set -euo pipefail
ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"
python3 -I -B docs/tasks/artifacts/handoff/contract.py
printf 'handoff probes: 1 pass / 0 fail\n'
