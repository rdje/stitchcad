#!/usr/bin/env bash
# docs/tasks/artifacts/uncertainty/run_uncertainty_census.sh
# G0-CONTRACT.19 — the census behind two claims the book makes about itself:
#   (1) "uncertainty is data": every claim the book marks as unverified, assumed, unknown or provisional is
#       enumerable, with the authority that resolves it; and
#   (2) "unknown facts are never silently defaulted" (roadmap §2, ontology §5): a marker with no named
#       resolving authority in its own section is a silent default, and is refused.
#
# WHY A CENSUS AND NOT A SENTENCE: the population grows with every chapter, and the seat that resolves most of
# it is VACANT (governance §8.1) — so "what is waiting on a human" is exactly the kind of claim that stays true
# in prose while going false in fact. A reader should be able to ask one command what the project does not know
# yet, and get a list with owners instead of a reassurance. It is also the guard against a marker disappearing
# by accident: an edit that quietly drops `assumed` from a fixture constant promotes a guess to a fact, and
# nothing else in the tree would notice.
#
# WHAT IT CHECKS, mechanically. Each numbered rule fails the census:
#   U1 ownership   every BLOCKING marker (`assumed`, `unknown`, `unverified-with-owner`) inside a
#                  **Verification status** section — the section every spec chapter carries for exactly this
#                  purpose — names a resolving authority from the declared list below, in that section. The
#                  list is closed and each entry carries its reason, because an authority pattern nobody
#                  declared is how a census starts passing on prose that names nobody.
#                  Scope is the whole point: a marker OUTSIDE such a section is usually the chapter defining
#                  the state or naming it as a category, and deciding that a mention IS a claim would be a
#                  classifier guessing at meaning — the mistake the glossary census avoids by making its
#                  bold-span rule an advisory. Those occurrences are printed as A2 for a human to read.
#   U2 vocabulary  every marker the census recognises is one of the states the book's own chapters declare
#                  (ontology §5's five states, the standards chapter's three claim statuses, the matrix's
#                  `(proposed)`, governance's `vacant`); a new spelling is refused rather than silently
#                  uncounted, so the vocabulary cannot drift by typo.
# ADVISORY, printed and never a failure:
#   A1 the whole population — per marker, per file, and per resolving authority — so the count that a human
#      seat is waiting on is read off a run instead of off a memory.
#
# GRANULARITY, declared because a probe arm found it: U1 reads the SECTION, not the bullet, so a claim inherits
# the resolver its section names. Per-bullet scoping would need to decide which sentence owns which authority —
# a classifier guessing at meaning, and the mistake the glossary census avoids. What the census therefore
# guarantees is that no chapter declares statuses without naming who resolves them, not that every sentence is
# individually owned; the `NEW-CHAPTER` probe arm pins the population half of that.
#
# HONEST LIMITS: it reads MARKERS, not meaning. A sentence that hedges without using a marker is invisible to
# it, and a marker used rhetorically ("nothing here is `assumed`") counts as a claim — the second is the safe
# direction, and the first is what review is for. It cannot tell whether an owner is plausible, only that one
# is named; whether the domain expert agrees is governance §8.1's vacant seat, not a census.
#
# Usage:  bash docs/tasks/artifacts/uncertainty/run_uncertainty_census.sh
#         UNCERTAINTY_ROOT=<dir> bash docs/tasks/artifacts/uncertainty/run_uncertainty_census.sh
# Output: per-file and per-marker tables, then
#         `uncertainty census: <markers> markers / <files> files / <unowned> unowned / <failures> failure(s)`
#         exit 1 on any failure, exit 2 when an input is missing.
set -uo pipefail
export LC_ALL=C

ROOT="${UNCERTAINTY_ROOT:-$(git rev-parse --show-toplevel 2>/dev/null || pwd)}"
cd "$ROOT" || { echo "uncertainty census: REFUSED — cannot enter $ROOT" >&2; exit 2; }
BOOK="docs/book/src"
[ -d "$BOOK" ] || { echo "uncertainty census: REFUSED — $BOOK not found" >&2; exit 2; }

python3 - "$BOOK" <<'PY'
import pathlib, re, sys

book = pathlib.Path(sys.argv[1])

# ── the vocabulary, declared here because U2 refuses anything else ────────────────────────────
BLOCKING = ("assumed", "unknown", "unverified-with-owner")   # ontology §5 + standards §1
LABELS = BLOCKING + ("read-in-repo", "cited-from-roadmap", "read-external", "known", "derived")
OTHER = ("(proposed)", "vacant")                            # the matrix's proposal state, governance §8
# name -> compiled matcher, kept as pairs so the census REPORTS the marker as the book spells it and not as
# a regular expression (the first cut printed `unverified\-with\-owner`, which U2 then refused as undeclared
# vocabulary — a census whose own output fails its own rule is a census with a bug, not a finding).
MARKERS = [(m, re.compile(r"`%s`" % re.escape(m))) for m in LABELS] \
        + [(m, re.compile(re.escape(m))) for m in OTHER]

# ── the resolving authorities, each with the reason it counts ─────────────────────────────────
# An authority is somebody who can RESOLVE the claim, not something the sentence mentions. A bare gate id is
# deliberately absent: "frozen as a golden at G2" says WHEN, not WHO, and counting it made the rule almost
# impossible to fail — measured, when this census's own UNOWNED probe arm passed for the wrong reason.
AUTHORITIES = [
    (re.compile(r"domain expert"), "governance §8.1's vacant seat — the only authority for a sewing judgement"),
    (re.compile(r"governance"), "the chapter that assigns every seat and its acting authority"),
    (re.compile(r"director"), "the principal, who names humans and rules on proposals"),
    (re.compile(r"procurement"), "the seat that obtains documents, seats and devices"),
    (re.compile(r"`[A-Z][A-Z0-9]+(?:-[A-Z0-9]+)*\.[0-9]+[a-z]?`"), "a task-tree leaf that owns the resolution"),
    (re.compile(r"decision_[a-z0-9-]+\.md"), "a layer-C record that carries the decision"),
]

def sections(lines):
    """(start, end, heading) per ## / ### section, so 'the same section' is a real scope."""
    marks = [(i, re.sub(r"^#+\s*", "", l).strip()) for i, l in enumerate(lines) if re.match(r"^#{2,4} ", l)]
    out = []
    for k, (i, h) in enumerate(marks):
        end = marks[k + 1][0] if k + 1 < len(marks) else len(lines)
        out.append((i, end, h))
    if not out:
        out = [(0, len(lines), "(no heading)")]
    return out

fails = 0
def bad(msg):
    global fails
    fails += 1
    print("  x %s" % msg)

rows = []          # (file, line, marker, heading, authority or "")
unknown_words = {} # a hedging word the census does not recognise, for U2's report

files = sorted(p for p in book.rglob("*.md"))
for f in files:
    text = f.read_text(encoding="utf-8")
    lines = text.splitlines()
    secs = sections(lines)
    fenced = False
    for n, line in enumerate(lines):
        if line.lstrip().startswith("```"):
            fenced = not fenced
            continue
        if fenced:
            continue
        start, end, head = next(((i, e, h) for i, e, h in secs if i <= n < e), (0, len(lines), "(no heading)"))
        section = "\n".join(lines[start:end])
        for marker, rx in MARKERS:
            if not rx.search(line):
                continue
            auth = next((arx.pattern for arx, _ in AUTHORITIES if arx.search(section)), "")
            rows.append((str(f), n + 1, marker, head, auth))

print("=== uncertainty census ===")
STATUS_SECTION = re.compile(r"[Vv]erification status")
print("-- U1 every blocking marker in a Verification-status section names a resolving authority")
unowned = 0
scope = [r for r in rows if r[2] in BLOCKING and STATUS_SECTION.search(r[3])]
for path, n, marker, head, auth in scope:
    if not auth:
        unowned += 1
        bad("%s:%d marks `%s` in “%s” and that section names no resolving authority — an assumption "
            "nobody owns is a silent default, which roadmap §2 forbids" % (path, n, marker, head[:48]))
print("  blocking markers inside a status section: %d · unowned: %d" % (len(scope), unowned))

print("-- U2 the vocabulary is the one the book declares")
declared = set(LABELS) | set(OTHER)
seen = set(r[2] for r in rows)
extra = seen - declared
for e in sorted(extra):
    bad("marker `%s` is not in the declared vocabulary — add it with its reason or fix the spelling" % e)
print("  distinct markers seen: %d · undeclared: %d" % (len(seen), len(extra)))

print("-- A1 advisory: what is waiting, per marker")
for m in sorted(seen):
    hits = [r for r in rows if r[2] == m]
    print("  · %-22s %2d occurrence(s) in %d file(s)" % (m, len(hits), len(set(r[0] for r in hits))))

print("-- A1 advisory: what is waiting, per file")
for path in sorted(set(r[0] for r in rows)):
    hits = [r for r in rows if r[0] == path]
    print("  · %-52s %2d" % (path, len(hits)))

print("-- A2 advisory: blocking markers OUTSIDE a status section (read them; a mention is not a claim)")
outside = [r for r in rows if r[2] in BLOCKING and not STATUS_SECTION.search(r[3])]
for path, n, marker, head, auth in outside[:12]:
    print("  · %s:%d `%s` in “%s”" % (path, n, marker, head[:52]))
if len(outside) > 12:
    print("  · … %d more" % (len(outside) - 12))
print("  outside a status section: %d" % len(outside))

print("-- A1 advisory: who owes the resolution")
for arx, why in AUTHORITIES:
    hits = [r for r in rows if r[4] == arx.pattern]
    if hits:
        print("  · %-28s %2d — %s" % (arx.pattern, len(hits), why))

print("uncertainty census: %d markers / %d files / %d unowned / %d failure(s)"
      % (len(rows), len(set(r[0] for r in rows)), unowned, fails))
sys.exit(1 if fails else 0)
PY
rc=$?
exit "$rc"
