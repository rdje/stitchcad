#!/usr/bin/env bash
# scripts/gauge.sh — how far the product stands from each gate's exit, read from the roadmap's own clauses.
#
# WHY: the director's ruling of 2026-11-06 asks that work stay on the roadmap. A count of commits or leaves cannot
# show that, so this counts one thing: exit clauses whose real product path passes its check. It only rises when
# a clause's row changes from `unbuilt` to `instrument`, and `scripts/check_roadmap_lock.sh` refuses that change
# unless the check runs and passes.
#
# Usage:  bash scripts/gauge.sh            counts from the clause tables, closing each gate against the roadmap
#         bash scripts/gauge.sh --checks   also runs every met G1 to G6 clause's check
#         (G0's checks run in `make probes`, through the exit review's own probe suite)
# Output: one line per gate, then `gauge: <met> of <total> clauses met through G6`
#         exit 0 when every gate closes and every check run passes.
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT" || exit 1
REVIEW=docs/tasks/artifacts/g0_exit/run_g0_exit_review.sh
MODE="${1:-}"
case "$MODE" in ''|--checks) ;; *) echo "gauge: usage: gauge.sh [--checks]" >&2; exit 2 ;; esac

# A check run against a library built from a copy of the source says nothing about the source (D400).
if [ "$MODE" = --checks ]; then
  bash scripts/check_build_fingerprints.sh || exit 1
fi

status=0
total_met=0
total_all=0
for gate in G0 G1 G2 G3 G4 G5 G6; do
  skip=1
  if [ "$MODE" = --checks ] && [ "$gate" != G0 ]; then skip=0; fi
  out="$(GATE_REVIEW_GATE="$gate" G0_EXIT_SKIP_CHECKS="$skip" bash "$REVIEW" 2>&1)"
  rc=$?
  line="$(printf '%s\n' "$out" | grep -E "^$gate EXIT: " | tail -1)"
  if [ "$rc" -ne 0 ] || [ -z "$line" ]; then
    printf '%s\n' "$out" | grep -E '^  x |REFUSED' | head -5 >&2
    echo "gauge: $gate does not close against the roadmap" >&2
    status=1
    continue
  fi
  if printf '%s' "$line" | grep -q 'GATE FAILS'; then
    printf '%s\n' "$out" | grep -E 'NOT MET.*exit=' >&2
    echo "gauge: a met clause of $gate failed its check" >&2
    status=1
  fi
  met="$(printf '%s' "$line" | sed -E 's/^G[0-9] EXIT: ([0-9]+) met .*/\1/')"
  all="$(printf '%s' "$line" | sed -E 's/^.* ([0-9]+) clauses .*/\1/')"
  echo "$gate: $met of $all clauses met"
  total_met=$((total_met + met))
  total_all=$((total_all + all))
done
echo "gauge: $total_met of $total_all clauses met through G6"
exit "$status"
