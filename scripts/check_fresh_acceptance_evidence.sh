#!/usr/bin/env bash
# scripts/check_fresh_acceptance_evidence.sh — FRESH-ACCEPTANCE-EVIDENCE (PROJECT doctrine).
#
# THE RULE: a staged CODE change must be accompanied, IN THE SAME COMMIT'S DIFF, by the three
# hard-gated acceptance boxes — ROOT CAUSE, ADDRESSED, NO REGRESSION — added ticked and carrying
# tool-output evidence, in at least one staged task-tree leaf.
#
# WHY THIS EXISTS (measured, not theorised — defect D15 in docs/tasks/PLANNING.md):
#   the inherited universal check scripts/check_task_acceptance.sh extracts the FIRST bullet
#   matching each label anywhere in a staged leaf file. In a multi-leaf tree file — the shape this
#   project uses by design — that has three measured consequences:
#     facet 1 (false GREEN)  a code change owned by leaf B is accepted on leaf A's evidence,
#                            committed long ago: probe arm HOLE-1 -> exit=0.
#     facet 2 (false RED)    an honest, evidenced leaf is refused because an unticked placeholder
#                            for a future leaf sits above it: arms HOLE-2/HOLE-3 -> exit=1 / exit=0.
#     facet 3 (over-reach)   EVERY staged leaf file is judged, so co-staging a documentation tree
#                            with a code change puts that tree's boxes under the same rule.
#   Freshness closes facet 1 — evidence committed earlier cannot answer for this change — and is
#   immune to section order, which is what facets 2 and 3 turn on.
#
# ⚠ WHAT THIS CHECK CANNOT DO, stated rather than hidden: a check registered in the project slot
#   can only ADD refusals; it cannot relax a universal one. Facets 2 and 3 are therefore handled by
#   authoring rules (docs/decisions/decision_acceptance-evidence-per-leaf.md): no unticked
#   placeholder boxes in tree files, and a code commit stages the leaf that owns it.
#   To avoid adding a FOURTH false-red class, this check requires fresh boxes in AT LEAST ONE staged
#   leaf, never in all of them: a co-staged documentation tree is not required to invent evidence.
#
# NO FORKED SIGNATURE LIST: the tool-output signature regex is read from the universal check at
#   runtime, and .doctrine/evidence_tokens.txt is appended exactly as the universal check does. If
#   that extraction ever fails, this check REFUSES (exit 2) rather than silently weakening — a green
#   gate that judges nothing is the failure class this spine must not ship.
#
# ⚠ SIGNATURE MATCHING MUST USE THE SAME ENGINE AS THE UNIVERSAL CHECK (`grep -qE`), NOT awk.
#   DEFAULT_SIG carries GNU regex extensions — `\b` word boundaries and `{n}` intervals — that the
#   awk on this platform (BSD awk 20200816) does not implement. Measured over a 36-line corpus of
#   realistic evidence strings: `awk '$0 ~ sig'` leaves **12** unmatched (including `error[E0432]`
#   and `rc=0`, the two commonest Rust evidence shapes), while `grep -qE` leaves **2** — and both
#   are bad samples, not dead families. GNU grep 3.12 and BSD grep 2.6.0-FreeBSD agree, so the
#   universal check is portable; a re-implementation in awk would not have been. awk is used here
#   only for bullet STRUCTURE, where it needs no regex from the signature list.
#
# CONTRACT (DOCTRINE_ENFORCEMENT.md): exit code is the verdict; explains on stderr; deterministic;
#   read-only; staged-scope-aware; path-agnostic; fast. `--self-test` proves the arms fire.
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT" || exit 1

UNIVERSAL="scripts/check_task_acceptance.sh"
LABEL_KWS='root.?cause addressed no.?regress'

# ── what counts as a code change (the same seam the universal check uses) ───────────────────
default_code_re='(^|/)(crates|src|scripts)/|\.(rs|sh)$|(^|/)Makefile$'
if [ -f .doctrine/code_paths.txt ]; then
  code_re="$(grep -vE '^[[:space:]]*(#|$)' .doctrine/code_paths.txt | paste -sd'|' -)"
  [ -n "$code_re" ] || code_re="$default_code_re"
else
  code_re="$default_code_re"
fi

# ── the signature list, read from the universal check (never forked) ────────────────────────
signature_regex() {
  local base extra=""
  base="$(sed -n "s/^DEFAULT_SIG='\(.*\)'\$/\1/p" "$1" 2>/dev/null | head -1)"
  [ -n "$base" ] || return 1
  if [ -f .doctrine/evidence_tokens.txt ]; then
    extra="$(grep -vE '^[[:space:]]*(#|$)' .doctrine/evidence_tokens.txt | paste -sd'|' -)"
  fi
  if [ -n "$extra" ]; then printf '%s|%s\n' "$base" "$extra"; else printf '%s\n' "$base"; fi
}

# ── the verdict, as a pure function so it can carry ground truth ────────────────────────────
# $1 code staged · $2 a leaf staged · $3/$4/$5 the three boxes fresh+ticked+evidenced in at least
# one staged leaf. Echoes: ok | no-leaf | needs-fresh-evidence
fresh_verdict() {
  if [ "$1" != "1" ]; then printf 'ok\n'; return 0; fi
  if [ "$2" != "1" ]; then printf 'no-leaf\n'; return 0; fi
  if [ "$3" = "1" ] && [ "$4" = "1" ] && [ "$5" = "1" ]; then printf 'ok\n'; return 0; fi
  printf 'needs-fresh-evidence\n'
}

# Does the STAGED DIFF of one file add a ticked, evidenced bullet for one label?
# $1 = leaf file (relative to the repo the ambient git index belongs to) · $2 = label regex ·
# $3 = signature regex. Exit 0 = yes.
# Structure comes from awk (portable: it matches no signature); the signature test comes from
# `grep -qE`, the engine the universal check uses — see the portability note in the header.
adds_fresh_box() {
  local bullet
  bullet="$(git diff --cached -U0 -- "$1" 2>/dev/null \
    | grep -E '^\+' | grep -vE '^\+\+\+' | sed 's/^\+//' \
    | awk -v kw="$2" '
        BEGIN{ inbox=0; ticked=0; body="" }
        function flush() { if (inbox && ticked) print body; inbox=0; ticked=0; body="" }
        {
          line=$0
          if (line ~ /^[[:space:]]*-[[:space:]]*\[[xX ]\]/) {
            flush()
            if (match(tolower(line), kw)) {
              inbox=1
              ticked = (line ~ /^[[:space:]]*-[[:space:]]*\[[xX]\]/) ? 1 : 0
              body=line
            }
            next
          }
          if (inbox) {
            # a bullet continues through indented and blank lines; anything flush-left ends it
            if (line ~ /^[[:space:]]+/ || line ~ /^[[:space:]]*$/) { body = body "\n" line; next }
            flush()
          }
        }
        END{ flush() }
      ')"
  [ -n "$bullet" ] || return 1
  printf '%s\n' "$bullet" | grep -qE "$3"
}

# ── ground truth: the pure verdict must discriminate, on EVERY invocation ───────────────────
verdict_controls() {
  local spec want got misses=0
  for spec in "0:0:0:0:0:ok" "0:1:1:1:1:ok" "1:0:0:0:0:no-leaf" "1:1:1:1:1:ok" \
              "1:1:1:1:0:needs-fresh-evidence" "1:1:0:1:1:needs-fresh-evidence" \
              "1:1:1:0:1:needs-fresh-evidence" "1:1:0:0:0:needs-fresh-evidence"; do
    want="${spec##*:}"
    got="$(fresh_verdict "$(echo "$spec" | cut -d: -f1)" "$(echo "$spec" | cut -d: -f2)" \
            "$(echo "$spec" | cut -d: -f3)" "$(echo "$spec" | cut -d: -f4)" \
            "$(echo "$spec" | cut -d: -f5)")"
    if [ "$got" != "$want" ]; then
      printf 'fresh-acceptance-evidence: CONTROL MISSED: %s expected=%s got=%s\n' "$spec" "$want" "$got" >&2
      misses=$((misses + 1))
    fi
  done
  [ "$misses" -eq 0 ]
}

# ── extractor arms: a fresh box is found; stale, unticked and bare boxes are not ────────────
extractor_arms() { # $1 = path to a universal check to read signatures from
  local sig tmp arms=0 misses=0 got
  sig="$(signature_regex "$1")" || { echo "fresh-acceptance-evidence: cannot read signatures from $1" >&2; return 1; }
  # Scratch stays on the REPOSITORY volume (target/ is gitignored): project-owned temporary
  # workspaces never default to the system temp dir.
  tmp="$ROOT/target/doctrine_scratch/fresh_acceptance_evidence"
  rm -rf "$tmp"; mkdir -p "$tmp/docs/tasks" "$tmp/scripts" || return 1
  cp "$UNIVERSAL" "$tmp/scripts/check_task_acceptance.sh" 2>/dev/null || true
  ( cd "$tmp" && git init -q . && git config user.email s@e.invalid && git config user.name s \
    && printf '# seed\n' > docs/tasks/T.md && git add -A && git commit -qm "SEED-0001: seed" ) >/dev/null 2>&1

  arm() { # $1 label · $2 file content · $3 expected exit (0 found / 1 not found)
    arms=$((arms + 1))
    printf '%s\n' "$2" > "$tmp/docs/tasks/T.md"
    ( cd "$tmp" && git add -A ) >/dev/null 2>&1
    if ( cd "$tmp" && adds_fresh_box "docs/tasks/T.md" 'root.?cause' "$sig" ); then got=0; else got=1; fi
    if [ "$got" != "$3" ]; then
      printf 'fresh-acceptance-evidence: ARM MISSED: %s expected=%s got=%s\n' "$1" "$3" "$got" >&2
      misses=$((misses + 1))
    fi
  }

  arm "added, ticked, evidenced" '### T.1

- [x] **ROOT CAUSE (WHY + WHERE)** — `cargo test` reported `test result: FAILED. 0 passed; 1 failed`.' 0
  arm "added but unticked" '### T.1

- [ ] **ROOT CAUSE (WHY + WHERE)** — `cargo test` reported `test result: FAILED. 0 passed; 1 failed`.' 1
  arm "added, ticked, no tool output" '### T.1

- [x] **ROOT CAUSE (WHY + WHERE)** — the parser mishandles the header.' 1
  arm "added, ticked, evidence on a continuation line" '### T.1

- [x] **ROOT CAUSE (WHY + WHERE)** — the extractor reports:
  `test result: ok. 1 passed; 0 failed`.' 0
  # The arm that pins the engine choice: `\b` and `{n}` are GNU extensions that this platform's awk
  # does not implement, so a signature test written in awk fails on the commonest Rust evidence.
  arm "GNU-extension families only (\\brc=, {n} interval)" '### T.1

- [x] **ROOT CAUSE (WHY + WHERE)** — `cargo build` printed `error[E0432]`, and the census exited `rc=1`.' 0
  arm "a different label only" '### T.1

- [x] **ADDRESSED (verified)** — before `test result: FAILED`, after `test result: ok`.' 1
  rm -rf "$tmp"
  printf 'fresh-acceptance-evidence: extractor arms %d/%d\n' "$((arms - misses))" "$arms"
  [ "$misses" -eq 0 ]
}

if ! verdict_controls; then
  echo "FRESH-ACCEPTANCE-EVIDENCE: the verdict function does not discriminate; REFUSING." >&2
  exit 2
fi
if [ "${1:-}" = "--self-test" ]; then
  verdict_controls && extractor_arms "$UNIVERSAL" || exit 1
  echo "FRESH-ACCEPTANCE-EVIDENCE --self-test: 8 verdict controls + 6 extractor arms"
  exit 0
fi

# ── the judgement ───────────────────────────────────────────────────────────────────────────
staged="$(git diff --cached --name-only --diff-filter=ACM 2>/dev/null || true)"
[ -n "$staged" ] || { echo "FRESH-ACCEPTANCE-EVIDENCE: ok (nothing staged)"; exit 0; }

code_staged=0
printf '%s\n' "$staged" | grep -qE "$code_re" && code_staged=1
[ "$code_staged" = "1" ] || { echo "FRESH-ACCEPTANCE-EVIDENCE: ok (no staged code change)"; exit 0; }

SIG="$(signature_regex "$UNIVERSAL")" || {
  echo "FRESH-ACCEPTANCE-EVIDENCE: could not read the signature list out of $UNIVERSAL — the" >&2
  echo "  universal check changed shape. Fix this extractor rather than weaken the doctrine." >&2
  exit 2
}

leaves="$(printf '%s\n' "$staged" | grep -E '^docs/tasks/.*\.md$' | grep -vE '(^|/)TEMPLATE\.md$' || true)"
leaf_staged=0; [ -n "$leaves" ] && leaf_staged=1

root_found=0; addr_found=0; reg_found=0
for f in $leaves; do
  adds_fresh_box "$f" 'root.?cause' "$SIG" && root_found=1
  adds_fresh_box "$f" 'addressed'   "$SIG" && addr_found=1
  adds_fresh_box "$f" 'no.?regress' "$SIG" && reg_found=1
done

verdict="$(fresh_verdict "$code_staged" "$leaf_staged" "$root_found" "$addr_found" "$reg_found")"
case "$verdict" in
  ok)
    echo "FRESH-ACCEPTANCE-EVIDENCE: ok (this commit adds its own ticked, evidence-backed boxes)"
    exit 0 ;;
  no-leaf)
    { echo "FRESH-ACCEPTANCE-EVIDENCE: a CODE change is staged with no task-tree leaf."
      echo "  Stage the docs/tasks/<TREE>.md leaf that owns this change." ; } >&2
    exit 1 ;;
  needs-fresh-evidence)
    { echo "FRESH-ACCEPTANCE-EVIDENCE: a CODE change is staged, but this commit ADDS no fresh"
      echo "  acceptance evidence. Missing box(es):"
      [ "$root_found" = 1 ] || echo "    - ROOT CAUSE (WHY + WHERE)"
      [ "$addr_found" = 1 ] || echo "    - ADDRESSED (verified)"
      [ "$reg_found" = 1 ] || echo "    - NO REGRESSION"
      echo "  Add them, ticked, with the invocation + output + exit status INSIDE each bullet, in a"
      echo "  '### <leaf-id>' subsection of the leaf that owns this change."
      echo "  Why: evidence committed for an earlier leaf cannot answer for this one (defect D15,"
      echo "  docs/decisions/decision_acceptance-evidence-per-leaf.md)."
      printf '  staged leaves: %s\n' "$(printf '%s' "$leaves" | tr '\n' ' ')" ; } >&2
    exit 1 ;;
esac
