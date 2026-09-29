#!/usr/bin/env bash
# docs/tasks/artifacts/task_acceptance/run_multileaf_shadowing_probe.sh
# SPINE.7 — measures defect D15 in the inherited TASK-ACCEPTANCE gate.
#
# THE DEFECT, in one sentence: scripts/check_task_acceptance.sh extracts the FIRST bullet
# matching each hard-gated label (ROOT CAUSE / ADDRESSED / NO REGRESSION) anywhere in a staged
# docs/tasks/*.md and judges that, so in a MULTI-LEAF tree file — the shape this project uses by
# design — the verdict depends on section ORDER rather than on the leaf that owns the change:
#
#   HOLE-1 (false GREEN)  leaf A's ticked, evidence-backed checklist sits above leaf B's
#                         unticked one; the staged code change is owned by B. The gate reports
#                         "every staged code-change leaf carries a ticked, evidence-backed
#                         checklist" and exits 0 — on evidence belonging to a different leaf.
#                         This is the same leakage class the check's own header says box-scoping
#                         closed ("a co-staged unrelated leaf supplying the evidence"); the
#                         shipped probe suite exercises it across FILES only.
#   HOLE-2 (false RED)    an honest leaf whose checklist is ticked with real tool output is
#                         rejected because an unticked placeholder for a FUTURE leaf appears
#                         earlier in the same file.
#
# Each arm builds a throwaway repository and runs the SHIPPED check against a real staged diff,
# so every verdict below is the gate's own, not a paraphrase of it. Scratch repositories live
# under target/doctrine_scratch (gitignored, same volume as the repository) rather than in the
# system temp dir: project-owned scratch stays on the project volume.
#
# ARMS  (want = the verdict this probe asserts TODAY, defects included)
#   CTRL-1  one leaf, ticked boxes carrying tool output, code change   -> exit 0
#   CTRL-2  one leaf, ticked boxes with NO tool output, code change    -> exit 1  (evidence-scoped)
#   CTRL-3  code change with no owning leaf staged                     -> exit 1  (ownership)
#   HOLE-1  two leaves in one file, A ticked above / B unticked        -> exit 0  ⚠ FALSE GREEN
#   HOLE-2  honest ticked leaf below an unticked future placeholder    -> exit 1  ⚠ FALSE RED
#   HOLE-3  HOLE-2 with the placeholder removed                        -> exit 0  (attribution)
#
# ⚠ The HOLE arms assert DEFECTIVE behaviour on purpose: they are the measurement, not an
# endorsement. When the defect is fixed — upstream in the spine, or shadowed locally by the
# SPINE.8 project doctrine (FRESH-ACCEPTANCE-EVIDENCE) — HOLE-1 and HOLE-2 must be re-pointed at
# the corrected verdicts, and this header updated in the same commit. A probe that keeps passing
# against a fixed gate would be reporting the old bug forever.
#
# Usage:  bash docs/tasks/artifacts/task_acceptance/run_multileaf_shadowing_probe.sh
# Output: per-arm lines plus a final `probes: N pass / M fail` (exit nonzero if M > 0).
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
GUARD="$ROOT/scripts/check_task_acceptance.sh"
[ -f "$GUARD" ] || { echo "probe: REFUSED — $GUARD not found" >&2; exit 2; }

WORK="$ROOT/target/doctrine_scratch/multileaf_shadowing"
rm -rf "$WORK"; mkdir -p "$WORK"; trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0

mkrepo() { # $1 = name -> prints the repo dir
  local d="$WORK/$1"
  mkdir -p "$d/scripts" "$d/docs/tasks" "$d/crates/app/src"
  cp "$GUARD" "$d/scripts/check_task_acceptance.sh"
  git -C "$d" init -q .
  git -C "$d" config user.email probe@example.invalid
  git -C "$d" config user.name probe
  printf '# seed\n' > "$d/docs/tasks/SEED.md"
  printf 'fn main() {}\n' > "$d/crates/app/src/main.rs"
  git -C "$d" add -A >/dev/null 2>&1
  git -C "$d" commit -qm "SEED-0001: seed" >/dev/null 2>&1
  printf '%s' "$d"
}

codechange() { printf 'fn main() { println!("owned by the leaf under test"); }\n' > "$1/crates/app/src/main.rs"; }

# A ticked checklist whose bullets carry real tool-output signatures.
ticked() { # $1 = leaf id
  cat <<EOF
### $1

- [x] **ROOT CAUSE (WHY + WHERE)** — \`cargo build\` reported \`error[E0432]\` at crates/app/src/main.rs:4.
- [x] **ADDRESSED (verified)** — before: \`test result: FAILED\`; after: \`test result: ok\`.
- [x] **NO REGRESSION** — \`cargo test\` → \`test result: ok. 42 passed; 0 failed\`, \`rc=0\`.
EOF
}

unticked() { # $1 = leaf id
  cat <<EOF
### $1

- [ ] **ROOT CAUSE (WHY + WHERE)** — pending
- [ ] **ADDRESSED (verified)** — pending
- [ ] **NO REGRESSION** — pending
EOF
}

arm() { # label · repo · wanted exit (or "nonzero") · note
  local label="$1" d="$2" want="$3" note="$4" out rc ok=1
  out="$(cd "$d" && bash scripts/check_task_acceptance.sh 2>&1)"; rc=$?
  case "$want" in
    nonzero) [ "$rc" -ne 0 ] || ok=0 ;;
    *) [ "$rc" -eq "$want" ] || ok=0 ;;
  esac
  if [ "$ok" -eq 1 ]; then
    printf '  ✓ %-7s exit=%s  %s\n' "$label" "$rc" "$note"; pass=$((pass+1))
  else
    printf '  ✗ %-7s exit=%s (wanted %s)  %s\n' "$label" "$rc" "$want" "$note"; fail=$((fail+1))
    printf '%s\n' "$out" | sed 's/^/        /'
  fi
}

echo "multi-leaf shadowing probe — scripts/check_task_acceptance.sh as shipped"

# ---------------------------------------------------------------- CTRL-1: the gate works
d="$(mkrepo ctrl1)"; codechange "$d"
{ echo "# TREE"; echo; ticked "TREE.1"; } > "$d/docs/tasks/TREE.md"
git -C "$d" add -A >/dev/null
arm CTRL-1 "$d" 0 "one leaf, ticked + evidence → accepted (the property the gate exists for)"

# ---------------------------------------------------------------- CTRL-2: evidence-scoped
d="$(mkrepo ctrl2)"; codechange "$d"
{
  echo "# TREE"; echo
  echo "### TREE.1"; echo
  echo "- [x] **ROOT CAUSE (WHY + WHERE)** — the parser mishandles the header."
  echo "- [x] **ADDRESSED (verified)** — it works now."
  echo "- [x] **NO REGRESSION** — the suite is green."
} > "$d/docs/tasks/TREE.md"
git -C "$d" add -A >/dev/null
arm CTRL-2 "$d" nonzero "ticked boxes with NO tool output → refused (box-scoping is real)"

# ---------------------------------------------------------------- CTRL-3: ownership
d="$(mkrepo ctrl3)"; codechange "$d"; git -C "$d" add -A >/dev/null
arm CTRL-3 "$d" nonzero "code with no staged leaf → refused"

# ---------------------------------------------------------------- HOLE-1: false GREEN
d="$(mkrepo hole1)"; codechange "$d"
{
  echo "# TREE"; echo
  ticked "TREE.1 (done earlier — its evidence is about a DIFFERENT change)"; echo
  unticked "TREE.2 (the leaf that owns THIS code change)"; echo
} > "$d/docs/tasks/TREE.md"
git -C "$d" add -A >/dev/null
arm HOLE-1 "$d" 0 "⚠ D15 false GREEN: leaf 2's change accepted on leaf 1's evidence"

# ---------------------------------------------------------------- HOLE-2: false RED
d="$(mkrepo hole2)"; codechange "$d"
{
  echo "# TREE"; echo
  unticked "TREE.9 (a FUTURE leaf, kept as a placeholder)"; echo
  ticked "TREE.1 (the leaf that owns THIS code change, with real evidence)"; echo
} > "$d/docs/tasks/TREE.md"
git -C "$d" add -A >/dev/null
arm HOLE-2 "$d" nonzero "⚠ D15 false RED: an honest, evidenced leaf refused by a placeholder above it"

# ---------------------------------------------------------------- HOLE-3: attribution
d="$(mkrepo hole3)"; codechange "$d"
{
  echo "# TREE"; echo
  ticked "TREE.1 (the leaf that owns THIS code change, with real evidence)"; echo
} > "$d/docs/tasks/TREE.md"
git -C "$d" add -A >/dev/null
arm HOLE-3 "$d" 0 "the same leaf without the placeholder → accepted (the placeholder caused HOLE-2)"

echo
printf 'probes: %d pass / %d fail\n' "$pass" "$fail"
[ "$fail" -eq 0 ] || exit 1
exit 0
