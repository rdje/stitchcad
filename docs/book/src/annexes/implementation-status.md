# Annex: implementation status and requirement owners

This is the technical companion to [availability](../availability.md). It maps the current foundation
families to roadmap requirements, public code and task-owned verification. These are structural/local
integration claims. None establishes generated geometry, physical fit, factory acceptance or release.
The implementation rows are cross-checked by the book publication instrument; semantic signoff still
requires the named contracts and later proof owners.

| Family | Roadmap | Public source and verification | Book contract |
| --- | --- | --- | --- |
| Units/tolerances | §4.2; G0/G1 | sc-units; G1-SLICE.2/.5a.3c.1 | [Numerical contract](../spec/units-and-tolerances.md), [wide rounding](numeric-rounding.md) |
| Declared command catalog | §4.4/§7.8; G1 | sc-core command; G1-SLICE.6.1 | [Typed discovery](../spec/command-layer.md#implemented-typed-catalog); handlers/authority execution follow |
| Mutation request metadata | §4.4/§10; G1 | sc-core command; G1-SLICE.6.2.2.a | [Actor/revision/retry/group context](../spec/command-layer.md#implemented-mutation-context); validation/execution follows |
| Initial mutation payloads | §4.4; G1 | sc-core command; G1-SLICE.6.2.2.b.1 | [Measurement/material intents](../spec/command-layer.md#implemented-initial-mutation-payloads); registry/auth/handlers follow |
| Identity/references | §4.1; G1 | sc-core ontology; G1-SLICE.3a/.3b | [Model and current repairs](../spec/ontology-implementation.md) |
| Pieces/copies, sewing, marks/allowances | §3.1/§4.1; G1 | sc-core ontology; G1-SLICE.3c.1–.3 | [Executable ontology](../spec/ontology-implementation.md) |
| Garment constructions/closures | §3.1; G1 | sc-core ontology; G1-SLICE.3c.4 | [Constructions](../spec/ontology-constructions.md), [closures](../spec/ontology-closures.md), [review](../spec/ontology-review.md) |
| Canonical length/state/source | §2/§8; G1 | sc-core value; G1-SLICE.4a.1 | [Length inputs](../spec/measurement-inputs.md) |
| Measurement metadata/tables | §3.1/§3.4; G1 | sc-measure; G1-SLICE.4a.2/.3 | [Measurements](../spec/measurement-metadata.md) |
| Ease mappings/sets | §3.1/§3.3; G1 | sc-measure; G1-SLICE.4b | [Ease](../spec/ease-inputs.md) |
| Size membership | §3.4; G1 | sc-measure; G1-SLICE.4c.1 | [Membership](../spec/size-membership.md) |
| Garment chart observations/collections | §3.3/§3.4; G1 | sc-measure; G1-SLICE.4c.3a/.3b | [Observations](../spec/size-chart-observations.md), [collections](../spec/size-chart-collections.md) |
| MTM inputs | §3.3/§3.4; G1 | sc-measure; G1-SLICE.4c.3c | [MTM introduction](../spec/mtm-input-charts.md), [API](mtm-input-contract.md) |
| Formula literal inputs/normalized arenas | §4.1; G1 | sc-core recipe; G1-SLICE.5a.3c.2/.3 | [Literal normalization](formula-literals.md) |
| Canonical expression identity | §4.1; G1 | sc-core recipe; G1-SLICE.5a.3d.2/.3 | [Owned canonical bytes](formula-literals.md#serialize-canonical-expression-identity) |
| Single formula statement syntax | §4.1; G1 | sc-core recipe; G1-SLICE.5a.3e.1 | [Statement API](formula-statements.md) |
| Ordered formula recipe syntax | §4.1; G1 | sc-core recipe; G1-SLICE.5a.3e.2 | [Recipe API](formula-statements.md#parse-an-ordered-recipe) |
| Complete recipe literal inputs | §4.1; G1 | sc-core recipe; G1-SLICE.5a.3f.1b | [Normalized statements/recipes](formula-recipe-inputs.md) |
| Owned statement/recipe identity | §4.1; G1 | sc-core recipe; G1-SLICE.5a.3f.1c | [Exact owned bytes](formula-recipe-inputs.md#own-canonical-statement-and-recipe-identity) |
| Formula lexing/expression syntax | §4.1; G1 | sc-core recipe; G1-SLICE.5a.1/.2b.2 | [Syntax API](formula-syntax.md) |
| Formula kind/origin/reserved metadata | §4.1; G1 | sc-core recipe; G1-SLICE.5b.2a | [Declaration metadata](formula-declarations.md) |
| Immutable formula source declarations | §4.1; G1 | sc-core recipe; G1-SLICE.5b.2b | [Sourced declarations](formula-declarations.md#immutable-sourced-declarations) |
| Initial formula namespace/read | §4.1; G1 | sc-core recipe; G1-SLICE.5b.2c.2/.2d.1 | [Admission, collisions and reads](formula-declarations.md#exact-declared-name-reads) |
| Ordered formula name scope | §4.1; G1 | sc-core recipe; G1-SLICE.5b.2d.2 | [Actual prior bindings](formula-name-scopes.md) |
| Formula operator kind signatures | §4.1; G1 | sc-core recipe; G1-SLICE.5b.3a | [Closed operator matrix](formula-operator-signatures.md) |
| Formula built-in/selector signatures | §4.1; G1 | sc-core recipe; G1-SLICE.5b.3b | [Closed calls and roles](formula-builtin-signatures.md) |
| Initial-scope expression proof | §4.1; G1 | sc-core recipe; G1-SLICE.5b.3c.2b.2 | [Bounded kind checking](formula-wanted-signatures.md#bounded-product-expression-checking) |
| Current-statement kind proof | §4.1; G1 | sc-core recipe; G1-SLICE.5b.3c.3b | [Actual scope checking](formula-name-scopes.md#check-the-actual-current-statement) |
| Complete recipe kind proof | §4.1; G1 | sc-core recipe; G1-SLICE.5b.4a | [Atomic static acceptance](formula-checked-recipes.md) |
| Canonical rational values and four operators | §4.2; G1 | sc-core recipe; G1-SLICE.5c.1.1/.2 | [Exact value kernel](formula-rationals.md); typed execution follows |
| Exact numeric values/operators/binding conversion | §4.2; G1 | sc-core recipe; G1-SLICE.5c.2/.3 | [Exact and stored values](formula-rationals.md); full execution follows |
| Correctly rounded sqrt/hypot | §4.2; G1 | sc-core recipe; G1-SLICE.5d.2.1/.2 | [Exact root values](formula-rationals.md#correctly-rounded-sqrt-and-hypot); source provenance available |
| Correctly rounded arc_length | §4.2; G1 | sc-core recipe; G1-SLICE.5d.3.1 | [Certified arc values](formula-rationals.md#bounded-correctly-rounded-arc_length); source provenance available |
| Correctly rounded sin/cos | §4.2; G1 | sc-core recipe; G1-SLICE.5d.3.2.1 | [Certified trigonometric values](formula-rationals.md#correctly-rounded-sine-and-cosine); sourced calls available |
| Correctly rounded atan/atan2 | §4.2; G1 | sc-core recipe; G1-SLICE.5d.3.3.1 | [Native inverse values](formula-trig-certificates.md#native-inverse-values); sourced calls available |
| Formula wanted-kind catalogs | §4.1; G1 | sc-core recipe; G1-SLICE.5b.3c.1 | [Typed expected rules](formula-wanted-signatures.md) |
| Canonical length authored-read adapter | §2/§4.1; G1 | sc-core recipe; G1-SLICE.5e.1.1 | [Source/state reads](formula-runtime-validation.md#canonical-length-authored-reads); full execution follows |
| Canonical scalar state/value content | §2/§4.1; G1 | sc-core value; G1-SLICE.5e.1.2 | [Scalar content](../spec/measurement-inputs.md#canonical-general-scalar-content); registry/evaluation follows |
| General scalar authored-read adapter | §2/§4.1; G1 | sc-core recipe; G1-SLICE.5e.1.3 | [Actual target/kind reads](formula-runtime-validation.md#general-scalar-authored-reads); registry/context execution follows |
| Explicit reserved tolerance contexts | §3.1/§4.1; G1 | sc-core recipe; G1-SLICE.5e.1.4 | [Intrinsic/optional providers](formula-runtime-validation.md#explicit-tolerance-contexts); source approval/execution follows |
| Core size-context frames/reads | §3.1/§4.1; G1 | sc-core recipe; G1-SLICE.5e.1.5.1 | [Supplied source/range values](formula-runtime-validation.md#core-size-context-frames); actual membership bridge below |
| Authored membership to size context | §3.1/§3.4; G1 | sc-measure membership; G1-SLICE.5e.1.5.2 | [Exact pin/member/order bridge](../spec/size-membership.md#formula-size-context); axes/global registry follow |
| Canonical/stored numeric reads | §4.2; G1 | sc-core recipe; G1-SLICE.5e.1.6 | [Exact integer bridges](formula-rationals.md#canonical-and-stored-numeric-reads); source propagation follows |
| Approximation-source set/class guard | §4.2; G1 | sc-core recipe; G1-SLICE.5e.3.1 | [Immutable sources/T1 refusal](formula-runtime-validation.md#product-source-set-primitive); full propagation follows |
| Sourced scalar operators/bindings | §4.2; G1 | sc-core recipe; G1-SLICE.5e.3.2 | [Numeric/Boolean sources and reads](formula-runtime-validation.md#product-scalar-operators-and-bindings); AST execution follows |
| Sourced implemented numeric calls | §4.2; G1 | sc-core recipe; G1-SLICE.5e.3.3/.6 | [Actual kernels/input unions](formula-runtime-validation.md#product-implemented-numeric-calls); tangent/inverse/AST follow |
| Exact named tolerance comparison | §3.1/§4.2; G1 | sc-core recipe; G1-SLICE.5e.3.4 | [Actual values/context/sources](formula-runtime-validation.md#product-named-tolerance-comparisons); assertion/AST execution follows |
| Named assertion value outcomes | §5.1/§5.2; G1 | sc-core recipe; G1-SLICE.5e.3.5 | [Actual true/failure records](formula-runtime-validation.md#product-named-assertion-outcomes); whole statement execution follows |
| Lazy conditional value control | grammar§7; G1 | sc-core recipe; G1-SLICE.5e.2.1 | [Selected callback/source retention](formula-runtime-validation.md#product-lazy-conditional-value-control); full AST proof/execution follows |
| Actual per-node static certificates | §4.1/grammar§7; G1 | sc-core recipe; G1-SLICE.5e.2.3 | [Pinned kinds/roles/sources](formula-checked-nodes.md); expression/statement execution follows |

Reference [static signature evidence](formula-static-validation.md) covers the complete finite
kind/function matrix without executing values. Reference namespace/preflight/full static review
is verified at G1-SLICE.5b.1; library-owned whole-recipe acceptance is implemented at .5b.4a.
product metadata, sourced declarations, initial namespaces, exact reads and ordered metadata scopes are available;
Operator/built-in/selector signatures, typed wanted catalogs and initial-scope expression proofs
and current-statement checks are available. The [coupled review](formula-static-validation.md#coupled-public-static-review)
verifies their public behavior and the actual reference; [whole recipe proofs](formula-checked-recipes.md) are available; coupled whole review is verified at .5b.4b.

## Remaining proofs

The [trig threshold certificates](formula-trig-certificates.md) cover all1M forward and45M inverse
targets with exact full readers. Sin/cos and atan/atan2 value kernels are implemented;
tangent's separate resource plan and implementation remain open.

Full SizeSet axes/breaks/composition and resolved profile intent remain G1-SLICE.4c work. D70's axes
cardinality ruling is pending. Recipe evaluation is .5, bus/undo .6, storage .7, constraints .8,
API/MCP .9 and CLI .10; browser/runtime/canvas/license proofs and full G1 exit remain later children.
The current ontology supports authored operation identities; that is not an executed recipe.

G2 validates geometry and the reference-skirt CLI/export proof. G3 executes garment constructions,
grading and the declared envelope. G4 validates profile/evidence/policy behavior. G5 proves UI/API/MCP
workflow parity and real UI/i18n behavior. G6 validates external receivers and reliability, and G7 owns
independent review and a scoped production declaration. V1 assembly and V2 simulation retain separate
tracks; neither is provided by the current libraries.

`make check` verifies strict library tests. `make wasm` verifies cross-compilation only. Book/topic
navigation, source/rendered local links, chapter/index coverage and the scoped status map are checked
by `docs/tasks/artifacts/book_publication/run_book_publication_probes.sh` from the repository root.
The map does not prove all semantics by matching API names: canonical tests and explicit deferred
requirements remain necessary. Test counts in historical review chapters are dated review snapshots;
current counts are derived from current test execution rather than inferred from those chapters.
