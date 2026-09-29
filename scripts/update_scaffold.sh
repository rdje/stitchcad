#!/usr/bin/env bash
# scripts/update_scaffold.sh — pull the latest project-NEUTRAL spine from the bedrock template
# into THIS project, WITHOUT touching the roadmap, task-trees, decisions, code, or any file whose
# local copy carries project content.
#
#   scripts/update_scaffold.sh <bedrock-repo-url-or-local-path> [--dry-run] [--force-project-sections]
#
# TWO CLASSES OF FILE, and the distinction is the whole point (defect D17, leaf SPINE.9):
#
#   NEUTRAL            carries no project content by construction. Overwritten freely.
#   PROJECT-CONTENT    the template itself instructs the project to write into it, so the local copy
#                      is authoritative and a sync would silently destroy it. Backed up, reported
#                      and SKIPPED unless --force-project-sections is passed.
#
# The measured failure this prevents: the previous single-list version classified
# `docs/TASK_TREE.md`, `TOOLBOX.md`, `README_POLICY.md` and `docs/tasks/TEMPLATE.md` as neutral
# ("safe to overwrite because it never carries project content") while `docs/TASK_TREE.md` held this
# project's Active Task Trees index — layer-B navigation, the thing a resuming session reads first —
# and `TOOLBOX.md` held the project toolbox table its own header asks the project to fill in. One run
# of the documented "keep the spine current" command would have replaced both with template blanks,
# and `git diff` afterwards is the only place the loss would have been visible.
#
# Everything project-owned (CLAUDE.md, README.md, ROADMAP.md, the live docs, the project doctrine
# slot, the curated knowledge-map/subsystems.md, all of docs/tasks/ except TEMPLATE.md, and all of
# docs/decisions/ except TEMPLATE.md) is not listed at all and is never touched.
#
# Scratch (the clone) and backups stay under target/ — the repository volume — never in the system
# temp dir. After syncing: review `git diff`, run `make gate`, and commit.
set -euo pipefail

usage() {
  cat <<'EOF'
usage: scripts/update_scaffold.sh <bedrock-repo-url-or-local-path> [options]

  --dry-run                   report what would happen; change nothing
  --force-project-sections    overwrite project-content files too (they are backed up first)
  --allow-dirty               proceed even though the working tree has uncommitted changes
  -h, --help                  this text

Refuses a dirty tree by default: a sync that overwrites files on top of uncommitted work makes
both the update and any loss invisible in `git diff`.
EOF
}

SRC="${1:-}"; shift || true
DRY=0; FORCE=0; ALLOW_DIRTY=0
for arg in "$@"; do
  case "$arg" in
    --dry-run) DRY=1 ;;
    --force-project-sections) FORCE=1 ;;
    --allow-dirty) ALLOW_DIRTY=1 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "update_scaffold: unknown option '$arg'" >&2; usage >&2; exit 2 ;;
  esac
done
[ -n "$SRC" ] || { usage >&2; exit 2; }

ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

if [ "$ALLOW_DIRTY" != 1 ] && [ -n "$(git status --porcelain 2>/dev/null)" ]; then
  { echo "update_scaffold: REFUSED — the working tree is dirty."
    echo "  Commit or stash first (a sync over uncommitted work hides both the update and any"
    echo "  loss in 'git diff'), or pass --allow-dirty to proceed anyway."
    git status --short | sed 's/^/    /' >&2; } >&2
  exit 1
fi

STAMP="$(date -u +%Y%m%dT%H%M%SZ)-$$"
SCRATCH="$ROOT/target/scaffold_sync/$STAMP"
BACKUP="$ROOT/target/scaffold_backup/$STAMP"

# ── the classification ───────────────────────────────────────────────────────────────────────
# Echoes the reason a file is PROJECT-CONTENT, or nothing when it is neutral. Kept as a function
# (not an associative array) so the script runs on bash 3.2, which macOS still ships.
project_content_reason() {
  case "$1" in
    docs/TASK_TREE.md)        echo "carries the Active Task Trees index (layer-B navigation)" ;;
    TOOLBOX.md)               echo "carries the project toolbox table its own header asks you to fill in" ;;
    README_POLICY.md)         echo "the adopted policy is project-owned and carries a local adoption note" ;;
    DOCTRINE_ENFORCEMENT.md)  echo "its 'Adding a doctrine' step invites project rows in the registry mirror" ;;
    COMMIT.md)                echo "MEMORY_ARCHITECTURE names the commit-workflow doc a per-project knob" ;;
    AGENTS.md)                echo "a bootstrap pointer the project may extend with harness notes" ;;
    docs/tasks/TEMPLATE.md)   echo "the leaf form a project may extend with its own conventions" ;;
    docs/decisions/TEMPLATE.md) echo "the record form a project may extend (e.g. an answers: line)" ;;
    *)                        echo "" ;;
  esac
}

NEUTRAL=(
  MEMORY_ARCHITECTURE.md
  docs/TASK_TREE_README.md
  .githooks/pre-commit
  .githooks/commit-msg
  scripts/check_doctrines.sh
  scripts/check_memory_architecture.sh
  scripts/check_live_doc_currency.sh
  scripts/check_no_background_jobs.sh
  scripts/check_lesson_promotion.sh
  scripts/check_routing_evidence.sh
  scripts/check_gap_claims.sh
  scripts/check_table_arity.sh
  scripts/check_readme_stability.sh
  scripts/check_waiver_routing.sh
  scripts/check_task_acceptance.sh
  scripts/check_docpaths.sh
  scripts/check_task_tree_ownership.sh
  .doctrine/README.md
  knowledge-map/scripts/gen_knowledge_map.sh
  knowledge-map/scripts/check_knowledge_map.sh
  DOCTRINE_VERSION
)
PROJECT_CONTENT=(
  docs/TASK_TREE.md
  TOOLBOX.md
  README_POLICY.md
  DOCTRINE_ENFORCEMENT.md
  COMMIT.md
  AGENTS.md
  docs/tasks/TEMPLATE.md
  docs/decisions/TEMPLATE.md
)

# ── fetch the source ─────────────────────────────────────────────────────────────────────────
if [ "$DRY" = 1 ]; then SCRATCH="$ROOT/target/scaffold_sync/dryrun-$$"; fi
rm -rf "$SCRATCH"; mkdir -p "$SCRATCH"
cleanup() { rm -rf "$SCRATCH"; }
trap cleanup EXIT
if [ -d "$SRC/.git" ]; then
  cp -R "$SRC" "$SCRATCH/bedrock"
  rm -rf "$SCRATCH/bedrock/.git"
else
  git clone --depth 1 "$SRC" "$SCRATCH/bedrock" >/dev/null 2>&1 \
    || { echo "clone failed: $SRC" >&2; exit 1; }
fi

synced=0; skipped=0; forced=0; unchanged=0; absent=0
backup_of() { # $1 = relative path -> prints the backup path (creating the dir)
  local dir; dir="$BACKUP/$(dirname "$1")"
  [ -d "$dir" ] || mkdir -p "$dir"
  printf '%s\n' "$BACKUP/$1"
}

echo "=== scaffold sync ($([ "$DRY" = 1 ] && echo 'DRY RUN — nothing will be written' || echo 'live')) ==="
echo "  source:  $SRC"
echo "  backup:  $([ "$DRY" = 1 ] && echo '(dry run: no backups written)' || echo "$BACKUP")"
echo

for f in "${NEUTRAL[@]}"; do
  if [ ! -f "$SCRATCH/bedrock/$f" ]; then absent=$((absent + 1)); continue; fi
  if [ -f "$f" ] && cmp -s "$SCRATCH/bedrock/$f" "$f"; then unchanged=$((unchanged + 1)); continue; fi
  if [ "$DRY" = 1 ]; then
    echo "  would sync   $f (neutral)"
  else
    if [ -f "$f" ]; then
      b="$(backup_of "$f")"; cp "$f" "$b"
      cp "$SCRATCH/bedrock/$f" "$f"
      echo "  synced       $f (neutral, previous copy backed up)"
    else
      mkdir -p "$(dirname "$f")"; cp "$SCRATCH/bedrock/$f" "$f"
      echo "  synced       $f (neutral, new)"
    fi
  fi
  synced=$((synced + 1))
done

for f in "${PROJECT_CONTENT[@]}"; do
  if [ ! -f "$SCRATCH/bedrock/$f" ]; then absent=$((absent + 1)); continue; fi
  if [ -f "$f" ] && cmp -s "$SCRATCH/bedrock/$f" "$f"; then unchanged=$((unchanged + 1)); continue; fi
  reason="$(project_content_reason "$f")"
  if [ ! -f "$f" ]; then
    # No local copy: nothing to lose, so a project-content file may be seeded.
    if [ "$DRY" = 1 ]; then echo "  would seed   $f (absent locally)"
    else mkdir -p "$(dirname "$f")"; cp "$SCRATCH/bedrock/$f" "$f"; echo "  seeded       $f (absent locally)"; fi
    synced=$((synced + 1)); continue
  fi
  if [ "$FORCE" = 1 ]; then
    if [ "$DRY" = 1 ]; then
      echo "  would FORCE  $f — project content: $reason"
    else
      b="$(backup_of "$f")"; cp "$f" "$b"
      cp "$SCRATCH/bedrock/$f" "$f"
      echo "  FORCED       $f — project content overwritten ($reason)"
      echo "               re-apply your local sections; the previous copy is at ${b#"$ROOT"/}"
    fi
    forced=$((forced + 1)); continue
  fi
  if [ "$DRY" = 1 ]; then
    echo "  would SKIP   $f — project content: $reason"
  else
    b="$(backup_of "$f")"; cp "$f" "$b"
    echo "  SKIPPED      $f — project content: $reason"
    echo "               differs from upstream; kept your copy, backed up to ${b#"$ROOT"/}"
    echo "               merge by hand, or re-run with --force-project-sections"
  fi
  skipped=$((skipped + 1))
done

if [ "$DRY" != 1 ]; then
  chmod +x scripts/*.sh knowledge-map/scripts/*.sh .githooks/pre-commit .githooks/commit-msg 2>/dev/null || true
fi

echo
echo "  neutral synced/would-sync: $synced · project-content skipped: $skipped · forced: $forced"
echo "  already identical: $unchanged · absent from the source: $absent"
echo "✓ scaffold $(cat DOCTRINE_VERSION 2>/dev/null || echo '?') → $(cat "$SCRATCH/bedrock/DOCTRINE_VERSION" 2>/dev/null || echo '?')."
if [ "$DRY" = 1 ]; then
  echo "  DRY RUN: nothing was written. Re-run without --dry-run to apply."
else
  echo "  Review 'git diff', re-apply any local sections the report mentions, run 'make gate', then commit."
fi
