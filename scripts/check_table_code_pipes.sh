#!/usr/bin/env bash
# scripts/check_table_code_pipes.sh — TABLE-CODE-PIPE (PROJECT doctrine).
#
# THE RULE: in a staged `.md` table row, a pipe inside a code span must be escaped (`\|`). A raw one is
# refused, with the file, the line and the cell that carries it.
#
# ⭐ WHY THIS EXISTS (measured against a rendered page, not argued from a specification — defect D22, settled
#   by `SPINE.15`, oracle `docs/tasks/artifacts/table_render/run_table_render_probes.sh`): a raw pipe between
#   backticks is STILL a cell separator. A three-column row whose first cell held a code span containing a raw
#   pipe rendered as three cells — the first truncated at the pipe, the code span broken open, and the row's
#   rightmost cell **dropped silently**. The source still reads correctly, every gate stays green, and the
#   reader of the built book loses a column of a contract. That is the failure `TABLE-ARITY-RATCHET` was ported
#   to catch, and the
#   inherited `scripts/check_table_arity.sh` cannot catch this instance: it documents its cell rule as "pipes
#   NOT inside an inline code span" and self-tests exactly that, so it treats a code span as protective.
#
# ⛔ THE INHERITED CHECKER IS NEUTRAL SPINE CODE and is therefore NOT patched here — it is re-synced by
#   `scripts/update_scaffold.sh`, and a local edit would be silently overwritten or, worse, silently diverge.
#   This check is the project-slot answer (defect D47), and the divergence is reported upstream.
#
# SCOPE, stated because over-reach trains bypass:
#   * table rows only — a line whose first non-space character is `|`. A pipe in a code span in ordinary prose
#     harms nothing: there is no cell to split.
#   * outside fenced code blocks — a block is quoted content, and quoting a broken row is how this defect gets
#     documented (this file's own header does it).
#   * staged files only, like every other doctrine check here: fast, pre-commit friendly, and the tree is
#     judged by the unconditional checks instead.
#
# HONEST LIMITS: this proves the escape is present, never that the resulting cell MEANS the right thing, and
# it reads one renderer's behaviour (mdBook/pulldown-cmark) as the ground truth — GitHub's current renderer, a
# factory PLM viewer and a printed page were not tested. Escaping is safe under all of them, including any that
# would have protected the span, which is why the rule does not wait for a second oracle.
#
# MODES: no argument judges the STAGED set (the doctrine, hook- and CI-fast). `--all` reports the whole
# tracked tree instead, advisory, so the claim "no existing file violates this" is re-derivable by one command
# rather than being a memory of the slice that measured it — the same reason `check_gap_claims.sh` carries
# `--all`. `--self-test` proves the arms fire.
#
# CONTRACT (DOCTRINE_ENFORCEMENT.md): exit code is the verdict; explains on stderr; deterministic; read-only;
# staged-scope-aware; path-agnostic; fast.
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT" || exit 1

PY_SRC=$(cat <<'PY'
import re, sys, pathlib
# A code span is a run of one or more backticks, closed by a run of the same length (CommonMark). Inside it,
# a pipe that is not preceded by a backslash splits the cell when the line is a table row.
SPAN = re.compile(r"(`+)(.+?)\1", re.S)
RAW = re.compile(r"(?<!\\)\|")
bad = 0
for path in sys.argv[1:]:
    p = pathlib.Path(path)
    if p.suffix != ".md" or not p.is_file():
        continue
    fenced = False
    for n, line in enumerate(p.read_text(encoding="utf-8").splitlines(), 1):
        if line.lstrip().startswith("```"):
            fenced = not fenced
            continue
        if fenced or not line.lstrip().startswith("|"):
            continue
        for m in SPAN.finditer(line):
            if RAW.search(m.group(2)):
                print("TABLE-CODE-PIPE: %s:%d — a raw `|` inside a code span splits the cell and the "
                      "renderer drops the rightmost one" % (path, n), file=sys.stderr)
                print("    %s" % line.strip()[:160], file=sys.stderr)
                print("    write `%s` instead: escape every pipe inside a table cell (COMMIT.md, "
                      "Table authoring)" % m.group(0).replace("|", "\\|"), file=sys.stderr)
                bad += 1
sys.exit(1 if bad else 0)
PY
)

if [ "${1:-}" = "--self-test" ]; then
  d="target/doctrine_scratch/table_code_pipes"
  rm -rf "$d"; mkdir -p "$d"
  fails=0; arms=0
  t() { # $1 label · $2 expected exit · $3 markdown content
    arms=$((arms + 1))
    printf '%b' "$3" > "$d/case.md"
    if python3 -c "$PY_SRC" "$d/case.md" >/dev/null 2>&1; then got=0; else got=1; fi
    if [ "$got" = "$2" ]; then printf '  ✓ %-16s exit=%s  %s\n' "$1" "$got" "$4"
    else printf '  ✗ %-16s exit=%s (wanted %s)  %s\n' "$1" "$got" "$2" "$4" >&2; fails=$((fails + 1)); fi
  }
  H='| a | b | c |\n|---|---|---|\n'
  t RAW-PIPE      1 "${H}| \`x | y\` | 2 | 3 |\n"        "a raw pipe in a code span is refused"
  t ESCAPED       0 "${H}| \`x \\| y\` | 2 | 3 |\n"      "an escaped pipe is accepted"
  t REAL-SEP      0 "${H}| x | y | z |\n"                "an ordinary separator is none of this check's business"
  t FENCED        0 "| a |\n|---|\n\n\`\`\`\n| \`x | y\` | 2 |\n\`\`\`\n"  "a quoted row inside a fence is documentation, not a table"
  t DOUBLE-TICK   1 "${H}| \`\`x | y\`\` | 2 | 3 |\n"    "a double-backtick span is a span too"
  t PROSE         0 "A sentence with \`x | y\` in it.\n" "prose has no cell to split"
  t INDENTED      1 "${H}  | \`x | y\` | 2 | 3 |\n"      "an indented table row is still a table row"
  rm -rf "$d"
  printf 'table-code-pipe --self-test: %d arms, %d failed\n' "$arms" "$fails"
  [ "$fails" -eq 0 ] || exit 1
  exit 0
fi

if [ "${1:-}" = "--all" ]; then
  # advisory over the whole tracked tree: the backlog, if any, is printed and the exit code still reports it
  files="$(git ls-files '*.md')"
  [ -n "$files" ] || { echo "TABLE-CODE-PIPE --all: no tracked .md files" >&2; exit 2; }
  if printf '%s\n' "$files" | xargs python3 -c "$PY_SRC"; then
    echo "table-code-pipe --all: $(printf '%s\n' "$files" | grep -c .) tracked .md files, 0 offending table rows"
    exit 0
  fi
  echo "table-code-pipe --all: offending rows above; the staged gate refuses them the moment they are touched" >&2
  exit 1
fi

staged="$(git diff --cached --name-only --diff-filter=ACM 2>/dev/null || true)"
[ -n "$staged" ] || exit 0
md="$(printf '%s\n' "$staged" | grep -E '\.md$' || true)"
[ -n "$md" ] || exit 0

# shellcheck disable=SC2086  # the file list is newline-separated paths from git, word-split on purpose
if printf '%s\n' "$md" | xargs python3 -c "$PY_SRC"; then
  exit 0
fi
echo "  A renderer splits a table cell at a raw pipe even inside a code span, and drops the rightmost cell" >&2
echo "  without a diagnostic. Escape the pipe, or move the content out of the table." >&2
exit 1
