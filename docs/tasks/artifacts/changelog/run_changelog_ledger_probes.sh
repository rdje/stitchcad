#!/usr/bin/env bash
# docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh
# G0-CONTRACT.1 — the verifier the containment doctrine's rollover protocol demands.
#
# LIVE_DOCUMENT_SIZE_CONTAINMENT.md's "Atomic partition, rollover, and archive protocol" step 6 requires
# the transition to run link, ordering, uniqueness and retrieval checks, and its archive-descriptor
# contract requires "an executable proof that retrieval reproduces the declared content". A rollover
# performed by hand without those checks is how two real defects landed here:
#   D29 — the live window was not in the newest-first order its own header declares, so "seal the
#         oldest" would have sealed the wrong end;
#   D30 — the sealed segment's descriptor claimed coverage through `STITCHCAD-SPINE-0004c` while that
#         entry was still live, so the pointer sent a reader to a segment that did not contain it.
#
# THE FIVE RULES, each an arm on the real tree and an arm on a copy broken in exactly one way:
#   ORDER      the live window's entry ids appear in commit order, newest first
#   NO-DUP     no id in the live window is also an entry of a sealed segment (uniqueness)
#   DESCRIPTOR every sealed segment's recorded sha256 equals the digest of its sealed content, and its
#              declared line count equals the content's
#   COVERAGE   every id a segment's descriptor names as covered is actually in that segment
#   POINTER    the live pointer and the segments on disk name each other, both directions
#
# THE SEALED CONTENT, defined exactly because a digest is only a proof if both sides mean the same
# bytes: everything after the segment's first `---` rule and the single blank line that follows it.
# For part1 that is lines 17–381, 365 lines / 30452 bytes / `sha256:f4aec75a…`, which is what its
# descriptor declares — verified, not assumed.
#
# HONEST LIMITS: ORDER needs the commit history, so it derives the expected order from `git log` on the
# real tree and reads it from a file on a synthetic one — a probe that could only run where git exists
# could never be handed a broken order to notice. COVERAGE carries one declared, reasoned exemption:
# part1's coverage line is wrong and part1 is immutable, so the correction lives in a superseding
# record (part2's descriptor and the live pointer), never in an edit of the sealed file.
#
# THE PENDING SEAM, and why it exists: a changelog entry is written BEFORE the commit that carries it, so
# between the append and the commit its id is in the file and not in the history — a false red on every
# single slice, and a permanent false red is how a check teaches authors to ignore it. `LEDGER_PENDING=<id>`
# declares that one id as the newest; ORDER prints that it honoured the seam, and the seam cannot hide an
# inversion between two committed ids. After the commit the variable is unset and the arm derives
# everything from history alone.
#
# Usage:  bash docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh
# Output: per-arm lines plus `probes: N pass / M fail` (exit nonzero if M > 0).
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

WORK="${TMPDIR:-$ROOT/target/scratch}/changelog_ledger_probes"
rm -rf "$WORK"; mkdir -p "$WORK"; trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { printf '  ✓ %-11s %s\n' "$1" "$2"; pass=$((pass+1)); }
bad() { printf '  ✗ %-11s %s\n' "$1" "$2"; fail=$((fail+1))
        [ -n "${3:-}" ] && printf '%s\n' "$3" | grep -E ' FAIL | PASS | SKIP ' | head -8 | sed 's/^/          /'; }

# A coverage exemption is a claim, so it carries its reason.
DEFAULT_EXEMPT="$WORK/coverage_exempt.tsv"
cat > "$DEFAULT_EXEMPT" <<'EOF'
stitchcad-changelog-part1.md	D30: its coverage line names STITCHCAD-SPINE-0004c, which stayed live. The segment is immutable, so the correction is the superseding record in part2's descriptor and in the live pointer.
EOF
: > "$WORK/no_exempt.tsv"

# ── the checker: five rules over one root, printed as `RULE PASS|FAIL|SKIP detail` ─────────────
# $1 = root to judge · $2 = expected id order, newest first, one per line (empty → derive from git)
# $3 = coverage-exemption TSV
ledger_verdicts() {
  local root="$1" order="${2:-}" exempt="${3:-/dev/null}"
  local log="$root/CHANGELOG.md" hist="$root/docs/history"
  local live sealed ids id pos prev prevpos rc detail dup f base
  [ -f "$log" ] || { printf 'POINTER FAIL no CHANGELOG.md at %s\n' "$root"; return; }

  live="$(grep -oE '^## STITCHCAD-[A-Za-z0-9]+-[0-9]+[a-c]?' "$log" | sed 's/^## //')"
  sealed="$(grep -hoE '^## STITCHCAD-[A-Za-z0-9]+-[0-9]+[a-c]?' "$hist"/*.md 2>/dev/null \
            | sed 's/^## //' | LC_ALL=C sort -u)"

  # ORDER ------------------------------------------------------------------
  local pending_note=""
  if [ -n "$order" ] && [ -f "$order" ]; then ids="$(cat "$order")"
  else
    ids="$(git log --format='%s' 2>/dev/null | grep -oE 'STITCHCAD-[A-Za-z0-9]+-[0-9]+[a-c]?' | awk '!s[$0]++')"
    if [ -n "${LEDGER_PENDING:-}" ]; then
      ids="$(printf '%s\n%s\n' "$LEDGER_PENDING" "$ids" | awk '!s[$0]++')"
      pending_note=" (pending id $LEDGER_PENDING honoured as newest)"
    fi
  fi
  prev=""; prevpos=0; rc=0; detail=""
  while IFS= read -r id; do
    [ -n "$id" ] || continue
    pos="$(printf '%s\n' "$ids" | grep -nx -- "$id" | cut -d: -f1 | head -1)"
    if [ -z "$pos" ]; then detail="$detail $id(absent-from-history)"; rc=1; continue; fi
    if [ "$prevpos" -ne 0 ] && [ "$pos" -lt "$prevpos" ]; then detail="$detail $id-older-than-$prev"; rc=1; fi
    prev="$id"; prevpos="$pos"
  done <<< "$live"
  if [ "$rc" -eq 0 ]; then
    printf 'ORDER PASS %s live entries in commit order, newest first%s\n' \
      "$(printf '%s\n' "$live" | grep -c .)" "$pending_note"
  else printf 'ORDER FAIL the live window is not newest-first:%s\n' "$detail"; fi

  # NO-DUP -----------------------------------------------------------------
  dup="$(comm -12 <(printf '%s\n' "$live" | LC_ALL=C sort -u) <(printf '%s\n' "$sealed") | tr '\n' ' ')"
  if [ -z "${dup// /}" ]; then printf 'NO-DUP PASS no live entry is also sealed\n'
  else printf 'NO-DUP FAIL both live and sealed: %s\n' "$dup"; fi

  # DESCRIPTOR + COVERAGE --------------------------------------------------
  for f in "$hist"/stitchcad-changelog-part*.md; do
    [ -f "$f" ] || continue
    base="${f##*/}"
    local rule want_sha got_sha content n want_lines have_lines cov miss exempted
    rule="$(grep -n '^---$' "$f" | head -1 | cut -d: -f1)"
    if [ -z "$rule" ]; then printf 'DESCRIPTOR FAIL %s has no --- rule sealing its descriptor from its content\n' "$base"
    else
      content="$(sed -n "$((rule + 2)),\$p" "$f")"
      want_sha="$(grep -oE 'sha256:[0-9a-f]{64}' "$f" | head -1 | cut -d: -f2)"
      got_sha="$(printf '%s\n' "$content" | shasum -a 256 | cut -d' ' -f1)"
      have_lines="$(printf '%s\n' "$content" | grep -c '')"
      want_lines="$(grep -oE '\*\*Sealed identity:\*\* [0-9]+ lines' "$f" | grep -oE '^[^ ]+ [0-9]+' | grep -oE '[0-9]+')"
      if [ -z "$want_sha" ]; then
        printf 'DESCRIPTOR FAIL %s records no sha256\n' "$base"
      elif [ "$got_sha" != "$want_sha" ]; then
        printf 'DESCRIPTOR FAIL %s content hashes to %s…, its descriptor declares %s…\n' "$base" "${got_sha:0:16}" "${want_sha:0:16}"
      elif [ -n "$want_lines" ] && [ "$want_lines" != "$have_lines" ]; then
        printf 'DESCRIPTOR FAIL %s declares %s sealed lines; the content that hashes is %s\n' "$base" "$want_lines" "$have_lines"
      else
        printf 'DESCRIPTOR PASS %s: %s lines reproduce sha256:%s…\n' "$base" "$have_lines" "${want_sha:0:16}"
      fi
    fi
    exempted="$(awk -F'\t' -v b="$base" '$1 == b { print $2 }' "$exempt")"
    if [ -n "$exempted" ]; then printf 'COVERAGE SKIP %s exempt — %s\n' "$base" "$exempted"; continue; fi
    cov="$(grep -m1 -oE '\*\*Coverage:\*\*.*' "$f")"
    miss=""
    if [ -n "$cov" ]; then
      for id in $(grep -oE 'STITCHCAD-[A-Za-z0-9]+-[0-9]+[a-c]?' <<<"$cov"); do
        grep -q "^## $id " "$f" || miss="$miss $id"
      done
      if [ -z "$miss" ]; then printf 'COVERAGE PASS %s names only ids it contains\n' "$base"
      else printf 'COVERAGE FAIL %s descriptor names%s, which the segment does not contain\n' "$base" "$miss"; fi
    else
      printf 'COVERAGE FAIL %s declares no Coverage line\n' "$base"
    fi
  done

  # POINTER ----------------------------------------------------------------
  local on_disk pointed missing extra
  on_disk="$(ls "$hist"/stitchcad-changelog-part*.md 2>/dev/null | sed 's#.*/##' | LC_ALL=C sort)"
  pointed="$(grep -oE 'stitchcad-changelog-part[0-9]+\.md' "$log" | LC_ALL=C sort -u)"
  missing="$(comm -13 <(printf '%s\n' "$pointed") <(printf '%s\n' "$on_disk") | tr '\n' ' ')"
  extra="$(comm -23 <(printf '%s\n' "$pointed") <(printf '%s\n' "$on_disk") | tr '\n' ' ')"
  if [ -z "${missing// /}" ] && [ -z "${extra// /}" ]; then
    printf 'POINTER PASS the live pointer and %s sealed segment(s) name each other\n' "$(printf '%s\n' "$on_disk" | grep -c .)"
  else printf 'POINTER FAIL unpointed segments:[%s] pointers to nothing:[%s]\n' "$missing" "$extra"; fi
}

mkroot() { # a synthetic root: the real ledger copied, to be broken in exactly one way
  local d="$1"; rm -rf "$d"; mkdir -p "$d/docs/history"
  cp "$ROOT/CHANGELOG.md" "$d/CHANGELOG.md"
  for f in "$ROOT"/docs/history/stitchcad-changelog-part*.md; do [ -f "$f" ] && cp "$f" "$d/docs/history/"; done
}
REAL_ORDER="$WORK/real_order.txt"
git log --format='%s' | grep -oE 'STITCHCAD-[A-Za-z0-9]+-[0-9]+[a-c]?' | awk '!s[$0]++' > "$REAL_ORDER"

arm() { # $1 name · $2 rule that must FAIL, or "-" for "everything holds" · $3 root · $4 order · $5 exempt
  local out
  out="$(ledger_verdicts "$3" "$4" "$5")"
  if [ "$2" = "-" ]; then
    if grep -q ' FAIL ' <<<"$out"; then bad "$1" "the ledger fails a rule" "$out"
    else ok "$1" "$(printf '%s\n' "$out" | grep -cE ' (PASS|SKIP) ') verdicts, 0 failures"; fi
  else
    if grep -q "^$2 FAIL" <<<"$out"; then
      ok "$1" "$2 notices it: $(grep -m1 "^$2 FAIL" <<<"$out" | cut -c1-104)"
    else bad "$1" "$2 did NOT notice the break" "$out"; fi
  fi
}

echo "changelog-ledger probes — the rollover protocol's ordering, uniqueness and retrieval checks"

arm REAL "-" "$ROOT" "" "$DEFAULT_EXEMPT"   # LEDGER_PENDING, if set, is inherited by the checker

# ---------------------------------------------------------------- ORDER has teeth
D="$WORK/order"; rm -rf "$D"; mkdir -p "$D/docs/history"
{ printf '# CHANGELOG.md\n\n## STITCHCAD-AAA-0001 — the older slice\n\nbody\n\n'
  printf '## STITCHCAD-AAA-0002 — the newer slice\n\nbody\n'; } > "$D/CHANGELOG.md"
printf 'STITCHCAD-AAA-0002\nSTITCHCAD-AAA-0001\n' > "$D/order.txt"
arm ORDER-RED ORDER "$D" "$D/order.txt" "$DEFAULT_EXEMPT"

# ---------------------------------------------------------------- NO-DUP has teeth
D="$WORK/dup"; mkroot "$D"
first_sealed="$(grep -hoE '^## STITCHCAD-[A-Za-z0-9]+-[0-9]+[a-c]?' "$D"/docs/history/*.md | head -1 | sed 's/^## //')"
printf '\n## %s — sealed, then duplicated back into the live window\n\nbody\n' "$first_sealed" >> "$D/CHANGELOG.md"
arm DUP-RED NO-DUP "$D" "$REAL_ORDER" "$DEFAULT_EXEMPT"

# ---------------------------------------------------------------- DESCRIPTOR has teeth
D="$WORK/digest"; mkroot "$D"
seg="$(ls "$D"/docs/history/stitchcad-changelog-part*.md | head -1)"
printf '\nOne byte of silent drift in a segment that is supposed to be immutable.\n' >> "$seg"
arm DIGEST-RED DESCRIPTOR "$D" "$REAL_ORDER" "$DEFAULT_EXEMPT"

# ---------------------------------------------------------------- COVERAGE has teeth
D="$WORK/coverage"; mkroot "$D"
seg="$(ls "$D"/docs/history/stitchcad-changelog-part*.md | tail -1)"
printf '\n- **Coverage:** from `STITCHCAD-NOTHING-9999` through `STITCHCAD-NOTHING-9998`\n' >> "$seg"
arm COVERAGE-RED COVERAGE "$D" "$REAL_ORDER" "$WORK/no_exempt.tsv"

# ---------------------------------------------------------------- POINTER has teeth
D="$WORK/pointer"; mkroot "$D"
sed 's/stitchcad-changelog-part1\.md/stitchcad-changelog-partX.md/g' "$ROOT/CHANGELOG.md" > "$D/CHANGELOG.md"
arm POINTER-RED POINTER "$D" "$REAL_ORDER" "$DEFAULT_EXEMPT"

echo
printf 'probes: %d pass / %d fail\n' "$pass" "$fail"
[ "$fail" -eq 0 ] || exit 1
exit 0
