#!/usr/bin/env bash
# docs/tasks/artifacts/reference_fixture/run_fixture_derivation.sh
# G0-CONTRACT.13d — the instrument behind every number the reference fixture publishes.
#
# WHY A TOOL AND NOT A RE-READ: the fixture is the subject of every G2 golden file, mutation test,
# offset-pathology case and agent evaluation, and it has already been wrong twice in ways a reader
# cannot see: D33 put the waist side point 7.0 cm off and the chapter's own balance still closed
# exactly, and D27 left §4 and §6 describing two different garments for eleven commits. Both were
# found by hand, after the fact, and the arithmetic that found them was typed into a scratch `python3
# -c` that nobody could re-run (CLAIM_VERIFICATION.md leg 3: a measurement whose instrument lived in
# scratch is a "trust me" with extra steps — the breach this repository already committed once as D20).
# So the re-derivation is a tracked producer, and `make probes` keeps it honest.
#
# WHAT IT CHECKS, mechanically. Each numbered rule fails the run:
#   F1 derived rows   every §4 row's formula, EVALUATED AS WRITTEN over the tables above it, equals the
#                     published Result at the precision the chapter publishes (a formula may refer only
#                     to a §2/§3/§7 constant or to a row declared ABOVE it — the evaluation order §4
#                     promises and ADR-0003 will formalize). An identifier nothing declares is a refusal
#                     that names it, never a silent 0.
#   F2 closure rows   a row whose formula is `A = B` must have both sides equal, and both published
#                     numbers must match what the sides compute to. These are the fixture's oracles
#                     (decision_fixture-oracles-derive-the-finished-dimension).
#   P1 piece account  every piece §6 lists is accounted for in §8's attachment account, either by span
#                     ids that §8's span table really declares or by a non-sewn method from the closed
#                     list; and every piece a span names is a piece §6 lists. This is D27's rule: the
#                     defect was a piece with no span and no reason.
#   P2 piece count    the package-completeness count §12 publishes equals the number of piece rows §6
#                     lists. D27 shipped a count that belonged to the reading the chapter rejected.
#   B1 band shape     §6's waistband piece list and §4's band-width row describe ONE construction: a
#                     single folded band means cut width = 2 × wb_width + sa_waist + sa_wb_bottom, a
#                     faced two-piece band means each piece = wb_width + sa_waist + sa_wb_bottom. The
#                     two formulas differ by wb_width, which is exactly how the defect hid.
# ADVISORY, printed and never a failure:
#   A1 computed table  every derived row with its computed value, so a human can eyeball the whole
#                      derivation instead of trusting the summary line.
#
# HONEST LIMITS: it proves ARITHMETIC and CROSS-REFERENCE agreement inside one chapter. It cannot tell
# whether a formula is the right garment engineering — `2 × wb_width + sa_waist + sa_wb_bottom` computes
# faithfully whatever the construction, and only the domain reviewer G0-CONTRACT.14 names can say the
# construction is right for this skirt. Nor does it read §5's prose recipe: a drafting step that
# contradicts §4 in words and not in a table is outside every rule here (D33 lived in §5 prose, and is
# caught now only because §4 gained `waist_closure` as a row). Values are parsed as decimal centimetres;
# a row published in another unit is a parse failure, not a conversion.
#
# DEPENDENCY: python3 (>= 3.8) for the expression parser — already used by `scripts/check_table_arity.sh`
# and `docs/tasks/artifacts/waiver_routing/run_waiver_routing_probes.sh`, so this adds no new toolchain
# requirement. Its absence is a refusal (exit 2), never a green run.
#
# Usage:  bash docs/tasks/artifacts/reference_fixture/run_fixture_derivation.sh
#         FIXTURE_ROOT=<dir> bash docs/tasks/artifacts/reference_fixture/run_fixture_derivation.sh
#         FIXTURE_CHAPTER=<path-to-reference-skirt.md> bash ...   (the probe suites use this)
# Output: per-section tables, then
#         `fixture derivation: <rows> derived rows / <checks> closure checks / <pieces> pieces / <n> mismatch(es)`
#         exit 1 on any mismatch, exit 2 when an input or the interpreter is missing.
set -uo pipefail
export LC_ALL=C

ROOT="${FIXTURE_ROOT:-$(git rev-parse --show-toplevel 2>/dev/null || pwd)}"
CHAPTER="${FIXTURE_CHAPTER:-$ROOT/docs/book/src/spec/reference-skirt.md}"
[ -f "$CHAPTER" ] || { echo "fixture derivation: REFUSED — $CHAPTER not found" >&2; exit 2; }
command -v python3 >/dev/null 2>&1 || { echo "fixture derivation: REFUSED — python3 not found" >&2; exit 2; }

python3 - "$CHAPTER" <<'PY'
import re, sys, unicodedata
from decimal import Decimal, ROUND_HALF_UP, InvalidOperation

path = sys.argv[1]
text = open(path, encoding="utf-8").read()
lines = text.splitlines()

fails = 0
def bad(msg):
    global fails
    fails += 1
    print("  x %s" % msg, file=sys.stderr)

# ── markdown plumbing ────────────────────────────────────────────────────────────────────────
def sections():
    """(section number, [lines]) for every `## n. Title` heading, in order."""
    out, cur, num = [], [], None
    for ln in lines:
        m = re.match(r"^## (\d+)\. ", ln)
        if m:
            if num is not None:
                out.append((num, cur))
            num, cur = m.group(1), []
        elif num is not None:
            cur.append(ln)
    if num is not None:
        out.append((num, cur))
    return out

SEC = {n: b for n, b in sections()}

def split_row(ln):
    """Cells of a GFM table row, honouring code spans and escaped pipes (as check_table_arity.sh does)."""
    out, buf, in_code = [], [], False
    i = 0
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
    if out and out[0] == "":
        out = out[1:]
    if out and out[-1] == "":
        out = out[:-1]
    return out

def tables(body):
    """Every GFM table in `body` as (header_cells, data_rows), in order."""
    out, header, rows = [], None, []
    for ln in body + [""]:
        if ln.lstrip().startswith("|"):
            cells = split_row(ln.strip())
            if cells and all(re.fullmatch(r":?-{2,}:?", c.strip()) for c in cells if c.strip()):
                continue                       # the separator row
            if header is None:
                header = cells
            else:
                rows.append(cells)
        elif header is not None:
            out.append((header, rows))
            header, rows = None, []
    return out

def table_rows(body, header_first_cell):
    """The first table in `body` whose header's first cell is `header_first_cell`.
    A section may carry several tables (§2 measurements and ease, §8 spans and the account), so the
    header is matched rather than assumed — the mistake probe arm R1 taught the glossary census,
    which examined no cell at all and still printed a green verdict."""
    for header, rows in tables(body):
        if header and debacktick(header[0]) == header_first_cell:
            return header, rows
    return [], []

def debacktick(s):
    return s.replace("`", "").strip()

# ── numbers ─────────────────────────────────────────────────────────────────────────────────
NUM = r"[-+]?\d+(?:\.\d+)?"

def parse_value(cell):
    """A declared constant: `4.0 cm`, `1/4`, `0.0 cm`. Returns Decimal or None."""
    s = debacktick(cell)
    s = s.replace("cm", " ").replace("≈", " ").strip()
    s = s.replace("−", "-").replace("×", "*").replace("÷", "/")
    m = re.fullmatch(r"\s*(%s)\s*/\s*(%s)\s*" % (NUM, NUM), s)
    if m:
        return Decimal(m.group(1)) / Decimal(m.group(2))
    m = re.fullmatch(r"\s*(%s)\s*" % NUM, s)
    if m:
        return Decimal(m.group(1))
    return None

def decimals(s):
    return len(s.split(".")[1]) if "." in s else 0

def render(value, dp):
    q = Decimal(1).scaleb(-dp)
    return str(value.quantize(q, rounding=ROUND_HALF_UP))

# ── the expression parser (no eval: the grammar is the contract) ────────────────────────────
class PErr(Exception):
    pass

def tokenize(src):
    src = src.replace("−", "-").replace("×", "*").replace("÷", "/").replace("–", "-")
    src = src.replace("`", "")
    toks, i = [], 0
    while i < len(src):
        c = src[i]
        if c.isspace():
            i += 1; continue
        if c in "+-*/()=":
            toks.append(c); i += 1; continue
        if c == "√":
            toks.append("√"); i += 1; continue
        if c == "²":
            toks.append("**2"); i += 1; continue
        m = re.match(r"\d+(?:\.\d+)?", src[i:])
        if m:
            toks.append(m.group(0)); i += len(m.group(0)); continue
        m = re.match(r"[A-Za-z_][A-Za-z0-9_]*", src[i:])
        if m:
            toks.append(m.group(0)); i += len(m.group(0)); continue
        raise PErr("character %r is not in the formula grammar" % c)
    return toks

class Parser:
    """expr := term (('+'|'-') term)*  ·  term := unary (('*'|'/') unary)*
       unary := '√' unary | '-' unary | power  ·  power := atom ('**2')?"""
    def __init__(self, toks, symbols):
        self.t, self.i, self.syms = toks, 0, symbols
    def peek(self):
        return self.t[self.i] if self.i < len(self.t) else None
    def take(self):
        self.i += 1
        return self.t[self.i - 1]
    def expr(self):
        v = self.term()
        while self.peek() in ("+", "-"):
            op = self.take()
            rhs = self.term()
            v = v + rhs if op == "+" else v - rhs
        return v
    def term(self):
        v = self.unary()
        while self.peek() in ("*", "/"):
            op = self.take()
            rhs = self.unary()
            if op == "*":
                v = v * rhs
            else:
                if rhs == 0:
                    raise PErr("division by zero")
                v = v / rhs
        return v
    def unary(self):
        if self.peek() == "√":
            self.take()
            v = self.unary()
            if v < 0:
                raise PErr("√ of a negative value")
            return Decimal(str(float(v) ** 0.5))
        if self.peek() == "-":
            self.take()
            return -self.unary()
        return self.power()
    def power(self):
        v = self.atom()
        if self.peek() == "**2":
            self.take()
            v = v * v
        return v
    def atom(self):
        tk = self.peek()
        if tk is None:
            raise PErr("formula ended mid-expression")
        if tk == "(":
            self.take()
            v = self.expr()
            if self.peek() != ")":
                raise PErr("unbalanced parenthesis")
            self.take()
            return v
        if re.fullmatch(r"\d+(?:\.\d+)?", tk):
            self.take()
            return Decimal(tk)
        if re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", tk):
            self.take()
            if tk not in self.syms:
                raise PErr("token `%s` is declared by no table above this row" % tk)
            return self.syms[tk]
        raise PErr("unexpected token %r" % tk)

def evaluate(formula, symbols):
    p = Parser(tokenize(formula), symbols)
    v = p.expr()
    if p.i != len(p.t):
        raise PErr("trailing %r after a complete expression" % " ".join(p.t[p.i:]))
    return v

# ── symbol tables: §2 measurements, §2 ease, §3 constants, §7 allowances ────────────────────
symbols = {}
where = {}

def declare(token, value, origin, cell):
    if value is None:
        bad("%s: `%s` has no parseable value in %s (cell %r)" % (origin, token, origin, cell))
        return
    if token in symbols and symbols[token] != value:
        bad("`%s` is declared twice with different values: %s in %s, %s in %s"
            % (token, symbols[token], where[token], value, origin))
        return
    symbols[token], where[token] = value, origin

if "2" not in SEC or "3" not in SEC or "4" not in SEC:
    print("fixture derivation: REFUSED — the chapter must carry §2, §3 and §4", file=sys.stderr)
    sys.exit(2)

hdr, rows = table_rows(SEC["2"], "Token")
for r in rows:
    if len(r) >= 4:
        declare(debacktick(r[0]), parse_value(r[3]), "§2 measurements", r[3])
hdr, rows = table_rows(SEC["2"], "Ease token")
for r in rows:
    if len(r) >= 4:
        declare(debacktick(r[0]), parse_value(r[3]), "§2 ease", r[3])
hdr, rows = table_rows(SEC["3"], "Token")
for r in rows:
    if len(r) >= 2:
        declare(debacktick(r[0]), parse_value(r[1]), "§3 constants", r[1])
if "7" in SEC:
    hdr, rows = table_rows(SEC["7"], "Token")
    for r in rows:
        if len(r) >= 3 and debacktick(r[0]) != "—":
            declare(debacktick(r[0]), parse_value(r[2]), "§7 allowances", r[2])

print("-- constants read from the chapter")
print("  §2/§3/§7 declarations: %d" % len(symbols))

# ── F1 + F2: §4's derived rows ──────────────────────────────────────────────────────────────
hdr, rows = table_rows(SEC["4"], "Token")
derived, checks, computed = 0, 0, []
for r in rows:
    if len(r) < 4:
        bad("§4 row has %d cells, the table declares 4: %r" % (len(r), r))
        continue
    token, formula_cell, result_cell = debacktick(r[0]), r[2], r[3]
    sides = [s.strip() for s in debacktick(formula_cell).split("=")]
    published = [p.strip() for p in debacktick(result_cell).split("=")]
    try:
        values = []
        for s in sides:
            values.append(evaluate(s, symbols))
    except PErr as e:
        bad("§4 `%s`: %s — formula %r" % (token, e, formula_cell))
        continue
    if len(published) != len(sides):
        bad("§4 `%s`: formula has %d side(s) but the result publishes %d"
            % (token, len(sides), len(published)))
        continue
    ok = True
    for v, pub in zip(values, published):
        m = re.match(r"^(%s)\s*cm$" % NUM, pub)
        if not m:
            bad("§4 `%s`: result cell %r is not a length in cm" % (token, pub))
            ok = False
            break
        dp = decimals(m.group(1))
        got = render(v, dp)
        if got != m.group(1):
            bad("§4 `%s`: %s computes to %s cm, the chapter publishes %s cm"
                % (token, formula_cell.strip(), got, m.group(1)))
            ok = False
    if len(sides) == 2:
        checks += 1
        if values[0] != values[1]:
            bad("§4 closure `%s` does not close: %s = %s but %s = %s"
                % (token, sides[0], values[0], sides[1], values[1]))
            ok = False
        if ok:
            print("  closure %-26s %s = %s  CLOSES" % (token, values[0], values[1]))
    else:
        derived += 1
        if ok:
            symbols[token] = values[0]          # later rows may refer to it (evaluation order)
            computed.append((token, render(values[0], 3)))
        else:
            symbols[token] = values[0]
print("-- F1 derived rows / F2 closure rows")
print("  derived: %d · closures: %d" % (derived, checks))

# ── P1 + P2: pieces, spans and the attachment account ───────────────────────────────────────
NON_SEWN = ("fused",)
pieces = []
if "6" in SEC:
    hdr, rows = table_rows(SEC["6"], "Piece")
    pieces = [debacktick(r[0]) for r in rows if len(r) >= 2 and debacktick(r[0]) not in ("—", "")]
spans = {}
if "8" in SEC:
    hdr, rows = table_rows(SEC["8"], "Span")
    for r in rows:
        if len(r) >= 3:
            spans[debacktick(r[0])] = (r[1], r[2])
account = {}
if "8" in SEC:
    hdr, rows = table_rows(SEC["8"], "Piece")
    for r in rows:
        if len(r) >= 2:
            account[debacktick(r[0])] = debacktick(r[1])

for piece, how in sorted(account.items()):
    if piece not in pieces:
        bad("§8 accounts for `%s`, which §6 does not list as a piece" % piece)
for p in pieces:
    if p not in account:
        bad("§6 lists piece `%s` but §8's attachment account does not account for it — "
            "a piece with neither a span nor a declared non-sewn attachment is defect D27's shape" % p)
        continue
    how = account[p]
    m = re.match(r"^spans?\s+(.*)$", how)
    if m:
        for sid in re.findall(r"`?([a-z][a-z0-9_]*)`?", m.group(1)):
            if sid not in spans:
                bad("§8's account for `%s` names span `%s`, which the span table does not declare" % (p, sid))
        continue
    m = re.match(r"^no span — (%s)" % "|".join(NON_SEWN), how)
    if not m:
        bad("§8's account for `%s` is neither `spans …` nor a non-sewn method from %s: %r"
            % (p, "/".join(NON_SEWN), how))
for sid, (a, b) in sorted(spans.items()):
    for side in (a, b):
        for named in re.findall(r"`([a-z][a-z0-9_]*)`", side):
            if named in pieces:
                continue
            if named in spans:
                continue                     # a stop landmark naming another span is not a piece
            bad("§8's span `%s` names `%s`, which §6 does not list as a piece" % (sid, named))

count_pub = None
if "12" in SEC:
    m = re.search(r"Package completeness:\s*(\d+)\s*pieces?", "\n".join(SEC["12"]))
    if m:
        count_pub = int(m.group(1))
if count_pub is None:
    bad("§12 publishes no `Package completeness: N pieces` count for P2 to check")
elif count_pub != len(pieces):
    bad("§12 publishes %d pieces but §6 lists %d (%s) — the count and the list are different readings"
        % (count_pub, len(pieces), ", ".join(pieces)))
print("-- P1 piece account / P2 piece count")
print("  pieces: %d · spans: %d · accounted: %d · published count: %s"
      % (len(pieces), len(spans), len(account), count_pub))

# ── B1: the band's shape, agreed between §4 and §6 ─────────────────────────────────────────
band_pieces = [p for p in pieces if p.startswith("waistband") and "interfacing" not in p]
width_rows = {t: v for t, v in symbols.items() if "cut_width" in t or t.endswith("_width")}
for req in ("wb_width", "sa_waist", "sa_wb_bottom"):
    if req not in symbols:
        bad("B1 cannot run: `%s` is declared by no table" % req)
if len(band_pieces) == 1:
    want = 2 * symbols["wb_width"] + symbols["sa_waist"] + symbols["sa_wb_bottom"]
    shape = "a single band folded lengthwise"
elif len(band_pieces) == 2:
    want = symbols["wb_width"] + symbols["sa_waist"] + symbols["sa_wb_bottom"]
    shape = "a faced two-piece band"
else:
    want, shape = None, "no readable band shape (%d band pieces)" % len(band_pieces)
    bad("B1: §6 lists %d waistband fabric pieces — %s" % (len(band_pieces), ", ".join(band_pieces) or "none"))
if want is not None:
    row = [t for t in width_rows if t == "waistband_cut_width"]
    if not row:
        bad("B1: §6 describes %s but §4 declares no `waistband_cut_width` row" % shape)
    elif symbols["waistband_cut_width"] != want:
        bad("B1: §6 describes %s (per-piece cut width %s cm) but `waistband_cut_width` publishes %s cm "
            "— §4 and §6 are different garments, which is defect D27" % (shape, want, symbols["waistband_cut_width"]))
    else:
        print("-- B1 band shape")
        print("  %s · cut width %s cm · agrees with §6's %d band piece(s)"
              % (shape, symbols["waistband_cut_width"], len(band_pieces)))

print("-- A1 advisory: every derived row as this run computed it")
for t, v in computed:
    print("  · %-26s %s cm" % (t, v))

print("fixture derivation: %d derived rows / %d closure checks / %d pieces / %d mismatch(es)"
      % (derived, checks, len(pieces), fails))
sys.exit(1 if fails else 0)
PY
rc=$?
exit "$rc"
