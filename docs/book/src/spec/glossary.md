# Glossary

> **Status:** normative vocabulary, gate **G0** (roadmap §11, G0 exit clause "glossary of construction
> terms"). Every other chapter of this book uses these terms with these meanings; where a chapter needs
> a term this glossary does not carry, the chapter is wrong, not the glossary.

One term, one meaning. A garment word travels between a patternmaker, a sewing expert, a factory's
cutting room, three other CAD systems and an AI agent, and each of them has a word for the same thing
and often the same word for a different thing. This glossary is the place where that ambiguity dies:
it names the concept in plain language, binds it to the canonical object that specifies it, lists what
other people call it, and records the **machine token** the software uses for it.

## How to read an entry

Every entry is a row of five cells in one of the parts below:

| Cell | What it carries |
| --- | --- |
| **Term** | the English headword, with a ⚠ when the term is safety-relevant (see below) |
| **What it means** | enough plain language to use the term correctly — not a restatement of the rule |
| **Canonical object** | the chapter and clause that specifies the concept normatively, or the leaf that will |
| **Also called** | synonyms from factory floors, other CAD systems and other languages; `—` when there are none |
| **Machine token** | the identifier the software uses, or `→ token` when another entry owns it, or `—` when the term has none |

Two conventions make the last two columns trustworthy:

- **An entry that owns a token owns it alone.** Where several terms share one identifier — `button`,
  `buttonhole`, `hook and bar` and `zipper` are all a `Closure` — exactly one entry owns the token and
  the others cross-reference it with `→`. A token that two entries owned would be a token with two
  meanings, which is the defect this chapter exists to prevent.
- **A definition never competes with its chapter.** The glossary says what a word means; the canonical
  object says what must be true. If the two disagree, the chapter wins and the glossary entry is a
  defect.

## The machine-token rule

A **machine token** is the identifier a concept carries in the API, in serialized files, in formulas and
in diagnostics: `dart_intake`, `SeamAllowance`, `seam_allowance_policy`. Tokens are the contract with
machines, and this rule governs them:

1. **A token is never rendered raw to a human.** A user sees the localized term; an artifact carries the
   token only where a machine reads it. A token surfacing in a dialog, a label or a printed tech pack is
   a defect, and the internationalization chapter's externalization lint is what catches it
   (`G0-CONTRACT.16`).
2. **A token has exactly one meaning.** The census below derives this rather than asserting it.
3. **Tokens are locale-independent ASCII.** A decimal comma, a translated field name or a case-folded
   identifier must never change what a file means (roadmap §7.6). Value tokens are `snake_case`; object
   tokens are the `CamelCase` type name; enumerated values are `snake_case` or the code the target
   format itself defines (`V`, `castle`, `CUT`).
4. **A token is declared where it is used.** A chapter that uses a token — a formula, a table, a fixture
   — declares it in a table of its own, so no name is ever used that nothing introduces. The reference
   skirt does this for every measurement, constant, derived value, allowance and span it names.

## Safety-relevant terms

A ⚠ marks a term whose mistranslation or misreading causes a **wrong cut or a factory rejection**, not
merely a confused sentence. Roadmap §7.6 names the two families: notch types, and the sew/cut line
aliases factories use in rejection emails. Those are the minimum; the mark is applied wherever the
confusion is physical.

The consequence is a shipping rule, not a styling one: **a ⚠ term must have a reviewed termbase entry in
every language before that language ships.** Translating the interface around an unreviewed ⚠ term is
how a cutting room receives a pattern whose "seam line" is its cut line. `G0-CONTRACT.16` owns the
mechanism; this column owns the list.

## The parts, and what each covers

The glossary is partitioned by domain rather than paginated by letter, so a term sits next to the terms
it is confused with. The A–Z index below is the single-page way in.

| Part | Covers |
| --- | --- |
| [Measurements, ease and sizes](glossary/measurements-and-fit.md) | body measurements and POMs, landmarks and procedures, ease and fit intent, size sets, labels and order, grade rules and their attributes |
| [The recipe and the piece](glossary/recipe-and-pieces.md) | the design as construction recipe, drafting operations, pieces and their geometry, cut and net lines, fold and pair, label data |
| [Seams, sewing and allowances](glossary/seams-and-allowances.md) | seam spans and the sewing graph, ease distribution, `walk` and `true`, seam allowances and the five corner treatments |
| [Marks, closures and finishes](glossary/marks-and-closures.md) | every notch type, grainlines and bias, darts, tucks, pleats and gathers, hems, facings, linings, closures, pockets, notions |
| [The model and its numbers](glossary/model-and-numbers.md) | identity and stable references, the internal unit, the five tolerance classes, determinism, canonicalization |
| [Profiles, uncertainty and release](glossary/profiles-and-release.md) | Factory Profiles and their parameters, the five uncertainty states, evidence, the policy matrix, approval and the release package |
| [Interchange and the envelope](glossary/interchange-and-envelope.md) | artifacts and dialects, DXF layers and metadata blocks, HPGL and PDF, grading interchange, and the supported / rejected / deferred vocabulary |
| [Commands and agent authority](glossary/commands-and-authority.md) | the command bus, atomic groups, preview and commit, revision preconditions, undo granularity, the five agent authority levels |

## The terms, A–Z

- [.rul](glossary/interchange-and-envelope.md)
- [A-line](glossary/recipe-and-pieces.md)
- [AAMA](glossary/interchange-and-envelope.md)
- [acceptance state](glossary/profiles-and-release.md)
- [actor](glossary/commands-and-authority.md)
- [adaptive precision](glossary/model-and-numbers.md)
- [agent authority level](glossary/commands-and-authority.md)
- [all-contours-graded](glossary/interchange-and-envelope.md)
- [allowance width](glossary/seams-and-allowances.md)
- [angle](glossary/model-and-numbers.md)
- [apex](glossary/marks-and-closures.md)
- [approval](glossary/profiles-and-release.md)
- [approve (authority)](glossary/commands-and-authority.md)
- [area](glossary/model-and-numbers.md)
- [armscye](glossary/seams-and-allowances.md)
- [artifact](glossary/interchange-and-envelope.md)
- [artifact policy matrix](glossary/profiles-and-release.md)
- [assembly order](glossary/seams-and-allowances.md)
- [assertion (recipe)](glossary/recipe-and-pieces.md)
- [assumed](glossary/profiles-and-release.md)
- [ASTM D6673](glossary/interchange-and-envelope.md)
- [atomic group](glossary/commands-and-authority.md)
- [audit trail](glossary/commands-and-authority.md)
- [axis (of a size set)](glossary/measurements-and-fit.md)
- [balance notch](glossary/seams-and-allowances.md)
- [base size](glossary/measurements-and-fit.md)
- [bespoke structure](glossary/recipe-and-pieces.md)
- [bias](glossary/marks-and-closures.md)
- [BLOCK (DXF)](glossary/interchange-and-envelope.md)
- [block (pattern)](glossary/recipe-and-pieces.md)
- [bodice](glossary/recipe-and-pieces.md)
- [body measurement](glossary/measurements-and-fit.md)
- [body scan](glossary/interchange-and-envelope.md)
- [bounding box](glossary/recipe-and-pieces.md)
- [button](glossary/marks-and-closures.md)
- [buttonhole](glossary/marks-and-closures.md)
- [CAM driver](glossary/interchange-and-envelope.md)
- [canonicalization](glossary/model-and-numbers.md)
- [cap ease](glossary/seams-and-allowances.md)
- [castle notch](glossary/marks-and-closures.md)
- [centre back (CB)](glossary/recipe-and-pieces.md)
- [centre front (CF)](glossary/recipe-and-pieces.md)
- [check notch](glossary/marks-and-closures.md)
- [chordal tolerance](glossary/model-and-numbers.md)
- [circular arc](glossary/model-and-numbers.md)
- [classic collar](glossary/recipe-and-pieces.md)
- [closure](glossary/marks-and-closures.md)
- [collar fall](glossary/recipe-and-pieces.md)
- [collar stand](glossary/recipe-and-pieces.md)
- [colorway](glossary/interchange-and-envelope.md)
- [command](glossary/commands-and-authority.md)
- [command bus](glossary/commands-and-authority.md)
- [commit (authority)](glossary/commands-and-authority.md)
- [conditional](glossary/recipe-and-pieces.md)
- [conservative default](glossary/profiles-and-release.md)
- [construction recipe](glossary/recipe-and-pieces.md)
- [consumption](glossary/interchange-and-envelope.md)
- [corner treatment](glossary/seams-and-allowances.md)
- [costing](glossary/interchange-and-envelope.md)
- [count](glossary/model-and-numbers.md)
- [cubic Bézier](glossary/model-and-numbers.md)
- [cumulative grading](glossary/measurements-and-fit.md)
- [custom size chart](glossary/measurements-and-fit.md)
- [cut line](glossary/recipe-and-pieces.md)
- [cut on fold](glossary/recipe-and-pieces.md)
- [cut quantity](glossary/recipe-and-pieces.md)
- [cut-as-1](glossary/interchange-and-envelope.md)
- [dart](glossary/marks-and-closures.md)
- [dart intake](glossary/marks-and-closures.md)
- [dart leg](glossary/marks-and-closures.md)
- [declared domain](glossary/model-and-numbers.md)
- [declared zero](glossary/measurements-and-fit.md)
- [deferred](glossary/interchange-and-envelope.md)
- [dependency closure](glossary/profiles-and-release.md)
- [depth](glossary/measurements-and-fit.md)
- [derived](glossary/profiles-and-release.md)
- [design](glossary/recipe-and-pieces.md)
- [design ease](glossary/measurements-and-fit.md)
- [deterministic replay](glossary/model-and-numbers.md)
- [dev shell](glossary/interchange-and-envelope.md)
- [diagnostic](glossary/profiles-and-release.md)
- [differential](glossary/seams-and-allowances.md)
- [digitizing](glossary/interchange-and-envelope.md)
- [dimension error](glossary/model-and-numbers.md)
- [disposition](glossary/profiles-and-release.md)
- [double notch](glossary/marks-and-closures.md)
- [drafting operation](glossary/recipe-and-pieces.md)
- [drafting system](glossary/recipe-and-pieces.md)
- [drill hole](glossary/marks-and-closures.md)
- [DXF](glossary/interchange-and-envelope.md)
- [DXF layer](glossary/interchange-and-envelope.md)
- [ease](glossary/measurements-and-fit.md)
- [ease distribution](glossary/seams-and-allowances.md)
- [EdgeRef](glossary/model-and-numbers.md)
- [entity id](glossary/model-and-numbers.md)
- [envelope (corner)](glossary/seams-and-allowances.md)
- [equivalence report](glossary/profiles-and-release.md)
- [ERP](glossary/interchange-and-envelope.md)
- [evaluation order](glossary/recipe-and-pieces.md)
- [evidence](glossary/profiles-and-release.md)
- [exact ratio](glossary/model-and-numbers.md)
- [exact rational arithmetic](glossary/model-and-numbers.md)
- [expression](glossary/model-and-numbers.md)
- [extension](glossary/recipe-and-pieces.md)
- [extreme size](glossary/measurements-and-fit.md)
- [face side](glossary/recipe-and-pieces.md)
- [facing](glossary/marks-and-closures.md)
- [Factory Profile](glossary/profiles-and-release.md)
- [feature matrix](glossary/interchange-and-envelope.md)
- [fit intent](glossary/measurements-and-fit.md)
- [fixed perimeter](glossary/measurements-and-fit.md)
- [format quantization](glossary/model-and-numbers.md)
- [formula graph](glossary/recipe-and-pieces.md)
- [front-end adapter](glossary/commands-and-authority.md)
- [fused attachment](glossary/marks-and-closures.md)
- [gather](glossary/marks-and-closures.md)
- [generate (authority)](glossary/commands-and-authority.md)
- [geometric approximation](glossary/model-and-numbers.md)
- [girth](glossary/measurements-and-fit.md)
- [golden file](glossary/model-and-numbers.md)
- [grade point](glossary/measurements-and-fit.md)
- [grade rule](glossary/measurements-and-fit.md)
- [grain](glossary/marks-and-closures.md)
- [grainline](glossary/marks-and-closures.md)
- [hard restriction](glossary/profiles-and-release.md)
- [hem](glossary/marks-and-closures.md)
- [hole](glossary/recipe-and-pieces.md)
- [hook and bar](glossary/marks-and-closures.md)
- [HPGL/PLT](glossary/interchange-and-envelope.md)
- [I-notch](glossary/marks-and-closures.md)
- [idempotency](glossary/commands-and-authority.md)
- [imported geometry](glossary/recipe-and-pieces.md)
- [importer comparison](glossary/model-and-numbers.md)
- [inclusion policy](glossary/seams-and-allowances.md)
- [incremental grading](glossary/measurements-and-fit.md)
- [inspect (authority)](glossary/commands-and-authority.md)
- [instance](glossary/recipe-and-pieces.md)
- [interchange dialect](glossary/interchange-and-envelope.md)
- [interfacing](glossary/marks-and-closures.md)
- [internal construction line](glossary/recipe-and-pieces.md)
- [internal unit](glossary/model-and-numbers.md)
- [kind (of a value)](glossary/model-and-numbers.md)
- [knit or stretch block](glossary/recipe-and-pieces.md)
- [known](glossary/profiles-and-release.md)
- [label data](glossary/recipe-and-pieces.md)
- [landmark](glossary/measurements-and-fit.md)
- [layer index](glossary/recipe-and-pieces.md)
- [leather](glossary/recipe-and-pieces.md)
- [length](glossary/model-and-numbers.md)
- [lining](glossary/marks-and-closures.md)
- [loss report](glossary/interchange-and-envelope.md)
- [made-to-measure (MTM)](glossary/measurements-and-fit.md)
- [manifest](glossary/profiles-and-release.md)
- [marker](glossary/interchange-and-envelope.md)
- [material](glossary/recipe-and-pieces.md)
- [MCP](glossary/commands-and-authority.md)
- [measurement](glossary/measurements-and-fit.md)
- [measurement table](glossary/measurements-and-fit.md)
- [microdegree](glossary/model-and-numbers.md)
- [micrometre](glossary/model-and-numbers.md)
- [mirror](glossary/recipe-and-pieces.md)
- [mirrored pair](glossary/recipe-and-pieces.md)
- [miter (corner)](glossary/seams-and-allowances.md)
- [multi-size branching](glossary/recipe-and-pieces.md)
- [name binding](glossary/recipe-and-pieces.md)
- [nap](glossary/marks-and-closures.md)
- [negative ease](glossary/measurements-and-fit.md)
- [nesting](glossary/interchange-and-envelope.md)
- [net line](glossary/recipe-and-pieces.md)
- [non-goal](glossary/interchange-and-envelope.md)
- [notch](glossary/marks-and-closures.md)
- [notch depth](glossary/marks-and-closures.md)
- [notch encoding](glossary/marks-and-closures.md)
- [notch type](glossary/marks-and-closures.md)
- [notions](glossary/marks-and-closures.md)
- [numerical tolerance](glossary/model-and-numbers.md)
- [off-grain](glossary/marks-and-closures.md)
- [offset](glossary/seams-and-allowances.md)
- [one-to-many span](glossary/seams-and-allowances.md)
- [order object](glossary/measurements-and-fit.md)
- [outer boundary](glossary/recipe-and-pieces.md)
- [package completeness](glossary/profiles-and-release.md)
- [parameter](glossary/recipe-and-pieces.md)
- [parameterized reference](glossary/model-and-numbers.md)
- [partial span](glossary/seams-and-allowances.md)
- [pathology corpus](glossary/interchange-and-envelope.md)
- [pattern](glossary/recipe-and-pieces.md)
- [PDF](glossary/interchange-and-envelope.md)
- [pen map](glossary/interchange-and-envelope.md)
- [physical acceptance](glossary/model-and-numbers.md)
- [piece](glossary/recipe-and-pieces.md)
- [pleat](glossary/marks-and-closures.md)
- [PLM](glossary/interchange-and-envelope.md)
- [plotter unit](glossary/interchange-and-envelope.md)
- [pocket](glossary/marks-and-closures.md)
- [point of measure (POM)](glossary/measurements-and-fit.md)
- [PointRef](glossary/model-and-numbers.md)
- [precedence](glossary/profiles-and-release.md)
- [preference](glossary/profiles-and-release.md)
- [preview / commit](glossary/commands-and-authority.md)
- [print placement](glossary/interchange-and-envelope.md)
- [procedure](glossary/measurements-and-fit.md)
- [production release](glossary/profiles-and-release.md)
- [progress](glossary/commands-and-authority.md)
- [propose (authority)](glossary/commands-and-authority.md)
- [provenance](glossary/profiles-and-release.md)
- [PST](glossary/interchange-and-envelope.md)
- [quadrant](glossary/recipe-and-pieces.md)
- [R12 / R13](glossary/interchange-and-envelope.md)
- [raster tracing](glossary/interchange-and-envelope.md)
- [ratio](glossary/model-and-numbers.md)
- [ready-to-wear (RTW)](glossary/measurements-and-fit.md)
- [recipe replay](glossary/recipe-and-pieces.md)
- [reconstruction](glossary/measurements-and-fit.md)
- [reference drafting](glossary/recipe-and-pieces.md)
- [registration mark](glossary/interchange-and-envelope.md)
- [rejected](glossary/interchange-and-envelope.md)
- [release package](glossary/profiles-and-release.md)
- [repair task](glossary/model-and-numbers.md)
- [resolved size set](glossary/measurements-and-fit.md)
- [revision](glossary/recipe-and-pieces.md)
- [revision precondition](glossary/commands-and-authority.md)
- [robust predicate](glossary/model-and-numbers.md)
- [roll line](glossary/recipe-and-pieces.md)
- [rounding rule](glossary/model-and-numbers.md)
- [scale square](glossary/interchange-and-envelope.md)
- [seam](glossary/seams-and-allowances.md)
- [seam allowance](glossary/seams-and-allowances.md)
- [seam length](glossary/seams-and-allowances.md)
- [seam span](glossary/seams-and-allowances.md)
- [self-intersection](glossary/seams-and-allowances.md)
- [sew-as-1](glossary/interchange-and-envelope.md)
- [sewing graph](glossary/seams-and-allowances.md)
- [shirt](glossary/recipe-and-pieces.md)
- [shrinkage](glossary/measurements-and-fit.md)
- [single notch](glossary/marks-and-closures.md)
- [size break](glossary/measurements-and-fit.md)
- [size chart](glossary/measurements-and-fit.md)
- [size label](glossary/measurements-and-fit.md)
- [size order](glossary/measurements-and-fit.md)
- [size set](glossary/measurements-and-fit.md)
- [size system](glossary/measurements-and-fit.md)
- [size-set transformation](glossary/measurements-and-fit.md)
- [slant (corner)](glossary/seams-and-allowances.md)
- [slash and spread](glossary/recipe-and-pieces.md)
- [sleeve cap](glossary/seams-and-allowances.md)
- [slit notch](glossary/marks-and-closures.md)
- [smoothing](glossary/measurements-and-fit.md)
- [spreading](glossary/interchange-and-envelope.md)
- [SST](glossary/interchange-and-envelope.md)
- [stable topological reference](glossary/model-and-numbers.md)
- [stack point](glossary/measurements-and-fit.md)
- [stale-ification](glossary/profiles-and-release.md)
- [step (corner)](glossary/seams-and-allowances.md)
- [stitches per inch (SPI)](glossary/seams-and-allowances.md)
- [stop landmark](glossary/seams-and-allowances.md)
- [stripe reference](glossary/marks-and-closures.md)
- [structural limit](glossary/model-and-numbers.md)
- [supported](glossary/interchange-and-envelope.md)
- [supported envelope](glossary/interchange-and-envelope.md)
- [T-notch](glossary/marks-and-closures.md)
- [target system](glossary/profiles-and-release.md)
- [tech pack](glossary/interchange-and-envelope.md)
- [tessellation](glossary/model-and-numbers.md)
- [tiled export](glossary/interchange-and-envelope.md)
- [tolerance class](glossary/model-and-numbers.md)
- [tolerance name](glossary/model-and-numbers.md)
- [trim (corner)](glossary/seams-and-allowances.md)
- [trousers](glossary/recipe-and-pieces.md)
- [true (operation)](glossary/seams-and-allowances.md)
- [tuck](glossary/marks-and-closures.md)
- [turn point](glossary/seams-and-allowances.md)
- [typed diagnostic](glossary/model-and-numbers.md)
- [U-notch](glossary/marks-and-closures.md)
- [undo/redo granularity](glossary/commands-and-authority.md)
- [universal package](glossary/interchange-and-envelope.md)
- [unknown](glossary/profiles-and-release.md)
- [unresolved reference](glossary/model-and-numbers.md)
- [V-notch](glossary/marks-and-closures.md)
- [vacant seat](glossary/commands-and-authority.md)
- [waistband](glossary/recipe-and-pieces.md)
- [walk (operation)](glossary/seams-and-allowances.md)
- [wearing ease](glossary/measurements-and-fit.md)
- [width (flat)](glossary/measurements-and-fit.md)
- [winding](glossary/recipe-and-pieces.md)
- [workflow parity](glossary/commands-and-authority.md)
- [wrong side](glossary/recipe-and-pieces.md)
- [yield](glossary/interchange-and-envelope.md)
- [zipper](glossary/marks-and-closures.md)

## What this glossary is for, and what it is not

**It is the source termbase.** Roadmap §7.6 asks for a glossary/termbase per language, with the
safety-relevant terms first. This chapter is the English source those termbases are built from: the
term, the token, the ⚠ mark and the canonical object are the four fields a translator needs, and the
"Also called" column is the list of synonyms a translation must not silently merge. The format a
shipped termbase takes is `G0-CONTRACT.16`'s decision, not this chapter's.

**It is not a specification.** Nothing here is normative about behaviour; every rule lives in the
chapter the "Canonical object" column names. A glossary entry that started stating requirements would
become a second, drifting copy of the specification.

**It is not complete by declaration.** Its completeness is derived:

```bash
bash docs/tasks/artifacts/glossary/run_glossary_census.sh
# → glossary census: <terms> terms / <parts> parts / <tokens> tokens / 0 failure(s)
```

The census enforces the structural claims — one meaning per term, one owner per token, every canonical
reference resolving to a chapter clause, a roadmap clause or a real task-tree leaf, every ⚠ the roadmap
requires still in place, this index equal to the one derived from the parts — and it enforces the
coverage claim: every machine token the specification chapters use is either owned here, declared by
the chapter that uses it, or exempted with a written reason. What it cannot judge is whether a
definition is *correct*; that is the domain review, and the seat that performs it is **vacant**
([governance §8.1](../governance.md)).

## Verification status of the synonyms

- **Object names, tokens and clause references** — *exact*: they are quoted from the specification
  chapters and from `crates/sc-units`, and the census checks each reference resolves.
- **Terms and definitions drawn from the roadmap** — *cited from the roadmap*: the notch-type
  vocabulary, the AAMA and ASTM layer names, SST/PST, `.rul` attributes, the cut/sew swap, the
  tolerance classes and the uncertainty states all come from `ROADMAP.md`, which records its own review
  provenance in its disposition log.
- **Trade synonyms in other languages, and other CAD systems' words for a term** — *general industry
  usage, unverified in this repository*. They are offered as recognition aids for a reader who arrives
  from another system, not as claims about any vendor's documentation or user interface. The domain
  reviewer — a seat governance §8.1 records as **vacant** — confirms or corrects them, and every ⚠ term's
  synonyms are the ones that must be confirmed before a termbase ships.
- **Terms whose precise meaning a later chapter owns** — the entry says which leaf specifies them
  (`stack point`, `fixed perimeter` and `smoothing` are named by the roadmap's grading clause without
  being defined there, and this glossary does not invent a definition for them).
