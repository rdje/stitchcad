#!/usr/bin/env bash
# docs/tasks/artifacts/live_doc_size/run_live_doc_size_probes.sh
# SPINE.4.3 — end-to-end probes for scripts/check_live_doc_size.sh (the LIVE-DOC-SIZE project doctrine).
#
# The check's own `--self-test` proves each REFUSAL CLASS fires against a synthetic registry. These
# probes prove the two things a synthetic registry cannot:
#   REAL-1  the check passes on THIS repository's real registry and real tree — i.e. every tracked
#           Markdown file is claimed by a row, and nothing is over a ceiling today;
#   REAL-2  ⭐ delete one row from a COPY of the real registry and the check must refuse that exact
#           file as an unclassified live surface. Without this arm, "coverage" is a word: a check that
#           only ever passes on the real tree has never been seen to notice a surface that lost its row.
#   REAL-3  ⭐ declare an absent revision token in the actual roadmap row's copied at= field and
#           require its exact refusal, without assuming a current version (D275). This makes the
#           baseline revision-aware rather than decorative: a re-based baseline that nobody re-measures
#           is a silent widening, and only this arm has ever seen the refusal fire on real data.
#   MISSING a check whose data plane is absent must REFUSE (exit 2), not report green over an absence.
#
# The real registry is never mutated: REAL-2 and MISSING run against copies under target/.
#
# Usage:  bash docs/tasks/artifacts/live_doc_size/run_live_doc_size_probes.sh
# Output: per-arm lines plus `probes: N pass / M fail` (exit nonzero if M > 0).
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
CHECK="$ROOT/scripts/check_live_doc_size.sh"
REG="$ROOT/.doctrine/live_document_size"
[ -f "$CHECK" ] || { echo "probe: REFUSED — $CHECK not found" >&2; exit 2; }

WORK="$ROOT/target/doctrine_scratch/live_doc_size_probes"
rm -rf "$WORK"; mkdir -p "$WORK"; trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { printf '  ✓ %-8s %s\n' "$1" "$2"; pass=$((pass+1)); }
bad() { printf '  ✗ %-8s %s\n' "$1" "$2"; fail=$((fail+1)); [ -n "${3:-}" ] && printf '%s\n' "$3" | sed 's/^/        /'; }

echo "live-doc-size probes — scripts/check_live_doc_size.sh"

# ---------------------------------------------------------------- SELF-TEST: refusal classes
out="$(bash "$CHECK" --self-test 2>&1)"; rc=$?
if [ "$rc" -eq 0 ] && grep -q '0 failed' <<<"$out"; then
  ok SELF-TEST "$(grep -o '[0-9]* arms, 0 failed' <<<"$out") — every refusal class fires on a synthetic registry"
else
  bad SELF-TEST "the check's own self-test failed (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- REAL-1: this tree, this registry
out="$(bash "$CHECK" 2>&1)"; rc=$?
if [ "$rc" -eq 0 ] && grep -q 'live-doc-size: OK' <<<"$out"; then
  ok REAL-1 "$(grep -o 'OK — .*' <<<"$out")"
else
  bad REAL-1 "the real tree is not inside its declared bounds (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- REAL-2: coverage has teeth
mkdir -p "$WORK/reg"
grep -v '^roadmap	' "$REG/surfaces.tsv" > "$WORK/reg/surfaces.tsv"
cp "$REG/routes.tsv" "$WORK/reg/routes.tsv"
# Note: dropping the row also strands route R04, so this arm expects BOTH refusals (an unclassified
# surface and a route to an unclassified destination); the assertion is on the coverage message.

out="$(LIVE_DOC_SIZE_REGISTRY="$WORK/reg" bash "$CHECK" 2>&1)"; rc=$?
if [ "$rc" -ne 0 ] && grep -q 'unclassified live surface: ROADMAP.md' <<<"$out"; then
  ok REAL-2 "deleting the roadmap row makes the check name ROADMAP.md as unclassified (exit=$rc)"
else
  bad REAL-2 "coverage is decorative: removing a row did not produce an unclassified-surface refusal (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- REAL-3: a stale revision baseline
mkdir -p "$WORK/stale"
STALE_TOKEN=probe_absent_revision_d275
if ! awk -F'\t' -v OFS='\t' -v stale="$STALE_TOKEN" '
  $1=="roadmap" {
    count=split($20, parts, ";"); found=0
    for(i=1;i<=count;i++) if(parts[i] ~ /^at=/) {parts[i]="at=" stale; found++}
    if(found!=1) exit 2
    debt=parts[1]; for(i=2;i<=count;i++) debt=debt ";" parts[i]
    $20=debt; changed++
  }
  {print}
  END {if(changed!=1) exit 2}
' "$REG/surfaces.tsv" > "$WORK/stale/surfaces.tsv"; then
  bad REAL-3 "cannot mutate exactly one actual roadmap revision field" ""
fi
cp "$REG/routes.tsv" "$WORK/stale/routes.tsv"
grep -Fq "at=$STALE_TOKEN" "$WORK/stale/surfaces.tsv" || bad REAL-3 "the registry copy was not staled" ""
out="$(LIVE_DOC_SIZE_REGISTRY="$WORK/stale" bash "$CHECK" 2>&1)"; rc=$?
expected="$(printf 'debt baseline was measured at revision `%s`' "$STALE_TOKEN")"
if [ "$rc" -ne 0 ] && grep -Fq "$expected" <<<"$out"; then
  ok REAL-3 "a baseline measured at a revision the file no longer declares is refused by name (exit=$rc)"
else
  bad REAL-3 "the mutated revision lacked its exact refusal (exit=$rc)" "$out"
fi

# ---------------------------------------------------------------- MISSING: refuse, do not skip
mkdir -p "$WORK/empty"
out="$(LIVE_DOC_SIZE_REGISTRY="$WORK/empty" bash "$CHECK" 2>&1)"; rc=$?
if [ "$rc" -eq 2 ] && grep -q 'REFUSED' <<<"$out"; then
  ok MISSING "an absent data plane refuses with exit=2 instead of reporting green over nothing"
else
  bad MISSING "a missing registry did not refuse (exit=$rc)" "$out"
fi

# D274: one real checker finishes while another remains inside its native measurement call.
out="$(python3 -I -B "$ROOT/docs/tasks/artifacts/live_doc_size/concurrent_contract.py" 2>&1)"; rc=$?
if [ "$rc" -eq 0 ]; then
  ok OVERLAP "$out"
else
  bad OVERLAP "concurrent invocation scratch is not isolated (exit=$rc)" "$out"
fi

# D279/D280: native content-width and exact Git storage under newline conversion settings.
out="$(python3 -I -B "$ROOT/docs/tasks/artifacts/live_doc_size/newline_contract.py" 2>&1)"; rc=$?
if [ "$rc" -eq 0 ]; then
  ok NEWLINES "native content-width, raw-byte storage and effective-attribute controls passed (exit=0)"
else
  bad NEWLINES "newline measurement/storage contract failed (exit=$rc)" "$out"
fi

echo
printf 'probes: %d pass / %d fail\n' "$pass" "$fail"
[ "$fail" -eq 0 ] || exit 1
exit 0
