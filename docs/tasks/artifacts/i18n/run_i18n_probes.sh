#!/usr/bin/env bash
# docs/tasks/artifacts/i18n/run_i18n_probes.sh
# G0-CONTRACT.16 — end-to-end probes for the i18n census.
#
# WHY: the census exists so that "every message id this product can emit is inventoried" is derived rather
# than asserted, and an instrument that has only ever agreed with the chapter it was written against has
# never been seen to notice anything. Each RED arm copies the real tree, removes ONE property, and requires
# a refusal that names it.
#
# ⚠ CODE-GROWS is the arm that proves the census reads the crate and not a list kept beside it: it adds a
#   variant to a COPY of `crates/sc-units/src/error.rs` and requires the inventory to be refused as
#   incomplete. It mutates a copy because the tracked source is product code and a probe has no business
#   editing it.
# ⚠ Every mutation is applied by exact replacement and REFUSES (exit 3) when its pattern is absent.
# ⚠ CONTROL is the arm that proves the others are not vacuous.
#
#   REAL             the tree as it stands: 8 families, 64 message ids, 0 failures
#   FAMILY-GAP       a family row dropped, so its ids are inventoried by nobody → M1
#   COUNT-DRIFT      a family's published count no longer matches its sources → M1
#   FAMILY-INVENTED  a family nobody's tokens belong to → M1
#   CODE-GROWS       the crate gains an error variant the inventory does not cover → M1
#   TIER             a safety-carrying family tiered `standard` → M3
#   EXEMPTION        a lint exemption with no reason → M4
#   DIAGNOSTIC       an `i18n_*` token §11 does not declare → M5
#   LINK             a citation of a clause the target does not carry → M6
#   CONTROL          an unrelated prose edit → still green
#   MISSING          an absent chapter REFUSES (exit 2) instead of reporting green
#
# Usage:  bash docs/tasks/artifacts/i18n/run_i18n_probes.sh
#         TMPDIR=<dir> bash …   (`make probes` pins it to the repository volume)
# Output: per-arm lines plus `probes: N pass / M fail` (exit nonzero if M > 0).
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
TOOL="$ROOT/docs/tasks/artifacts/i18n/run_i18n_census.sh"
BOOK="$ROOT/docs/book/src"
CHAPTER="$BOOK/spec/i18n-architecture.md"
UNITS="$ROOT/crates/sc-units/src/error.rs"
for f in "$TOOL" "$CHAPTER" "$UNITS"; do
  [ -f "$f" ] || { echo "probe: REFUSED — $f not found" >&2; exit 2; }
done
command -v python3 >/dev/null 2>&1 || { echo "probe: REFUSED — python3 not found" >&2; exit 2; }

WORK="${TMPDIR:-$ROOT/target/scratch}/i18n_probes"
rm -rf "$WORK"; mkdir -p "$WORK"; trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { printf '  ✓ %-17s %s\n' "$1" "$2"; pass=$((pass+1)); }
bad() { printf '  ✗ %-17s %s\n' "$1" "$2"; fail=$((fail+1))
        [ -n "${3:-}" ] && printf '%s\n' "$3" | grep -E '^  x |i18n census:|REFUSED' | head -4 | sed 's/^/                  /'; }

# tree <arm>: a copy of the book AND of the crate's error source, so the census reads a whole consistent
# tree and one arm's mutation cannot reach another's.
tree() {
  local d="$WORK/$1"; rm -rf "$d"; mkdir -p "$d/crates"
  cp -R "$BOOK" "$d/src"
  cp "$UNITS" "$d/crates/error.rs"
  printf '%s' "$d"
}
run() { I18N_BOOK="$1/src" I18N_UNITS_ERROR="$1/crates/error.rs" bash "$TOOL" 2>&1; }
CH() { printf '%s' "$1/src/spec/i18n-architecture.md"; }

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

echo "i18n probes — docs/tasks/artifacts/i18n/run_i18n_census.sh"

# ---------------------------------------------------------------- REAL
T="$(tree real)"; out="$(run "$T")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q '/ 0 failure(s)' <<<"$out"; then
  ok REAL "$(grep -o 'i18n census: .*' <<<"$out")"
else
  bad REAL "the real tree does not pass its own census (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- FAMILY-GAP (M1)
T="$(tree family_gap)"
droprow "$(CH "$T")" '| `geom_*` | 1 |' || bad FAMILY-GAP "the mutation did not apply" ""
out="$(run "$T")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q '`geom_offset_budget` is declared by .* and no family in §10 covers it' <<<"$out"; then
  ok FAMILY-GAP "an id whose family the inventory drops is refused by name (exit=$rc)"
else
  bad FAMILY-GAP "an uninventoried id was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- COUNT-DRIFT (M1)
T="$(tree count_drift)"
mutate "$(CH "$T")" '| `formula_*` | 12 |' '| `formula_*` | 11 |' || bad COUNT-DRIFT "the mutation did not apply" ""
out="$(run "$T")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q "the family \`formula_\` publishes 11 id(s) and the sources declare 12" <<<"$out"; then
  ok COUNT-DRIFT "a count kept by hand that drifts from its source is refused (exit=$rc)"
else
  bad COUNT-DRIFT "a wrong count was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- FAMILY-INVENTED (M1)
T="$(tree family_invented)"
mutate "$(CH "$T")" '| `i18n_*` | 5 | §11 below | `standard` |' \
  '| `i18n_*` | 5 | §11 below | `standard` |
| `mystery_*` | 3 | nowhere | `standard` |' || bad FAMILY-INVENTED "the mutation did not apply" ""
out="$(run "$T")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'declares the family `mystery_` and no source table declares a token' <<<"$out"; then
  ok FAMILY-INVENTED "a family nobody's tokens belong to is refused (exit=$rc)"
else
  bad FAMILY-INVENTED "an invented family was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- CODE-GROWS (M1)
T="$(tree code_grows)"
mutate "$T/crates/error.rs" '    EmptyDerivation,' \
  '    EmptyDerivation,
    LocaleDataMissing,' || bad CODE-GROWS "the mutation did not apply" ""
out="$(run "$T")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'the family `unit_` publishes 5 id(s) and the sources declare 6' <<<"$out"; then
  ok CODE-GROWS "a variant added to the crate reddens the inventory's count, so the census reads the code (exit=$rc)"
else
  bad CODE-GROWS "a new error variant was accepted by a stale inventory (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- TIER (M3)
T="$(tree tier)"
mutate "$(CH "$T")" '| `env_*` | 21 | [envelope §10](feature-matrix.md) | `safety` where the row is a notch' \
  '| `env_*` | 21 | [envelope §10](feature-matrix.md) | `standard` where the row is a notch' \
  || bad TIER "the mutation did not apply" ""
out="$(run "$T")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'carries the notch and cut/sew refusals roadmap §7.6 names' <<<"$out"; then
  ok TIER "a safety-carrying family tiered as ordinary is refused (exit=$rc)"
else
  bad TIER "a mistiered safety family was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- EXEMPTION (M4)
T="$(tree exemption)"
mutate "$(CH "$T")" \
  '| a machine token, a diagnostic code, a kind or state name | never rendered raw to a user (§1); the glossary owns it |' \
  '| a machine token, a diagnostic code, a kind or state name | — |' \
  || bad EXEMPTION "the mutation did not apply" ""
out="$(run "$T")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'lint exemption .* carries no reason' <<<"$out"; then
  ok EXEMPTION "a lint exemption with no reason is refused, because that is how exemptions grow (exit=$rc)"
else
  bad EXEMPTION "an unjustified exemption was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- DIAGNOSTIC (M5)
T="$(tree diagnostic)"
mutate "$(CH "$T")" 'is `i18n_geometry_mirrored`, which is' 'is `i18n_geometry_flipped`, which is' \
  || bad DIAGNOSTIC "the mutation did not apply" ""
out="$(run "$T")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'names `i18n_geometry_flipped`, which its §11 does not declare' <<<"$out"; then
  ok DIAGNOSTIC "an undeclared diagnostic of this layer is refused by name (exit=$rc)"
else
  bad DIAGNOSTIC "an undeclared diagnostic was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- LINK (M6)
T="$(tree link)"
mutate "$(CH "$T")" '[envelope §10](feature-matrix.md) | `safety`' '[envelope §19](feature-matrix.md) | `safety`' \
  || bad LINK "the mutation did not apply" ""
out="$(run "$T")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'cites §19, which feature-matrix.md has no heading for' <<<"$out"; then
  ok LINK "a citation of a clause the target does not carry is refused (exit=$rc)"
else
  bad LINK "a dead clause citation was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- CONTROL
T="$(tree control)"
mutate "$(CH "$T")" '## 7. Pseudolocalization' $'## 7. Pseudolocalization\n\nA sentence that changes no rule.' \
  || bad CONTROL "the mutation did not apply" ""
out="$(run "$T")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q '/ 0 failure(s)' <<<"$out"; then
  ok CONTROL "an unrelated prose edit leaves the verdict green, so the RED arms are not vacuous"
else
  bad CONTROL "the census reddened on a change that breaks no rule (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- MISSING
T="$(tree missing)"; rm -f "$(CH "$T")"
out="$(run "$T")"; rc=$?
if [ "$rc" -eq 2 ] && grep -q 'REFUSED' <<<"$out"; then
  ok MISSING "an absent chapter refuses with exit=2 instead of reporting green"
else
  bad MISSING "a missing chapter did not refuse (exit=$rc)" "$out"
fi

echo
printf 'probes: %d pass / %d fail\n' "$pass" "$fail"
[ "$fail" -eq 0 ] || exit 1
exit 0
