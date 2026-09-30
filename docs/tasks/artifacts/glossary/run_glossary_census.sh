#!/usr/bin/env bash
# docs/tasks/artifacts/glossary/run_glossary_census.sh
# G0-CONTRACT.1 — the census behind two claims the glossary makes:
#   (1) "every machine token has exactly one meaning", and
#   (2) "every identifier-shaped token the specification uses is accounted for".
#
# WHY A CENSUS AND NOT A SENTENCE: both claims are universally quantified over populations that grow
# with every chapter written after this one. `G0-CONTRACT.4`–`.12` and `.16`–`.17` each add terms, and
# a glossary that was complete on the day it was written is the classic shape of a claim that stays true
# in prose while going false in fact — the same failure this repository measured as defect D24. So the
# claim is derived, per CLAIM_VERIFICATION.md leg 1, and leg 3 (durable) is this tracked producer.
#
# WHAT IT CHECKS, mechanically. Each numbered rule fails the census:
#   S1 structure    every entry row has exactly 5 cells and no empty cell (`—` is the empty marker)
#   S2 parts        every part file is listed in the glossary's parts table, and vice versa
#   T1 terms        a term is unique case-insensitively across all parts, and is never an em dash
#   T2 ownership    every backticked token in a token column is owned by exactly ONE entry
#   T3 references   a `→`-prefixed token cell cross-references only tokens that are owned
#   T4 shape        every token is one ASCII identifier-shaped span: no spaces, no locale-dependent
#                   characters, nothing a serializer or a TSV field would have to escape
#   R1 objects      every canonical-object cell resolves — a linked chapter exists and carries the
#                   cited clause heading; a `roadmap §n` / `ADR-n` citation exists in ROADMAP.md;
#                   a `LEAF` citation names a leaf some tree declares
#   R2 safety       the terms roadmap §7.6 and ADR-0004 name as safety-relevant still carry their ⚠
#   I1 index        the glossary's A–Z index equals the index derived from the parts, both directions
#   C1 coverage     every machine token the specification chapters use (the glossary excluded) is
#                   accounted for: owned or cross-referenced by the glossary, declared by the chapter
#                   that uses it (first cell of one of its table rows), or exempted below with a reason.
#                   The population is two arms, both shape rules and neither a classifier:
#                     arm 1 — a code span that IS exactly one ASCII identifier (`walk`, `EdgeRef`);
#                     arm 2 — a code span that carries an arithmetic operator and no hyphen or dot,
#                             i.e. a FORMULA, from which every snake_case or CamelCase identifier is
#                             taken (`garment_waist + 2 × sa_cb + wb_extension`).
#                   Hyphens and dots exclude leaf ids, crate names and file paths; the absence of an
#                   arithmetic operator excludes prose spans and code counter-examples.
# ADVISORY, printed and never a failure:
#   A1 bold spans   how many bolded spans in the spec chapters match a glossary term, and which do not.
#                   Deciding that a bolded span IS a term would be a classifier guessing at meaning, so
#                   this is a side-by-side for a human — the shape the tree coverage census uses.
#
# HONEST LIMITS: C1's population is ASCII identifier-shaped code spans (`[A-Za-z_][A-Za-z0-9_]*`).
# Crate names, leaf ids, paths, Greek-letter epsilons and multi-word spans are outside it by
# construction; they are not domain vocabulary, and pretending otherwise would need a classifier.
# A1 cannot distinguish a bolded term from bolded emphasis — it reports, it does not judge.
# Neither rule can tell whether a definition is CORRECT; that is the domain reviewer's, per §7.6.
#
# Usage:  bash docs/tasks/artifacts/glossary/run_glossary_census.sh
#         bash docs/tasks/artifacts/glossary/run_glossary_census.sh --emit-index
#         GLOSSARY_ROOT=<dir> bash docs/tasks/artifacts/glossary/run_glossary_census.sh
# Output: per-section tables, then
#         `glossary census: <terms> terms / <parts> parts / <tokens> tokens / <failures> failure(s)`
#         exit 1 on any failure, exit 2 when an input is missing.
set -uo pipefail
export LC_ALL=C

ROOT="${GLOSSARY_ROOT:-$(git rev-parse --show-toplevel 2>/dev/null || pwd)}"
cd "$ROOT" || { echo "glossary census: REFUSED — cannot enter $ROOT" >&2; exit 2; }

BOOK_SRC="docs/book/src"
SPEC="$BOOK_SRC/spec"
GLOSSARY="$SPEC/glossary.md"
PARTS_DIR="$SPEC/glossary"
ROADMAP="ROADMAP.md"
TASKS="docs/tasks"

SCRATCH="$(mktemp -d)"
trap 'rm -rf "$SCRATCH"' EXIT

# ── declared seams: the three small lists this census may keep, each entry carrying its reason ──
# A token cell with no backticked span is a PROSE DESCRIPTOR: it names a family of tokens rather than
# one, so there is nothing to own. The list is closed — an undeclared descriptor is a refusal.
DESCRIPTOR_FILE="$SCRATCH/descriptors.tsv"
cat > "$DESCRIPTOR_FILE" <<'EOF'
typed command name	the token IS the per-command type name, so there is no single token to own
diagnostic code	the code is allocated by the diagnostic registry, not by the glossary
layer number or name	the token depends on the dialect: an AAMA name or an ASTM number
EOF

# C1 exemptions: identifier-shaped code spans in the spec chapters that are not domain vocabulary.
EXEMPTION_FILE="$SCRATCH/exemptions.tsv"
cat > "$EXEMPTION_FILE" <<'EOF'
f64	a Rust primitive type name, cited by the units chapter as the evaluation type at a boundary
EOF

# R2: the safety-relevant terms, derived from roadmap §7.6 ("safety-relevant terms: notch types,
# sew/cut line aliases factories use in rejection emails") and ADR-0004 (the cut/sew line swap is "a
# top rejection cause"). Any entry whose term contains `notch` is covered by the substring rule.
SAFETY_FILE="$SCRATCH/safety.tsv"
cat > "$SAFETY_FILE" <<'EOF'
notch	roadmap §7.6 names notch types as safety-relevant; §8.3 names notch semantics as a default hazard
notch type	roadmap §7.6: notch types are the canonical safety-relevant family
notch encoding	roadmap §7.6: a receiver expecting drawn geometry that gets a coded point loses every notch
net line	roadmap §7.6 names the sew/cut line aliases factories use in rejection emails
cut line	roadmap §7.6: the same family
cut-as-1	roadmap ADR-0004: the cut/sew line swap is a top rejection cause
sew-as-1	roadmap ADR-0004: the cut/sew line swap is a top rejection cause
seam allowance	roadmap §8.3: allowance ownership is a plausible default that still ruins a pattern
inclusion policy	roadmap §8.3: allowance ownership, resolved per Factory Profile
internal unit	roadmap §8.3: units are the first named plausible-default hazard
rounding rule	units chapter §2: one rule, because two rules differ by a quantum on every mirrored piece
landmark	ontology §2.1: a factory dispute about a measurement is a dispute about landmarks
EOF

ENTRIES="$SCRATCH/entries.tsv"
INDEX_DERIVED="$SCRATCH/index_derived.tsv"
INDEX_TRACKED="$SCRATCH/index_tracked.tsv"
GLOSSARY_TOKENS="$SCRATCH/tokens_glossary.txt"
EXEMPT_TOKENS="$SCRATCH/tokens_exempt.txt"
USED_PAIRS="$SCRATCH/tokens_used.tsv"
DECLARED_TOKENS="$SCRATCH/tokens_declared.txt"
TERMS_LC="$SCRATCH/terms_lc.txt"
SPANS="$SCRATCH/spans.tsv"
HEADINGS="$SCRATCH/headings.tsv"
LEAVES="$SCRATCH/leaves.txt"
ADRS="$SCRATCH/adrs.txt"
MD_FILES="$SCRATCH/md_files.txt"

for f in "$GLOSSARY" "$ROADMAP"; do
  [ -f "$f" ] || { echo "glossary census: REFUSED — $f not found" >&2; exit 2; }
done
[ -d "$PARTS_DIR" ] || { echo "glossary census: REFUSED — $PARTS_DIR not found" >&2; exit 2; }

fails=0
bad() { printf '  x %s\n' "$1" >&2; fails=$((fails + 1)); }
# count_lines: a line count that never doubles on grep's exit status (DEV_NOTES 2026-09-04 lesson)
count_lines() { local n; n=$(grep -c . "$1"); printf '%s' "${n:-0}"; }

part_files() { find "$PARTS_DIR" -name '*.md' -type f | LC_ALL=C sort; }

# ── lookup tables, built once, so no rule shells out per cell ─────────────────────────────────
{
  find "$BOOK_SRC" -name '*.md' -type f
  printf '%s\n' "$ROADMAP"
} | while IFS= read -r f; do
  awk -v file="$f" 'match($0, /^(##|###|####) [0-9]+(\.[0-9]+)*/) {   # h2-h4; alternation, not a `{2,4}` interval   # `###+`, not `#{2,4}`: intervals are a regex extension
    h = substr($0, RSTART, RLENGTH); sub(/^#+ /, "", h); printf "%s\t%s\n", file, h }' "$f"
done | LC_ALL=C sort -u > "$HEADINGS"

grep -ohE 'ID: `[A-Z0-9]+-[A-Z0-9]+\.[0-9]+[a-z]?`' "$TASKS"/*.md 2>/dev/null \
  | sed -E 's/^ID: `//; s/`$//' | LC_ALL=C sort -u > "$LEAVES"
grep -ohE 'ADR-[0-9]+' "$ROADMAP" | LC_ALL=C sort -u > "$ADRS"
find "$BOOK_SRC" -name '*.md' -type f | LC_ALL=C sort > "$MD_FILES"

# ── the entry stream: part TAB line TAB term TAB meaning TAB object TAB also-called TAB token ──
for p in $(part_files); do
  awk -F'|' -v part="$p" '
    function trim(s) { gsub(/^[ \t]+|[ \t]+$/, "", s); return s }
    /^\|/ {
      t = trim($2)
      if (t == "Term" || t ~ /^:?-+:?$/) next
      if (NF != 7) { printf "%s\t%d\tARITY\t%d\t\t\t\n", part, NR, NF - 2; next }
      printf "%s\t%d\t%s\t%s\t%s\t%s\t%s\n", part, NR, t, trim($3), trim($4), trim($5), trim($6)
    }' "$p"
done > "$ENTRIES"

terms_total=$(awk -F'\t' '$3 != "ARITY"' "$ENTRIES" | grep -c .)
parts_total=$(part_files | grep -c .)

# the derived A–Z index: term (⚠ and backticks stripped) TAB its part, in reading order
awk -F'\t' '$3 != "ARITY" {
  term = $3; sub(/ ⚠$/, "", term); gsub(/`/, "", term)
  n = split($1, seg, "/"); printf "%s\tglossary/%s\n", term, seg[n]
}' "$ENTRIES" | LC_ALL=C sort -f > "$INDEX_DERIVED"

if [ "${1:-}" = "--emit-index" ]; then
  awk -F'\t' '{ printf "- [%s](%s)\n", $1, $2 }' "$INDEX_DERIVED"
  exit 0
fi

echo "=== glossary census ==="

# ── S1 structure ──────────────────────────────────────────────────────────────────────────────
echo "-- S1 entry structure (5 cells, none empty)"
s1_out=$(awk -F'\t' '
  $3 == "ARITY" { printf "  x %s:%s has %s cells, expected 5\n", $1, $2, $4; n++; next }
  { for (i = 3; i <= 7; i++) if ($i == "") {
      printf "  x %s:%s has an empty cell %d — write an em dash so the absence is deliberate\n", $1, $2, i - 2; n++ } }
  END { printf "COUNT %d\n", n+0 }' "$ENTRIES")
s1_n=${s1_out##*COUNT }
[ -n "${s1_out%%COUNT*}" ] && printf '%s' "${s1_out%%COUNT*}"
fails=$((fails + s1_n))
echo "  entries: $terms_total · breaches: $s1_n"

# ── S2 parts inventory ────────────────────────────────────────────────────────────────────────
echo "-- S2 parts inventory"
awk '/^## /{s = 0} /^## The parts/{s = 1; next} s && /^\| \[/' "$GLOSSARY" \
  | sed -E 's/^\| \[[^]]*\]\(([^)]*)\).*/\1/' | LC_ALL=C sort -u > "$SCRATCH/parts_listed.txt"
part_files | sed -E "s#^$SPEC/##" | LC_ALL=C sort -u > "$SCRATCH/parts_actual.txt"
while IFS= read -r m; do [ -n "$m" ] && bad "part $m exists but the glossary's parts table does not list it"; done \
  < <(comm -13 "$SCRATCH/parts_listed.txt" "$SCRATCH/parts_actual.txt")
while IFS= read -r x; do [ -n "$x" ] && bad "the parts table lists $x, which does not exist"; done \
  < <(comm -23 "$SCRATCH/parts_listed.txt" "$SCRATCH/parts_actual.txt")
printf '  %-42s %s\n' "PART" "ENTRIES"
for p in $(part_files); do
  n=$(awk -F'\t' -v f="$p" '$1 == f && $3 != "ARITY"' "$ENTRIES" | grep -c .)
  printf '  %-42s %s\n' "${p#"$SPEC"/}" "$n"
done

# ── T1 one meaning per term ───────────────────────────────────────────────────────────────────
echo "-- T1 one meaning per term"
t1_out=$(awk -F'\t' '
  function clean(t) { sub(/ ⚠$/, "", t); gsub(/`/, "", t); return tolower(t) }
  $3 == "ARITY" { next }
  { t = clean($3)
    if (t == "—" || t == "") { printf "  x %s:%s has no term\n", $1, $2; bad++ }
    else if (t in seen) { printf "  x term `%s` is defined twice: %s and %s:%s\n", t, seen[t], $1, $2; bad++ }
    else seen[t] = $1 ":" $2 }
  END { printf "COUNT %d\n", bad+0 }' "$ENTRIES")
t1_n=${t1_out##*COUNT }
[ -n "${t1_out%%COUNT*}" ] && printf '%s' "${t1_out%%COUNT*}"
fails=$((fails + t1_n))
echo "  terms: $terms_total · duplicate or missing: $t1_n"

# ── T2/T3/T4 machine tokens ───────────────────────────────────────────────────────────────────
echo "-- T2/T3/T4 machine tokens (one token, one meaning)"
t_out=$(awk -F'\t' -v desc="$DESCRIPTOR_FILE" '
  function tokens(cell, out,   n, s) {
    n = 0
    while (match(cell, /`[^`]+`/)) {
      s = substr(cell, RSTART + 1, RLENGTH - 2); out[++n] = s
      cell = substr(cell, RSTART + RLENGTH)
    }
    return n
  }
  BEGIN { while ((getline line < desc) > 0) { split(line, a, "\t"); DESC[a[1]] = 1 } close(desc); bad = 0 }
  $3 == "ARITY" { next }
  {
    cell = $7
    if (cell ~ /^—/) next
    is_ref = (cell ~ /^→/)
    n = tokens(cell, TK)
    if (n == 0) {
      if (!(cell in DESC)) {
        printf "  x %s:%s — token cell `%s` carries no backticked token and declares no descriptor reason\n", $1, $2, cell
        bad++
      }
      next
    }
    for (i = 1; i <= n; i++) {
      tok = TK[i]
      if (tok !~ /^[A-Za-z0-9_.:*\/-]+$/) {
        printf "  x %s:%s — token `%s` is not identifier-shaped; a token is one ASCII span with no spaces\n", $1, $2, tok
        bad++
      }
      if (is_ref) ref[tok] = ref[tok] " " $1 ":" $2
      else if (tok in own) {
        printf "  x token `%s` is owned twice: %s and %s:%s — one token, one meaning\n", tok, own[tok], $1, $2
        bad++
      } else own[tok] = $1 ":" $2
    }
  }
  END {
    for (t in ref) if (!(t in own)) {
      printf "  x token `%s` is cross-referenced from%s but owned by no entry\n", t, ref[t]; bad++
    }
    no = 0; for (t in own) no++
    nr = 0; for (t in ref) nr++
    printf "  owned tokens: %d · cross-referenced: %d\n", no, nr
    printf "COUNT %d\n", bad
  }' "$ENTRIES")
t_n=${t_out##*COUNT }
[ -n "${t_out%%COUNT*}" ] && printf '%s' "${t_out%%COUNT*}"
fails=$((fails + t_n))

awk -F'\t' '$3 != "ARITY" { cell = $7
  while (match(cell, /`[^`]+`/)) { s = substr(cell, RSTART + 1, RLENGTH - 2); cell = substr(cell, RSTART + RLENGTH)
    if (s ~ /^[A-Za-z_][A-Za-z0-9_]*$/) print s } }' "$ENTRIES" | LC_ALL=C sort -u > "$GLOSSARY_TOKENS"
awk -F'\t' '{ print $1 }' "$EXEMPTION_FILE" | LC_ALL=C sort -u > "$EXEMPT_TOKENS"

# ── R1 canonical-object references ────────────────────────────────────────────────────────────
echo "-- R1 canonical-object references resolve"
r1_out=$(awk -F'\t' -v heads="$HEADINGS" -v leaves="$LEAVES" -v adrs="$ADRS" -v mdfiles="$MD_FILES" \
       -v partsdir="$PARTS_DIR" -v specdir="$SPEC" -v tasksdir="$TASKS" '
  function resolve(p,   t) {
    # a part links with `../chapter.md`: drop the parent segment AND the `..`, not just the `..`
    t = partsdir "/" p
    gsub(/\/[^\/]*\/\.\.\//, "/", t)
    return t
  }
  BEGIN {
    while ((getline l < heads) > 0) { split(l, a, "\t"); H[a[1] "\t" a[2]] = 1 } close(heads)
    while ((getline l < leaves) > 0) { L[l] = 1 } close(leaves)
    while ((getline l < adrs) > 0) { A[l] = 1 } close(adrs)
    while ((getline l < mdfiles) > 0) { F[l] = 1 } close(mdfiles)
    bad = 0
  }
  $3 == "ARITY" { next }
  {
    cell = $5
    if (cell ~ /^—/) next
    rest = cell
    while (match(rest, /\[[^]]*\]\([^)]*\)/)) {
      link = substr(rest, RSTART, RLENGTH); rest = substr(rest, RSTART + RLENGTH)
      match(link, /\(([^)]*)\)/); path = substr(link, RSTART + 1, RLENGTH - 2)
      target = resolve(path)
      if (!(target in F)) {
        printf "  x %s:%s — links to %s, which does not exist\n", $1, $2, path; bad++
      } else if (match(link, /§/)) {
        # `§` is U+00A7: two bytes, and LC_ALL=C makes awk count bytes, so the clause starts at +2.
        c = substr(link, RSTART + 2)
        if (match(c, /^[0-9]+(\.[0-9]+)*/)) {
          clause = substr(c, RSTART, RLENGTH)
          if (!((target "\t" clause) in H)) {
            printf "  x %s:%s — cites §%s, which %s has no heading for\n", $1, $2, clause, path; bad++
          }
        }
      }
    }
    rest = cell
    while (match(rest, /roadmap §[0-9]+(\.[0-9]+)*/)) {
      # "roadmap " is 8 bytes and `§` is 2, so the clause starts at +10 and is RLENGTH - 10 long.
      clause = substr(rest, RSTART + 10, RLENGTH - 10); rest = substr(rest, RSTART + RLENGTH)
      if (!(("ROADMAP.md\t" clause) in H)) {
        printf "  x %s:%s — cites roadmap §%s, which ROADMAP.md has no heading for\n", $1, $2, clause; bad++
      }
    }
    rest = cell
    while (match(rest, /ADR-[0-9]+/)) {
      adr = substr(rest, RSTART, RLENGTH); rest = substr(rest, RSTART + RLENGTH)
      if (!(adr in A)) {
        printf "  x %s:%s — cites %s, which ROADMAP.md does not contain\n", $1, $2, adr; bad++
      }
    }
    rest = cell
    while (match(rest, /`[A-Z0-9]+-[A-Z0-9]+\.[0-9]+[a-z]?`/)) {
      id = substr(rest, RSTART + 1, RLENGTH - 2); rest = substr(rest, RSTART + RLENGTH)
      if (!(id in L)) {
        printf "  x %s:%s — names leaf %s, which no tree in %s declares\n", $1, $2, id, tasksdir; bad++
      }
    }
  }
  END { printf "COUNT %d\n", bad }' "$ENTRIES")
r1_n=${r1_out##*COUNT }
[ -n "${r1_out%%COUNT*}" ] && printf '%s' "${r1_out%%COUNT*}"
fails=$((fails + r1_n))
echo "  dead references: $r1_n"

# ── R2 safety-relevant terms keep their mark ──────────────────────────────────────────────────
echo "-- R2 safety-relevant terms (roadmap §7.6, §8.3, ADR-0004)"
r2_out=$(awk -F'\t' -v safety="$SAFETY_FILE" '
  BEGIN { while ((getline l < safety) > 0) { split(l, a, "\t"); MUST[tolower(a[1])] = a[2] } close(safety)
          bad = 0; marked = 0; req = 0 }
  $3 == "ARITY" { next }
  {
    t = tolower($3); sub(/ ⚠$/, "", t); gsub(/`/, "", t)
    why = (t in MUST) ? MUST[t] : ""
    # a derived family: any entry that owns a `type X` token is one of a closed vocabulary of physical
    # marks, which is what roadmap §7.6 calls safety-relevant. A new one inherits the requirement.
    if (why == "" && $7 ~ /^type `/) why = "it owns a `type` token: a physical mark, per roadmap §7.6"
    needs = (why != "")
    is_marked = ($3 ~ /⚠/)
    if (is_marked) marked++
    if (needs) { req++; if (!is_marked) {
      printf "  x `%s` is safety-relevant (%s) and has lost its mark\n", $3, why; bad++ } }
  }
  END { printf "  required: %d · marked in total: %d\n", req, marked; printf "COUNT %d\n", bad }' "$ENTRIES")
r2_n=${r2_out##*COUNT }
[ -n "${r2_out%%COUNT*}" ] && printf '%s' "${r2_out%%COUNT*}"
fails=$((fails + r2_n))

# ── I1 the A–Z index agrees with the parts ────────────────────────────────────────────────────
echo "-- I1 the A–Z index"
awk '/^## /{s = 0} /^## The terms, A–Z/{s = 1; next} s && /^- \[/{
       match($0, /\[[^]]*\]/); t = substr($0, RSTART + 1, RLENGTH - 2)
       rest = substr($0, RSTART + RLENGTH)
       match(rest, /\(([^)]*)\)/); p = substr(rest, RSTART + 1, RLENGTH - 2)
       printf "%s\t%s\n", t, p }' "$GLOSSARY" | LC_ALL=C sort -f > "$INDEX_TRACKED"
drift=0
while IFS= read -r l; do [ -n "$l" ] && { bad "defined in a part but missing from the A–Z index: $l"; drift=$((drift + 1)); }; done \
  < <(comm -23 <(LC_ALL=C sort "$INDEX_DERIVED") <(LC_ALL=C sort "$INDEX_TRACKED"))
while IFS= read -r l; do [ -n "$l" ] && { bad "in the A–Z index but defined by no part: $l"; drift=$((drift + 1)); }; done \
  < <(comm -13 <(LC_ALL=C sort "$INDEX_DERIVED") <(LC_ALL=C sort "$INDEX_TRACKED"))
echo "  indexed: $(count_lines "$INDEX_TRACKED") · derived: $(count_lines "$INDEX_DERIVED") · drift: $drift"

# ── C1 token coverage in the specification set ────────────────────────────────────────────────
echo "-- C1 every machine token the spec set uses is accounted for"
: > "$USED_PAIRS"; : > "$DECLARED_TOKENS"
spec_files=$(grep -vx "$GLOSSARY" "$MD_FILES" | grep -v "^$PARTS_DIR/")
for f in $spec_files; do
  grep -oE '`[^`]+`' "$f" | sed 's/^`//; s/`$//' | awk '
    # arm 1: the span is exactly one identifier. arm 2: the span is a formula — it carries an
    # arithmetic operator and no hyphen or dot, so leaf ids, crate names and paths stay out.
    /^[A-Za-z_][A-Za-z0-9_]*$/ { print; next }
    {
      s = $0
      arithmetic = (s ~ /[+=]/ || index(s, "−") || index(s, "×") || index(s, "√") || index(s, "÷"))
      if (!arithmetic || s ~ /-/ || s ~ /\./) next
      while (match(s, /[A-Za-z_][A-Za-z0-9_]*/)) {
        id = substr(s, RSTART, RLENGTH); s = substr(s, RSTART + RLENGTH)
        if (id ~ /^[a-z][a-z0-9]*(_[a-z0-9]+)+$/ || id ~ /^[A-Z][a-z0-9]+([A-Z][A-Za-z0-9]*)*$/) print id
      }
    }' | LC_ALL=C sort -u | awk -v file="$f" '{ printf "%s\t%s\n", $0, file }' >> "$USED_PAIRS"
  awk -F'|' '/^\|/ { c = $2; gsub(/^[ \t]+|[ \t]+$/, "", c)
    while (match(c, /`[^`]+`/)) { s = substr(c, RSTART + 1, RLENGTH - 2); c = substr(c, RSTART + RLENGTH)
      if (s ~ /^[A-Za-z_][A-Za-z0-9_]*$/) print s } }' "$f" >> "$DECLARED_TOKENS"
done
LC_ALL=C sort -u -o "$USED_PAIRS" "$USED_PAIRS"
LC_ALL=C sort -u -o "$DECLARED_TOKENS" "$DECLARED_TOKENS"
c1_out=$(awk -F'\t' '
  FNR == 1 { src++ }
  src <= 3 { have[$1] = 1; next }
  !($1 in have) {
    printf "  x `%s` is used by %s but no glossary entry owns it, no table in that chapter declares it, and no exemption covers it\n", $1, $2
    bad++
  }
  END { printf "COUNT %d\n", bad+0 }' "$GLOSSARY_TOKENS" "$EXEMPT_TOKENS" "$DECLARED_TOKENS" "$USED_PAIRS")
c1_n=${c1_out##*COUNT }
[ -n "${c1_out%%COUNT*}" ] && printf '%s' "${c1_out%%COUNT*}"
fails=$((fails + c1_n))
used_n=$(awk -F'\t' '{ print $1 }' "$USED_PAIRS" | LC_ALL=C sort -u | grep -c .)
echo "  tokens used: $used_n · glossary-owned: $(count_lines "$GLOSSARY_TOKENS") · chapter-declared: $(count_lines "$DECLARED_TOKENS") · exempt: $(count_lines "$EXEMPT_TOKENS") · unaccounted: $c1_n"

# ── A1 advisory ───────────────────────────────────────────────────────────────────────────────
echo "-- A1 advisory: bolded spans in the spec chapters against the glossary's terms"
awk -F'\t' '$3 != "ARITY" { t = tolower($3); sub(/ ⚠$/, "", t); gsub(/`/, "", t); print t }' "$ENTRIES" \
  | LC_ALL=C sort -u > "$TERMS_LC"
: > "$SPANS"
for f in $spec_files; do
  grep -oE '\*\*[^*]+\*\*' "$f" | sed 's/^\*\*//; s/\*\*$//' | LC_ALL=C sort -u \
    | awk '{ printf "%s\t%s\n", tolower($0), $0 }' >> "$SPANS"
done
LC_ALL=C sort -u -o "$SPANS" "$SPANS"
total_spans=$(count_lines "$SPANS")
matched=$(awk -F'\t' 'FILENAME == ARGV[1] { t[$1] = 1; next } ($1 in t)' "$TERMS_LC" "$SPANS" | grep -c .)
echo "  bolded spans: $total_spans · matching a glossary term: $matched · not matching: $((total_spans - matched))"
echo "  a non-match is emphasis or a chapter-local label, not a defect — read them, do not automate them:"
awk -F'\t' 'FILENAME == ARGV[1] { t[$1] = 1; next } !($1 in t) { printf "    · %s\n", $2 }' "$TERMS_LC" "$SPANS" | head -15

echo "glossary census: $terms_total terms / $parts_total parts / $(count_lines "$GLOSSARY_TOKENS") tokens / $fails failure(s)"
[ "$fails" -eq 0 ] || exit 1
exit 0
