# What you can use today

StitchCAD currently provides three Rust libraries. They implement numerical and structural
foundations; they do not yet generate a garment pattern or provide an end-user application.
G0 contract review is mostly complete with human closure unapproved. G1 remains in progress.

## Implemented foundations

| Library | Available behavior |
| --- | --- |
| sc-units | Fixed-point units, explicit conversions, rounding and typed tolerances/errors |
| sc-core | Stable identities/references, structural garment objects, physical copy identities, canonical length inputs, borrowed formula lexing, bounded expression/ordered-recipe syntax, whole-recipe literal normalization and owned expression/statement/recipe identity |
| sc-measure | Body/garment metadata and tables, Ease mappings/sets, size membership and authored garment/MTM charts |

These APIs can inspect authored content and refuse invalid current references. Unknown/derived scalar
inputs refuse numeric fallbacks. Some complete structural inventories can still contain unresolved
values. The [implementation-status annex](annexes/implementation-status.md) links each family to its
source, roadmap requirement and verification owner.

Earlier slices have local strict native/WASM evidence. New rational values have one focused
native contract target; broader/platform verification is deferred. Cross-compiling libraries does
not prove a working browser application. No new remote-run verdict is claimed.

Whole statement/recipe [literal normalization](annexes/formula-recipe-inputs.md) preserves input
metadata and global refusal context. The same normalized owners produce owned statement/recipe
identity bytes for comparison; [the example](annexes/formula-recipe-inputs.md#own-canonical-statement-and-recipe-identity)
shows alias equality and preserved order. Binding, numerical execution and project storage follow.

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
proof with ordered source dependencies. Independent whole-factory review is complete;
remaining numeric functions and recipe execution remain pending.

[Canonical rational values](annexes/formula-rationals.md) can now be constructed and reduced exactly,
with full 128-bit component magnitudes, independent sign and a typed zero-denominator refusal.
All four rational operators now reduce before width checks and report typed zero/width refusals.
Completed numeric values now enforce exact length/area limits and nonnegative Count, with a
normalized-literal bridge. All four quantity operators now apply exact ppm scaling and result domains.
Exact unary negation/square and all six numeric comparisons are available, preserving fractional
values and raw angle turns. Exact abs/min/max/clamp are available with typed argument/bound refusals.
Explicit round_to is available at exact same-kind steps. Binding value conversion now rounds once,
checks signed i64 storage and preserves Boolean kind. Correctly rounded sqrt(area/ratio), signed-length
hypot and signed/full-turn arc_length are available. Trigonometric/inverse functions and complete
statement/recipe execution remain pending.
[Canonical length authored reads](annexes/formula-runtime-validation.md#canonical-length-authored-reads)
now preserve source/state beside exact values or observation/evaluation requests.
[Canonical general scalar content](spec/measurement-inputs.md#canonical-general-scalar-content)
also retains raw angle turns, typed scalar state/provenance and computation-only Area. The
[general scalar read adapter](annexes/formula-runtime-validation.md#general-scalar-authored-reads)
now verifies the supplied canonical target/kind and retains state. Full registry/context resolution
and derived evaluation remain pending.
[Explicit tolerance contexts](annexes/formula-runtime-validation.md#explicit-tolerance-contexts)
now provide fixed numerical/geometric values and class-checked optional format/importer/physical
providers. Missing optional values refuse with their actual name/context label.
[Core size-context frames](annexes/formula-runtime-validation.md#core-size-context-frames) now read
structurally checked supplied ordinal/count/base values or named absence. Derivation from actual
authored membership/pin remains the next bridge; axes and full SizeSet proof remain pending.

## Planned workflows

Recipe evaluation, geometric construction, grading, profiles, storage and crash recovery, the command
bus/API/MCP server, CLI and native/browser applications remain unimplemented as complete workflows.
Complete SizeSet composition is still partial work; no axis representation is selected while its
specification decision is pending. Exporters, factory acceptance and production approval remain later
proofs, with human review and scoped evidence required.

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

Per-feature checks follow the [focused-check policy](annexes/checking-policy.md).
These broader commands verify their documented scope. Full gate status and the next task are recorded in the
repository's LIVE_STATUS.md and MEMORY.md. Feature changes update the book in the same commit.
For an introduction, begin with [the learning path](learn/design-to-pattern.md); for a specific
contract, use the [topic index](topic-index.md).
