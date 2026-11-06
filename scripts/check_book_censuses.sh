#!/usr/bin/env bash
# scripts/check_book_censuses.sh — BOOK-CENSUSES (PROJECT doctrine).
#
# THE RULE: at every commit the book must satisfy its glossary census (every machine token a chapter uses
# is declared), its formula-language census, and two structural rules: every registered chapter has a topic
# index row, and no chapter links outside the book's source tree.
#
# WHY THIS EXISTS (defects D384/D386, measured): these rules ran only inside `make probes`, which runs once per
# 400 commits. The glossary census went red between 0655 and 0700 and reached 794 undeclared token/chapter
# pairs before anyone saw it; two chapters lacked index rows from 0871/0874; and 0968 and 0971 each landed a
# book fault while every commit gate stayed green. A rule that only runs before a push reports months of drift
# in one sitting, so the cheap real-tree verdicts run here instead.
#
# SCOPE: the working tree, because that is what the censuses read, and CI runs the same script over a clean
# checkout. The censuses' own fault arms stay in their probe suites; this runs only their real-tree verdicts.
#
# HONEST LIMITS: this cannot tell whether a declared meaning is CORRECT. It does not render the book, so the
# publication suite's rendered checks (malformed HTML, anchors, API status) remain pre-push checks.
#
# Usage:  bash scripts/check_book_censuses.sh               the real tree
#         bash scripts/check_book_censuses.sh --self-test   proves each arm fires and the real tree passes
#
# CONTRACT (DOCTRINE_ENFORCEMENT.md): exit code is the verdict; explains on stderr; deterministic; read-only
# outside --self-test scratch; path-agnostic; fast.
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT" || exit 1
GLOSSARY="$ROOT/docs/tasks/artifacts/glossary/run_glossary_census.sh"
FORMULA="$ROOT/docs/tasks/artifacts/formula_language/run_formula_language_census.sh"

STRUCTURE_SRC=$(cat <<'PY'
import os, re, sys
source = sys.argv[1]
link = re.compile(r'\]\(([^)\s#]+)(?:#[^)\s]*)?\)')
def targets(text):
    # Fenced examples are quoted content, and inline code spans are not links.
    text = re.sub(r'^```.*?^```[^\n]*$', '', text, flags=re.M | re.S)
    return link.findall(re.sub(r'`[^`\n]*`', '', text))
summary = open(os.path.join(source, 'SUMMARY.md'), encoding='utf-8').read()
registered = [t for t in targets(summary) if t.endswith('.md')]
index_path = os.path.join(source, 'topic-index.md')
indexed = set(targets(open(index_path, encoding='utf-8').read()))
bad = 0
for chapter in registered:
    if chapter != 'topic-index.md' and chapter not in indexed:
        print('BOOK-CENSUSES: %s has no topic-index.md row' % chapter, file=sys.stderr)
        bad += 1
root = os.path.realpath(source)
for folder, _, names in os.walk(source):
    for name in sorted(names):
        if not name.endswith('.md'):
            continue
        path = os.path.join(folder, name)
        for target in targets(open(path, encoding='utf-8').read()):
            if re.match(r'[a-z][a-z0-9+.-]*:', target):
                continue
            resolved = os.path.realpath(os.path.join(folder, target))
            if resolved != root and not resolved.startswith(root + os.sep):
                print('BOOK-CENSUSES: %s links outside the book source: %s' % (os.path.relpath(path, source), target),
                      file=sys.stderr)
                bad += 1
sys.exit(1 if bad else 0)
PY
)

# $1 = root holding docs/book/src, ROADMAP.md and docs/tasks. Prints the verdict; returns nonzero on a breach.
verdict() {
  local root="$1" out rc=0
  if ! out="$(GLOSSARY_ROOT="$root" LC_ALL=C bash "$GLOSSARY" 2>&1)"; then
    printf '%s\n' "$out" | grep -E '^ +x |REFUSED|^glossary census' | head -40 >&2
    printf 'BOOK-CENSUSES: declare each name above in a table of the chapter that uses it\n' >&2
    rc=1
  fi
  if ! out="$(FORMULA_BOOK="$root/docs/book/src" bash "$FORMULA" 2>&1)"; then
    printf '%s\n' "$out" | grep -E '^ +x |REFUSED|^formula-language census' | head -40 >&2
    rc=1
  fi
  python3 -I -B -c "$STRUCTURE_SRC" "$root/docs/book/src" || rc=1
  return "$rc"
}

if [ "${1:-}" = "--self-test" ]; then
  d="target/doctrine_scratch/book_censuses"
  fails=0; arms=0
  fresh() {
    rm -rf "$d"; mkdir -p "$d"
    git archive HEAD docs/book/src ROADMAP.md docs/tasks | tar -x --exclude='docs/tasks/artifacts/*' -C "$d"
    # The working tree's book is the population under test, not the last commit's.
    cp -R docs/book/src/. "$d/docs/book/src/"
  }
  arm() { # $1 name · $2 wanted status · $3 text the output must name
    local got=0 out
    arms=$((arms + 1))
    out="$(verdict "$PWD/$d" 2>&1)" || got=$?
    if { [ "$2" = 0 ] && [ "$got" = 0 ]; } || { [ "$2" != 0 ] && [ "$got" != 0 ] && grep -qF -- "$3" <<<"$out"; }; then
      printf '  ok %-14s status=%s\n' "$1" "$got"
    else
      printf '  BAD %-13s status=%s wanted=%s\n%s\n' "$1" "$got" "$2" "$out" | head -12
      fails=$((fails + 1))
    fi
  }
  fresh; arm real 0 ''
  fresh; printf '\nUses `UndeclaredProbeToken` here.\n' >> "$d/docs/book/src/spec/units-and-tolerances.md"
  arm undeclared 1 'UndeclaredProbeToken'
  fresh; printf '\nThe formula `(left + right)?` is not grammar.\n' >> "$d/docs/book/src/spec/units-and-tolerances.md"
  arm formula 1 'L6b'
  fresh; grep -v 'annexes/semantic-recipes.md' "$d/docs/book/src/topic-index.md" > "$d/index.tmp"
  mv "$d/index.tmp" "$d/docs/book/src/topic-index.md"
  arm index 1 'annexes/semantic-recipes.md has no topic-index.md row'
  fresh; printf '\nSee [the record](../../../decisions/x.md).\n' >> "$d/docs/book/src/annexes/logical-inputs.md"
  arm outside 1 'links outside the book source'
  fresh; printf '\nA code span `[x](../../../y.md)` and a fence are not links.\n' >> "$d/docs/book/src/annexes/logical-inputs.md"
  arm code-span 0 ''
  rm -rf "$d"
  printf 'book-censuses --self-test: %d arms, %d failed\n' "$arms" "$fails"
  [ "$fails" -eq 0 ]; exit
fi

verdict "$ROOT"
