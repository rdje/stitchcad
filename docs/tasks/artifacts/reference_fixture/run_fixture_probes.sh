#!/usr/bin/env bash
# docs/tasks/artifacts/reference_fixture/run_fixture_probes.sh
# G0-CONTRACT.13d — end-to-end probes for the fixture derivation instrument.
#
# WHY: the instrument's whole job is to NOTICE, and an instrument that has only ever agreed with the
# chapter it was written against has never been seen to notice anything. The two defects it exists
# because of were both invisible to reading: D33 was a drafting step that satisfied every declared
# quantity while producing a garment 28.0 cm too small at the waist, and D27 was a piece list and a width
# formula describing two different garments for nine commits. So each RED arm below copies the real
# chapter, removes ONE property, and requires a refusal that names it.
#
# ⚠ Every mutation is applied by exact replacement and REFUSES if its pattern is absent, because a
#   mutation that silently did not apply is a green arm that tested nothing — the trap TOOLBOX.md records
#   from the matrix suite (an arm that renamed a token's declaration and its use together).
# ⚠ A RED arm must remove the property, not one instance of it: BAND-PAIR restores the faced two-piece
#   reading rather than deleting a row, so B1, P1 and P2 all lose their subject at once.
#
#   REAL           the chapter as it stands re-derives with 0 mismatches
#   BAND-PAIR      §6 back to a faced two-piece band → B1 names D27, P1 names the unsewn pieces, P2 the count
#   NO-ACCOUNT     a piece dropped from §8's attachment account → P1 refuses it by name
#   COUNT          §12's published piece count disagrees with §6's list → P2 refuses
#   FORMULA        a formula edited so it no longer computes the published number → F1 refuses both rows
#   UNKNOWN-TOKEN  a formula naming a token no table declares → refused by name, never read as 0
#   CLOSURE        a published closure equality falsified → F2 refuses
#   CONSTANT       a §3 constant changed → every row that depends on it disagrees (F1 recomputes, not echoes)
#   MISSING        an absent chapter REFUSES (exit 2) instead of reporting green
#
# Usage:  bash docs/tasks/artifacts/reference_fixture/run_fixture_probes.sh
# Output: per-arm lines plus `probes: N pass / M fail` (exit nonzero if M > 0).
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
TOOL="$ROOT/docs/tasks/artifacts/reference_fixture/run_fixture_derivation.sh"
CHAPTER="$ROOT/docs/book/src/spec/reference-skirt.md"
for f in "$TOOL" "$CHAPTER"; do
  [ -f "$f" ] || { echo "probe: REFUSED — $f not found" >&2; exit 2; }
done

WORK="${TMPDIR:-$ROOT/target/scratch}/fixture_probes"
rm -rf "$WORK"; mkdir -p "$WORK"; trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { printf '  ✓ %-14s %s\n' "$1" "$2"; pass=$((pass+1)); }
bad() { printf '  ✗ %-14s %s\n' "$1" "$2"; fail=$((fail+1))
        [ -n "${3:-}" ] && printf '%s\n' "$3" | grep -E '^  x |fixture derivation:|REFUSED' | head -8 | sed 's/^/           /'; }

# copy <name>: a fresh chapter per arm, so no arm inherits another's mutation. The name is an argument,
# not a counter — a counter incremented inside `$(copy)` would increment in a subshell and every arm
# would share one file, which is the shape of a probe that passes for the wrong reason.
copy() { local f="$WORK/$1.md"; cp "$CHAPTER" "$f"; printf '%s' "$f"; }

# mutate <file> <old> <new>: exact replacement, refuses (exit 3) when the pattern is absent
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

# droprow <file> <substring>: delete the first line containing it, refuses when no line does
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

run() { FIXTURE_CHAPTER="$1" bash "$TOOL" 2>&1; }

echo "fixture probes — docs/tasks/artifacts/reference_fixture/run_fixture_derivation.sh"

# ---------------------------------------------------------------- REAL
out="$(run "$CHAPTER")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q '/ 0 mismatch(es)' <<<"$out"; then
  ok REAL "$(grep -o 'fixture derivation: .*' <<<"$out")"
else
  bad REAL "the real chapter does not re-derive cleanly (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- BAND-PAIR (B1 + P1 + P2)
F="$(copy band_pair)"
mutate "$F" '| `waistband` | 1 | — | — | shell | 1 |' \
  '| `waistband_outer` | 1 | — | — | shell | 1 |
| `waistband_inner` | 1 | — | — | shell | 1 |' || bad BAND-PAIR "the mutation did not apply" ""
out="$(run "$F")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'which is defect D27' <<<"$out" \
   && grep -q '§12 publishes 5 pieces but §6 lists 6' <<<"$out" \
   && grep -q 'does not account for it' <<<"$out"; then
  ok BAND-PAIR "the faced two-piece reading is refused by B1, P1 and P2 at once (exit=$rc)"
else
  bad BAND-PAIR "a band shape §4 disagrees with was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- NO-ACCOUNT (P1)
F="$(copy no_account)"
droprow "$F" '| `waistband_interfacing` | no span' || bad NO-ACCOUNT "the mutation did not apply" ""
out="$(run "$F")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'piece `waistband_interfacing` but §8' <<<"$out" \
   && grep -q 'neither a span nor a declared non-sewn attachment' <<<"$out"; then
  ok NO-ACCOUNT "a piece nothing sews and nothing accounts for is refused by name (exit=$rc)"
else
  bad NO-ACCOUNT "an unaccounted piece was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- COUNT (P2)
F="$(copy count)"
mutate "$F" 'Package completeness: 5 pieces' 'Package completeness: 6 pieces' \
  || bad COUNT "the mutation did not apply" ""
out="$(run "$F")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q '§12 publishes 6 pieces but §6 lists 5' <<<"$out"; then
  ok COUNT "a published piece count that belongs to another reading is refused (exit=$rc)"
else
  bad COUNT "a wrong piece count was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- FORMULA (F1 + F2)
F="$(copy formula)"
mutate "$F" '`garment_waist + 2 × sa_cb + wb_extension` | 80.0 cm' \
  '`garment_waist + sa_cb + wb_extension` | 80.0 cm' || bad FORMULA "the mutation did not apply" ""
out="$(run "$F")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q '`waistband_pattern_length`: .* computes to 78.5 cm' <<<"$out" \
   && grep -q 'closure `waistband_length_closure` does not close' <<<"$out"; then
  ok FORMULA "an edited formula is evaluated as written and disagreed with (exit=$rc)"
else
  bad FORMULA "a formula that no longer computes its published value was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- UNKNOWN-TOKEN (F1)
F="$(copy unknown_token)"
mutate "$F" '`2 × wb_width + sa_waist + sa_wb_bottom`' '`2 × wb_height + sa_waist + sa_wb_bottom`' \
  || bad UNKNOWN-TOKEN "the mutation did not apply" ""
out="$(run "$F")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'token `wb_height` is declared by no table' <<<"$out"; then
  ok UNKNOWN-TOKEN "a token no table declares is refused by name, not read as zero (exit=$rc)"
else
  bad UNKNOWN-TOKEN "an undeclared token was silently evaluated (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- CLOSURE (F2)
F="$(copy closure)"
mutate "$F" '| 28.0 cm = 28.0 cm |' '| 28.0 cm = 27.0 cm |' || bad CLOSURE "the mutation did not apply" ""
out="$(run "$F")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q '`allocation_balance`: .* the chapter publishes 27.0 cm' <<<"$out"; then
  ok CLOSURE "a falsified closure equality is refused (exit=$rc)"
else
  bad CLOSURE "a closure that does not close was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- CONSTANT (F1 recomputes)
F="$(copy constant)"
mutate "$F" '| `ss_suppress` | 3.0 cm |' '| `ss_suppress` | 4.0 cm |' \
  || bad CONSTANT "the mutation did not apply" ""
out="$(run "$F")"; rc=$?
hits=$(grep -c 'computes to' <<<"$out")
if [ "$rc" -eq 1 ] && [ "$hits" -ge 3 ] && grep -q '`dart_intake`' <<<"$out" \
   && grep -q '`side_seam_total`' <<<"$out"; then
  ok CONSTANT "one changed constant disagrees with $hits published rows, so F1 recomputes rather than echoes"
else
  bad CONSTANT "a changed constant left the published rows unchallenged (exit=$rc, hits=$hits)" "$out"
fi

# ---------------------------------------------------------------- MISSING
out="$(run "$WORK/absent.md")"; rc=$?
if [ "$rc" -eq 2 ] && grep -q 'REFUSED' <<<"$out"; then
  ok MISSING "an absent chapter refuses with exit=2 instead of reporting green"
else
  bad MISSING "a missing chapter did not refuse (exit=$rc)" "$out"
fi

echo
printf 'probes: %d pass / %d fail\n' "$pass" "$fail"
[ "$fail" -eq 0 ] || exit 1
exit 0
