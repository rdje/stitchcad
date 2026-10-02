#!/usr/bin/env bash
# Scoped publication checks and copied-fixture refusal tests; scratch stays on this repo volume.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"
make book
python3 -I -B docs/tasks/artifacts/book_publication/book_publication.py
