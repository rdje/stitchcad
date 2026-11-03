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
private-field guards also pass. This implements the Root source bridge. The following sections compose the selected acyclic graph
and bounded mixed canonical joins. Geometry/current frame/provenance,semantic dependencies and
combined allocator/lifetime verification remain separate owned stages.


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


## Stream original mixed dependency occurrences

`CheckedDraftingProgramOrder::dependencies()` now streams every original checked scalar/native
name use in mixed order. It allocates no whole edge vector and reads no values or source provider.
The fused iterator visits repeated names and untaken conditional branches separately. Native steps
without scalar arguments contribute no occurrence;their geometry/semantic dependencies remain
separate. Empty programmes and programmes containing only literals retain an empty stream.

`FormulaCheckedStatement::dependencies()` exposes the existing whole-recipe statement producer.
The mixed stream reuses it,so a let emits its RHS uses and an assertion emits its real tolerance
header first,then left/right operand uses. The tolerance keeps its actual annotation span and fixed
reserved declaration;it is not an invented expression node. A standalone current-statement proof
still grants only its local checked scope,while the mixed owner supplies the complete ordered proof.

Native argument order follows actual checked fields:Point X then Y;center-arc Radius,StartAngle,
SweepAngle;slash/spread opening. Private exhaustive proof-field views guard those operands,so an
added checked argument cannot silently disappear. Each name use comes from the original operand
proof's dependency slice. No normalized source is reparsed,cloned,extracted or renumbered.

```rust
use sc_core::{construction::{CheckedDraftingProgramOrder,DraftingInitialInputProvider},design::DesignMetadataHost};
fn name_uses<H:DesignMetadataHost,S:DraftingInitialInputProvider<H> + ?Sized>(
    order:&CheckedDraftingProgramOrder<'_, '_, '_, '_, '_, '_, '_,H,S>)->usize {
    order.dependencies().count()
}
```

`DraftingProgramNameDependency` is a private consumer-pinned view. `step()` returns the actual
`DraftingProgramOrderStep`;`position()` is its genuine mixed position. `statement_index()` returns
the genuine scalar ordinal or None for a native argument. `role()` distinguishes Scalar's existing
Binding/AssertionTolerance/Left/Right roles,Point coordinate,Arc argument and SlashSpread opening.
`span()` and `declaration()` preserve the original source location and supplier metadata. Copying a
view copies only loans/metadata;it cannot outlive the actual complete consumer proof. Default Debug
shows position,ordinal,role and span while omitting authored names and source payloads.

For the earlier seven-step example,the six name uses are `width,anchor,second,eps_geo,second,width`
at mixed positions `2,3,5,7,7,7`. Their scalar ordinals are `None,2,None,3,3,3`:the assertion header
has scalar ordinal3,not its mixed position7. A dedicated repeated-Point fixture emits13 uses across
both conditional branches,X/Y,all three Arc fields,Slash opening,the repeated Point and the three
assertion roles. Its borrowed spans select the original name bytes in each native document or the
whole Root recipe. No field is deduplicated or assigned a synthetic ordinal.

Focused checks cover all argument roles,header/operand order,repeated/untaken uses,original spans,
actual consumer pointer identity,nth traversal and repeated end reads. Existing mixed-order checks
and private-field/lifetime examples pass. These views establish original dependency occurrences. The bounded canonical stage below joins
actual scalar inputs;geometry/semantic source binding and coupled verification remain separate.
Supplier metadata alone does not grant source validity.


## Bind the complete mixed canonical dependency stream

`DraftingProgramDependencyInputs::bind_canonical_inputs(&limits)` now binds every original
Input/LengthInput occurrence to the actual Root canonical inventory. It accepts only the already
selected source/order/acyclic owners. It cannot accept a caller namespace,canonical inventory,
source id or independently rebuilt proof as a substitute.

Choose both `DraftingProgramCanonicalLimits::new(occurrences,retained_records)` allowances
explicitly. There is no default. Let D be every original mixed dependency occurrence and C be
those whose actual declaration source is Input or LengthInput. Assertion tolerance headers,
prior recipe bindings,reserved names and Point/Edge metadata all contribute to D;repeated names
and both conditional branches remain separate. The additional logical record bound is D+C:
D result slots and C canonical associations,including their optional borrowed Derived relation.
These allowances cover no content/name/order/source/graph buffers already admitted by earlier
policies,and grant no allocator bytes,capacity,scratch,CPU or runtime budget.

The factory completes checked arithmetic for D,C and D+C first. It then checks occurrences,
then retained records,before reserving the full D-slot result vector and attempting any native
canonical join. No source/provider callback or authored numeric read occurs. The first refusal
returns the original complete inputs and selected policy,never a successful binding prefix.

Each canonical occurrence reuses `FormulaCanonicalInput::resolve` against the actual Root inputs.
Its existing family,identity,kind and dedicated Length-owner checks remain authoritative. Unknown
is retained as Unknown;it needs observation when a later value boundary actually reads it. Derived
matching reuses the native `FormulaSourceInputs::derived_inputs()` iterator and actual declaration
pointer,then retains that original `FormulaDerivedInput` and the exact `FormulaCheckedSource` in
the selected complete source proof. The source pointer is the original Derived state's source
owner,not an independently selected equal id. Its checked target kind and native acyclic graph
remain the existing proofs. A shared Derived target used twice retains two original consumer
occurrences,each borrowing the same actual supplier proof.

`BoundDraftingProgramCanonicalInputs` privately owns the complete result vector. `inputs()` and
`limits()` retain the original selected objects;`shape()` exposes actual D,C,D+C populations.
`occurrences()` preserves original stream order. Each slot's `occurrence()` keeps its actual
consumer,role,span,scalar ordinal and declaration. `canonical()` returns the native inventory join
and optional `derived()` relation,or None for Recipe/Reserved/Point/Edge metadata. None grants no
geometry/currentness certificate;those original declarations remain available to their later stage.

```rust
use sc_core::{construction::*,design::DesignMetadataHost};
fn canonical_counts<H:DesignMetadataHost,S:DraftingInitialInputProvider<H>>(
    inputs:&DraftingProgramDependencyInputs<'_, '_, '_, '_, '_, '_, '_,H,S>,
    limits:&DraftingProgramCanonicalLimits) -> Result<(usize,usize),String> {
    let bound=inputs.bind_canonical_inputs(limits).map_err(|e|e.to_string())?;
    assert!(std::ptr::eq(bound.inputs(),inputs));
    assert!(std::ptr::eq(bound.limits(),limits));
    Ok((bound.occurrences().len(),bound.shape().canonical_count()))
}
```

The focused mixed example has four genuine positions:scalar `width=private_length+derived_length`,
a Point using both Unknown inputs and Derived Length,scalar Count with both `derived_count`
branches,and an assertion with an `eps_geo` header. It yields D=13,C=10,D+C=23. Limits `(12,0)`
refuse the complete 13 occurrences before the record quota;`(13,22)` refuses the complete 23
records;`(13,23)` retains all uses. Tests verify actual canonical declarations and selected source
pointers for both Derived representation families,unchanged Unknown states and zero provider calls.
An empty actual dependency stream accepts `(0,0)`,even when the recipe contains a literal binding.

`DraftingProgramCanonicalError` preserves the selected complete inputs and policy. `refusal()`
retains the first population/quota refusal or original allocator cause inline. A native canonical
failure boxes only its semantic diagnostic pair:`DraftingProgramCanonicalJoinFailure` retains
its original stream index,actual occurrence and original typed native cause. Default formatting
omits authored names,payloads and values;explicit getters preserve original diagnostics. Existing
private source/order proofs already certify their original canonical owners,so this defensive
native failure is not fabricated through a dishonest provider in the focused positive fixture.

This completes mixed canonical binding with focused checks. Geometry/semantic associations,whole
programme composition and the separately tracked coupled verification remain outstanding. It
provides no geometry execution,authored value,physical result or whole-programme acceptance.


## Original geometry uses in formulas

`BoundDraftingProgramCanonicalInputs::formula_geometry_dependencies()` streams every original
Point/Edge declaration used in checked formulas. It visits all selected Root-source proofs in
original source order,including unused bodies,then the complete mixed canonical occurrence
vector in original order. Every repeated name and untaken branch participates. This is an
allocation-free borrowed view;no source is extracted,reparsed,cloned or queried for values.

Each private `DraftingProgramFormulaGeometryUse` retains `consumer()`,`index()`,`span()`,
`declaration()` and `reference()`. The reference is the original declaration's PointRef or EdgeRef;
it is no supplier/currentness/frame certificate. The two inspectable consumer variants are:

- `Source { source_index,source }`:the original zero-based source-proof vector position and
  actual `FormulaCheckedSource` object. `index()` is that source's original dependency index.
- `Mixed(slot)`:the actual `DraftingProgramCanonicalOccurrence` slot in the complete retained
  vector. Its original mixed consumer,role and scalar ordinal remain on `slot.occurrence()`.
  `index()` is the original complete mixed dependency index,including nongeometry slots.

Filtering arithmetic inputs,prior recipe bindings and reserved names does not renumber later
uses. In `if(size_count==1,x(private_point),x(private_point))`,the two Point uses retain indices1
and2 because the original reserved `size_count` occupies index0. Source-local indices and mixed
stream indices stay in their own actual consumer domains;no combined statement number is made.

```rust
use sc_core::{construction::*,design::DesignMetadataHost};
fn geometry_formula_uses<H:DesignMetadataHost,S:DraftingInitialInputProvider<H> + ?Sized>(
    inputs:&BoundDraftingProgramCanonicalInputs<'_, '_, '_, '_, '_, '_, '_, '_,H,S>) -> usize {
    inputs.formula_geometry_dependencies().count()
}
```

A focused fixture has three unused Root sources,including both branches of the conditional above,
and a mixed programme with a prior scalar,a Point,an Arc and an assertion. The stream keeps six
Root-source geometry uses followed by seven mixed ones. Those seven retain complete mixed indices
`0,1,3,4,8,9,10`;the intervening reserved/prior-scalar/header dependencies remain in their original
canonical slots. Uses of planned `anchor` and `boundary` keep the actual earlier fixed Point/Arc
references,while existing `private_point`/`private_edge` keep their original persistent references.
Tests compare actual source/slot pointers,original metadata-name loans and exact name bytes at
borrowed spans,and cover nth traversal,repeated end reads,empty cohorts and zero provider calls.
Default formatting omits source text and metadata names;explicit getters preserve original data.

The stream is fused and grants no native reference-field coverage,geometry supplier acceptance,
new resource allowance,currentness,physical values or whole-programme acceptance. Direct native
fields and their mixed consumers are implemented below. Earlier-creator requests,complete geometry
bounds and binding remain separate owned leaves;combined verification stays deferred until the
agreed integration feature is implemented.


## Original native geometry in complete canonical programmes

`BoundDraftingProgramCanonicalInputs::native_geometry_dependencies()` attaches every original
native geometry field to its actual checked mixed consumer. It follows the complete order proof
and its same privately admitted normalized programme slots,then reuses each original native
`geometry_dependencies()` producer. No detached operation-id table,reconstructed family schema,
filtered statement ordinal,invented name or fabricated source span participates.

Each private `DraftingProgramNativeGeometryUse` keeps `step()`,the actual
`DraftingProgramOrderStep`;`position()` is that consumer's genuine one-based mixed position.
`native()` returns the original `DraftingNativeGeometryUse`,retaining its exact raw input,
zero-based local field index,typed role and selector loans. `reference()` exposes that same
original PointRef or EdgeRef. Equal operation IDs cannot replace the original input object.
The scalar-source ordinal remains on actual scalar proofs;native fields acquire none.

```rust
use sc_core::{construction::*,design::DesignMetadataHost};
fn native_uses<H:DesignMetadataHost,S:DraftingInitialInputProvider<H> + ?Sized>(
    inputs:&BoundDraftingProgramCanonicalInputs<'_, '_, '_, '_, '_, '_, '_, '_,H,S>) -> usize {
    inputs.native_geometry_dependencies().count()
}
```

The focused fixture interleaves three scalar statements with every native family and an additional
Line whose ID and endpoint values equal an earlier Line but whose declaration is a distinct object.
Their fields retain mixed positions3 and16,each with local field indices0 and1,and borrow the two
correct original declarations. The intervening scalar statement at position4 never shifts the
later fields into a native-only ordinal. Quadratic/cubic controls,both Slash motion and Mirror axis
modes,and three repeated Walk pairs retain their original order and actual selector/pair/anchor
loans. A scalar-only programme yields no direct native fields,even with unevaluated division by zero.
Tests compare genuine consumer and input pointers,original pair/anchor pointers,local field order,
nth traversal,fused repeated end reads,opaque diagnostics and zero source/provider calls.

This iterator allocates no buffer and reads no authored value. Empty native slots and semantic-only
families supply no direct fields;their formulas or actual Root-bound semantic records retain their
separate obligations. The view accepts no existing or earlier planned geometry supplier,frame,
current snapshot,semantic target,resource allowance,physical result or whole programme. Creating-step
classification is implemented below;complete geometry bounds and source binding remain separate.


## Classify original geometry requests before source lookup

`BoundDraftingProgramCanonicalInputs::geometry_requests()` preserves each original geometry
occurrence and classifies its creator before any source/provider/value callback. Root formula
sources come first. Mixed formula and native streams merge by genuine consumer position;formula
uses precede direct native fields at the same position. Each producer keeps its original local
order,indices,roles and selectors. Empty streams and repeated end reads remain fused.

Each private `DraftingProgramGeometryRequest` keeps its `occurrence()` and `target()` together.
The inspectable `DraftingProgramGeometryOccurrence` variants retain the original Formula or Native
view. `position()` is None for a Root source,otherwise the actual one-based mixed consumer position;
`reference()` is the original PointRef/EdgeRef. No name,span or scalar ordinal is manufactured.

For each occurrence,the classifier searches the complete original programme's native operations.
Two matching creator identities refuse as Ambiguous with the first two actual step witnesses,
even when the second is later than the consumer. A unique programme creator in a Root source
refuses RootSource. Same-position and later creators refuse SelfCreator and FutureCreator before
output checks. No match yields Existing,which still needs actual existing-source binding.

An earlier Point,Line,Arc or Bézier must supply its exact original fixed output,including kind and
tag;otherwise FixedOutput retains that actual output and step. Offset,Slash,Truing and Join accept
only Edge requests;Mirror permits Point and Edge requests. These variable families still require
an authentic original family field. CloseDart,AddNotch,Grainline and Walk have no direct Point/Edge
output in this request schema and refuse Unsupported. A wrong variable output kind refuses
OutputKind. Their separate semantic inventories cannot be replaced by a direct geometry request.

An Earlier target carries a privately constructed `DraftingGeometryStepRequest`. Its `step()`
borrows the genuine earlier normalized native owner;`input()` exposes that exact original input,
`reference()` retains the requested output and `requires_role_binding()` distinguishes variable
roles from exact fixed outputs. Unnamed outputs need no fabricated global name or named-output
occurrence. For example,an unnamed earlier Mirror point yields a variable request;its actual
point-role identity,manifest,frame and snapshot remain obligations of subsequent source binding.

```rust
use sc_core::{construction::*,design::DesignMetadataHost};
fn requests<H:DesignMetadataHost,S:DraftingInitialInputProvider<H> + ?Sized>(
    inputs:&BoundDraftingProgramCanonicalInputs<'_, '_, '_, '_, '_, '_, '_, '_,H,S>) -> usize {
    inputs.geometry_requests().filter(|r|r.target().is_ok()).count()
}
```

A refusal remains beside the original occurrence,and iteration can continue for inspection.
Neither an individual Earlier request nor a successful prefix certifies the complete programme.
The iterator allocates no buffers or diagnostic boxes. Each occurrence scans actual native
creators in the complete mixed sequence;this grants no complete work/CPU allowance. The later
controller must admit complete geometry work before reservations or source callbacks,then retain
all bindings or the first original refusal. The [unnamed planned-source port](drafting-context-sources.md#source-facet-for-unnamed-earlier-geometry)
is implemented;complete bounds,actual supplier bindings and coupled whole-programme verification
remain separately owned.
