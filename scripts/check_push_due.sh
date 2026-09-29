#!/usr/bin/env bash
# scripts/check_push_due.sh — is an EXCEPTIONAL push due?
#
# THE RULE (COMMIT.md → Push cadence): the cadence is 400 commits between pushes, **except** that a
# push is due as soon as an unpushed commit touches CI or a doctrine check — because such a change is
# unverified until a runner executes it. Layer E4 (CI) is the un-bypassable backstop; a workflow or a
# gate that has only ever run on one developer machine has not been verified at all, and this
# repository has already measured why that matters (BSD awk lacks `\b` and `{n}`; this machine's PATH
# shadows BSD userland with GNU coreutils).
#
# Exit 0 = no exceptional push is due (prints the state either way).
# Exit 1 = an exceptional push IS due; the triggering files are listed.
# Exit 2 = refused (not a git repository, or no upstream to compare against).
#
# The comparison base is `PUSH_DUE_BASE` (default: the upstream of the current branch), which makes the
# "due" arm testable against an older revision without inventing unpushed commits.
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || {
  echo "push-due: REFUSED — not inside a git repository" >&2; exit 2; }
cd "$ROOT" || exit 2

BRANCH="$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo '')"
BASE="${PUSH_DUE_BASE:-}"
if [ -z "$BASE" ]; then
  BASE="$(git rev-parse --abbrev-ref --symbolic-full-name '@{upstream}' 2>/dev/null || true)"
fi
if [ -z "$BASE" ]; then
  echo "push-due: REFUSED — no upstream for '$BRANCH' and no PUSH_DUE_BASE given; nothing to compare against" >&2
  exit 2
fi
if ! git rev-parse --verify --quiet "$BASE" >/dev/null; then
  echo "push-due: REFUSED — '$BASE' does not resolve to a revision" >&2
  exit 2
fi

# Paths whose change is unverified until a runner executes it.
# What triggers an exceptional push is "a runner must re-verify this", NOT "the filename looks like a
# check". So the registered doctrine checks are DERIVED from the two registries rather than globbed:
# an unregistered helper (this file, for one) is not executed by any runner, and treating it as a
# trigger creates a standing false obligation — measured: the first cut of this list globbed
# `scripts/check_*.sh`, so committing this very helper reported an exceptional push due for a file no
# CI job reads.
#
# ⚠ Pathspec shapes matter and were measured, not assumed: a bare prefix such as `scripts/check_`
# matches nothing, because a git pathspec matches whole path components unless it carries a wildcard.
# That bug silently missed every doctrine check script, and only the RED arm — which compares against
# a revision known to contain them — showed the omission.
registry_checks() {
  {
    # the universal registry, the project slot, and the drivers themselves
    grep -oE '(scripts|knowledge-map/scripts)/[A-Za-z0-9_.-]+\.sh' scripts/check_doctrines.sh 2>/dev/null
    grep -oE '(scripts|knowledge-map/scripts)/[A-Za-z0-9_.-]+\.sh' scripts/check_doctrines.project.sh 2>/dev/null
    printf 'scripts/check_doctrines.sh\nscripts/check_doctrines.project.sh\n'
  } | sed '/^$/d' | LC_ALL=C sort -u
}

TRIGGER_PATHS=(
  '.github/workflows/*'
  '.doctrine/*'
  '.githooks/*'
)
while IFS= read -r _c; do
  [ -n "$_c" ] && TRIGGER_PATHS+=("$_c")
done < <(registry_checks)

ahead="$(git rev-list --count "$BASE"..HEAD 2>/dev/null || echo 0)"
cadence=400

printf 'push-due: branch %s vs %s — %s unpushed commit(s), cadence %s\n' "$BRANCH" "$BASE" "$ahead" "$cadence"

if [ "$ahead" -eq 0 ]; then
  echo "push-due: nothing to push."
  exit 0
fi

hits=""
for p in "${TRIGGER_PATHS[@]}"; do
  found="$(git diff --name-only "$BASE"..HEAD -- "$p" 2>/dev/null)"
  [ -n "$found" ] && hits="${hits}${found}"$'\n'
done
hits="$(printf '%s' "$hits" | sed '/^$/d' | LC_ALL=C sort -u)"

if [ -n "$hits" ]; then
  n="$(printf '%s\n' "$hits" | wc -l | tr -d ' ')"
  echo "push-due: EXCEPTIONAL PUSH DUE — $n unpushed file(s) under CI/doctrine paths are unverified by CI:"
  printf '%s\n' "$hits" | sed 's/^/    /'
  echo "  Push, then record the observed CI verdict in the owning leaf (never before observing it)."
  exit 1
fi

if [ "$ahead" -ge "$cadence" ]; then
  echo "push-due: PUSH DUE — $ahead unpushed commits reached the $cadence-commit cadence."
  exit 1
fi

echo "push-due: no push due ($ahead < $cadence, no CI/doctrine paths touched)."
exit 0
