#!/usr/bin/env bash
# docs/tasks/artifacts/g0_exit/run_g0_exit_review_probes.sh
# G0-CONTRACT.15 — probes for the gate exit review.
#
# WHY: a gate review is the one artifact in this repository that certifies everything else, so the instrument
# that derives it must be seen to notice a gap. The failure it exists to prevent is the quiet kind: a clause
# dropped from the review because nobody mentioned it, a deliverable renamed, a check that stopped checking.
# Each RED arm copies the real tree, removes ONE property, and requires the refusal that names it.
#
# ⚠ ROADMAP-GROWS is the arm that proves the closure is against the roadmap and not against a list kept
#   beside it: it adds a clause to a COPY of §11's exit bullet and requires the review to refuse. It edits a
#   copy because the roadmap is the director's to amend.
# ⚠ CHECK-FAILS proves the review reports a failing instrument as a failing clause rather than as a broken
#   review — the difference between "the gate is not met" and "the reviewer crashed".
# ⚠ CONTROL keeps the verdict identical under an unrelated edit, so the RED arms are not vacuous.
#
#   REAL               the review completes: 18 met / 1 not met / 19 clauses, exit 0
#   CLAUSE-DROPPED     a row removed, so a roadmap fragment is dispositioned by nobody → refusal
#   ROW-UNMATCHED      a row keyed on words the roadmap does not contain → refusal
#   ROADMAP-GROWS      the roadmap's exit list gains a clause no row covers → refusal
#   DELIVERABLE-MISSING a row cites a path that does not exist → refusal
#   CHECK-FAILS        an instrument-backed clause whose check fails → NOT MET and "GATE FAILS"
#   NO-BLOCKER         a human-act clause with no named blocker → refusal
#   MALFORMED          a row with the wrong cell count → refusal
#   CONTROL            an unrelated comment edit → the identical verdict
#   MISSING            an absent data plane REFUSES (exit 2) instead of reporting a gate
#
# Usage:  bash docs/tasks/artifacts/g0_exit/run_g0_exit_review_probes.sh
#         TMPDIR=<dir> bash …   (`make probes` pins it to the repository volume)
# Output: per-arm lines plus `probes: N pass / M fail` (exit nonzero if M > 0).
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
TOOL="$ROOT/docs/tasks/artifacts/g0_exit/run_g0_exit_review.sh"
PLANE="$ROOT/docs/tasks/artifacts/g0_exit/g0_exit_clauses.tsv"
for f in "$TOOL" "$PLANE" "$ROOT/ROADMAP.md"; do
  [ -f "$f" ] || { echo "probe: REFUSED — $f not found" >&2; exit 2; }
done
command -v python3 >/dev/null 2>&1 || { echo "probe: REFUSED — python3 not found" >&2; exit 2; }

WORK="${TMPDIR:-$ROOT/target/scratch}/g0_exit_probes"
rm -rf "$WORK"; mkdir -p "$WORK"; trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { printf '  ✓ %-19s %s\n' "$1" "$2"; pass=$((pass+1)); }
bad() { printf '  ✗ %-19s %s\n' "$1" "$2"; fail=$((fail+1))
        [ -n "${3:-}" ] && printf '%s\n' "$3" | grep -E '^  x |G0 EXIT|REFUSED' | head -4 | sed 's/^/                    /'; }

# plane <arm>: a copy of the data plane and of the roadmap, so an arm can mutate either without touching
# the tracked originals. The book and the instruments stay real: the review runs their checks for real.
plane() {
  local d="$WORK/$1"; rm -rf "$d"; mkdir -p "$d"
  cp "$PLANE" "$d/clauses.tsv"
  cp "$ROOT/ROADMAP.md" "$d/ROADMAP.md"
  printf '%s' "$d"
}
run() { G0_EXIT_CLAUSES="$1/clauses.tsv" G0_EXIT_ROADMAP="$1/ROADMAP.md" bash "$TOOL" 2>&1; }

mutate() {
  python3 - "$1" "$2" "$3" <<'PY'
import pathlib, sys
p, old, new = pathlib.Path(sys.argv[1]), sys.argv[2], sys.argv[3]
s = p.read_text(encoding="utf-8")
if old not in s:
    print("mutation REFUSED — pattern absent: %r" % old[:70], file=sys.stderr)
    sys.exit(3)
p.write_text(s.replace(old, new, 1), encoding="utf-8")
PY
}

droprow() {
  python3 - "$1" "$2" <<'PY'
import pathlib, sys
p, needle = pathlib.Path(sys.argv[1]), sys.argv[2]
lines = p.read_text(encoding="utf-8").splitlines(keepends=True)
for i, ln in enumerate(lines):
    if needle in ln:
        del lines[i]
        p.write_text("".join(lines), encoding="utf-8")
        sys.exit(0)
print("mutation REFUSED — no line contains: %r" % needle[:70], file=sys.stderr)
sys.exit(3)
PY
}

echo "g0-exit-review probes — docs/tasks/artifacts/g0_exit/run_g0_exit_review.sh"

# ---------------------------------------------------------------- REAL
P="$(plane real)"; out="$(run "$P")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q '18 met / 1 not met / 19 clauses' <<<"$out" \
   && grep -q 'review status: complete' <<<"$out"; then
  ok REAL "$(grep -o 'G0 EXIT: .*' <<<"$out" | cut -c1-96)"
else
  bad REAL "the real review did not complete as expected (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- CLAUSE-DROPPED
P="$(plane dropped)"
droprow "$P/clauses.tsv" $'G0-02\texit\tglossary of' || bad CLAUSE-DROPPED "the mutation did not apply" ""
out="$(run "$P")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'is dispositioned by no row' <<<"$out"; then
  ok CLAUSE-DROPPED "a roadmap clause nobody reviews is refused, naming the fragment (exit=$rc)"
else
  bad CLAUSE-DROPPED "a dropped clause was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- ROW-UNMATCHED
P="$(plane unmatched)"
mutate "$P/clauses.tsv" $'G0-05\texit\tinstantiation paths\t' $'G0-05\texit\tdual pathways\t' \
  || bad ROW-UNMATCHED "the mutation did not apply" ""
out="$(run "$P")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q "keys on 'dual pathways', which no fragment" <<<"$out"; then
  ok ROW-UNMATCHED "a row keyed on words the roadmap does not contain reviews nothing, and says so (exit=$rc)"
else
  bad ROW-UNMATCHED "an unmatched row was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- ROADMAP-GROWS
P="$(plane grows)"
mutate "$P/ROADMAP.md" 'approval states & release contract (§9) specified;' \
  'approval states & release contract (§9) specified; a fabricated conformance certificate;' \
  || bad ROADMAP-GROWS "the mutation did not apply" ""
out="$(run "$P")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'a fabricated conformance certificate…) is dispositioned by no row' <<<"$out"; then
  ok ROADMAP-GROWS "a clause the roadmap grows reddens the review until somebody dispositions it (exit=$rc)"
else
  bad ROADMAP-GROWS "a new roadmap clause was ignored (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- DELIVERABLE-MISSING
P="$(plane deliverable)"
mutate "$P/clauses.tsv" 'docs/book/src/spec/standards.md	bash docs/tasks/artifacts/standards' \
  'docs/book/src/spec/standards-v2.md	bash docs/tasks/artifacts/standards' \
  || bad DELIVERABLE-MISSING "the mutation did not apply" ""
out="$(run "$P")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q "names the deliverable 'docs/book/src/spec/standards-v2.md', which does not exist" <<<"$out"; then
  ok DELIVERABLE-MISSING "a review citing an artifact that is not there is refused (exit=$rc)"
else
  bad DELIVERABLE-MISSING "a missing deliverable was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- CHECK-FAILS
P="$(plane checkfails)"
mutate "$P/clauses.tsv" $'G0-14\texit\treference skirt\tdocs/book/src/spec/reference-skirt.md\tbash docs/tasks/artifacts/reference_fixture/run_fixture_derivation.sh\t' \
  $'G0-14\texit\treference skirt\tdocs/book/src/spec/reference-skirt.md\tfalse\t' \
  || bad CHECK-FAILS "the mutation did not apply" ""
out="$(run "$P")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q 'G0-14  NOT MET' <<<"$out" && grep -q 'GATE FAILS' <<<"$out"; then
  ok CHECK-FAILS "a failing check is reported as an unmet clause and a failing gate, not as a broken review"
else
  bad CHECK-FAILS "a failing check did not fail the gate (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- NO-BLOCKER
P="$(plane noblocker)"
mutate "$P/clauses.tsv" 'no evaluation seat is being procured and no searcher exists; the director'"'"'s ruling of 2026-09-30 accepts this' '-' \
  || bad NO-BLOCKER "the mutation did not apply" ""
out="$(run "$P")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'is a human act with no named blocker' <<<"$out"; then
  ok NO-BLOCKER "an unmet clause with nobody owing it is refused: that is a gap, not a verdict (exit=$rc)"
else
  bad NO-BLOCKER "a blocker-less human clause was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- MALFORMED
P="$(plane malformed)"
mutate "$P/clauses.tsv" $'G0-11\texit\tstandards identified\tdocs/book/src/spec/standards.md\tbash docs/tasks/artifacts/standards/run_standards_census.sh\tinstrument' \
  $'G0-11\texit\tstandards identified\tdocs/book/src/spec/standards.md\tinstrument' \
  || bad MALFORMED "the mutation did not apply" ""
out="$(run "$P")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'has 7 cells, the schema declares 8' <<<"$out"; then
  ok MALFORMED "a malformed row is refused rather than read generously (exit=$rc)"
else
  bad MALFORMED "a malformed row was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- CONTROL
P="$(plane control)"
printf '# a comment that changes no row\n' >> "$P/clauses.tsv"
out="$(run "$P")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q '18 met / 1 not met / 19 clauses' <<<"$out"; then
  ok CONTROL "an unrelated comment leaves the verdict identical, so the RED arms are not vacuous"
else
  bad CONTROL "a comment changed the verdict (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- MISSING
P="$(plane missing)"; rm -f "$P/clauses.tsv"
out="$(run "$P")"; rc=$?
if [ "$rc" -eq 2 ] && grep -q 'REFUSED' <<<"$out"; then
  ok MISSING "an absent data plane refuses with exit=2 instead of reporting a gate"
else
  bad MISSING "a missing data plane did not refuse (exit=$rc)" "$out"
fi

echo
printf 'probes: %d pass / %d fail\n' "$pass" "$fail"
[ "$fail" -eq 0 ] || exit 1
exit 0
