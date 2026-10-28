#!/usr/bin/env bash
# docs/tasks/artifacts/glossary/run_glossary_probes.sh
# G0-CONTRACT.1 — end-to-end probes for the glossary census (run_glossary_census.sh).
#
# WHY A SUITE AND NOT A GREEN RUN: a census that has only ever passed on the tree it was written
# against has never been seen to NOTICE anything. Each RED arm below copies the real inputs into a
# synthetic root, breaks exactly one property, and requires the census to name that property. The real
# tree is never mutated.
#
#   REAL-1          the census passes on this repository as it stands
#   REAL-2          `--emit-index` reproduces the tracked A–Z index byte for byte, so the index cannot
#                   drift from the parts it is derived from
#   DUP-TOKEN       two entries owning one token is refused (T2 — one token, one meaning)
#   DANGLING-REF    a `→` cross-reference to a token nothing owns is refused (T3)
#   DEAD-CLAUSE     a canonical-object cell citing a clause the chapter does not have is refused (R1)
#   SAFETY-LOST     a safety-relevant term that lost its ⚠ is refused (R2)
#   INDEX-DRIFT     a term defined in a part but missing from the A–Z index is refused (I1)
#   UNDECLARED      a machine token used by a chapter and declared nowhere is refused (C1)
#   ARITY           an entry row with the wrong cell count is refused (S1)
#   MISSING         a root with no glossary REFUSES (exit 2) rather than reporting green over nothing
#
# Usage:  bash docs/tasks/artifacts/glossary/run_glossary_probes.sh
# Output: per-arm lines plus `probes: N pass / M fail` (exit nonzero if M > 0).
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
CENSUS="$ROOT/docs/tasks/artifacts/glossary/run_glossary_census.sh"
[ -f "$CENSUS" ] || { echo "probe: REFUSED — $CENSUS not found" >&2; exit 2; }

WORK="${TMPDIR:-$ROOT/target/scratch}/glossary_probes"
rm -rf "$WORK"; mkdir -p "$WORK"; trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { printf '  ✓ %-14s %s\n' "$1" "$2"; pass=$((pass+1)); }
bad() { printf '  ✗ %-14s %s\n' "$1" "$2"; fail=$((fail+1))
        [ -n "${3:-}" ] && printf '%s\n' "$3" | grep -E '^  x |census:|REFUSED' | head -8 | sed 's/^/          /'; }

# a synthetic root carrying the four inputs the census reads: the book, the roadmap, the trees
mkroot() {
  local d="$1"; rm -rf "$d"; mkdir -p "$d"
  ( cd "$ROOT" && tar cf - docs/book/src ROADMAP.md docs/tasks/*.md 2>/dev/null ) | ( cd "$d" && tar xf - )
}
# run the census against a synthetic root; echo its output, return its exit status
run() { local d="$1"; GLOSSARY_ROOT="$d" bash "$CENSUS" 2>&1; }
# portable in-place edit: sed to a temp file, then move (BSD and GNU sed disagree about -i)
sedfile() { local f="$1"; shift; sed "$@" "$f" > "$f.tmp" && mv "$f.tmp" "$f"; }

echo "glossary probes — docs/tasks/artifacts/glossary/run_glossary_census.sh"

# ---------------------------------------------------------------- REAL-1: this tree, as it stands
out="$(run "$ROOT")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q '/ 0 failure(s)' <<<"$out"; then
  ok REAL-1 "$(grep -o 'glossary census: .*' <<<"$out")"
else
  bad REAL-1 "the real glossary does not satisfy its own census (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- REAL-2: the index is derived, not typed
D="$WORK/index"; mkroot "$D"
tracked="$(awk '/^## /{s=0} /^## The terms, A–Z/{s=1; next} s && /^- \[/' "$D/docs/book/src/spec/glossary.md")"
emitted="$(GLOSSARY_ROOT="$D" bash "$CENSUS" --emit-index)"
if [ -n "$emitted" ] && [ "$tracked" = "$emitted" ]; then
  ok REAL-2 "--emit-index reproduces the tracked A–Z index byte for byte ($(grep -c . <<<"$emitted") entries)"
else
  bad REAL-2 "the tracked index and the derived index differ, so one of them is hand-carried" \
      "$(diff <(printf '%s\n' "$tracked") <(printf '%s\n' "$emitted") | head -10)"
fi

# ---------------------------------------------------------------- DUP-TOKEN (T2)
D="$WORK/dup"; mkroot "$D"
sedfile "$D/docs/book/src/spec/glossary/measurements-and-fit.md" 's/| `base_size` |/| `SizeSet` |/'
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'token `SizeSet` is owned twice' <<<"$out"; then
  ok DUP-TOKEN "a second entry claiming \`SizeSet\` is refused and named (exit=$rc)"
else
  bad DUP-TOKEN "two entries owning one token was not refused (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- DANGLING-REF (T3)
D="$WORK/dangling"; mkroot "$D"
sedfile "$D/docs/book/src/spec/glossary/measurements-and-fit.md" 's/→ `Ease`/→ `Nosuchtoken`/'
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'token `Nosuchtoken` is cross-referenced' <<<"$out"; then
  ok DANGLING-REF "a cross-reference to an unowned token is refused and named (exit=$rc)"
else
  bad DANGLING-REF "a dangling \`→\` cross-reference was not refused (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- DEAD-CLAUSE (R1)
D="$WORK/clause"; mkroot "$D"
sedfile "$D/docs/book/src/spec/glossary/measurements-and-fit.md" 's/\[ontology §2\.2\]/[ontology §99.9]/'
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'cites §99.9, which .* has no heading for' <<<"$out"; then
  ok DEAD-CLAUSE "a citation to a clause the chapter does not have is refused (exit=$rc)"
else
  bad DEAD-CLAUSE "an invented clause number was not refused (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- SAFETY-LOST (R2)
D="$WORK/safety"; mkroot "$D"
sedfile "$D/docs/book/src/spec/glossary/recipe-and-pieces.md" 's/^| net line ⚠ |/| net line |/'
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q '`net line` is safety-relevant' <<<"$out"; then
  ok SAFETY-LOST "removing the ⚠ from \`net line\` is refused, with the roadmap clause that requires it (exit=$rc)"
else
  bad SAFETY-LOST "a safety-relevant term lost its mark and the census did not notice (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- INDEX-DRIFT (I1)
D="$WORK/indexdrift"; mkroot "$D"
sedfile "$D/docs/book/src/spec/glossary.md" '/^- \[net line\]/d'
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'missing from the A–Z index: net line' <<<"$out"; then
  ok INDEX-DRIFT "a term dropped from the A–Z index is refused by name (exit=$rc)"
else
  bad INDEX-DRIFT "index drift was not refused (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- UNDECLARED (C1)
D="$WORK/undeclared"; mkroot "$D"
printf '\nA chapter may mention `mystery_token` in prose.\n' >> "$D/docs/book/src/spec/ontology.md"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q '`mystery_token` is used by' <<<"$out"; then
  ok UNDECLARED "a token a chapter uses and nothing declares is refused by name (exit=$rc)"
else
  bad UNDECLARED "an undeclared machine token was not refused (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- D155/D218/D250 actual declarations (C1)
# Remove each real declaration from a fresh book copy; the standing census must name it.
while IFS=$'\t' read -r family token chapter; do
  D="$WORK/$family-$token"; mkroot "$D"
  # Every fault retains an independent use. Definition-only names otherwise lose the obligation
  # together with their row. The green copied baseline proves the retained use is declared first.
  printf '\nDeclaration usage control: `%s`.\n' "$token" >> "$D/$chapter"
  out="$(run "$D")"; baseline_rc=$?
  expected_pairs="$(python3 -I -B - "$D/$chapter" "$token" "$family" "$D" <<'PY_DECLARATION'
from pathlib import Path
import re
import sys
p, token, family, root = Path(sys.argv[1]), sys.argv[2], sys.argv[3], Path(sys.argv[4])
rows = p.read_text().splitlines(keepends=True)
removed = [row for row in rows if row.startswith('| `' + token + '` |')]
assert len(removed) == 1, ('declaration fault anchor', family, token, len(removed))
p.write_text(''.join(row for row in rows if row not in removed))
# Independent Python oracle for the census's two documented code-span shape arms. Count token/use
# chapter pairs, not token identities. Other chapters may legitimately use this same declaration.
book = root / 'docs/book/src'
users = set()
for chapter in book.rglob('*.md'):
    if chapter == book / 'spec/glossary.md' or (book / 'spec/glossary') in chapter.parents:
        continue
    for line in chapter.read_text().splitlines():
        for span in re.findall(r'`([^`]+)`', line):
            if span == token:
                users.add(chapter)
            elif '-' not in span and '.' not in span and any(op in span for op in '+=−×√÷'):
                for identifier in re.findall(r'[A-Za-z_][A-Za-z0-9_]*', span):
                    if identifier == token and (re.fullmatch(r'[a-z][a-z0-9]*(_[a-z0-9]+)+', identifier)
                                               or re.fullmatch(r'[A-Z][a-z0-9]+([A-Z][A-Za-z0-9]*)*', identifier)):
                        users.add(chapter)
assert users, ('retained use missing', family, token)
print(len(users))
PY_DECLARATION
)"; mutation_rc=$?
  out="$(run "$D")"; rc=$?
  needle=$(printf '  x `%s` is used by' "$token")
  named_pairs=$(grep -Fc "$needle" <<<"$out")
  all_failures=$(grep -c '^  x ' <<<"$out")
  if [ "$baseline_rc" -eq 0 ] && [ "$mutation_rc" -eq 0 ] && [ "$rc" -eq 1 ] \
     && [ "$named_pairs" = "$expected_pairs" ] && [ "$all_failures" = "$expected_pairs" ] \
     && grep -Eq "unaccounted: $expected_pairs$" <<<"$out"; then
    ok "$family-$token" "actual declaration removal refused in exactly $expected_pairs independently derived using chapter(s)"
  else
    bad "$family-$token" "actual declaration fault failed its named census assertion (mutation=$mutation_rc, exit=$rc)" "$out"
  fi
done <<'DECLARATIONS'
D155	name	docs/book/src/spec/formula-language.md
D155	origins	docs/book/src/spec/formula-language.md
D155	prior_source	docs/book/src/spec/formula-language.md
D155	attempted_source	docs/book/src/spec/formula-language.md
D155	declaration_index	docs/book/src/spec/formula-language.md
D155	collision	docs/book/src/annexes/formula-static-validation.md
D155	eps_chord	docs/book/src/annexes/formula-static-validation.md
D218	AssertionError	docs/book/src/annexes/ledger-fixtures.md
D218	HISTORY_ARCHIVE_ROOT	docs/book/src/annexes/ledger-fixtures.md
D218	Refusal	docs/book/src/annexes/formula-static-signatures.md
D218	enter_producer	docs/book/src/build-and-checks.md
D218	mutable_sources	docs/book/src/build-and-checks.md
D218	output_files	docs/book/src/build-and-checks.md
D218	plan_producer	docs/book/src/build-and-checks.md
D218	prepare_producer	docs/book/src/build-and-checks.md
D218	python3	docs/book/src/annexes/ledger-fixtures.md
D218	sources	docs/book/src/build-and-checks.md
D250	CommandRequest	docs/book/src/spec/command-layer.md
D250	CommandScope	docs/book/src/spec/command-scopes.md
D250	ArtifactPayload	docs/book/src/spec/artifact-requests.md
D250	SizeMemberTarget	docs/book/src/spec/evaluation-requests.md
D250	BlankAlgorithm	docs/book/src/spec/release-contract.md
D250	GarmentSizeSetInputs	docs/book/src/spec/size-sets.md
D250	SizeSetResolutionIntent	docs/book/src/spec/size-set-transformations.md
D250	FormulaApproximationSources	docs/book/src/annexes/formula-runtime-validation.md
D250	FormulaBoundValue	docs/book/src/annexes/formula-rationals.md
D250	FormulaCheckedExpression	docs/book/src/annexes/formula-checked-nodes.md
D250	bb	docs/book/src/spec/model-resource-accounting.md
D250	e	docs/book/src/spec/source-resource-accounting.md
DECLARATIONS

# ---------------------------------------------------------------- A1: bounded sample, complete counts, quiet signals
D="$WORK/advisory"; mkroot "$D"
python3 -I -B - "$D/docs/book/src/spec/ontology.md" <<'ADVISORY_INPUT'
from pathlib import Path
import sys
with Path(sys.argv[1]).open('a') as stream:
    for index in range(4096):
        stream.write(f'\n**0000 advisory emphasis {index:04d} ' + 'x' * 96 + '**\n')
ADVISORY_INPUT
advisory_input_rc=$?
for disposition in default ignored; do
  detail="$(python3 -I -B - "$CENSUS" "$D" "$disposition" "$advisory_input_rc" <<'ADVISORY_CHECK'
from pathlib import Path
import os
import re
import string
import subprocess
import sys
census, root, disposition, input_rc = sys.argv[1:]
command = ['bash', census] if disposition == 'default' else [
    'bash', '-c', 'trap "" PIPE; bash "$1"', 'advisory-signal-control', census]
run = subprocess.run(command, env={**os.environ, 'GLOSSARY_ROOT': root}, capture_output=True, text=True)
samples = [line for line in run.stdout.splitlines() if line.startswith('    · ')]
# Independent full-population oracle: retain existing emphasis that sorts before the added spans.
book = Path(root) / 'docs/book/src'
fold_ascii = str.maketrans(string.ascii_uppercase, string.ascii_lowercase)
terms = set()
for part in (book / 'spec/glossary').glob('*.md'):
    for line in part.read_text().splitlines():
        if not line.startswith('|'):
            continue
        term = line.split('|')[1].strip()
        if term == 'Term' or re.fullmatch(r':?-+:?', term):
            continue
        terms.add(term.translate(fold_ascii).removesuffix(' ⚠').replace('`', ''))
spans = set()
for chapter in book.rglob('*.md'):
    if chapter == book / 'spec/glossary.md' or (book / 'spec/glossary') in chapter.parents:
        continue
    for line in chapter.read_text().splitlines():
        for span in re.findall(r'\*\*([^*]+)\*\*', line):
            spans.add((span.translate(fold_ascii), span))
ordered = sorted(spans, key=lambda pair: ('\t'.join(pair)).encode('utf-8'))
nonmatching = [original for folded, original in ordered if folded not in terms]
expected = ['    · ' + span for span in nonmatching[:15]]
counts = re.search(r'bolded spans: ([0-9]+) · matching a glossary term: ([0-9]+) · not matching: ([0-9]+)', run.stdout)
expected_counts = (len(spans), len(spans) - len(nonmatching), len(nonmatching))
actual_counts = tuple(map(int, counts.groups())) if counts else None
if input_rc != '0' or run.returncode != 0 or run.stderr or samples != expected or actual_counts != expected_counts or '/ 0 failure(s)' not in run.stdout:
    print(f'REFUSED — advisory {disposition}: input={input_rc}, rc={run.returncode}, '
          f'samples={len(samples)}, stderr={run.stderr!r}')
    sys.exit(1)
print('4096 extra spans; independent full counts/first15 samples, zero stderr and census rc=0')
ADVISORY_CHECK
)"; advisory_rc=$?
  if [ "$advisory_rc" -eq 0 ]; then
    ok "A1-$disposition" "$detail"
  else
    bad "A1-$disposition" "actual large advisory population failed its output/signal assertion" "$detail"
  fi
done

# ---------------------------------------------------------------- ARITY (S1)
D="$WORK/arity"; mkroot "$D"
printf '| extra term | meaning | object | also called | token | a sixth cell |\n' \
  >> "$D/docs/book/src/spec/glossary/recipe-and-pieces.md"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'has 6 cells, expected 5' <<<"$out"; then
  ok ARITY "a six-cell entry row is refused, naming the file and line (exit=$rc)"
else
  bad ARITY "a malformed entry row was not refused (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- MISSING: refuse, do not skip
D="$WORK/empty"; mkdir -p "$D"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 2 ] && grep -q 'REFUSED' <<<"$out"; then
  ok MISSING "a root with no glossary refuses with exit=2 instead of reporting green over nothing"
else
  bad MISSING "an absent glossary did not refuse (exit=$rc)" "$out"
fi

echo
printf 'probes: %d pass / %d fail\n' "$pass" "$fail"
[ "$fail" -eq 0 ] || exit 1
exit 0
