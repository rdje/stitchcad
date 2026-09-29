#!/usr/bin/env bash
# scripts/check_doctrines.project.sh — THE PROJECT-SPECIFIC DOCTRINE SLOT.
#
# StitchCAD's own mechanizable doctrines live here, never in the universal driver
# (scripts/check_doctrines.sh), which is shared spine code re-synced by
# scripts/update_scaffold.sh. A project check can only ADD refusals; it cannot relax a universal
# one — which is why some inherited gaps are closed by authoring rules recorded in
# docs/decisions/ rather than by code.
#
# It runs LAST in scripts/check_doctrines.sh, i.e. in the pre-commit hook (E3) and in CI (E4).
# Exit 0 = all project doctrines pass; nonzero (with a message on stderr) = a breach that blocks
# the commit. Keep every check here cheap and deterministic; heavy proofs belong in CI jobs.
#
# ── THE PROJECT REGISTRY (human-readable mirror; the driver prints one PROJECT-SPECIFIC row) ──
#
# | ID | Proves | Check |
# | --- | --- | --- |
# | `FRESH-ACCEPTANCE-EVIDENCE` | a staged CODE change adds its OWN ticked, evidence-backed acceptance boxes in this commit — evidence committed for an earlier leaf cannot answer for it (closes defect D15 facet 1, which the universal first-match-per-file scan cannot see) | `scripts/check_fresh_acceptance_evidence.sh` |
# | `LIVE-DOC-SIZE` | every tracked live document is classified in the containment data plane with an owner, lifecycle, health target and inclusive ceiling; every route ends at a classified destination; and the resulting TREE is inside those bounds (`LIVE_DOCUMENT_SIZE_CONTAINMENT.md`) | `scripts/check_live_doc_size.sh` |
#
# Adding a doctrine: write `scripts/check_<name>.sh` (cheap, deterministic, self-describing, with
# `--self-test` arms including a control seen RED), append its row above, and register it in
# PROJECT_DOCTRINES below.
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT" || exit 1

# Each entry: "ID|relative/path/to/check.sh"
PROJECT_DOCTRINES=(
  "FRESH-ACCEPTANCE-EVIDENCE|scripts/check_fresh_acceptance_evidence.sh"
  "LIVE-DOC-SIZE|scripts/check_live_doc_size.sh"
)

fails=0
for entry in "${PROJECT_DOCTRINES[@]}"; do
  id="${entry%%|*}"; path="${entry##*|}"
  if [ ! -x "$path" ] && [ ! -f "$path" ]; then
    printf 'PROJECT %s: missing check %s\n' "$id" "$path" >&2
    fails=$((fails + 1)); continue
  fi
  if out="$(bash "$path" 2>&1)"; then
    printf 'PROJECT %s: ok\n' "$id"
  else
    rc=$?
    printf 'PROJECT %s: BREACH (exit=%s)\n' "$id" "$rc" >&2
    printf '%s\n' "$out" | sed 's/^/  /' >&2
    fails=$((fails + 1))
  fi
done

if [ "$fails" -ne 0 ]; then
  echo "PROJECT-SPECIFIC: $fails doctrine breach(es)" >&2
  exit 1
fi
echo "PROJECT-SPECIFIC: ${#PROJECT_DOCTRINES[@]} project doctrine(s) green"
exit 0
