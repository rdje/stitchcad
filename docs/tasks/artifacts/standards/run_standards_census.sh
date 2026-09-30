#!/usr/bin/env bash
# docs/tasks/artifacts/standards/run_standards_census.sh
# G0-CONTRACT.7 — the census behind the standards chapter's central claim: no external standard is cited
# anywhere in this book without a registered role, a status from the closed vocabulary, and a named owner.
#
# WHY A CENSUS AND NOT A SENTENCE: the claim quantifies over every chapter, including the ten G0 chapters
# not yet written and every guide that follows. A standards claim is the easiest kind to smuggle in — one
# sentence beginning "per ISO 8559 …" makes a normative chapter depend on a document nobody has read here,
# and nothing about the sentence looks like a claim that needs an owner. CLAIM_VERIFICATION.md leg 1 asks
# for the claim to be re-derived and leg 3 asks for a tracked producer; this is both.
#
# WHAT IT CHECKS, mechanically. Each numbered rule fails the census:
#   S1 registry shape  every registry row has 4 cells: designation, status, owner, role
#   S2 registration    every designation appearing anywhere under docs/book/src/ is a registry row
#   S3 status          every row's status is one of the three the chapter declares, and no owner cell is
#                      empty or an em dash
#   S4 evidence        a row claiming `read-in-repo` carries a citation (a clause or table reference) in its
#                      STATUS or ROLE cell, never merely somewhere in the row: an owner cell carrying a
#                      cross-reference once satisfied the whole-line grep, so an unevidenced claim passed
#                      (defect D45, found when an owner cell gained a section sign)
# ADVISORY, printed and never a failure:
#   A1 where each designation is used, per file, so a claim cannot hide in one chapter
#
# HONEST LIMITS: the population is designations matching the declared SHAPES below (`ISO n`, `ASTM Dn`,
# `EN n`, `AAMA`) plus a declared extras list. A standard cited by a name that matches no shape — "the
# German DIN system", "a vendor's spec sheet" — is outside the census, and adding it means adding a shape
# or an extra, not teaching the census to read prose. S4 checks that a citation is PRESENT, not that it is
# true: only a human who has read the document can say that.
#
# Usage:  bash docs/tasks/artifacts/standards/run_standards_census.sh
#         STANDARDS_ROOT=<dir> bash docs/tasks/artifacts/standards/run_standards_census.sh
# Output: per-section tables, then
#         `standards census: <rows> registered / <designations> designations used / <failures> failure(s)`
#         exit 1 on any failure, exit 2 when an input is missing.
set -uo pipefail
export LC_ALL=C

ROOT="${STANDARDS_ROOT:-$(git rev-parse --show-toplevel 2>/dev/null || pwd)}"
cd "$ROOT" || { echo "standards census: REFUSED — cannot enter $ROOT" >&2; exit 2; }

BOOK_SRC="docs/book/src"
CHAPTER="$BOOK_SRC/spec/standards.md"
[ -f "$CHAPTER" ] || { echo "standards census: REFUSED — $CHAPTER not found" >&2; exit 2; }

SCRATCH="$(mktemp -d)"
trap 'rm -rf "$SCRATCH"' EXIT
REGISTRY="$SCRATCH/registry.tsv"   # designation TAB status TAB owner TAB row
USES="$SCRATCH/uses.tsv"           # designation TAB file
EXTRAS="$SCRATCH/extras.tsv"

# Designations the shapes below cannot express, one per line, each with its reason.
cat > "$EXTRAS" <<'EOF'
EOF

fails=0
bad() { printf '  x %s\n' "$1" >&2; fails=$((fails + 1)); }

# ── populations ───────────────────────────────────────────────────────────────────────────────
# the registry: 6-cell table rows in the chapter, first cell = designation
awk -F'|' -v ch="$CHAPTER" '
  function trim(s) { gsub(/^[ \t]+|[ \t]+$/, "", s); return s }
  /^\|/ {
    if (NF - 2 != 4) next
    d = trim($2)
    if (d == "Designation" || d ~ /^:?-+:?$/) next
    printf "%s\t%s\t%s\t%d\n", d, trim($3), trim($4), NR
  }' "$CHAPTER" > "$REGISTRY"

# every designation used anywhere in the book, normalized to its base form
uses() {
  find "$BOOK_SRC" -name '*.md' -type f | LC_ALL=C sort | while IFS= read -r f; do
    grep -oE 'ISO [0-9]+|ASTM D[0-9]+|EN [0-9]+|AAMA' "$f" 2>/dev/null | LC_ALL=C sort -u \
      | awk -v file="$f" '{ printf "%s\t%s\n", $0, file }'
  done
  if [ -s "$EXTRAS" ]; then
    find "$BOOK_SRC" -name '*.md' -type f | LC_ALL=C sort | while IFS= read -r f; do
      while IFS= read -r x; do
        [ -n "$x" ] && grep -qF -- "$x" "$f" && printf '%s\t%s\n' "$x" "$f"
      done < "$EXTRAS"
    done
  fi
}
uses | LC_ALL=C sort -u > "$USES"

# grep -c prints 0 AND exits 1 on no match, so `|| printf 0` would emit a second line (a lesson this
# repository already paid for): capture the count, never append a fallback to grep's own output
registry_rows=$(grep -c . "$REGISTRY"); registry_rows=${registry_rows:-0}
used_total=$(awk -F'\t' '{ print $1 }' "$USES" | LC_ALL=C sort -u | grep -c .); used_total=${used_total:-0}

echo "=== standards census ==="

# ── S1 registry shape ─────────────────────────────────────────────────────────────────────────
echo "-- S1 registry shape (4 cells: designation, status, owner, role)"
s1=$(awk -F'|' -v ch="$CHAPTER" '
  /^\|/ {
    n = NF - 2
    c2 = $2; gsub(/^[ \t]+|[ \t]+$/, "", c2)
    if (c2 == "Designation" || c2 ~ /^:?-+:?$/) next
    if (n != 3 && n != 4) { printf "  x %s:%d is a %d-cell row in the standards chapter; the registry uses 4 and the other tables 3\n", ch, NR, n; bad++ }
  }
  END { printf "COUNT %d\n", bad+0 }' "$CHAPTER")
s1n=${s1##*COUNT }; [ -n "${s1%%COUNT*}" ] && printf '%s' "${s1%%COUNT*}"
fails=$((fails + s1n))
echo "  registry rows: $registry_rows · shape breaches: $s1n"

# ── S2 every designation used in the book is registered ───────────────────────────────────────
echo "-- S2 every designation used in the book is registered"
s2=$(awk -F'\t' '
  FILENAME == ARGV[1] { reg[$1] = $4; next }
  !($1 in reg) {
    if (!($1 in seen)) { printf "  x `%s` is used in the book (first at %s) but the registry has no row for it\n", $1, $2; bad++; seen[$1] = 1 }
  }
  END { printf "COUNT %d\n", bad+0 }' "$REGISTRY" "$USES")
s2n=${s2##*COUNT }; [ -n "${s2%%COUNT*}" ] && printf '%s' "${s2%%COUNT*}"
fails=$((fails + s2n))

# ── S3 status vocabulary and owner ────────────────────────────────────────────────────────────
echo "-- S3 every row carries a status from the closed vocabulary and a named owner"
s3=$(awk -F'\t' -v ch="$CHAPTER" '
  BEGIN { OK["`read-in-repo`"] = 1; OK["`cited-from-roadmap`"] = 1; OK["`unverified-with-owner`"] = 1 }
  {
    # the status cell may carry its source in parentheses — the STATUS is the backticked token it starts with
    st = $2
    if (match(st, /`[^`]+`/)) st = substr(st, RSTART, RLENGTH)
    if ($2 !~ /^`/ || !(st in OK)) {
      printf "  x %s:%s — status `%s` is not one of the three the chapter declares\n", ch, $4, $2; bad++
    } else STATUS[$4] = st
    if ($3 == "" || $3 == "—") { printf "  x %s:%s — `%s` has no owner, so its claim belongs to nobody\n", ch, $4, $1; bad++ }
  }
  END { printf "COUNT %d\n", bad+0 }' "$REGISTRY")
s3n=${s3##*COUNT }; [ -n "${s3%%COUNT*}" ] && printf '%s' "${s3%%COUNT*}"
fails=$((fails + s3n))
echo "  rows: $registry_rows · breaches: $s3n"

# ── S4 a read-in-repo claim carries its citation ──────────────────────────────────────────────
# The citation is sought in the STATUS and ROLE cells only. A whole-row grep was satisfied by an owner
# cell that carried a cross-reference, which is a green verdict on a claim nobody evidenced (D45).
echo "-- S4 a read-in-repo row carries the citation that earns it"
s4=$(awk -F'\t' -v ch="$CHAPTER" '
  $2 ~ /^`read-in-repo`/ {
    line = ""
    while ((getline l < ch) > 0) if (l ~ /^\|/ && index(l, $1) == 3) { line = l; break }
    close(ch)
    n = split(line, cells, "|")            # "" | designation | status | owner | role | ""
    cite = (n >= 3 ? cells[3] : "") " " (n >= 5 ? cells[5] : "")
    if (cite !~ /§|clause|table [0-9]|Table [0-9]/) {
      printf "  x %s:%s claims `read-in-repo` for `%s` but its status and role cells cite no clause or table\n", ch, $4, $1; bad++
    }
  }
  END { printf "COUNT %d\n", bad+0 }' "$REGISTRY")
s4n=${s4##*COUNT }; [ -n "${s4%%COUNT*}" ] && printf '%s' "${s4%%COUNT*}"
fails=$((fails + s4n))
rir=$(awk -F'\t' '$2 ~ /^`read-in-repo`/' "$REGISTRY" | grep -c .)
echo "  read-in-repo rows: ${rir:-0} · breaches: $s4n"

# ── A1 advisory: where each designation is used ───────────────────────────────────────────────
echo "-- A1 advisory: where each designation is used"
awk -F'\t' '{ u[$1] = u[$1] " " $2 } END { for (d in u) printf "  · %-16s in%s\n", d, u[d] }' "$USES" | LC_ALL=C sort

echo "standards census: $registry_rows registered / $used_total designations used / $fails failure(s)"
[ "$fails" -eq 0 ] || exit 1
exit 0
