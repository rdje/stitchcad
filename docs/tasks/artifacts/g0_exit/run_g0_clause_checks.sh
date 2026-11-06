#!/usr/bin/env bash
# docs/tasks/artifacts/g0_exit/run_g0_clause_checks.sh
# G1-SLICE.16.g0-checks (D392) — what six G0 exit clauses actually ask, in place of "the file exists".
#
# WHY: G0-04, 05, 06, 08, 13 and 17 passed on `test -f` or one `grep -q`, so a chapter could be emptied with its
# clause still counted as met. Each row now checks what its clause asks, reading the roadmap where the roadmap
# lists it, so a clause the roadmap grows reddens the row too.
#
#   G0-04  every object roadmap §3.1 names has a section in the ontology chapter with real content
#   G0-05  both instantiation paths, their equivalence contract and the extreme-size check are specified
#   G0-06  the ownership decision is active and states its rules, the book agrees, and the type exists
#   G0-08  ADR-0001 is active, every crate takes the workspace license, and no dependency is copyleft-only
#   G0-13  review paths, roles, conflict resolution and approval ownership are specified, and seats are named
#   G0-17  hosted CI runs fmt, clippy at deny-warnings, the tests, a real browser build and the doctrines
#
# HONEST LIMITS: these read a specification's structure and substance by size. They cannot tell whether the
# words are right; that is what the reviews in governance §1 are for.
#
# Usage:  bash docs/tasks/artifacts/g0_exit/run_g0_clause_checks.sh <row>
#         G0_CLAUSE_ROOT=<dir> bash …   read a scratch copy (the probe suite uses this)
# Output: one line naming what passed, or `  x` lines naming each fault. Exit 0 when the clause holds.
set -uo pipefail
export LC_ALL=C
TOOL_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd -P)"
ROOT="${G0_CLAUSE_ROOT:-$TOOL_ROOT}"
[ "$#" -eq 1 ] || { echo "g0 clause check: usage: run_g0_clause_checks.sh <row>" >&2; exit 2; }
cd "$ROOT" || exit 2

exec python3 -I -B - "$1" <<'PY'
import json
import pathlib
import re
import subprocess
import sys

MIN_WORDS = 25
faults = []


def read(path):
    target = pathlib.Path(path)
    if not target.is_file():
        faults.append("%s does not exist" % path)
        return ""
    return target.read_text(encoding="utf-8")


def sections(text):
    """Heading → the words beneath it, down to the next heading of any level, fenced examples included."""
    found, heading, body = {}, None, []
    fenced = False
    for line in text.splitlines():
        if line.startswith("```"):
            fenced = not fenced
        match = None if fenced else re.match(r"#{1,6}\s+(.*)", line)
        if match:
            if heading is not None:
                found[heading] = len(" ".join(body).split())
            heading, body = match.group(1), []
        else:
            body.append(line)
    if heading is not None:
        found[heading] = len(" ".join(body).split())
    return found


def need_section(found, name, path):
    matches = [heading for heading in found if name.lower() in heading.lower()]
    if not matches:
        faults.append("%s has no section for %s" % (path, name))
    elif max(found[heading] for heading in matches) < MIN_WORDS:
        faults.append("%s's section for %s has under %d words" % (path, name, MIN_WORDS))


# Roadmap §3.1 calls `walk` and `true` operations, and a corner belongs to its allowance, so these three are
# specified inside the drafting-operations and SeamAllowance sections rather than under headings of their own.
WITHIN_A_SECTION = {"Corner": "**corner treatment**", "Truing": "**`true`**", "Walking": "**`walk`**"}


def roadmap_objects():
    roadmap = read("ROADMAP.md")
    block = re.search(r"^### 3\.1 Core objects\n(.*?)^### ", roadmap, flags=re.M | re.S)
    if not block:
        faults.append("ROADMAP.md has no §3.1 to read the objects from")
        return []
    names = []
    # A bullet's label is every bold run before its first colon, and may wrap across lines.
    for bullet in re.findall(r"^- (.*?)(?=^- |\Z)", block.group(1), flags=re.M | re.S):
        for label in re.findall(r"\*\*([^*]+)\*\*", bullet.split(":", 1)[0]):
            names += [part.strip() for part in re.split(r"[/,+]|\band\b", label) if part.strip()]
    return names


def ontology():
    path = "docs/book/src/spec/ontology.md"
    text = read(path)
    found = sections(text)
    names = roadmap_objects()
    for name in names:
        if name in WITHIN_A_SECTION:
            if WITHIN_A_SECTION[name] not in text:
                faults.append("%s does not specify %s (%s)" % (path, name, WITHIN_A_SECTION[name]))
        else:
            need_section(found, name, path)
    need_section(found, "persistent-identity contract", path)
    return "%d objects from roadmap §3.1 each have a section" % len(names)


def instantiation():
    path = "docs/book/src/spec/instantiation-paths.md"
    found = sections(read(path))
    for name in ("measurement-driven regeneration", "grade-rule instantiation", "equivalence contract", "extreme sizes"):
        need_section(found, name, path)
    return "both paths, the equivalence contract and the extreme-size check are specified"


def size_sets():
    decision = read("docs/decisions/decision_size-set-ownership.md")
    if not re.search(r"\*\*Status:\*\* `active`", decision):
        faults.append("the size-set ownership decision is not active")
    rules = re.findall(r"^\d+\. \*\*", decision, flags=re.M)
    if len(rules) < 3:
        faults.append("the size-set ownership decision states %d rules, under 3" % len(rules))
    book = read("docs/book/src/spec/size-sets.md")
    if len(book.split()) < 200:
        faults.append("the size-sets chapter has under 200 words")
    if "Ownership" not in book and "ownership" not in book:
        faults.append("the size-sets chapter does not state ownership")
    code = "\n".join(path.read_text(encoding="utf-8") for path in pathlib.Path("crates").glob("*/src/**/*.rs"))
    if not re.search(r"\bpub (?:struct|enum) SizeSet\b", code):
        faults.append("no crate defines SizeSet")
    return "the decision is active with %d rules, and the book and the type agree" % len(rules)


def licence():
    decision = read("docs/decisions/decision_adr-0001-license-and-solver.md")
    if not re.search(r"\*\*Status:\*\* `active`", decision):
        faults.append("ADR-0001 is not active")
    if 'license = "MIT OR Apache-2.0"' not in read("Cargo.toml"):
        faults.append("the workspace does not declare MIT OR Apache-2.0")
    manifests = sorted(pathlib.Path("crates").glob("*/Cargo.toml"))
    for path in manifests:
        if not re.search(r"^license\.workspace = true", path.read_text(encoding="utf-8"), flags=re.M):
            faults.append("%s does not take the workspace license" % path)
    done = subprocess.run(["cargo", "metadata", "--format-version", "1", "--offline", "--locked"],
                          capture_output=True, text=True)
    if done.returncode != 0:
        faults.append("cargo metadata failed: %s" % done.stderr.strip()[:160])
        return ""
    packages = json.loads(done.stdout)["packages"]
    for package in packages:
        text = package.get("license") or ""
        if not text:
            faults.append("%s declares no license" % package["name"])
        elif "GPL" in text and " OR " not in text:
            faults.append("%s is copyleft only: %s" % (package["name"], text))
    return "ADR-0001 is active, %d crates take the workspace license, %d packages censused" % (len(manifests), len(packages))


def governance():
    path = "docs/book/src/governance.md"
    text = read(path)
    found = sections(text)
    for name in ("Two review paths", "Roles", "Sewist-versus-programmer conflict resolution",
                 "Golden-file approval ownership", "three seats nobody holds"):
        need_section(found, name, path)
    if not re.search(r"^\|[^|\n]*project owner[^|\n]*\|", text, flags=re.M | re.I):
        faults.append("%s has no table row for the project owner" % path)
    if "vacant" not in text:
        faults.append("%s no longer says which seat is vacant" % path)
    return "review paths, roles, conflict resolution, approval ownership and seats are specified"


def hosted():
    rust = read(".github/workflows/rust.yml")
    for needle, what in (("cargo fmt --all -- --check", "the format check"),
                         ("-- -D warnings", "clippy at deny-warnings"),
                         ("cargo test --all", "the tests"),
                         ("cargo build --target wasm32-unknown-unknown", "a real browser build")):
        if not re.search(r"^\s*(?:run:\s*)?.*%s" % re.escape(needle), rust, flags=re.M):
            faults.append("rust.yml does not run %s" % what)
    doctrines = read(".github/workflows/doctrines.yml")
    if not re.search(r"^\s*run: scripts/check_doctrines\.sh", doctrines, flags=re.M):
        faults.append("doctrines.yml does not run the doctrine enforcer")
    for name, text in (("rust.yml", rust), ("doctrines.yml", doctrines)):
        if not re.search(r"^  push:", text, flags=re.M):
            faults.append("%s does not run on push" % name)
    return "hosted CI runs fmt, clippy, the tests, a browser build and the doctrines on push"


CHECKS = {"G0-04": ontology, "G0-05": instantiation, "G0-06": size_sets, "G0-08": licence,
          "G0-13": governance, "G0-17": hosted}
row = sys.argv[1]
if row not in CHECKS:
    print("g0 clause check: REFUSED — no check for %s" % row, file=sys.stderr)
    sys.exit(2)
summary = CHECKS[row]()
for fault in faults:
    print("  x %s: %s" % (row, fault))
if faults:
    print("%s: %d fault(s)" % (row, len(faults)))
    sys.exit(1)
print("%s: %s" % (row, summary))
PY
