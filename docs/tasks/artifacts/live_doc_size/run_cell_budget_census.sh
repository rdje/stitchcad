#!/usr/bin/env bash
# docs/tasks/artifacts/live_doc_size/run_cell_budget_census.sh
# SPINE.4.4 — the measurement behind a table-shaped collection's maxline health target.
#
# WHY THIS EXISTS: `.doctrine/live_document_size/surfaces.tsv` derives a collection's per-part health from
# the SHAPE of its content, "never from today's largest file" — and for the maximum-content-line axis the
# shape of a book part is its TABLES, not its prose. The `book_collection` maxline health was `200` B, a
# number that fits a prose paragraph and not one five-column termbase row: every glossary part exceeded it
# (widest `272` B) while staying well inside the `320` B ceiling, so the check printed a permanent warning
# with no defect behind it. A warning nobody can act on teaches the next reader to skip the ones that mean
# something, which is the same failure as a gate that cries wolf.
#
# So the target is derived from a CELL BUDGET: a table row's bytes are the sum of its cells plus the
# separator overhead — `3 × columns + 1`, one pipe per column plus a trailing pipe and one space on each
# side of every cell. Measuring each column's own distribution (header cell included, because a header is a
# line too) gives the widest row a shape legitimately needs, instead of guessing a number and calling the
# guess a target.
#
# WHAT IT PRINTS
#   per table SHAPE (the header cells, which is what makes two tables the same shape):
#     data rows counted, and per column the widest cell and the 95th-percentile cell in bytes
#     two derived row budgets: `p95-sum` (the steady state — what a column needs, per column) and `max-sum`
#     (every column at its widest at once, which no single row has to reach)
#   then the recommendation: the largest `p95-sum` over the shapes, and whether it COVERS the widest line
#   any part of the population actually carries. A target below today's legitimate widest row warns on
#   arrival, and this tool refuses (exit 1) rather than print such a number as a recommendation.
#
# HONEST LIMITS: a shape's identity is its header text, so renaming a column starts a new shape — deliberate,
# because the budget belongs to the contract the header states. p95 is over the CELLS of one column, not over
# row widths, so `p95-sum` is the sum of per-column steady states and not the p95 of rows; for a small table
# (fewer than 20 rows) the p95 of a column IS its maximum, and the two budgets differ only by which column
# peaks where. Cells are bytes under LC_ALL=C, so accented or CJK text costs what it costs on the wire — the
# same axis the checker measures. Prose is not a shape and contributes only to the population's widest line.
#
# Usage:  bash docs/tasks/artifacts/live_doc_size/run_cell_budget_census.sh
#         bash docs/tasks/artifacts/live_doc_size/run_cell_budget_census.sh --self-test
#         CELL_BUDGET_GLOB='docs/book/src/spec/glossary/*.md' bash docs/tasks/artifacts/live_doc_size/run_cell_budget_census.sh
# Output: per-shape tables, then
#         `cell budget: <shapes> shapes / <rows> data rows measured / recommended maxline health <N> B`
#         exit 1 when the recommendation does not cover the population's widest line, exit 2 on bad input.
set -uo pipefail
export LC_ALL=C

ROOT="${CELL_BUDGET_ROOT:-$(git rev-parse --show-toplevel 2>/dev/null || pwd)}"
cd "$ROOT" || { echo "cell budget: REFUSED — cannot enter $ROOT" >&2; exit 2; }
GLOB="${CELL_BUDGET_GLOB:-docs/book/src/*.md}"

SCRATCH="$(mktemp -d)"
trap 'rm -rf "$SCRATCH"' EXIT
CELLS="$SCRATCH/cells.tsv"      # shape TAB column TAB bytes TAB hdr|data

# ── the walker: bash finds the files, awk emits one line per cell ───────────────────────────
# Cells are split honouring code spans and escaped pipes, the same way scripts/check_table_arity.sh splits
# them, so the two instruments agree about where a cell ends. The arithmetic deliberately lives in the shell
# below: one pass that both walks tables and accumulates statistics is the shape that hides an off-by-one.
walk() { # $1 = file · emits "shape<TAB>col<TAB>bytes<TAB>hdr|data" per cell, then "WIDEST<TAB>bytes<TAB>file"
  awk -v file="$1" '
    function splitrow(line, cells,   buf, i, c, incode, n) {
      n = 0; buf = ""; incode = 0
      for (i = 1; i <= length(line); i++) {
        c = substr(line, i, 1)
        if (c == "\\" && substr(line, i + 1, 1) == "|") { buf = buf "|"; i++; continue }
        if (c == "`") { incode = !incode; buf = buf c; continue }
        if (c == "|" && !incode) { n++; cells[n] = buf; buf = ""; continue }
        buf = buf c
      }
      n++; cells[n] = buf
      if (n >= 1 && cells[1] ~ /^[ \t]*$/) { for (i = 1; i < n; i++) cells[i] = cells[i + 1]; n-- }
      if (n >= 1 && cells[n] ~ /^[ \t]*$/) n--
      return n
    }
    function trim(s) { gsub(/^[ \t]+|[ \t]+$/, "", s); return s }
    function issep(nc, cells,   i) {
      if (nc <= 0) return 0
      for (i = 1; i <= nc; i++) if (trim(cells[i]) !~ /^:?-+:?$/) return 0
      return 1
    }
    function emit(shape, nc, cells, kind,   i) {
      for (i = 1; i <= nc; i++) printf "%s\t%d\t%d\t%s\n", shape, i, length(trim(cells[i])), kind
    }
    BEGIN { state = "prose"; shape = ""; w = 0 }
    {
      n = length($0); if (n > w) w = n
      if ($0 ~ /^[ \t]*\|/) {
        nc = splitrow($0, c)
        if (state == "hdr" && issep(nc, c)) {            # the separator fixes the shape of the table
          shape = ""
          for (i = 1; i <= hnc; i++) shape = shape (i > 1 ? "|" : "") trim(hc[i])
          ncols = hnc
          emit(shape, hnc, hc, "hdr")
          state = "data"
          next
        }
        if (state == "data") { emit(shape, ncols, c, "data"); next }
        for (i = 1; i <= nc; i++) hc[i] = c[i]           # a header candidate: the row before a separator
        hnc = nc; state = "hdr"
        next
      }
      state = "prose"
    }
    END { printf "WIDEST\t%d\t%s\n", w, file }
  ' "$1"
}

if [ "${1:-}" = "--self-test" ]; then
  # A synthetic population whose correct budget is known by construction. One shape, three columns; with the
  # header cells counted, the per-column widest are 5, 6 and 5 bytes, so p95-sum = 16 and the separator
  # overhead is 3*3+1 = 10 -> a 26-byte row budget, which covers the widest line in the file (the 24-byte
  # header row). The tool reads TRACKED files, so the scratch directory is a throwaway repository with the
  # file staged: `git ls-files` lists a staged file, which is why no commit is needed.
  d="$SCRATCH/selftest"; mkdir -p "$d"
  printf '| alpha | beta | gamma |\n| --- | --- | --- |\n| abcd | abcdef | ab |\n| ab | abcd | a |\n\nprose, not a shape\n' > "$d/t.md"
  ( cd "$d" && git init -q . && git add t.md ) >/dev/null 2>&1
  out="$(CELL_BUDGET_ROOT="$d" CELL_BUDGET_GLOB='*.md' bash "$0" 2>&1)"; rc=$?
  fails=0
  arm() { # $1 = name · $2 = expected substring
    if grep -qF -- "$2" <<<"$out"; then printf '  ✓ %-13s %s\n' "$1" "$2"
    else printf '  ✗ %-13s expected the substring "%s" in the output\n' "$1" "$2" >&2; fails=$((fails+1)); fi
  }
  arm SHAPE-SEEN 'alpha|beta|gamma'
  arm COL-WIDEST 'column 2: widest 6 B'
  arm P95-SUM    'p95-sum 16 B'
  arm OVERHEAD   'separator overhead 10 B'
  arm RECOMMEND  'recommended maxline health 26 B'
  arm COVERS     'widest actual line (24 B): yes'
  arm EXIT-OK    'data rows measured / recommended maxline health 26 B'
  [ "$rc" -eq 0 ] || { printf '  ✗ EXIT-OK      exit=%s, expected 0\n' "$rc" >&2; fails=$((fails+1)); }
  printf 'probes: %d pass / %d fail\n' $((7 - fails)) "$fails"
  [ "$fails" -eq 0 ] || exit 1
  exit 0
fi

# A portable file list rather than `mapfile`: these instruments run on whatever bash a clone has, and macOS
# ships 3.2, which has no `mapfile`.
FILE_LIST="$SCRATCH/files.txt"
git ls-files -- "$GLOB" > "$FILE_LIST"
nfiles=$(grep -c . "$FILE_LIST" | tr -d ' ')
[ "${nfiles:-0}" -gt 0 ] || { echo "cell budget: REFUSED — the glob \`$GLOB\` matches no tracked file" >&2; exit 2; }

: > "$CELLS"; widest=0; wfile=""
while IFS= read -r f; do
  [ -n "$f" ] || continue
  while IFS=$'\t' read -r a b c d; do
    if [ "$a" = "WIDEST" ]; then
      if [ "$b" -gt "$widest" ]; then widest="$b"; wfile="$c"; fi
    else
      printf '%s\t%s\t%s\t%s\n' "$a" "$b" "$c" "$d" >> "$CELLS"
    fi
  done < <(walk "$f")
done < "$FILE_LIST"

datarows=$(awk -F'\t' '$4 == "data" && $2 == 1 { n++ } END { print n+0 }' "$CELLS")
shapes=$(cut -f1 "$CELLS" | LC_ALL=C sort -u | grep -c . | tr -d ' ')

echo "cell-budget census — population: $GLOB ($nfiles files)"
echo "-- the population's widest line, of any shape"
echo "  $widest B in $wfile"

# ── per shape: columns, their widest and 95th-percentile cells, and the two derived budgets ──
recommend=0
while IFS= read -r shape; do
  [ -n "$shape" ] || continue
  ncols=$(awk -F'\t' -v s="$shape" '$1 == s { if ($2 + 0 > n) n = $2 + 0 } END { print n+0 }' "$CELLS")
  nrows=$(awk -F'\t' -v s="$shape" '$1 == s && $2 == 1 && $4 == "data" { n++ } END { print n+0 }' "$CELLS")
  overhead=$((3 * ncols + 1))
  printf -- '-- shape: %s\n' "$shape"
  printf '   %d data rows · %d columns · separator overhead %d B\n' "$nrows" "$ncols" "$overhead"
  p95sum=0; maxsum=0
  for col in $(seq 1 "$ncols"); do
    vals="$SCRATCH/col.txt"
    awk -F'\t' -v s="$shape" -v c="$col" '$1 == s && $2 == c { print $3 }' "$CELLS" | LC_ALL=C sort -n > "$vals"
    n=$(grep -c . "$vals" | tr -d ' ')
    [ "${n:-0}" -gt 0 ] || continue
    mx=$(tail -1 "$vals")
    idx=$(( (95 * n + 99) / 100 )); [ "$idx" -lt 1 ] && idx=1; [ "$idx" -gt "$n" ] && idx=$n
    p95=$(sed -n "${idx}p" "$vals")
    printf '   column %d: widest %s B · p95 %s B\n' "$col" "${mx:-0}" "${p95:-0}"
    p95sum=$((p95sum + ${p95:-0})); maxsum=$((maxsum + ${mx:-0}))
  done
  printf '   derived row budget: p95-sum %d B (+%d overhead = %d B) · max-sum %d B (+%d = %d B)\n' \
    "$p95sum" "$overhead" "$((p95sum + overhead))" "$maxsum" "$overhead" "$((maxsum + overhead))"
  cand=$((p95sum + overhead))
  [ "$cand" -gt "$recommend" ] && recommend="$cand"
done < <(cut -f1 "$CELLS" | LC_ALL=C sort -u)

# ── the recommendation, and the check that keeps it honest ───────────────────────────────────
covers="yes"
[ "$recommend" -lt "$widest" ] && covers="NO"
echo "-- recommendation"
echo "  the largest p95-sum over all shapes is $recommend B"
echo "  covers the population's widest actual line ($widest B): $covers"
if [ "$covers" = "NO" ]; then
  { echo "  x a health target below the widest legitimate row warns on arrival. Raise the per-column budget"
    echo "    or take the max-sum of the shape carrying that row — do not shorten the row to fit a guess."
  } >&2
fi

echo "cell budget: $shapes shapes / $datarows data rows measured / recommended maxline health $recommend B"
[ "$covers" = "yes" ] || exit 1
exit 0
