# Complete formula recipe proofs

> **Implemented:** sc-core recipe, G1-SLICE.5b.4a. A normalized recipe can return an immutable
> proof that every statement passes names and kinds in its actual prior-binding scope.
> Coupled whole-factory review is verified at .5b.4b. [Admitted scalar execution](formula-recipe-execution.md) is available;construction operations,
> persistence and physical approval retain their later owners.

Use this interface after complete [syntax and literal input checks](formula-recipe-inputs.md).
FormulaNormalizedRecipe::check_kinds consumes a checked FormulaNamespace and borrows the actual
normalized recipe. It returns FormulaCheckedRecipe only after every statement succeeds. Initial
namespace construction can refuse before this call; those admission errors have no recipe ordinal.

## Accept a complete ordered recipe

```rust
use sc_core::recipe::{FormulaNamespace, FormulaRecipe};
let recipe = FormulaRecipe::parse("let n:count=1\nassert check:eps_num=n==n")
    .unwrap().normalize_literals().unwrap();
let proof = recipe.check_kinds(FormulaNamespace::new([]).unwrap()).unwrap();
assert!(std::ptr::eq(proof.recipe(), &recipe));
assert_eq!(proof.statements().len(), 2);
assert_eq!(proof.dependencies().count(), 3);
assert_eq!(proof.canonical_recipe(), recipe.canonical_form());
```

The validator privately stages a cursor and statement proofs. It checks a header, then the
statement's scoped operands and annotation/comparison, then advances the successful annotation.
A failed let cannot supply a later statement. Self and future names remain unavailable; assertion
labels supply no scalar. A header collision refuses before its RHS is inspected. Existing
[call and child priorities](formula-name-scopes.md#check-the-actual-current-statement) remain intact,
including checks in both conditional branches.

statements() exposes the complete immutable ordered slice of FormulaCheckedStatement proofs.
Each retains its original normalized statement and genuine one-based position. The vector cannot
be replaced or mutated through this API. An empty recipe produces an empty complete proof.
Clone the initial namespace before the call if another check needs the same immutable metadata;
the private staging cursor never modifies that caller's reusable clone.

## Read ordered dependency occurrences

FormulaRecipeNameDependency has statement_index(), role(), span() and declaration() getters.
The consumer ordinal includes assertions. FormulaRecipeDependencyRole is a closed enum:

| Role | Original source |
| --- | --- |
| Binding | A name occurrence in a let's RHS |
| AssertionTolerance | The assertion's original symbolic tolerance header |
| AssertionLeft | A name occurrence in its left operand |
| AssertionRight | A name occurrence in its right operand |

The iterator visits statements in authored order. Within an assertion it yields the tolerance
header first, then left operand occurrences, then right operand occurrences. The header is genuine
metadata with its annotation span and fixed reserved declaration; it is not a fabricated expression
node. Repetitions and untaken-branch occurrences remain distinct. The iterator streams the existing
operand proofs, without allocating a duplicate whole edge vector.

The supplier's declaration retains its real initial input, reserved, geometry or prior-let source.
A prior-let source ordinal is strictly earlier than its consumer; this source language cannot create
a forward dependency cycle. An initial or reserved source has no recipe ordinal. Complete recipe
acceptance gives every prior annotation a successful check; a [local statement proof](formula-name-scopes.md)
alone cannot make that guarantee.

Unknown canonical inputs remain valid metadata, with no state read or numerical fallback:

```rust
use sc_core::{name::MachineToken, ontology::EntityId, recipe::*, value::*};
let width = MachineToken::new("width").unwrap();
let record = LengthDeclaration::new(LengthDeclarationDefinition {
    id: EntityId::from_bits(1), source: EntityId::from_bits(2),
    state: LengthState::Unknown { observation: EntityId::from_bits(3) },
}).unwrap();
let input = FormulaDeclaration::length_input(&width, FormulaInputOrigin::Measurement,
    EntityId::from_bits(4), &record);
let initial = FormulaNamespace::new([
    FormulaInitialDeclaration::try_from(input).unwrap(),
]).unwrap();
let recipe = FormulaRecipe::parse(
    "let doubled:length=if(is_base_size,width+width,width)\nassert check:eps_geo=doubled==width"
).unwrap().normalize_literals().unwrap();
let proof = recipe.check_kinds(initial).unwrap();
let uses: Vec<_> = proof.dependencies().collect();
assert_eq!(uses.iter().map(|d| d.declaration().name()).collect::<Vec<_>>(),
    ["is_base_size", "width", "width", "width", "eps_geo", "doubled", "width"]);
assert!(matches!(uses.get(1).unwrap().declaration().source(),
    FormulaDeclarationSource::LengthInput { declaration, .. }
    if std::ptr::eq(declaration, &record)));
assert!(matches!(uses.get(5).unwrap().declaration().source(),
    FormulaDeclarationSource::Recipe { statement_index: 1, .. }));
```

canonical_recipe() returns the authored normalized recipe's exact owned identity bytes.
Those bytes do not include the initial namespace's supplier identities. Equal recipe bytes with
different measurement records can therefore resolve different dependency sources. Inspect those
sources separately; canonical recipe bytes alone do not identify a resolved computation or release.
No graph fingerprint, storage identity or numeric dependency result is granted by this interface.

## Keep the actual first refusal

FormulaRecipeCheckError is privately constructed and retains recipe(), statement(),
statement_index(), span() and token(). FormulaRecipeCheckRefusal distinguishes Namespace (the
actual header collision) from Statement (the actual nested expression/annotation/comparison error).
Its read-only payload preserves existing typed arguments. The error exposes no successful prefix
or final namespace. A late refusal still retains the complete normalized recipe, including its suffix.

```rust
use sc_core::recipe::*;
let recipe = FormulaRecipe::parse(
    "let first:length=1 mm\nassert prior:eps_geo=first==first\nlet bad:length=1.0\nlet later:count=1"
).unwrap().normalize_literals().unwrap();
let result = recipe.check_kinds(FormulaNamespace::new([]).unwrap());
assert!(result.is_err());
let error = result.unwrap_err();
assert_eq!(error.statement_index(), 3);
assert_eq!(error.token(), "formula_dimension");
assert!(std::ptr::eq(error.recipe(), &recipe));
assert_eq!(error.span(), error.statement().annotation_span());
assert_eq!(error.canonical_recipe(), recipe.canonical_form());
assert!(matches!(error.refusal(), FormulaRecipeCheckRefusal::Statement(nested)
    if matches!(nested.refusal(), FormulaStatementCheckRefusal::BindingDimension(args)
        if args.expression_kind() == FormulaKind::Ratio && args.wanted_kind() == FormulaKind::Length)));
```

Header collisions use the original binding name span. Other failures retain the existing actual
statement error span: annotation for a let mismatch, nested node for an expression refusal, and
whole statement for the assertion comparison. No comparison separator span or combined AST is
invented. canonical_statement() and canonical_recipe() are explicit identity requests from the
actual owner, even after rejection; these requests do not certify acceptance.

Reserved-name refusals keep formula_rebinding and real binding sources, including metadata with
no ordinal. An initial-name collision uses formula_ambiguous_name. Earlier-let rebinding retains
both genuine recipe positions. D131's diagnostic repair changes no grammar:

```rust
use sc_core::recipe::*;
let recipe = FormulaRecipe::parse(
    "let first:count=1\nassert prior:eps_num=1==1\nlet eps_num:length=missing"
).unwrap().normalize_literals().unwrap();
let result = recipe.check_kinds(FormulaNamespace::new([]).unwrap());
assert!(result.is_err());
let error = result.unwrap_err();
assert_eq!(error.token(), "formula_rebinding");
assert_eq!(error.statement_index(), 3);
assert!(matches!(error.refusal(), FormulaRecipeCheckRefusal::Namespace(collision)
    if matches!(collision.binding_sources()[0].source(), FormulaDeclarationSource::Reserved(_))
    && matches!(collision.binding_sources()[1].source(),
        FormulaDeclarationSource::Recipe { statement_index: 3, .. })));
```

Proofs and errors borrow their normalized owner and any canonical source records. Keep those
owners alive while inspecting them; they cannot escape a local recipe or local input record.
Default Debug omits authored source; error Display contains only the stable diagnostic token.
Explicit source/name/canonical getters deliberately expose authored content to the caller.

## Verification and remaining authority

The reference copied-book consumer rechecks the whole book/loaders and then its effective
selectors before shell delegation. The selector plan contains the four selected files,every
parts file and direct local links from the contract,grammar and examples. Its current default
population is24,including docs/book/src/annexes/formula-rationals.md and formula-recipe-execution.md.
The latter is a direct dependency of the language contract. The capture oracle retains
strict set equality;the public shell's actual L7 resolver and producer independently agree on
that population. Removing only the linked execution chapter from the actual final plan is refused.
All23 earlier oracle paths and the original capture/guard/classifier statements stay exact.

The guard calibration passes six intercepted dispatches,pure196,thirteen guard negatives,
fourteen physical refusals and owned no-effect admission controls. Its authored child metrics
remain separate from the real reference consumer:the actual21-statement replay and25-statement
measurement ceiling pass,and11 original reference faults plus2 consumer/1 measurement faults
produce14 actual assertion refusals. These are reference-consumer proofs,not product execution.

The [execution sequence](formula-execution-plan.md) joins these actual static owners to sourced value
primitives. [Actual per-node certificates](formula-checked-nodes.md) are available; expression/statement
execution is available for the [admitted scalar subset](formula-recipe-execution.md). [Whole-proof runtime admission](formula-recipe-execution.md#whole-checked-recipe-admission)
now reserves complete populations before runtime preparation. [Whole scalar source preparation](formula-recipe-execution.md#prepare-every-whole-recipe-scalar-source)
binds actual inventories/reserved roles and earlier checked suppliers without values. Neither
static proof,resource admission,source preparation nor supplied callbacks execute a whole recipe.

```bash
cargo test -p sc-core --test formula_checked_recipe_contract -- --nocapture
python3 -I -B docs/tasks/artifacts/formula_structure/recipe_owner_contract.py
python3 -I -B docs/tasks/artifacts/formula_structure/checked_recipe_mutations.py
```

Public contracts cover ordered bindings/assertions, complete source occurrences, private owner/
borrow boundaries and bounded traversal/drop. Historical counts, compiler guards, fault sets,
dispatch controls and exact source/artifact restoration receipts remain in
`docs/history/stitchcad-devnotes-part516.md`, retrievable through history_archive.py read.
The native/documentation commands above remain maintained; classifier-only mode is also available:

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/checked_recipe_mutations.py --classifier-only
```

Classifier-only plans no native child/source/output writes. Run mutations alone and consume exact
source restoration before another build/probe. Body failures must identify current assertion macros;
dispatch controls and compiled native proof retain separate scopes. Existing D154's known reference
ordinal gap remains live below, with its original repair owner; this retirement does not close it.

A statically valid false assertion, zero divisor, negative square root, absent size/export context
or too-wide stored result still has no execution verdict. The proof reads no values, providers or
geometry and cannot approve those cases. Coupled whole-factory review closes .5b.4b; exact arithmetic
and binding are .5c, irrational algorithms .5d, execution/replay .5e and operations .5f. Persistence,
physical geometry, export readiness and human approval retain their separate roadmap owners.

The [coupled static review](formula-static-validation.md#coupled-public-static-review) checks26723
shared cases, including13967 actual whole-recipe invocations, all published statements/refusals,
source-derived complete dependency occurrences and62 selected refusal contexts. Six real reference
and28 real Rust faults (19 whole factory) discriminate; sources and the current artifact restore
before later checks. All expression cases also run in complete recipes; their independent expected
outcomes never come from product output. Numerical and physical authority remains deferred.

D154 tracks the reference's missing available whole-refusal ordinal. The Rust error retains its
actual ordinal; the reference's static-call trace observes it but its propagated FErr arguments
omit it. Repair .5b.4c is required before static-lane closure. No missing reference context is
fabricated in this comparison, and this gap grants no numerical or physical authority.


## Root sources in the actual mixed scope

The mixed programme's [actual initial namespace](formula-declarations.md#actual-mixed-program-initial-namespace)
now exposes `check_root_sources(FormulaSourceCheckLimits)`. It uses that exact programme's complete
normalized Root source corpus and private initial map. Callers supply only the existing explicit
source policy;they cannot substitute another namespace,inventory,source corpus or accepted proof.
This path includes actual bound extra Profile/Material/geometry metadata in addition to Root names;
it does not rebuild the older allocating Design namespace factory.

The bridge invokes the original `FormulaNormalizedSources::check_sources()` factory. Its complete
source count,checked arithmetic,node count,Derived-target count and retained-proof bounds precede
proof reservation and checking. Every source,including unused bodies and all branches,is checked.
Original canonical input joins and actual Derived consumer root-kind checks remain that factory's
rules. No metadata revalidation,source callback,geometry query,authored/computed value or namespace
clone is introduced. Previously admitted held-content/name/source-association limits remain required.

For example,the actual extra names `private_count` and `private_length` borrow an Unknown Count
and an Unknown dedicated Length from this Root. Source9003 is `private_count+1`;source9004 is the
unused `private_length`. A real canonical Count103 derives from9003. The complete existing native
requirements are two sources,four nodes,one Derived target and **2 + 2*4 =10** retained proof records.
Allowances1 source,3 nodes,0 targets or9 records retain their original complete required2/4/1/10
refusals in that order. Replacing unused9004 by `private_missing` still returns its original source
index1/expression cause. Replacing9003 by `private_length` returns the actual Count consumer's
TargetKind refusal with expected Count/actual Length. Neither Unknown input is read or defaulted.

`CheckedDraftingProgramSources` privately retains the exact original `BoundDraftingProgramInitials`,
actual `DesignNormalizedFormulaSources` object and complete original `FormulaCheckedSources`.
`inputs()`,`normalized()` and `proof()` expose those owners,all original source/consumer proofs and
the selected native policy. Equal model IDs or independently normalized equal text cannot replace
the retained actual objects. The proof borrows original declarations/arenas,not the namespace map
allocation. It therefore remains usable while subsequent mixed checking consumes that map:

```rust
use sc_core::{construction::{DraftingProgramInitialNamespace,DraftingInitialInputProvider,
    DraftingProgramOrderLimits},design::DesignMetadataHost,recipe::FormulaSourceCheckLimits};
fn source_and_step_counts<H:DesignMetadataHost,S:DraftingInitialInputProvider<H> + ?Sized>(
    names:DraftingProgramInitialNamespace<'_, '_, '_, '_, '_, '_, '_,H,S>,
    source_policy:FormulaSourceCheckLimits,order_policy:DraftingProgramOrderLimits)
    ->Result<(usize,usize),String> {
    let sources=names.check_root_sources(source_policy).map_err(|error|error.to_string())?;
    let order=names.check_order(&order_policy).map_err(|error|error.to_string())?;
    Ok((sources.proof().sources().len(),order.steps().len()))
}
```

`DraftingProgramSourceCheckError` retains those same original associations and normalization object,
selected limits and the original native source error. `cause()` preserves quota/allocation,
expression/canonical and actual consumer-kind witnesses. The original allocator cause stays inline;
this adapter creates no diagnostic box after reservation failure. Default diagnostics omit authored
names/source payloads,while explicit native cause inspection exposes its original typed arguments.
No successful source prefix is returned. An empty actual source/Derived-target scope accepts zero
native source allowances and still retains its actual containing Root,even with a nonempty recipe.

Focused native checks cover actual extra-name loans,consumer kind,unused source error,all four quota
priorities,empty scope and both source/order proofs surviving map consumption. Public lifetime and
private-field guards also pass. This implements the Root source bridge;canonical dependency/cycle
composition,mixed canonical joins,geometry/current frame/provenance,semantic dependencies and
combined allocator/lifetime verification remain owned by the next dependency stages.


## Select exact mixed dependency owners

`CheckedDraftingProgramSources::dependency_inputs(order,acyclic_graph)` now joins the actual
source,mixed-order and native acyclic proof objects before mixed canonical work. Build the graph
with the existing `sources.proof().prepare_dependencies(explicit_dependency_limits)` and
`dependencies.admit_acyclic(explicit_cycle_limits)` factories. Their complete source/edge/scratch
bounds and original cycle/allocator causes remain unchanged;the new join duplicates no graph.

The join checks two exact identities,in this order:

1. `order.inputs()` must be this source wrapper's actual `BoundDraftingProgramInitials` object.
2. `acyclic_graph.dependencies().checked()` must be this wrapper's exact native `proof()` object.

Two independent initial admissions can contain the same actual programme/names/snapshot and
still be different selected proof owners. Two source checks can use the same initial associations,
normalization object and policy and still produce different native proof objects. A graph from the
second source check cannot substitute for the first. Equal Root IDs/revisions,text,corpus or empty
graph shapes supply no owner identity. Source-graph mismatch is examined only after order-input
identity succeeds.

```rust
use sc_core::{construction::{CheckedDraftingProgramSources,CheckedDraftingProgramOrder,
    DraftingInitialInputProvider},design::DesignMetadataHost,recipe::FormulaAcyclicSourceDependencies};
fn selected_sources<'j,'i,'r,'b,'n,'p,'a,H:DesignMetadataHost,S:DraftingInitialInputProvider<H> + ?Sized>(
    sources:&'j CheckedDraftingProgramSources<'j,'i,'r,'b,'n,'p,'a,H,S>,
    order:&'j CheckedDraftingProgramOrder<'j,'i,'r,'b,'n,'p,'a,H,S>,
    graph:&'j FormulaAcyclicSourceDependencies<'j>)->Result<usize,String> {
    let inputs=sources.dependency_inputs(order,graph).map_err(|error|error.to_string())?;
    Ok(inputs.sources().proof().sources().len())
}
```

`DraftingProgramDependencyInputs` privately retains all three original proof references. Its
`sources()`,`order()` and `graph()` getters preserve the actual normalized/input owners and every
original source/order/dependency/cycle policy. The join allocates and queries nothing,reads no
values and supplies no new work allowance. It cannot outlive those selected proofs. Empty source
and acyclic scopes still require the actual selected owner objects.

A refusal keeps every supplied object in `DraftingProgramDependencyOwnerError`,including a foreign
order or graph. `issue()` reports OrderInputs or SourceGraph;the three getters expose the actual
provided witnesses. Default diagnostics omit authored names and source payloads. No accepted
source/order/graph prefix is returned and no fallback owner is chosen.

Focused checks discriminate independently recomputed source proofs and initial associations over
the same actual data,plus equal-looking foreign Roots. They preserve exact provided pointers,
original policies and first-mismatch precedence without callbacks. Public lifetime/private-field
checks pass. This establishes selected provenance;streaming original mixed dependency occurrences,
complete additional bounds/canonical joins,geometry/semantic bindings and whole feature verification
remain later owned stages. The acyclic source relation alone is no whole operation graph or execution.
