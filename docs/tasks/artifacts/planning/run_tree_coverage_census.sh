#!/usr/bin/env bash
# docs/tasks/artifacts/planning/run_tree_coverage_census.sh
# PLANNING.3 — the census behind the claim "the whole roadmap is represented as task-trees".
#
# WHY A CENSUS AND NOT A SENTENCE: "every lane is owned" is a universally quantified claim over two
# populations (the roadmap's gates/tracks, and the tree files on disk). It is false the moment one
# member of either population is missing, and it is exactly the kind of claim that stays true in prose
# while going false in fact — this project measured that (defect D24: five of ten lanes had no tree
# after the claim had been written twice). So the claim is derived, per CLAIM_VERIFICATION.md leg 1.
#
# WHAT IT CHECKS, in both directions:
#   1. every roadmap §11 lane (G0–G7, V1, V2) has a tree file whose Metadata names that lane;
#   2. every TREE file on disk is registered in docs/TASK_TREE.md (no orphan trees). A tree is recognised
#      STRUCTURALLY — it declares `- Tree ID:` — not by its filename, because docs/tasks/ also holds the
#      evidence siblings the containment registry prescribes for a tree past 1000 lines. Enumerating every
#      file as a tree reported two lane-less orphans the moment the first sibling landed (defect D44);
#      enumerating by a filename suffix would let a real tree hide by being named oddly. A non-tree file
#      must be linked from a tree, which is what makes it a sibling rather than a stray;
#   3. every index entry resolves to a file on disk (no dead links — defect D1's class);
#   4. every tree declares a lane: either a roadmap gate/track, or an explicitly repo-local lane
#      (PLANNING, SPINE, BOOTSTRAP), so a tree can never be lane-less by omission;
#   5. advisory: per gate tree, the leaf count and the exit-clause table row count, printed beside the
#      roadmap's own clause count for that gate — a human-readable side-by-side, because matching prose
#      clauses to table rows mechanically would be a classifier guessing at meaning.
#
# Usage:  bash docs/tasks/artifacts/planning/run_tree_coverage_census.sh
#         TREE_COVERAGE_ROOT=<dir> bash docs/tasks/artifacts/planning/run_tree_coverage_census.sh
# Output: the census tables plus
#         `census: <lanes> lanes / <trees> trees / <siblings> siblings / <unowned> unowned / <orphans> orphan(s) / <dead> dead link(s)`
#         and exit nonzero if anything is unowned, orphaned or dead-linked.
set -uo pipefail
ROOT="${TREE_COVERAGE_ROOT:-$(git rev-parse --show-toplevel)}"; cd "$ROOT" || exit 2
ROADMAP="ROADMAP.md"; INDEX="docs/TASK_TREE.md"; TASKS="docs/tasks"
LOCAL_LANES="PLANNING SPINE BOOTSTRAP"

for f in "$ROADMAP" "$INDEX"; do
  [ -f "$f" ] || { echo "census: REFUSED — $f not found" >&2; exit 2; }
done

unowned=0; orphans=0; dead=0; lanes=0; trees=0; siblings=0

# ── population 1: the roadmap's own lanes ────────────────────────────────────────────────────
# bash 3.2 compatible (no mapfile): the spine must run on whatever bash the platform ships.
ROADMAP_LANES=()
while IFS= read -r _lane; do
  [ -n "$_lane" ] && ROADMAP_LANES+=("$_lane")
done < <(grep -oE '^### (G[0-7]|V[12]) ' "$ROADMAP" | awk '{print $2}' | LC_ALL=C sort)

echo "=== 1. roadmap lanes (§11) → owning tree ==="
printf '  %-5s %-22s %s\n' "LANE" "TREE" "METADATA CITES LANE"
for lane in "${ROADMAP_LANES[@]}"; do
  lanes=$((lanes + 1))
  # The lane's tree is the file that DECLARES itself a tree, not the first alphabetical match: a lane's
  # evidence sibling sorts before the tree itself (`X-evidence.md` < `X.md`), and picking it made the lane
  # look unowned the moment the containment split landed (defect D44).
  match=""
  for cand in "$TASKS"/${lane}-*.md; do
    [ -f "$cand" ] || continue
    if grep -qE '^- Tree ID:' "$cand"; then match="$cand"; break; fi
  done
  if [ -z "$match" ]; then
    printf '  %-5s %-22s %s\n' "$lane" "— NONE —" "UNOWNED"
    unowned=$((unowned + 1)); continue
  fi
  base="$(basename "$match" .md)"
  if grep -q "gate \*\*${lane}\|track \*\*${lane}\|§11 .*${lane}\|\*\*${lane} " "$match" 2>/dev/null \
     || grep -qE "Roadmap lane:.*${lane}" "$match"; then
    cited=yes
  else
    cited="NO — the tree does not name its lane"; unowned=$((unowned + 1))
  fi
  printf '  %-5s %-22s %s\n' "$lane" "$base" "$cited"
done

# ── population 2: the tree files on disk ──────────────────────────────────────────────────────
echo
echo "=== 2. files in docs/tasks/ → tree (registered, lane declared) or sibling (linked from a tree) ==="
printf '  %-22s %-12s %s\n' "TREE" "IN INDEX" "LANE"
for f in "$TASKS"/*.md; do
  b="$(basename "$f")"
  [ "$b" = TEMPLATE.md ] && continue
  base="${b%.md}"
  if grep -qE '^- Tree ID:' "$f"; then
    trees=$((trees + 1))
    if grep -q "tasks/$b" "$INDEX"; then inidx=yes; else inidx="NO"; orphans=$((orphans + 1)); fi
    lane="$(grep -m1 -E '^- Roadmap lane:' "$f" | sed 's/^- Roadmap lane: *//; s/`//g' | cut -c1-58)"
    [ -n "$lane" ] || { lane="NO LANE DECLARED"; orphans=$((orphans + 1)); }
    printf '  %-22s %-12s %s\n' "$base" "$inidx" "${lane:-(unset)}"
  else
    # Not a tree. It is legitimate only as a sibling some tree links to — the evidence split the
    # containment registry prescribes. An unlinked non-tree file in docs/tasks/ is a stray: nothing owns it.
    siblings=$((siblings + 1))
    esc="$(printf '%s' "$b" | sed 's/\./\\./g')"
    owner="$(grep -lE "\\]\\($esc\\)" "$TASKS"/*.md 2>/dev/null | grep -v "^$f$" | head -1)"
    if [ -n "$owner" ]; then
      printf '  %-22s %-12s %s\n' "$base" "sibling" "evidence of $(basename "$owner" .md)"
    else
      printf '  %-22s %-12s %s\n' "$base" "ORPHAN" "declares no \`- Tree ID:\` and no tree links to it"
      orphans=$((orphans + 1))
    fi
  fi
done

# ── direction 3: index entries must resolve ──────────────────────────────────────────────────
echo
echo "=== 3. index entries → files on disk ==="
while IFS= read -r link; do
  if [ ! -f "docs/$link" ]; then
    echo "  DEAD LINK: $link"; dead=$((dead + 1))
  fi
done < <(grep -oE 'tasks/[A-Za-z0-9_-]+\.md' "$INDEX" | LC_ALL=C sort -u)
[ "$dead" -eq 0 ] && echo "  no dead links"

# ── advisory: clause coverage per gate tree ──────────────────────────────────────────────────
echo
echo "=== 4. advisory — exit clauses vs leaves per gate tree ==="
printf '  %-16s %-14s %-8s %s\n' "TREE" "CLAUSE ROWS" "LEAVES" "ROADMAP §11 CLAUSES (semicolon split, advisory)"
for lane in "${ROADMAP_LANES[@]}"; do
  # the same structural rule as population 1, and for the same reason: `ls ${lane}-*.md | head -1` sorts an
  # evidence sibling before its tree, so the advisory reported the sibling and the lane's real tree vanished
  # from the table (defect D44's second instance, found by reading the output rather than the exit code).
  match=""
  for cand in "$TASKS"/${lane}-*.md; do
    [ -f "$cand" ] || continue
    if grep -qE '^- Tree ID:' "$cand"; then match="$cand"; break; fi
  done
  [ -n "$match" ] || continue
  base="$(basename "$match" .md)"
  rows="$(awk '/^## Acceptance Criteria/{s=1;next} /^## /{s=0} s&&/^\|/{print}' "$match" \
          | grep -cE '\|[^|]*\.[0-9]+[^|]*\|' || true)"
  leaves="$(grep -cE "^- ID: \`${base}\.[0-9]" "$match" || true)"
  # the exit clause is wrapped prose: take the whole gate block, join the Exit bullet and its
  # continuation lines, then count semicolon-separated clauses (advisory only — see the header).
  rm_clauses="$(awk -v l="### $lane " '
      index($0,l)==1 {f=1; next}
      /^### |^---$/   {f=0}
      f && /^- \*\*Exit:/ {ine=1}
      f && ine && /^[[:space:]]*$/ {ine=0}
      f && ine {printf "%s ", $0}
    ' "$ROADMAP" | awk -F';' '{print NF}')"
  printf '  %-16s %-14s %-8s %s\n' "$base" "${rows:-0}" "${leaves:-0}" "${rm_clauses:-n/a}"
done

echo
printf 'census: %d lanes / %d trees / %d sibling(s) / %d unowned / %d orphan(s) / %d dead link(s)\n' \
       "$lanes" "$trees" "$siblings" "$unowned" "$orphans" "$dead"
if [ "$unowned" -ne 0 ] || [ "$orphans" -ne 0 ] || [ "$dead" -ne 0 ]; then
  echo "census: FAILED — a roadmap lane without a tree, a tree without an index row or lane, a file in" >&2
  echo "  docs/tasks/ that is neither a tree nor a linked sibling, or a dead index link means the capture" >&2
  echo "  claim is false. Fix the tree set, not the census." >&2
  exit 1
fi
exit 0
