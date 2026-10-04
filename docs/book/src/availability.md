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

Strict native tests and the three-library WASM cross-compilation are checked locally. Cross-compiling
libraries is not evidence of a working browser application. Older metadata CI has been observed;
there is no new remote-run claim for later local slices.

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
exact arithmetic, binding and execution remain pending.

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

Contributors can run these commands from the repository root:

```bash
make check
make wasm
make book
```

They verify their documented scope. Full gate status and the next task are recorded in the
repository's LIVE_STATUS.md and MEMORY.md. Feature changes update the book in the same commit.
For an introduction, begin with [the learning path](learn/design-to-pattern.md); for a specific
contract, use the [topic index](topic-index.md).
