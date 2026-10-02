# ADR-0003 — the construction recipe is primary, its formula language is specified here, and Aldrich is the named drafting system

- **Type:** `decision` (ADR-0003 in `ROADMAP.md` §5)
- **Date:** `2026-09-30` (absolute); the roadmap locked the paradigm at v0.2 and required the formula
  language "specified HERE, not later", so this record settles it before any evaluator exists
- **Status:** `active`
- **Owner / source:** leaf `G0-CONTRACT.9`, from roadmap §5 ADR-0003, §2.2, §3.1, §3.3, §4.2, §15.1
  and §15.3; the normative text is `docs/book/src/spec/formula-language.md` and its two parts

answers: "what is the formula language?" · "why does a recipe round twice and not once per operator?" · "why can a formula not compare size labels?" · "which drafting system ships as the reference blocks?" · "why not Seamly2D's drafting?" · "what does a recipe's `assert` exist for?" · "what would reopen ADR-0003?"

## The decision

**The construction recipe is the primary authoring model, its expression language has a normative
G0 specification, and the one documented drafting system that ships as reference blocks is Aldrich's
metric pattern cutting** (Winifred Aldrich, *Metric Pattern Cutting for Women's Wear*, Wiley).
Geometric sketch constraints stay optional local annotations, and grading stays a second
instantiation path rather than a re-solve. The paradigm itself is roadmap §15.1 and is not reopened
here; what this record decides is everything the roadmap left to G0: the language, and the system.

## The language: fourteen decisions, and the alternative each rejected

The normative text is the chapter; this is the reasoning, so a later session does not re-litigate it.

| Decision | Rejected | Why |
| --- | --- | --- |
| every binding declares its kind, checked before any value is computed | inference alone | a declared kind is what makes a wrong expression visible at authoring time; inference would type a mistake consistently |
| exact rational arithmetic; two rounding points (an irrational call, a binding) | per-operator fixed-point rounding | rounding at every operator accumulates, and two routes to one value then differ by more than T1 — the closure checks the fixture depends on would stop closing |
| irrational functions specified by their *result* — the nearest internal quantum, ties away from zero | "evaluate in `f64`" | a different libm is a different answer, and determinism is a release property (units §7) |
| no text values anywhere in the language | strings for labels and notes | a size label is prose (size-sets §3) and a note would not survive canonicalization; both would drag localization into the evaluator |
| closed operator and function sets, with the product/quotient table as the whole law | an extensible built-in set | an open set cannot be conformance-tested, and the envelope is the release claim's boundary |
| `assert` is a statement form, and a failed one refuses the recipe | checks that live only in tests | D33 was a recipe that satisfied every declared quantity and drafted a waist 28.0 cm short; the oracle has to travel with the recipe |
| declaration order is the evaluation order; a forward reference is refused | topological reordering | reordering makes the recipe's meaning depend on its evaluator, and the recipe is the authority |
| single assignment, and a name collision between origins is refused | scoped rebinding or shadowing | a profile parameter that quietly overrides a measurement is a pattern nobody authored |
| the size context is ordinals and a base-size flag; branching on a label is impossible | a label comparison | a label is prose and the authored order is data — the language offers the second and refuses the first |
| five reserved tolerance names carry the class into a comparison | a numeric tolerance argument | units §3 requires every comparison to name its class; a name is data and a literal is not |
| `point` and `edge` are values but not bindable by `let` | formulas that construct geometry | only an operation can own an entity's identity (ontology §1); a formula that invented a point would create geometry nothing holds |
| four structural limits, with the node limit derived from what the book measures | no limit, or a guessed one | a corrupt or adversarial recipe must be refused rather than exhaust memory, and a limit nobody measures is a round number |
| the canonical form is an S-expression and is the formula's identity | JSON, or the surface spelling | JSON reintroduces a float-formatting question the units chapter closes; two spellings of one formula must not be two formulas |
| an envelope refusal raises the envelope's token, not a formula token | a formula token for everything | two token sets competing for one refusal is how a diagnostic registry drifts |

The chapter is partitioned into a contract, a grammar part and an examples part because one file
landed at 599 lines / 37 317 B against a `book_collection` per-part health of 400 / 24 576 and a
ceiling of 700 / 40 960 — 87 % of the ceiling on the day it was born, with every table normative and
nothing to trim. The partition is the containment doctrine's own remedy for a `partitioned_canonical`
surface, and the glossary set the precedent. The three parts are 308 / 241 / 92 lines and
20 593 / 12 452 / 7 116 B, each inside health, and the contract's §7 parts table is censused in both
directions so a fourth file cannot appear unlisted.

**The margin the census enforces.** `max_expression_nodes` = 256 must exceed the largest expression
the book's examples carry by **20×** (measured: 7 nodes). The margin is declared here rather than in
the instrument so that raising it is a recorded decision, and the instrument refuses a limit that
does not clear it.

## The drafting system: what was read, and what was not

Roadmap ADR-0003 names the lineage — "Seamly2D/Aldrich/Müller & Sohn" — so the candidate set is the
roadmap's, not an invention. Each was checked from this machine on `2026-09-30`; every source is
`read-external` (a URL and the date read, per the fixture chapter's convention), none is a standard,
and none upgrades a claim about one (`docs/book/src/spec/standards.md` §1 keeps that vocabulary
closed).

| Candidate | What was read | Verdict |
| --- | --- | --- |
| Aldrich | OpenLibrary's work record OL16995319W and its editions (`https://openlibrary.org/works/OL16995319W.json`): six editions, Wiley and Blackwell, 224 pp in 2008 and 256 pp in the 2015 edition, ISBN 9781119028284. The title itself carries *metric* and *women's wear* | **named** |
| Seamly2D | the GitHub repository record (`https://api.github.com/repos/FashionFreedom/Seamly2D`): licence **GPL-3.0** | rejected: its drafting is embodied in GPL code, so the reference blocks would be a projection of it — the licence coupling ADR-0001 rejected for `slvs`, and not an independent oracle for our own engine |
| Müller & Sohn | `https://muellerundsohn.de/` returned an interstitial ("Just a moment…"), and an OpenLibrary title search returned nothing | rejected for v1: the least verifiable of the three from here, and a German-language source a reader of this book could not follow a citation into. It stays the substitution candidate §Re-open names |
| Armstrong | OpenLibrary (`https://openlibrary.org/search.json?q=patternmaking+for+fashion+design+armstrong`): 16 editions from 1987, 818 pp median, Prentice Hall and Pearson | not named by the roadmap's lineage, and its bibliographic record carries no metric claim in its title |
| FreeSewing | the site and the monorepo record (`https://api.github.com/repos/freesewing/freesewing`): **MIT**, and **archived** at the time of reading | rejected: a collection of individually authored parametric designs rather than one taught method, and an archived upstream is a poor normative reference |

Two more sources were attempted and not read, recorded so the next session does not re-try them:
`https://seamly.net/` returned nothing, and the Google Books API answered HTTP 429 (quota).

**What is adopted, and what is not.** The *method* is adopted: construction sequences and their
measurement-derived numbers, cited per step by edition and page, entering as `read-external` when
they are read. No text, table or illustration is reproduced, and nothing has been read yet — so
every number that will come from the system is `unverified-with-owner` today, not `assumed` and not
`known`. The reference skirt is deliberately **not** a transcription of it: the fixture declares its
own constants and so carries none of this decision's verification debt, which keeps it an
independent subject for the G2 goldens.

**Owners.** `G3-GRADING.16` (created by this leaf) ships the blocks, because a decision with no
owner is a wish — the same rule `.4c` applied to the roadmap's envelope criterion. The procurement
owner seat owes the source and the vacant sewing/factory domain expert seat owes the review of the
transcription (`docs/book/src/governance.md` §8.1). Whether shipping drafting data *derived from* a
commercial source needs a licence clearance is flagged to the project owner's seat as an open
question; this record does not answer a legal question it has no authority over, and it does not
assume the answer is no.

## How to apply

- Implement `G1-SLICE.5` against the chapter, and make every [worked example (§§2–4)](../book/src/spec/formula-language/examples.md) a test — the census at
  `docs/tasks/artifacts/formula_language/run_formula_language_census.sh` is the oracle those tests
  are written from, and its evaluator reads the chapter's own tables rather than a copy of them.
- A new operator, function, kind, diagnostic or unit token is a change to the chapter's tables first;
  the census refuses an example that uses vocabulary no table declares, and refuses a declared
  function the evaluator does not implement.
- Refinement at G1-SLICE.5a.3e.3: diagnostic index/canonical expression are supplied where known;
  malformed syntax retains exact span/rule and never invented canonical context (D110). Complete
  assertion/recipe identity bytes are specified by D109/.3f.1a under engineering delegation; expression bytes
  are settled by D103. Product recipe normalization/serialization is implemented at .1b/.1c and reviewed at .3f.2. These
  qualifications do not change numeric or evaluation semantics.
- Never widen a structural limit to land a recipe, and never add a rounding step: both are recorded
  decisions, not implementation conveniences.
- Cite the system per step and reproduce nothing; a number with no citation is `assumed` and belongs
  to the vacant domain seat, not to this record.

## What would reverse it

Three conditions, each requiring a recorded decision: (1) the named domain expert substitutes a
drafting system — Müller & Sohn is the standing candidate — which re-derives every block and
re-cites every number; (2) a G1 or G2 measurement shows the exact-rational evaluator cannot meet a
performance budget the product needs, in which case the fix is a representation change under the
same value semantics and never a change of the rounding points; (3) a real recipe needs a construct
[formula exclusions §6](../book/src/spec/formula-language.md#6-exclusions) excludes, which arrives as a v2 candidate with a worked example over a garment, or not at all.

## D124 diagnostic recognition proposal — pending director ruling

Reviewed2026-10-02 at G1-SLICE.5b.1c.1. Contract6 assigns formula_unsupported to loops and
function/macro definitions, but v1 defines no source forms for them. Actual reference returns
formula_unbound_name for loop(width), repeat(2,width) and while(width>0 um); fn helper(width)=width
and macro helper(width)=width return formula_parse. A declared loop identifier remains valid.
This is a diagnostic-contract gap, not permission to add executable loops or functions.

Recommended: retain the three reserved grammar keywords. Unknown calls stay formula_unbound_name;
malformed loop/definition syntax stays formula_parse. Recognized non-square exponents retain
formula_unsupported, and envelope calls retain their envelope tokens. Clarify the exclusions table
to separate unsupported capabilities from diagnostic recognition by the closed grammar.

Alternative: define an explicit closed set of recognizable excluded forms that raise
formula_unsupported. That requires exact source spellings and keyword/call-role rules, including
whether a previously valid parameter named loop, repeat, fn or macro remains valid. No such list
is inferred from these illustrative examples. Neither choice enables any excluded capability.
G1-SLICE.5b.1c.2 owns the ruling, implementation and closure; until then the reference is not an
oracle for the table's unspecified excluded-form diagnostics. Other static evidence retains scope.
