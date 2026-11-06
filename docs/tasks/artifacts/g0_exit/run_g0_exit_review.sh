#!/usr/bin/env bash
# docs/tasks/artifacts/g0_exit/run_g0_exit_review.sh
# G0-CONTRACT.15 — the gate G0 exit review, derived and executed rather than written.
#
# WHY A TOOL AND NOT A PARAGRAPH: the leaf's acceptance is that every clause is `met` with a cited artifact
# or `not met` with a named blocker, and that **no clause is marked met on prose alone**. A review written as
# prose is exactly prose alone, so this instrument runs the check each clause cites and prints its verdict —
# the review is the output of a command anybody can re-run, including a reviewer who did not write the
# chapters. It also closes the roadmap in both directions: every fragment of §11's G0 exit bullet must be
# dispositioned by a row, and every `exit` row's key must appear in a fragment, so a clause cannot be dropped
# from the review by forgetting to mention it.
#
# WHAT IT DOES, mechanically:
#   1. parses ROADMAP.md's G0 `**Exit:**` bullet into fragments (split on `;`, markdown emphasis stripped)
#      and its `Fixture:` bullet as one more fragment;
#   2. reads `g0_exit_clauses.tsv` and refuses a malformed row, an `exit` row whose key matches no fragment,
#      a fragment no row dispositions, or a row that declares a deliverable that does not exist;
#   3. RUNS every `instrument` row's check and records its exit status and last output line;
#   4. prints the clause-by-clause review, then the summary and the gate's status:
#        `G0 EXIT: <met> met / <not met> not met / <clauses> clauses — <verdict>`
#      A `director` row is `not met` with its named blocker, which is the honest state rather than a gap:
#      naming a human is an act this repository has no authority to perform.
#
# SELF-APPLICATION, disclosed rather than hidden: the party running this review authored most of the
# chapters it reviews. Governance §6.1 rule 2 withholds approval of a decision's evidence from its author,
# so the review's output is evidence and the gate's *closure* stays unapproved until an independent reviewer
# or the director accepts it — see `docs/decisions/decision_self-application-under-delegation.md`. The
# mitigation is the shape of this file: every verdict is a command's exit status, so independence is
# available to anybody who runs it.
#
# HONEST LIMITS: a check proves what it checks. `test -f` proves a deliverable exists, not that it is right;
# the rows that can cite a census do, and the rows that cannot say so by citing the weakest check that is
# still a command. The review does not re-run `make gate` or `make probes` (the tree's leaves cite those per
# slice); it runs the clause-level evidence.
#
# OTHER GATES (leaf `G1-SLICE.16.gauge`): `GATE_REVIEW_GATE=G1` … `G6` reviews that gate the same way from
# `gate_exit_clauses.tsv`, which adds an `owner` column and the authority `unbuilt`: no working product path
# yet, so the clause is not met and nothing runs. G0's table, verdict and refusals are unchanged.
#
# Usage:  bash docs/tasks/artifacts/g0_exit/run_g0_exit_review.sh
#         GATE_REVIEW_GATE=<G0…G6> bash …  the gate to review (default G0)
#         G0_EXIT_ROOT=<dir> bash …      a scratch copy of the tree (the probe suites use this)
#         G0_EXIT_CLAUSES=<path> / G0_EXIT_ROADMAP=<path>
#         G0_EXIT_SKIP_CHECKS=1          parse and close the clauses without running the checks (fast path);
#                                        `met` then counts what the table claims, and the verdict says so
# Output: the clause-by-clause review, then
#         `G0 EXIT: <met> met / <not met> not met / <n> clauses — <verdict>`
#         exit 0 when the review completed (whatever it found), 1 on a malformed data plane, 2 on a missing
#         input. The exit status reports the REVIEW, not the gate: a gate with `not met` clauses is a
#         successful review of an open gate.
set -uo pipefail
export LC_ALL=C
TOOL_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd -P)"

ROOT="${G0_EXIT_ROOT:-$TOOL_ROOT}"
GATE="${GATE_REVIEW_GATE:-G0}"
case "$GATE" in
  G0) DEFAULT_CLAUSES=g0_exit_clauses.tsv ;;
  G[1-6]) DEFAULT_CLAUSES=gate_exit_clauses.tsv ;;
  *) echo "g0 exit review: REFUSED — $GATE is not a gate this review covers" >&2; exit 2 ;;
esac
CLAUSES="${G0_EXIT_CLAUSES:-$ROOT/docs/tasks/artifacts/g0_exit/$DEFAULT_CLAUSES}"
ROADMAP="${G0_EXIT_ROADMAP:-$ROOT/ROADMAP.md}"
SKIP="${G0_EXIT_SKIP_CHECKS:-0}"

for f in "$CLAUSES" "$ROADMAP"; do
  [ -f "$f" ] || { echo "g0 exit review: REFUSED — $f not found" >&2; exit 2; }
done
command -v python3 >/dev/null 2>&1 || { echo "g0 exit review: REFUSED — python3 not found" >&2; exit 2; }

cd "$ROOT" || { echo "g0 exit review: REFUSED — cannot enter $ROOT" >&2; exit 2; }

# Prepare every effective store before child checks; verification is fresh, without a trusted marker.
# Closing the clauses alone runs no child, so it needs no store.
if [ "$SKIP" != "1" ] && ! python3 -I -B "$TOOL_ROOT/scripts/local_environment.py" --verify >/dev/null 2>&1; then
  exec python3 -I -B "$TOOL_ROOT/scripts/local_environment.py" -- bash "$TOOL_ROOT/docs/tasks/artifacts/g0_exit/run_g0_exit_review.sh" "$@"
fi

python3 - "$CLAUSES" "$ROADMAP" "$SKIP" "$GATE" <<'PY'
import pathlib, re, subprocess, sys

CLAUSES, ROADMAP, SKIP, GATE = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
fails = 0
def bad(msg):
    global fails
    fails += 1
    print("  x %s" % msg)

def read(p):
    return pathlib.Path(p).read_text(encoding="utf-8")

# ── the roadmap's own clause list, parsed rather than recalled ─────────────────────────────
rm = read(ROADMAP).replace("*", "")
lines = rm.splitlines()
start = next((i for i, l in enumerate(lines) if l.startswith("### %s —" % GATE)), None)
if start is None:
    print("g0 exit review: REFUSED — ROADMAP.md carries no '### %s —' heading" % GATE)
    sys.exit(2)
end = next(i for i, l in enumerate(lines) if i > start and l.startswith("### "))
gate = lines[start:end]

def bullet(needle):
    out, taking = [], False
    for l in gate:
        if l.lstrip().startswith("- ") and needle in l:
            taking = True
            out.append(l.lstrip()[2:])
            continue
        if taking:
            if l.strip() == "" or l.lstrip().startswith("- ") or l.startswith("#"):
                break
            out.append(l.strip())
    return " ".join(out)

if not bullet("Exit:"):
    bad("the roadmap's %s exit bullet did not parse, so this review would be reading nothing" % GATE)
fragments = []
for label in ("Exit:", "Exit (envelope coverage):", "Agent gate"):
    fragments += [f.strip() for f in bullet(label).split(":", 1)[-1].split(";") if f.strip()]
for label in ("Fixture:", "WASM evidence:"):
    if bullet(label):
        fragments.append(bullet(label).strip())
fragments = [re.sub(r"\s+", " ", f) for f in fragments]

# ── the data plane ────────────────────────────────────────────────────────────────────────
WIDTH = 8 if GATE == "G0" else 9
AUTHORITIES = ("instrument", "director") if GATE == "G0" else ("instrument", "director", "unbuilt")
rows = []
for n, line in enumerate(read(CLAUSES).splitlines(), 1):
    if not line.strip() or line.lstrip().startswith("#"):
        continue
    cells = line.split("\t")
    if cells[0] == "id":
        continue
    if len(cells) != WIDTH:
        bad("%s:%d has %d cells, the schema declares %d" % (pathlib.Path(CLAUSES).name, n, len(cells), WIDTH))
        continue
    if not cells[0].startswith(GATE + "-"):
        continue
    rows.append(dict(zip(("id", "source", "key", "deliverable", "check", "authority", "blocker",
                          "note", "owner"), cells)))
tree_ids = set()
if GATE != "G0":
    for path in sorted(pathlib.Path("docs/tasks").glob("*.md")):
        tree_ids.update(re.findall(r"^- ID: `([^`]+)`", read(path), flags=re.M))
for r in rows:
    if r["source"] not in ("exit", "clause"):
        bad("%s declares source %r, which is neither `exit` nor `clause`" % (r["id"], r["source"]))
    if r["authority"] not in AUTHORITIES:
        bad("%s declares authority %r, which is none of %s" % (r["id"], r["authority"], ", ".join(AUTHORITIES)))
    if GATE != "G0" and r["owner"] not in tree_ids:
        bad("%s names owner %r, which is not a task-tree leaf" % (r["id"], r["owner"]))
    if r["authority"] == "instrument" and r["check"] in ("", "-"):
        bad("%s claims an instrument decides it and declares no check" % r["id"])
    if r["authority"] == "director" and r["blocker"] in ("", "-"):
        bad("%s is a human act with no named blocker — an unmet clause with nobody owing it is a gap, "
            "not a verdict" % r["id"])
    for path in [x.strip() for x in r["deliverable"].split(",") if x.strip()]:
        if GATE != "G0" and path == "-":
            continue
        if not pathlib.Path(path).exists():
            bad("%s names the deliverable %r, which does not exist" % (r["id"], path))

# ── closure against the roadmap, both directions ──────────────────────────────────────────
exit_rows = [r for r in rows if r["source"] == "exit"]
def keys(r):
    # a roadmap clause may span two `;`-separated fragments ("governance model drafted (project owner
    # named" / "sewist-vs-programmer review paths defined)"), so one row may carry several keys
    return [k.strip() for k in r["key"].split(",") if k.strip()]
for r in exit_rows:
    for k in keys(r):
        if not any(k in f for f in fragments):
            bad("%s keys on %r, which no fragment of the roadmap's %s exit list contains — a row that "
                "matches nothing reviews nothing" % (r["id"], k, GATE))
for i, f in enumerate(fragments, 1):
    if not any(k in f for r in exit_rows for k in keys(r)):
        bad("fragment %d of the roadmap's %s exit list (%s…) is dispositioned by no row"
            % (i, GATE, f[:56]))

print("=== %s exit review (derived, not written) ===" % GATE)
print("-- the population, read from ROADMAP.md §11")
for i, f in enumerate(fragments, 1):
    print("  %2d. %s" % (i, f[:104]))

# ── run the checks ────────────────────────────────────────────────────────────────────────
met, notmet = [], []
print("-- the clause-by-clause review")
for r in rows:
    if r["authority"] == "director":
        notmet.append(r)
        print("  %-6s NOT MET  %-42s blocked on a human act" % (r["id"], r["deliverable"][:42]))
        print("         blocker: %s" % r["blocker"])
        if r["note"] != "-":
            print("         note:    %s" % r["note"])
        continue
    if r["authority"] == "unbuilt":
        notmet.append(r)
        print("  %-6s NOT MET  no working product path yet; owner %s" % (r["id"], r["owner"]))
        if r["note"] != "-":
            print("         note:    %s" % r["note"])
        continue
    if SKIP == "1":
        met.append(r)
        print("  %-6s SKIPPED  %s (G0_EXIT_SKIP_CHECKS=1)" % (r["id"], r["check"][:70]))
        continue
    proc = subprocess.run(["bash", "-c", r["check"]], capture_output=True, text=True)
    tail = [l for l in (proc.stdout + proc.stderr).splitlines() if l.strip()]
    verdict = tail[-1].strip()[:96] if tail else "(no output)"
    if proc.returncode == 0:
        met.append(r)
        print("  %-6s MET      %-44s %s" % (r["id"], r["deliverable"][:44], verdict))
        if r["note"] != "-":
            print("         note:    %s" % r["note"])
    else:
        notmet.append(r)
        print("  %-6s NOT MET  %-44s exit=%d %s"
              % (r["id"], r["deliverable"][:44], proc.returncode, verdict))

print("-- the review")
print("  clauses: %d (%d from §11's exit list, %d from elsewhere in the roadmap)"
      % (len(rows), len(exit_rows), len(rows) - len(exit_rows)))
for r in notmet:
    reason = "its check failed"
    if r["authority"] == "unbuilt":
        reason = "no working product path yet; owner " + r["owner"]
    elif r["blocker"] != "-":
        reason = r["blocker"]
    print("  · %s is NOT MET — %s" % (r["id"], reason))
gate_open = [r for r in notmet if r["authority"] == "director"]
gate_unbuilt = [r for r in notmet if r["authority"] == "unbuilt"]
gate_failed = [r for r in notmet if r["authority"] == "instrument"]
if fails:
    verdict = "the data plane is malformed, so the review itself is refused"
elif gate_failed:
    verdict = "GATE FAILS: an instrument-backed clause did not pass"
elif SKIP == "1":
    verdict = "checks not run: these counts are what the clause table claims"
elif gate_unbuilt:
    verdict = "%d clause(s) have no working product path yet" % len(gate_unbuilt)
elif gate_open:
    verdict = ("the engineering clauses are met and the gate stays open on %d human act(s) this "
               "repository cannot perform" % len(gate_open))
else:
    verdict = "every clause met"
print("%s EXIT: %d met / %d not met / %d clauses — %s"
      % (GATE, len(met), len(notmet), len(rows), verdict))
print("review status: %s (self-application disclosed: the reviewing party authored most of the chapters;"
      % ("REFUSED — the data plane does not close" if fails else "complete")
      + " governance §6.1 withholds the gate's approval from its author)")
sys.exit(1 if fails else 0)
PY
rc=$?
exit "$rc"
