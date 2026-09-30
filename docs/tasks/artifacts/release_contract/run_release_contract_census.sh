#!/usr/bin/env bash
# docs/tasks/artifacts/release_contract/run_release_contract_census.sh
# G0-CONTRACT.12 — the census behind the release contract's claim to be the roadmap's §9 and §8.2.
#
# WHY A TOOL AND NOT A RE-READ: this chapter realises two roadmap clauses that are *lists* — nine manifest
# fields and six acceptance states — plus §8.2's example policy matrix. A list realised in prose drifts: a
# field is renamed, a rung is merged, an artifact class is forgotten, and every gate stays green because
# nothing compares the two documents. So the census parses the roadmap itself and compares, in both
# directions: a field the roadmap names and the chapter lacks is a refusal, and so is a field the chapter
# invents and attributes to the roadmap. The same shape closed ADR-0004's layer table (`G0-CONTRACT.10`).
#
# WHAT IT CHECKS, mechanically. Each numbered rule fails the run:
#   C1 manifest    the chapter's "Roadmap §9 field" column and the roadmap's own comma-separated list are
#                  the same set, parsed with parenthesis awareness (one field carries a nested comma).
#   C2 ladder      the chapter's §5 state column equals, IN ORDER, the arrow chain roadmap §9 names — an
#                  acceptance ladder whose order drifted is a different ladder.
#   C3 matrix      every artifact class in the roadmap's §8.2 header is a column of the chapter's matrix;
#                  every row of the roadmap's example is mapped by the chapter's tuning table; every
#                  uncertainty state the ontology §5 declares is dispositioned in the chapter's §8.
#   C4 vocabulary  every cell of the chapter's matrix is a disposition its own table declares, and every
#                  declared disposition is used by at least one cell — a word in the vocabulary that no
#                  cell carries is a word nobody can look up.
#   C5 diagnostics every `release_*` / `env_*` token the chapter names is declared by its §9 or by the
#                  envelope's §10, and every §9 row names the arguments its diagnostic carries.
#   C6 references  every link resolves, and a `§n` inside a link names a heading the target carries.
# ADVISORY, printed and never a failure:
#   A1 the manifest, the ladder and the matrix as this run read them, so a human can eyeball the whole
#      contract instead of trusting the summary line.
#
# HONEST LIMITS: it proves the chapter agrees with the roadmap, with the ontology and with itself. It
# cannot tell whether a disposition is the right one for a factory — the matrix's cells are project
# decisions tuned at G4 against evidence this repository does not have yet, and §10 of the chapter says so.
# The roadmap's §9 bullet is parsed by shape ("Package = immutable manifest:" then a comma list), so a
# rewrite of that bullet is reported as a parse failure and refused rather than silently passing.
#
# Usage:  bash docs/tasks/artifacts/release_contract/run_release_contract_census.sh
#         RELEASE_ROOT=<dir> bash …        a scratch copy of the tree (the probe suites use this)
#         RELEASE_BOOK=<dir> / RELEASE_CHAPTER=<path> / RELEASE_ROADMAP=<path>
# Output: per-section tables, then
#         `release-contract census: <fields> manifest fields / <states> states / <rows> matrix rows / <n> failure(s)`
#         exit 1 on any failure, exit 2 when an input is missing.
set -uo pipefail
export LC_ALL=C

ROOT="${RELEASE_ROOT:-$(git rev-parse --show-toplevel 2>/dev/null || pwd)}"
BOOK="${RELEASE_BOOK:-$ROOT/docs/book/src}"
CHAPTER="${RELEASE_CHAPTER:-$BOOK/spec/release-contract.md}"
ONTOLOGY="${RELEASE_ONTOLOGY:-$BOOK/spec/ontology.md}"
MATRIX="${RELEASE_MATRIX:-$BOOK/spec/feature-matrix.md}"
ROADMAP="${RELEASE_ROADMAP:-$ROOT/ROADMAP.md}"

for f in "$CHAPTER" "$ONTOLOGY" "$MATRIX" "$ROADMAP"; do
  [ -f "$f" ] || { echo "release-contract census: REFUSED — $f not found" >&2; exit 2; }
done
command -v python3 >/dev/null 2>&1 || { echo "release-contract census: REFUSED — python3 not found" >&2; exit 2; }

python3 - "$CHAPTER" "$ONTOLOGY" "$MATRIX" "$ROADMAP" <<'PY'
import pathlib, re, sys

CHAPTER, ONTOLOGY, MATRIX, ROADMAP = sys.argv[1:5]
fails = 0
def bad(msg):
    global fails
    fails += 1
    print("  x %s" % msg)

def read(p):
    return pathlib.Path(p).read_text(encoding="utf-8")

def debacktick(s):
    return s.replace("`", "").strip()

def norm(s):
    return re.sub(r"\s+", " ", debacktick(s)).strip().rstrip(".")

def code_spans(text):
    out, fenced = [], False
    for line in text.splitlines():
        if line.lstrip().startswith("```"):
            fenced = not fenced
            continue
        if not fenced:
            out.extend(re.findall(r"`([^`]+)`", line))
    return out

def split_row(ln):
    out, buf, in_code, i = [], [], False, 0
    while i < len(ln):
        c = ln[i]
        if c == "\\" and i + 1 < len(ln) and ln[i + 1] == "|":
            buf.append("|"); i += 2; continue
        if c == "`":
            in_code = not in_code; buf.append(c); i += 1; continue
        if c == "|" and not in_code:
            out.append("".join(buf).strip()); buf = []; i += 1; continue
        buf.append(c); i += 1
    out.append("".join(buf).strip())
    if out and out[0] == "": out = out[1:]
    if out and out[-1] == "": out = out[:-1]
    return out

def tables(lines):
    out, header, rows = [], None, []
    for ln in list(lines) + [""]:
        if ln.lstrip().startswith("|"):
            cells = split_row(ln.strip())
            if cells and all(re.fullmatch(r":?-{2,}:?", c.strip()) for c in cells if c.strip()):
                continue
            if header is None: header = cells
            else: rows.append(cells)
        elif header is not None:
            out.append((header, rows)); header, rows = None, []
    return out

def sections(path):
    out, cur = {}, None
    for ln in read(path).splitlines():
        m = re.match(r"^#{2,4} (\d+(?:\.\d+)*)\.?\s", ln)
        if m:
            cur = m.group(1); out[cur] = []
        elif cur is not None:
            out[cur].append(ln)
    return out

def headings(path):
    return {m.group(1) for m in
            (re.match(r"^#{2,4} (\d+(?:\.\d+)*)\.?\s", l) for l in read(path).splitlines()) if m}

def table_in(body, first_cell):
    for header, rows in tables(body):
        if header and debacktick(header[0]) == first_cell:
            return header, rows
    return [], []

print("=== release-contract census ===")
CH, ON, MX = sections(CHAPTER), sections(ONTOLOGY), sections(MATRIX)
for name, sec, need in (("the release chapter", CH, ("2", "5", "8", "9")),
                        ("the ontology", ON, ("5",)),
                        ("the envelope", MX, ("10",))):
    missing = [c for c in need if c not in sec]
    if missing:
        print("release-contract census: REFUSED — %s carries no §%s" % (name, ", §".join(missing)))
        sys.exit(2)

# ── the roadmap's own lists, parsed rather than recalled ───────────────────────────────────
rm = read(ROADMAP)
rm_sec = sections(ROADMAP)
if "9" not in rm_sec or "8.2" not in rm_sec:
    print("release-contract census: REFUSED — ROADMAP.md carries no §9 or §8.2")
    sys.exit(2)

def join_bullet(body, needle):
    """The whole of a wrapped bullet: the first cut of the interchange census read one line of one
    paragraph and reported 21 breaches against a chapter that was right."""
    out, taking = [], False
    for ln in body:
        if ln.lstrip().startswith("- ") and needle in ln:
            taking = True
            out.append(ln.lstrip()[2:])
            continue
        if taking:
            if ln.strip() == "" or ln.lstrip().startswith("- ") or ln.startswith("#"):
                break
            out.append(ln.strip())
    return " ".join(out)

manifest_bullet = join_bullet(rm_sec["9"], "immutable manifest")
if not manifest_bullet:
    bad("C1 ROADMAP.md §9 carries no 'immutable manifest' bullet for this census to read")
tail = manifest_bullet.split("immutable manifest:", 1)[-1] if "immutable manifest:" in manifest_bullet else ""
roadmap_fields, depth, cur = [], 0, []
for ch in tail:
    if ch == "(": depth += 1
    elif ch == ")": depth -= 1
    if ch == "," and depth == 0:
        roadmap_fields.append(norm("".join(cur))); cur = []
        continue
    cur.append(ch)
roadmap_fields.append(norm("".join(cur)))
roadmap_fields = [f for f in roadmap_fields if f]

state_bullet = join_bullet(rm_sec["9"], "Acceptance states are graduated")
chain = state_bullet.split("graduated:", 1)[-1] if "graduated:" in state_bullet else ""
roadmap_states = [norm(s) for s in chain.split("→") if norm(s)]
roadmap_states = [s.split(". ")[0].strip() for s in roadmap_states]

rm_hdr, rm_rows = table_in(rm_sec["8.2"], "Unknown affects")
roadmap_classes = [norm(c) for c in rm_hdr[1:]]
roadmap_matrix_rows = [norm(r[0]) for r in rm_rows if r]
if not roadmap_fields or not roadmap_states or not roadmap_classes:
    bad("C1 the roadmap parsed to %d field(s), %d state(s) and %d artifact class(es) — the shape changed, "
        "so this census is reading nothing" % (len(roadmap_fields), len(roadmap_states),
                                               len(roadmap_classes)))
print("-- the population, read from ROADMAP.md and the ontology")
print("  manifest fields: %d · acceptance states: %d · artifact classes: %s"
      % (len(roadmap_fields), len(roadmap_states), " / ".join(roadmap_classes)))
print("  the roadmap's example matrix rows: %s" % ", ".join(roadmap_matrix_rows))

# ── C1 the manifest ───────────────────────────────────────────────────────────────────────
hdr, rows = table_in(CH["2"], "Field")
manifest_rows = rows
if len(hdr) < 4:
    bad("C1 the chapter's §2 table is not `Field | Roadmap §9 field | Carries | Source`")
chapter_fields = [norm(r[1]) for r in rows if len(r) >= 2]
c1 = 0
for f in roadmap_fields:
    if f not in chapter_fields:
        bad("C1 the roadmap names the manifest field %r and §2 carries no row quoting it" % f); c1 += 1
for f in chapter_fields:
    if f not in roadmap_fields:
        bad("C1 §2 attributes %r to roadmap §9, which does not name it — a field this chapter adds is a "
            "project decision and must not wear the roadmap's authority" % f); c1 += 1
for r in rows:
    if len(r) >= 4 and (not r[2].strip() or not r[3].strip()):
        bad("C1 the manifest field %r has no source or no content" % debacktick(r[0])); c1 += 1
print("-- C1 the manifest is the roadmap's list")
print("  fields: %d · breaches: %d" % (len(manifest_rows), c1))

# ── C2 the ladder, in order ───────────────────────────────────────────────────────────────
hdr, rows = table_in(CH["5"], "State")
chapter_states = [norm(r[0]) for r in rows if r]
c2 = 0
if chapter_states != roadmap_states:
    bad("C2 the chapter's ladder is %s and the roadmap's is %s — an acceptance ladder whose order drifted "
        "is a different ladder" % (" → ".join(chapter_states), " → ".join(roadmap_states)))
    c2 += 1
for r in rows:
    if len(r) < 4 or not r[2].strip() or not r[3].strip():
        bad("C2 the state %r declares no evidence or no granter" % debacktick(r[0])); c2 += 1
print("-- C2 the acceptance ladder")
print("  states: %d (%s) · breaches: %d" % (len(chapter_states), " → ".join(chapter_states), c2))

# ── C3 the policy matrix against §8.2 and the ontology's states ───────────────────────────
m_hdr, m_rows = table_in(CH["8"], "Unknown affects")
chapter_classes = [norm(c) for c in m_hdr[1:]]
c3 = 0
for cls in roadmap_classes:
    if cls not in chapter_classes:
        bad("C3 the roadmap's §8.2 names the artifact class %r and the chapter's matrix has no such "
            "column" % cls); c3 += 1
map_hdr, map_rows = table_in(CH["8"], "Roadmap §8.2 row")
mapped = [norm(r[0]) for r in map_rows if r]
for r in roadmap_matrix_rows:
    if r not in mapped:
        bad("C3 the roadmap's example row %r is not mapped by §8's tuning table, so the tuning is silent "
            "about it" % r); c3 += 1
for r in mapped:
    if r not in roadmap_matrix_rows:
        bad("C3 §8's tuning table quotes %r as a roadmap §8.2 row, and the roadmap has no such row" % r)
        c3 += 1
onto_states = [debacktick(r[0]) for r in table_in(ON["5"], "State")[1] if r]
body8 = "\n".join(CH["8"])
for st in onto_states:
    if "`%s`" % st not in body8 and st not in body8:
        bad("C3 the ontology declares the state `%s` and §8 never dispositions it" % st); c3 += 1
print("-- C3 the matrix covers the roadmap's classes, its example rows and every state")
print("  matrix: %d row(s) × %d artifact class(es) · ontology states: %d · breaches: %d"
      % (len(m_rows), len(chapter_classes), len(onto_states), c3))

# ── C4 the disposition vocabulary is closed and used ──────────────────────────────────────
d_hdr, d_rows = table_in(CH["8"], "Disposition")
vocabulary = {debacktick(r[0]) for r in d_rows if r}
used = set()
c4 = 0
for r in m_rows:
    for cell in r[1:]:
        token = debacktick(cell)
        if token not in vocabulary:
            bad("C4 a matrix cell carries %r, which §8's disposition table does not declare" % token); c4 += 1
        used.add(token)
for token in sorted(vocabulary - used):
    if token == "permit":
        continue                      # `permit` is the resolved case: no unknown row may carry it
    bad("C4 §8 declares the disposition `%s` and no matrix cell carries it" % token); c4 += 1
if "permit" in used:
    bad("C4 a matrix cell says `permit`: this matrix is consulted for the `unknown` state only, and an "
        "unknown is never simply permitted"); c4 += 1
print("-- C4 the disposition vocabulary")
print("  declared: %s · used by a cell: %s · breaches: %d"
      % (" ".join(sorted(vocabulary)), " ".join(sorted(used)), c4))

# ── C5 diagnostics ────────────────────────────────────────────────────────────────────────
d_hdr, d_rows = table_in(CH["9"], "Token")
declared = {debacktick(r[0]) for r in d_rows if r}
envelope = {debacktick(r[0]) for r in table_in(MX["10"], "Token")[1] if r}
c5 = 0
for r in d_rows:
    if len(r) < 3 or not r[2].strip() or r[2].strip() == "—":
        bad("C5 diagnostic `%s` names no required arguments" % (debacktick(r[0]) if r else "?")); c5 += 1
used_tokens = {s for s in code_spans(read(CHAPTER)) if re.fullmatch(r"(release|env)_[a-z_]+", s)}
for token in sorted(used_tokens):
    if token not in declared and token not in envelope:
        bad("C5 the chapter names `%s`, which neither its §9 nor the envelope's §10 declares" % token)
        c5 += 1
print("-- C5 the diagnostic set")
print("  declared here: %d · borrowed from the envelope: %d · breaches: %d"
      % (len(declared), len(used_tokens & envelope), c5))

# ── C6 references ─────────────────────────────────────────────────────────────────────────
c6 = 0
base = str(pathlib.Path(CHAPTER).parent)
def resolve(target):
    absolute = base.startswith("/")
    segs = []
    for s in (base + "/" + target).split("/"):
        if s in ("", "."): continue
        if s == "..":
            if segs: segs.pop()
            continue
        segs.append(s)
    out = "/".join(segs)
    return "/" + out if absolute else out
for text, target in re.findall(r"\[([^\]]*)\]\(([^)]+)\)", read(CHAPTER)):
    if target.startswith(("http://", "https://", "#")): continue
    path = resolve(target.split("#")[0])
    if not pathlib.Path(path).is_file():
        bad("C6 the chapter links to %s, which does not exist" % target); c6 += 1; continue
    m = re.search(r"§(\d+(?:\.\d+)*)", text)
    if m and m.group(1) not in headings(path):
        bad("C6 the chapter cites §%s, which %s has no heading for" % (m.group(1), target)); c6 += 1
print("-- C6 references")
print("  breaches: %d" % c6)

print("-- A1 advisory: the contract as this run read it")
for r in manifest_rows:
    if len(r) >= 4:
        print("  · %-28s %-34s %s" % (debacktick(r[0]), norm(r[1]), r[3].strip()[:40]))
for r in m_rows:
    print("  · matrix %-30s %s" % (r[0].strip()[:30], " | ".join(debacktick(c) for c in r[1:])))

total = c1 + c2 + c3 + c4 + c5 + c6
print("release-contract census: %d manifest fields / %d states / %d matrix rows / %d failure(s)"
      % (len(manifest_rows), len(chapter_states), len(m_rows), total))
sys.exit(1 if total else 0)
PY
rc=$?
exit "$rc"
