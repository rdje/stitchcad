#!/usr/bin/env bash
# scripts/check_live_doc_size.sh — LIVE-DOC-SIZE (PROJECT doctrine).
#
# THE RULE: every live document in this repository is classified in the containment data plane
# (.doctrine/live_document_size/surfaces.tsv) with an owner, a lifecycle class, a health target and an
# INCLUSIVE enforcement ceiling; every route a bounded surface emits ends at a classified destination
# (routes.tsv); and the resulting TREE — not the staged diff — is inside those bounds.
#
# It refuses on: an unclassified tracked Markdown surface; a malformed registry row (field count,
# empty owner/authority, unknown lifecycle or kind, non-numeric bound, a ceiling below its health
# target); an absolute or off-volume path in the data plane; a path/glob that matches nothing when the
# row does not declare itself empty; a line / byte / max-content-line / file-count / aggregate overflow
# past a ceiling; a widened transition-debt baseline; a debt baseline measured at a REVISION the file no
# longer declares (`at=<token>`, which is what makes a baseline revision-aware instead of frozen); and a
# route whose destination has no surface row or contradicts its lifecycle. It WARNS without failing at
# 80% of a health target.
#
# WHY: a bounded file that routes its overflow into an unbounded neighbour is not contained — the
# measured upstream failure was a status file that reached 1 547 057 bytes after a README cap displaced
# the pressure instead of removing it. See LIVE_DOCUMENT_SIZE_CONTAINMENT.md (adopted) and
# docs/decisions/decision_live-document-containment-proportionate-adoption.md.
#
# SHAPE: bash expands and MEASURES (lines, bytes, max content-line bytes, deterministically under
# LC_ALL=C); one awk pass EVALUATES every rule against the registry. Keeping them apart is what makes
# the evaluator testable on a synthetic registry (--self-test) without touching the real tree.
#
# HONEST LIMITS: size checks prove boundedness, not semantic truth — a surface claiming currency needs
# its own lifecycle-specific verifier (the KNOWLEDGE-MAP doctrine covers the derived map). This check
# cannot prove a ceiling is RIGHT, only that it is declared, derived from a stated measurement, and not
# exceeded. `--self-test` proves the refusal classes fire; it cannot prove the registry is complete
# against surfaces nobody thought of — that is what the coverage rule is for.
#
# CONTRACT (DOCTRINE_ENFORCEMENT.md): exit code is the verdict; explains on stderr; deterministic;
# read-only; fast; unconditional (it judges the tree, not the staged set).
set -uo pipefail
export LC_ALL=C
ROOT="${LIVE_DOC_SIZE_ROOT:-$(git rev-parse --show-toplevel)}"
cd "$ROOT" || exit 1

REGISTRY="${LIVE_DOC_SIZE_REGISTRY:-.doctrine/live_document_size}"
SURFACES="$REGISTRY/surfaces.tsv"
ROUTES="$REGISTRY/routes.tsv"
SCRATCH="$ROOT/target/doctrine_scratch/live_doc_size"
LIFECYCLES='bounded_snapshot partitioned_canonical maintained_reference generated_projection rolling_ledger archive_terminal external_terminal frozen_legacy'

for f in "$SURFACES" "$ROUTES"; do
  if [ ! -f "$f" ]; then
    echo "LIVE-DOC-SIZE: REFUSED — $f is missing; the data plane is the authority this check reads." >&2
    exit 2
  fi
done

# ── measure: emit "sid <TAB> path <TAB> lines <TAB> bytes <TAB> maxline" per part ─────────────
measure_registry() { # $1 = surfaces.tsv · $2 = output file
  local out="$2" parts sid glob kind p l b m
  : > "$out.parts"
  while IFS=$'\t' read -r sid glob kind; do
    case "$sid" in ''|'#'*) continue ;; esac
    case "$kind" in
      file)       printf '%s\t%s\n' "$sid" "$glob" >> "$out.parts" ;;
      list)       printf '%s\n' "$glob" | tr ';' '\n' | sed '/^$/d' \
                    | while IFS= read -r p; do printf '%s\t%s\n' "$sid" "$p"; done >> "$out.parts" ;;
      collection|binary_collection) git -C "$ROOT" ls-files -- "$glob" 2>/dev/null \
                    | while IFS= read -r p; do printf '%s\t%s\n' "$sid" "$p"; done >> "$out.parts" ;;
      external)   : ;;
      *)          printf '%s\t%s\n' "$sid" "$glob" >> "$out.parts" ;;
    esac
  done < <(grep -vE '^(#|$)' "$1" | cut -f1-3)
  : > "$out"
  while IFS=$'\t' read -r sid p; do
    if [ -f "$ROOT/$p" ]; then
      b=$(wc -c < "$ROOT/$p" | tr -d ' ')
      if awk -F'\t' -v id="$sid" '$1==id && $3=="binary_collection" {found=1} END{exit !found}' "$1"; then
        l=0; m=0; fl=""
      else
        l=$(wc -l < "$ROOT/$p" | tr -d ' ')
        m=$(awk '{n=length($0); if(n>x)x=n} END{print x+0}' "$ROOT/$p")
        fl=$(head -1 "$ROOT/$p" | tr '\t' ' ')
      fi
    else
      l=0; b=0; m=0; fl=""
    fi
    printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$sid" "$p" "$l" "$b" "$m" "$fl" >> "$out"
  done < "$out.parts"
  rm -f "$out.parts"
}

# ── evaluate: one awk pass over registry + routes + measurements + tracked list ───────────────
evaluate() { # $1 surfaces · $2 routes · $3 measurements · $4 tracked-md list
  awk -F'\t' -v lifecycles="$LIFECYCLES" '
    function bad(m)  { printf "LIVE-DOC-SIZE: %s\n", m > "/dev/stderr"; fails++ }
    function warn(m) { printf "live-doc-size: WARNING %s\n", m > "/dev/stderr"; warns++ }
    function num(x)  { return (x ~ /^[0-9]+$/) }
    function pct(a,h){ return (h+0 > 0) ? (100 * a / h) : 0 }
    BEGIN{ fails=0; warns=0; ns=0; nr=0; nt=0; nm=0
           split(lifecycles, A, " "); for (i in A) OKLC[A[i]]=1 }

    FILENAME == ARGV[1] {                                  # ---- surfaces registry
      if ($0 ~ /^#/ || $0 ~ /^[[:space:]]*$/) next
      ns++
      if (NF != 21) { bad(sprintf("surface row `%s` has %d fields, expected 21", $1, NF)); next }
      sid=$1
      if (sid in SEEN) bad("duplicate surface_id " sid)
      SEEN[sid]=1; KIND[sid]=$3; LIFE[sid]=$4; OWNER[sid]=$5; AUTH[sid]=$6; PATHG[sid]=$2; DEBT[sid]=$20
      HL[sid]=$11; HB[sid]=$12; HM[sid]=$13
      CL[sid]=$14; CB[sid]=$15; CM[sid]=$16; CF[sid]=$17; AL[sid]=$18; AB[sid]=$19
      EMPTY[sid]=($10 == "0") ? 1 : 0
      if ($3 !~ /^(file|list|collection|binary_collection|external)$/) bad(sid ": unknown kind `" $3 "`")
      if (!($4 in OKLC))                              bad(sid ": unknown lifecycle class `" $4 "`")
      if ($5 == "" || $5 == "-")                      bad(sid ": no owner")
      if ($6 == "" || $6 == "-")                      bad(sid ": no authority")
      if ($2 ~ /^\// || $2 ~ /^~/ || $2 ~ /\/(Users|home|private|Volumes)\//)
                                                      bad(sid ": absolute or off-volume path `" $2 "`")
      for (c = 11; c <= 19; c++)
        if ($c != "-" && !num($c)) bad(sprintf("%s: bound column %d is neither a number nor - (`%s`)", sid, c, $c))
      if (num($11) && num($14) && $14+0 < $11+0) bad(sid ": line ceiling " $14 " is below its health target " $11)
      if (num($12) && num($15) && $15+0 < $12+0) bad(sid ": byte ceiling " $15 " is below its health target " $12)
      if (num($13) && num($16) && $16+0 < $13+0) bad(sid ": maxline ceiling " $16 " is below its health target " $13)
      next
    }

    FILENAME == ARGV[2] {                                  # ---- routes registry
      if ($0 ~ /^#/ || $0 ~ /^[[:space:]]*$/) next
      nr++; rid=$1
      if (NF != 8) { bad(sprintf("route `%s` has %d fields, expected 8", rid, NF)); next }
      if ($3 ~ /^\// || $3 ~ /^~/ || $3 ~ /\/(Users|home|private|Volumes)\//)
        bad(rid ": route destination is an absolute or off-volume path")
      if ($5 !~ /^(reader_navigation|author_overflow|both)$/) bad(rid ": unknown route class `" $5 "`")
      RSURF[nr]=rid; RDEST[nr]=$4; RLIFE[nr]=$6
      next
    }

    FILENAME == ARGV[3] {                                  # ---- measurements
      sid=$1; nm++
      if (!(sid in SEEN)) { bad("measurement for an unregistered surface `" sid "`"); next }
      PARTOF[$2]=sid; nparts[sid]++
      if ($3+0 > pl[sid]+0) pl[sid]=$3+0
      if ($4+0 > pb[sid]+0) pb[sid]=$4+0
      if ($5+0 > pm[sid]+0) pm[sid]=$5+0
      if ($6 != "") FIRST[$2]=$6
      al[sid]+=$3+0; ab[sid]+=$4+0
      next
    }

    FILENAME == ARGV[4] { TRACKED[++nt]=$0; next }         # ---- every tracked .md file

    END{
      for (i = 1; i <= nt; i++)
        if (!(TRACKED[i] in PARTOF))
          bad("unclassified live surface: " TRACKED[i] " is tracked and claimed by no registry row")

      for (sid in SEEN) {
        if (KIND[sid] == "external") continue
        p = nparts[sid]+0
        if (p == 0 && EMPTY[sid] != 1)
          bad(sid ": its path/glob matched no file and meas_files is not declared 0 — a silent under-match is how a collection loses its bounds")
        if (num(CF[sid]) && p > CF[sid]+0)          bad(sprintf("%s: %d files exceed the file-count ceiling %d", sid, p, CF[sid]))
        if (num(CL[sid]) && pl[sid] > CL[sid]+0)    bad(sprintf("%s: %d lines exceed the line ceiling %d", sid, pl[sid], CL[sid]))
        if (num(CB[sid]) && pb[sid] > CB[sid]+0)    bad(sprintf("%s: %d bytes exceed the byte ceiling %d", sid, pb[sid], CB[sid]))
        if (num(CM[sid]) && pm[sid] > CM[sid]+0)    bad(sprintf("%s: widest line %d B exceeds the maxline ceiling %d", sid, pm[sid], CM[sid]))
        if (num(AL[sid]) && al[sid] > AL[sid]+0)    bad(sprintf("%s: aggregate %d lines exceed the ceiling %d", sid, al[sid], AL[sid]))
        if (num(AB[sid]) && ab[sid] > AB[sid]+0)    bad(sprintf("%s: aggregate %d bytes exceed the ceiling %d", sid, ab[sid], AB[sid]))
        if (num(HL[sid]) && pct(pl[sid], HL[sid]) >= 80) warn(sprintf("%s: %d lines = %.0f%% of its %d-line health target", sid, pl[sid], pct(pl[sid],HL[sid]), HL[sid]))
        if (num(HB[sid]) && pct(pb[sid], HB[sid]) >= 80) warn(sprintf("%s: %d bytes = %.0f%% of its %d-byte health target", sid, pb[sid], pct(pb[sid],HB[sid]), HB[sid]))
        if (num(HM[sid]) && pct(pm[sid], HM[sid]) >= 80) warn(sprintf("%s: widest line %d B = %.0f%% of its %d B target", sid, pm[sid], pct(pm[sid],HM[sid]), HM[sid]))
        if (DEBT[sid] != "" && DEBT[sid] != "-") {
          nd = split(DEBT[sid], darr, ";")
          for (d = 1; d <= nd; d++) {
            split(darr[d], kv, "=")
            axis = kv[1]; base = kv[2]+0
            # `at=<token>` is not a size axis: it is the REVISION the baselines beside it were measured at,
            # and it is what makes a baseline revision-aware instead of frozen. A stored copy of a
            # mechanically owned value needs an executed freshness oracle — the containment adoption note
            # deferred trigger 3, fired by the first roadmap amendment: if the file no longer declares that
            # revision on its first line, the baseline is stale and the row is REFUSED, not silently
            # satisfied. So a revision may re-base its baseline, but only in the commit that revises, which
            # is where the recorded authority for the growth has to be anyway.
            if (axis == "at") {
              token = substr(darr[d], 4)
              if (KIND[sid] != "file") { bad(sid ": debt axis `at=` needs a kind=file surface, this is `" KIND[sid] "`"); continue }
              f = PATHG[sid]
              if (token == "") { bad(sid ": debt axis `at=` declares no revision token"); continue }
              if (!(f in FIRST)) { bad(sid ": debt axis `at=" token "` but " f " was not measured"); continue }
              if (index(FIRST[f], token) == 0)
                bad(sprintf("%s: its debt baseline was measured at revision `%s`, which %s no longer declares (first line: `%s`) — a revision re-bases its own baseline in the same commit, under a recorded authority", sid, token, f, substr(FIRST[f], 1, 60)))
              continue
            }
            act = (axis=="lines") ? pl[sid] : (axis=="bytes") ? pb[sid] : (axis=="maxline") ? pm[sid] \
                : (axis=="agglines") ? al[sid] : (axis=="aggbytes") ? ab[sid] : -1
            if (act < 0) { bad(sid ": debt axis `" axis "` is not one of lines/bytes/maxline/agglines/aggbytes"); continue }
            if (act > base) bad(sprintf("%s: transition debt WIDENED on %s (%d > baseline %d) — a baseline never grows", sid, axis, act, base))
          }
        }
      }

      for (r = 1; r <= nr; r++) {
        if (!(RDEST[r] in SEEN)) { bad(RSURF[r] ": routes to surface_id `" RDEST[r] "`, which has no registry row — an unclassified sink"); continue }
        if (RLIFE[r] != "" && RLIFE[r] != "-" && RLIFE[r] != LIFE[RDEST[r]])
          bad(RSURF[r] ": lifecycle `" RLIFE[r] "` contradicts the destination surface lifecycle `" LIFE[RDEST[r]] "`")
      }

      if (fails > 0) {
        printf "LIVE-DOC-SIZE: %d breach(es) · %d warning(s) · %d surface row(s) · %d route row(s) · %d file(s) measured\n",
               fails, warns, ns, nr, nm > "/dev/stderr"
        exit 1
      }
      printf "live-doc-size: OK — %d surfaces, %d routes, %d files measured, %d warning(s)\n", ns, nr, nm, warns
      exit 0
    }
  ' "$1" "$2" "$3" "$4"
}

# ── ground truth: synthetic registries, so every refusal class has been seen to fire ──────────
self_test() {
  local d="$SCRATCH/selftest" fails=0 arms=0 rc
  rm -rf "$d"; mkdir -p "$d/reg" "$d/tree"
  printf 'tracked file\n' > "$d/tree/ok.md"

  mkreg() {
    cat > "$d/reg/surfaces.tsv" <<'EOF'
#surface_id	path_or_glob	kind	lifecycle	owner	authority	meas_lines	meas_bytes	meas_maxline	meas_files	health_lines	health_bytes	health_maxline	ceil_lines	ceil_bytes	ceil_maxline	ceil_files	agg_ceil_lines	agg_ceil_bytes	debt	notes
one	tree/ok.md	file	bounded_snapshot	leaf-x	COMMIT.md	1	13	12	-	10	200	100	20	400	200	-	-	-	-	a note
EOF
    cat > "$d/reg/routes.tsv" <<'EOF'
#route_id	emitted_by	destination	surface_id	class	lifecycle	pressure_control	notes
R01	README.md	tree/ok.md	one	reader_navigation	bounded_snapshot	ceilings	a note
EOF
  }
  run() { ( cd "$d" && LIVE_DOC_SIZE_ROOT="$d" LIVE_DOC_SIZE_REGISTRY=reg \
            LIVE_DOC_SIZE_TRACKED="$d/tracked.txt" \
            bash "$ROOT/scripts/check_live_doc_size.sh" >/dev/null 2>&1 ); }
  arm() { # label · expected rc · note
    arms=$((arms+1)); run; rc=$?
    if [ "$rc" = "$2" ]; then printf '  ✓ %-14s exit=%s  %s\n' "$1" "$rc" "$3"
    else printf '  ✗ %-14s exit=%s (wanted %s)  %s\n' "$1" "$rc" "$2" "$3"; fails=$((fails+1)); fi
  }

  printf 'tree/ok.md\n' > "$d/tracked.txt"
  mkreg; arm GREEN-1 0 "a classified, in-bounds surface passes"

  mkreg; printf 'tree/ok.md\ntree/unclaimed.md\n' > "$d/tracked.txt"
  arm RED-COVERAGE 1 "a tracked file claimed by no row is refused"
  printf 'tree/ok.md\n' > "$d/tracked.txt"

  mkreg; sed -i.bak 's/\tbounded_snapshot\t/\tbogus_class\t/' "$d/reg/surfaces.tsv"
  arm RED-LIFECYCLE 1 "an unknown lifecycle class is refused"

  mkreg; sed -i.bak 's/\tleaf-x\t/\t-\t/' "$d/reg/surfaces.tsv"
  arm RED-OWNER 1 "a row with no owner is refused"

  mkreg; awk -F'\t' 'BEGIN{OFS="\t"} (!/^#/ && $1=="one") {$14=1} {print}' "$d/reg/surfaces.tsv" > "$d/reg/s.tmp" \
    && mv "$d/reg/s.tmp" "$d/reg/surfaces.tsv"
  arm RED-CEILING 1 "a line ceiling below the measured size is refused"

  mkreg; sed -i.bak 's#one\ttree/ok.md#one\t/abs/tree/ok.md#' "$d/reg/surfaces.tsv"
  arm RED-PATH 1 "an absolute path in the data plane is refused"

  mkreg; printf 'R02\tREADME.md\ttree/x.md\tnosuch\tboth\trolling_ledger\t-\t-\n' >> "$d/reg/routes.tsv"
  arm RED-ROUTE 1 "a route to an unclassified destination is refused"

  mkreg; sed -i.bak 's/\t-\ta note$/\tlines=1\ta note/' "$d/reg/surfaces.tsv"
  printf 'a much longer tracked file that exceeds its recorded debt baseline by a wide margin\n' >> "$d/tree/ok.md"
  arm RED-DEBT 1 "a widened transition-debt baseline is refused"
  printf 'tracked file\n' > "$d/tree/ok.md"

  # the revision-aware baseline: `at=<token>` must be declared by the file's first line, and declaring it
  # must NOT disable the size check beside it.
  mkreg; sed -i.bak 's/\t-\ta note$/\tlines=1;bytes=13;at=tracked\ta note/' "$d/reg/surfaces.tsv"
  arm GREEN-AT 0 "a baseline measured at the revision the file declares passes"

  mkreg; sed -i.bak 's/\t-\ta note$/\tlines=1;bytes=13;at=v0.9\ta note/' "$d/reg/surfaces.tsv"
  arm RED-AT-STALE 1 "a baseline measured at a revision the file no longer declares is refused"

  mkreg; sed -i.bak 's/\t-\ta note$/\tlines=1;bytes=13;at=tracked\ta note/' "$d/reg/surfaces.tsv"
  printf 'a second line, so the file outgrows the baseline it was re-based at\n' >> "$d/tree/ok.md"
  arm RED-AT-WIDEN 1 "declaring the right revision does not license growth past the baseline"
  printf 'tracked file\n' > "$d/tree/ok.md"

  mkreg; sed -i.bak 's/\t-\ta note$/\tlines=1;bytes=13;at=tracked\ta note/' "$d/reg/surfaces.tsv"
  sed -i.bak 's#\ttree/ok.md\tfile#\ttree/ok.md\tcollection#' "$d/reg/surfaces.tsv"
  arm RED-AT-KIND 1 "a revision token on a collection row is refused rather than ignored"

  mkreg; sed -i.bak 's#\ttree/ok.md\tfile#\ttree/absent-*.md\tcollection#' "$d/reg/surfaces.tsv"
  arm RED-GLOB 1 "a collection glob matching nothing, undeclared, is refused"

  mkreg; printf 'two\ttree/ok.md\tfile\n' >> "$d/reg/surfaces.tsv"
  arm RED-FIELDS 1 "a row with the wrong field count is refused, not skipped"

  mkreg; sed -i.bak 's/\tbounded_snapshot\tleaf-x\t/\tbounded_snapshot\tleaf-x\t/' "$d/reg/surfaces.tsv"
  printf 'x' >> "$d/reg/surfaces.tsv"   # break the trailing row shape
  arm RED-EMPTY 1 "a registry that stops being parseable is refused, not skipped"

  # Exercise binary measurement on an existing tracked input, without creating
  # a nested Git repository. It must count real bytes and suppress text axes.
  printf 'bin\t.githooks/commit-msg\tbinary_collection\n' > "$d/binary.tsv"
  measure_registry "$d/binary.tsv" "$d/binary-measured.tsv"
  arms=$((arms+1))
  if awk -F'\t' '$3==0 && $4>0 && $5==0 && $6=="" {ok=1} END{exit !ok}' "$d/binary-measured.tsv"; then
    printf '  ✓ BINARY-MEASURE exit=0  actual bytes counted; line/maxline axes suppressed\n'
  else
    printf '  ✗ BINARY-MEASURE text parsing or lost byte measurement\n'; fails=$((fails+1))
  fi
  printf 'bin\t.githooks/commit-msg\tbinary_collection\tarchive_terminal\tSPINE.19.2\tCOMMIT.md\t0\t0\t0\t1\t-\t-\t-\t0\t1\t0\t1\t0\t1\t-\tprobe\n' > "$d/binary.tsv"
  : > "$d/no-routes.tsv"; : > "$d/no-tracked.txt"
  arms=$((arms+1))
  if evaluate "$d/binary.tsv" "$d/no-routes.tsv" "$d/binary-measured.tsv" "$d/no-tracked.txt" >/dev/null 2>&1; then
    printf '  ✗ BINARY-BOUND actual binary overflow passed\n'; fails=$((fails+1))
  else
    printf '  ✓ BINARY-BOUND exit=1  real binary bytes exceed their bound\n'
  fi

  rm -rf "$d"
  printf 'live-doc-size --self-test: %d arms, %d failed\n' "$arms" "$fails"
  [ "$fails" -eq 0 ]
}

if [ "${1:-}" = "--self-test" ]; then
  self_test || exit 1
  exit 0
fi

# ── the judgement, over the real tree ────────────────────────────────────────────────────────
rm -rf "$SCRATCH"; mkdir -p "$SCRATCH"
if [ -n "${LIVE_DOC_SIZE_TRACKED:-}" ]; then
  cp "$LIVE_DOC_SIZE_TRACKED" "$SCRATCH/tracked_md.txt"
else
  git -C "$ROOT" ls-files '*.md' > "$SCRATCH/tracked_md.txt"
fi
measure_registry "$SURFACES" "$SCRATCH/measurements.tsv"
evaluate "$SURFACES" "$ROUTES" "$SCRATCH/measurements.tsv" "$SCRATCH/tracked_md.txt"
rc=$?
rm -rf "$SCRATCH"
exit "$rc"
