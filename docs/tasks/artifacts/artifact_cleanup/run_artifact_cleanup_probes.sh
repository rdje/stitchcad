#!/usr/bin/env bash
# Safety controls never delete selected real project outputs; fixtures stay on the repository volume.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"
python3 -I -B docs/tasks/artifacts/artifact_cleanup/cleanup_probes.py
