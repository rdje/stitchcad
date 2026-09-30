#!/usr/bin/env bash
# docs/tasks/artifacts/uncertainty/run_uncertainty_probes.sh
# G0-CONTRACT.19 — end-to-end probes for the uncertainty census.
#
# WHY: the census's claim is that nothing the book admits it does not know is unowned, and a census that has
# only ever agreed with the book has never been seen to notice anything. Each RED arm copies the real book into
# a synthetic root, removes ONE property, and requires the refusal that names it.
#
#   REAL          the book as it stands: every blocking marker inside a Verification-status section names an
#                 authority, and the census reports the population it found
#   OWNED-GREEN   a synthetic chapter whose `assumed` claim names a leaf passes — the control that makes the
#                 two RED arms below mean something rather than merely fail
#   UNOWNED       the same chapter with no resolver named is refused by name — the arm that matters, because
#                 the failure it stands for is an `assumed` constant nobody has to confirm
#   NEW-CHAPTER   a chapter added AFTER the census was written is scanned, and its unowned claim is refused —
#                 the rule is about the population, not about the files that exist today
#   OUT-OF-SCOPE  a blocking marker OUTSIDE a status section is reported as an advisory and does NOT fail, which
#                 is the arm that keeps U1 from becoming a classifier: a chapter defining the state `unknown`
#                 is not making an unknown claim
#   MISSING       a root with no book REFUSES (exit 2) instead of reporting green over nothing
#
# Usage:  bash docs/tasks/artifacts/uncertainty/run_uncertainty_probes.sh
# Output: per-arm lines plus `probes: N pass / M fail` (exit nonzero if M > 0).
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
CENSUS="$ROOT/docs/tasks/artifacts/uncertainty/run_uncertainty_census.sh"
[ -f "$CENSUS" ] || { echo "probe: REFUSED — $CENSUS not found" >&2; exit 2; }

WORK="${TMPDIR:-$ROOT/target/scratch}/uncertainty_probes"
rm -rf "$WORK"; mkdir -p "$WORK"; trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { printf '  ✓ %-13s %s\n' "$1" "$2"; pass=$((pass+1)); }
bad() { printf '  ✗ %-13s %s\n' "$1" "$2"; fail=$((fail+1))
        [ -n "${3:-}" ] && printf '%s\n' "$3" | grep -E '^  x |census:|REFUSED' | head -5 | sed 's/^/            /'; }

mkroot() { # a synthetic root holding a copy of the real book
  local d="$1"; rm -rf "$d"; mkdir -p "$d"
  ( cd "$ROOT" && tar cf - docs/book/src 2>/dev/null ) | ( cd "$d" && tar xf - )
}
run() { UNCERTAINTY_ROOT="$1" bash "$CENSUS" 2>&1; }
sedfile() { local f="$1"; shift; sed "$@" "$f" > "$f.tmp" && mv "$f.tmp" "$f"; }
SKIRT="docs/book/src/spec/reference-skirt.md"

echo "uncertainty probes — docs/tasks/artifacts/uncertainty/run_uncertainty_census.sh"

# ---------------------------------------------------------------- REAL
out="$(run "$ROOT")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q '/ 0 unowned / 0 failure(s)' <<<"$out"; then
  ok REAL "$(grep -o 'uncertainty census: .*' <<<"$out")"
else
  bad REAL "the real book does not satisfy the census (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- synthetic arms: the rule, not the chapter
# Mutating the real book cannot prove U1 has teeth, because a section that names five authorities still names
# one after a single substitution — which is why the first cut of this arm passed for the wrong reason. A
# minimal synthetic chapter makes the property the only thing in the file.
synth() { # $1 = dir · $2 = the status section's body
  local d="$1"; rm -rf "$d"; mkdir -p "$d/docs/book/src/spec"
  { printf '# Synthetic chapter\n\n## 1. Content\n\nA claim with no marker at all.\n\n'
    printf '## 9. Verification status of the claims in this chapter\n\n%s\n' "$2"
  } > "$d/docs/book/src/spec/synthetic.md"
}

D="$WORK/owned"
synth "$D" '- **The hem depth** — `assumed`, and `G9-SYNTH.1` owns confirming it.'
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q 'status section: 1' <<<"$out" && grep -q 'unowned: 0' <<<"$out"; then
  ok OWNED-GREEN "an assumption whose section names a leaf passes (exit=$rc)"
else
  bad OWNED-GREEN "an owned assumption was refused, so the RED arms below prove nothing (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- UNOWNED (U1)
D="$WORK/unowned"
synth "$D" '- **The hem depth** — `assumed`, and nobody is named to confirm it.'
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'marks `assumed`' <<<"$out" && grep -q 'names no resolving authority' <<<"$out"; then
  ok UNOWNED "an assumption whose section names no resolver is refused by name (exit=$rc)"
else
  bad UNOWNED "an unowned assumption was accepted (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- NEW-CHAPTER (U1 over a growing population)
# A new CHAPTER, not a new bullet in an owned section: U1 reads the section, so an unowned bullet inside a
# section that names a resolver elsewhere inherits it — which is the rule's declared granularity, not a hole
# to be papered over. What must be proven is that the census scans the population as it grows.
D="$WORK/newchapter"; mkroot "$D"
mkdir -p "$D/docs/book/src/spec"
printf '# A chapter added after the census was written\n\n## 7. Verification status of the claims\n\n- **The collar roll** — `assumed`, and nobody is named.\n' \
  > "$D/docs/book/src/spec/late-chapter.md"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'late-chapter.md:5 marks `assumed`' <<<"$out"; then
  ok NEW-CHAPTER "a chapter added later is scanned, and its unowned claim is refused by name (exit=$rc)"
else
  bad NEW-CHAPTER "a new chapter's unowned claim was accepted, so U1 only knows today's files (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- OUT-OF-SCOPE (U1 must not classify prose)
# ontology §5 DEFINES the states; a marker there is a definition, not a claim, and must not fail the census.
D="$WORK/scope"; mkroot "$D"
if grep -q '`unknown`' "$D/docs/book/src/spec/ontology.md"; then
  out="$(run "$D")"; rc=$?
  if [ "$rc" -eq 0 ] && grep -q 'outside a status section' <<<"$out"; then
    ok OUT-OF-SCOPE "a marker outside a status section stays an advisory (exit=$rc, $(grep -o 'outside a status section: [0-9]*' <<<"$out"))"
  else
    bad OUT-OF-SCOPE "the census judged a definition as a claim (exit=$rc)" "$out"
  fi
else
  bad OUT-OF-SCOPE "the ontology no longer defines \`unknown\`, so this arm proves nothing" ""
fi

# ---------------------------------------------------------------- MISSING
D="$WORK/empty"; mkdir -p "$D"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 2 ] && grep -q 'REFUSED' <<<"$out"; then
  ok MISSING "a root with no book refuses with exit=2 instead of reporting green over nothing"
else
  bad MISSING "an absent book did not refuse (exit=$rc)" "$out"
fi

echo
printf 'probes: %d pass / %d fail\n' "$pass" "$fail"
[ "$fail" -eq 0 ] || exit 1
exit 0
