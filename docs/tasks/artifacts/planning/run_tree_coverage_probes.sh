#!/usr/bin/env bash
# docs/tasks/artifacts/planning/run_tree_coverage_probes.sh
# PLANNING lane (added by the slice that fixed defect D44) — end-to-end probes for the tree-coverage census.
#
# WHY THIS SUITE EXISTS: the census behind "the whole roadmap is represented as task-trees" had no probe
# suite and no gate ran it, so it went RED for two committed slices and nobody noticed — the evidence split
# the containment registry prescribes put `*-evidence.md` files under `docs/tasks/`, and a census that
# enumerates every file there as a tree reported two lane-less orphans (defect D44). A census whose green
# nobody re-derives is a claim, not an instrument: `make probes` is what turns it into one.
#
# Each arm copies the real roadmap, index and tree directory into a synthetic root, breaks exactly one
# property, and requires the census to name it. The real tree is never mutated.
#
#   REAL            the census passes on this repository as it stands, siblings counted as siblings
#   SIBLING-RED     a tree that stops linking its evidence sibling turns that sibling into an orphan
#   STRAY-RED       a file in docs/tasks/ that is neither a tree nor linked is an orphan
#   UNOWNED-RED     a roadmap lane whose tree file is missing is UNOWNED
#   LANELESS-RED    a tree whose metadata stops declaring its lane is an orphan
#   DEADLINK-RED    an index row pointing at a file that does not exist is a dead link
#   MISSING         a root with no roadmap REFUSES (exit 2) instead of reporting green over nothing
#
# Usage:  bash docs/tasks/artifacts/planning/run_tree_coverage_probes.sh
# Output: per-arm lines plus `probes: N pass / M fail` (exit nonzero if M > 0).
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
CENSUS="$ROOT/docs/tasks/artifacts/planning/run_tree_coverage_census.sh"
[ -f "$CENSUS" ] || { echo "probe: REFUSED — $CENSUS not found" >&2; exit 2; }

WORK="${TMPDIR:-$ROOT/target/scratch}/tree_coverage_probes"
rm -rf "$WORK"; mkdir -p "$WORK"; trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { printf '  ✓ %-13s %s\n' "$1" "$2"; pass=$((pass+1)); }
bad() { printf '  ✗ %-13s %s\n' "$1" "$2"; fail=$((fail+1))
        [ -n "${3:-}" ] && printf '%s\n' "$3" | grep -E 'ORPHAN|UNOWNED|DEAD LINK|^census:|REFUSED' | head -6 | sed 's/^/            /'; }

mkroot() { # $1 = dir; copies the three inputs the census reads
  local d="$1"; rm -rf "$d"; mkdir -p "$d/docs"
  cp "$ROOT/ROADMAP.md" "$d/ROADMAP.md"
  cp "$ROOT/docs/TASK_TREE.md" "$d/docs/TASK_TREE.md"
  cp -R "$ROOT/docs/tasks" "$d/docs/tasks"
  rm -rf "$d/docs/tasks/artifacts"
}
run() { TREE_COVERAGE_ROOT="$1" bash "$CENSUS" 2>&1; }
summary() { grep -o 'census: [0-9]* lanes.*' <<<"$1" | head -1; }

echo "tree-coverage probes — docs/tasks/artifacts/planning/run_tree_coverage_census.sh"

# ---------------------------------------------------------------- REAL
out="$(run "$ROOT")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q '/ 0 orphan(s) / 0 dead link(s)' <<<"$out"; then
  ok REAL "$(summary "$out")"
else
  bad REAL "the real tree does not satisfy its own coverage census (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- SIBLING-RED: an unlinked sibling is a stray
D="$WORK/sibling"; mkroot "$D"
python3 - "$D/docs/tasks/G0-CONTRACT.md" <<'PY'
import pathlib, sys
p = pathlib.Path(sys.argv[1]); t = p.read_text(encoding="utf-8")
n = t.replace("](G0-CONTRACT-evidence.md)", "](nowhere.md)")
assert n != t, "the link this arm removes was not found"
p.write_text(n, encoding="utf-8")
PY
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -qE 'G0-CONTRACT-evidence +ORPHAN' <<<"$out"; then
  ok SIBLING-RED "a tree that stops linking its evidence sibling makes the census name the orphan (exit=$rc)"
else
  bad SIBLING-RED "an unlinked evidence sibling was still accepted as a sibling (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- STRAY-RED
D="$WORK/stray"; mkroot "$D"
printf '# not a tree, and nothing links to it\n' > "$D/docs/tasks/STRAY-NOTES.md"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -qE 'STRAY-NOTES +ORPHAN' <<<"$out"; then
  ok STRAY-RED "a file in docs/tasks/ that is neither tree nor sibling is refused by name (exit=$rc)"
else
  bad STRAY-RED "a stray file in the tree directory was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- UNOWNED-RED: a lane with no tree
D="$WORK/unowned"; mkroot "$D"
rm -f "$D/docs/tasks/G2-2D.md"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'NONE' <<<"$out" && grep -q '/ 1 unowned /' <<<"$out"; then
  ok UNOWNED-RED "a roadmap lane whose tree is missing is reported UNOWNED by name (exit=$rc)"
else
  bad UNOWNED-RED "a lane with no tree was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- LANELESS-RED: a tree that stops declaring its lane
# Population 2 is what catches this one: population 1 accepts any mention of the lane anywhere in the file,
# so a tree whose METADATA line is deleted is still "cited" by its own prose and only the per-file rule
# notices. Both refusals are asserted so the arm cannot pass on the wrong one.
D="$WORK/laneless"; mkroot "$D"
python3 - "$D/docs/tasks/G2-2D.md" <<'PY'
import pathlib, sys
p = pathlib.Path(sys.argv[1]); t = p.read_text(encoding="utf-8")
lines = [l for l in t.splitlines(keepends=True) if not l.startswith("- Roadmap lane:")]
assert len(lines) != len(t.splitlines()), "no lane line to remove"
p.write_text("".join(lines), encoding="utf-8")
PY
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'NO LANE DECLARED' <<<"$out" \
   && grep -q '/ 1 orphan(s) /' <<<"$out"; then
  ok LANELESS-RED "a tree whose metadata stops declaring its lane is an orphan (exit=$rc)"
else
  bad LANELESS-RED "a lane-less tree was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- DEADLINK-RED
D="$WORK/deadlink"; mkroot "$D"
printf '| [`NOPE`](tasks/NOPE.md) | a lane nobody owns | `active` | `.1` | repo-local |\n' >> "$D/docs/TASK_TREE.md"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'DEAD LINK: tasks/NOPE.md' <<<"$out"; then
  ok DEADLINK-RED "an index row pointing at a file that does not exist is refused by name (exit=$rc)"
else
  bad DEADLINK-RED "a dead index link was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- MISSING
D="$WORK/empty"; mkdir -p "$D"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 2 ] && grep -q 'REFUSED' <<<"$out"; then
  ok MISSING "a root with no roadmap refuses with exit=2 instead of reporting green over nothing"
else
  bad MISSING "an absent roadmap did not refuse (exit=$rc)" "$out"
fi

echo
printf 'probes: %d pass / %d fail\n' "$pass" "$fail"
[ "$fail" -eq 0 ] || exit 1
exit 0
