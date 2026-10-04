#!/usr/bin/env bash
# Derive the 400-commit push cadence (COMMIT.md); never perform a push.
# Director ruling 2026-10-04: no automatic CI/doctrine exceptions.
# Exit 0 = below cadence; 1 = cadence reached; 2 = comparison refused.
# PUSH_DUE_BASE overrides the upstream for a read-only historical comparison.
set -uo pipefail

refuse() { echo "push-due: REFUSED — $1" >&2; exit 2; }
ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || refuse 'not inside a git repository'
cd "$ROOT" || refuse 'cannot enter repository root'
BRANCH="$(git rev-parse --abbrev-ref HEAD 2>/dev/null)" || refuse 'cannot identify current branch'
BASE="${PUSH_DUE_BASE:-}"
if [ -z "$BASE" ]; then
  BASE="$(git rev-parse --abbrev-ref --symbolic-full-name '@{upstream}' 2>/dev/null)" ||
    refuse "no upstream for '$BRANCH' and no PUSH_DUE_BASE given"
fi
[ -n "$BASE" ] || refuse 'empty comparison base'
git rev-parse --verify --quiet --end-of-options "${BASE}^{commit}" >/dev/null ||
  refuse "'$BASE' does not resolve to a commit"

ahead="$(git rev-list --count "${BASE}..HEAD" 2>/dev/null)" || refuse 'cannot count unpushed commits'
[[ "$ahead" =~ ^(0|[1-9][0-9]*)$ ]] || refuse 'Git returned an invalid commit count'
cadence=400
printf 'push-due: branch %s vs %s — %s unpushed commit(s), cadence %s\n' "$BRANCH" "$BASE" "$ahead" "$cadence"

if [ "$ahead" = 0 ]; then
  echo 'push-due: nothing to push.'
  exit 0
fi
# Canonical decimal counts longer than the threshold are due without integer overflow.
if [ "${#ahead}" -gt "${#cadence}" ] || [ "$ahead" -ge "$cadence" ]; then
  echo "push-due: PUSH DUE — $ahead unpushed commits reached the $cadence-commit cadence."
  exit 1
fi
echo "push-due: no push due ($ahead < $cadence)."
exit 0
