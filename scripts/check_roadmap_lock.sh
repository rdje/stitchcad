#!/usr/bin/env bash
# scripts/check_roadmap_lock.sh — ROADMAP-LOCK (PROJECT doctrine).
#
# THE RULE: every commit names the roadmap clause it advances, and the roadmap, the task tree and the book
# agree about where each clause stands. The director's ruling of 2026-11-06 asks for this after about 650
# commits took the next open leaf instead of the next roadmap clause
# (`docs/decisions/decision_product-work-takes-the-frontier.md`).
#
# WHAT A COMMIT MESSAGE CARRIES:
#   Clause: G1-02            the row of `gate_exit_clauses.tsv` (or G0's table) this commit advances;
#                            several may be listed, separated by commas
#   Depends: G1-02           needed when the clause belongs to a later gate than the earliest open one;
#                            names the open earlier clause that needs the work
#   Clause: none             only with `Repair: D392` (a logged defect) or `Director: 2026-11-06` (a recorded ruling)
#   Book: none — <why>       only when `crates/*/src` changed and the book did not
#
# WHAT IS REFUSED, each with the reason:
#   clause     no `Clause:` line, an unknown row, a later gate without `Depends:`, or a `none` with no reason
#   leaf       the subject's leaf is not in the tree, or its status is `blocked`
#   closure    a gate's rows and the roadmap's exit text no longer close, or a row's owner is not a tree leaf
#   book       the book's gate table or clause table disagrees with the clause tables
#   source     `crates/*/src` changed with no book change and no `Book: none — <why>`
#   depth      a new `G<n>-` leaf more than four levels below its family (an owner and three below it)
#   flip       a row became `instrument` with a check that fails, or that only looks at files
#   stop       50 commits without a rise in met clauses and without a report to the director
#
# HONEST LIMITS: whether a commit's work really advances the clause it names, and whether a named dependency
# is real, stays the author's judgment. The gauge (`scripts/gauge.sh`) is what shows drift: it counts clauses
# met, and only a product path that passes its check raises it.
#
# Usage:  bash scripts/check_roadmap_lock.sh <message-file>      the staged commit (the commit-msg hook)
#         bash scripts/check_roadmap_lock.sh --replay            every commit after this file was added (CI)
#         bash scripts/check_roadmap_lock.sh --self-test         proves each arm fires
#
# CONTRACT (DOCTRINE_ENFORCEMENT.md): exit code is the verdict; explains on stderr; deterministic; read-only;
# path-agnostic. Its own checks and flip runs write only where those checks write.
set -uo pipefail
export LC_ALL=C
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT" || exit 1
[ "$#" -ge 1 ] || { echo "ROADMAP-LOCK: usage: check_roadmap_lock.sh <message-file>|--replay|--self-test" >&2; exit 2; }

exec python3 -I -B - "$@" <<'PY'
import functools
import re
import subprocess
import sys

G0_TABLE = "docs/tasks/artifacts/g0_exit/g0_exit_clauses.tsv"
GATE_TABLE = "docs/tasks/artifacts/g0_exit/gate_exit_clauses.tsv"
REVIEW = "docs/tasks/artifacts/g0_exit/run_g0_exit_review.sh"
BOOK = "docs/book/src/availability.md"
REPORTS = "docs/book/src/distance-reports.md"
RULINGS = "docs/decisions/decision_product-work-takes-the-frontier.md"
THIS = "scripts/check_roadmap_lock.sh"
STOP_AFTER = 50
MAX_DEPTH = 4
GATES = ("G0", "G1", "G2", "G3", "G4", "G5", "G6")


def git(*args, check=True):
    done = subprocess.run(("git",) + args, capture_output=True, text=True, encoding="utf-8")
    if check and done.returncode != 0:
        raise SystemExit("ROADMAP-LOCK: git %s failed: %s" % (" ".join(args), done.stderr.strip()))
    return done.stdout


class Tree:
    """A commit's tree, or the index for the commit being made."""

    def __init__(self, rev):
        self.rev = rev  # None is the index

    def read(self, path):
        spec = ":" + path if self.rev is None else "%s:%s" % (self.rev, path)
        done = subprocess.run(("git", "show", spec), capture_output=True, text=True, encoding="utf-8")
        return done.stdout if done.returncode == 0 else None

    def leaves(self):
        where = ["--cached"] if self.rev is None else []
        what = [] if self.rev is None else [self.rev]
        args = ["grep", "-h", "-A1"] + where + ["-E", "-e", r"^- ID: `"] + what + ["--", "docs/tasks"]
        done = subprocess.run(("git",) + tuple(args), capture_output=True, text=True, encoding="utf-8")
        if done.returncode not in (0, 1):
            raise SystemExit("ROADMAP-LOCK: git grep failed: %s" % done.stderr.strip())
        found, current = {}, None
        for line in done.stdout.splitlines():
            match = re.match(r"- ID: `([^`]+)`", line)
            if match:
                current = match.group(1)
                found.setdefault(current, "")
                continue
            status = re.match(r"\s*Status: `([^`]+)`", line)
            if status and current:
                found[current] = status.group(1)
        return found


def rows_of(text):
    rows = {}
    for line in (text or "").splitlines():
        if not line.strip() or line.startswith("#"):
            continue
        cells = line.split("\t")
        if cells[0] == "id" or len(cells) < 8:
            continue
        rows[cells[0]] = {"authority": cells[5], "check": cells[4], "owner": cells[8] if len(cells) > 8 else "-"}
    return rows


def gate_of(clause):
    return clause.split("-", 1)[0]


def earliest_open(rows):
    for gate in GATES:
        if any(gate_of(key) == gate and row["authority"] == "unbuilt" for key, row in rows.items()):
            return gate
    return None


def counts(rows):
    table = {}
    for key, row in rows.items():
        gate = gate_of(key)
        met, total = table.get(gate, (0, 0))
        table[gate] = (met + (row["authority"] == "instrument"), total + 1)
    return table


def lines_named(message, label):
    found = []
    for line in message.splitlines():
        if line.startswith("#"):
            continue
        match = re.match(r"%s:\s*(.*\S)\s*$" % label, line)
        if match:
            found.append(match.group(1))
    return found


def clause_rule(message, rows, rulings, known_defect):
    """Every commit names a clause;a later gate needs a named dependency;`none` needs a reason."""
    named = lines_named(message, "Clause")
    if not named:
        return ["clause: no `Clause:` line. Name the row this advances, or `Clause: none` with `Repair: Dnnn` or `Director: <date>`"]
    clauses = [part.strip() for line in named for part in line.split(",") if part.strip()]
    if clauses == ["none"]:
        repairs = lines_named(message, "Repair")
        directors = lines_named(message, "Director")
        if not repairs and not directors:
            return ["clause: `Clause: none` needs `Repair: Dnnn` or `Director: <date>`"]
        problems = []
        for repair in repairs:
            if not re.fullmatch(r"D\d+", repair) or not known_defect(repair):
                problems.append("clause: `Repair: %s` names no logged defect" % repair)
        for date in directors:
            if "## Director ruling — %s" % date not in (rulings or ""):
                problems.append("clause: `Director: %s` names no recorded ruling in %s" % (date, RULINGS))
        return problems
    problems = []
    frontier = earliest_open(rows)
    depends = [part.strip() for line in lines_named(message, "Depends") for part in line.split(",") if part.strip()]
    for clause in clauses:
        if clause not in rows:
            problems.append("clause: %s is not a row of the clause tables" % clause)
            continue
        if frontier is None or GATES.index(gate_of(clause)) <= GATES.index(frontier):
            continue
        earlier = [d for d in depends if d in rows and rows[d]["authority"] == "unbuilt"
                   and GATES.index(gate_of(d)) < GATES.index(gate_of(clause))]
        if not earlier:
            problems.append("clause: %s belongs to %s while %s is the earliest open gate; a `Depends:` line must name "
                            "the open earlier clause that needs it" % (clause, gate_of(clause), frontier))
    return problems


def leaf_rule(subject, leaves):
    match = re.search(r"\(leaf ([^)\s]+)\)", subject)
    if not match:
        return ["leaf: the subject names no `(leaf <ID>)`"]
    leaf = match.group(1)
    if leaf not in leaves:
        return ["leaf: %s is not in the task tree" % leaf]
    if leaves[leaf] == "blocked":
        return ["leaf: %s is `blocked`; the director's hold or its named blocker has to lift first" % leaf]
    return []


def owner_rule(rows, leaves):
    return ["closure: %s names owner %s, which is not a tree leaf" % (key, row["owner"])
            for key, row in rows.items() if row["owner"] != "-" and row["owner"] not in leaves]


def book_rule(book, rows):
    """The book's gate table and the frontier gate's clause table state what the clause tables state."""
    problems = []
    cells = {}
    for line in (book or "").splitlines():
        if line.startswith("|"):
            parts = [part.strip() for part in line.strip().strip("|").split("|")]
            cells[parts[0].strip("`")] = parts
    for gate, (met, total) in sorted(counts(rows).items()):
        row = cells.get(gate)
        if row is None or len(row) < 3 or row[1] != str(met) or row[2] != str(total):
            shown = "absent" if row is None else "%s of %s" % (row[1], row[2] if len(row) > 2 else "?")
            problems.append("book: %s's gate row says %s;the clause tables say %d of %d" % (gate, shown, met, total))
    frontier = earliest_open(rows)
    for key, row in rows.items():
        if gate_of(key) != frontier:
            continue
        stated = cells.get(key)
        wanted = "met" if row["authority"] == "instrument" else "not met"
        if stated is None or len(stated) < 3 or stated[2] != wanted:
            problems.append("book: clause %s must state `%s` in %s" % (key, wanted, BOOK))
    return problems


def source_rule(message, changed):
    code = [path for path in changed if re.match(r"crates/[^/]+/src/", path)]
    if not code or any(path.startswith("docs/book/src/") for path in changed):
        return []
    if any(re.match(r"none — \S", reason) for reason in lines_named(message, "Book")):
        return []
    return ["source: %s changed with no book change; add the book's words, or `Book: none — <why>`" % code[0]]


def depth_rule(added):
    problems = []
    for leaf in added:
        if not re.match(r"G\d-", leaf):
            continue
        if len(leaf.split(".")) - 1 > MAX_DEPTH:
            problems.append("depth: %s is deeper than an owner and three levels; build it, or attach it higher" % leaf)
    return problems


def flip_problems(before, after, run):
    """A clause reaches `met` only through a check that runs the product and passes."""
    problems = []
    for key, row in after.items():
        if row["authority"] != "instrument" or before.get(key, {}).get("authority") == "instrument":
            continue
        check = row["check"]
        if not re.search(r"\b(cargo|bash|python3)\b", check) or re.fullmatch(r"\s*(test|grep)\b.*", check):
            problems.append("flip: %s only looks at files: %s" % (key, check[:70]))
        elif run(check) != 0:
            problems.append("flip: %s's check failed: %s" % (key, check[:70]))
    return problems


def stop_rule(recent, raises, touches_report):
    """50 commits without a rise,or a report saying why,stop the next commit."""
    if len(recent) < STOP_AFTER:
        return []
    if any(raises(sha) or touches_report(sha) for sha in recent):
        return []
    return ["stop: %d commits since the last met clause or report. Record what stands in the way in %s, "
            "which the director reads, then continue" % (STOP_AFTER, REPORTS)]


def all_rows(tree):
    rows = rows_of(tree.read(G0_TABLE))
    rows.update(rows_of(tree.read(GATE_TABLE)))
    return rows


def closure_problems():
    problems = []
    for gate in GATES:
        environment = {"GATE_REVIEW_GATE": gate, "G0_EXIT_SKIP_CHECKS": "1"}
        done = subprocess.run(["env"] + ["%s=%s" % pair for pair in environment.items()] + ["bash", REVIEW],
                              capture_output=True, text=True, encoding="utf-8")
        if done.returncode != 0:
            shown = [line.strip() for line in done.stdout.splitlines() if line.startswith("  x ")][:3]
            problems.append("closure: %s's clause rows and the roadmap do not close: %s" % (gate, "; ".join(shown) or done.stderr.strip()[:160]))
    return problems


def instrument_count(tree):
    return sum(row["authority"] == "instrument" for row in rows_of(tree.read(GATE_TABLE)).values())


def lock_epoch():
    added = git("log", "--diff-filter=A", "--format=%H", "--", THIS, check=False).split()
    return added[-1] if added else None


def recent_commits(head, epoch):
    args = ["rev-list", "--max-count=%d" % STOP_AFTER, head]
    if epoch:
        args.append("^" + epoch)
    return git(*args, check=False).split()


@functools.lru_cache(maxsize=None)
def raises(sha):
    return instrument_count(Tree(sha)) > instrument_count(Tree(sha + "^"))


@functools.lru_cache(maxsize=None)
def touches_report(sha):
    return bool(git("diff-tree", "--no-commit-id", "--name-only", "-r", sha, "--", REPORTS, check=False).strip())


def run_shell(check):
    return subprocess.run(["bash", "-c", check], capture_output=True, text=True).returncode


def judge(message, tree, parent, changed, run, epoch):
    subject = message.splitlines()[0] if message.splitlines() else ""
    rows = all_rows(tree)
    leaves = tree.leaves()
    logged = (tree.read("docs/tasks/PLANNING.md") or "") + (tree.read("docs/tasks/PLANNING-evidence.md") or "")
    known = lambda defect: re.search(r"\b%s\b" % defect, logged) is not None
    problems = clause_rule(message, rows, tree.read(RULINGS), known)
    problems += leaf_rule(subject, leaves)
    problems += owner_rule(rows, leaves)
    problems += book_rule(tree.read(BOOK), rows)
    problems += source_rule(message, changed)
    problems += depth_rule(sorted(set(leaves) - set(parent.leaves())))
    problems += flip_problems(all_rows(parent), rows, run)
    return problems


def staged():
    message = open(sys.argv[1], encoding="utf-8").read()
    changed = git("diff", "--cached", "--name-only").split("\n")
    tree, parent = Tree(None), Tree("HEAD")
    problems = judge(message, tree, parent, changed, run_shell, lock_epoch())
    problems += closure_problems()
    if instrument_count(tree) <= instrument_count(parent) and REPORTS not in changed:
        problems += stop_rule(recent_commits("HEAD", lock_epoch()), raises, touches_report)
    return problems


def replay():
    epoch = lock_epoch()
    if epoch is None:
        return []
    problems = []
    for sha in git("rev-list", "--reverse", "HEAD", "^" + epoch).split():
        message = git("log", "-1", "--format=%B", sha)
        changed = git("diff-tree", "--no-commit-id", "--name-only", "-r", sha).split("\n")
        # A replay never runs flip checks:the gauge runs every met clause's check on hosted CI.
        found = judge(message, Tree(sha), Tree(sha + "^"), changed, lambda _check: 0, epoch)
        found += stop_rule(recent_commits(sha + "^", epoch), raises, touches_report) if not (raises(sha) or touches_report(sha)) else []
        problems += ["%s %s" % (sha[:8], problem) for problem in found]
    return problems + closure_problems()


def self_test():
    rows = {"G1-01": {"authority": "unbuilt", "check": "-", "owner": "G1-SLICE.11"},
            "G2-01": {"authority": "unbuilt", "check": "-", "owner": "G2-2D.8"},
            "G0-01": {"authority": "instrument", "check": "bash x", "owner": "-"}}
    known = lambda defect: defect == "D392"
    rulings = "## Director ruling — 2026-11-06: stay on the roadmap\n"
    book_ok = "| G0 | 1 | 1 |\n| G1 | 0 | 1 |\n| G2 | 0 | 1 |\n| `G1-01` | asks | not met | x | y |\n"
    results = []

    def arm(name, found, wanted):
        results.append((name, (wanted in " ".join(found)) if wanted else not found, found))

    arm("named clause passes", clause_rule("s\n\nClause: G1-01\n", rows, rulings, known), None)
    arm("no clause refused", clause_rule("s\n", rows, rulings, known), "no `Clause:` line")
    arm("unknown clause refused", clause_rule("s\n\nClause: G9-99\n", rows, rulings, known), "not a row")
    arm("later gate refused", clause_rule("s\n\nClause: G2-01\n", rows, rulings, known), "earliest open gate")
    arm("later gate with open dependency passes", clause_rule("s\n\nClause: G2-01\nDepends: G1-01\n", rows, rulings, known), None)
    arm("dependency on a met clause refused", clause_rule("s\n\nClause: G2-01\nDepends: G0-01\n", rows, rulings, known), "earliest open gate")
    arm("earlier gate passes", clause_rule("s\n\nClause: G0-01\n", rows, rulings, known), None)
    arm("none alone refused", clause_rule("s\n\nClause: none\n", rows, rulings, known), "needs `Repair")
    arm("none with a logged defect passes", clause_rule("s\n\nClause: none\nRepair: D392\n", rows, rulings, known), None)
    arm("none with an unlogged defect refused", clause_rule("s\n\nClause: none\nRepair: D999\n", rows, rulings, known), "no logged defect")
    arm("none with a recorded ruling passes", clause_rule("s\n\nClause: none\nDirector: 2026-11-06\n", rows, rulings, known), None)
    arm("none with an unrecorded ruling refused", clause_rule("s\n\nClause: none\nDirector: 2026-01-01\n", rows, rulings, known), "no recorded ruling")
    arm("a comment is not a clause", clause_rule("s\n\n# Clause: G1-01\n", rows, rulings, known), "no `Clause:` line")
    arm("open leaf passes", leaf_rule("X-1 (leaf A.1): s", {"A.1": "pending"}), None)
    arm("no leaf refused", leaf_rule("X-1: s", {"A.1": "pending"}), "names no")
    arm("unknown leaf refused", leaf_rule("X-1 (leaf B.1): s", {"A.1": "pending"}), "not in the task tree")
    arm("blocked leaf refused", leaf_rule("X-1 (leaf A.1): s", {"A.1": "blocked"}), "is `blocked`")
    arm("owner in the tree passes", owner_rule(rows, {"G1-SLICE.11": "", "G2-2D.8": ""}), None)
    arm("missing owner refused", owner_rule(rows, {"G1-SLICE.11": ""}), "not a tree leaf")
    arm("book agrees", book_rule(book_ok, rows), None)
    arm("book count wrong", book_rule(book_ok.replace("| G1 | 0 | 1 |", "| G1 | 1 | 1 |"), rows), "G1's gate row")
    arm("book gate absent", book_rule(book_ok.replace("| G2 | 0 | 1 |\n", ""), rows), "G2's gate row says absent")
    arm("book clause state wrong", book_rule(book_ok.replace("not met", "met"), rows), "must state `not met`")
    arm("book clause absent", book_rule(book_ok.replace("| `G1-01` | asks | not met | x | y |\n", ""), rows), "must state")
    arm("source with book passes", source_rule("s", ["crates/a/src/x.rs", "docs/book/src/y.md"]), None)
    arm("source without book refused", source_rule("s", ["crates/a/src/x.rs"]), "with no book change")
    arm("source with a stated reason passes", source_rule("s\n\nBook: none — a private rename\n", ["crates/a/src/x.rs"]), None)
    arm("empty reason refused", source_rule("s\n\nBook: none\n", ["crates/a/src/x.rs"]), "with no book change")
    arm("tests alone pass", source_rule("s", ["crates/a/tests/x.rs"]), None)
    arm("depth at the limit passes", depth_rule(["G1-SLICE.8.a.b.c"]), None)
    arm("too deep refused", depth_rule(["G1-SLICE.8.a.b.c.d"]), "deeper than")
    arm("other trees are not capped", depth_rule(["PLANNING.a.b.c.d.e.f"]), None)
    good = {"G1-01": {"authority": "instrument", "check": "cargo test -p x", "owner": "o"}}
    weak = {"G1-01": {"authority": "instrument", "check": "test -f crates/x", "owner": "o"}}
    grep = {"G1-01": {"authority": "instrument", "check": "grep -q cargo docs/x", "owner": "o"}}
    arm("flip with a passing check passes", flip_problems(rows, good, lambda _: 0), None)
    arm("flip with a failing check refused", flip_problems(rows, good, lambda _: 1), "check failed")
    arm("flip that only looks at files refused", flip_problems(rows, weak, lambda _: 0), "only looks at files")
    arm("flip that greps a word refused", flip_problems(rows, grep, lambda _: 0), "only looks at files")
    arm("no flip runs nothing", flip_problems(good, good, lambda _: 1), None)
    arm("fewer than the limit passes", stop_rule(["a"] * 49, lambda _: False, lambda _: False), None)
    arm("the limit stops", stop_rule(["a"] * 50, lambda _: False, lambda _: False), "50 commits")
    arm("a rise inside the limit passes", stop_rule(["a"] * 49 + ["b"], lambda sha: sha == "b", lambda _: False), None)
    arm("a report inside the limit passes", stop_rule(["a"] * 49 + ["b"], lambda _: False, lambda sha: sha == "b"), None)
    arm("frontier moves", [] if earliest_open({"G1-01": good["G1-01"], "G2-01": rows["G2-01"]}) == "G2" else ["wrong"], None)
    # The real tree, read both ways: a Git argument slip once read an empty tree and blamed every owner.
    for name, tree in (("the index", Tree(None)), ("HEAD", Tree("HEAD"))):
        arm("%s has leaves" % name, [] if "G1-SLICE.8" in tree.leaves() else ["no leaves read"], None)
        arm("%s has clause rows" % name, [] if "G0-01" in all_rows(tree) else ["no rows read"], None)
    failed = [item for item in results if not item[1]]
    for name, _, found in failed:
        print("ROADMAP-LOCK self-test FAILED: %s: %r" % (name, found), file=sys.stderr)
    print("roadmap lock self-test: %d arms, %d failed" % (len(results), len(failed)))
    return 1 if failed else 0


if sys.argv[1] == "--self-test":
    sys.exit(self_test())
found = replay() if sys.argv[1] == "--replay" else staged()
for problem in found:
    print("ROADMAP-LOCK: %s" % problem, file=sys.stderr)
if found:
    print("ROADMAP-LOCK: the commit message and the clause tables are in scripts/check_roadmap_lock.sh's header", file=sys.stderr)
sys.exit(1 if found else 0)
PY
