#!/usr/bin/env bash
# docs/tasks/artifacts/command_layer/run_command_layer_probes.sh
# G0-CONTRACT.17 — end-to-end probes for the command-layer census.
#
# WHY: the census exists so that "this chapter is roadmap §4.4 and §7.8" is derived rather than asserted,
# and an instrument that has only ever agreed with the chapter it was written against has never been seen to
# notice anything. Each RED arm copies the real book, removes ONE property, and requires a refusal naming it.
#
# ⚠ Every mutation is applied by exact replacement and REFUSES (exit 3) when its pattern is absent.
# ⚠ Each arm removes exactly one property: LEVEL-EXTRA adds a sixth level and leaves the five alone,
#   LEVEL-MISSING drops one and leaves the rest, so the two directions of K3 are proved separately rather
#   than one arm covering for the other.
# ⚠ CONTROL is the arm that proves the others are not vacuous.
#
#   REAL               the book as it stands: 17 commands, 5 classes, 5 levels, 0 failures
#   COMMAND-GAP        a command the roadmap names, dropped from §1.1 → K1
#   CLASS-UNDECLARED   a row whose class §1 does not declare → K2
#   AUTHORITY-LEVEL    a row whose authority §7 does not declare → K2
#   REVERSIBILITY      a row whose reversibility is not in §1's vocabulary → K2
#   LEVEL-EXTRA        a sixth authority level → K3
#   LEVEL-MISSING      one of the roadmap's five levels dropped → K3
#   HUMAN-ONLY         §7 stops saying approval cannot be an agent's → K3
#   COLUMN-UNGRAUNDED  a parity column with no declared source → K4
#   ABSENT-UNDECLARED  the parity cell vocabulary loses `absent` → K4
#   DIAGNOSTIC         a `command_*` token §9 does not declare → K5
#   LINK               a citation of a clause the target does not carry → K6
#   CONTROL            an unrelated prose edit → still green
#   MISSING            an absent chapter REFUSES (exit 2) instead of reporting green
#
# Usage:  bash docs/tasks/artifacts/command_layer/run_command_layer_probes.sh
#         TMPDIR=<dir> bash …   (`make probes` pins it to the repository volume)
# Output: per-arm lines plus `probes: N pass / M fail` (exit nonzero if M > 0).
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
TOOL="$ROOT/docs/tasks/artifacts/command_layer/run_command_layer_census.sh"
BOOK="$ROOT/docs/book/src"
CHAPTER="$BOOK/spec/command-layer.md"
for f in "$TOOL" "$CHAPTER" "$ROOT/ROADMAP.md"; do
  [ -f "$f" ] || { echo "probe: REFUSED — $f not found" >&2; exit 2; }
done
command -v python3 >/dev/null 2>&1 || { echo "probe: REFUSED — python3 not found" >&2; exit 2; }

WORK="${TMPDIR:-$ROOT/target/scratch}/command_layer_probes"
rm -rf "$WORK"; mkdir -p "$WORK"; trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { printf '  ✓ %-18s %s\n' "$1" "$2"; pass=$((pass+1)); }
bad() { printf '  ✗ %-18s %s\n' "$1" "$2"; fail=$((fail+1))
        [ -n "${3:-}" ] && printf '%s\n' "$3" | grep -E '^  x |command-layer census:|REFUSED' | head -4 | sed 's/^/                   /'; }

book() {
  local d="$WORK/$1"; rm -rf "$d"; mkdir -p "$d"
  cp -R "$BOOK" "$d/src"
  printf '%s' "$d"
}
run() { COMMAND_BOOK="$1/src" bash "$TOOL" 2>&1; }
CH() { printf '%s' "$1/src/spec/command-layer.md"; }

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

echo "command-layer probes — docs/tasks/artifacts/command_layer/run_command_layer_census.sh"

# ---------------------------------------------------------------- REAL
B="$(book real)"; out="$(run "$B")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q '/ 0 failure(s)' <<<"$out"; then
  ok REAL "$(grep -o 'command-layer census: .*' <<<"$out")"
else
  bad REAL "the real book does not pass its own census (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- COMMAND-GAP (K1)
B="$(book command_gap)"
droprow "$(CH "$B")" '| `SplitSeamSpan` | `mutation` |' || bad COMMAND-GAP "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'roadmap §4.4 names `SplitSeamSpan` and §1.1 carries no row' <<<"$out"; then
  ok COMMAND-GAP "a command the roadmap names and the chapter drops is refused by name (exit=$rc)"
else
  bad COMMAND-GAP "a dropped roadmap command was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- CLASS-UNDECLARED (K2)
B="$(book class)"
mutate "$(CH "$B")" '| `AddNotch` | `mutation` |' '| `AddNotch` | `inspection` |' \
  || bad CLASS-UNDECLARED "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q '`AddNotch` declares class .inspection., which §1 does not' <<<"$out"; then
  ok CLASS-UNDECLARED "a command in a class the chapter does not declare is refused (exit=$rc)"
else
  bad CLASS-UNDECLARED "an undeclared class was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- AUTHORITY-LEVEL (K2)
B="$(book authority)"
mutate "$(CH "$B")" '| `ExportDxf` | `artifact` | `generate` |' '| `ExportDxf` | `artifact` | `execute` |' \
  || bad AUTHORITY-LEVEL "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q '`ExportDxf` declares authority .execute., which §7 does not' <<<"$out"; then
  ok AUTHORITY-LEVEL "a command requiring an authority nobody holds is refused (exit=$rc)"
else
  bad AUTHORITY-LEVEL "an undeclared authority was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- REVERSIBILITY (K2)
B="$(book reversible)"
mutate "$(CH "$B")" '| `ChangeMeasurement` | `mutation` | `commit` | `yes` |' \
  '| `ChangeMeasurement` | `mutation` | `commit` | `maybe` |' || bad REVERSIBILITY "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q '`ChangeMeasurement` declares reversibility .maybe.' <<<"$out"; then
  ok REVERSIBILITY "a reversibility outside §1's vocabulary is refused (exit=$rc)"
else
  bad REVERSIBILITY "an undeclared reversibility was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- LEVEL-EXTRA (K3)
B="$(book level_extra)"
mutate "$(CH "$B")" '| `approve` | `ApprovePackage` | be held by an agent at all |' \
  '| `approve` | `ApprovePackage` | be held by an agent at all |
| `administer` | anything | anything |' || bad LEVEL-EXTRA "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q '§7 declares the level `administer`, which roadmap §7.8 does not name' <<<"$out"; then
  ok LEVEL-EXTRA "a sixth authority level is refused: it is a governance change, not a chapter edit (exit=$rc)"
else
  bad LEVEL-EXTRA "an invented authority level was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- LEVEL-MISSING (K3)
B="$(book level_missing)"
droprow "$(CH "$B")" '| `propose` | build a candidate group' || bad LEVEL-MISSING "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'roadmap §7.8 names the authority level `propose` and §7 does not declare it' <<<"$out"; then
  ok LEVEL-MISSING "a level the roadmap names and the chapter drops is refused (exit=$rc)"
else
  bad LEVEL-MISSING "a dropped authority level was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- HUMAN-ONLY (K3)
B="$(book human_only)"
mutate "$(CH "$B")" '- **`approve` is human-only** and no graph mutation can manufacture it' \
  '- **`approve` is usually a person** and no graph mutation can manufacture it' \
  || bad HUMAN-ONLY "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'does not state that `approve` is human-only' <<<"$out"; then
  ok HUMAN-ONLY "softening the one property §7.8 calls unmanufacturable is refused (exit=$rc)"
else
  bad HUMAN-ONLY "a softened human-only rule was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- COLUMN-UNGRAUNDED (K4)
B="$(book column)"
mutate "$(CH "$B")" '| state parity | the digest of the resulting design, identical across adapters or the difference explained |' \
  '| state parity | — |' || bad COLUMN-UNGRAUNDED "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q "parity column .state parity. declares no source" <<<"$out"; then
  ok COLUMN-UNGRAUNDED "a parity column nobody derives is refused (exit=$rc)"
else
  bad COLUMN-UNGRAUNDED "an ungrounded parity column was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- ABSENT-UNDECLARED (K4)
B="$(book absent)"
droprow "$(CH "$B")" '| `absent` | the adapter has no entry point' || bad ABSENT-UNDECLARED "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q "cell vocabulary does not declare \`absent\`" <<<"$out"; then
  ok ABSENT-UNDECLARED "a gap with no declared spelling is refused (exit=$rc)"
else
  bad ABSENT-UNDECLARED "an undeclared cell value was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- DIAGNOSTIC (K5)
B="$(book diagnostic)"
mutate "$(CH "$B")" '`command_revision_stale`, naming both revisions' \
  '`command_revision_old`, naming both revisions' || bad DIAGNOSTIC "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'names `command_revision_old`, which §9 does not declare' <<<"$out"; then
  ok DIAGNOSTIC "a diagnostic no table declares is refused by name (exit=$rc)"
else
  bad DIAGNOSTIC "an undeclared diagnostic was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- LINK (K6)
B="$(book link)"
mutate "$(CH "$B")" '[release §7](release-contract.md)' '[release §17](release-contract.md)' \
  || bad LINK "the mutation did not apply" ""
out="$(run "$B")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'cites §17, which release-contract.md has no heading for' <<<"$out"; then
  ok LINK "a citation of a clause the target does not carry is refused (exit=$rc)"
else
  bad LINK "a dead clause citation was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- CONTROL
B="$(book control)"
mutate "$(CH "$B")" '## 6. Structured errors and progress' \
  $'## 6. Structured errors and progress\n\nA sentence that changes no rule.' \
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
