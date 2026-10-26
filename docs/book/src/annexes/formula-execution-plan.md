# Joining the checked formula runtime

This is the bounded implementation sequence for G1-SLICE.5e.2, not an available whole evaluator.
The value-level APIs now compute exact operators, implemented numeric calls, named tolerance
comparisons, assertions and lazy conditional callbacks. The complete static proof APIs already
preserve normalized source owners and resolved declaration metadata. The next work joins them
without treating supplied value metadata as proof of an actual expression or statement.

## Existing contracts to reuse

| Boundary | Available contract | Execution must retain |
| --- | --- | --- |
| Normalized expression | Private flat arena, original source spans and structural edges | Exact node owner; no substitute syntax or recursive destruction |
| Checked expression | Actual normalized owner, per-node kinds/roles and ordered sourced name uses | Both branches passed static checks before a value/provider read |
| Checked statement | Exact let/assert operand proofs and actual one-based recipe position | Original name/annotation/class and available canonical identity |
| Checked recipe | Complete immutable statement proofs in authored order | No accepted prefix after a later static error |
| Sourced values | Exact operators, implemented calls, stored reads and comparison outcomes | All executed inputs, condition/taken sources, actual failure data |

The source APIs are crates/sc-core/src/recipe/checked.rs,
crates/sc-core/src/recipe/checked_statement.rs, crates/sc-core/src/recipe/checked_recipe.rs and
crates/sc-core/src/recipe/normalized.rs. The static checker computes intermediate kinds on a heap
stack and now retains [actual per-node certificates](formula-checked-nodes.md) beside the expression,
root kind and dependencies. Runtime code can consume the selected branch's real node proof.

## Next implementation slices

1. **Actual per-node certificates — .5e.2.3, implemented0275.** Each successful node retains its inferred kind and
   direct symbolic tolerance role in the checked owner. Child views must stay pinned to that owner
   and its original spans. Neither a foreign arena nor a supplied index may forge a certificate.
   Name dependencies, wanted-kind alternatives and first static error ordering stay unchanged.
   No value, input state, tolerance provider or geometry query enters this static phase.
2. **Bounded sourced expression execution — .5e.2.4, next.** Consume those node proofs in source order.
   Use the existing exact value kernels and real declaration-source adapters. A conditional checks
   both complete branch proofs, then evaluates only its condition and selected branch. All evaluated
   call operands contribute, including candidates or bounds whose numeric value is not returned.
   Errors retain the actual failing node, owning expression and available canonical identity.
3. **Whole declaration-order execution — .5e.2.5.** Consume the actual checked recipe. Every let
   evaluates its RHS before its single numeric binding conversion; only a successful binding becomes
   visible later. Boolean bindings stay Boolean. Assertions use their actual class/name and refuse
   false comparisons. Statement ordinals include assertions and come from the real proof owner.
4. **Atomic acceptance and replay — .5e.4.** A later runtime failure yields no accepted partial
   recipe, geometry or replacement result. Dependency validation must cover persisted corruption and
   actual derived-input edges before execution can recurse. Fresh replay starts at statement one;
   cache equivalence and full accepted-result identity are integration obligations.

These are implementation boundaries within one runtime. They add no language syntax, execution
authority or exception to static-before-value validation. A detached expression scope has no recipe
ordinal; a whole statement supplies its real ordinal. Neither invents unavailable context.

## Source and provider integration

Name reads use the actual resolved declaration, not a fresh name-only search that could find another
source. Canonical length/general scalar adapters retain source/state and return observation or
dependency-evaluation requirements. Known/Assumed/Preference values remain labelled beside their
numeric payloads. Derived inputs follow their sole authored formula; they do not become separately
authored or unlabelled cached values. Registry/currentness and evidence truth remain their existing
owners; a readable value is not approval or evidence-scope validation.

Reserved tolerances use the explicit context and its actual quantitative provider. Optional classes
remain absent until supplied. Size values come from actual pinned membership/order/base identity;
axes still await the D70 ruling. Static availability metadata never supplies a missing runtime value.

Geometry reads retain real point/edge references and the published operation/selector contracts
under .5f.3. Point coordinates need separate approximation-source sets. Direction normalization is
specific to dir; raw angle arithmetic and inverse principal answers keep their signed values. The
parameter/geometry error budget uses actual edge length, not an inferred bounding-box guarantee.

## Dependencies that still prevent full runtime closure

The numerical kernels cover rational arithmetic, quantization, roots, arc_length and certified sin/cos.
Tan and atan/atan2 still require the practical all-input resource strategy and implementations
owned by .5d.1.2.2.2 and .5d.3.2.2/.3. This plan selects no replacement math backend, precision cutoff or
caller placeholder. An admitted function with an unfinished implementation cannot be silently
reclassified as a language refusal or published as a canonical computed result.

Complete source/derived-input resolution, actual geometry selectors, statement execution and atomic
publication also remain open. Internal slices must state their supported scope. Public whole-runtime
acceptance remains pending until every required family has its real implementation and evidence.
API/MCP publication, artifact authority, browser execution and production approval retain their
separate roadmap owners.

## Verification during implementation

Each implementation slice gets one focused check of its changed behavior. Node certificates need
actual-owner/span/kind/role and foreign-node controls. Expression execution needs actual checked
branches, taken-only unknowns, operation order and sourced failures. Statement execution needs
once-rounded later reads, real assertion data and actual ordinals. These checks cannot be replaced
by a reference evaluator or a supplied-value callback demonstration.

Whole worked-book population, independent runtime oracles, replay/cache fault controls, geometry
integration and observed cross-platform byte identity remain .5g/.5e.4 milestones. Broad campaigns
and full CI follow the feature-delivery policy and required pre-push gate. Passing a focused slice
does not close those later obligations or the G1 gate.
