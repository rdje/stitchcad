#!/usr/bin/env bash
# docs/tasks/artifacts/feature_matrix/run_feature_matrix_probes.sh
# G0-CONTRACT.4 — end-to-end probes for the feature-matrix census.
#
# WHY A SUITE AND NOT A GREEN RUN: a census that has only ever passed on the tree it was written against
# has never been seen to NOTICE anything. This suite learned that the hard way in the slice that wrote
# it: the first "portable" heading regex (`###+`, replacing a `{2,4}` interval) silently stopped matching
# the ontology's `##`-level sections, so the coverage rule quietly required three clauses fewer — and only
# a RED arm that removes a citation could have told the difference between "covered" and "not looking".
# Each arm below copies the real inputs into a synthetic root, breaks exactly one property, and requires
# the census to name it. The real tree is never mutated.
#
#   REAL              the census passes on this repository as it stands
#   ODD-SHAPE         a table row that is neither 5 cells nor 3 is refused (M1)
#   BAD-DISPOSITION   a disposition outside {supported, rejected, deferred} is refused (M1)
#   UNDECLARED-DIAG   a refusal naming a token §10 does not declare is refused (M2)
#   UNUSED-DIAG       a declared diagnostic no row raises is refused (M2)
#   UNCITED-ONTOLOGY  an ontology object clause no row cites is refused (M3)
#   MISSING-NONGOAL   a roadmap §1.3 non-goal with no rejected row is refused (M4)
#   MISSING-ENVELOPE  an envelope garment with no supported row is refused (M5)
#   BAD-GATE          a gate cell naming a gate roadmap §11 does not have is refused (M6)
#   MISSING           a root with no matrix REFUSES (exit 2) rather than reporting green over nothing
#
# Usage:  bash docs/tasks/artifacts/feature_matrix/run_feature_matrix_probes.sh
# Output: per-arm lines plus `probes: N pass / M fail` (exit nonzero if M > 0).
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
CENSUS="$ROOT/docs/tasks/artifacts/feature_matrix/run_feature_matrix_census.sh"
[ -f "$CENSUS" ] || { echo "probe: REFUSED — $CENSUS not found" >&2; exit 2; }

WORK="${TMPDIR:-$ROOT/target/scratch}/feature_matrix_probes"
rm -rf "$WORK"; mkdir -p "$WORK"; trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { printf '  ✓ %-17s %s\n' "$1" "$2"; pass=$((pass+1)); }
bad() { printf '  ✗ %-17s %s\n' "$1" "$2"; fail=$((fail+1))
        [ -n "${3:-}" ] && printf '%s\n' "$3" | grep -E '^  x |census:|REFUSED' | head -6 | sed 's/^/          /'; }

MATRIX_REL="docs/book/src/spec/feature-matrix.md"
mkroot() {
  local d="$1"; rm -rf "$d"; mkdir -p "$d"
  ( cd "$ROOT" && tar cf - docs/book/src ROADMAP.md 2>/dev/null ) | ( cd "$d" && tar xf - )
}
run() { local d="$1"; FEATURE_MATRIX_ROOT="$d" bash "$CENSUS" 2>&1; }
sedfile() { local f="$1"; shift; sed "$@" "$f" > "$f.tmp" && mv "$f.tmp" "$f"; }

echo "feature-matrix probes — docs/tasks/artifacts/feature_matrix/run_feature_matrix_census.sh"

# ---------------------------------------------------------------- REAL
out="$(run "$ROOT")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q '/ 0 failure(s)' <<<"$out"; then
  ok REAL "$(grep -o 'feature-matrix census: .*' <<<"$out")"
else
  bad REAL "the real matrix does not satisfy its own census (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- ODD-SHAPE (M1)
D="$WORK/shape"; mkroot "$D"
printf '| a four cell row | supported | why | G2 |\n' >> "$D/$MATRIX_REL"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'is a 4-cell table row' <<<"$out"; then
  ok ODD-SHAPE "a row with the wrong cell count is refused (exit=$rc)"
else bad ODD-SHAPE "a malformed row was not refused (exit=$rc)" "$out"; fi

# ---------------------------------------------------------------- BAD-DISPOSITION (M1)
D="$WORK/disp"; mkroot "$D"
sedfile "$D/$MATRIX_REL" 's/^| costing | rejected |/| costing | refused |/'
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -qE 'disposition|no rejected row' <<<"$out"; then
  ok BAD-DISPOSITION "a disposition outside the closed vocabulary is refused (exit=$rc)"
else bad BAD-DISPOSITION "an invented disposition was accepted (exit=$rc)" "$out"; fi

# ---------------------------------------------------------------- UNDECLARED-DIAG (M2)
D="$WORK/undecl"; mkroot "$D"
# anchor on the ROW: the same token also appears in the §10 declaration table, and renaming both copies
# renames nothing the census can see (measured — the first cut of this arm passed for that reason)
sedfile "$D/$MATRIX_REL" 's#^| costing | rejected | roadmap §1.3 | G2 | `ngo_costing` |#| costing | rejected | roadmap §1.3 | G2 | `ngo_costing_typo` |#'
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q '`ngo_costing_typo`, which the diagnostic table does not declare' <<<"$out"; then
  ok UNDECLARED-DIAG "a refusal naming an undeclared token is refused by name (exit=$rc)"
else bad UNDECLARED-DIAG "an undeclared diagnostic token was accepted (exit=$rc)" "$out"; fi

# ---------------------------------------------------------------- UNUSED-DIAG (M2)
D="$WORK/unused"; mkroot "$D"
sedfile "$D/$MATRIX_REL" 's/^| `ngo_costing` | costing is requested/| `ngo_costing_renamed` | costing is requested/'
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'declared but no row raises it' <<<"$out"; then
  ok UNUSED-DIAG "a diagnostic nothing raises is refused by name (exit=$rc)"
else bad UNUSED-DIAG "an unreachable diagnostic was accepted (exit=$rc)" "$out"; fi

# ---------------------------------------------------------------- UNCITED-ONTOLOGY (M3)
D="$WORK/uncited"; mkroot "$D"
# every citation of the clause, not one of them: three rows cite ontology §4.4, so de-citing one leaves
# the clause covered and the arm green for the wrong reason (measured — the first cut did exactly that)
sedfile "$D/$MATRIX_REL" 's/ontology §4\.4/§4.4 of the model/g'
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'ontology §4.4 specifies objects but no matrix row cites it' <<<"$out"; then
  ok UNCITED-ONTOLOGY "an ontology object clause that lost its citation is refused (exit=$rc)"
else bad UNCITED-ONTOLOGY "a silently unlisted ontology clause was accepted (exit=$rc)" "$out"; fi

# ---------------------------------------------------------------- MISSING-NONGOAL (M4)
D="$WORK/nongoal"; mkroot "$D"
sedfile "$D/$MATRIX_REL" 's/^| costing | rejected |/| price engineering | rejected |/'
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q "non-goal \`costing\`" <<<"$out"; then
  ok MISSING-NONGOAL "a roadmap non-goal whose row was renamed away is refused by name (exit=$rc)"
else bad MISSING-NONGOAL "a dropped non-goal was accepted (exit=$rc)" "$out"; fi

# ---------------------------------------------------------------- MISSING-ENVELOPE (M5)
D="$WORK/envelope"; mkroot "$D"
sedfile "$D/$MATRIX_REL" 's/^| classic collar | supported |/| a rolled neckline | supported |/'
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q "envelope garment \`classic collar\`" <<<"$out"; then
  ok MISSING-ENVELOPE "an envelope garment with no supported row is refused by name (exit=$rc)"
else bad MISSING-ENVELOPE "a dropped envelope garment was accepted (exit=$rc)" "$out"; fi

# ---------------------------------------------------------------- BAD-GATE (M6)
D="$WORK/gate"; mkroot "$D"
sedfile "$D/$MATRIX_REL" 's/^| tuck and pleat | supported | ontology §4.3 models both; G3'"'"'s exit names "darts, folds" | G3 |/| tuck and pleat | supported | ontology §4.3 models both | soon |/'
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'names no gate, track or recorded gap' <<<"$out"; then
  ok BAD-GATE "a gate cell of prose instead of a gate is refused (exit=$rc)"
else bad BAD-GATE "a gate-less commitment was accepted (exit=$rc)" "$out"; fi

# ---------------------------------------------------------------- MISSING
D="$WORK/empty"; mkdir -p "$D"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 2 ] && grep -q 'REFUSED' <<<"$out"; then
  ok MISSING "a root with no matrix refuses with exit=2 instead of reporting green over nothing"
else bad MISSING "an absent matrix did not refuse (exit=$rc)" "$out"; fi

echo
printf 'probes: %d pass / %d fail\n' "$pass" "$fail"
[ "$fail" -eq 0 ] || exit 1
exit 0
