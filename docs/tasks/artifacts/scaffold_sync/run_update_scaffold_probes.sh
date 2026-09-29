#!/usr/bin/env bash
# docs/tasks/artifacts/scaffold_sync/run_update_scaffold_probes.sh
# SPINE.9 — RED / GREEN / CONTROL probes for scripts/update_scaffold.sh, the scaffold updater whose
# previous single-list version classified four project-content files as NEUTRAL ("safe to overwrite
# because it never carries project content") — among them docs/TASK_TREE.md, which holds this
# project's Active Task Trees index, i.e. layer-B navigation (defect D17).
#
# ⭐ ARM-2 IS THE POINT. One run of the documented "keep the spine current" command used to replace
#    a project's task-tree index with template blanks. The arm builds exactly that situation — local
#    project rows plus a changed upstream copy — and asserts the local rows survive, the report says
#    SKIPPED, and a backup of the local copy exists.
# ⭐ ARM-4 AND ARM-5 are the "changes nothing" arms: a dry run must not write (including backups or
#    file modes), and a dirty tree must be refused, because a sync over uncommitted work hides both
#    the update and any loss in `git diff`.
#
# Each arm builds a throwaway project repository AND a throwaway bedrock source under
# target/doctrine_scratch (repository volume, gitignored); no network, no write outside the scratch.
#
# Usage:  bash docs/tasks/artifacts/scaffold_sync/run_update_scaffold_probes.sh
# Output: per-arm lines plus `probes: N pass / M fail` (exit nonzero if M > 0).
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
UPDATER="$ROOT/scripts/update_scaffold.sh"
[ -f "$UPDATER" ] || { echo "probe: REFUSED — $UPDATER not found" >&2; exit 2; }

WORK="$ROOT/target/doctrine_scratch/scaffold_sync_probes"
rm -rf "$WORK"; mkdir -p "$WORK"; trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
note() { printf '  ✓ %-7s %s\n' "$1" "$2"; pass=$((pass+1)); }
bad()  { printf '  ✗ %-7s %s\n' "$1" "$2"; fail=$((fail+1)); [ -n "${3:-}" ] && printf '%s\n' "$3" | sed 's/^/        /'; return 0; }

# A throwaway bedrock source: a real git repo so the updater takes its local-path branch (no network).
mksource() { # $1 = name
  local d="$WORK/src-$1"
  rm -rf "$d"; mkdir -p "$d/scripts" "$d/docs/tasks" "$d/.githooks" "$d/knowledge-map/scripts" "$d/.doctrine"
  printf 'upstream MEMORY_ARCHITECTURE v2\n' > "$d/MEMORY_ARCHITECTURE.md"
  printf '# Task-Tree Workflow\n\n## Active Task Trees\n\n| Tree | Status |\n| --- | --- |\n| _none yet_ | |\n' > "$d/docs/TASK_TREE.md"
  printf '#!/bin/sh\nexit 0\n' > "$d/.githooks/pre-commit"
  printf '0.9.9\n' > "$d/DOCTRINE_VERSION"
  # deliberately NO TOOLBOX.md and NO scripts/check_table_arity.sh: the "absent upstream" control
  git -C "$d" init -q . >/dev/null 2>&1
  git -C "$d" config user.email src@example.invalid; git -C "$d" config user.name src
  git -C "$d" add -A >/dev/null 2>&1; git -C "$d" commit -qm "SRC-0001: upstream" >/dev/null 2>&1
  printf '%s' "$d"
}

mkproject() { # $1 = name
  local d="$WORK/proj-$1"
  rm -rf "$d"; mkdir -p "$d/scripts" "$d/docs/tasks" "$d/.githooks" "$d/knowledge-map/scripts" "$d/.doctrine"
  cp "$UPDATER" "$d/scripts/update_scaffold.sh"
  printf 'local MEMORY_ARCHITECTURE v1\n' > "$d/MEMORY_ARCHITECTURE.md"
  printf '# Task-Tree Workflow\n\n## Active Task Trees\n\n| Tree | Status |\n| --- | --- |\n| [`PLANNING`](tasks/PLANNING.md) | active |\n| [`G0-CONTRACT`](tasks/G0-CONTRACT.md) | active |\n' > "$d/docs/TASK_TREE.md"
  printf '# TOOLBOX\n\n| Tool | Answers |\n| --- | --- |\n| our-probe | where does it stop? |\n' > "$d/TOOLBOX.md"
  printf '#!/bin/sh\nexit 0\n' > "$d/.githooks/pre-commit"
  printf '0.6.1\n' > "$d/DOCTRINE_VERSION"
  git -C "$d" init -q . >/dev/null 2>&1
  git -C "$d" config user.email proj@example.invalid; git -C "$d" config user.name proj
  printf '/target\n' > "$d/.gitignore"
  git -C "$d" add -A >/dev/null 2>&1; git -C "$d" commit -qm "PROJ-0001: project" >/dev/null 2>&1
  printf '%s' "$d"
}

run() { # repo · source · extra args… ; captures output+rc into RUN_OUT / RUN_RC
  local d="$1" s="$2"; shift 2
  RUN_OUT="$(cd "$d" && bash scripts/update_scaffold.sh "$s" "$@" 2>&1)"; RUN_RC=$?
}

# A portable mtime read. ⚠ Pinned to /usr/bin/stat on purpose: this machine's PATH puts GNU
# coreutils first, where `stat -f %m` means "filesystem status of a file named %m" and fails, while
# BSD stat needs exactly that form. A probe that measures with whatever `stat` happens to be in PATH
# measures the PATH, not the thing under test — the same class of error as testing a gate with a
# different regex engine than the gate uses. Fails loudly if neither form works.
mtime() {
  local m
  if m="$(/usr/bin/stat -f %m "$1" 2>/dev/null)" && [ -n "$m" ]; then printf '%s\n' "$m"; return 0; fi
  if m="$(/usr/bin/stat -c %Y "$1" 2>/dev/null)" && [ -n "$m" ]; then printf '%s\n' "$m"; return 0; fi
  printf 'UNREADABLE-MTIME\n'
}

echo "scaffold-sync probes — scripts/update_scaffold.sh"

# ---------------------------------------------------------------- ARM-1: neutral files sync
src="$(mksource 1)"; proj="$(mkproject 1)"
run "$proj" "$src"
if [ "$RUN_RC" = 0 ] && grep -q 'upstream MEMORY_ARCHITECTURE v2' "$proj/MEMORY_ARCHITECTURE.md" \
   && grep -q '^  synced       MEMORY_ARCHITECTURE.md' <<<"$RUN_OUT"; then
  note ARM-1 "a neutral file takes the upstream copy (exit=$RUN_RC)"
else
  bad ARM-1 "neutral file was not synced (exit=$RUN_RC)" "$RUN_OUT"
fi

# ---------------------------------------------------------------- ARM-2: project content survives
if grep -q 'PLANNING' "$proj/docs/TASK_TREE.md" && grep -q 'SKIPPED      docs/TASK_TREE.md' <<<"$RUN_OUT" \
   && ! grep -q '_none yet_' "$proj/docs/TASK_TREE.md"; then
  if ls "$proj"/target/scaffold_backup/*/docs/TASK_TREE.md >/dev/null 2>&1; then
    note ARM-2 "⭐ the task-tree index survived; SKIPPED reported; a backup of it exists"
  else
    bad ARM-2 "index survived but no backup was written" "$(ls -R "$proj/target" 2>&1)"
  fi
else
  bad ARM-2 "PROJECT CONTENT WAS CLOBBERED — the index no longer holds the project rows" "$(cat "$proj/docs/TASK_TREE.md"; printf '\n---\n%s\n' "$RUN_OUT")"
fi

# ---------------------------------------------------------------- ARM-3: TOOLBOX.md is guarded too
if grep -q 'our-probe' "$proj/TOOLBOX.md" && grep -q 'absent from the source' <<<"$RUN_OUT"; then
  note ARM-3 "TOOLBOX.md kept its project rows; a file missing upstream is reported absent, not deleted"
else
  bad ARM-3 "TOOLBOX.md lost its project rows" "$RUN_OUT"
fi

# ---------------------------------------------------------------- ARM-4: --dry-run writes nothing
src="$(mksource 4)"; proj="$(mkproject 4)"
before="$(cd "$proj" && find . -path ./.git -prune -o -type f -print0 | sort -z | xargs -0 shasum | shasum | cut -d' ' -f1)"
run "$proj" "$src" --dry-run
after="$(cd "$proj" && find . -path ./.git -prune -o -type f -print0 | sort -z | xargs -0 shasum | shasum | cut -d' ' -f1)"
if [ "$before" = "$after" ] && grep -q 'DRY RUN' <<<"$RUN_OUT" && grep -q 'would SKIP   docs/TASK_TREE.md' <<<"$RUN_OUT" \
   && grep -q 'would sync   MEMORY_ARCHITECTURE.md' <<<"$RUN_OUT"; then
  note ARM-4 "a dry run reports both classes and writes nothing (tree checksum unchanged)"
else
  bad ARM-4 "dry run mutated the tree or mis-reported (before=$before after=$after)" "$RUN_OUT"
fi

# ---------------------------------------------------------------- ARM-5: a dirty tree is refused
src="$(mksource 5)"; proj="$(mkproject 5)"
printf 'uncommitted work\n' >> "$proj/MEMORY_ARCHITECTURE.md"
run "$proj" "$src"
rc_refused="$RUN_RC"; out_refused="$RUN_OUT"
# Nothing may have been written by the refused run — check BEFORE the second run overwrites it.
wrote_nothing=no; grep -q 'uncommitted work' "$proj/MEMORY_ARCHITECTURE.md" || wrote_nothing=yes
run "$proj" "$src" --allow-dirty
rc_allowed="$RUN_RC"; out_allowed="$RUN_OUT"
if [ "$rc_refused" != 0 ] && grep -q 'REFUSED' <<<"$out_refused" \
   && ! grep -q '^  synced' <<<"$out_refused" && [ "$wrote_nothing" = no ] \
   && [ "$rc_allowed" = 0 ] && ! grep -q 'REFUSED' <<<"$out_allowed" \
   && grep -q 'upstream MEMORY_ARCHITECTURE v2' "$proj/MEMORY_ARCHITECTURE.md"; then
  note ARM-5 "a dirty tree is refused having written nothing (exit=$rc_refused); --allow-dirty proceeds and syncs (exit=$rc_allowed)"
else
  bad ARM-5 "dirty-tree guard misbehaved (refused exit=$rc_refused wrote_nothing=$wrote_nothing, allow-dirty exit=$rc_allowed)" \
    "--- refused run ---$out_refused--- allowed run ---$out_allowed"
fi

# ---------------------------------------------------------------- ARM-6: --force backs up first
src="$(mksource 6)"; proj="$(mkproject 6)"
run "$proj" "$src" --force-project-sections
if grep -q '_none yet_' "$proj/docs/TASK_TREE.md" && grep -q 'FORCED       docs/TASK_TREE.md' <<<"$RUN_OUT"; then
  bk="$(ls "$proj"/target/scaffold_backup/*/docs/TASK_TREE.md 2>/dev/null | head -1)"
  if [ -n "$bk" ] && grep -q 'PLANNING' "$bk"; then
    note ARM-6 "--force overwrites as asked, and the backup still holds the project rows"
  else
    bad ARM-6 "--force overwrote without a usable backup" "$(ls -R "$proj/target" 2>&1)"
  fi
else
  bad ARM-6 "--force did not overwrite (exit=$RUN_RC)" "$RUN_OUT"
fi

# ---------------------------------------------------------------- CTRL-1: an identical file is left alone
src="$(mksource 7)"; proj="$(mkproject 7)"
cp "$src/.githooks/pre-commit" "$proj/.githooks/pre-commit"
mtime_before="$(mtime "$proj/.githooks/pre-commit")"
sleep 1
run "$proj" "$src"
mtime_after="$(mtime "$proj/.githooks/pre-commit")"
if [ "$mtime_before" = "UNREADABLE-MTIME" ] || [ "$mtime_after" = "UNREADABLE-MTIME" ]; then
  bad CTRL-1 "the probe cannot read mtimes on this platform — refusing to claim a verdict"
elif [ "$mtime_before" = "$mtime_after" ] && grep -q 'already identical: [1-9]' <<<"$RUN_OUT" \
     && ! ls "$proj"/target/scaffold_backup/*/.githooks/pre-commit >/dev/null 2>&1; then
  note CTRL-1 "an identical file is not rewritten (mtime unchanged, no backup, counted identical)"
else
  bad CTRL-1 "an identical file was rewritten (mtime $mtime_before → $mtime_after)" "$RUN_OUT"
fi

echo
printf 'probes: %d pass / %d fail\n' "$pass" "$fail"
[ "$fail" -eq 0 ] || exit 1
exit 0
