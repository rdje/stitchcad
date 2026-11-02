# ADR-0003 — the construction recipe is primary, its formula language is specified here, and Aldrich is the named drafting system

- **Type:** `decision` (ADR-0003 in `ROADMAP.md` §5)
- **Date:** `2026-09-30` (absolute); the roadmap locked the paradigm at v0.2 and required the formula
  language "specified HERE, not later", so this record settles it before any evaluator exists
- **Status:** `active`
- **Owner / source:** leaf `G0-CONTRACT.9`, from roadmap §5 ADR-0003, §2.2, §3.1, §3.3, §4.2, §15.1
  and §15.3; the normative text is `docs/book/src/spec/formula-language.md` and its two parts

answers: "what is the formula language?" · "why does a recipe round twice and not once per operator?" · "why can a formula not compare size labels?" · "which drafting system ships as the reference blocks?" · "why not Seamly2D's drafting?" · "what does a recipe's `assert` exist for?" · "what would reopen ADR-0003?" · "how is a truing curve selected?" · "what does an exact native join require?" · "what binds an ordered program to its actual frame and geometry snapshot?"

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

[Whole original chapter-partition rationale](../history/window29.md#stitchcad-devnotes-part997md) retains
the historical sizes and two-way census policy;the normative chapters remain canonical.

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

## Closed diagnostic-policy discussions

[Whole D124/D131/D121 discussions](../history/window29.md#stitchcad-devnotes-part948md) retain their
source byte-exact. Current formula diagnostics and indexed D131 binding-source decision remain normative.

## Native exact nondeforming join domain —0773

Author and applier:the same Codex engineer under the director's2026-11-01 PNT delegation;
owner G1-SLICE.5f.1.c.join.outputs.domains. This declares one explicit native join domain.
Actual execution/envelope/domain evidence remains **unapproved**,for the independent reviewer/domain seat.

An exact splice removes the two selected coincident banks without stretching,gathering,refitting,
snapping or forking a copy. Actual allocated ease must resolve to zero;retain the original
declaration or Portion rather than authoring another amount. Nonzero resolved ease contradicts
this domain. Unknown/unsupported/uncertified/resource/cancel causes remain actual causes,not
proof of incompatibility. A different deformation policy requires a separately declared variant.

Placement is explicit:keep both original resolved placements,or keep named side A/B fixed and
require unique nondegenerate rigid alignment of the other. The moving authored Piece's parity
is explicitly Preserve or Reflect;selected-copy reflection never supplies it by inference.
Alignment uses both actual Same/Opposite endpoint pairs in their original held frames. Actual
geometry must reconcile physical-copy transforms and prove entire-bank coincidence and valid
complete joined topology. Endpoints alone prove neither congruence nor a numeric interior map.
Ambiguous/degenerate alignment requires an actual refusal;no additional anchor or pose is guessed.

Consequences:tracked native splice tests/compiler guards and book examples check explicit choices,
authentic parents/endpoints,partial Portion ease and reflected copies. Native requirement binding
performs no geometry,allocation or provider call. .join.outputs.review checks this declared domain;
G1 .5f.2/.5f.3/.5f.4.geometry,G2/G3 and .5g retain actual context,geometry,garment and integration proofs.
Output metadata/copy/contour/identity/repair obligations remain separate joined-Piece declarations.

Reopen with a recorded recipe-policy/version change if an actual supported garment needs another
join domain. Preserve original ease/copy/held-frame semantics and explicit placement choices;
never broaden an exact splice silently. The canonical API contract is the
[native join reference](../book/src/annexes/drafting-join-contracts.md).

## Native constrained truing priority —0770

Author and applier:the same Codex engineer under the director's2026-11-01 PNT delegation;
owner G1-SLICE.5f.1.c.true.variants.selection. This adopts native declaration semantics;
execution/domain evidence remains **unapproved**,for the independent gate reviewer/domain seat.

The recipe supplies ordered,nonempty curve alternatives for each selected output role. Each
alternative retains an explicit DirectedRange of an earlier declared curve. Outside fragments
retain their original geometry requirement;source preservation supplies no new-curve groups.
The policy is first certified-feasible **complete assignment**,in authored Cartesian order,
rightmost group fastest. Identity sorting does not establish priority. Source locks and length
alone leave distinct equal-length curves;0769 records an exact reflected-quadratic witness.

Whole N/M/K/combination/workspace bounds precede native field scans/allocation. Native admission
only proves complete local attribution and priority data. Actual selection checks the complete
proposed Piece,all coupled copies/observers,endpoint/stops and sourced fit/ease budgets.
Unavailable/unsupported/uncertified/resource/cancel outcomes retain their causes;only a proved
infeasible assignment can be skipped. Exhaustion is scoped to the declared alternatives.

Consequences:tracked truing curve-admission tests and book examples verify bounds,completeness,
original order and authentic source fields. G1-SLICE.5f.4.geometry/G3-GRADING.2 own actual
candidate generation,certified feasibility/error propagation and envelope evidence;the native
coverage review stays .true.variants.review. These checks grant no independent approval.
The canonical contract is the truing whole-Piece book companion's curve-selection section.

Reopen with a recorded recipe-policy/version decision if an actual supported garment requires
a different selection rule or domain. Preserve authored priority/replay semantics and review
all affected examples;no implicit optimizer,default curve or silent search-domain widening.

## Actual containing context —0822

Owner:G1-SLICE.5f.2.context.contract. The
[containing context contract](../book/src/annexes/drafting-operation-contracts.md#containing-context-contract--implementation-pending)
requires actual Design/program/revision,authored Cartesian frame and typed current snapshot sources.
Earlier declared outputs confer static availability,not live geometry. The .context owner implements
bounded immutable source binding before .program;whole verification remains .verify,geometry G2/G3.
