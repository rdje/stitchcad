#!/usr/bin/env bash
# docs/tasks/artifacts/fresh_evidence/run_fresh_evidence_probes.sh
# SPINE.8 — RED / GREEN / CONTROL probes for scripts/check_fresh_acceptance_evidence.sh, the
# project doctrine that closes defect D15 facet 1 (a code change accepted on evidence committed for
# an EARLIER leaf) without adding a fourth false-red class.
#
# ⭐ CTRL-4 IS THE POINT OF THE DESIGN. The inherited universal check judges EVERY staged leaf file,
#    so a documentation tree co-staged with a code change is held to the same evidence rule
#    (D15 facet 3, measured on commit STITCHCAD-SPINE-0007). A freshness check that required fresh
#    boxes in every staged leaf would multiply that false-red class; requiring them in AT LEAST ONE
#    staged leaf does not. CTRL-4 pins the difference.
# ⭐ RED-1 PRINTS BOTH VERDICTS. The same staged state that the universal check accepts (exit 0, on
#    leaf A's older evidence) is refused by this one (exit 1). Without that pairing the probe would
#    show our check firing, not that it fires where the inherited one cannot.
#
# Each probe builds a throwaway repository under target/doctrine_scratch (repository volume,
# gitignored) because both checks read the STAGED diff.
#
# Usage:  bash docs/tasks/artifacts/fresh_evidence/run_fresh_evidence_probes.sh
# Output: per-arm lines plus `probes: N pass / M fail` (exit nonzero if M > 0).
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
FRESH="$ROOT/scripts/check_fresh_acceptance_evidence.sh"
UNIVERSAL="$ROOT/scripts/check_task_acceptance.sh"
for f in "$FRESH" "$UNIVERSAL"; do
  [ -f "$f" ] || { echo "probe: REFUSED — $f not found" >&2; exit 2; }
done

WORK="$ROOT/target/doctrine_scratch/fresh_evidence_probes"
rm -rf "$WORK"; mkdir -p "$WORK"; trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0

mkrepo() { # $1 = name -> prints the repo dir
  local d="$WORK/$1"
  mkdir -p "$d/scripts" "$d/docs/tasks" "$d/crates/app/src"
  cp "$FRESH" "$d/scripts/check_fresh_acceptance_evidence.sh"
  cp "$UNIVERSAL" "$d/scripts/check_task_acceptance.sh"
  mkdir -p "$d/.doctrine"
  cp "$ROOT/.doctrine/evidence_tokens.txt" "$d/.doctrine/evidence_tokens.txt" 2>/dev/null || true
  cp "$ROOT/.doctrine/code_paths.txt" "$d/.doctrine/code_paths.txt" 2>/dev/null || true
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

# A ticked checklist carrying tool output the signature list recognises on every platform.
fresh_boxes() { # $1 = leaf id
  cat <<EOF
### $1

- [x] **ROOT CAUSE (WHY + WHERE)** — \`cargo test\` reported \`test result: FAILED. 0 passed; 1 failed\`.
- [x] **ADDRESSED (verified)** — after the fix: \`test result: ok. 4 passed; 0 failed\`.
- [x] **NO REGRESSION** — \`make gate\` → \`=== all doctrines green ===\`, \`test result: ok\`.
EOF
}

arm() { # label · repo · wanted exit (or "nonzero") · note
  local label="$1" d="$2" want="$3" note="$4" out rc ok=1
  out="$(cd "$d" && bash scripts/check_fresh_acceptance_evidence.sh 2>&1)"; rc=$?
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

echo "fresh-acceptance-evidence probes — scripts/check_fresh_acceptance_evidence.sh"

# ---------------------------------------------------------------- GREEN-1: the honest commit
d="$(mkrepo green1)"; codechange "$d"
{ echo "# TREE"; echo; fresh_boxes "TREE.1"; } > "$d/docs/tasks/TREE.md"
git -C "$d" add -A >/dev/null
arm GREEN-1 "$d" 0 "code + its own fresh, ticked, evidenced boxes → accepted"

# ---------------------------------------------------------------- GREEN-2: GNU-extension evidence
# The signature list carries `\b` and `{n}` families; they are matched with grep (as the universal
# check does), not awk, which on this platform would not match them at all.
d="$(mkrepo green2)"; codechange "$d"
{
  echo "# TREE"; echo; echo "### TREE.1"; echo
  echo "- [x] **ROOT CAUSE (WHY + WHERE)** — \`cargo build\` printed \`error[E0432]\`; the census exited \`rc=1\`."
  echo "- [x] **ADDRESSED (verified)** — after: \`error[E0432]\` gone, \`rc=0\`."
  echo "- [x] **NO REGRESSION** — \`cargo test\` → \`test result: ok. 4 passed; 0 failed\`."
} > "$d/docs/tasks/TREE.md"
git -C "$d" add -A >/dev/null
arm GREEN-2 "$d" 0 "evidence citing only \\b / {n} families → accepted (grep semantics, not awk)"

# ---------------------------------------------------------------- RED-1: D15 facet 1 closed
# Leaf TREE.1 landed in an EARLIER commit with real evidence; this commit changes code owned by
# TREE.2 and adds no boxes of its own.
d="$(mkrepo red1)"
{ echo "# TREE"; echo; fresh_boxes "TREE.1"; } > "$d/docs/tasks/TREE.md"
git -C "$d" add -A >/dev/null
git -C "$d" commit -qm "TREE-0001 (leaf TREE.1): earlier work, its own evidence" >/dev/null 2>&1
codechange "$d"
printf '\n- ID: `TREE.2`\n  Status: `active`\n' >> "$d/docs/tasks/TREE.md"
git -C "$d" add -A >/dev/null
arm RED-1 "$d" nonzero "code owned by leaf 2, evidence committed for leaf 1 → REFUSED"
uni_rc=0; (cd "$d" && bash scripts/check_task_acceptance.sh >/dev/null 2>&1) || uni_rc=$?
if [ "$uni_rc" -eq 0 ]; then
  printf '        ↳ the inherited universal check accepts this same staged state (exit=%s): the\n' "$uni_rc"
  printf '          freshness check is what closes D15 facet 1, and only the project slot can.\n'
  pass=$((pass+1))
else
  printf '  ✗ PAIRING  the universal check refused too (exit=%s) — the pairing arm is inconclusive\n' "$uni_rc"
  fail=$((fail+1))
fi

# ---------------------------------------------------------------- RED-2: no owning leaf
d="$(mkrepo red2)"; codechange "$d"; git -C "$d" add -A >/dev/null
arm RED-2 "$d" nonzero "code with no leaf staged → refused (defense in depth with OWNERSHIP)"

# ---------------------------------------------------------------- CTRL-1: docs-only commit
d="$(mkrepo ctrl1)"
{ echo "# TREE"; echo; echo "- prose only, no code in this commit."; } > "$d/docs/tasks/TREE.md"
git -C "$d" add -A >/dev/null
arm CTRL-1 "$d" 0 "no staged code change → nothing to judge (a docs commit is never blocked)"

# ---------------------------------------------------------------- CTRL-2: fresh but unticked
d="$(mkrepo ctrl2)"; codechange "$d"
{
  echo "# TREE"; echo; echo "### TREE.1"; echo
  echo "- [ ] **ROOT CAUSE (WHY + WHERE)** — \`test result: FAILED. 0 passed; 1 failed\`."
  echo "- [ ] **ADDRESSED (verified)** — \`test result: ok. 4 passed; 0 failed\`."
  echo "- [ ] **NO REGRESSION** — \`=== all doctrines green ===\`."
} > "$d/docs/tasks/TREE.md"
git -C "$d" add -A >/dev/null
arm CTRL-2 "$d" nonzero "fresh boxes left unticked → refused"

# ---------------------------------------------------------------- CTRL-3: ticked, no evidence
d="$(mkrepo ctrl3)"; codechange "$d"
{
  echo "# TREE"; echo; echo "### TREE.1"; echo
  echo "- [x] **ROOT CAUSE (WHY + WHERE)** — the parser mishandles the header."
  echo "- [x] **ADDRESSED (verified)** — it works now."
  echo "- [x] **NO REGRESSION** — the suite is green."
} > "$d/docs/tasks/TREE.md"
git -C "$d" add -A >/dev/null
arm CTRL-3 "$d" nonzero "fresh ticked boxes with no tool output → refused"

# ---------------------------------------------------------------- CTRL-4: co-staged doc tree
d="$(mkrepo ctrl4)"; codechange "$d"
{ echo "# TREE"; echo; fresh_boxes "TREE.1"; } > "$d/docs/tasks/TREE.md"
{ echo "# OTHER"; echo; echo "- a documentation tree that owns no code, edited in the same commit."; } \
  > "$d/docs/tasks/OTHER.md"
git -C "$d" add -A >/dev/null
arm CTRL-4 "$d" 0 "an unrelated doc leaf co-staged with the owning leaf → still accepted"

echo
printf 'probes: %d pass / %d fail\n' "$pass" "$fail"
[ "$fail" -eq 0 ] || exit 1
exit 0
