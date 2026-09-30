#!/usr/bin/env bash
# docs/tasks/artifacts/table_render/run_table_render_probes.sh
# SPINE.15 — the rendered-page oracle behind the table-authoring convention (defect D22).
#
# WHY A RENDERER AND NOT A SPEC: D22 asked whether a GFM renderer splits a table cell on a raw `|` inside a
# CODE SPAN. The inherited `scripts/check_table_arity.sh` assumes it does not ("a pipe inside a code span is
# not a separator" is one of its own self-test arms), and `DOCTRINE_ENFORCEMENT.md` records the consequence it
# is guarding against — "GFM silently DROPS extra cells and PADS missing ones". Reading a specification cannot
# settle which of the two is right, so this suite renders a page and counts the cells the renderer emitted.
# The oracle is mdBook's HTML backend (pulldown-cmark with GFM tables), which is the renderer this
# repository's own book ships through; a second renderer would strengthen it and is recorded as a missing leg
# in the leaf rather than implied.
#
#   RAW-PIPE    a raw `|` inside a code span SPLITS the cell: the backticks do not protect it, the row's
#               cells shift, and the rightmost cell is silently dropped — content lost with no diagnostic
#   ESCAPED     `\|` inside the same code span keeps the row intact and renders a literal pipe
#   DIVERGES    the inherited arity checker's own self-test asserts the opposite (a code-span pipe is not a
#               separator), which is the divergence D22 names: the checker under-reports what the renderer
#               does, so the convention must be escape-always and cannot rely on the gate
#
# HONEST LIMITS: this proves one renderer's behaviour, on this machine's mdbook. It cannot prove what a
# factory's PLM viewer, GitHub's current renderer or a printed page do; the convention it supports
# (escape every pipe in a table cell) is the safe one under all of them, which is why the convention does not
# wait for a second oracle. Requires `mdbook`; without it the suite REFUSES (exit 2) rather than reporting
# green over a page nobody rendered.
#
# Usage:  bash docs/tasks/artifacts/table_render/run_table_render_probes.sh
# Output: per-arm lines plus `probes: N pass / M fail` (exit nonzero if M > 0).
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
ARITY="$ROOT/scripts/check_table_arity.sh"
command -v mdbook >/dev/null 2>&1 || { echo "probe: REFUSED — mdbook is not installed, so no page can be rendered" >&2; exit 2; }
[ -f "$ARITY" ] || { echo "probe: REFUSED — $ARITY not found" >&2; exit 2; }

WORK="${TMPDIR:-$ROOT/target/scratch}/table_render_probes"
rm -rf "$WORK"; mkdir -p "$WORK/src"; trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { printf '  ✓ %-11s %s\n' "$1" "$2"; pass=$((pass+1)); }
bad() { printf '  ✗ %-11s %s\n' "$1" "$2"; fail=$((fail+1))
        [ -n "${3:-}" ] && printf '%s\n' "$3" | head -6 | sed 's/^/            /'; }

printf '[book]\ntitle = "table-render oracle"\n' > "$WORK/book.toml"
printf '# Summary\n\n- [Probe](probe.md)\n' > "$WORK/src/SUMMARY.md"
cat > "$WORK/src/probe.md" <<'PAGE'
# Probe

| Case | Second | Third |
| --- | --- | --- |
| A raw pipe in a code span: `x | y` | 2 | 3 |
| B escaped pipe in a code span: `x \| y` | 2 | 3 |
PAGE
mdbook build "$WORK" >/dev/null 2>&1 || { echo "probe: REFUSED — mdbook build failed" >&2; exit 2; }

# the rendered rows, as "cellcount<TAB>cell1<TAB>cell2<TAB>..."
python3 - "$WORK/book/probe.html" > "$WORK/rows.tsv" <<'PY'
import re, sys, html
h = open(sys.argv[1], encoding="utf-8").read()
body = h[h.index("<table>"):]
for row in re.findall(r"<tr>(.*?)</tr>", body, re.S):
    cells = re.findall(r"<t[dh][^>]*>(.*?)</t[dh]>", row, re.S)
    text = [html.unescape(re.sub(r"<[^>]+>", "", c)).strip() for c in cells]
    print("%d\t%s" % (len(cells), "\t".join(text)))
PY

echo "table-render probes — the rendered page, not a reading of the specification"
while IFS=$'\t' read -r n rest; do printf '  · %s cell(s): %s\n' "$n" "$(printf '%s' "$rest" | tr '\t' '|')"; done < "$WORK/rows.tsv"

# ---------------------------------------------------------------- RAW-PIPE
# The assertion is the PROPERTY, not a cell count: the first cell is truncated at the pipe (the code span is
# broken open) and the row's rightmost cell is gone. Measured both ways, because the first cut of this arm
# asserted "2 cells" from a 2-column page and a 3-column page split into 3 cells with the last one dropped -
# the same defect, a different shape, and an arm that would have gone red on the truth.
raw="$(awk -F'\t' '$2 ~ /^A raw pipe/ { print; exit }' "$WORK/rows.tsv")"
first="$(cut -f2 <<<"$raw")"; last="$(awk -F'\t' '{print $NF}' <<<"$raw")"
if [ "$first" = 'A raw pipe in a code span: `x' ] && [ "$last" != "3" ]; then
  ok RAW-PIPE "a raw pipe inside a code span split the cell at the pipe and the rightmost cell (3) was dropped, leaving: $last"
else
  bad RAW-PIPE "the renderer did not split on a code-span pipe, so D22's premise is wrong: [$raw]" "$(cat "$WORK/rows.tsv")"
fi

# ---------------------------------------------------------------- ESCAPED
esc="$(awk -F'\t' '$2 ~ /^B escaped pipe/ { print; exit }' "$WORK/rows.tsv")"
if [ "$(cut -f1 <<<"$esc")" = "3" ] && grep -q 'x | y' <<<"$esc"; then
  ok ESCAPED "an escaped pipe kept all $(cut -f1 <<<"$esc") cells and rendered a literal pipe"
else
  bad ESCAPED "escaping did not preserve the row: [$esc]" "$(cat "$WORK/rows.tsv")"
fi

# ---------------------------------------------------------------- DIVERGES
out="$(bash "$ARITY" --self-test 2>&1)"; rc=$?
if grep -q 'arm ok  a pipe inside a code span is not a separator' <<<"$out"; then
  ok DIVERGES "the inherited arity checker still asserts a code-span pipe is not a separator (its self-test, exit=$rc)"
else
  bad DIVERGES "the inherited checker's behaviour changed, so this divergence claim must be re-measured (exit=$rc)" "$out"
fi

echo
printf 'probes: %d pass / %d fail\n' "$pass" "$fail"
[ "$fail" -eq 0 ] || exit 1
exit 0
