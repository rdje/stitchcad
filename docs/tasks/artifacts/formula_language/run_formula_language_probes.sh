#!/usr/bin/env bash
# docs/tasks/artifacts/formula_language/run_formula_language_probes.sh
# G0-CONTRACT.9 — end-to-end probes for the formula-language census.
#
# WHY: the census is the reason the formula chapter's numbers are earned rather than typed, and an
# instrument that has only ever agreed with the chapter it was written against has never been seen to
# notice anything. This repository has shipped that shape twice: a probe arm that renamed a token's
# declaration and its use together, and a "portability" edit that silently stopped matching headings —
# both green, both testing nothing (TOOLBOX.md records the trap). So each RED arm below copies the real
# book, removes ONE property, and requires a refusal that names it.
#
# ⚠ Every mutation is applied by exact replacement and REFUSES (exit 3) when its pattern is absent,
#   because a mutation that silently did not apply is a green arm that tested nothing.
# ⚠ A RED arm must remove the property, not one instance of it: each arm below mutates a rule's whole
#   subject (a canonical form, a published value, a signature row, a clause number, a declared limit)
#   rather than one occurrence of a name that another row could still satisfy.
# ⚠ The CONTROL arm is the one that proves the others are not vacuous: an unrelated prose edit must
#   leave the verdict green, so a census that reddens on any diff is caught here rather than believed.
#
#   REAL             the book as it stands: 17 bindings, 4 assertions, 13 refusals, 0 mismatches
#   LITERAL          a canonical form in grammar §2 that its own unit table does not produce → L1
#   VALUE            a published binding value in examples §2 that the expression does not compute → L2
#   EXPRESSION       an edited expression, so the row no longer computes what it publishes → L2
#   FIXTURE-DRIFT    the fixture chapter's published value for a shared name → L3, the D27 class
#   ASSERTION        a falsified closure check in examples §3 → L4
#   REFUSAL-TOKEN    a refusal row naming a diagnostic other than the one raised → L5
#   UNDECLARED-CALL  a call to a function grammar §6 does not declare → L6a
#   OPERATOR         an operator character the grammar and display table do not declare → L6b
#   FUNCTION-PARITY  a function row dropped, so the evaluator implements what the chapter does not → L6c
#   CLAUSE           a link citing a clause the target has no heading for → L7
#   PART             a part file the contract's §7 table does not list → L7
#   LIMIT            a structural limit under what the book measures → L8
#   CONTROL          an unrelated prose edit → still green
#   MISSING          an absent book REFUSES (exit 2) instead of reporting green
#
# Usage:  bash docs/tasks/artifacts/formula_language/run_formula_language_probes.sh
#         TMPDIR=<dir> bash …   (`make probes` pins it to the repository volume)
# Output: per-arm lines plus `probes: N pass / M fail` (exit nonzero if M > 0).
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
TOOL="$ROOT/docs/tasks/artifacts/formula_language/run_formula_language_census.sh"
BOOK="$ROOT/docs/book/src"
CONTRACT="$BOOK/spec/formula-language.md"
GRAMMAR="$BOOK/spec/formula-language/grammar.md"
EXAMPLES="$BOOK/spec/formula-language/examples.md"
FIXTURE="$BOOK/spec/reference-skirt.md"
for f in "$TOOL" "$CONTRACT" "$GRAMMAR" "$EXAMPLES" "$FIXTURE"; do
  [ -f "$f" ] || { echo "probe: REFUSED — $f not found" >&2; exit 2; }
done
command -v python3 >/dev/null 2>&1 || { echo "probe: REFUSED — python3 not found" >&2; exit 2; }

WORK="${TMPDIR:-$ROOT/target/scratch}/formula_probes"
rm -rf "$WORK"; mkdir -p "$WORK"; trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { printf '  ✓ %-16s %s\n' "$1" "$2"; pass=$((pass+1)); }
bad() { printf '  ✗ %-16s %s\n' "$1" "$2"; fail=$((fail+1))
        [ -n "${3:-}" ] && printf '%s\n' "$3" | grep -E '^  x |formula-language census:|REFUSED' | head -6 | sed 's/^/             /'; }

# book <arm>: a fresh copy of the whole book per arm, so no arm inherits another's mutation and so the
# census's link resolution, its book-wide operator scan and its fixture reading all stay faithful.
book() {
  local d="$WORK/$1"
  rm -rf "$d"; mkdir -p "$d"
  cp -R "$BOOK" "$d/src"
  printf '%s' "$d/src"
}
run() { FORMULA_BOOK="$1" bash "$TOOL" 2>&1; }

# mutate <file> <old> <new>: exact replacement, refuses when the pattern is absent
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

echo "formula-language probes — docs/tasks/artifacts/formula_language/run_formula_language_census.sh"

# ---------------------------------------------------------------- REAL
out="$(run "$BOOK")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q '/ 0 mismatch(es)' <<<"$out"; then
  ok REAL "$(grep -o 'formula-language census: .*' <<<"$out")"
else
  bad REAL "the real book does not pass its own census (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- LITERAL (L1)
B="$(book literal)"
mutate "$B/spec/formula-language/grammar.md" '| `2.5 cm` | length | `length:25000` |' \
  '| `2.5 cm` | length | `length:2500` |' || bad LITERAL "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'grammar §2: `2.5 cm` canonicalizes to length:25000' <<<"$out"; then
  ok LITERAL "a canonical form the unit table does not produce is refused (exit=$rc)"
else
  bad LITERAL "a wrong canonical form was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- VALUE (L2)
B="$(book value)"
mutate "$B/spec/formula-language/examples.md" '| `garment_waist / 4` | 18.5 cm |' \
  '| `garment_waist / 4` | 18.6 cm |' || bad VALUE "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'computes to 18.5, the chapter publishes 18.6' <<<"$out"; then
  ok VALUE "a published value the expression does not compute is refused (exit=$rc)"
else
  bad VALUE "a wrong published value was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- EXPRESSION (L2)
B="$(book expression)"
mutate "$B/spec/formula-language/examples.md" '`garment_waist + 2 * sa_cb + wb_extension`' \
  '`garment_waist + sa_cb + wb_extension`' || bad EXPRESSION "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q '`waistband_pattern_length`: .* computes to 78.5' <<<"$out"; then
  ok EXPRESSION "an edited expression is evaluated as written and disagreed with (exit=$rc)"
else
  bad EXPRESSION "an expression that no longer computes its row was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- FIXTURE-DRIFT (L3)
B="$(book fixture_drift)"
mutate "$B/spec/reference-skirt.md" '| `quarter_waist` | quarter waist | `garment_waist / 4` | 18.5 cm |' \
  '| `quarter_waist` | quarter waist | `garment_waist / 4` | 18.6 cm |' \
  || bad FIXTURE-DRIFT "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'two chapters, two garments' <<<"$out"; then
  ok FIXTURE-DRIFT "a fixture value this chapter disagrees with is refused as the D27 class (exit=$rc)"
else
  bad FIXTURE-DRIFT "two chapters describing two garments were accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- ASSERTION (L4)
B="$(book assertion)"
mutate "$B/spec/formula-language/examples.md" '== 2 * wb_width`' '== wb_width`' \
  || bad ASSERTION "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q '`waistband_width_closure`' <<<"$out"; then
  ok ASSERTION "a falsified closure check is refused by name (exit=$rc)"
else
  bad ASSERTION "an assertion that does not hold was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- REFUSAL-TOKEN (L5)
B="$(book refusal_token)"
mutate "$B/spec/formula-language/examples.md" '| `waist_girth + 2.5` | `formula_dimension` |' \
  '| `waist_girth + 2.5` | `formula_parse` |' || bad REFUSAL-TOKEN "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'raises formula_dimension, the row names formula_parse' <<<"$out"; then
  ok REFUSAL-TOKEN "a refusal row naming the wrong diagnostic is refused (exit=$rc)"
else
  bad REFUSAL-TOKEN "a wrong diagnostic name was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- UNDECLARED-CALL (L6a)
B="$(book undeclared_call)"
mutate "$B/spec/formula-language/examples.md" '`hypot(hip_to_hem_drop, a_line_flare)`' \
  '`frobnicate(hip_to_hem_drop, a_line_flare)`' || bad UNDECLARED-CALL "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q '`frobnicate(` is no declared function' <<<"$out"; then
  ok UNDECLARED-CALL "a call to a function the grammar does not declare is refused by name (exit=$rc)"
else
  bad UNDECLARED-CALL "an undeclared function was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- OPERATOR (L6b)
B="$(book operator)"
mutate "$B/spec/formula-language/examples.md" '`waist_girth + ease_waist`' \
  '`waist_girth ⊕ ease_waist`' || bad OPERATOR "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q "uses '⊕'" <<<"$out"; then
  ok OPERATOR "an operator no table declares is refused, anywhere in the book (exit=$rc)"
else
  bad OPERATOR "an undeclared operator was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- FUNCTION-PARITY (L6c)
B="$(book parity)"
droprow "$B/spec/formula-language/grammar.md" '| `arc_length` | angle, length | length |' \
  || bad FUNCTION-PARITY "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'implements `arc_length` and the chapter does not declare it' <<<"$out"; then
  ok FUNCTION-PARITY "a declared function the evaluator lacks — or the reverse — is refused (exit=$rc)"
else
  bad FUNCTION-PARITY "a divergence between the tables and the evaluator was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- CLAUSE (L7)
B="$(book clause)"
mutate "$B/spec/formula-language/examples.md" '[the contract §5.3](../formula-language.md)' \
  '[the contract §5.9](../formula-language.md)' || bad CLAUSE "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'cites §5.9, which .* has no heading for' <<<"$out"; then
  ok CLAUSE "a citation of a clause the target does not carry is refused (exit=$rc)"
else
  bad CLAUSE "a dead clause citation was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- PART (L7)
B="$(book part)"
printf '# An unlisted part\n' > "$B/spec/formula-language/notes.md"
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'notes.md exists but the contract.s §7 parts table does not list it' <<<"$out"; then
  ok PART "a part file the contract's parts table does not list is refused (exit=$rc)"
else
  bad PART "an unlisted part was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- LIMIT (L8)
B="$(book limit)"
mutate "$B/spec/formula-language.md" '| `max_expression_nodes` | 256 |' \
  '| `max_expression_nodes` | 4 |' || bad LIMIT "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'under the 20x margin' <<<"$out"; then
  ok LIMIT "a structural limit below what the book measures is refused (exit=$rc)"
else
  bad LIMIT "an unusable limit was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- CONTROL
B="$(book control)"
mutate "$B/spec/formula-language.md" '## 1. The paradigm, and the drafting system named with it' \
  $'## 1. The paradigm, and the drafting system named with it\n\nA sentence that changes no rule.' \
  || bad CONTROL "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q '/ 0 mismatch(es)' <<<"$out"; then
  ok CONTROL "an unrelated prose edit leaves the verdict green, so the RED arms are not vacuous"
else
  bad CONTROL "the census reddened on a change that breaks no rule (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- MISSING
out="$(FORMULA_BOOK="$WORK/absent" bash "$TOOL" 2>&1)"; rc=$?
if [ "$rc" -eq 2 ] && grep -q 'REFUSED' <<<"$out"; then
  ok MISSING "an absent book refuses with exit=2 instead of reporting green"
else
  bad MISSING "a missing book did not refuse (exit=$rc)" "$out"
fi

echo
printf 'probes: %d pass / %d fail\n' "$pass" "$fail"
[ "$fail" -eq 0 ] || exit 1
exit 0
