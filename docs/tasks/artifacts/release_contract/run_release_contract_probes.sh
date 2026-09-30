#!/usr/bin/env bash
# docs/tasks/artifacts/release_contract/run_release_contract_probes.sh
# G0-CONTRACT.12 — end-to-end probes for the release-contract census.
#
# WHY: the census exists so that "this chapter is roadmap §9 and §8.2" is a derived claim rather than a
# sentence, and an instrument that has only ever agreed with the chapter it was written against has never
# been seen to notice anything. Each RED arm copies the real book, removes ONE property, and requires a
# refusal that names it.
#
# ⚠ Every mutation is applied by exact replacement and REFUSES (exit 3) when its pattern is absent.
# ⚠ Each arm's synthetic change removes exactly one property: the arms that drop a row drop only that row,
#   and the arms that rename a cell rename only that cell, so a refusal is attributable.
# ⚠ CONTROL is the arm that proves the others are not vacuous.
#
#   REAL               the book as it stands: 9 manifest fields, 6 states, 8 matrix rows, 0 failures
#   FIELD-GAP          a manifest row dropped → C1 names the roadmap field nobody realises
#   FIELD-INVENTED     a row attributed to a §9 field the roadmap does not name → C1
#   LADDER-ORDER       two acceptance states swapped → C2
#   LADDER-RENAME      one acceptance state renamed → C2
#   CLASS-GAP          an artifact class column renamed → C3
#   TUNING-GAP         a row of the §8.2 tuning table dropped → C3
#   STATE-UNRULED      an ontology state §8 stops dispositioning → C3
#   DISPOSITION-NEW    a matrix cell carrying a word the vocabulary does not declare → C4
#   DISPOSITION-UNUSED a declared disposition no cell carries → C4
#   DIAGNOSTIC         a token named that neither §9 nor the envelope declares → C5
#   LINK               a citation of a clause the target does not carry → C6
#   CONTROL            an unrelated prose edit → still green
#   MISSING            an absent chapter REFUSES (exit 2) instead of reporting green
#
# Usage:  bash docs/tasks/artifacts/release_contract/run_release_contract_probes.sh
#         TMPDIR=<dir> bash …   (`make probes` pins it to the repository volume)
# Output: per-arm lines plus `probes: N pass / M fail` (exit nonzero if M > 0).
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
TOOL="$ROOT/docs/tasks/artifacts/release_contract/run_release_contract_census.sh"
BOOK="$ROOT/docs/book/src"
CHAPTER="$BOOK/spec/release-contract.md"
for f in "$TOOL" "$CHAPTER" "$ROOT/ROADMAP.md"; do
  [ -f "$f" ] || { echo "probe: REFUSED — $f not found" >&2; exit 2; }
done
command -v python3 >/dev/null 2>&1 || { echo "probe: REFUSED — python3 not found" >&2; exit 2; }

WORK="${TMPDIR:-$ROOT/target/scratch}/release_contract_probes"
rm -rf "$WORK"; mkdir -p "$WORK"; trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { printf '  ✓ %-19s %s\n' "$1" "$2"; pass=$((pass+1)); }
bad() { printf '  ✗ %-19s %s\n' "$1" "$2"; fail=$((fail+1))
        [ -n "${3:-}" ] && printf '%s\n' "$3" | grep -E '^  x |release-contract census:|REFUSED' | head -4 | sed 's/^/                    /'; }

book() {
  local d="$WORK/$1"; rm -rf "$d"; mkdir -p "$d"
  cp -R "$BOOK" "$d/src"
  printf '%s' "$d"
}
run() { RELEASE_BOOK="$1/src" bash "$TOOL" 2>&1; }
CH() { printf '%s' "$1/src/spec/release-contract.md"; }

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

mutate_all() {
  python3 - "$1" "$2" "$3" <<'PY'
import pathlib, sys
p, old, new = pathlib.Path(sys.argv[1]), sys.argv[2], sys.argv[3]
s = p.read_text(encoding="utf-8")
if old not in s:
    print("mutation REFUSED — pattern absent: %r" % old[:70], file=sys.stderr)
    sys.exit(3)
p.write_text(s.replace(old, new), encoding="utf-8")
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

echo "release-contract probes — docs/tasks/artifacts/release_contract/run_release_contract_census.sh"

# ---------------------------------------------------------------- REAL
B="$(book real)"; out="$(run "$B")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q '/ 0 failure(s)' <<<"$out"; then
  ok REAL "$(grep -o 'release-contract census: .*' <<<"$out")"
else
  bad REAL "the real book does not pass its own census (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- FIELD-GAP (C1)
B="$(book field_gap)"
droprow "$(CH "$B")" '| `artifact_hashes` |' || bad FIELD-GAP "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q "the roadmap names the manifest field 'artifact hashes'" <<<"$out"; then
  ok FIELD-GAP "a manifest field the roadmap names and the chapter drops is refused (exit=$rc)"
else
  bad FIELD-GAP "a dropped manifest field was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- FIELD-INVENTED (C1)
B="$(book field_invented)"
mutate "$(CH "$B")" '| `design_revision` | design revision |' '| `design_revision` | design intention |' \
  || bad FIELD-INVENTED "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q "attributes 'design intention' to roadmap §9" <<<"$out"; then
  ok FIELD-INVENTED "a field wearing the roadmap's authority without its wording is refused (exit=$rc)"
else
  bad FIELD-INVENTED "an invented manifest field was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- LADDER-ORDER (C2)
B="$(book ladder_order)"
python3 - "$(CH "$B")" <<'PY'
import pathlib, sys
p = pathlib.Path(sys.argv[1]); lines = p.read_text(encoding="utf-8").splitlines(keepends=True)
i = next(k for k, l in enumerate(lines) if l.startswith("| `internally checked` |"))
j = next(k for k, l in enumerate(lines) if l.startswith("| `independently inspected` |"))
lines[i], lines[j] = lines[j], lines[i]
p.write_text("".join(lines), encoding="utf-8")
PY
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'C2 the chapter.s ladder is' <<<"$out"; then
  ok LADDER-ORDER "a rung moved is a different ladder, and the census prints both orders (exit=$rc)"
else
  bad LADDER-ORDER "a reordered ladder was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- LADDER-RENAME (C2)
B="$(book ladder_rename)"
mutate "$(CH "$B")" '| `target-imported` |' '| `target-imported-ok` |' || bad LADDER-RENAME "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'C2 the chapter.s ladder is' <<<"$out"; then
  ok LADDER-RENAME "a renamed state no longer matches the roadmap's chain (exit=$rc)"
else
  bad LADDER-RENAME "a renamed acceptance state was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- CLASS-GAP (C3)
B="$(book class_gap)"
mutate "$(CH "$B")" '| Unknown affects | PDF preview | DXF/PLT draft | Production release |' \
  '| Unknown affects | PDF preview | DXF/PLT draft | Production |' || bad CLASS-GAP "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q "names the artifact class 'Production release'" <<<"$out"; then
  ok CLASS-GAP "an artifact class the roadmap names and the matrix drops is refused (exit=$rc)"
else
  bad CLASS-GAP "a dropped artifact class was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- TUNING-GAP (C3)
B="$(book tuning_gap)"
droprow "$(CH "$B")" '| Notch geometry | notch geometry or type |' || bad TUNING-GAP "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q "example row 'Notch geometry' is not mapped" <<<"$out"; then
  ok TUNING-GAP "a tuning of the roadmap's example that the chapter stops recording is refused (exit=$rc)"
else
  bad TUNING-GAP "an unrecorded tuning was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- STATE-UNRULED (C3)
B="$(book state_unruled)"
mutate "$(CH "$B")" '`known` is `permit`; `preference` is `permit` with its provenance' \
  '`known` is `permit`; a house default is `permit` with its provenance' \
  || bad STATE-UNRULED "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'the ontology declares the state `preference` and §8 never dispositions it' <<<"$out"; then
  ok STATE-UNRULED "an uncertainty state the matrix stops ruling on is refused (exit=$rc)"
else
  bad STATE-UNRULED "an undispositioned state was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- DISPOSITION-NEW (C4)
B="$(book disposition_new)"
mutate "$(CH "$B")" '| a cosmetic label | `badge` | `badge` | `block` |' \
  '| a cosmetic label | `badge` | `allow` | `block` |' || bad DISPOSITION-NEW "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q "a matrix cell carries 'allow'" <<<"$out"; then
  ok DISPOSITION-NEW "a disposition outside the declared vocabulary is refused (exit=$rc)"
else
  bad DISPOSITION-NEW "an undeclared disposition was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- DISPOSITION-UNUSED (C4)
B="$(book disposition_unused)"
# only the matrix cells: replacing every `sidecar` would delete its declaration too, and an arm that
# removes the rule along with the breach reports a green census for the wrong reason
mutate_all "$(CH "$B")" '| `badge` | `sidecar` | `block` |' '| `badge` | `badge` | `block` |' \
  || bad DISPOSITION-UNUSED "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'declares the disposition `sidecar` and no matrix cell carries it' <<<"$out"; then
  ok DISPOSITION-UNUSED "a vocabulary word no cell carries is refused, so the table stays usable (exit=$rc)"
else
  bad DISPOSITION-UNUSED "an unused disposition was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- DIAGNOSTIC (C5)
B="$(book diagnostic)"
mutate "$(CH "$B")" 'is refused: `release_disposition_missing`.' 'is refused: `release_waiver_granted`.' \
  || bad DIAGNOSTIC "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'names `release_waiver_granted`, which neither its §9 nor' <<<"$out"; then
  ok DIAGNOSTIC "a diagnostic no table declares is refused by name (exit=$rc)"
else
  bad DIAGNOSTIC "an undeclared diagnostic was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- LINK (C6)
B="$(book link)"
mutate "$(CH "$B")" '[governance §6](../governance.md)' '[governance §16](../governance.md)' \
  || bad LINK "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'cites §16, which ../governance.md has no heading for' <<<"$out"; then
  ok LINK "a citation of a clause the target does not carry is refused (exit=$rc)"
else
  bad LINK "a dead clause citation was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- CONTROL
B="$(book control)"
mutate "$(CH "$B")" '## 4. Package completeness' $'## 4. Package completeness\n\nA sentence that changes no rule.' \
  || bad CONTROL "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q '/ 0 failure(s)' <<<"$out"; then
  ok CONTROL "an unrelated prose edit leaves the verdict green, so the RED arms are not vacuous"
else
  bad CONTROL "the census reddened on a change that breaks no rule (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- MISSING
B="$(book missing)"; rm -f "$(CH "$B")"
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 2 ] && grep -q 'REFUSED' <<<"$out"; then
  ok MISSING "an absent chapter refuses with exit=2 instead of reporting green"
else
  bad MISSING "a missing chapter did not refuse (exit=$rc)" "$out"
fi

echo
printf 'probes: %d pass / %d fail\n' "$pass" "$fail"
[ "$fail" -eq 0 ] || exit 1
exit 0
