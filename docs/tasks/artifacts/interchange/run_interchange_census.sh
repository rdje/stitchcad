#!/usr/bin/env bash
# docs/tasks/artifacts/interchange/run_interchange_census.sh
# G0-CONTRACT.10 — the census behind the claims the interchange-dialects chapter makes about itself.
#
# WHY A TOOL AND NOT A RE-READ: the chapter's central claim is a closure — every layer the roadmap's
# ADR-0004 names is dispositioned, every axis has a party that resolves it, every target is a tuple over
# declared values, and every refusal names a declared diagnostic. A closure over a population that grows
# (a receiver added at G6, an entity a partner asks for, a layer a new convention introduces) is exactly
# the claim that stays true in prose while going false in fact, which this repository measured as D24 and
# fixed by deriving instead of asserting. So the layer table is compared against ROADMAP.md itself: amend
# the roadmap's convention and this census reddens until the chapter dispositions the change.
#
# WHAT IT CHECKS, mechanically. Each numbered rule fails the run:
#   D1 roadmap closure  every layer number and range in the roadmap's ADR-0004 "Layer table (corrected)"
#                       bullet has a row in the chapter's §3, and every AAMA name the roadmap lists
#                       appears in that table's AAMA column.
#   D2 one layer, one meaning  no layer is claimed twice; every row states what it carries and which
#                       StitchCAD object writes it; the blank layer says so; and no AAMA name appears in
#                       §3 that the roadmap does not list (an invented eighth name is a refusal).
#   D3 registry closure every axis the chapter declares is either carried by a registered target or
#                       marked not applicable to it; no target row has an empty cell; and every target
#                       token the chapter names anywhere is one §2 registers.
#   D4 entity policy    every entity row's status is from the closed vocabulary {both releases, never};
#                       a refused entity carries a reason; the written set is release-independent, which
#                       is the chapter's own claim that one writer serves both targets; and the policy's
#                       floor and ceiling hold (POLYLINE and VERTEX written, SPLINE and LWPOLYLINE not).
#   D5 diagnostics      every `dialect_*` / `env_*` token the chapter names is declared by its §11 or by
#                       the envelope's §10, and every §11 row names the arguments the diagnostic carries.
#   D6 references       every link in the chapter resolves, and a `§n` inside a link names a heading the
#                       target really carries.
# ADVISORY, printed and never a failure:
#   A1 the layer table and the registry as this run read them, so a human can eyeball the whole
#      disposition instead of trusting the summary line.
#
# HONEST LIMITS: it proves the chapter agrees with the roadmap, with the envelope and with itself. It
# cannot tell whether a layer mapping is what a receiver actually expects — that is G6's oracle and no
# census can substitute for a machine reading the file back. It reads the roadmap's bullet by shape (a
# number or range followed by a description), so a rewrite of that bullet into prose would be reported as
# "no layers found" and refused rather than silently passing.
#
# Usage:  bash docs/tasks/artifacts/interchange/run_interchange_census.sh
#         INTERCHANGE_ROOT=<dir> bash …          a scratch copy of the tree (the probe suites use this)
#         INTERCHANGE_CHAPTER=<path> bash …      another copy of the chapter
# Output: per-section tables, then
#         `interchange census: <layers> layers / <targets> targets / <entities> entities / <n> failure(s)`
#         exit 1 on any failure, exit 2 when an input is missing.
set -uo pipefail
export LC_ALL=C

ROOT="${INTERCHANGE_ROOT:-$(git rev-parse --show-toplevel 2>/dev/null || pwd)}"
BOOK="${INTERCHANGE_BOOK:-$ROOT/docs/book/src}"
CHAPTER="${INTERCHANGE_CHAPTER:-$BOOK/spec/interchange-dialects.md}"
MATRIX="${INTERCHANGE_MATRIX:-$BOOK/spec/feature-matrix.md}"
ROADMAP="${INTERCHANGE_ROADMAP:-$ROOT/ROADMAP.md}"

for f in "$CHAPTER" "$MATRIX" "$ROADMAP"; do
  [ -f "$f" ] || { echo "interchange census: REFUSED — $f not found" >&2; exit 2; }
done
command -v python3 >/dev/null 2>&1 || { echo "interchange census: REFUSED — python3 not found" >&2; exit 2; }

python3 - "$CHAPTER" "$MATRIX" "$ROADMAP" <<'PY'
import pathlib, re, sys

CHAPTER, MATRIX, ROADMAP = sys.argv[1:4]
fails = 0
def bad(msg):
    global fails
    fails += 1
    print("  x %s" % msg)

def read(p):
    return pathlib.Path(p).read_text(encoding="utf-8")

def debacktick(s):
    return s.replace("`", "").strip()

def code_spans(text):
    out, fenced = [], False
    for line in text.splitlines():
        if line.lstrip().startswith("```"):
            fenced = not fenced
            continue
        if not fenced:
            out.extend(re.findall(r"`([^`]+)`", line))
    return out

# ── markdown plumbing ─────────────────────────────────────────────────────────────────────
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

print("=== interchange census ===")
DIA, MAT = sections(CHAPTER), sections(MATRIX)
for name, sec, need in (("the dialects chapter", DIA, ("1", "2", "3", "6", "11")),
                        ("the envelope", MAT, ("10",))):
    missing = [c for c in need if c not in sec]
    if missing:
        print("interchange census: REFUSED — %s carries no §%s" % (name, ", §".join(missing)))
        sys.exit(2)

# ── the roadmap's own layer list, parsed rather than recalled ─────────────────────────────
rm_lines = read(ROADMAP).splitlines()
start = next((i for i, l in enumerate(rm_lines) if "Layer table (corrected)" in l), None)
if start is None:
    print("interchange census: REFUSED — ROADMAP.md carries no 'Layer table (corrected)' bullet")
    sys.exit(2)
# The bullet WRAPS. Reading only its first line found 3 of 17 layers and no AAMA names at all, which
# printed as 21 chapter breaches and was the instrument measuring one line of a paragraph.
end = start + 1
while end < len(rm_lines) and rm_lines[end].strip() and not rm_lines[end].lstrip().startswith("- "):
    end += 1
bullet = " ".join(l.strip() for l in rm_lines[start:end])
roadmap_layers = [m.group(0).replace("-", "–") for m in
                  re.finditer(r"(?<![\w.])\d{1,2}(?:[–-]\d{1,2})?(?=[ ,])", bullet)]
m = re.search(r"\(([A-Z]{3,}(?:/[A-Z]{3,})+)\)", bullet)
roadmap_names = m.group(1).split("/") if m else []
if len(roadmap_layers) < 10 or not roadmap_names:
    bad("D1 the roadmap's bullet parsed to %d layer(s) and %d AAMA name(s) — the shape changed, so this "
        "census is reading nothing" % (len(roadmap_layers), len(roadmap_names)))
print("-- the population, read from ROADMAP.md")
print("  layers: %s" % " ".join(roadmap_layers))
print("  AAMA names: %s" % " ".join(roadmap_names))

# ── §3 the layer table ────────────────────────────────────────────────────────────────────
hdr, layer_rows = table_in(DIA["3"], "ASTM layer")
if not layer_rows:
    bad("D1 the chapter's §3 has no table whose first column is `ASTM layer`")
chapter_layers, chapter_names, seen = [], {}, {}
d1 = d2 = 0
for r in layer_rows:
    if len(r) < 4:
        bad("D2 §3 row has %d cells, the table declares 4: %r" % (len(r), r)); d2 += 1; continue
    layer, carries, obj, aama = debacktick(r[0]), r[1].strip(), r[2].strip(), debacktick(r[3])
    norm = layer.replace("-", "–")
    chapter_layers.append(norm)
    if norm in seen:
        bad("D2 layer %s is claimed twice in §3 — one layer, one meaning" % layer); d2 += 1
    seen[norm] = obj
    if not carries or carries == "—":
        bad("D2 layer %s states nothing it carries" % layer); d2 += 1
    if not obj or obj == "—":
        bad("D2 layer %s names no StitchCAD object (write `nothing` and why, rather than a dash)" % layer)
        d2 += 1
    for n in re.findall(r"\b[A-Z]{3,}\b", aama):
        chapter_names.setdefault(n, []).append(layer)
for layer in roadmap_layers:
    if layer.replace("-", "–") not in chapter_layers:
        bad("D1 the roadmap names layer %s and §3 has no row for it" % layer); d1 += 1
for extra in chapter_layers:
    if extra not in [l.replace("-", "–") for l in roadmap_layers]:
        bad("D1 §3 carries layer %s, which the roadmap's ADR-0004 does not name — a new convention needs "
            "a recorded source, not a row" % extra); d1 += 1
for n in chapter_names:
    if n not in roadmap_names:
        bad("D2 §3 uses the AAMA name %s, which the roadmap does not list (an invented name is a new "
            "convention)" % n); d2 += 1
for n in roadmap_names:
    if n not in chapter_names:
        bad("D1 the roadmap lists the AAMA name %s and §3 never uses it" % n); d1 += 1
print("-- D1 roadmap closure / D2 one layer, one meaning")
print("  layers in §3: %d · AAMA names used: %d · breaches: %d" % (len(chapter_layers),
                                                                   len(chapter_names), d1 + d2))

# ── §1 + §2 the axes and the registry ─────────────────────────────────────────────────────
axis_hdr, axis_rows = table_in(DIA["1"], "Token")
axes = [debacktick(r[0]) for r in axis_rows if r]
tgt_hdr, tgt_rows = table_in(DIA["2"], "Target")
targets = {}
d3 = 0
for r in tgt_rows:
    if len(r) < len(tgt_hdr):
        bad("D3 §2 row has %d cells, the table declares %d: %r" % (len(r), len(tgt_hdr), r)); d3 += 1
        continue
    token = debacktick(r[0])
    targets[token] = dict(zip([debacktick(h) for h in tgt_hdr], r))
    for i, cell in enumerate(r):
        if not cell.strip():
            bad("D3 target `%s` leaves the %s column empty — an axis nobody resolved is a default an "
                "implementation chose" % (token, tgt_hdr[i])); d3 += 1
columns = [debacktick(h) for h in tgt_hdr[1:]]
# D3a: an axis the target resolves IS a registry column, and a registry column IS a declared axis —
# both directions, read from the chapter's own last column rather than from a list hardcoded here.
registry_columns = {"Target", "Proven at"}
carried = {}
for r in axis_rows:
    if len(r) < 5: continue
    token, column = debacktick(r[0]), r[4].strip()
    if column and column != "—":
        carried[token] = column
        if column not in columns:
            bad("D3 axis `%s` says the registry carries it as %r, and §2 has no such column" % (token, column))
            d3 += 1
for column in columns:
    if column in registry_columns: continue
    if column not in carried.values():
        bad("D3 §2 carries a %r column that no axis in §1 declares — an axis added silently is an "
            "axis nobody resolved" % column)
        d3 += 1
for span in code_spans(read(CHAPTER)):
    if re.fullmatch(r"(dxf[-_][a-z0-9-]+|plt|pdf-[a-z0-9]+)", span) and span not in targets:
        bad("D3 the chapter names the target `%s`, which §2 does not register — a tuple nobody validated "
            "is not a target" % span); d3 += 1
print("-- D3 the registry is closed")
print("  axes: %d (%d carried by a registry column) · targets: %d (%s) · breaches: %d"
      % (len(axes), len(carried), len(targets), ", ".join(sorted(targets)), d3))

# ── §6 the entity policy ──────────────────────────────────────────────────────────────────
ent_hdr, ent_rows = table_in(DIA["6"], "Entity")
written, refused, d4 = set(), set(), 0
STATUS = ("both releases", "never")
for r in ent_rows:
    if len(r) < 3:
        bad("D4 §6 row has %d cells, the table declares 3: %r" % (len(r), r)); d4 += 1; continue
    entity, status, why = debacktick(r[0]), r[1].strip().lower(), r[2].strip()
    if status not in STATUS:
        bad("D4 entity `%s` has status %r, which is not one of %s — a per-release entity set is two "
            "writers and two ways to disagree" % (entity, r[1], " / ".join(STATUS))); d4 += 1
    if status == "never":
        refused.add(entity)
        if not why or why == "—":
            bad("D4 entity `%s` is refused with no reason" % entity); d4 += 1
    else:
        written.add(entity)
for need in ("POLYLINE", "VERTEX"):
    if need not in written:
        bad("D4 `%s` is not in the written set, so the polyline-only policy has no floor" % need); d4 += 1
for forbid in ("SPLINE", "LWPOLYLINE"):
    if forbid not in refused:
        bad("D4 `%s` is not refused, against the roadmap's claim that legacy importers accept only "
            "POLYLINE" % forbid); d4 += 1
if "ARC" in written or "CIRCLE" in written:
    bad("D4 a standalone curve entity is written while §7 says an arc travels as a bulge — two families "
        "for one boundary"); d4 += 1
print("-- D4 the entity policy")
print("  written: %s" % " ".join(sorted(written)))
print("  refused: %s" % " ".join(sorted(refused)))
print("  breaches: %d" % d4)

# ── §11 diagnostics ───────────────────────────────────────────────────────────────────────
diag_hdr, diag_rows = table_in(DIA["11"], "Token")
declared = {debacktick(r[0]) for r in diag_rows if r}
envelope = {debacktick(r[0]) for r in table_in(MAT["10"], "Token")[1] if r}
d5 = 0
for r in diag_rows:
    if len(r) < 3 or not r[2].strip() or r[2].strip() == "—":
        bad("D5 diagnostic `%s` names no required arguments — a structured diagnostic with no arguments "
            "is prose with a code" % (debacktick(r[0]) if r else "?")); d5 += 1
used = {s for s in code_spans(read(CHAPTER)) if re.fullmatch(r"(dialect|env)_[a-z_]+", s)}
for token in sorted(used):
    if token not in declared and token not in envelope:
        bad("D5 the chapter names `%s`, which neither its §11 nor the envelope's §10 declares" % token)
        d5 += 1
for token in sorted(declared - used):
    print("  · declared and not used in this chapter: `%s`" % token)
print("-- D5 the diagnostic set")
print("  declared here: %d · borrowed from the envelope: %d · used: %d · breaches: %d"
      % (len(declared), len(declared & envelope) + len(used & envelope), len(used), d5))

# ── D6 references ─────────────────────────────────────────────────────────────────────────
d6 = 0
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
        bad("D6 the chapter links to %s, which does not exist" % target); d6 += 1; continue
    m = re.search(r"§(\d+(?:\.\d+)*)", text)
    if m and m.group(1) not in headings(path):
        bad("D6 the chapter cites §%s, which %s has no heading for" % (m.group(1), target)); d6 += 1
print("-- D6 references")
print("  breaches: %d" % d6)

print("-- A1 advisory: the layer table as this run read it")
for r in layer_rows:
    if len(r) >= 4:
        print("  · %-8s %-6s %s" % (debacktick(r[0]), debacktick(r[3]), r[2].strip()[:64]))
print("-- A1 advisory: the registry as this run read it")
for token, row in sorted(targets.items()):
    print("  · %-20s %s" % (token, " · ".join("%s=%s" % (k, debacktick(v))
                                              for k, v in row.items() if k != "Target")[:110]))

total = d1 + d2 + d3 + d4 + d5 + d6
print("interchange census: %d layers / %d targets / %d entities / %d failure(s)"
      % (len(chapter_layers), len(targets), len(ent_rows), total))
sys.exit(1 if total else 0)
PY
rc=$?
exit "$rc"
