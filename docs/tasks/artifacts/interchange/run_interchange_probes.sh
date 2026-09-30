#!/usr/bin/env bash
# docs/tasks/artifacts/interchange/run_interchange_probes.sh
# G0-CONTRACT.10 — end-to-end probes for the interchange census.
#
# WHY: the census exists so that "every layer the roadmap names is dispositioned" is a derived claim
# rather than a sentence, and an instrument that has only ever agreed with the chapter it was written
# against has never been seen to notice anything. Each RED arm below copies the real book, removes ONE
# property, and requires a refusal that names it.
#
# ⚠ Every mutation is applied by exact replacement and REFUSES (exit 3) when its pattern is absent: a
#   mutation that silently did not apply is a green arm that tested nothing.
# ⚠ ROADMAP-GROWS is the arm that proves the census reads the roadmap instead of a list hardcoded beside
#   it — the property it removes is in a file the chapter never mentions, and it mutates a COPY, because
#   the tracked roadmap is the director's to amend and a probe has no business editing it.
# ⚠ CONTROL is the arm that proves the others are not vacuous: an unrelated prose edit must stay green.
#
#   REAL               the book as it stands: 17 layers, 4 targets, 12 entities, 0 failures
#   LAYER-GAP          a §3 row dropped, so a layer the roadmap names is undispositioned → D1
#   ROADMAP-GROWS      the roadmap's convention gains a layer the chapter does not carry → D1
#   AAMA-INVENTED      an eighth AAMA name the roadmap does not list → D2
#   DUP-LAYER          one layer claimed by two rows → D2
#   AXIS-COLUMN        a registry column no axis declares → D3
#   UNREGISTERED       a target token named in prose that §2 does not register → D3
#   ENTITY-DRIFT       a refused entity marked written → D4
#   ENTITY-STATUS      a per-release entity set → D4
#   DIAGNOSTIC         a token named that neither §11 nor the envelope declares → D5
#   LINK               a citation of a clause the target does not carry → D6
#   CONTROL            an unrelated prose edit → still green
#   MISSING            an absent chapter REFUSES (exit 2) instead of reporting green
#
# Usage:  bash docs/tasks/artifacts/interchange/run_interchange_probes.sh
#         TMPDIR=<dir> bash …   (`make probes` pins it to the repository volume)
# Output: per-arm lines plus `probes: N pass / M fail` (exit nonzero if M > 0).
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
TOOL="$ROOT/docs/tasks/artifacts/interchange/run_interchange_census.sh"
BOOK="$ROOT/docs/book/src"
CHAPTER="$BOOK/spec/interchange-dialects.md"
for f in "$TOOL" "$CHAPTER" "$ROOT/ROADMAP.md"; do
  [ -f "$f" ] || { echo "probe: REFUSED — $f not found" >&2; exit 2; }
done
command -v python3 >/dev/null 2>&1 || { echo "probe: REFUSED — python3 not found" >&2; exit 2; }

WORK="${TMPDIR:-$ROOT/target/scratch}/interchange_probes"
rm -rf "$WORK"; mkdir -p "$WORK"; trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { printf '  ✓ %-16s %s\n' "$1" "$2"; pass=$((pass+1)); }
bad() { printf '  ✗ %-16s %s\n' "$1" "$2"; fail=$((fail+1))
        [ -n "${3:-}" ] && printf '%s\n' "$3" | grep -E '^  x |interchange census:|REFUSED' | head -5 | sed 's/^/               /'; }

# book <arm>: a fresh copy of the whole book, so the census's link resolution and its reading of the
# envelope chapter stay faithful while one file is mutated.
book() {
  local d="$WORK/$1"
  rm -rf "$d"; mkdir -p "$d"
  cp -R "$BOOK" "$d/src"
  cp "$ROOT/ROADMAP.md" "$d/ROADMAP.md"
  printf '%s' "$d"
}
run() { INTERCHANGE_BOOK="$1/src" INTERCHANGE_ROADMAP="$1/ROADMAP.md" bash "$TOOL" 2>&1; }

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

echo "interchange probes — docs/tasks/artifacts/interchange/run_interchange_census.sh"

# ---------------------------------------------------------------- REAL
W="$(book real)"; out="$(run "$W")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q '/ 0 failure(s)' <<<"$out"; then
  ok REAL "$(grep -o 'interchange census: .*' <<<"$out")"
else
  bad REAL "the real book does not pass its own census (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- LAYER-GAP (D1)
W="$(book layer_gap)"
droprow "$W/src/spec/interchange-dialects.md" '| 13 | drill holes |' || bad LAYER-GAP "the mutation did not apply" ""
out="$(run "$W")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'the roadmap names layer 13 and §3 has no row for it' <<<"$out"; then
  ok LAYER-GAP "a layer the roadmap names and the chapter drops is refused by number (exit=$rc)"
else
  bad LAYER-GAP "an undispositioned layer was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- ROADMAP-GROWS (D1)
W="$(book roadmap_grows)"
mutate "$W/ROADMAP.md" '15 annotation text,' '15 annotation text, 16 seam-test curve,' \
  || bad ROADMAP-GROWS "the mutation did not apply" ""
out="$(run "$W")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'the roadmap names layer 16 and §3 has no row for it' <<<"$out"; then
  ok ROADMAP-GROWS "a convention the roadmap grows is refused until the chapter dispositions it (exit=$rc)"
else
  bad ROADMAP-GROWS "a roadmap layer the chapter does not carry was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- AAMA-INVENTED (D2)
W="$(book aama)"
mutate "$W/src/spec/interchange-dialects.md" '| 14 | the sew line | the net line, allowance excluded | DRAW |' \
  '| 14 | the sew line | the net line, allowance excluded | SEWLINE |' || bad AAMA-INVENTED "the mutation did not apply" ""
out="$(run "$W")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'uses the AAMA name SEWLINE, which the roadmap does not list' <<<"$out"; then
  ok AAMA-INVENTED "an eighth AAMA name is refused by name (exit=$rc)"
else
  bad AAMA-INVENTED "an eighth AAMA name was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- DUP-LAYER (D2)
W="$(book dup_layer)"
mutate "$W/src/spec/interchange-dialects.md" '| 7 | grainline | `Grainline`, directed | DRAW |' \
  '| 7 | grainline | `Grainline`, directed | DRAW |
| 7 | a second meaning | something else entirely | DRAW |' || bad DUP-LAYER "the mutation did not apply" ""
out="$(run "$W")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'layer 7 is claimed twice' <<<"$out"; then
  ok DUP-LAYER "one layer with two meanings is refused (exit=$rc)"
else
  bad DUP-LAYER "a doubly claimed layer was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- AXIS-COLUMN (D3)
W="$(book axis_column)"
mutate "$W/src/spec/interchange-dialects.md" '| Target | Layer naming | Release | Grading carriage | Entity policy | Proven at |' \
  '| Target | Layer naming | Dialect | Grading carriage | Entity policy | Proven at |' \
  || bad AXIS-COLUMN "the mutation did not apply" ""
out="$(run "$W")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q '§2 has no such column' <<<"$out" && grep -q 'no axis in §1 declares' <<<"$out"; then
  ok AXIS-COLUMN "a registry column and an axis that disagree are refused in both directions (exit=$rc)"
else
  bad AXIS-COLUMN "a column no axis declared was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- UNREGISTERED (D3)
W="$(book unregistered)"
mutate "$W/src/spec/interchange-dialects.md" 'Neither is a superset of' \
  'A `dxf-astm-num-r12` variant is tempting. Neither is a superset of' \
  || bad UNREGISTERED "the mutation did not apply" ""
out="$(run "$W")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'names the target `dxf-astm-num-r12`, which §2 does not register' <<<"$out"; then
  ok UNREGISTERED "a target nobody validated is refused by name (exit=$rc)"
else
  bad UNREGISTERED "an unregistered target was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- ENTITY-DRIFT (D4)
W="$(book entity_drift)"
mutate "$W/src/spec/interchange-dialects.md" '| `SPLINE` | never |' '| `SPLINE` | both releases |' \
  || bad ENTITY-DRIFT "the mutation did not apply" ""
out="$(run "$W")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q '`SPLINE` is not refused' <<<"$out"; then
  ok ENTITY-DRIFT "a legacy importer's refusal quietly widened is caught (exit=$rc)"
else
  bad ENTITY-DRIFT "a written SPLINE was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- ENTITY-STATUS (D4)
W="$(book entity_status)"
mutate "$W/src/spec/interchange-dialects.md" '| `TEXT` | both releases |' '| `TEXT` | R13 only |' \
  || bad ENTITY-STATUS "the mutation did not apply" ""
out="$(run "$W")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'a per-release entity set is two' <<<"$out"; then
  ok ENTITY-STATUS "an entity set that differs per release is refused (exit=$rc)"
else
  bad ENTITY-STATUS "a per-release entity set was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- DIAGNOSTIC (D5)
W="$(book diagnostic)"
mutate "$W/src/spec/interchange-dialects.md" 'is refused with `dialect_unregistered`, naming the' \
  'is refused with `dialect_mystery`, naming the' || bad DIAGNOSTIC "the mutation did not apply" ""
out="$(run "$W")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'names `dialect_mystery`, which neither its §11 nor' <<<"$out"; then
  ok DIAGNOSTIC "a diagnostic no table declares is refused by name (exit=$rc)"
else
  bad DIAGNOSTIC "an undeclared diagnostic was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- LINK (D6)
W="$(book link)"
mutate "$W/src/spec/interchange-dialects.md" '([units §3](units-and-tolerances.md))' \
  '([units §12](units-and-tolerances.md))' || bad LINK "the mutation did not apply" ""
out="$(run "$W")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'cites §12, which units-and-tolerances.md has no heading for' <<<"$out"; then
  ok LINK "a citation of a clause the target does not carry is refused (exit=$rc)"
else
  bad LINK "a dead clause citation was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- CONTROL
W="$(book control)"
mutate "$W/src/spec/interchange-dialects.md" '## 7. Tessellation policy' \
  $'## 7. Tessellation policy\n\nA sentence that changes no rule.' || bad CONTROL "the mutation did not apply" ""
out="$(run "$W")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q '/ 0 failure(s)' <<<"$out"; then
  ok CONTROL "an unrelated prose edit leaves the verdict green, so the RED arms are not vacuous"
else
  bad CONTROL "the census reddened on a change that breaks no rule (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- MISSING
W="$(book missing)"; rm -f "$W/src/spec/interchange-dialects.md"
out="$(run "$W")"; rc=$?
if [ "$rc" -eq 2 ] && grep -q 'REFUSED' <<<"$out"; then
  ok MISSING "an absent chapter refuses with exit=2 instead of reporting green"
else
  bad MISSING "a missing chapter did not refuse (exit=$rc)" "$out"
fi

echo
printf 'probes: %d pass / %d fail\n' "$pass" "$fail"
[ "$fail" -eq 0 ] || exit 1
exit 0
