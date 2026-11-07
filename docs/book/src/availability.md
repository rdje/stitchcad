# What you can use today

StitchCAD currently provides eight Rust libraries and [a headless command line](spec/command-line.md). They
implement numerical and structural foundations; they do not yet generate a garment pattern or provide an
end-user application.
G0 contract review is mostly complete, and G1's seven exit clauses are met with stated limits. Human closure of both is
unapproved. G2 is in progress.

## Distance to the finished product

The roadmap's seven gates to a declared product end at G6. This table counts one thing: exit clauses whose
real product path passes its check. Commits and leaves do not move it.

| Gate | Exit clauses met | Exit clauses | What it proves |
| --- | --- | --- | --- |
| G0 | 18 | 19 | The product and its semantics are specified. The evaluation-seat clause stays open by ruling |
| G1 | 7 | 7 | The architecture runs for real in a browser |
| G2 | 7 | 8 | A reference skirt is drafted and exported correctly |
| G3 | 5 | 11 | Every garment the roadmap names drafts, grades and exports |
| G4 | 0 | 8 | Factory profiles work, including what is still unknown |
| G5 | 0 | 6 | The native and browser applications exist |
| G6 | 0 | 7 | The conformance lab shows it holds up outside this repository |

Run it yourself:

```text
$ bash scripts/gauge.sh
G0: 18 of 19 clauses met
G1: 7 of 7 clauses met
G2: 7 of 8 clauses met
G3: 5 of 11 clauses met
G4: 0 of 8 clauses met
G5: 0 of 6 clauses met
G6: 0 of 7 clauses met
gauge: 37 of 66 clauses met through G6
```

A count here comes from the clause tables, and a clause counts as met only when its check runs a real
product path and passes. `bash scripts/gauge.sh --checks` runs those checks again.

Commercial CAD is parked until StitchCAD is feature complete, so G6's import-filter clause stays not met.
G2's printed square stays not met until a person measures one. Both are the director's to close.

## Distance to the G3 exit

G3 is the earliest open gate that software can close. G2's only open clause needs a person with a ruler, so it no longer holds the frontier. G3 goes from one garment to every garment the roadmap names, with construction and grading.

| Clause | What the roadmap asks | Met | Where it stands | Owning leaf |
| --- | --- | --- | --- | --- |
| `G3-01` | A darted bodice and a set-in sleeve with declared ease | met | [A cap fitted to its armhole by a declared ease](spec/reference-bodice.md), which the walk calls intended, and the same difference undeclared is a defect. Five sizes draw and close. Its constants are an engineer's, and nobody has cut it in cloth | `G3-GRADING.4` |
| `G3-02` | Darts, folds, walking and truing | met | [Closures](spec/closures.md) carry their intake and close, and [seams](spec/walking-and-truing.md) walk, plant notches and true as bus commands with undo. No garment yet has an eased seam, a tuck, a pleat or a gather | `G3-GRADING.2` |
| `G3-03` | Stable references survive edits | met | [Notches, grade points and seam ends hold named edges](spec/stable-references.md). A split, merge, reverse, trim or mirror keeps them on the same cloth, a delete raises a repair task, and a cutting file is refused while one is open. Closures still hold positions | `G3-GRADING.3` |
| `G3-04` | Both instantiation paths | not met | [Both work](spec/grading.md): every size from the recipe, and every size from a table of rules, which are measured to differ by up to 4.5 mm on the bodice. A report comparing them is next | `G3-GRADING.11` |
| `G3-05` | A grade re-imported into an independent engine | not met | Not started | `G3-GRADING.8` |
| `G3-06` | Extreme sizes reconstructed | not met | Not started | `G3-GRADING.9` |
| `G3-07` | The grading modes | not met | Not started | `G3-GRADING.10` |
| `G3-08` | Offset and grading golden suites | not met | [The offset corpus](spec/offsets.md) and [the skirt's goldens](spec/reference-skirt.md) exist. No grade does | `G3-GRADING.12` |
| `G3-09` | Every garment the roadmap names | not met | One exists, the skirt | `G3-GRADING.14` |
| `G3-10` | A classic collar | met | [A stand, a fall and a roll line](spec/reference-bodice.md), fitted to the neck. Its edges differ by the declared roll exactly, and the roll line is never on the cut layer. Re-drafted at five sizes, and grade rules are `G3-GRADING.7` | `G3-GRADING.15` |
| `G3-11` | Trousers carrying their constructions | met | [A crotch curve, a fitted inseam and a waistband](spec/reference-trousers.md), with an in-seam pocket as an object and a buttonhole whose length comes from its button. Re-drafted at five sizes, and grade rules are `G3-GRADING.7` | `G3-GRADING.5` |

## Distance to the G2 exit

G2 is the first correct pattern. Seven of its eight clauses are met, each with its stated limits. The eighth needs a person with a ruler, and [physical evaluation](spec/physical-evaluation.md) says how.

| Clause | What the roadmap asks | Met | Where it stands | Owning leaf |
| --- | --- | --- | --- | --- |
| `G2-01` | The reference skirt drafted through the command line, with deterministic replay | met | [The session](spec/reference-skirt.md) drafts all five pieces with the book's numbers, and two runs of the real program give identical bytes. Sewn, it has not been: no person has checked the shape | `G2-2D.8` |
| `G2-02` | A minimal 2D viewer and editor over the same commands | met | [The dev shell](spec/dev-shell.md) draws the skirt and edits it through the bus, and what it records replays through the command line to the same bytes, in real Chrome. It is a page, not the egui the roadmap names, because that fetch was refused here. Saving does not outlive the page | `G2-2D.9` |
| `G2-03` | Canonical DXF and a dimensionally correct PDF | met | [The DXF](spec/dxf-output.md) is frozen for the reference skirt. [The PDF](spec/pdf-output.md) measures true within a pixel under two renderers nobody here wrote. No CAD system has opened the DXF, and no third-party library has read it. No PDF has been printed | `G2-2D.5` |
| `G2-04` | A printed scale square verified on paper | not met | [The PDF](spec/pdf-output.md) measures true on screen. [Five minutes with a ruler](spec/physical-evaluation.md) does the rest, and it has been asked for | `G2-2D.7` |
| `G2-05` | Offsets pass the pathology corpus | met | [The offset engine](spec/offsets.md) passes 37 cases, covering acute tips, loops and seam-length error, judged by checks that share no code with it. The corner shapes are unreviewed by a sewing expert | `G2-2D.3` |
| `G2-06` | Native save and load preserve semantics | met | [A project](spec/project-store.md) holds how to draw the garment and redraws it to the same bytes, save and load and save leaves every byte, and two designs with identical contours and different recipes stay distinct. Undo history does not cross a reopen | `G2-2D.10` |
| `G2-07` | Mutation tests fail for the right reasons | met | [31 mutants](spec/mutation-tests.md), each required to be caught by the check that names it. Grade mutants wait for G3, and the sewing spans are a fixture's table until the graph is data | `G2-2D.11` |
| `G2-08` | An agent given only MCP and the docs produces a semantically equal instance | met | [Two pinned-model sessions](spec/api-and-mcp.md) with only those drafted the skirt, and the command line replays each to a golden match. A first session failed, for reasons the book then fixed, and it is kept. This tests reading and tools, not design | `G2-2D.13` |

## Distance to the G1 exit

Every G1 clause counts as met, each with its stated limits. This is where they stand:

| Clause | What the roadmap asks | Met | Where it stands | Owning leaf |
| --- | --- | --- | --- | --- |
| `G1-01` | Three runtime profiles compile | met | [All three build](spec/runtime-profiles.md), each with its own command. The headless one pulls in no viewport or GUI, and the browser artifact asks its host for nothing. Compiling is all this proves | `.11` |
| `G1-02` | A real browser run: load, evaluate a recipe, one CSP constraint, save and reopen, render, export | met | [Real Chrome runs all six steps](spec/browser-capabilities.md) across two browser processes, and its results equal the native program's. The drawing is a schematic, because drafting is G2 | `.12` |
| `G1-03` | Canvas-hosting spike for ADR-0002 | met | [Measured on the declared corpus](spec/canvas-hosting.md): WebGPU in the browser and native wgpu on the desktop. The verdict holds for macOS only, because no Windows or Linux machine with a display exists here | `.13` |
| `G1-04` | Command bus with revision checks and undo | met | [The command line](spec/command-line.md) drives it for real: measurement edits, a refused stale revision, undo and redo. The bus's other commands are still library only | `.6` |
| `G1-05` | Crash-consistent save and recovery | met | [A project](spec/project-store.md) survives a crash at every step of a save, a migration and an autosave, and a killed process, on a real directory. Power loss is not tested, and the browser side comes with the browser run | `.7` |
| `G1-06` | License-compatible solver builds on all targets | met | [The kernel](spec/constraints.md) builds natively and for the browser with no dependencies, and is in the headless and browser profiles. [The license census](build-and-checks.md#the-license-census) finds no copyleft. Windows is not built anywhere yet | `.15` |
| `G1-07` | Browser capability matrix with tested versions and limitations | met | [The matrix](spec/browser-capabilities.md) states one Chrome on one machine. Other browsers are listed as not tested, and they belong to G5 | `.12` |

Drafting the reference skirt itself is the G2 gate. It needs the geometry kernel and drafting execution,
which this table does not include.

Since the G1 row reached 6/18 on 2026-10-26, about 650 commits have mostly widened how exactly inputs
are admitted. That work is real, but it has not moved the exit: a recipe still cannot produce a
pattern.

The director ruled on 2026-11-06 that input checks are on hold, and that work goes all in on features.
More checks come later, as downstream consumers, users or feature completeness show the need. Existing
checks stay and keep passing.

A feature means the roadmap's next requirement, built by its existing task-tree owner to the contract
this book gives, with this table and the book's examples changing in the same commit. It refuses what it
does not support yet with a plain error. The constraint kernel `.8`, the project store `.7`, the headless
command line `.10`, the runtime profiles `.11`, the license census `.15`, the browser run `.12` and the canvas spike `.13`
are done, and the G1 exit review `.16` passed. The geometry kernel `G2-2D.1`, the offset engine `G2-2D.2`, its corpus `G2-2D.3`, the
canonicalizer `G2-2D.4`, the DXF writer `G2-2D.5` and the drafted skirt `G2-2D.8` are done. The PDF writer `G2-2D.6` is written and measured, and waits only on the library comparison. The printed square `G2-2D.7` is measured on screen and waits for a person with a ruler. The viewer and editor `G2-2D.9`
is done as a page, and its toolkit and native window stay open at `G1-SLICE.14`. Native save and load `G2-2D.10` is done. The mutation tests `G2-2D.11` and [the conformance matrix](spec/conformance-matrix.md) `G2-2D.12` are done. [The API and MCP server](spec/api-and-mcp.md) `G1-SLICE.9` works over local stdio. The agent gate `G2-2D.13` passed, and the toile protocol `G2-2D.15` is written. What G2 has left is a person with a ruler, so G3 has begun: [closures](spec/closures.md) `G3-GRADING.1` are done, and [walking and truing](spec/walking-and-truing.md) `G3-GRADING.2` too. [References that survive an edit](spec/stable-references.md) `G3-GRADING.3` is done too. [The bodice and sleeve](spec/reference-bodice.md) `G3-GRADING.4` is done too. [The classic collar](spec/reference-bodice.md) `G3-GRADING.15` is done too. [The trousers](spec/reference-trousers.md) `G3-GRADING.5` are done too. [Regeneration](spec/grading.md) `G3-GRADING.6` is done too. [Grade rules and the `.rul` file](spec/grade-rules.md) `G3-GRADING.7` are done too. The next one is the report comparing the two paths, `G3-GRADING.11`.

## Implemented foundations

| Library | Available behavior |
| --- | --- |
| sc-units | Fixed-point units, explicit conversions, rounding and typed tolerances/errors |
| sc-core | Identities/references, profile pins, structural garment/[Material](spec/material-inputs.md) metadata, physical copies, owned inputs/parameters/recipe source, formula syntax/normalization/identity/static proofs, numeric/read APIs and typed requests/authorization |
| sc-measure | Owned measurement/table/Ease records, membership/axes/breaks, garment/MTM inputs and immutable profile intent |
| sc-constraints | [Typed profile constraints and a deterministic solver](spec/constraints.md): which values fit, or which facts and rules conflict |
| sc-artifacts | [One written form for a drawn pattern](spec/canonical-artifacts.md), comparison by meaning, [DXF](spec/dxf-output.md) in one named mode, and true-scale [PDF](spec/pdf-output.md) |
| sc-geometry | [Lines, arcs and cubic Béziers with exact predicates](spec/geometry.md), and [seam allowances](spec/offsets.md) |
| sc-viewport | [Exact screen and world coordinates, picking and snapping](spec/viewport.md), independent of any renderer |
| sc-store | [Projects saved as crash-consistent commits of canonical text](spec/project-store.md), with migrations and autosave, on a directory or in memory |

These APIs can inspect authored content and refuse invalid current references. Unknown/derived scalar
inputs refuse numeric fallbacks. Some complete structural inventories can still contain unresolved
values. The [implementation-status annex](annexes/implementation-status.md) links each family to its
source, roadmap requirement and verification owner.

Local strict native/WASM checks and the [scoped numerical review](annexes/formula-irrational-review.md)
pass,including execution on one native/Chrome pair. Application/device coverage remains pending.
Cross-compiling libraries does not prove a working browser application. No new remote-run verdict is claimed.

Whole statement/recipe [literal normalization](annexes/formula-recipe-inputs.md) preserves input
metadata and global refusal context. The same normalized owners produce owned statement/recipe
identity bytes for comparison; [the example](annexes/formula-recipe-inputs.md#own-canonical-statement-and-recipe-identity)
shows alias equality and preserved order. Admitted scalar recipe execution is available;
geometry integration and project storage remain pending.

[Formula declaration metadata](annexes/formula-declarations.md) describes kinds, origins and the
contexts that supply reserved names. Immutable declarations retain canonical input identities and
actual recipe/geometry sources. These libraries can identify a declared kind without an export,
profile or size value. Initial namespaces and ordered name reads are available, along with
[operator/built-in signatures](annexes/formula-builtin-signatures.md), typed wanted rules and
[source-bearing call lookup](annexes/formula-call-lookup.md).
[Initial-scope expression checking](annexes/formula-wanted-signatures.md#bounded-product-expression-checking)
retains complete kinds, refusals and sourced dependencies.
[Current-statement checking](annexes/formula-name-scopes.md#check-the-actual-current-statement)
checks actual scoped operands and annotations. [Coupled static review](annexes/formula-static-validation.md#coupled-public-static-review)
verifies those interfaces against independent rules and the actual reference.
[Whole recipe acceptance](annexes/formula-checked-recipes.md) returns a complete immutable static
proof with ordered source dependencies. Independent static whole-recipe review is complete;
the separate scalar evaluator supplies numerical execution with explicit policies and limits.

[Canonical rational values](annexes/formula-rationals.md) can now be constructed and reduced exactly,
with full 128-bit component magnitudes, independent sign and a typed zero-denominator refusal.
All four rational operators now reduce before width checks and report typed zero/width refusals.
Completed numeric values now enforce exact length/area limits and nonnegative Count, with a
normalized-literal bridge. All four quantity operators now apply exact ppm scaling and result domains.
Immutable approximation-call source sets, symbolic T1 admission checks and sourced numeric/Boolean
operators/bindings, implemented numeric calls, exact tolerance comparisons, named assertion outcomes
and lazy conditional value control are available.
The scalar evaluator propagates actual executed-call sources through admitted syntax and bindings.
The [checked-runtime sequence](annexes/formula-execution-plan.md) names the bounded next slices and
completed scalar/source paths and remaining geometry dependencies;full runtime acceptance stays separate.
[Actual per-node certificates](annexes/formula-checked-nodes.md) now retain kinds, symbolic roles,
resolved source/callee identities and typed children in the same successful static owner.
Exact unary negation/square and all six numeric comparisons are available, preserving fractional
values and raw angle turns. Exact abs/min/max/clamp are available with typed argument/bound refusals.
Explicit round_to is available at exact same-kind steps. Binding value conversion now rounds once,
checks signed i64 storage and preserves Boolean kind. Correctly rounded sqrt(area/ratio), signed-length
hypot and signed/full-turn arc_length are available. Native sin/cos/atan/atan2 kernels and supplied-scalar adapters are available.
Explicit-resource tangent values,calls and expression/source/recipe/Design policies are available;scoped24-row/both-mode acceptance passes.
Admitted scalar AST/statement/recipe execution is available;geometry execution remains pending.
[Canonical length authored reads](annexes/formula-runtime-validation.md#canonical-length-authored-reads)
now preserve source/state beside exact values or observation/evaluation requests.
[Canonical general scalar content](spec/measurement-inputs.md#canonical-general-scalar-content)
also retains raw angle turns, typed scalar state/provenance and computation-only Area. The
[general scalar read adapter](annexes/formula-runtime-validation.md#general-scalar-authored-reads)
now verifies the supplied canonical target/kind and retains state. Full registry/context resolution
and complete Design readiness remain pending. [Scoped source execution](annexes/formula-source-proofs.md#execute-an-actual-selected-scalar-source)
now evaluates a selected available-scalar source and demanded Derived suppliers with original
read witnesses and unbound roots. Eight native contracts pass,including27 independent whole-source
reference cases. [Native source-backed recipes](annexes/formula-design-integration.md#execute-an-admitted-recipe-with-actual-source-plans)
now combine admitted source/recipe plans,earned unbound roots and real ordered let storage with
complete read/error views. Actual Design plans now execute after fresh whole held-input/current
admission and retain their exact containing root/model limits. Native6/Design5 focused contracts
and compiler guards pass;23 independent paired-mode cases and scoped scalar integration pass.
Complete Design readiness remains pending.
[Explicit tolerance contexts](annexes/formula-runtime-validation.md#explicit-tolerance-contexts)
now provide fixed numerical/geometric values and class-checked optional format/importer/physical
providers. Missing optional values refuse with their actual name/context label.
[Core size-context frames](annexes/formula-runtime-validation.md#core-size-context-frames) now read
structurally checked supplied ordinal/count/base values or named absence. The
[actual membership bridge](spec/size-membership.md#formula-size-context) now derives them from
immutable authored order/base after explicit pin matching. [Authored axes/breaks/composite inputs](spec/size-sets.md) and
[immutable profile intent](spec/size-set-transformations.md) are implemented. Structural coverage
does not prove numeric availability, physical truth or applied profile effects.

[Typed artifact requests](spec/artifact-requests.md) name explicit DXF/PLT/PDF/tech-pack inputs and the same
artifact-specific uncertainty inspection target. These declarations generate no files or closure.

[Core authorization](spec/command-scopes.md#core-authorization) checks trusted host identity/grants,
exact scopes and human-only approval permission. Transport authentication and handlers remain pending.

## Planned workflows

Recipe evaluation, geometric construction, grading, profiles, storage and crash recovery, the command
bus/API/MCP server, CLI and native/browser applications remain unimplemented as complete workflows.
The measurement/Ease/SizeSet structural library is reviewed at G1-SLICE.4d.2; it supplies
authored inputs/intent. Exporters, factory acceptance and production approval retain later proof
owners, human review and scoped evidence.

The detailed [model](spec/ontology.md), [command](spec/command-layer.md) and
[release](spec/release-contract.md) annexes specify intended behavior. A normative requirement in an
annex is not a claim that a feature ships today. The [supported-envelope matrix](spec/feature-matrix.md)
is the intended release scope, not a list of completed features.

## Reproduce the library checks

At later integration milestones, contributors can run these commands from the repository root:

```bash
make check
make wasm
make book
```

Per-feature checks follow the [focused-check policy](annexes/publication-checks.md#checks).
These broader commands verify their documented scope. Full gate status and the next task are recorded in the
repository's LIVE_STATUS.md and MEMORY.md. Feature changes update the book in the same commit.
For an introduction, begin with [the learning path](learn/design-to-pattern.md); for a specific
contract, use the [topic index](topic-index.md).
