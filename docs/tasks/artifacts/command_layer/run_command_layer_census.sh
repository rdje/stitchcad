#!/usr/bin/env bash
# docs/tasks/artifacts/command_layer/run_command_layer_census.sh
# G0-CONTRACT.17 — the census behind the command layer's claim to be the roadmap's §4.4 and §7.8.
#
# WHY A TOOL AND NOT A RE-READ: the chapter's contract is a set of closed vocabularies — five command
# classes, five authority levels, four reversibility values, seventeen commands — and a closed set is exactly
# the claim that decays silently: a command added at G1 with no class, an authority level renamed in one
# table and not the other, a parity column nobody grounded. The roadmap names five commands and five
# authority levels in prose, so both lists are parsed out of ROADMAP.md here and compared, which is the same
# shape that closed the interchange layer table (`G0-CONTRACT.10`) and the release manifest (`G0-CONTRACT.12`).
#
# WHAT IT CHECKS, mechanically. Each numbered rule fails the run:
#   K1 roadmap commands  every command §4.4 names has a row in §1.1, and every row's command is
#                        identifier-shaped (a command is a type name, not a sentence).
#   K2 row closure       every §1.1 row fills all six cells, and its class, authority and reversibility are
#                        members of the vocabularies §1 and §7 declare — derived from those tables, not from
#                        a list kept here.
#   K3 authority levels  §7's levels are exactly the five roadmap §7.8 names, in both directions, and the
#                        chapter states that `approve` cannot be held by an agent.
#   K4 parity table      every column of §8 declares where it comes from, and the cell vocabulary declares
#                        `absent` — a column with no source is a column somebody fills in by hand.
#   K5 diagnostics       every `command_*` token the chapter names is declared by §9, and every §9 row names
#                        the arguments its diagnostic carries.
#   K6 references        every link resolves, and a `§n` inside a link names a heading the target carries.
# ADVISORY, printed and never a failure:
#   A1 the command table as read — per class, the commands and their authority — so a human can eyeball the
#      whole surface instead of trusting the summary line.
#
# HONEST LIMITS: it proves the specification agrees with the roadmap and with itself. No command exists in
# code yet (`G1-SLICE.6` writes the bus), so nothing here can check that an implementation honours the
# contract — that is G1's acceptance, and this census is the list its tests are written from. It reads the
# roadmap's two prose lists by shape (backticked CamelCase in §4.4; a slash-separated list in §7.8), so a
# rewrite of either is reported as "nothing parsed" and refused rather than silently passing.
#
# Usage:  bash docs/tasks/artifacts/command_layer/run_command_layer_census.sh
#         COMMAND_ROOT=<dir> bash …   a scratch copy of the tree (the probe suites use this)
#         COMMAND_BOOK=<dir> / COMMAND_CHAPTER=<path> / COMMAND_ROADMAP=<path>
# Output: per-section tables, then
#         `command-layer census: <commands> commands / <classes> classes / <levels> levels / <n> failure(s)`
#         exit 1 on any failure, exit 2 when an input is missing.
set -uo pipefail
export LC_ALL=C

ROOT="${COMMAND_ROOT:-$(git rev-parse --show-toplevel 2>/dev/null || pwd)}"
BOOK="${COMMAND_BOOK:-$ROOT/docs/book/src}"
CHAPTER="${COMMAND_CHAPTER:-$BOOK/spec/command-layer.md}"
ROADMAP="${COMMAND_ROADMAP:-$ROOT/ROADMAP.md}"

for f in "$CHAPTER" "$ROADMAP"; do
  [ -f "$f" ] || { echo "command-layer census: REFUSED — $f not found" >&2; exit 2; }
done
command -v python3 >/dev/null 2>&1 || { echo "command-layer census: REFUSED — python3 not found" >&2; exit 2; }

python3 - "$CHAPTER" "$ROADMAP" <<'PY'
import pathlib, re, sys

CHAPTER, ROADMAP = sys.argv[1:3]
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

print("=== command-layer census ===")
CH, RM = sections(CHAPTER), sections(ROADMAP)
for name, sec, need in (("the chapter", CH, ("1", "1.1", "7", "8", "9")),
                        ("ROADMAP.md", RM, ("4.4", "7.8"))):
    missing = [c for c in need if c not in sec]
    if missing:
        print("command-layer census: REFUSED — %s carries no §%s" % (name, ", §".join(missing)))
        sys.exit(2)

# ── the roadmap's own two lists, parsed rather than recalled ──────────────────────────────
rm_commands = sorted({s for s in code_spans("\n".join(RM["4.4"]))
                      if re.fullmatch(r"[A-Z][A-Za-z0-9]+", s)})
rm78 = re.sub(r"\s+", " ", "\n".join(RM["7.8"]))
m = re.search(r"authority is scoped from day one: ([a-z /]+?) are distinct", rm78)
rm_levels = sorted(x.strip() for x in m.group(1).split("/")) if m else []
if len(rm_commands) < 5 or len(rm_levels) != 5:
    bad("K1/K3 the roadmap parsed to %d command(s) and %d authority level(s) — the shape changed, so this "
        "census is reading nothing" % (len(rm_commands), len(rm_levels)))
print("-- the population, read from ROADMAP.md")
print("  §4.4 commands: %s" % " ".join(rm_commands))
print("  §7.8 levels:   %s" % " ".join(rm_levels))

# ── the chapter's vocabularies, derived from its own tables ───────────────────────────────
cls_hdr, cls_rows = table_in(CH["1"], "Class")
classes = {debacktick(r[0]) for r in cls_rows if r}
reversibility = {debacktick(r[3]).strip() for r in cls_rows if len(r) > 3}
lvl_hdr, lvl_rows = table_in(CH["7"], "Level")
levels = {debacktick(r[0]) for r in lvl_rows if r}

# ── K1 + K2: the command table ────────────────────────────────────────────────────────────
cmd_hdr, cmd_rows = table_in(CH["1.1"], "Command")
if len(cmd_hdr) < 6:
    bad("K2 §1.1's table is not `Command | Class | Authority | Reversible | Granularity | What it does`")
commands, k1, k2 = [], 0, 0
for r in cmd_rows:
    if len(r) < 6:
        bad("K2 a command row has %d cells, the table declares 6: %r" % (len(r), r)); k2 += 1; continue
    name = debacktick(r[0])
    commands.append(name)
    if not re.fullmatch(r"[A-Z][A-Za-z0-9]+", name):
        bad("K1 `%s` is not identifier-shaped — a command is a type name, not a sentence" % name); k1 += 1
    for i, (cell, vocab, label) in enumerate(((r[1], classes, "class"), (r[2], levels, "authority"),
                                              (r[3], reversibility, "reversibility")), start=1):
        value = debacktick(cell).strip()
        if value not in vocab:
            bad("K2 `%s` declares %s %r, which §%s does not" % (name, label, value,
                                                               "1" if label != "authority" else "7"))
            k2 += 1
    if not r[4].strip() or not r[5].strip():
        bad("K2 `%s` leaves its granularity or its description empty" % name); k2 += 1
for name in rm_commands:
    if name not in commands:
        bad("K1 roadmap §4.4 names `%s` and §1.1 carries no row for it" % name); k1 += 1
print("-- K1 the roadmap's commands / K2 the row vocabularies")
print("  commands: %d · classes: %d · reversibility values: %d · breaches: %d"
      % (len(commands), len(classes), len(reversibility), k1 + k2))

# ── K3: the authority levels ─────────────────────────────────────────────────────────────
k3 = 0
for level in rm_levels:
    if level not in levels:
        bad("K3 roadmap §7.8 names the authority level `%s` and §7 does not declare it" % level); k3 += 1
for level in sorted(levels):
    if level not in rm_levels:
        bad("K3 §7 declares the level `%s`, which roadmap §7.8 does not name — a sixth level is a "
            "governance change, not a chapter edit" % level); k3 += 1
body7 = "\n".join(CH["7"])
if "human-only" not in body7:
    bad("K3 §7 does not state that `approve` is human-only, which is the one property roadmap §7.8 calls "
        "a capability no graph mutation can manufacture"); k3 += 1
print("-- K3 the authority levels")
print("  declared: %s · breaches: %d" % (" ".join(sorted(levels)), k3))

# ── K4: the parity table's columns are grounded ──────────────────────────────────────────
col_hdr, col_rows = table_in(CH["8"], "Column")
cell_hdr, cell_rows = table_in(CH["8"], "Cell value")
k4 = 0
for r in col_rows:
    if len(r) < 2 or not r[1].strip() or r[1].strip() == "—":
        bad("K4 the parity column %r declares no source — a column nobody derives is a column somebody "
            "fills in by hand" % (r[0] if r else "?")); k4 += 1
if "absent" not in {debacktick(r[0]) for r in cell_rows if r}:
    bad("K4 §8's cell vocabulary does not declare `absent`, so a gap has no declared spelling"); k4 += 1
print("-- K4 the parity table")
print("  columns: %d · cell values: %d · breaches: %d" % (len(col_rows), len(cell_rows), k4))

# ── K5: diagnostics ──────────────────────────────────────────────────────────────────────
d_hdr, d_rows = table_in(CH["9"], "Token")
declared = {debacktick(r[0]) for r in d_rows if r}
k5 = 0
for r in d_rows:
    if len(r) < 3 or not r[2].strip() or r[2].strip() == "—":
        bad("K5 diagnostic `%s` names no required arguments" % (debacktick(r[0]) if r else "?")); k5 += 1
fields = {debacktick(r[0]) for r in table_in(CH["2"], "Field")[1] if r}
for token in sorted({s for s in code_spans(read(CHAPTER)) if re.fullmatch(r"command_[a-z_]+", s)}):
    if token in fields:
        continue                      # a field of the command shape, not a diagnostic
    if token not in declared:
        bad("K5 the chapter names `%s`, which §9 does not declare" % token); k5 += 1
print("-- K5 the diagnostic set")
print("  declared: %d · command-shape fields excluded: %d · breaches: %d"
      % (len(declared), len(fields & {s for s in code_spans(read(CHAPTER))}), k5))

# ── K6: references ───────────────────────────────────────────────────────────────────────
k6 = 0
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
        bad("K6 the chapter links to %s, which does not exist" % target); k6 += 1; continue
    mm = re.search(r"§(\d+(?:\.\d+)*)", text)
    if mm and mm.group(1) not in headings(path):
        bad("K6 the chapter cites §%s, which %s has no heading for" % (mm.group(1), target)); k6 += 1
print("-- K6 references")
print("  breaches: %d" % k6)

print("-- A1 advisory: the command surface as this run read it")
for cls in sorted(classes):
    rows = [r for r in cmd_rows if len(r) >= 3 and debacktick(r[1]) == cls]
    print("  · %-11s %2d command(s): %s" % (cls, len(rows),
                                            " ".join(debacktick(r[0]) for r in rows)[:100]))

total = k1 + k2 + k3 + k4 + k5 + k6
print("command-layer census: %d commands / %d classes / %d levels / %d failure(s)"
      % (len(commands), len(classes), len(levels), total))
sys.exit(1 if total else 0)
PY
rc=$?
exit "$rc"
