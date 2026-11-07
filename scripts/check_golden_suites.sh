#!/usr/bin/env bash
# The offset corpus and the graded families, by one command, because G3 asks that a mutation in either turn the suite red.
set -u
cd "$(dirname "$0")/.." || exit 2
L="python3 -I -B scripts/local_environment.py --"
failed=0
$L cargo test -p sc-geometry --test offset --test offset_corpus --test metamorphic || failed=1
$L cargo test -p sc-cli --test graded_goldens --test modes --test skirt || failed=1
if [ "$failed" -eq 0 ]; then
  echo "golden suites: OK"
else
  echo "golden suites: FAILED"
fi
exit "$failed"
