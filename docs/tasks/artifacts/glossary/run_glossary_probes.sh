#!/usr/bin/env bash
# docs/tasks/artifacts/glossary/run_glossary_probes.sh
# G0-CONTRACT.1 — end-to-end probes for the glossary census (run_glossary_census.sh).
#
# WHY A SUITE AND NOT A GREEN RUN: a census that has only ever passed on the tree it was written
# against has never been seen to NOTICE anything. Each RED arm below copies the real inputs into a
# synthetic root, breaks exactly one property, and requires the census to name that property. The real
# tree is never mutated.
#
#   REAL-1          the census passes on this repository as it stands
#   REAL-2          `--emit-index` reproduces the tracked A–Z index byte for byte, so the index cannot
#                   drift from the parts it is derived from
#   DUP-TOKEN       two entries owning one token is refused (T2 — one token, one meaning)
#   DANGLING-REF    a `→` cross-reference to a token nothing owns is refused (T3)
#   DEAD-CLAUSE     a canonical-object cell citing a clause the chapter does not have is refused (R1)
#   SAFETY-LOST     a safety-relevant term that lost its ⚠ is refused (R2)
#   INDEX-DRIFT     a term defined in a part but missing from the A–Z index is refused (I1)
#   UNDECLARED      a machine token used by a chapter and declared nowhere is refused (C1)
#   ARITY           an entry row with the wrong cell count is refused (S1)
#   MISSING         a root with no glossary REFUSES (exit 2) rather than reporting green over nothing
#
# Usage:  bash docs/tasks/artifacts/glossary/run_glossary_probes.sh
# Output: per-arm lines plus `probes: N pass / M fail` (exit nonzero if M > 0).
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
CENSUS="$ROOT/docs/tasks/artifacts/glossary/run_glossary_census.sh"
[ -f "$CENSUS" ] || { echo "probe: REFUSED — $CENSUS not found" >&2; exit 2; }

WORK="${TMPDIR:-$ROOT/target/scratch}/glossary_probes"
rm -rf "$WORK"; mkdir -p "$WORK"; trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { printf '  ✓ %-14s %s\n' "$1" "$2"; pass=$((pass+1)); }
bad() { printf '  ✗ %-14s %s\n' "$1" "$2"; fail=$((fail+1))
        [ -n "${3:-}" ] && printf '%s\n' "$3" | grep -E '^  x |census:|REFUSED' | head -8 | sed 's/^/          /'; }

# a synthetic root carrying the four inputs the census reads: the book, the roadmap, the trees
mkroot() {
  local d="$1"; rm -rf "$d"; mkdir -p "$d"
  ( cd "$ROOT" && tar cf - docs/book/src ROADMAP.md docs/tasks/*.md 2>/dev/null ) | ( cd "$d" && tar xf - )
}
# run the census against a synthetic root; echo its output, return its exit status
run() { local d="$1"; GLOSSARY_ROOT="$d" bash "$CENSUS" 2>&1; }
# portable in-place edit: sed to a temp file, then move (BSD and GNU sed disagree about -i)
sedfile() { local f="$1"; shift; sed "$@" "$f" > "$f.tmp" && mv "$f.tmp" "$f"; }

echo "glossary probes — docs/tasks/artifacts/glossary/run_glossary_census.sh"

# ---------------------------------------------------------------- REAL-1: this tree, as it stands
out="$(run "$ROOT")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q '/ 0 failure(s)' <<<"$out"; then
  ok REAL-1 "$(grep -o 'glossary census: .*' <<<"$out")"
else
  bad REAL-1 "the real glossary does not satisfy its own census (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- REAL-2: the index is derived, not typed
D="$WORK/index"; mkroot "$D"
tracked="$(awk '/^## /{s=0} /^## The terms, A–Z/{s=1; next} s && /^- \[/' "$D/docs/book/src/spec/glossary.md")"
emitted="$(GLOSSARY_ROOT="$D" bash "$CENSUS" --emit-index)"
if [ -n "$emitted" ] && [ "$tracked" = "$emitted" ]; then
  ok REAL-2 "--emit-index reproduces the tracked A–Z index byte for byte ($(grep -c . <<<"$emitted") entries)"
else
  bad REAL-2 "the tracked index and the derived index differ, so one of them is hand-carried" \
      "$(diff <(printf '%s\n' "$tracked") <(printf '%s\n' "$emitted") | head -10)"
fi

# ---------------------------------------------------------------- DUP-TOKEN (T2)
D="$WORK/dup"; mkroot "$D"
sedfile "$D/docs/book/src/spec/glossary/measurements-and-fit.md" 's/| `base_size` |/| `SizeSet` |/'
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'token `SizeSet` is owned twice' <<<"$out"; then
  ok DUP-TOKEN "a second entry claiming \`SizeSet\` is refused and named (exit=$rc)"
else
  bad DUP-TOKEN "two entries owning one token was not refused (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- DANGLING-REF (T3)
D="$WORK/dangling"; mkroot "$D"
sedfile "$D/docs/book/src/spec/glossary/measurements-and-fit.md" 's/→ `Ease`/→ `Nosuchtoken`/'
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'token `Nosuchtoken` is cross-referenced' <<<"$out"; then
  ok DANGLING-REF "a cross-reference to an unowned token is refused and named (exit=$rc)"
else
  bad DANGLING-REF "a dangling \`→\` cross-reference was not refused (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- DEAD-CLAUSE (R1)
D="$WORK/clause"; mkroot "$D"
sedfile "$D/docs/book/src/spec/glossary/measurements-and-fit.md" 's/\[ontology §2\.2\]/[ontology §99.9]/'
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'cites §99.9, which .* has no heading for' <<<"$out"; then
  ok DEAD-CLAUSE "a citation to a clause the chapter does not have is refused (exit=$rc)"
else
  bad DEAD-CLAUSE "an invented clause number was not refused (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- SAFETY-LOST (R2)
D="$WORK/safety"; mkroot "$D"
sedfile "$D/docs/book/src/spec/glossary/recipe-and-pieces.md" 's/^| net line ⚠ |/| net line |/'
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q '`net line` is safety-relevant' <<<"$out"; then
  ok SAFETY-LOST "removing the ⚠ from \`net line\` is refused, with the roadmap clause that requires it (exit=$rc)"
else
  bad SAFETY-LOST "a safety-relevant term lost its mark and the census did not notice (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- INDEX-DRIFT (I1)
D="$WORK/indexdrift"; mkroot "$D"
sedfile "$D/docs/book/src/spec/glossary.md" '/^- \[net line\]/d'
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'missing from the A–Z index: net line' <<<"$out"; then
  ok INDEX-DRIFT "a term dropped from the A–Z index is refused by name (exit=$rc)"
else
  bad INDEX-DRIFT "index drift was not refused (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- UNDECLARED (C1)
D="$WORK/undeclared"; mkroot "$D"
printf '\nA chapter may mention `mystery_token` in prose.\n' >> "$D/docs/book/src/spec/ontology.md"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q '`mystery_token` is used by' <<<"$out"; then
  ok UNDECLARED "a token a chapter uses and nothing declares is refused by name (exit=$rc)"
else
  bad UNDECLARED "an undeclared machine token was not refused (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- ARITY (S1)
D="$WORK/arity"; mkroot "$D"
printf '| extra term | meaning | object | also called | token | a sixth cell |\n' \
  >> "$D/docs/book/src/spec/glossary/recipe-and-pieces.md"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'has 6 cells, expected 5' <<<"$out"; then
  ok ARITY "a six-cell entry row is refused, naming the file and line (exit=$rc)"
else
  bad ARITY "a malformed entry row was not refused (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- MISSING: refuse, do not skip
D="$WORK/empty"; mkdir -p "$D"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 2 ] && grep -q 'REFUSED' <<<"$out"; then
  ok MISSING "a root with no glossary refuses with exit=2 instead of reporting green over nothing"
else
  bad MISSING "an absent glossary did not refuse (exit=$rc)" "$out"
fi

echo
printf 'probes: %d pass / %d fail\n' "$pass" "$fail"
[ "$fail" -eq 0 ] || exit 1
exit 0
