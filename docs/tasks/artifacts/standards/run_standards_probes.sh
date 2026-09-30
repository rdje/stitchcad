#!/usr/bin/env bash
# docs/tasks/artifacts/standards/run_standards_probes.sh
# G0-CONTRACT.7 — end-to-end probes for the standards census.
#
# WHY: a census that has only ever passed on the tree it was written against has never been seen to notice
# anything. The claim this one guards — "no standard is cited anywhere in the book without a status and an
# owner" — is exactly the kind that fails by accretion: one future chapter writes "per ISO 4915 …" and the
# registry never hears about it. Each RED arm below copies the real book into a synthetic root, smuggles in
# one such claim, and requires the census to name it.
#
#   REAL           the census passes on this repository as it stands
#   UNREGISTERED   a designation cited in a chapter and absent from the registry is refused (S2)
#   BAD-STATUS     a status outside the closed vocabulary is refused (S3)
#   NO-OWNER       a registered standard whose owner cell is an em dash is refused (S3)
#   BARE-READ      a `read-in-repo` claim carrying no clause or table citation is refused (S4)
#   MISSING        a root with no standards chapter REFUSES (exit 2) instead of reporting green
#
# Usage:  bash docs/tasks/artifacts/standards/run_standards_probes.sh
# Output: per-arm lines plus `probes: N pass / M fail` (exit nonzero if M > 0).
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
CENSUS="$ROOT/docs/tasks/artifacts/standards/run_standards_census.sh"
[ -f "$CENSUS" ] || { echo "probe: REFUSED — $CENSUS not found" >&2; exit 2; }

WORK="${TMPDIR:-$ROOT/target/scratch}/standards_probes"
rm -rf "$WORK"; mkdir -p "$WORK"; trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { printf '  ✓ %-13s %s\n' "$1" "$2"; pass=$((pass+1)); }
bad() { printf '  ✗ %-13s %s\n' "$1" "$2"; fail=$((fail+1))
        [ -n "${3:-}" ] && printf '%s\n' "$3" | grep -E '^  x |census:|REFUSED' | head -6 | sed 's/^/          /'; }

CHAPTER_REL="docs/book/src/spec/standards.md"
mkroot() {
  local d="$1"; rm -rf "$d"; mkdir -p "$d"
  ( cd "$ROOT" && tar cf - docs/book/src 2>/dev/null ) | ( cd "$d" && tar xf - )
}
run() { local d="$1"; STANDARDS_ROOT="$d" bash "$CENSUS" 2>&1; }
sedfile() { local f="$1"; shift; sed "$@" "$f" > "$f.tmp" && mv "$f.tmp" "$f"; }

echo "standards probes — docs/tasks/artifacts/standards/run_standards_census.sh"

# ---------------------------------------------------------------- REAL
out="$(run "$ROOT")"; rc=$?
if [ "$rc" -eq 0 ] && grep -q '/ 0 failure(s)' <<<"$out"; then
  ok REAL "$(grep -o 'standards census: .*' <<<"$out")"
else
  bad REAL "the real book does not satisfy the standards census (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- UNREGISTERED (S2)
D="$WORK/unregistered"; mkroot "$D"
printf '\nSeam strength follows ISO 4915, as everybody knows.\n' >> "$D/docs/book/src/spec/ontology.md"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q '`ISO 4915` is used in the book' <<<"$out"; then
  ok UNREGISTERED "a standard smuggled into a chapter is refused by name (exit=$rc)"
else bad UNREGISTERED "an unregistered standard citation was accepted (exit=$rc)" "$out"; fi

# ---------------------------------------------------------------- BAD-STATUS (S3)
D="$WORK/status"; mkroot "$D"
sedfile "$D/$CHAPTER_REL" 's/| `cited-from-roadmap` (§3.4) |/| as is well known |/'
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'is not one of the three the chapter declares' <<<"$out"; then
  ok BAD-STATUS "a status outside the closed vocabulary is refused (exit=$rc)"
else bad BAD-STATUS "an invented status was accepted (exit=$rc)" "$out"; fi

# ---------------------------------------------------------------- NO-OWNER (S3)
D="$WORK/owner"; mkroot "$D"
sedfile "$D/$CHAPTER_REL" 's#| `cited-from-roadmap` (§3.1) | `G0-CONTRACT.14` names the reviewer |#| `cited-from-roadmap` (§3.1) | — |#'
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'has no owner, so its claim belongs to nobody' <<<"$out"; then
  ok NO-OWNER "a registered standard with no owner is refused (exit=$rc)"
else bad NO-OWNER "an ownerless claim was accepted (exit=$rc)" "$out"; fi

# ---------------------------------------------------------------- BARE-READ (S4)
D="$WORK/bare"; mkroot "$D"
sedfile "$D/$CHAPTER_REL" 's/^| ISO 8559 | `cited-from-roadmap` (§3.1) |/| ISO 8559 | `read-in-repo` |/'
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 1 ] && grep -q 'claims `read-in-repo` for `ISO 8559` but cites no clause or table' <<<"$out"; then
  ok BARE-READ "a read-in-repo claim with no citation behind it is refused (exit=$rc)"
else bad BARE-READ "an unevidenced read-in-repo claim was accepted (exit=$rc)" "$out"; fi

# ---------------------------------------------------------------- MISSING
D="$WORK/empty"; mkdir -p "$D"
out="$(run "$D")"; rc=$?
if [ "$rc" -eq 2 ] && grep -q 'REFUSED' <<<"$out"; then
  ok MISSING "a root with no standards chapter refuses with exit=2 instead of reporting green"
else bad MISSING "an absent chapter did not refuse (exit=$rc)" "$out"; fi

echo
printf 'probes: %d pass / %d fail\n' "$pass" "$fail"
[ "$fail" -eq 0 ] || exit 1
exit 0
