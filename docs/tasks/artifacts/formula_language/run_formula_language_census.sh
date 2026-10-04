#!/usr/bin/env bash
# docs/tasks/artifacts/formula_language/run_formula_language_census.sh
# G0-CONTRACT.9 — the instrument behind every claim the formula-language chapter makes about itself.
#
# WHY A TOOL AND NOT A RE-READ: the chapter's acceptance is that the language is implementable from
# the chapter alone. Prose cannot prove that, and a worked example whose published number was typed
# rather than computed is the D20/D33 shape this repository has already paid for twice: a number
# nobody can re-run, in a chapter every G1 test will be written from. So the census carries a
# REFERENCE EVALUATOR — a parser, a kind checker and an exact-rational evaluator — and the tables it
# type-checks with are read FROM THE CHAPTER, not hardcoded beside it. A signature the chapter does
# not declare cannot be evaluated; one it declares wrongly reddens this run instead of the product.
#
# WHAT IT CHECKS, mechanically. Each numbered rule fails the run:
#   L1 literals    every row of grammar §2's literal table canonicalizes to the form it publishes,
#                  using the unit ratios grammar §2.1 declares, so the two tables cannot disagree.
#   L2 bindings    every row of examples §2 parses, kind-checks against the kind it declares,
#                  evaluates over the fixture's own declared constants in declaration order, and
#                  renders to the published value at the published precision.
#   L3 agreement   a §2 token the fixture chapter also derives equals the fixture's published value.
#                  Two chapters describing two garments is defect D27's class, checked not read.
#   L4 assertions  every examples §3 `assert` holds at the tolerance class it names, and its verdict
#                  cell says so.
#   L5 refusals    every examples §4 row raises exactly the diagnostic the row names — the static
#                  checks before any value, the runtime ones while evaluating. A row that raises
#                  nothing, or another token, is a refusal.
#   L6 vocabulary  (a) every function call the three parts write is declared by grammar §6/§6.1, is
#                  the one special form, or is an envelope construct the contract §5.3 routes;
#                  (b) every formula-shaped code span uses only characters the grammar and the display
#                  table declare, so no chapter can invent an operator — over two populations, every
#                  span in the three parts and every span elsewhere in the book that carries a
#                  declared operator, because a span whose only operator is an invented one carries no
#                  declared operator to be found by; (c) the evaluator implements exactly what the chapter declares;
#                  (e) explicit single-span Rust annotations outside normative parts are well formed;
#                  (d) every `formula_*` / `env_*` token the three parts name is declared by the
#                  contract's §5.2 or §5.3.
#   L7 references  every link in the three parts resolves, a `§n` inside a link names a heading the
#                  target really carries, and the contract's §7 parts table lists exactly the files
#                  in the parts directory (both directions).
#   L8 limits      the declared `max_expression_nodes` exceeds the largest expression this book's
#                  examples carry by the margin the decision record states, so the limit is derived.
# ADVISORY, printed and never a failure:
#   A1 computed    every binding with the value this run computed, beside the published one.
#   A2 measured    the symbol table's size, the statement count, the largest expression, the widest
#                  rational, the deepest conditional — the numbers L8 and the margin claim rest on.
#
# HONEST LIMITS: it proves the chapter agrees with itself, with the fixture and with ONE evaluator
# written from it. It is not the product: `sc-core`'s implementation at G1 is, and this script is the
# oracle that implementation's examples are checked against (leaf `G1-SLICE.5`). It cannot tell
# whether a formula is the right garment engineering — `dart_intake_max` = 5.0 cm computes faithfully
# and is still `assumed`, which the vacant domain seat owns. Geometry is modelled as far as the
# examples need and no further: a point carries the coordinates its declaration gives it and an edge
# carries its length, so `point_at` returns an opaque point rather than a position on a curve the
# evaluator does not have. Irrational functions are computed in Decimal at 60 significant digits with
# pi derived by Machin's formula rather than typed, so a correctly-rounded result is this evaluator's
# own and not a libm's; a tie at the half-quantum is resolved by the units chapter's rule.
#
# DEPENDENCY: python3 (>= 3.8), already required by three other instruments here, so this adds no new
# toolchain requirement. Its absence is a refusal (exit 2), never a green run.
#
# Usage:  bash docs/tasks/artifacts/formula_language/run_formula_language_census.sh
#         FORMULA_ROOT=<dir> bash …      a scratch copy of the tree (the probe suites use this)
#         FORMULA_BOOK=<dir> bash …      another book source tree
#         FORMULA_CONTRACT / FORMULA_GRAMMAR / FORMULA_EXAMPLES / FORMULA_FIXTURE / FORMULA_PARTS_DIR
# Output: per-section tables, then
#         `formula-language census: <bindings> bindings / <assertions> assertions / <refusals> refusals / <n> mismatch(es)`
#         exit 1 on any mismatch, exit 2 when an input or the interpreter is missing.
set -uo pipefail
export LC_ALL=C

ROOT="${FORMULA_ROOT:-$(git rev-parse --show-toplevel 2>/dev/null || pwd)}"
BOOK="${FORMULA_BOOK:-$ROOT/docs/book/src}"
CONTRACT="${FORMULA_CONTRACT:-$BOOK/spec/formula-language.md}"
PARTS_DIR="${FORMULA_PARTS_DIR:-$BOOK/spec/formula-language}"
GRAMMAR="${FORMULA_GRAMMAR:-$PARTS_DIR/grammar.md}"
EXAMPLES="${FORMULA_EXAMPLES:-$PARTS_DIR/examples.md}"
FIXTURE="${FORMULA_FIXTURE:-$BOOK/spec/reference-skirt.md}"

for f in "$CONTRACT" "$GRAMMAR" "$EXAMPLES" "$FIXTURE"; do
  [ -f "$f" ] || { echo "formula-language census: REFUSED — $f not found" >&2; exit 2; }
done
[ -d "$PARTS_DIR" ] || { echo "formula-language census: REFUSED — $PARTS_DIR not found" >&2; exit 2; }
[ -d "$BOOK" ] || { echo "formula-language census: REFUSED — $BOOK not found" >&2; exit 2; }
command -v python3 >/dev/null 2>&1 || { echo "formula-language census: REFUSED — python3 not found" >&2; exit 2; }

python3 - "$CONTRACT" "$GRAMMAR" "$EXAMPLES" "$FIXTURE" "$BOOK" "$PARTS_DIR" <<'PY'
import pathlib, re, sys
from decimal import Decimal, getcontext, ROUND_HALF_UP
from fractions import Fraction

getcontext().prec = 60
CONTRACT, GRAMMAR, EXAMPLES, FIXTURE, BOOK, PARTS_DIR = sys.argv[1:7]

# the margin the decision record states for max_expression_nodes over the largest measured expression
NODE_MARGIN = 20

fails = 0
def bad(msg):
    global fails
    fails += 1
    print("  x %s" % msg)

def read(p):
    return pathlib.Path(p).read_text(encoding="utf-8")

def debacktick(s):
    return s.replace("`", "").strip()

def code_spans(text, allow_foreign=False):
    """Line-local inline spans outside fences, with an explicit one-span Rust annotation.

    Only callers scanning non-normative book chapters may allow foreign code. Invalid or detached
    markers refuse rather than silently changing a formula population. This is author-declared
    language context, not inference from punctuation or a Rust parser.
    """
    out, fenced = [], False
    for line in text.splitlines():
        if line.lstrip().startswith("```"):
            fenced = not fenced
            continue
        if fenced:
            continue
        spans = list(re.finditer(r"`([^`]+)`", line))
        excluded = set()
        for marker in re.finditer(r"<!--\s*stitchcad-inline\b.*?(?:-->|$)", line):
            if any(span.start() < marker.start() < span.end() for span in spans):
                continue  # A marker quoted inside code is literal content.
            next_span = next((span for span in spans if span.start() >= marker.end()), None)
            valid = marker.group() == "<!-- stitchcad-inline: rust -->" and next_span is not None \
                and not line[marker.end():next_span.start()].strip()
            if not allow_foreign or not valid:
                bad("L6e inline context: expected one immediate Rust span outside normative formula parts")
                continue
            excluded.add(next_span.start())
        out.extend(span.group(1) for span in spans if span.start() not in excluded)
    return out

def identifiers(cell):
    return re.findall(r"`([A-Za-z_][A-Za-z0-9_]*)`", cell)

# ── markdown plumbing (the reader the other instruments here use) ───────────────────────────
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

def table_in(body, first_cell):
    for header, rows in tables(body):
        if header and debacktick(header[0]) == first_cell:
            return header, rows
    return [], []

def headings(path):
    out = set()
    for ln in read(path).splitlines():
        m = re.match(r"^#{2,4} (\d+(?:\.\d+)*)\.?\s", ln)
        if m: out.add(m.group(1))
    return out

# ── numbers ────────────────────────────────────────────────────────────────────────────────
RATIO_SCALE = 1000000

def rnd(fr):
    """Round to an integer, half away from zero — the units chapter's one rule."""
    fr = Fraction(fr)
    n, d = fr.numerator, fr.denominator
    if d == 1: return n
    sign = -1 if n < 0 else 1
    q, r = divmod(abs(n), d)
    if 2 * r >= d: q += 1
    return sign * q

def to_true(kind, v):
    return Fraction(v) / RATIO_SCALE if kind == "ratio" else Fraction(v)

def from_true(kind, x):
    return Fraction(x) * RATIO_SCALE if kind == "ratio" else Fraction(x)

def dfraction(fr):
    return Decimal(Fraction(fr).numerator) / Decimal(Fraction(fr).denominator)

def render(kind, internal, dp):
    """Published Value cells: cm, deg, cm², dimensionless numbers or Boolean text."""
    if kind == "boolean":
        if internal not in (0, 1):
            raise FErr("formula_domain", "Boolean display requires 0 or 1, measured=%s" % internal)
        return "true" if internal == 1 else "false"
    if kind == "count":
        return str(internal)
    den = {"length": 10000, "area": 100000000}.get(kind, RATIO_SCALE)
    value = Decimal(internal) / Decimal(den)
    return format(value.quantize(Decimal(1).scaleb(-dp), rounding=ROUND_HALF_UP), "f")

# ── irrational functions, with pi derived rather than typed ────────────────────────────────
class FErr(Exception):
    def __init__(self, token, msg, arguments=None):
        Exception.__init__(self, "%s: %s" % (token, msg))
        self.token, self.msg = token, msg
        self.arguments = dict(arguments) if arguments is not None else {}

def _atan_series(x):
    total, num, x2 = Decimal(0), x, x * x
    for k in range(600):
        t = num / (2 * k + 1)
        total += t if k % 2 == 0 else -t
        num *= x2
        if abs(t) < Decimal(10) ** -52: break
    return total

def d_atan(x):
    neg = x < 0
    x = abs(x)
    halvings = 0
    while x > Decimal("0.5"):
        x = x / (1 + (1 + x * x).sqrt()); halvings += 1
    v = _atan_series(x) * (2 ** halvings)
    return -v if neg else v

PI = 4 * (4 * d_atan(Decimal(1) / Decimal(5)) - d_atan(Decimal(1) / Decimal(239)))

def d_atan2(y, x):
    if x > 0: return d_atan(y / x)
    if x < 0: return d_atan(y / x) + (PI if y >= 0 else -PI)
    if y > 0: return PI / 2
    if y < 0: return -PI / 2
    raise FErr("formula_domain", "atan2(0, 0) has no direction")

def _reduce(x):
    two_pi = 2 * PI
    x = x % two_pi
    return x - two_pi if x > PI else x

def _series(x, sine):
    x = _reduce(x)
    total = Decimal(0)
    term = x if sine else Decimal(1)
    for k in range(300):
        total += term
        if sine: term *= -x * x / ((2 * k + 2) * (2 * k + 3))
        else:    term *= -x * x / ((2 * k + 1) * (2 * k + 2))
        if abs(term) < Decimal(10) ** -52: break
    return total

def d_sin(x): return _series(x, True)
def d_cos(x): return _series(x, False)

def norm_angle(udeg):
    return int(udeg) % (360 * 1000000)

def d_radians(udeg):
    """One direct conversion from internal microdegrees; preserve signed/fractional sweeps."""
    return dfraction(udeg) * PI / (180 * 1000000)

ARITH = ("length", "angle", "area", "ratio", "count")
NEGATABLE = ("length", "angle", "area", "ratio")
IMPLEMENTED = {"sqrt", "hypot", "abs", "min", "max", "clamp", "round_to", "sin", "cos", "tan",
               "atan", "atan2", "arc_length", "if", "within",
               "x", "y", "dist", "dir", "len", "param_at", "point_at"}

class Val:
    def __init__(self, kind, v, sources=(), components=None):
        self.kind, self.v = kind, v
        self.sources = frozenset(sources)
        self.components = components

    def influenced(self, *values, sources=()):
        """Retain executed dependencies even when their numeric contribution cancels."""
        combined = self.sources.union(sources, *(value.sources for value in values))
        return Val(self.kind, self.v, combined, self.components)

    def __repr__(self):
        return "%s:%s" % (self.kind, self.v)

# ── the evaluator: written from grammar §1–§7, driven by the chapter's own tables ──────────
class Evaluator:
    def __init__(self, bindable, pairs, sigs, reserved, envelope, limits, units, domains, storage, origins=None, context=None, *, operator_signatures):
        self.operator_signatures = operator_signatures
        self.bindable, self.pairs, self.sigs = bindable, pairs, sigs
        self.reserved, self.envelope, self.limits, self.units = reserved, envelope, limits, units
        self.domains, self.storage = domains, storage
        self.origins = frozenset(origins if origins is not None else
                                ("measurement", "ease", "parameter", "profile", "material",
                                 "geometry", "recipe", "size", "tolerance"))
        self.context = context
        self.max_nodes = self.max_bits = self.max_if = 0

    TOK = re.compile(r"""(?P<cmp>==|!=|<=|>=|<|>)|(?P<op>[-+*/^=(),:])|(?P<num>\d+\.\d+|\d+)
                       |(?P<str>"[^"]*")|(?P<id>[A-Za-z_][A-Za-z0-9_]*)|(?P<sp>[ \t\n\r\f\v]+)|(?P<bad>.)""", re.X)

    @staticmethod
    def _identifier(text, allow_keyword=False):
        if not re.fullmatch(r"[a-z][a-z0-9]*(?:_[a-z0-9]+)*", text):
            raise FErr("formula_parse", "identifier %r violates lower-snake spelling" % text)
        if not allow_keyword and text in {"let", "assert", "if"}:
            raise FErr("formula_parse", "keyword %r is not an identifier here" % text)
        return text

    def tokenize(self, src):
        if not src.isascii():
            raise FErr("formula_parse", "machine syntax must be ASCII")
        raw, i = [], 0
        while i < len(src):
            m = self.TOK.match(src, i)
            if not m:
                raise FErr("formula_parse", "character %r is not in the grammar" % src[i])
            i = m.end()
            kind, text = m.lastgroup, m.group()
            if kind == "bad":
                raise FErr("formula_parse", "character %r is not in the grammar" % text)
            if kind == "str":
                raise FErr("formula_parse", "the language has no text value (%s)" % text)
            if kind == "id":
                self._identifier(text, allow_keyword=True)
            raw.append((kind, text, m.start()))
        kept = [t for t in raw if t[0] != "sp"]
        # Inspect original source gaps, including absent whitespace, before any source respelling.
        for left, right in zip(kept, kept[1:]):
            if left[0] == "num" and right[0] == "id" and right[1] in self.units:
                gap = src[left[2] + len(left[1]):right[2]]
                if gap != " ":
                    raise FErr("formula_parse", "a literal's unit follows it with exactly one space")
        return [(k, t) for k, t, _ in kept], [s for _, _, s in kept]

    def parse(self, src, *, literal_inputs=True):
        self.src = src
        self.t, self.starts = self.tokenize(src)
        self.i = 0
        node = self.p_expr(literal_inputs)
        if self.i != len(self.t):
            raise FErr("formula_parse", "trailing %r after a complete expression"
                       % " ".join(t[1] for t in self.t[self.i:]))
        self.check_limits(node)
        return node

    def peek(self, k=0):
        j = self.i + k
        return self.t[j] if j < len(self.t) else (None, None)

    def take(self):
        self.i += 1
        return self.t[self.i - 1]

    def p_expr(self, literal_inputs):
        left = self.p_add(literal_inputs)
        if self.peek()[0] == "cmp":
            op = self.take()[1]
            right = self.p_add(literal_inputs)
            if self.peek()[0] == "cmp":
                raise FErr("formula_parse", "a comparison does not chain")
            return ("cmp", op, left, right)
        return left

    def p_add(self, literal_inputs):
        node = self.p_mul(literal_inputs)
        while self.peek()[0] == "op" and self.peek()[1] in ("+", "-"):
            op = self.take()[1]
            node = ("bin", op, node, self.p_mul(literal_inputs))
        return node

    def p_mul(self, literal_inputs):
        node = self.p_unary(literal_inputs)
        while self.peek()[0] == "op" and self.peek()[1] in ("*", "/"):
            op = self.take()[1]
            node = ("bin", op, node, self.p_unary(literal_inputs))
        return node

    def p_unary(self, literal_inputs):
        if self.peek()[0] == "op" and self.peek()[1] == "-":
            self.take()
            return ("neg", self.p_unary(literal_inputs))
        return self.p_power(literal_inputs)

    def p_power(self, literal_inputs):
        node = self.p_postfix(literal_inputs)
        if self.peek()[0] == "op" and self.peek()[1] == "^":
            self.take()
            kind, text = self.peek()
            if kind != "num" or text != "2":
                raise FErr("formula_unsupported", "only the square is in the grammar (found %r)" % text)
            self.take()
            node = ("sq", node)
        return node

    def p_postfix(self, literal_inputs):
        kind, text = self.peek()
        if kind == "id" and self.peek(1)[1] == "(":
            name = self.take()[1]
            if name != "if":
                self._identifier(name)
            self.take()
            args = self.p_args(literal_inputs)
            if name == "if":
                if len(args) != 3:
                    raise FErr("formula_parse", "a conditional takes three parts, got %d" % len(args))
                return ("if", args[0], args[1], args[2])
            return ("call", name, args)
        return self.p_atom(literal_inputs)

    def p_args(self, literal_inputs):
        args = []
        if self.peek()[1] == ")":
            raise FErr("formula_parse", "call arguments require at least one expression")
        while True:
            args.append(self.p_expr(literal_inputs))
            if self.peek()[1] == ",":
                self.take(); continue
            if self.peek()[1] == ")":
                self.take(); return args
            raise FErr("formula_parse", "expected , or ) and found %r" % (self.peek()[1],))

    def p_atom(self, literal_inputs):
        kind, text = self.peek()
        if kind is None:
            raise FErr("formula_parse", "the expression ended mid-form")
        if text == "(":
            self.take()
            node = self.p_expr(literal_inputs)
            if self.peek()[1] != ")":
                raise FErr("formula_parse", "unbalanced parenthesis")
            self.take()
            return node
        if kind == "num":
            self.take()
            unit = None
            if self.peek()[0] == "id" and self.peek()[1] in self.units:
                unit = self.take()[1]
            if not literal_inputs:
                return ("raw_lit", text, unit)
            return self._literal_input(text, unit)
        if kind == "id":
            self._identifier(text)
            self.take()
            return ("name", text)
        raise FErr("formula_parse", "unexpected %r" % text)

    def _parse_syntax(self, src):
        """Whole-source syntax does not construct or normalize numeric literal values."""
        return self.parse(src, literal_inputs=False)

    def _literal_input(self, text, unit):
        """Normalize one original number/unit pair using the existing input rules."""
        value = Fraction(Decimal(text))
        if unit is None:
            if "." in text:
                self.see(from_true("ratio", value), "literal ratio conversion")
                return self.literal("ratio", from_true("ratio", value))
            self.see(value, "literal count")
            self.scalar("count", value, "literal count input")
            return ("lit", "count", value)
        ukind, factor = self.units[unit]
        self.see(value * factor, "literal %s conversion" % ukind)
        return self.literal(ukind, value * factor)

    def _normalize_input(self, node):
        """Iterative postorder conversion; no operators, names or values are evaluated."""
        pending, normalized = [(node, False)], []
        while pending:
            current, ready = pending.pop()
            if current[0] == "raw_lit":
                normalized.append(self._literal_input(current[1], current[2]))
                continue
            children = self.syntax_children(current)
            if not children:
                normalized.append(current)
                continue
            if not ready:
                pending.append((current, True))
                pending.extend((child, False) for child in reversed(children))
                continue
            converted = normalized[-len(children):]
            del normalized[-len(children):]
            if current[0] == "call":
                normalized.append(("call", current[1], converted))
            else:
                operands = iter(converted)
                normalized.append(tuple(next(operands) if isinstance(part, tuple) else part
                                        for part in current))
        assert len(normalized) == 1, "literal input traversal must yield exactly one root"
        return normalized[0]

    @staticmethod
    def syntax_children(node):
        """Semantic AST children; call argument lists are not themselves expression nodes."""
        if node[0] == "call":
            return node[2]
        return [child for child in node if isinstance(child, tuple)]

    def count_nodes(self, node):
        total, pending = 0, [node]
        while pending:
            current = pending.pop()
            total += 1
            pending.extend(self.syntax_children(current))
        return total

    def if_depth(self, node):
        deepest, pending = 0, [(node, 0)]
        while pending:
            current, depth = pending.pop()
            depth += int(current[0] == "if")
            deepest = max(deepest, depth)
            pending.extend((child, depth) for child in self.syntax_children(current))
        return deepest

    def check_limits(self, node):
        n = self.count_nodes(node)
        self.max_nodes = max(self.max_nodes, n)
        depth = self.if_depth(node)
        self.max_if = max(self.max_if, depth)
        if n > self.limits["max_expression_nodes"]:
            raise FErr("formula_domain", "expression has %d nodes, the limit is %d"
                       % (n, self.limits["max_expression_nodes"]))
        if depth > self.limits["max_if_depth"]:
            raise FErr("formula_domain", "conditional depth is %d, the limit is %d"
                       % (depth, self.limits["max_if_depth"]))

    def see(self, fr, operation="value"):
        """Measure and refuse reduced VALUE width, never an unreduced arithmetic temporary."""
        fr = Fraction(fr)
        bits = max(abs(fr.numerator).bit_length(), fr.denominator.bit_length())
        self.max_bits = max(self.max_bits, bits)
        bound = self.limits["max_rational_bits"]
        if bits > bound:
            raise FErr("formula_domain", "%s: rational max_rational_bits=%d, measured=%d"
                       % (operation, bound, bits))

    def scalar(self, kind, value, operation):
        """Exact completed scalar domain; independent of rounding and reduced-value width."""
        if kind not in self.domains:
            return
        low, high = self.domains[kind]
        value = Fraction(value)
        if value < low or (high is not None and value > high):
            bound = "[%s, %s]" % (low, high if high is not None else "unbounded")
            raise FErr("formula_domain", "%s: %s domain=%s, measured=%s"
                       % (operation, kind, bound, value))

    def stored(self, value, operation):
        """Bind once-rounded numeric integers; exact expression nodes remain wider."""
        if value.kind not in ARITH:
            return value
        integer = rnd(value.v)
        self.scalar(value.kind, integer, operation)
        bits, low, high = self.storage[value.kind]
        if integer < low or integer > high:
            raise FErr("formula_domain", "%s: %s storage=i%d [%s, %s], measured=%s"
                       % (operation, value.kind, bits, low, high, integer))
        return Val(value.kind, Fraction(integer)).influenced(value)

    def literal(self, kind, exact):
        rounded = Fraction(rnd(exact))
        self.scalar(kind, rounded, "literal %s input" % kind)
        return ("lit", kind, rounded)

    @staticmethod
    def _tolerance_name(node):
        if node[0] == "name" and node[1] in {"eps_num", "eps_geo", "eps_fmt", "eps_imp", "eps_phys"}:
            return node[1]
        return None

    def _dimension(self, operation, nodes, kinds, signatures, message):
        # Operands have already resolved; no value/state or invented kind enters this payload.
        roles = tuple(self._tolerance_name(node) for node in nodes)
        assert len(roles) == len(kinds)
        wanted = tuple({"operands": tuple(args), "variadic": variadic, "result": result}
                       for args, variadic, result in signatures)
        raise FErr("formula_dimension", message,
                   {"operation": operation, "operand_kinds": tuple(kinds),
                    "operand_tolerances": roles, "wanted_signatures": wanted})

    # -- static kind inference --
    def infer(self, node, env):
        tag = node[0]
        if tag == "lit": return node[1]
        if tag == "name": return self.kind_of_name(node[1], env)
        if tag == "neg":
            k = self.infer(node[1], env)
            if k not in NEGATABLE:
                self._dimension("-", (node[1],), (k,), self.operator_signatures["-", 1],
                                "unary - has no rule for %s" % k)
            return k
        if tag == "sq":
            k = self.infer(node[1], env)
            if k not in ("length", "ratio", "count"):
                self._dimension("^2", (node[1],), (k,), self.operator_signatures["^2", 1],
                                "^ 2 has no rule for %s" % k)
            return {"length": "area", "ratio": "ratio", "count": "count"}[k]
        if tag == "cmp":
            a, b = self.infer(node[2], env), self.infer(node[3], env)
            if a != b or a not in ARITH:
                self._dimension(node[1], node[2:4], (a, b), self.operator_signatures[node[1], 2],
                                "%s cannot compare %s with %s" % (node[1], a, b))
            return "boolean"
        if tag == "bin":
            op = node[1]
            a, b = self.infer(node[2], env), self.infer(node[3], env)
            if op in ("+", "-"):
                if a != b or a not in ARITH:
                    self._dimension(op, node[2:4], (a, b), self.operator_signatures[op, 2],
                                    "%s has no rule for %s and %s" % (op, a, b))
                return a
            if op == "*":
                pair = self.pairs.get((a, b)) or self.pairs.get((b, a))
            else:
                pair = self.pairs.get((a, b))
            res = pair.get(op) if pair else None
            if res is None:
                hint = " — an arc's length is arc_length(angle, radius)" \
                    if op == "*" and {a, b} == {"angle", "length"} else ""
                self._dimension(op, node[2:4], (a, b), self.operator_signatures[op, 2],
                                "%s has no rule for %s and %s%s" % (op, a, b, hint))
            return res
        if tag == "if":
            c = self.infer(node[1], env)
            a, b = self.infer(node[2], env), self.infer(node[3], env)
            if c != "boolean":
                self._dimension("if", node[1:4], (c, a, b), self.sigs["if"],
                                "a conditional's test is %s, not boolean" % c)
            if a != b or a not in ARITH:
                self._dimension("if", node[1:4], (c, a, b), self.sigs["if"],
                                "a conditional's two branches are %s and %s" % (a, b))
            return a
        if tag == "call":
            return self.infer_call(node[1], node[2], env)
        raise FErr("formula_parse", "unknown node %r" % (tag,))

    def kind_of_name(self, name, env):
        if name in self.reserved: return self.reserved[name][0]
        if name in env: return env[name]["kind"]
        self._missing_name(name)

    def _check_callee(self, name):
        # Call domains differ from data origins; refusal precedes any argument inspection.
        if name in self.envelope:
            token = self.envelope[name]
            payload = {"name": name, "lookup_scope": "formula_call", "origins_searched": ("envelope",)}
            if token == "env_nurbs":
                payload.update(curve_kind=name,
                               supported_curve_set=("line_segment", "circular_arc", "cubic_bezier"))
            else:
                payload.update(constraint_kind=name, recipe_alternative="ordered_construction_recipe")
            raise FErr(token, "the envelope owns this construct, not the language", payload)
        if name not in self.sigs:
            raise FErr("formula_unbound_name", "`%s` is no declared function or selector" % name,
                       {"name": name, "lookup_scope": "formula_call",
                        "origins_searched": ("envelope", "builtin_catalog")})

    def infer_call(self, name, args, env):
        self._check_callee(name)
        kinds = [self.infer(a, env) for a in args]
        if name == "within" and not (len(args) == 3 and self._tolerance_name(args[2]) is not None):
            self._dimension(name, args, kinds, self.sigs[name], "within's third argument is a tolerance name")
        for arg_kinds, variadic, result in self.sigs[name]:
            bound = self._matches(arg_kinds, variadic, kinds)
            if bound is not False:
                return bound if result in ("T", "N") else result
        self._dimension(name, args, kinds, self.sigs[name],
                        "`%s` has no signature for (%s)" % (name, ", ".join(kinds)))

    def _matches(self, want_list, variadic, kinds):
        """False when the signature does not fit; otherwise the kind `T` bound to (None if it has no T)."""
        if variadic:
            if len(kinds) < len(want_list): return False
            first = kinds[0]
            if first not in ARITH: return False
            if not all(k == first for k in kinds): return False
            return first
        if len(kinds) != len(want_list): return False
        bound = None
        for want, got in zip(want_list, kinds):
            if want == "tolerance":
                continue                      # infer_call already proved it is a tolerance name
            if want == "T":
                if got not in ARITH: return False
                if bound is None: bound = got
                elif bound != got: return False
            elif want == "N":
                if got not in NEGATABLE: return False
                if bound is None: bound = got
            elif want != got:
                return False
        return bound

    # -- evaluation --
    def evaluate(self, node, env):
        value = self._evaluate(node, env)
        if value.kind in ARITH:
            self.see(value.v, "%s %s result" % (node[0], value.kind))
            operation = node[0] + (" " + node[1] if node[0] in ("name", "bin", "call") else "")
            self.scalar(value.kind, value.v, "%s %s result" % (operation, value.kind))
        return value

    def _evaluate(self, node, env):
        tag = node[0]
        if tag == "lit": return Val(node[1], node[2])
        if tag == "name": return self.value_of_name(node[1], env)
        if tag == "neg":
            v = self.evaluate(node[1], env)
            return Val(v.kind, -v.v).influenced(v)
        if tag == "sq":
            v = self.evaluate(node[1], env)
            res = self.infer(node, env)
            exact = to_true(v.kind, v.v) ** 2
            return Val(res, from_true(res, exact)).influenced(v)
        if tag == "cmp":
            a = self.evaluate(node[2], env); b = self.evaluate(node[3], env)
            ok = {"==": a.v == b.v, "!=": a.v != b.v, "<": a.v < b.v,
                  "<=": a.v <= b.v, ">": a.v > b.v, ">=": a.v >= b.v}[node[1]]
            return Val("boolean", Fraction(1 if ok else 0)).influenced(a, b)
        if tag == "bin":
            op = node[1]
            a = self.evaluate(node[2], env); b = self.evaluate(node[3], env)
            res = self.infer(node, env)
            if op == "+": return Val(res, a.v + b.v).influenced(a, b)
            if op == "-": return Val(res, a.v - b.v).influenced(a, b)
            ta, tb = to_true(a.kind, a.v), to_true(b.kind, b.v)
            if op == "*":
                return Val(res, from_true(res, ta * tb)).influenced(a, b)
            if tb == 0:
                raise FErr("formula_division", "the divisor is zero")
            return Val(res, from_true(res, ta / tb)).influenced(a, b)
        if tag == "if":
            c = self.evaluate(node[1], env)
            return self.evaluate(node[2] if c.v else node[3], env).influenced(c)
        if tag == "call":
            return self.call(node[1], node[2], env)
        raise FErr("formula_parse", "unknown node %r" % (tag,))

    def _missing_name(self, name, origin=None, state=None):
        """Route absent values by declared origin; attach only actually available context."""
        arguments = {"name": name}
        if origin is not None:
            arguments["origin"] = origin
        if origin in ("measurement", "ease", "parameter", "profile", "material"):
            if state not in ("known", "assumed", "preference", "derived", "unknown"):
                raise FErr("formula_parse", "missing value for `%s` has no valid authored state" % name)
            token = "formula_unknown"
            arguments["state"] = state
            message = "`%s` is %s, so it has no value and none is invented" % (name, state)
        elif origin == "tolerance":
            token = "formula_tolerance_unbound"
            if self.context is not None:
                arguments["context"] = self.context
            message = "`%s` is read in a context that supplies no value for it" % name
        else:
            token = "formula_unbound_name"
            arguments["origins_searched"] = tuple(sorted(self.origins))
            message = "`%s` is declared by no origin visible here" % name if origin is None else (
                "`%s` has no visible value from origin %s" % (name, origin))
        raise FErr(token, message, arguments)

    def value_of_name(self, name, env):
        if name in self.reserved:
            kind, always, value = self.reserved[name]
            if value is None:
                self._missing_name(name, "tolerance" if name in
                                   {"eps_num", "eps_geo", "eps_fmt", "eps_imp", "eps_phys"} else "size")
            return Val(kind, Fraction(value))
        if name not in env:
            self._missing_name(name)
        e = env[name]
        state = e.get("state")
        if "state" in e and state not in ("known", "assumed", "preference", "derived", "unknown"):
            raise FErr("formula_parse", "value for `%s` has no valid authored state" % name)
        if state == "unknown" and e.get("value") is not None:
            raise FErr("formula_parse", "unknown `%s` cannot supply a value" % name)
        if state == "unknown" and e.get("lazy"):
            self._missing_name(name, e["origin"], state)
        if e.get("lazy"):
            e["value"] = self.resolve_geometry(e, env)
            e["lazy"] = False
        if e.get("value") is None:
            origin = e.get("origin")
            if not isinstance(origin, str) or origin not in self.origins:
                raise FErr("formula_parse", "missing value for `%s` has no valid declaration origin" % name)
            self._missing_name(name, origin, e.get("state"))
        return Val(e["kind"], e["value"], e.get("sources", ()), e.get("components"))

    def resolve_geometry(self, entry, env):
        """An operation's argument formulas are evaluated at the operation's position (§4.1)."""
        operation = entry["kind"]
        names = ("len",) if operation == "edge" else ("x", "y")
        nodes = tuple(self.parse(entry["exprs"][name]) for name in names)
        kinds = tuple(self.infer(node, env) for node in nodes)
        if any(kind != "length" for kind in kinds):
            raise FErr("formula_dimension", "%s arguments require length, received %s" % (operation, kinds),
                       {"diagnostic_scope": "geometry_arguments", "operation": operation,
                        "operand_names": names, "operand_kinds": kinds,
                        "wanted_kinds": ("length",) * len(names)})
        # All arguments are statically accepted before the first value or cache contribution.
        values = tuple(self.evaluate(node, env) for node in nodes)
        if operation == "edge":
            entry["sources"] = values[0].sources
            return Fraction(values[0].v)
        xs, ys = values
        entry["sources"] = xs.sources | ys.sources
        entry["components"] = (xs.sources, ys.sources)
        return (Fraction(xs.v), Fraction(ys.v))

    @staticmethod
    def tolerance_class(comparison, name, a, b):
        sources = a.sources | b.sources
        if name == "eps_num" and sources:
            raise FErr("formula_domain", "%s: `%s` requires T2 or looser; contributions=%s"
                       % (comparison, name, ", ".join(sorted(sources))),
                       {"comparison": comparison, "tolerance_class": name,
                        "contribution_sources": tuple(sorted(sources))})

    def call(self, name, args, env):
        self._check_callee(name)
        if name == "if":
            return self.evaluate(("if", args[0], args[1], args[2]), env)
        if name == "within":
            a = self.evaluate(args[0], env); b = self.evaluate(args[1], env)
            tol = self.evaluate(args[2], env)
            self.tolerance_class("within", args[2][1], a, b)
            return Val("boolean", Fraction(1 if abs(a.v - b.v) <= tol.v else 0)).influenced(a, b, tol)
        if name in ("min", "max"):
            vs = [self.evaluate(a, env) for a in args]
            return Val(vs[0].kind, (min if name == "min" else max)(v.v for v in vs)).influenced(*vs)
        vs = [self.evaluate(a, env) for a in args]
        k0 = vs[0].kind
        if name in ("x", "y"):
            index = 0 if name == "x" else 1
            sources = vs[0].components[index] if vs[0].components is not None else vs[0].sources
            return Val("length", vs[0].v[index], sources)
        if name == "len": return Val("length", vs[0].v).influenced(*vs)
        if name == "dist":
            p, q = vs
            return Val("length", self._hypot(p.v[0] - q.v[0], p.v[1] - q.v[1])).influenced(*vs, sources=(name,))
        if name == "dir":
            p, q = vs
            return Val("angle", norm_angle(rnd(self._deg2(q.v[1] - p.v[1], q.v[0] - p.v[0])))).influenced(*vs, sources=(name,))
        if name == "param_at":
            e, l = vs
            if l.v > e.v:
                raise FErr("formula_domain", "param_at: %d is past the edge's %d" % (l.v, e.v))
            return Val("ratio", from_true("ratio", to_true("length", l.v)
                                         / to_true("length", e.v))).influenced(*vs)
        if name == "point_at":
            t = to_true("ratio", vs[1].v)
            if t < 0 or t > 1:
                raise FErr("formula_domain", "point_at: parameter %s is outside [0, 1]" % t)
            return Val("point", vs[1].v).influenced(*vs)  # opaque position; no curve model
        if name == "abs": return Val(k0, abs(vs[0].v)).influenced(*vs)
        if name == "clamp":
            lo, hi = vs[1].v, vs[2].v
            if lo > hi:
                raise FErr("formula_domain", "clamp: low %s is above high %s" % (lo, hi))
            return Val(k0, min(max(vs[0].v, lo), hi)).influenced(*vs)
        if name == "round_to":
            step = vs[1].v
            if step == 0:
                raise FErr("formula_division", "round_to's step is zero")
            return Val(k0, rnd(Fraction(vs[0].v) / Fraction(step)) * step).influenced(*vs, sources=(name,))
        if name == "sqrt":
            if vs[0].v < 0:
                raise FErr("formula_domain", "sqrt of a negative %s" % k0)
            if k0 == "area":
                return Val("length", rnd(dfraction(vs[0].v).sqrt())).influenced(*vs, sources=(name,))
            return Val("ratio", rnd(from_true("ratio", d_sqrt_ratio(vs[0].v)))).influenced(*vs, sources=(name,))
        if name == "hypot":
            return Val("length", self._hypot(vs[0].v, vs[1].v)).influenced(*vs, sources=(name,))
        if name in ("sin", "cos", "tan"):
            if name == "tan" and vs[0].v % (180 * 1000000) == 90 * 1000000:
                raise FErr("formula_domain", "tan: angle is an exact odd-quarter-turn pole")
            rad = d_radians(vs[0].v)
            r = {"sin": d_sin, "cos": d_cos,
                 "tan": lambda z: d_sin(z) / d_cos(z)}[name](rad)
            return Val("ratio", rnd(from_true("ratio", r))).influenced(*vs, sources=(name,))
        if name == "atan":
            return Val("angle", rnd(d_atan(dfraction(to_true("ratio", vs[0].v)))
                                     * 180 / PI * 1000000)).influenced(*vs, sources=(name,))
        if name == "atan2":
            ya = dfraction(to_true(vs[0].kind, vs[0].v))
            xb = dfraction(to_true(vs[1].kind, vs[1].v))
            return Val("angle", rnd(d_atan2(ya, xb) * 180 / PI * 1000000)).influenced(*vs, sources=(name,))
        if name == "arc_length":
            rad = d_radians(vs[0].v)
            return Val("length", rnd(rad * dfraction(to_true("length", vs[1].v)))).influenced(*vs, sources=(name,))
        raise FErr("formula_unsupported", "`%s` is declared but not implemented here" % name)

    def _hypot(self, a, b):
        return rnd(dfraction(Fraction(a) ** 2 + Fraction(b) ** 2).sqrt())

    def _deg2(self, y, x):
        return d_atan2(dfraction(y), dfraction(x)) * 180 / PI * 1000000

    # -- namespaces and the single-statement static phase --
    def _reserved_source(self, name):
        """Static metadata only: never inspect a value or context availability."""
        size = name in ("size_index", "size_count", "is_base_size")
        context = ("size" if size else "always" if name in ("eps_num", "eps_geo")
                   else "profile" if name == "eps_phys" else "export")
        return {"role": "reserved", "kind": self.reserved[name][0],
                "origin": "size" if size else "tolerance", "required_context": context}

    def _recipe_source(self, src, name, kind, ordinal=None, offset=0):
        """Location from an actual parsed let; detached syntax has no recipe ordinal."""
        _, starts = self.tokenize(src)
        name_start = offset + starts[1]
        source = {"role": "recipe", "kind": kind, "origin": "recipe",
                  "span": (offset, offset + len(src)),
                  "name_span": (name_start, name_start + len(name))}
        if ordinal is not None:
            source["statement_index"] = ordinal
        return source

    def _initial_source(self, kind, origin, declaration_index=None):
        """Only ordered pair consumption supplies an actual declaration position."""
        source = {"role": "initial_declaration", "kind": kind, "origin": origin}
        if declaration_index is not None:
            source["declaration_index"] = declaration_index
        return source

    def _ambiguity_arguments(self, name, prior, attempted):
        """Retain both actual sources in binding order, including equal origins."""
        return {"name": name, "origins": (prior["origin"], attempted["origin"]),
                "prior_source": dict(prior), "attempted_source": dict(attempted)}

    def namespace(self, declarations):
        """Consume pairs before duplicate declarations can disappear in a dict."""
        env, sources = {}, {}
        for declaration_index, (name, entry) in enumerate(declarations, 1):
            self._identifier(name)
            try:
                kind, origin = entry["kind"], entry["origin"]
            except (KeyError, TypeError):
                raise FErr("formula_parse", "missing declaration kind/origin for `%s`" % name)
            if (not isinstance(kind, str) or not isinstance(origin, str)
                    or kind not in (*self.bindable, "point", "edge") or origin not in self.origins):
                raise FErr("formula_parse", "invalid declaration kind/origin for `%s`" % name)
            if name in self.reserved:
                raise FErr("formula_rebinding", "reserved `%s` cannot be declared by %s" % (name, origin),
                           {"name": name, "reason": "reserved_name",
                            "reserved_source": self._reserved_source(name),
                            "attempted_source": {"role": "initial_declaration", "kind": kind,
                                                 "origin": origin, "declaration_index": declaration_index}})
            if name in env:
                raise FErr("formula_ambiguous_name", "`%s` is declared by both %s and %s"
                           % (name, env[name]["origin"], origin),
                           self._ambiguity_arguments(name, sources[name],
                                                     self._initial_source(kind, origin, declaration_index)))
            env[name] = entry
            sources[name] = self._initial_source(kind, origin, declaration_index)
        return env

    def _binding_dimension(self, src, name, annotation, expression_kind=None, ordinal=None, offset=0):
        """Header arguments retain original source; no absent operand kind is invented."""
        _, starts = self.tokenize(src)
        start = starts[3]  # Validated let/name/colon/id header; original ASCII byte position.
        arguments = {"diagnostic_scope": "binding_annotation" if expression_kind is None else "binding_kind",
                     "operation": "let", "name": name,
                     "annotation_span": (offset + start, offset + start + len(annotation))}
        if expression_kind is None:
            arguments.update(raw_annotation=annotation, wanted_kinds=tuple(self.bindable))
            message = "`%s` is no kind a let may bind" % annotation
        else:
            arguments.update(declared_kind=annotation, expression_kind=expression_kind,
                             wanted_kinds=(annotation,))
            message = "`%s` is declared %s and the expression is %s" % (name, annotation, expression_kind)
        if ordinal is not None:
            arguments["statement_index"] = ordinal
        return FErr("formula_dimension", message, arguments)

    def syntax_statement(self, src, *, literal_inputs=True):
        """Parse a header and all operands without inspecting declarations or values."""
        parse = self.parse if literal_inputs else self._parse_syntax
        self.src = src
        toks, starts = self.tokenize(src)
        self.t, self.starts, self.i = toks, starts, 0
        head = self.peek()[1]
        if head == "let":
            self.take()
            name = self._want_identifier()
            self._want_op(":")
            kind = self._want("id")
            if kind not in self.bindable:
                raise self._binding_dimension(src, name, kind)
            self._want_op("=")
            return ("let", name, kind, parse(self._rest()))
        if head == "assert":
            self.take()
            name = self._want_identifier()
            self._want_op(":")
            tol_name = self._want("id")
            if tol_name not in {"eps_num", "eps_geo", "eps_fmt", "eps_imp", "eps_phys"}:
                raise FErr("formula_parse", "`%s` is not a TOLERANCE token" % tol_name)
            self._want_op("=")
            parts = self._split_top_level(self._rest())
            if len(parts) != 2:
                raise FErr("formula_parse", "an assert compares exactly two expressions")
            left, right = parse(parts[0]), parse(parts[1])
            return ("assert", name, tol_name, left, right)
        return ("expr", None, None, parse(src))

    def static_statement(self, src, env):
        """Inspect names, headers and kinds only; never observe state or compute a value."""
        return self._static_statement(src, env)

    def _syntax_statement_at(self, src, ordinal=None, offset=0, *, literal_inputs=True):
        """Validate original input and retain only actual enclosing source context."""
        try:
            checked = self.syntax_statement(src) if literal_inputs else self.syntax_statement(src, literal_inputs=False)
        except FErr as error:
            if error.arguments.get("diagnostic_scope") != "binding_annotation":
                raise
            arguments = dict(error.arguments)
            arguments["annotation_span"] = tuple(offset + index for index in arguments["annotation_span"])
            if ordinal is not None:
                arguments["statement_index"] = ordinal
            raise FErr(error.token, error.msg, arguments) from error
        return checked

    def _literal_statement_at(self, src, checked, ordinal=None, offset=0):
        """Convert checked raw syntax once; preserve original header and enclosing context."""
        return checked[:3] + tuple(self._normalize_input(node) for node in checked[3:])

    def _static_statement(self, src, env, ordinal=None, offset=0, prior_sources=None, checked=None):
        """Whole preflight supplies completed input; detached checking validates its own."""
        env = self.namespace(env.items())
        if checked is None:
            checked = self._syntax_statement_at(src, ordinal, offset)
        role, name = checked[:2]
        if role == "let":
            _, _, kind, node = checked
            if name in self.reserved:
                raise FErr("formula_rebinding", "reserved `%s` cannot be rebound" % name,
                           {"name": name, "reason": "reserved_name",
                            "reserved_source": self._reserved_source(name),
                            "attempted_source": self._recipe_source(src, name, kind, ordinal, offset)})
            if name in env:
                if env[name]["origin"] == "recipe":
                    prior = (prior_sources or {}).get(name)
                    arguments = {"name": name, "reason": "recipe_name",
                                 "prior_source": dict(prior) if prior is not None else
                                 {"role": "recipe", "kind": env[name]["kind"], "origin": "recipe"},
                                 "attempted_source": self._recipe_source(src, name, kind, ordinal, offset)}
                    if prior is not None:
                        arguments["prior_statement_index"] = prior["statement_index"]
                    if ordinal is not None:
                        arguments["statement_index"] = ordinal
                    raise FErr("formula_rebinding", "`%s` is bound twice in one recipe" % name, arguments)
                raise FErr("formula_ambiguous_name", "`%s` is declared by both %s and recipe"
                           % (name, env[name]["origin"]),
                           self._ambiguity_arguments(name,
                               self._initial_source(env[name]["kind"], env[name]["origin"]),
                               self._recipe_source(src, name, kind, ordinal, offset)))
            got = self.infer(node, env)
            if got != kind:
                raise self._binding_dimension(src, name, kind, got, ordinal, offset)
            return checked
        if role == "assert":
            _, _, tol_name, left, right = checked
            self.infer(("cmp", "==", left, right), env)
            return checked
        _, _, _, node = checked
        return ("expr", None, self.infer(node, env), node)

    def preflight(self, src, declarations):
        """Check the complete ordered source using metadata only; publish no partial plan."""
        env = self.namespace(declarations)
        toks, starts = self.tokenize(src)
        if not toks:
            return ()
        if toks[0][1] not in {"let", "assert"}:
            raise FErr("formula_parse", "a recipe contains only let/assert statements")
        boundaries, depth = [], 0
        for (_, text), start in zip(toks, starts):
            if depth == 0 and text in {"let", "assert"}:
                boundaries.append(start)
            if text == "(":
                depth += 1
            elif text == ")":
                depth = max(0, depth - 1)
        ends = boundaries[1:] + [len(src)]
        parsed = []
        for ordinal, (start, end) in enumerate(zip(boundaries, ends), 1):
            if ordinal > self.limits["max_recipe_statements"]:
                raise FErr("formula_domain", "recipe statement %d exceeds max_recipe_statements=%d"
                           % (ordinal, self.limits["max_recipe_statements"]))
            checked = self._syntax_statement_at(src[start:end], ordinal, start, literal_inputs=False)
            if checked[0] != "let" and checked[0] != "assert":
                raise FErr("formula_parse", "a recipe contains only let/assert statements")
            parsed.append((start, end, checked))
        inputs = []
        for ordinal, (start, end, checked) in enumerate(parsed, 1):
            checked = self._literal_statement_at(src[start:end], checked, ordinal, start)
            inputs.append((start, end, checked))
        plan, prior_sources = [], {}
        for ordinal, (start, end, checked) in enumerate(inputs, 1):
            checked = self._static_statement(src[start:end], env, ordinal, start, prior_sources, checked=checked)
            if checked[0] == "let":
                env[checked[1]] = {"kind": checked[2], "origin": "recipe"}
                prior_sources[checked[1]] = self._recipe_source(src[start:end], checked[1], checked[2], ordinal, start)
            plan.append((start, end, checked))
        return tuple(plan)

    # -- runtime statement adapter; existing tuple forms remain stable --
    def statement(self, src, env):
        checked = self.static_statement(src, env)
        role, name = checked[:2]
        if role == "let":
            _, _, kind, node = checked
            val = self.evaluate(node, env)
            val = self.stored(val, "let %s binding" % name)
            return ("let", name, kind, val, node)
        if role == "assert":
            _, _, tol_name, left, right = checked
            a = self.evaluate(left, env); b = self.evaluate(right, env)
            tol = self.evaluate(("name", tol_name), env)
            self.tolerance_class(name, tol_name, a, b)
            if abs(a.v - b.v) > tol.v:
                raise FErr("formula_assertion", "assert `%s`: %s != %s at `%s` (%s)"
                           % (name, a, b, tol_name, tol.v),
                           {"assertion_name": name, "left_kind": a.kind, "left_value": a.v,
                            "right_kind": b.kind, "right_value": b.v,
                            "tolerance_class": tol_name, "tolerance_value": tol.v})
            return ("assert", name, True, a, b)
        _, _, kind, node = checked
        val = self.evaluate(node, env)
        return ("expr", None, val.kind, val, node)

    def _want(self, kind):
        k, text = self.peek()
        if k != kind:
            raise FErr("formula_parse", "expected %s and found %r" % (kind, text))
        self.take(); return text

    def _want_identifier(self):
        return self._identifier(self._want("id"))

    def _want_op(self, op):
        text = self.peek()[1]
        if text != op:
            raise FErr("formula_parse", "expected %r and found %r" % (op, text))
        self.take()

    def _rest(self):
        """The unconsumed tail of the SOURCE, so spacing (and the one-space unit rule) survives."""
        if self.i >= len(self.starts):
            return ""
        out = self.src[self.starts[self.i]:]
        self.i = len(self.t)
        return out

    def _split_top_level(self, src):
        toks, _ = self.tokenize(src)
        depth, parts, cur = 0, [], []
        for kind, text in toks:
            if text == "(": depth += 1
            elif text == ")": depth -= 1
            if kind == "cmp" and text == "==" and depth == 0:
                parts.append(" ".join(t[1] for t in cur)); cur = []; continue
            cur.append((kind, text))
        parts.append(" ".join(t[1] for t in cur))
        return parts

def d_sqrt_ratio(internal):
    return dfraction(to_true("ratio", internal)).sqrt()

def first_span(cell):
    spans = code_spans(cell)
    return spans[0] if spans else debacktick(cell)

def parse_quantity(cell, units):
    """`74.0 cm`, `1 µm — the T1 class`, `5.0 cm` → (kind, internal integer) by the chapter's unit table."""
    s = debacktick(cell).replace("−", "-").replace("µm", "um").replace("°", "deg")
    s = s.replace("%", "pct").replace('"', "in")
    s = re.split(r"\s+—\s+", s)[0].strip()          # a value cell may carry prose after an em dash
    m = re.match(r"^([-+]?\d+(?:\.\d+)?)\s*([A-Za-z]+)?$", s)
    if not m: return None
    number = Fraction(Decimal(m.group(1)))
    unit = m.group(2)
    if unit is None:
        return ("ratio", rnd(from_true("ratio", number)))
    if unit not in units: return None
    ukind, factor = units[unit]
    return (ukind, rnd(number * factor))

def parse_ratio_literal(cell):
    """`1/4` → a Fraction, for a fixture constant declared as a fraction."""
    s = debacktick(cell).strip()
    m = re.fullmatch(r"([-+]?\d+(?:\.\d+)?)\s*/\s*(\d+(?:\.\d+)?)", s)
    if not m: return None
    return Fraction(Decimal(m.group(1))) / Fraction(Decimal(m.group(2)))

def scalar_domains(book):
    """Consume the declared scalar domains; the piece-box row requires geometry context."""
    unit_sections = sections(str(pathlib.Path(book) / "spec/units-and-tolerances.md"))
    domains = {}
    digits = str.maketrans("⁰¹²³⁴⁵⁶⁷⁸⁹", "0123456789")
    for row in table_in(unit_sections["1.1"], "Quantity")[1]:
        kind = "length" if row[0] == "a coordinate or a length" else "area" if row[0].startswith("an area") else None
        if kind is None:
            continue
        match = re.fullmatch(r"\|v\| ≤ 10([⁰¹²³⁴⁵⁶⁷⁸⁹]+) µm(²)?", row[1])
        if match is None or bool(match[2]) != (kind == "area") or kind in domains:
            raise ValueError("units §1.1: invalid or duplicate %s scalar domain" % kind)
        bound = 10 ** int(match[1].translate(digits))
        domains[kind] = (-bound, bound)
    if set(domains) != {"length", "area"}:
        raise ValueError("units §1.1: missing length/area scalar domain")
    kinds = sections(str(pathlib.Path(book) / "spec/formula-language.md"))
    count = [row for row in table_in(kinds["2"], "Kind")[1] if debacktick(row[0]) == "count"]
    if len(count) != 1 or "never negative" not in count[0][1]:
        raise ValueError("formula §2: missing nonnegative Count contract")
    domains["count"] = (0, None)
    return domains

def storage_domains(book):
    """Consume the numeric binding representations declared in formula §2."""
    kinds = sections(str(pathlib.Path(book) / "spec/formula-language.md"))
    storage = {}
    for row in table_in(kinds["2"], "Kind")[1]:
        kind = debacktick(row[0])
        if kind not in ARITH:
            continue
        match = re.search(r"\bi([0-9]+)\b", row[1])
        if match is None or kind in storage or int(match[1]) < 2:
            raise ValueError("formula §2: invalid or duplicate %s binding storage" % kind)
        bits = int(match[1])
        low = 0 if kind == "count" else -(1 << (bits - 1))
        storage[kind] = (bits, low, (1 << (bits - 1)) - 1)
    if set(storage) != set(ARITH):
        raise ValueError("formula §2: missing numeric binding storage")
    return storage

print("=== formula-language census ===")

# ── read the chapter's own tables ─────────────────────────────────────────────────────────
CON, GRA, EXA, FIX = sections(CONTRACT), sections(GRAMMAR), sections(EXAMPLES), sections(FIXTURE)
for name, sec, need in (("the contract", CON, ("2", "3", "3.1", "4.3", "5.2", "5.3", "7")),
                        ("the grammar", GRA, ("1.1", "2", "2.1", "3", "5.1", "6", "6.1")),
                        ("the examples", EXA, ("1", "2", "3", "4")),
                        ("the fixture", FIX, ("2", "3", "4", "7"))):
    missing = [c for c in need if c not in sec]
    if missing:
        print("formula-language census: REFUSED — %s carries no §%s" % (name, ", §".join(missing)))
        sys.exit(2)

# contract §2: the kinds, and which of them a `let` may bind
kinds, bindable = [], []
for r in table_in(CON["2"], "Kind")[1]:
    if len(r) >= 4:
        kinds.append(debacktick(r[0]))
        if r[3].strip().lower() == "yes": bindable.append(debacktick(r[0]))

# grammar §2.1: the unit tokens and their exact ratios
units = {}
for r in table_in(GRA["2.1"], "Unit token")[1]:
    if len(r) < 3: continue
    token, ukind = debacktick(r[0]), r[1].strip()
    m = re.match(r"^([×÷])\s*([\d\s]+)$", debacktick(r[2]).strip())
    if not m:
        bad("grammar §2.1: `%s` declares no exact ratio in %r" % (token, r[2])); continue
    number = Fraction(int(m.group(2).replace(" ", "").replace("\u2009", "")))
    true_factor = number if m.group(1) == "×" else 1 / number
    units[token] = (ukind, true_factor * (RATIO_SCALE if ukind == "ratio" else 1))

# contract §3: closed declaration origins
origins = [debacktick(row[0]) for row in table_in(CON["3"], "Origin")[1] if row]

# contract §3.1: reserved names, which of them are always bound, and their values
reserved = {}
for r in table_in(CON["3.1"], "Name")[1]:
    if len(r) < 4: continue
    name, kind = debacktick(r[0]), r[1].strip()
    always = r[3].strip().lower() == "always"
    q = parse_quantity(r[2], units)
    reserved[name] = (kind, always, q[1] if (always and q) else None)
    if always and q is None:
        bad("contract §3.1: `%s` is always bound but its value cell %r carries no quantity"
            % (name, r[2]))

# contract §4.3: the structural limits
limits = {}
for r in table_in(CON["4.3"], "Limit")[1]:
    if len(r) >= 2 and r[1].strip().isdigit():
        limits[debacktick(r[0])] = int(r[1].strip())
for need in ("max_expression_nodes", "max_recipe_statements", "max_if_depth", "max_rational_bits"):
    if need not in limits:
        bad("contract §4.3 declares no `%s`" % need)
        limits[need] = 1 << 30

# contract §5.2 / §5.3: the diagnostic set and the envelope's precedence
diagnostics = {debacktick(r[0]) for r in table_in(CON["5.2"], "Token")[1] if r}
envelope = {}
for r in table_in(CON["5.3"], "Construct asked for")[1]:
    if len(r) < 2: continue
    for construct in re.findall(r"`([a-z_]+)`", r[0]):
        envelope[construct] = debacktick(r[1])

# grammar §5.1: the product and quotient table
pairs = {}
for r in table_in(GRA["5.1"], "Left")[1]:
    if len(r) < 4: continue
    left, right = debacktick(r[0]), debacktick(r[1])
    prod = None if r[2].strip() == "—" else debacktick(r[2])
    quot = None if r[3].strip() == "—" else debacktick(r[3])
    pairs[(left, right)] = {"*": prod, "/": quot}
    if prod: pairs.setdefault((right, left), {})["*"] = prod

# grammar §5: diagnostic signatures, retaining the chapter's exact symbolic requirements.
operator_sigs = {}
for row in table_in(GRA["5"], "Operator")[1]:
    if len(row) < 3: continue
    for spelling in re.findall(r"`([^`]+)`", row[0]):
        operation = spelling.replace(" ", "")
        if operation in ("*", "/"):
            rows = [(list(pair), False, rules[operation]) for pair, rules in pairs.items()
                    if rules.get(operation) is not None]
            operator_sigs[operation, 2] = rows
        else:
            args = [part.strip() for part in debacktick(row[1]).split(",")]
            result = debacktick(row[2]).strip()
            operator_sigs.setdefault((operation, len(args)), []).append((args, False, result))

# grammar §6 and §6.1: the function and selector signatures
sigs = {}
def read_sigs(section, header):
    for r in table_in(GRA[section], header)[1]:
        if len(r) < 3: continue
        names = re.findall(r"`([A-Za-z_][A-Za-z0-9_]*)`", r[0])
        arg_cell = debacktick(r[1])
        variadic = "…" in arg_cell
        arg_kinds = [a.strip() for a in arg_cell.replace("…", "").split(",") if a.strip()]
        result = debacktick(r[2]).strip()
        for n in names:
            sigs.setdefault(n, []).append((arg_kinds, variadic, result))
read_sigs("6", "Function")
read_sigs("6.1", "Selector")
declared_functions = set(sigs)

try:
    domains = scalar_domains(BOOK)
    storage = storage_domains(BOOK)
except (OSError, ValueError, KeyError) as error:
    print("formula-language census: REFUSED — numeric domain source: %s" % error)
    sys.exit(2)
EV = Evaluator(bindable, pairs, sigs, reserved, envelope, limits, units, domains, storage, origins,
               operator_signatures=operator_sigs)
print("-- tables read from the chapter")
print("  kinds: %d (%d bindable) · unit tokens: %d · reserved: %d · signature rows: %d"
      % (len(kinds), len(bindable), len(units), len(reserved), sum(len(v) for v in sigs.values())))
print("  product/quotient pairs: %d · diagnostics: %d · envelope constructs: %d · limits: %d"
      % (len(pairs), len(diagnostics), len(envelope), len(limits)))

# ── the symbol table: the fixture's declared constants, nothing derived ─────────────────────
env = {}
def bind(name, kind, value, origin, state="known", lazy=False, exprs=None):
    entry = {"kind": kind, "value": value, "origin": origin, "state": state,
             "lazy": lazy, "exprs": exprs or {}}
    checked = EV.namespace([*env.items(), (name, entry)])
    env.update(checked)

fixture_units_ok = True
for sec, header, col, origin in (("2", "Token", 3, "measurement"), ("2", "Ease token", 3, "ease"),
                                 ("3", "Token", 1, "parameter"), ("7", "Token", 2, "profile")):
    for r in table_in(FIX[sec], header)[1]:
        if len(r) <= col: continue
        token = debacktick(r[0])
        if token == "—" or not token: continue
        q = parse_quantity(r[col], units)
        frac = parse_ratio_literal(r[col])
        if q:
            bind(token, q[0], q[1], origin)
        elif frac is not None:
            bind(token, "ratio", rnd(from_true("ratio", frac)), origin)
        else:
            bad("fixture §%s: `%s` has no parseable value in %r" % (sec, token, r[col]))
            fixture_units_ok = False

fixture_derived = {}
for r in table_in(FIX["4"], "Token")[1]:
    if len(r) < 4: continue
    token, result = debacktick(r[0]), debacktick(r[3])
    if "=" in result: continue          # a closure row publishes two sides, not a value
    q = parse_quantity(result, units)
    if q: fixture_derived[token] = q

# examples §1: the names this chapter adds
for r in table_in(EXA["1"], "Name")[1]:
    if len(r) < 5: continue
    name, kind, origin = debacktick(r[0]), r[1].strip(), debacktick(r[2])
    state_m = re.search(r"`(assumed|unknown|derived|known|preference)`", r[4])
    state = state_m.group(1) if state_m else "known"
    cell = r[3]
    if kind == "edge":
        m = re.search(r"`len = ([^`]+)`", cell)
        if not m:
            bad("examples §1: `%s` is an edge with no `len = …` binding" % name); continue
        bind(name, "edge", None, origin, state, lazy=True, exprs={"len": m.group(1)})
    elif kind == "point":
        xs = re.search(r"`x = ([^`]+)`", cell); ys = re.search(r"`y = ([^`]+)`", cell)
        if not xs or not ys:
            bad("examples §1: `%s` is a point with no `x = …` / `y = …` binding" % name); continue
        bind(name, "point", None, origin, state, lazy=True, exprs={"x": xs.group(1), "y": ys.group(1)})
    elif state == "unknown":
        bind(name, kind, None, origin, state)
    else:
        q = parse_quantity(cell, units)
        if not q:
            bad("examples §1: `%s` has no parseable binding in %r" % (name, cell)); continue
        if q[0] != kind:
            bad("examples §1: `%s` is declared %s and its binding is %s" % (name, kind, q[0]))
        bind(name, kind, q[1], origin, state)

print("  symbols: %d from the fixture, %d from examples §1"
      % (len([k for k, v in env.items() if v["origin"] != "geometry" or not v["lazy"]]),
         len(table_in(EXA["1"], "Name")[1])))

# ── L1: the literal table canonicalizes as it says ────────────────────────────────────────
print("-- L1 literal rows canonicalize through the unit table")
l1 = 0
for r in table_in(GRA["2"], "Literal")[1]:
    if len(r) < 3: continue
    literal, kind, canonical = debacktick(r[0]), r[1].strip(), debacktick(r[2])
    m = re.fullmatch(r"([a-z]+):(-?\d+)", canonical)
    if not m:
        bad("grammar §2: `%s` publishes canonical form %r, which is not kind:integer" % (literal, canonical))
        l1 += 1; continue
    try:
        node = EV.parse(literal)
    except FErr as e:
        bad("grammar §2: `%s` does not parse — %s" % (literal, e.msg)); l1 += 1; continue
    if node[0] != "lit":
        bad("grammar §2: `%s` is not a literal" % literal); l1 += 1; continue
    if node[1] != kind or node[1] != m.group(1):
        bad("grammar §2: `%s` is declared %s, canonicalizes to %s and publishes %s"
            % (literal, kind, node[1], m.group(1)))
        l1 += 1; continue
    if node[2] != int(m.group(2)):
        bad("grammar §2: `%s` canonicalizes to %s:%s, the chapter publishes %s"
            % (literal, node[1], node[2], canonical))
        l1 += 1
print("  literal rows: %d · mismatches: %d" % (len(table_in(GRA["2"], "Literal")[1]), l1))

# Check the ENTIRE worked recipe before its first statement can execute. Assertion labels
# introduce no bindings; the namespace used by preflight contains only kinds and origins.
bind_rows = table_in(EXA["2"], "Token")[1]
asserts = table_in(EXA["3"], "Assertion")[1]
worked_sources = []
for r in bind_rows:
    if len(r) < 4:
        bad("examples §2: malformed worked binding row before preflight")
        sys.exit(1)
    worked_sources.append("let %s: %s = %s" % (debacktick(r[0]), r[1].strip(), first_span(r[2])))
for r in asserts:
    if len(r) < 3:
        bad("examples §3: malformed worked assertion row before preflight")
        sys.exit(1)
    worked_sources.append(first_span(r[0]))
worked_source = "\n".join(worked_sources)
print("-- whole worked recipe static preflight (no statement execution)")
try:
    worked_plan = EV.preflight(worked_source, env.items())
except FErr as e:
    bad("worked recipe preflight: %s — %s" % (e.token, e.msg))
    print("formula-language census: REFUSED — worked recipe static error; no statement executed")
    sys.exit(1)
print("  statically accepted statements: %d" % len(worked_plan))

# ── L2 + L3: the bindings, in declaration order, against both chapters ────────────────────
print("-- L2 bindings evaluate / L3 they agree with the fixture")
computed, shared, l2, l3 = [], 0, 0, 0
for r in bind_rows:
    if len(r) < 4:
        bad("examples §2: a row has %d cells, the table declares 5: %r" % (len(r), r)); l2 += 1; continue
    token, kind, expr_cell, value_cell = debacktick(r[0]), r[1].strip(), r[2], r[3]
    src = "let %s: %s = %s" % (token, kind, first_span(expr_cell))
    try:
        stmt = EV.statement(src, env)
    except FErr as e:
        bad("examples §2 `%s`: %s — %s" % (token, e.token, e.msg)); l2 += 1; continue
    _, name, declared, val, _ = stmt
    if val.kind not in EV.bindable:
        bad("examples §2 `%s`: a let bound a %s, which §2 says is not bindable" % (token, val.kind))
        l2 += 1; continue
    if val.kind == "point" or isinstance(val.v, tuple):
        bad("examples §2 `%s`: a let bound a point" % token); l2 += 1; continue
    if val.v.denominator != 1:
        bad("examples §2 `%s`: a binding returned a noninteger" % token); l2 += 1; continue
    internal = val.v.numerator
    env[token] = {"kind": val.kind, "value": internal, "origin": "recipe", "state": "derived"}
    env[token]["sources"] = val.sources
    published = debacktick(value_cell).strip()
    if val.kind == "boolean":
        if published not in ("true", "false"):
            bad("examples §2 `%s`: value cell %r is not true or false"
                % (token, value_cell)); l2 += 1; continue
        dp, expected = 0, published
    else:
        m = re.fullmatch(r"([-+]?\d+(?:\.\d+)?)(?:\s*(cm²|cm|deg))?", published)
        if not m:
            bad("examples §2 `%s`: value cell %r is not a number with an optional unit"
                % (token, value_cell)); l2 += 1; continue
        dp = len(m.group(1).split(".")[1]) if "." in m.group(1) else 0
        unit = m.group(2)
        want_kind = {"cm": "length", "deg": "angle", "cm²": "area", None: val.kind}[unit]
        if want_kind != val.kind:
            bad("examples §2 `%s`: publishes %s but the expression is %s" % (token, unit or val.kind, val.kind))
            l2 += 1; continue
        expected = m.group(1)
    try:
        got = render(val.kind, internal, dp)
    except FErr as e:
        bad("examples §2 `%s`: %s — %s" % (token, e.token, e.msg)); l2 += 1; continue
    computed.append((token, val.kind, got, expected))
    if got != expected:
        bad("examples §2 `%s`: %s computes to %s, the chapter publishes %s"
            % (token, first_span(expr_cell), got, expected)); l2 += 1
    if token in fixture_derived:
        shared += 1
        fkind, fval = fixture_derived[token]
        if fkind != val.kind or fval != internal:
            bad("L3 `%s`: this chapter computes %s %s and the fixture chapter publishes %s %s — "
                "two chapters, two garments (the D27 class)"
                % (token, got, val.kind, render(fkind, fval, dp), fkind)); l3 += 1
print("  bindings: %d · mismatches: %d · names shared with the fixture: %d · disagreements: %d"
      % (len(bind_rows), l2, shared, l3))

# ── L4: the assertions ────────────────────────────────────────────────────────────────────
print("-- L4 assertions hold at the class they name")
l4 = 0
for r in asserts:
    if len(r) < 3:
        bad("examples §3: a row has %d cells, the table declares 3" % len(r)); l4 += 1; continue
    src, verdict = first_span(r[0]), r[2]
    try:
        stmt = EV.statement(src, env)
    except FErr as e:
        bad("examples §3: %s — %s" % (e.token, e.msg)); l4 += 1; continue
    _, name, holds, a, b = stmt
    says_holds = verdict.strip().lower().startswith("holds")
    if holds != says_holds:
        bad("examples §3 `%s`: the run says %s and the verdict cell says %r"
            % (name, "holds" if holds else "does not hold", verdict)); l4 += 1
    if not holds:
        bad("examples §3 `%s`: %s != %s at %s" % (name, a.v, b.v, "its class")); l4 += 1
    else:
        print("  assertion %-26s %s = %s  HOLDS" % (name, a.v, b.v))
print("  assertions: %d · failures: %d" % (len(asserts), l4))

# ── L5: the refusals raise what they say ──────────────────────────────────────────────────
print("-- L5 refusals raise the diagnostic they name")
l5, refusals = 0, table_in(EXA["4"], "Refusal")[1]
for r in refusals:
    if len(r) < 2:
        bad("examples §4: a row has %d cells, the table declares 3" % len(r)); l5 += 1; continue
    src, want = first_span(r[0]), debacktick(r[1])
    try:
        EV.statement(src, dict(env))
    except FErr as e:
        if e.token != want:
            bad("examples §4: %r raises %s, the row names %s" % (src, e.token, want)); l5 += 1
        continue
    bad("examples §4: %r raises nothing, the row names %s" % (src, want)); l5 += 1
print("  refusals: %d · wrong or missing: %d" % (len(refusals), l5))

# ── L6: the vocabulary is closed ──────────────────────────────────────────────────────────
print("-- L6 the vocabulary is declared, implemented and closed")
parts = [CONTRACT, GRAMMAR, EXAMPLES]
l6_start = fails
l6 = 0
# (a) every call is declared
callable_names = declared_functions | {"if"} | set(envelope)
for p in parts:
    text = read(p)
    for span in code_spans(text):
        for name in re.findall(r"([A-Za-z_][A-Za-z0-9_]*)\s*\(", span):
            if name not in callable_names:
                bad("L6a %s: `%s(` is no declared function, selector, special form or envelope "
                    "construct" % (pathlib.Path(p).name, name)); l6 += 1
# (b) the operator alphabet. Two populations, because a span whose ONLY operator is an invented one
# carries no declared operator to be found by: every span in the three parts (their expression columns
# are formula positions by construction), and every span anywhere in the book that carries a declared
# formula operator (a display-form formula in another chapter). The first arm is what makes the second
# honest — measured by the OPERATOR probe, which replaces `+` with `⊕` and must still be refused.
display_chars = set()
for r in table_in(GRA["3"], "Machine")[1]:
    for cell in (r[1] if len(r) > 1 else "", r[3] if len(r) > 3 else ""):
        display_chars |= set(debacktick(cell))
alphabet = set("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_ \t(),.+-.*/^=<>!:\"[]…") \
    | display_chars
formula_ops = set("+=*^<>−×÷√≤≥≠")
population = []
for p in parts:
    population += [(pathlib.Path(p).name, s) for s in code_spans(read(p))]
for f in sorted(pathlib.Path(BOOK).rglob("*.md")):
    if str(f) in {str(pathlib.Path(p)) for p in parts}:
        continue
    for span in code_spans(f.read_text(encoding="utf-8"), allow_foreign=True):
        if set(span) & formula_ops and re.search(r"[A-Za-z0-9]", span):
            population.append((f.name, span))
for fname, span in population:
    for ch in sorted(set(span) - alphabet):
        bad("L6b %s: the formula span %r uses %r, which the grammar and the display table do "
            "not declare" % (fname, span[:48], ch)); l6 += 1
# (c) the evaluator implements exactly what the chapter declares
missing = declared_functions - IMPLEMENTED
extra = IMPLEMENTED - declared_functions
for name in sorted(missing):
    bad("L6c the chapter declares `%s` and this evaluator does not implement it" % name); l6 += 1
for name in sorted(extra):
    bad("L6c this evaluator implements `%s` and the chapter does not declare it" % name); l6 += 1
# (d) every diagnostic token the parts name is declared
declared_tokens = diagnostics | set(envelope.values())
for p in parts:
    for name in sorted({s for s in code_spans(read(p)) if re.fullmatch(r"(formula|env)_[a-z_]+", s)}):
        if name not in declared_tokens:
            bad("L6d %s names `%s`, which neither contract §5.2 nor §5.3 declares"
                % (pathlib.Path(p).name, name)); l6 += 1
l6 = fails - l6_start
print("  calls checked · %d formula spans over an alphabet of %d characters · declared functions %d · "
      "implemented %d · diagnostics %d · breaches: %d"
      % (len(population), len(alphabet), len(declared_functions), len(IMPLEMENTED),
         len(declared_tokens), l6))

# ── L7: references and the parts table ────────────────────────────────────────────────────
print("-- L7 links resolve, clauses exist, the parts table is complete")
l7 = 0
def resolve(base_dir, target):
    absolute = base_dir.startswith("/")
    segs = []
    for s in (base_dir + "/" + target).split("/"):
        if s in ("", "."): continue
        if s == "..":
            if segs: segs.pop()
            continue
        segs.append(s)
    out = "/".join(segs)
    return "/" + out if absolute else out

for p in parts:
    base = str(pathlib.Path(p).parent)
    for text, target in re.findall(r"\[([^\]]*)\]\(([^)]+)\)", read(p)):
        if target.startswith(("http://", "https://", "#")): continue
        path = resolve(base, target.split("#")[0])
        if not pathlib.Path(path).is_file():
            bad("L7 %s links to %s, which does not exist" % (pathlib.Path(p).name, target)); l7 += 1
            continue
        m = re.search(r"§(\d+(?:\.\d+)*)", text)
        if m and m.group(1) not in headings(path):
            bad("L7 %s cites §%s, which %s has no heading for"
                % (pathlib.Path(p).name, m.group(1), target)); l7 += 1
listed = set()
for r in table_in(CON["7"], "Part")[1]:
    for target in re.findall(r"\]\(([^)]+)\)", r[0]):
        listed.add(resolve(str(pathlib.Path(CONTRACT).parent), target))
actual = {str(q) for q in sorted(pathlib.Path(PARTS_DIR).glob("*.md"))}
for extra_part in sorted(actual - listed):
    bad("L7 %s exists but the contract's §7 parts table does not list it" % extra_part); l7 += 1
for gone in sorted(listed - actual):
    bad("L7 the parts table lists %s, which does not exist" % gone); l7 += 1
print("  links checked in %d parts · listed parts %d · files in the directory %d · breaches: %d"
      % (len(parts), len(listed), len(actual), l7))

# ── L8: the structural limits are derived ─────────────────────────────────────────────────
print("-- L8 the structural limits exceed what the book measures")
l8 = 0
statements = len(worked_plan)
if EV.max_nodes * NODE_MARGIN > limits["max_expression_nodes"]:
    bad("L8 the largest expression measured %d nodes and `max_expression_nodes` is %d — under the "
        "%dx margin the decision record states" % (EV.max_nodes, limits["max_expression_nodes"],
                                                   NODE_MARGIN)); l8 += 1
if statements > limits["max_recipe_statements"]:
    bad("L8 the worked recipe carries %d statements and `max_recipe_statements` is %d"
        % (statements, limits["max_recipe_statements"])); l8 += 1
if EV.max_if > limits["max_if_depth"]:
    bad("L8 a conditional nests %d deep and `max_if_depth` is %d"
        % (EV.max_if, limits["max_if_depth"])); l8 += 1
if EV.max_bits > limits["max_rational_bits"]:
    bad("L8 an exact value needed %d bits and `max_rational_bits` is %d"
        % (EV.max_bits, limits["max_rational_bits"])); l8 += 1
print("  measured: %d nodes max · %d statements · %d deep · %d bits — declared: %d / %d / %d / %d"
      % (EV.max_nodes, statements, EV.max_if, EV.max_bits, limits["max_expression_nodes"],
         limits["max_recipe_statements"], limits["max_if_depth"], limits["max_rational_bits"]))

print("-- A1 advisory: every binding as this run computed it")
for token, kind, got, published in computed:
    flag = "" if got == published else "   <-- the chapter publishes %s" % published
    print("  · %-26s %-8s %s%s" % (token, kind, got, flag))

mismatches = l1 + l2 + l3 + l4 + l5 + l6 + l7 + l8
print("formula-language census: %d bindings / %d assertions / %d refusals / %d mismatch(es)"
      % (len(bind_rows), len(asserts), len(refusals), mismatches))
sys.exit(1 if mismatches else 0)
PY
rc=$?
exit "$rc"
