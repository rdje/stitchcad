#!/usr/bin/env bash
# docs/tasks/artifacts/feature_matrix/run_feature_matrix_census.sh
# G0-CONTRACT.4 — the census behind the feature matrix's acceptance clause: "nothing in the ontology is
# silently unlisted", and behind the roadmap's own promise that an unsupported construction produces an
# explicit diagnostic rather than an approximation.
#
# WHY A CENSUS AND NOT A SENTENCE: the matrix quantifies over three populations that other documents own
# and will keep changing — the ontology's object clauses, roadmap §1.3's non-goals and roadmap §3.2's
# envelope. A matrix that covered the ontology on the day it was written goes stale the first time
# `G0-CONTRACT.5`–`.12` adds a clause, and the failure is invisible: a feature nobody dispositioned is
# precisely the feature a factory discovers. So the claim is derived, per CLAIM_VERIFICATION.md leg 1,
# and leg 3 (durable) is this tracked producer.
#
# WHAT IT CHECKS, mechanically. Each numbered rule fails the census:
#   M1 shape       a disposition row has exactly 5 cells, a disposition from the closed vocabulary
#                  {supported, rejected, deferred}, a non-empty reason and a non-empty gate cell; any
#                  other table row in the chapter has 3 cells (the disposition, gap and diagnostic tables)
#   M2 diagnostics every `rejected` and `deferred` row names exactly one backticked diagnostic token;
#                  every such token is declared by the §10 table; every declared token is used by a row;
#                  a `supported` row names none
#   M3 ontology    every clause of the ontology chapter that specifies an object is cited as
#                  `ontology §n[.m]` by at least one row — the coverage clause of this leaf
#   M4 non-goals   every roadmap §1.3 bullet appears in a `rejected` row (matched by its first word, the
#                  extraction rule declared below, with the mapping printed for a human to read)
#   M5 envelope    every garment roadmap §3.2 names appears in a `supported` row, and every construction
#                  it names as unsupported appears in a `rejected` row
#   M6 gates       every gate cell names a real gate or track (one that roadmap §11 has a heading for) or
#                  the literal `unnamed (D32)`; a blank or prose-only cell is refused
#   M7 links       every markdown link in the chapter resolves to a file that exists, and a link that
#                  cites a clause cites one the target chapter actually has
# ADVISORY, printed and never a failure:
#   A1 the count of `unnamed (D32)` rows, on every run, because §14 of the chapter promises the gap stays
#      visible rather than being closed by forgetting it
#   A2 feature names that are not glossary terms, so the glossary absorbs them instead of drifting
#
# HONEST LIMITS: M4 matches a non-goal bullet to a row by the bullet's FIRST WORD, lowercased — a shape
# rule, not a classifier, and the mapping it produces is printed so a human can see every pairing. M5's
# keyword list is declared below with the roadmap sentence it comes from, because §3.2 states the envelope
# in prose and no shape rule can extract a garment list from a parenthetical. Neither rule can judge
# whether a disposition is CORRECT: whether a collar belongs in v1 is a product decision, and this census
# only proves the decision was written down.
#
# Usage:  bash docs/tasks/artifacts/feature_matrix/run_feature_matrix_census.sh
#         FEATURE_MATRIX_ROOT=<dir> bash docs/tasks/artifacts/feature_matrix/run_feature_matrix_census.sh
# Output: per-section tables, then
#         `feature-matrix census: <rows> rows / <diagnostics> diagnostics / <failures> failure(s)`
#         exit 1 on any failure, exit 2 when an input is missing.
set -uo pipefail
export LC_ALL=C

ROOT="${FEATURE_MATRIX_ROOT:-$(git rev-parse --show-toplevel 2>/dev/null || pwd)}"
cd "$ROOT" || { echo "feature-matrix census: REFUSED — cannot enter $ROOT" >&2; exit 2; }

MATRIX="docs/book/src/spec/feature-matrix.md"
ONTOLOGY="docs/book/src/spec/ontology.md"
GLOSSARY_DIR="docs/book/src/spec/glossary"
ROADMAP="ROADMAP.md"

for f in "$MATRIX" "$ONTOLOGY" "$ROADMAP"; do
  [ -f "$f" ] || { echo "feature-matrix census: REFUSED — $f not found" >&2; exit 2; }
done

SCRATCH="$(mktemp -d)"
trap 'rm -rf "$SCRATCH"' EXIT
ROWS="$SCRATCH/rows.tsv"          # line TAB feature TAB disposition TAB why TAB gate TAB diagnostic
DIAGS="$SCRATCH/diags.tsv"        # token TAB raised-when
ONTO_CLAUSES="$SCRATCH/onto.tsv"  # clause
NONGOALS="$SCRATCH/nongoals.tsv"  # keyword TAB bullet
GATES="$SCRATCH/gates.txt"        # real gate ids from roadmap §11
TERMS="$SCRATCH/terms.txt"        # glossary terms, lowercased

# ── declared seams, each with its reason ───────────────────────────────────────────────────────
# M5: the envelope keywords, quoted from roadmap §3.2's own parenthetical ("A-line skirt with waist dart
# and CB zipper; darted bodice; set-in sleeve; classic collar; trousers") and from the constructions the
# same clause names as unsupported ("knit stretch blocks, leather, fully bespoke structures").
ENVELOPE_SUPPORTED="$SCRATCH/envelope_supported.tsv"
cat > "$ENVELOPE_SUPPORTED" <<'EOF'
a-line skirt	roadmap §3.2: "A-line skirt with waist dart and CB zipper"
darted bodice	roadmap §3.2: "darted bodice"
set-in sleeve	roadmap §3.2: "set-in sleeve"
classic collar	roadmap §3.2: "classic collar"
trousers	roadmap §3.2: "trousers"
EOF
ENVELOPE_REJECTED="$SCRATCH/envelope_rejected.tsv"
cat > "$ENVELOPE_REJECTED" <<'EOF'
knit	roadmap §3.2: "Unsupported constructions (e.g., knit stretch blocks …)"
leather	roadmap §3.2: "… leather …"
bespoke	roadmap §3.2: "… fully bespoke structures"
EOF
# M3: the ontology sections that are NOT object clauses, so they are not required to be cited.
ONTOLOGY_EXCLUDED="$SCRATCH/onto_excluded.tsv"
cat > "$ONTOLOGY_EXCLUDED" <<'EOF'
8	§8 is the chapter's own verification-status list, not an object
9	§9 is the chapter's test obligations, which §14 of the matrix answers instead
EOF

fails=0
bad() { printf '  x %s\n' "$1" >&2; fails=$((fails + 1)); }

# ── populations ───────────────────────────────────────────────────────────────────────────────
# matrix rows: any table row whose third cell is a disposition, plus the diagnostic table's rows
awk -F'|' '
  function trim(s) { gsub(/^[ \t]+|[ \t]+$/, "", s); return s }
  /^\|/ {
    n = NF - 2
    if (n == 5) {
      d = trim($3)
      if (d == "supported" || d == "rejected" || d == "deferred")
        printf "%d\t%s\t%s\t%s\t%s\t%s\n", NR, trim($2), d, trim($4), trim($5), trim($6) > "'"$ROWS"'"
      printf "%d\t%s\t%d\n", NR, "FIVE", n > "'"$SCRATCH/shape.tsv"'"
    } else if (n == 3) {
      printf "%d\t%s\t%d\n", NR, "THREE", n > "'"$SCRATCH/shape.tsv"'"
    } else {
      printf "%d\t%s\t%d\n", NR, "ODD", n > "'"$SCRATCH/shape.tsv"'"
    }
  }' "$MATRIX"
# the diagnostic table: rows after the `| Token | Raised when |` header
awk -F'|' '
  function trim(s) { gsub(/^[ \t]+|[ \t]+$/, "", s); return s }
  /^\| Token \| Raised when/ { intable = 1; next }
  /^\|/ && intable {
    t = trim($2)
    if (t ~ /^:?-+:?$/) next
    gsub(/`/, "", t)
    if (t == "" ) { intable = 0; next }
    printf "%s\t%s\n", t, trim($3)
    next
  }
  intable && !/^\|/ { intable = 0 }
' "$MATRIX" > "$DIAGS"

awk 'match($0, /^(##|###|####) [0-9]+(\.[0-9]+)*/) {   # h2-h4; alternation, not a `{2,4}` interval   # `###+`, not `#{2,4}`: intervals are an extension
       h = substr($0, RSTART, RLENGTH); sub(/^#+ /, "", h); print h }' "$ONTOLOGY" | LC_ALL=C sort -u > "$ONTO_CLAUSES"

awk '/^### 1\.3 Non-goals/,/^---$/' "$ROADMAP" | grep '^- ' | sed 's/^- //' \
  | awk '{ kw = tolower($1); gsub(/[^a-z0-9-]/, "", kw); printf "%s\t%s\n", kw, $0 }' > "$NONGOALS"

awk 'match($0, /^### (G[0-7]|V[12]) /) { h = substr($0, RSTART, RLENGTH); sub(/^### /, "", h); sub(/ $/, "", h); print h }' \
  "$ROADMAP" | LC_ALL=C sort -u > "$GATES"

SPEC_DIR="docs/book/src/spec"
HEADINGS="$SCRATCH/headings.tsv"
{ find "$SPEC_DIR" -name '*.md' -type f; } | while IFS= read -r f; do
  awk -v file="$f" 'match($0, /^(##|###|####) [0-9]+(\.[0-9]+)*/) {
    h = substr($0, RSTART, RLENGTH); sub(/^#+ /, "", h); printf "%s\t%s\n", file, h }' "$f"
done | LC_ALL=C sort -u > "$HEADINGS"

if [ -d "$GLOSSARY_DIR" ]; then
  awk -F'|' '/^\|/ { t = $2; gsub(/^[ \t]+|[ \t]+$/, "", t)
         if (t == "Term" || t ~ /^:?-+:?$/) next
         sub(/ ⚠$/, "", t); gsub(/`/, "", t); print tolower(t) }' "$GLOSSARY_DIR"/*.md | LC_ALL=C sort -u > "$TERMS"
else : > "$TERMS"; fi

rows_total=$(grep -c . "$ROWS" 2>/dev/null || printf 0)
diag_total=$(grep -c . "$DIAGS" 2>/dev/null || printf 0)
echo "=== feature-matrix census ==="

# ── M1 shape ──────────────────────────────────────────────────────────────────────────────────
echo "-- M1 row shape (5 cells, a closed disposition vocabulary, no empty reason or gate)"
m1=$(awk -F'\t' '
  $2 == "ODD" { printf "  x %s:%s is a %s-cell table row; the matrix uses 5 and its other tables 3\n", "'"$MATRIX"'", $1, $3; bad++ }
  END { printf "COUNT %d\n", bad+0 }' "$SCRATCH/shape.tsv")
m1n=${m1##*COUNT }; [ -n "${m1%%COUNT*}" ] && printf '%s' "${m1%%COUNT*}"
m1b=$(awk -F'\t' '
  $3 != "supported" && $3 != "rejected" && $3 != "deferred" { printf "  x %s:%s disposition `%s` is not in the closed vocabulary\n", "'"$MATRIX"'", $1, $3; bad++ }
  $4 == "" || $4 == "—" { printf "  x %s:%s has no reason\n", "'"$MATRIX"'", $1; bad++ }
  $5 == "" || $5 == "—" { printf "  x %s:%s names no gate\n", "'"$MATRIX"'", $1; bad++ }
  $2 == "" { printf "  x %s:%s has no feature name\n", "'"$MATRIX"'", $1; bad++ }
  END { printf "COUNT %d\n", bad+0 }' "$ROWS")
m1bn=${m1b##*COUNT }; [ -n "${m1b%%COUNT*}" ] && printf '%s' "${m1b%%COUNT*}"
fails=$((fails + m1n + m1bn))
printf '  rows: %s · supported: %s · rejected: %s · deferred: %s · shape breaches: %s\n' \
  "$rows_total" \
  "$(awk -F'\t' '$3 == "supported"' "$ROWS" | grep -c . || printf 0)" \
  "$(awk -F'\t' '$3 == "rejected"' "$ROWS" | grep -c . || printf 0)" \
  "$(awk -F'\t' '$3 == "deferred"' "$ROWS" | grep -c . || printf 0)" \
  "$((m1n + m1bn))"

# ── M2 the diagnostic contract ────────────────────────────────────────────────────────────────
echo "-- M2 every refusal names a declared diagnostic, and every declared diagnostic is used"
m2=$(awk -F'\t' -v diags="$DIAGS" -v mat="$MATRIX" '
  BEGIN { while ((getline l < diags) > 0) { split(l, a, "\t"); D[a[1]] = 1 } close(diags); bad = 0 }
  {
    cell = $6
    n = 0; s = cell
    while (match(s, /`[^`]+`/)) { tok = substr(s, RSTART + 1, RLENGTH - 2); s = substr(s, RSTART + RLENGTH); T[++n] = tok }
    if ($3 == "supported") {
      if (n != 0) { printf "  x %s:%s is `supported` but names a diagnostic\n", mat, $1; bad++ }
    } else {
      if (n != 1) { printf "  x %s:%s is `%s` and names %d diagnostics, expected exactly 1\n", mat, $1, $3, n; bad++ }
      for (i = 1; i <= n; i++) {
        if (!(T[i] in D)) { printf "  x %s:%s names `%s`, which the diagnostic table does not declare\n", mat, $1, T[i]; bad++ }
        else U[T[i]] = 1
      }
    }
  }
  END { for (t in D) if (!(t in U)) { printf "  x diagnostic `%s` is declared but no row raises it\n", t; bad++ }
        printf "COUNT %d\n", bad }' "$ROWS")
m2n=${m2##*COUNT }; [ -n "${m2%%COUNT*}" ] && printf '%s' "${m2%%COUNT*}"
fails=$((fails + m2n))
echo "  declared diagnostics: $diag_total · breaches: $m2n"

# ── M3 ontology coverage ──────────────────────────────────────────────────────────────────────
echo "-- M3 every ontology object clause is cited by a row"
m3=$(awk -F'\t' -v clauses="$ONTO_CLAUSES" -v excluded="$ONTOLOGY_EXCLUDED" '
  BEGIN {
    nc = 0
    while ((getline l < clauses) > 0) { C[++nc] = l } close(clauses)
    while ((getline l < excluded) > 0) { split(l, a, "\t"); X[a[1]] = a[2] } close(excluded)
  }
  { s = $4
    while (match(s, /ontology §[0-9]+(\.[0-9]+)*/)) {
      # "ontology " is 9 bytes and the section sign is 2, so the clause starts at +11 (LC_ALL=C counts bytes)
      cited[substr(s, RSTART + 11, RLENGTH - 11)] = $1
      s = substr(s, RSTART + RLENGTH)
    }
  }
  END {
    bad = 0
    for (i = 1; i <= nc; i++) {
      c = C[i]; top = c; sub(/\..*/, "", top)
      if (top in X) continue
      # a section whose subsections carry the objects is a grouping, not a clause to cite: `## 4` is
      # covered by `### 4.1` … `### 4.7`, and requiring both would demand a citation for a heading
      grouping = 0
      if (c !~ /\./) for (j = 1; j <= nc; j++) if (index(C[j], c ".") == 1) grouping = 1
      if (grouping) continue
      if (!(c in cited)) { printf "  x ontology §%s specifies objects but no matrix row cites it\n", c; bad++ }
      else req++
    }
    for (c in cited) {
      found = 0
      for (i = 1; i <= nc; i++) if (C[i] == c) found = 1
      if (!found) { printf "  x a row cites ontology §%s, which the ontology chapter has no heading for\n", c; bad++ }
    }
    printf "  required object clauses cited: %d\n", req
    printf "COUNT %d\n", bad
  }' "$ROWS")
m3n=${m3##*COUNT }; [ -n "${m3%%COUNT*}" ] && printf '%s' "${m3%%COUNT*}"
fails=$((fails + m3n))
echo "  ontology clauses: $(grep -c . "$ONTO_CLAUSES") · groupings and excluded skipped · breaches: $m3n"

# ── M4 non-goal coverage ──────────────────────────────────────────────────────────────────────
echo "-- M4 every roadmap §1.3 non-goal appears in a rejected row"
m4=$(awk -F'\t' -v nongoals="$NONGOALS" '
  BEGIN { while ((getline l < nongoals) > 0) { split(l, a, "\t"); K[++n] = a[1]; B[n] = a[2] } close(nongoals) }
  $3 == "rejected" { f = tolower($2); R[++m] = f }
  END {
    bad = 0
    for (i = 1; i <= n; i++) {
      hit = ""
      for (j = 1; j <= m; j++) if (index(R[j], K[i])) { hit = R[j]; break }
      if (hit == "") { printf "  x non-goal `%s` (%s) has no rejected row\n", K[i], B[i]; bad++ }
      else printf "    · %-16s → %s\n", K[i], hit
    }
    printf "COUNT %d\n", bad
  }' "$ROWS")
m4n=${m4##*COUNT }; [ -n "${m4%%COUNT*}" ] && printf '%s' "${m4%%COUNT*}"
fails=$((fails + m4n))
echo "  non-goals: $(grep -c . "$NONGOALS") · breaches: $m4n"

# ── M5 envelope coverage ──────────────────────────────────────────────────────────────────────
echo "-- M5 the roadmap's envelope garments are supported, its refusals rejected"
m5=$(awk -F'\t' -v sup="$ENVELOPE_SUPPORTED" -v rej="$ENVELOPE_REJECTED" '
  BEGIN {
    while ((getline l < sup) > 0) { split(l, a, "\t"); S[++ns] = a[1]; SR[ns] = a[2] } close(sup)
    while ((getline l < rej) > 0) { split(l, a, "\t"); J[++nj] = a[1]; JR[nj] = a[2] } close(rej)
  }
  { f = tolower($2)
    if ($3 == "supported") SP[++p] = f
    if ($3 == "rejected")  RJ[++q] = f }
  END {
    bad = 0
    for (i = 1; i <= ns; i++) { hit = 0
      for (j = 1; j <= p; j++) if (index(SP[j], S[i])) hit = 1
      if (!hit) { printf "  x envelope garment `%s` (%s) has no supported row\n", S[i], SR[i]; bad++ } }
    for (i = 1; i <= nj; i++) { hit = 0
      for (j = 1; j <= q; j++) if (index(RJ[j], J[i])) hit = 1
      if (!hit) { printf "  x construction roadmap §3.2 refuses (`%s`, %s) has no rejected row\n", J[i], JR[i]; bad++ } }
    printf "COUNT %d\n", bad
  }' "$ROWS")
m5n=${m5##*COUNT }; [ -n "${m5%%COUNT*}" ] && printf '%s' "${m5%%COUNT*}"
fails=$((fails + m5n))
echo "  envelope garments: $(grep -c . "$ENVELOPE_SUPPORTED") · named refusals: $(grep -c . "$ENVELOPE_REJECTED") · breaches: $m5n"

# ── M6 the gate column names a real gate ──────────────────────────────────────────────────────
echo "-- M6 every gate cell names a real gate, track, or the recorded gap"
m6=$(awk -F'\t' -v gates="$GATES" '
  BEGIN { while ((getline l < gates) > 0) G[l] = 1; close(gates); bad = 0 }
  {
    cell = $5; ok = 0
    if (index(cell, "unnamed (D32)")) ok = 1
    s = cell
    while (match(s, /(G[0-9]|V[12])/)) {
      g = substr(s, RSTART, RLENGTH); s = substr(s, RSTART + RLENGTH)
      if (g in G) ok = 1
      else { printf "  x %s names `%s`, which roadmap §11 has no heading for\n", cell, g; bad++ }
    }
    if (!ok) { printf "  x `%s` (%s) names no gate, track or recorded gap\n", cell, $2; bad++ }
  }
  END { printf "COUNT %d\n", bad }' "$ROWS")
m6n=${m6##*COUNT }; [ -n "${m6%%COUNT*}" ] && printf '%s' "${m6%%COUNT*}"
fails=$((fails + m6n))
echo "  real gates in roadmap §11: $(grep -c . "$GATES") · breaches: $m6n"

# ── M7 every link in the chapter resolves ─────────────────────────────────────────────────────
echo "-- M7 every link in the chapter resolves"
m7=$(awk -v mat="$MATRIX" -v specdir="$SPEC_DIR" -v heads="$HEADINGS" '
  BEGIN { while ((getline l < heads) > 0) { split(l, a, "\t"); H[a[1] "\t" a[2]] = 1 } close(heads); bad = 0 }
  {
    rest = $0
    while (match(rest, /\[[^]]*\]\([^)]*\)/)) {
      link = substr(rest, RSTART, RLENGTH); rest = substr(rest, RSTART + RLENGTH)
      match(link, /\(([^)]*)\)/); path = substr(link, RSTART + 1, RLENGTH - 2)
      if (path ~ /^https?:/) continue
      target = specdir "/" path
      gsub(/\/[^\/]*\/\.\.\//, "/", target)
      if (system("test -f \"" target "\"") != 0) {
        printf "  x %s:%d links to %s, which does not exist\n", mat, NR, path; bad++; continue
      }
      if (match(link, /§/)) {
        c = substr(link, RSTART + 2)
        if (match(c, /^[0-9]+(\.[0-9]+)*/)) {
          clause = substr(c, RSTART, RLENGTH)
          if (!((target "\t" clause) in H)) {
            printf "  x %s:%d cites §%s, which %s has no heading for\n", mat, NR, clause, path; bad++
          }
        }
      }
    }
  }
  END { printf "COUNT %d\n", bad }' "$MATRIX")
m7n=${m7##*COUNT }; [ -n "${m7%%COUNT*}" ] && printf '%s' "${m7%%COUNT*}"
fails=$((fails + m7n))
echo "  dead links: $m7n"

# ── A1 advisory: the recorded gap stays visible ───────────────────────────────────────────────
gap=$(awk -F'\t' '$5 ~ /unnamed \(D32\)/ { n++; printf "    · %s (%s)\n", $2, $3 } END { printf "COUNT %d\n", n+0 }' "$ROWS")
gapn=${gap##*COUNT }
echo "-- A1 advisory: rows whose proof no gate has accepted (defect D32)"
[ -n "${gap%%COUNT*}" ] && printf '%s' "${gap%%COUNT*}"
echo "  D32 rows: $gapn — reported on every run so the gap is closed by a decision, not by being forgotten"

# ── A2 advisory: feature names the glossary does not carry ────────────────────────────────────
echo "-- A2 advisory: feature names that are not glossary terms (the glossary absorbs them, or a human says why not)"
awk -F'\t' -v terms="$TERMS" '
  BEGIN { while ((getline l < terms) > 0) T[l] = 1; close(terms) }
  { f = tolower($2); gsub(/^a /, "", f)
    if (!(f in T)) { n++; if (n <= 12) printf "    · %s\n", $2 } }
  END { printf "  feature rows: %d · not a glossary term: %d (a compound feature name is expected here; a new DOMAIN term is not)\n", NR, n+0 }' "$ROWS"

echo "feature-matrix census: $rows_total rows / $diag_total diagnostics / $fails failure(s)"
[ "$fails" -eq 0 ] || exit 1
exit 0
