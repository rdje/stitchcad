#!/usr/bin/env bash
# docs/tasks/artifacts/evidence_signatures/run_signature_portability_probe.sh
# SPINE.11 — the tracked producer behind a published number (defect D20).
#
# WHAT IT ANSWERS: which evidence strings does the acceptance gate's signature list actually
# recognise, and does the answer depend on the regex ENGINE doing the matching?
#
# WHY IT EXISTS: leaf SPINE.8 published "awk left 12 of 36 corpus evidence lines unmatched on this
# platform where grep -qE leaves 2" — and the corpus that produced those numbers lived in untracked
# scratch and was deleted by the SPINE.2 cleanup, so two tracked documents quoted a number no command
# could re-derive. That is a leg-3 (durability) breach of CLAIM_VERIFICATION.md §3: "a measured number
# whose instrument lives in a scratch or ignored directory is a 'trust me' with extra steps". This
# script and `evidence_corpus.txt` beside it are the tracked producer; `make probes` runs it.
#
# THE WATCHED CONSTANTS below are the published measurement. If the probe reports different numbers,
# the universal signature list or the platform's regex engines changed: RE-DERIVE the claim in
# docs/tasks/SPINE.md (leaf SPINE.8) and CHANGELOG.md in the same commit — do not edit the constants
# to make the probe green. A watched number that is silently retuned is worse than an unwatched one.
#
# Usage:  bash docs/tasks/artifacts/evidence_signatures/run_signature_portability_probe.sh
# Output: a per-line table, the two counts, then `probes: N pass / M fail`.
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
UNIVERSAL="$ROOT/scripts/check_task_acceptance.sh"
CORPUS="$HERE/evidence_corpus.txt"
for f in "$UNIVERSAL" "$CORPUS"; do
  [ -f "$f" ] || { echo "signature-probe: REFUSED — $f not found" >&2; exit 2; }
done

# ── watched constants (the published measurement; see the header before changing one) ──────────
EXPECT_LINES=36           # corpus size the claim is stated over
EXPECT_GREP_UNMATCHED=2   # both are bad samples, named below — not dead families
EXPECT_AWK_UNMATCHED=12   # the GNU-extension families BSD awk lacks
# The two lines grep does not match, and why they are samples rather than families:
BAD_SAMPLE_1='exited with code 2'      # the family is `exit(ed)?[ =:](code )?[0-9]+`: "exited: code 2" matches
BAD_SAMPLE_2='42 passed; 0 failed'     # the family's separator class is `[ ,/]+`, which excludes ';'

# ── the signature list, read from the universal check exactly as the gates read it ─────────────
base="$(sed -n "s/^DEFAULT_SIG='\(.*\)'\$/\1/p" "$UNIVERSAL" | head -1)"
if [ -z "$base" ]; then
  echo "signature-probe: REFUSED — could not read DEFAULT_SIG out of $UNIVERSAL;" >&2
  echo "  the universal check changed shape, so this probe would measure nothing. Fix the extractor." >&2
  exit 2
fi
extra=""
if [ -f .doctrine/evidence_tokens.txt ]; then
  extra="$(grep -vE '^[[:space:]]*(#|$)' .doctrine/evidence_tokens.txt | paste -sd'|' -)"
fi
if [ -n "$extra" ]; then SIG="$base|$extra"; else SIG="$base"; fi

matches_grep() { printf '%s' "$1" | grep -qE "$SIG"; }
matches_awk()  { printf '%s' "$1" | awk -v p="$SIG" '$0 ~ p {f=1} END{exit(f?0:1)}'; }

# ── the census ────────────────────────────────────────────────────────────────────────────────
lines=0; grep_unmatched=0; awk_unmatched=0
printf '  %-4s %-4s  %s\n' "grep" "awk" "evidence line"
while IFS= read -r line; do
  [ -n "$line" ] || continue
  lines=$((lines + 1))
  g=yes; a=yes
  matches_grep "$line" || { g=NO;  grep_unmatched=$((grep_unmatched + 1)); }
  matches_awk  "$line" || { a=NO;  awk_unmatched=$((awk_unmatched + 1)); }
  if [ "$g" = NO ] || [ "$a" = NO ]; then
    printf '  %-4s %-4s  %s\n' "$g" "$a" "$line"
  fi
done < "$CORPUS"

echo
printf 'corpus: %s lines · grep-unmatched: %s · awk-unmatched: %s\n' "$lines" "$grep_unmatched" "$awk_unmatched"
printf 'engine used by the gates: grep -qE (both GNU grep and BSD grep match the \\b and {n} families)\n'

# ── arms ──────────────────────────────────────────────────────────────────────────────────────
pass=0; fail=0
arm() { # label · condition-exit · note
  if [ "$2" -eq 0 ]; then printf '  ✓ %-8s %s\n' "$1" "$3"; pass=$((pass+1));
  else printf '  ✗ %-8s %s\n' "$1" "$3"; fail=$((fail+1)); fi
}

[ "$lines" -eq "$EXPECT_LINES" ]; arm CORPUS "$?" \
  "the corpus is the size the published claim is stated over ($EXPECT_LINES lines)"

[ "$grep_unmatched" -eq "$EXPECT_GREP_UNMATCHED" ]; arm GREP "$?" \
  "grep -qE leaves $EXPECT_GREP_UNMATCHED unmatched — both bad samples, so no family is dead under the gate's own engine"

# The two unmatched lines must be exactly the known bad samples: a third means a family really died.
samples_ok=1
matches_grep "$BAD_SAMPLE_1" && samples_ok=0
matches_grep "$BAD_SAMPLE_2" && samples_ok=0
if [ "$samples_ok" -eq 1 ]; then
  arm SAMPLES 0 "the two grep-unmatched lines are the documented bad samples, not evidence families"
else
  arm SAMPLES 1 "a documented bad sample now matches — the signature list changed shape"
fi

[ "$awk_unmatched" -eq "$EXPECT_AWK_UNMATCHED" ]; arm AWK "$?" \
  "awk leaves $EXPECT_AWK_UNMATCHED unmatched — the measured reason signature matching uses grep, not awk"

# A control that the census is not vacuous: a string with no tool output must match nothing.
if ! matches_grep 'the parser mishandles the header' && ! matches_awk 'the parser mishandles the header'; then
  arm CTRL 0 "a prose line with no tool output matches under neither engine (the census discriminates)"
else
  arm CTRL 1 "a prose line matched — the census would pass anything"
fi

echo
printf 'probes: %d pass / %d fail\n' "$pass" "$fail"
if [ "$fail" -ne 0 ]; then
  cat >&2 <<'EOF'
signature-probe: a watched constant moved. Do NOT retune it to go green:
  re-derive the claim in docs/tasks/SPINE.md (leaf SPINE.8) and CHANGELOG.md, and if the universal
  signature list changed shape, say so in the same commit. (CLAIM_VERIFICATION.md §5B: a constant
  that is a function of the repository is derived or gated — and a gate that is retuned silently
  is worse than no gate.)
EOF
  exit 1
fi
exit 0
