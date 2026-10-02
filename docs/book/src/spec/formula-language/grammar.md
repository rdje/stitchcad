# Grammar, operators and functions

> One part of [the formula language](../formula-language.md), which carries the contract: its §2 the
> kinds, §3 the names and their origins, §4 evaluation and rounding, §5 the diagnostics and §6 the
> exclusions — cited below as *the contract*. This part is the syntax, and the tables those rules are
> stated over. The worked examples that execute every rule are [their own part](examples.md).

## 1. The grammar

The machine form is ASCII, and an identifier is lower snake_case — the machine-token rule the
[glossary](../glossary.md) states.
The shared core MachineToken makes that spelling exact: a lowercase letter starts the token,
letters/digits continue each nonempty segment, and one underscore separates segments. No whitespace,
uppercase, Unicode, leading/trailing underscore or empty segment is accepted or normalized.

```
recipe    := statement*
statement := let | assert
let       := "let" IDENT ":" KIND "=" expr
assert    := "assert" IDENT ":" TOLERANCE "=" expr "==" expr
expr      := cmp
cmp       := add ( CMPOP add )?
add       := mul ( ADDOP mul )*
mul       := unary ( MULOP unary )*
unary     := "-" unary | power
power     := postfix ( "^" "2" )?
postfix   := "if" "(" expr "," expr "," expr ")" | IDENT "(" args ")" | atom
args      := expr ( "," expr )*
atom      := "(" expr ")" | LITERAL | IDENT
LITERAL   := DIGITS ( "." DIGITS )? ( " " UNIT )?
KIND      := "length" | "angle" | "area" | "ratio" | "count" | "boolean"
TOLERANCE := "eps_num" | "eps_geo" | "eps_fmt" | "eps_imp" | "eps_phys"
IDENT     := LOWER ( LOWER | DIGIT )* ( "_" ( LOWER | DIGIT )+ )*
LOWER     := "a" ... "z"       DIGIT := "0" ... "9"
CMPOP     := "==" | "!=" | "<=" | ">=" | "<" | ">"
ADDOP     := "+" | "-"      MULOP := "*" | "/"
UNIT      := "um" | "mm" | "cm" | "m" | "in" | "deg" | "pct"
```

### 1.1 Keywords

Three words are reserved and may not be names:

| Keyword | Where it appears |
| --- | --- |
| `let` | a binding statement — one name to one value |
| `assert` | a closure check at a named tolerance class ([examples §3](examples.md)) |
| `if` | the one special form (§7) |

An `assert` that does not hold refuses the whole recipe: it is the language-level form of the rule
that a fixture must derive its own finished dimensions.

### 1.2 Three ambiguities, settled

`- x ^ 2` is `- (x ^ 2)`, because the power rule binds tighter than unary minus. A comparison does
not chain, so `a < b < c` is `formula_parse`. And `^` accepts the literal `2` and nothing else
([the contract §6](../formula-language.md)).

## 2. Literals and their units

| Literal | Kind | Canonical form | Note |
| --- | --- | --- | --- |
| `4` | count | `count:4` | an integer with no unit is a count |
| `1.05` | ratio | `ratio:1050000` | a decimal with no unit is a scale factor |
| `2.5 cm` | length | `length:25000` | one conversion, one rounding (units §2) |
| `18 in` | length | `length:457200` | × 25 400 exactly |
| `90 deg` | angle | `angle:90000000` | degrees, because the domain speaks degrees |
| `150 pct` | ratio | `ratio:1500000` | a percent is a ratio ÷ 100, never a multiplier |

A literal's unit is separated by exactly one space. There is no area literal, because an area is
always derived ([units §1.3](../units-and-tolerances.md)); no radian literal, because π is
irrational and a radian angle could not be stored exactly — one enters only as the result of
`atan2` or `atan`; and no exponent notation. A fractional inch is presentation: the input parser
canonicalizes `1/2 in` to `0.5 in`, because a bare `/` is division and `1 / 2 in` is a count divided
by a length.

A **scale factor is a `ratio` and a repetition is a `count`**, and the kinds do not make them
interchangeable in meaning: `2 * wb_width` is two layers of a band, while `wb_width * 50` is fifty
bands and not "50 % more". The canonical form keeps the kind (§4), so a reviewer and a diff see
`count:50` where a percentage was meant.

### 2.1 Unit tokens

Closed, and each carries the exact ratio the [units chapter](../units-and-tolerances.md) §2
declares, so a literal converts once and rounds once:

| Unit token | Kind | To the internal unit | Displayed as |
| --- | --- | --- | --- |
| `um` | length | × 1 | µm |
| `mm` | length | × 1 000 | mm |
| `cm` | length | × 10 000 | cm |
| `m` | length | × 1 000 000 | m |
| `in` | length | × 25 400 | in, or `"` |
| `deg` | angle | × 1 000 000 | ° |
| `pct` | ratio | ÷ 100 | % |

## 3. The display form

| Machine | Display | Machine | Display |
| --- | --- | --- | --- |
| `-` | `−` | `sqrt(x)` | `√x` |
| `*` | `×` | `x ^ 2` | `x²` |
| `/` | `÷` | `<=` `>=` `!=` | `≤` `≥` `≠` |
| `um` | `µm` | `deg` | `°` |
| `pct` | `%` | `in` | `"` |

**The machine form is the only form a parser accepts**; the display form is produced by a renderer
and is presentation ([units §2.2](../units-and-tolerances.md)), so a locale may change it and may
never change the parsed meaning. A formula written as prose in this book — the fixture's
`√(hip_to_hem_drop² + a_line_flare²)` is one — is a display form, and every display operator it uses
has a machine form above. That closure is derived rather than assumed ([the contract
§9](../formula-language.md)).

## 4. The canonical form

A formula is stored as an S-expression: a node is `(op arg ...)`, a name is the bare identifier, a
literal is `kind:integer` in internal units. One space separates parts; there is no comment, no
trailing space and no float anywhere ([units §7](../units-and-tolerances.md)).

```
let garment_waist: length = waist_girth + ease_waist
    → (bind garment_waist length (+ waist_girth ease_waist))
let side_seam_length: length = hypot(hip_to_hem_drop, a_line_flare)
    → (bind side_seam_length length (hypot hip_to_hem_drop a_line_flare))
```

Canonical literal integers may use the existing 128-bit rational width; i64 is required for bound
numeric values, not for each literal child. Unary minus remains an operator: the lowest signed
microdegree angle uses a positive 2^63 literal below that operator. It can bind to signed i64 MIN
without sign folding. Literal conversion/rounding and scalar domains still apply ([contract §4.2](../formula-language.md)).
The director’s D95 ruling is recorded in `docs/decisions/decision_literals.md`; reference
node-boundary reference verification is recorded in the [expert annex](../../annexes/formula-syntax.md#reference-canonical-literal-width-and-identity-controls). Individual product literals now have
[explicit input normalization](../../annexes/formula-literals.md), including whole normalized syntax
arenas. Canonical serialization remains later work.

**A formula's identity is its canonical form.** `2.5 cm` and `25 mm` canonicalize to one node
(`length:25000`), so they are one formula: a diff, a hash and a golden compare canonical forms and
never spellings. Documentation of a step lives on the drafting operation that consumes the value,
not inside the expression — a comment that vanishes at canonicalization does not survive a save.

## 5. Operators

An operands column lists kinds positionally. Four notations appear in them, and they are grammar
notation rather than machine tokens — which is why they are set in italics and never in code spans,
so no reader and no census mistakes one for a name the language binds:

| Notation | In an operands column it means |
| --- | --- |
| *T* | one *arithmetic* kind — `length`, `angle`, `area`, `ratio` or `count` — bound consistently across the row |
| *N* | one of the four kinds a value may be negated: `length`, `angle`, `area`, `ratio` |
| *…* | variadic, from the number of kinds listed |
| *tolerance* | a reserved tolerance name ([the contract §3.1](../formula-language.md)) |

The tables are regular on purpose: the census [the contract](../formula-language.md) §8 names reads
them and type-checks every worked example with them, so a signature nobody wrote down cannot be
implemented and a signature written down wrongly reddens the run instead of the product.

| Operator | Operands | Result | Refused when |
| --- | --- | --- | --- |
| `+` `-` | T, T | T | the two kinds differ |
| `-` unary | N | N | the operand is a `count`, a `boolean`, a `point` or an `edge` |
| `*` | a pair §5.1 lists | as listed | the pair is absent |
| `/` | a pair §5.1 lists | as listed | the pair is absent, or the divisor is zero |
| `^ 2` | length | area | another exponent |
| `^ 2` | ratio | ratio | another exponent |
| `^ 2` | count | count | another exponent |
| `==` `!=` `<` `<=` `>` `>=` | T, T | boolean | the kinds differ, or the comparison chains |

A refusal is `formula_dimension` unless the row names another token ([the contract
§5.2](../formula-language.md)). A bare comparison is exact on internal integers; a comparison that
means "close enough" is `within` (§6), which names its class.

### 5.1 Products and quotients

Multiplication is commutative, so one row covers both orders; an em dash in a result cell means that
combination is refused.

| Left | Right | `*` gives | `/` gives |
| --- | --- | --- | --- |
| `length` | `length` | `area` | `ratio` |
| `length` | `ratio` | `length` | `length` |
| `length` | `count` | `length` | `length` |
| `angle` | `angle` | — | `ratio` |
| `angle` | `ratio` | `angle` | `angle` |
| `angle` | `count` | `angle` | `angle` |
| `area` | `length` | — | `length` |
| `area` | `ratio` | `area` | `area` |
| `area` | `count` | `area` | `area` |
| `area` | `area` | — | `ratio` |
| `ratio` | `ratio` | `ratio` | `ratio` |
| `ratio` | `count` | `ratio` | `ratio` |
| `count` | `count` | `count` | `ratio` |
| `count` | `ratio` | `ratio` | `count` |

An angle times a length is refused and the diagnostic names `arc_length`: the only meaningful
product of an angle and a radius is an arc's length, and it needs π, so it is a function with a
declared rounding rather than a product the model has no kind for.

## 6. Built-in functions

The set is closed. A name that is not in it, not bound and not reserved is `formula_unbound_name`; a
construct the envelope owns is refused with the envelope's own token ([the contract
§5.3](../formula-language.md)).

| Function | Arguments | Result | Rule |
| --- | --- | --- | --- |
| `sqrt` | area | length | the nearest quantum to the true root, ties away from zero; a negative operand is `formula_domain` |
| `sqrt` | ratio | ratio | the same, at the ratio's quantum |
| `hypot` | length, length | length | `sqrt` of the sum of squares, without materializing the area |
| `abs` | T | T | — |
| `min` | T, … | T | — |
| `max` | T, … | T | — |
| `clamp` | T, T, T | T | low above high is `formula_domain` |
| `round_to` | T, T | T | the nearest multiple of the second argument, ties away from zero; the step is part of the recipe, so a coarser quantum is visible in a diff |
| `sin` `cos` `tan` | angle | ratio | `tan` at an exact odd quarter-turn is `formula_domain` |
| `atan` | ratio | angle | signed principal angle, nearest microdegree; branch rules below |
| `atan2` | length, length | angle | signed principal angle of (y, x), nearest microdegree |
| `atan2` | ratio, ratio | angle | the same (y, x) convention |
| `arc_length` | angle, length | length | the angle turned through π/180 and multiplied by the radius |
| `if` | boolean, T, T | T | the one special form; the third part is mandatory (§7) |
| `within` | T, T, tolerance | boolean | the tolerance comparison; the class is the name ([the contract §3.1](../formula-language.md)), so a comparison always names its class |

Inverse trigonometric functions preserve their signed principal answer: `atan(-1.0)` returns
−45 degrees and `atan2(-1 um, -1 um)` returns −135 degrees. The first argument to atan2 is the
*y* component, the second is *x*; it resolves the quadrant that atan alone cannot determine.
A finite atan input has a true answer in (−90, +90) degrees; nearest-microdegree rounding can
reach either endpoint. Atan2's true branch is (−180, +180] degrees, and its rounded result is in
[−180, +180]. Exact zero has no sign: zero *y* with negative *x* gives +180 degrees, while a
negative *y* very near that axis may round to −180. Both components zero refuse with
`formula_domain`. Neither function applies direction modulo. The dir selector below does.

### 6.1 Geometry selectors

Read-only: a selector reports what an operation constructed and constructs nothing.

| Selector | Arguments | Result | Note |
| --- | --- | --- | --- |
| `x` `y` | point | length | the coordinate in the drafting frame the point was made in |
| `dist` | point, point | length | — |
| `dir` | point, point | angle | from the first to the second, normalized to `[0, 360)` |
| `len` | edge | length | the edge's arc length — the value `walk` reports (ontology §3.2) |
| `param_at` | edge, length | ratio | the parameter at that arc length from the edge's start; a length beyond `len` is `formula_domain` |
| `point_at` | edge, ratio | point | the point at that parameter; a ratio outside `[0, 1]` is `formula_domain` |

A parameter is the [ontology](../ontology.md) §1 rational in `[0, 1]` of an edge's own length, so a
notch placed by `param_at` survives an edit to the edge, a change of tessellation and a change of
units. The ratio's 10⁻⁶ quantum places it within 5 µm on an edge at the declared 10 m bounding-box
limit ([units §1.1](../units-and-tolerances.md)) — inside the T2 class, so a notch by parameter is
finer than the geometry it sits on.

## 7. Conditionals

`if(condition, then, else)` is the only special form, and its third part is mandatory: a missing
alternative would be an invented value, which roadmap §2.6 forbids. The condition must be a
`boolean`; a non-zero length is not true, and using one is `formula_dimension`.

The two halves are treated differently, and the difference matters: **both branches are checked
statically** — kinds, names, structure — so a kind error in an untaken branch is still a refusal,
while **only the taken branch is evaluated**, so an `unknown` name in the untaken one does not block
and cannot. That is what lets one recipe serve a size run where a fact is missing for sizes it does
not reach.

### 7.1 Multi-size branching

The size context supplies `size_index`, `size_count` and `is_base_size` ([the contract
§3.1](../formula-language.md)). A recipe **may not branch on a size label**: a label is prose and
not a token ([size sets](../size-sets.md) §3), and the language has no text values. A branch is on
an ordinal or — preferred — on a measurement, because a measurement is already size-resolved while
an ordinal is a position in an authored order that a re-authored size set changes silently. A branch
on an ordinal says what it means in its name (`dart_count`, never a nameless case).
