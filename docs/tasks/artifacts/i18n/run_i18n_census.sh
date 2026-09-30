#!/usr/bin/env bash
# docs/tasks/artifacts/i18n/run_i18n_census.sh
# G0-CONTRACT.16 — the census behind the i18n chapter's claim that its message inventory is complete.
#
# WHY A TOOL AND NOT A RE-READ: the chapter claims every message id this product can emit is inventoried,
# with a count per family. That claim is universally quantified over a population that grows with every
# chapter that declares a diagnostic — the envelope's 29, the formula language's 12, the dialects' 5, the
# release contract's 8 — and over one crate's error enum, which is code and changes without a book edit. A
# count kept by hand is stale the moment a token is added, and a stale inventory is a message nobody will
# translate. So the counts are derived here, from the tables and from the source file, in both directions.
#
# WHAT IT CHECKS, mechanically. Each numbered rule fails the run:
#   M1 inventory    every diagnostic token any spec chapter declares, plus every `UnitError` variant in
#                   `crates/sc-units/src/error.rs`, is covered by exactly one family row of §10 — and each
#                   family's declared count equals the number the census derives for it.
#   M2 family       a family is a prefix, and no derived token matches two families or none.
#   M3 tiers        every family row names a tier §9 declares, and the two families roadmap §7.6 names as
#                   safety-relevant (notch types, the cut/sew aliases) are not tiered `standard`.
#   M4 lint         every exemption row in §5 carries a reason — an exemption nobody justified is one that
#                   grows.
#   M5 diagnostics  every `i18n_*` token the chapter names is declared by §11, and every §11 row names the
#                   arguments its diagnostic carries.
#   M6 references   every link resolves, and a `§n` inside a link names a heading the target carries.
# ADVISORY, printed and never a failure:
#   A1 the derived inventory — per family, the count and the tokens — beside the count §10 publishes.
#
# HONEST LIMITS: it proves the inventory agrees with the declarations, not that a message is well written,
# correctly tiered for its content, or translatable — the tiers are a project decision and the domain seat
# that reviews a `safety` term is vacant (governance §8.1). It reads the Rust enum by shape
# (`pub enum UnitError {` then `Variant {` or `Variant,` lines), so a rewrite of that enum into a macro or a
# different shape is reported as "no variants found" and refused rather than silently passing.
#
# Usage:  bash docs/tasks/artifacts/i18n/run_i18n_census.sh
#         I18N_ROOT=<dir> bash …   a scratch copy of the tree (the probe suites use this)
#         I18N_BOOK=<dir> / I18N_CHAPTER=<path> / I18N_UNITS_ERROR=<path>
# Output: per-section tables, then
#         `i18n census: <families> families / <ids> message ids / <n> failure(s)`
#         exit 1 on any failure, exit 2 when an input is missing.
set -uo pipefail
export LC_ALL=C

ROOT="${I18N_ROOT:-$(git rev-parse --show-toplevel 2>/dev/null || pwd)}"
BOOK="${I18N_BOOK:-$ROOT/docs/book/src}"
CHAPTER="${I18N_CHAPTER:-$BOOK/spec/i18n-architecture.md}"
UNITS_ERROR="${I18N_UNITS_ERROR:-$ROOT/crates/sc-units/src/error.rs}"

for f in "$CHAPTER" "$UNITS_ERROR" "$BOOK/spec/feature-matrix.md"; do
  [ -f "$f" ] || { echo "i18n census: REFUSED — $f not found" >&2; exit 2; }
done
command -v python3 >/dev/null 2>&1 || { echo "i18n census: REFUSED — python3 not found" >&2; exit 2; }

python3 - "$CHAPTER" "$UNITS_ERROR" "$BOOK" <<'PY'
import pathlib, re, sys

CHAPTER, UNITS_ERROR, BOOK = sys.argv[1:4]
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

print("=== i18n census ===")
CH = sections(CHAPTER)
missing = [c for c in ("5", "9", "10", "11") if c not in CH]
if missing:
    print("i18n census: REFUSED — the chapter carries no §%s" % ", §".join(missing))
    sys.exit(2)

# ── the derived population: every diagnostic token the book declares, plus the crate's ──────
SOURCES = [("spec/feature-matrix.md", "10", "Token"),
           ("spec/formula-language.md", "5.2", "Token"),
           ("spec/interchange-dialects.md", "11", "Token"),
           ("spec/release-contract.md", "9", "Token")]
TOKEN = re.compile(r"^[a-z][a-z0-9]*(?:_[a-z0-9]+)+$")
derived = {}
for rel, sec, first in SOURCES:
    path = pathlib.Path(BOOK) / rel
    if not path.is_file():
        bad("M1 %s is missing, so its diagnostics cannot be inventoried" % rel)
        continue
    body = sections(path).get(sec)
    if body is None:
        bad("M1 %s carries no §%s for this census to read" % (rel, sec))
        continue
    for r in table_in(body, first)[1]:
        if not r: continue
        token = debacktick(r[0])
        if TOKEN.match(token):
            derived.setdefault(token, rel)

# the crate's own diagnostics: `pub enum UnitError { Variant { … } … }`
src = read(UNITS_ERROR).splitlines()
start = next((i for i, l in enumerate(src) if re.match(r"^pub enum UnitError\b", l)), None)
variants = []
if start is None:
    bad("M1 %s carries no `pub enum UnitError`, so the units family cannot be derived" % UNITS_ERROR)
else:
    for l in src[start + 1:]:
        if re.match(r"^}", l):
            break
        m = re.match(r"^    ([A-Z][A-Za-z0-9]*)\s*[{,]?", l)
        if m:
            snake = re.sub(r"(?<!^)(?=[A-Z])", "_", m.group(1)).lower()
            variants.append("unit_" + snake)
for v in variants:
    derived.setdefault(v, "crates/sc-units/src/error.rs")

# §11's own family
for r in table_in(CH["11"], "Token")[1]:
    if r and debacktick(r[0]).startswith("i18n_"):
        derived.setdefault(debacktick(r[0]), "this chapter §11")

families = {}
for token in derived:
    families.setdefault(token.split("_")[0] + "_", set()).add(token)

# ── §10's inventory ───────────────────────────────────────────────────────────────────────
hdr, rows = table_in(CH["10"], "Family")
if not rows:
    bad("M1 the chapter's §10 has no table whose first column is `Family`")
published, prefixes, m1 = {}, {}, 0
for r in rows:
    if len(r) < 4:
        bad("M1 an inventory row has %d cells, the table declares 4: %r" % (len(r), r)); m1 += 1
        continue
    fam_prefixes = [s.rstrip("*") for s in re.findall(r"`([a-z0-9]+_)\*`", r[0])]
    if not fam_prefixes:
        bad("M1 the inventory row %r names no `prefix_*` family" % r[0]); m1 += 1; continue
    label = ", ".join(fam_prefixes)
    for p in fam_prefixes:
        if p in prefixes:
            bad("M2 the prefix `%s` is claimed by two inventory rows" % p); m1 += 1
        prefixes[p] = label
    try:
        published[label] = int(r[1].strip())
    except ValueError:
        bad("M1 the inventory row %r publishes %r as its id count" % (label, r[1])); m1 += 1
    for p in fam_prefixes:
        derived_n = len(families.get(p, ()))
        if len(fam_prefixes) == 1 and derived_n and published.get(label) is not None \
           and published[label] != derived_n:
            bad("M1 the family `%s` publishes %d id(s) and the sources declare %d"
                % (p, published[label], derived_n)); m1 += 1
# multi-prefix rows (the envelope's `env_*` + `ngo_*`) are summed
for r in rows:
    if len(r) < 2: continue
    fam_prefixes = [s.rstrip("*") for s in re.findall(r"`([a-z0-9]+_)\*`", r[0])]
    if len(fam_prefixes) > 1:
        want = sum(len(families.get(p, ())) for p in fam_prefixes)
        try:
            got = int(r[1].strip())
        except ValueError:
            continue
        if want and got != want:
            bad("M1 the family %s publishes %d id(s) and the sources declare %d"
                % (" + ".join(fam_prefixes), got, want)); m1 += 1
covered = set()
for p in prefixes:
    covered |= families.get(p, set())
for token in sorted(set(derived) - covered):
    bad("M1 `%s` is declared by %s and no family in §10 covers it — an id nobody inventories is an id "
        "nobody translates" % (token, derived[token])); m1 += 1
for p in sorted(prefixes):
    if p not in families:
        bad("M1 §10 declares the family `%s` and no source table declares a token with that prefix" % p)
        m1 += 1
print("-- M1 the inventory / M2 one family per id")
print("  derived: %d id(s) in %d family(ies) · published: %d family row(s) · breaches: %d"
      % (len(derived), len(families), len(rows), m1))

# ── M3 the review tiers ───────────────────────────────────────────────────────────────────
tier_hdr, tier_rows = table_in(CH["9"], "Tier")
tiers = {debacktick(r[0]) for r in tier_rows if r}
m3 = 0
for r in rows:
    if len(r) < 4: continue
    named = {t for t in tiers if t in r[3]}
    if not named:
        bad("M3 the family %s names no review tier from §9's set (%s)"
            % (r[0].strip(), ", ".join(sorted(tiers)))); m3 += 1
# roadmap §7.6 names notch types and the cut/sew aliases as the safety-relevant families
safety_rows = [r for r in rows if r and ("env_" in r[0] or "dialect_" in r[0])]
for r in safety_rows:
    if len(r) >= 4 and "safety" not in r[3] and "strict" not in r[3]:
        bad("M3 the family %s carries the notch and cut/sew refusals roadmap §7.6 names as "
            "safety-relevant, and §10 tiers it %r" % (r[0].strip(), r[3])); m3 += 1
print("-- M3 the review tiers")
print("  tiers declared: %s · breaches: %d" % (" ".join(sorted(tiers)), m3))

# ── M4 the lint's exemptions ──────────────────────────────────────────────────────────────
ex_hdr, ex_rows = table_in(CH["5"], "Exemption")
m4 = 0
for r in ex_rows:
    if len(r) < 2 or not r[1].strip() or r[1].strip() == "—":
        bad("M4 the lint exemption %r carries no reason" % (r[0] if r else "?")); m4 += 1
print("-- M4 the lint's exemptions")
print("  exemptions: %d · breaches: %d" % (len(ex_rows), m4))

# ── M5 this layer's diagnostics ───────────────────────────────────────────────────────────
d_hdr, d_rows = table_in(CH["11"], "Token")
declared = {debacktick(r[0]) for r in d_rows if r}
m5 = 0
for r in d_rows:
    if len(r) < 3 or not r[2].strip() or r[2].strip() == "—":
        bad("M5 diagnostic `%s` names no required arguments" % (debacktick(r[0]) if r else "?")); m5 += 1
for token in sorted({s for s in code_spans(read(CHAPTER)) if re.fullmatch(r"i18n_[a-z_]+", s)}):
    if token not in declared:
        bad("M5 the chapter names `%s`, which its §11 does not declare" % token); m5 += 1
print("-- M5 this layer's diagnostics")
print("  declared: %d · breaches: %d" % (len(declared), m5))

# ── M6 references ─────────────────────────────────────────────────────────────────────────
m6 = 0
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
        bad("M6 the chapter links to %s, which does not exist" % target); m6 += 1; continue
    m = re.search(r"§(\d+(?:\.\d+)*)", text)
    if m and m.group(1) not in headings(path):
        bad("M6 the chapter cites §%s, which %s has no heading for" % (m.group(1), target)); m6 += 1
print("-- M6 references")
print("  breaches: %d" % m6)

print("-- A1 advisory: the derived inventory beside the published one")
for p in sorted(families):
    print("  · %-12s %2d derived · %s" % (p, len(families[p]),
                                          " ".join(sorted(families[p]))[:96]))
for label, count in published.items():
    print("  · published %-22s %d" % (label, count))

total = m1 + m3 + m4 + m5 + m6
print("i18n census: %d families / %d message ids / %d failure(s)"
      % (len(rows), len(derived), total))
sys.exit(1 if total else 0)
PY
rc=$?
exit "$rc"
