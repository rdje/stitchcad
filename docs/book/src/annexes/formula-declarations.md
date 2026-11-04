# Formula declaration metadata

> **Status:** implemented vocabulary and immutable sourced declarations at G1-SLICE.5b.2a/.2b in sc-core::recipe.
> Checked initial namespaces, [ordered name scopes](formula-name-scopes.md), expression/current-statement
> checks and [whole recipe kind proofs](formula-checked-recipes.md) are implemented. Separate scalar
> binding/execution is available. [Reference static review](formula-static-validation.md) retains its static scope.
> [Reserved-name diagnostic sources](formula-static-validation.md#reserved-name-diagnostic-sources)
> are specified, exercised in the reference and retained by the product namespace errors below.

The checker needs to know what a name means before fetching its value. An unknown measurement
can still be length. A size context can be absent while size_index still has the kind count.
The vocabulary describes metadata without numbers, input states or value providers. Sourced
declarations carry identities or borrow existing canonical records; their metadata queries read
no state or numerical value.
The normative populations are [contract §§2–3](../spec/formula-language.md#2-values-and-kinds).

## Kinds and origins

FormulaKind has exactly eight variants: Length, Angle, Area, Ratio, Count, Boolean, Point and Edge.
Each has an exact lowercase token. FormulaKind::ALL enumerates the complete table in its authored
order; from_token recognizes an exact token without trimming, aliases or case repair.

The six scalar kinds convert to/from the existing FormulaBindingKind statement annotations.
Point and Edge have no binding_kind: they are references to geometry that operations own.
These conversions do not infer an expression's kind or validate any operation's signature.

```rust
use sc_core::recipe::{FormulaBindingKind, FormulaKind};
assert_eq!(FormulaKind::from(FormulaBindingKind::Angle), FormulaKind::Angle);
assert_eq!(FormulaKind::Point.binding_kind(), None);
assert_eq!(FormulaKind::from_token("Length"), None);
```

FormulaOrigin has exactly nine variants: Measurement, Ease, Parameter, Profile, Material, Geometry,
Recipe, Size and Tolerance. Its ALL/token/from_token APIs mirror the kind vocabulary. An origin
identifies the semantic source domain. It does not certify a source record, a value or evidence.

## Reserved names and their contexts

FormulaReservedName represents the existing five FormulaToleranceName values plus SizeIndex,
SizeCount and IsBaseSize. Its eight-name population agrees with MachineToken's reserved input
classification. Only the three grammar words let/assert/if remain reserved identifiers; loop,
repeat, while, fn and macro remain ordinary names under the approved current grammar.

| Name | Kind | Origin | Required context | Tolerance role |
| --- | --- | --- | --- | --- |
| eps_num | length | tolerance | Always | Numerical |
| eps_geo | length | tolerance | Always | Geometric |
| eps_fmt | length | tolerance | Export | Format |
| eps_imp | length | tolerance | Export | Importer |
| eps_phys | length | tolerance | Profile | Physical |
| size_index | count | size | Size | none |
| size_count | count | size | Size | none |
| is_base_size | boolean | size | Size | none |

FormulaReservedContext describes the required value provider. It is distinct from FormulaOrigin:
eps_phys has tolerance origin and requires a profile to supply its value. Always describes the
language-defined classes; the metadata API does not fetch even those classes' numerical values.
Optional contexts stay declared when absent. Runtime missing-value diagnostics remain later work.

```rust
use sc_core::recipe::{FormulaKind, FormulaOrigin, FormulaReservedContext, FormulaReservedName};
let name = FormulaReservedName::from_token("eps_phys").unwrap();
assert_eq!(name.kind(), FormulaKind::Length);
assert_eq!(name.origin(), FormulaOrigin::Tolerance);
assert_eq!(name.required_context(), FormulaReservedContext::Profile);
assert_eq!(FormulaReservedName::SizeIndex.tolerance_name(), None);
```

A reserved spelling is a valid reference token and may not be rebound. Initial namespace collisions
are checked below; [ordered metadata scopes](formula-name-scopes.md), expression checks and complete
recipe kind proofs are available. Separate scalar evaluation reads admitted contexts;geometry and
full application policy retain their own obligations.

## Immutable sourced declarations

FormulaDeclaration borrows a validated MachineToken's exact name or the actual normalized recipe
source. Its private construction and Copy/Clone views retain those lifetimes. Default Debug for
both the declaration and its source shows only kind/origin, omitting customer names, identities,
canonical states and numeric values. Explicit source inspection remains available.

FormulaInputOrigin closes the five external domains: Measurement, Ease, Parameter, Profile and
Material. The input constructor carries both the metadata record identity and its separate
canonical declaration identity, plus the declared scalar annotation. FormulaScalarInputOrigin
restricts this generic path to Parameter, Profile and Material. Measurement and Ease must use the
canonical length adapter; they cannot claim angle/area/ratio/count/Boolean input kinds. Other scalar
metadata remains a claim:
canonical registry adapters must validate target identity, kind and provenance. It does not store
an authored or computed number, certify a source record, or fetch a value.

The length_input adapter borrows an actual immutable LengthDeclaration and forces the kind length.
Known, assumed, unknown, preference and derived records all declare that same kind. The canonical
record retains its sole value/state/source ownership; the declaration does not clone or cache it.

```rust
use sc_core::{name::MachineToken, ontology::EntityId, recipe::{
    FormulaBindingKind, FormulaDeclaration, FormulaScalarInputOrigin, FormulaKind,
}};
let name = MachineToken::new("desired_sweep").unwrap();
let declared = FormulaDeclaration::input(&name, FormulaScalarInputOrigin::Parameter,
    EntityId::from_bits(11), EntityId::from_bits(12), FormulaBindingKind::Angle);
assert_eq!(declared.kind(), FormulaKind::Angle);
assert_eq!(declared.name(), "desired_sweep");
```

Point and edge constructors retain the exact PointRef/EdgeRef creator and local tag, forcing their
geometry kind and origin. They neither resolve coordinates/curves nor certify prior-operation order.
The reserved constructor takes a FormulaReservedName and derives its fixed name/kind/origin without
requiring its optional context. Source locators are exposed through FormulaDeclarationSource.

The recipe constructor inspects an actual one-based position in a FormulaNormalizedRecipe.
Only a let contributes a declaration. Zero, absent positions and assertion labels return None;
an assertion label cannot become a scalar input. Returned metadata preserves the actual ordinal,
name, authored kind annotation and original whole-statement/name spans. The RHS is not inferred,
executed or bound, and namespace rules still have to refuse reserved-name let bindings.

```rust
use sc_core::recipe::{FormulaDeclaration, FormulaKind, FormulaRecipe};
let syntax = FormulaRecipe::parse("let width:length=missing assert check:eps_num=width==width").unwrap();
let recipe = syntax.normalize_literals().unwrap();
let declared = FormulaDeclaration::recipe(&recipe, 1).unwrap();
assert_eq!(declared.kind(), FormulaKind::Length);
assert!(FormulaDeclaration::recipe(&recipe, 2).is_none());
```

Here missing remains unvalidated syntax. A sourced declaration supplies metadata to the separate kind checker,
so this example grants no static acceptance or authority to compute a missing value.

## Verification boundary

The semantic and declaration contracts check normative kind/origin/reserved-name tables, exact
source identities and recipe ordinals. Compile-fail contracts preserve private construction and
borrow lifetimes. These are metadata and syntax proofs; registry admission, numeric reads and
physical geometry retain their separate owners. General scalar declarations cannot impersonate
the dedicated measurement length family.

Run focused contracts through the [local producer profile](../build-and-checks.md#local-producer-profile):

```bash
cargo test -p sc-core --test formula_semantic_contract
cargo test -p sc-core --test formula_declaration_contract
```

Historical fault sets, dispatch controls, counts and restoration receipts are retained exactly in
`docs/history/stitchcad-devnotes-part507.md`. Retrieve that record without Git:

```bash
python3 -I -B scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part507.md
```

The maintained semantic_mutations.py and declaration_mutations.py entries under
`docs/tasks/artifacts/formula_structure/` accept `--classifier-only` for read-only plan checks.
Run their mutation modes alone and consume source restoration before another build or probe.
A dispatch capture proves its planned effects separately from actual native behavior.

## Checked initial namespace

FormulaInitialDeclaration admits existing authored input and geometry declarations through
`TryFrom<FormulaDeclaration>`. Input, LengthInput, Point and Edge sources enter this boundary;
Recipe and Reserved return the original rejected declaration. A future let cannot become an
initial input, and an authored list cannot inject the language's own reserved metadata. Names
already borrow validated MachineToken values, so malformed spellings and the three grammar
keywords cannot enter this path. The declaration's canonical registry/operation-order validity
still requires its separately owned checks.

FormulaNamespace::new consumes these initial declarations in their authored order and includes
all eight reserved metadata entries. A collision refuses before replacing the earlier entry;
no partial namespace escapes. Two declarations from the same origin also collide, even with
identical metadata or values. Absent export, profile or size contexts do not hide the fixed kind
of their reserved names. Construction fetches no numeric value, state, provider or geometry.

This unknown measurement retains its actual canonical record while entering the namespace:

```rust
use sc_core::{
    name::MachineToken, ontology::EntityId,
    recipe::{FormulaDeclaration, FormulaDeclarationSource, FormulaInitialDeclaration,
        FormulaInputOrigin, FormulaKind, FormulaNamespace},
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState},
};
let name = MachineToken::new("body_width").unwrap();
let record = LengthDeclaration::new(LengthDeclarationDefinition {
    id: EntityId::from_bits(101), source: EntityId::from_bits(102),
    state: LengthState::Unknown { observation: EntityId::from_bits(103) },
}).unwrap();
let declaration = FormulaDeclaration::length_input(
    &name, FormulaInputOrigin::Measurement, EntityId::from_bits(104), &record);
let initial = FormulaInitialDeclaration::try_from(declaration).unwrap();
let namespace = FormulaNamespace::new([initial]).unwrap();
let found = namespace.declarations().find(|d| d.name() == "body_width").unwrap();
assert_eq!(found.kind(), FormulaKind::Length);
assert!(matches!(found.source(), FormulaDeclarationSource::LengthInput { declaration, .. }
    if std::ptr::eq(declaration, &record)));
assert_eq!(namespace.declarations().len(), 9); // one input plus eight reserved names
```

The declarations iterator is exact-size and ordered lexically by name for deterministic metadata
inspection. That order does not change recipe statement order. Namespace Clone retains the same
borrowed records and references; it copies no canonical value/state. Private fields and borrowed
lifetimes prevent detached names/records or direct namespace mutation. Default namespace Debug
shows only the declaration count; declaration/initial/error Debug omits authored names and IDs.

## Source-preserving collision errors

The initial FormulaNamespaceError has two cases. ReservedBinding carries the fixed FormulaReservedName and
actual attempted declaration; its token is formula_rebinding. AmbiguousName carries the first and
second authored declarations; its token is formula_ambiguous_name. name() explicitly exposes the
colliding spelling, and binding_sources() returns the earlier/reserved and attempted declarations
in that order. The existing source views retain both semantic origins, canonical identities,
record borrows or exact point/edge references. Initial sources and reserved metadata have no
recipe statement ordinal; this API fabricates none. [Contract5.2.1](../spec/formula-language.md#521-binding-refusal-sources)
and [reference argument evidence](formula-static-validation.md#reserved-name-diagnostic-sources)
define the source distinction.

The ambiguous source pair is boxed once on failure to keep the returned error compact. Error
Clone retains the same canonical borrows; it copies no authored numeric value or state.

A profile parameter attempting a reserved format-tolerance name refuses without a format context:

```rust
use sc_core::{name::MachineToken, ontology::EntityId, recipe::{
    FormulaBindingKind, FormulaDeclaration, FormulaInitialDeclaration, FormulaNamespace,
    FormulaNamespaceError, FormulaReservedContext, FormulaReservedName, FormulaScalarInputOrigin,
    FormulaToleranceName,
}};
let name = MachineToken::new("eps_fmt").unwrap();
let attempt = FormulaDeclaration::input(&name, FormulaScalarInputOrigin::Profile,
    EntityId::from_bits(201), EntityId::from_bits(202), FormulaBindingKind::Count);
let error = FormulaNamespace::new([
    FormulaInitialDeclaration::try_from(attempt).unwrap()
]).unwrap_err();
assert_eq!(error.token(), "formula_rebinding");
assert_eq!(error.name(), "eps_fmt");
assert!(matches!(error, FormulaNamespaceError::ReservedBinding { reserved, .. }
    if reserved == FormulaReservedName::Tolerance(FormulaToleranceName::Format)
        && reserved.required_context() == FormulaReservedContext::Export));
assert_eq!(error.binding_sources()[1].kind(), sc_core::recipe::FormulaKind::Count);
```

Default error Display exposes only the internal stable token. Localized user messages belong to
the command layer and use the explicit typed arguments. A successful initial namespace certifies
source admission and initial-name uniqueness. Exact initial reads are available below and [ordered name scopes](formula-name-scopes.md) retain prior
recipe annotations. Expression kind/signature checks and whole-recipe sourced dependency proofs are
implemented separately; initial namespace admission alone grants neither proof.
Numeric state/context validation, evaluation and geometric/registry correctness retain their owners.

Ten public contracts and three negative private/lifetime doctests cover closed reserved metadata, rejected recipe/reserved admission, three
scalar domains/six kinds, five canonical length domains/all five states, exact geometry refs,
all36 ordered origin collisions/equal origins, reserved attempts across every scalar annotation
and point/edge source, first-error ordering, deterministic inspection, lifetimes and opaque
formatting. Seventeen actual compiled admission/context/collision/source/order/privacy faults
fail public body assertions and restore the production source exactly; the watched structural
runner also refuses invalid fault classification and stale anchors.

The direct namespace entry plans the implementation and current public test before any read or
effect. Ordinary mode declares all eighteen fault/restoration diagnostic files and one mutable
implementation source. It preserves the seventeen original ordered cases, the complete ten-test
Cargo argument vector, prepared child stores and exact source restoration.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/namespace_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/namespace_mutations.py
```

Classifier-only validates both read-only inputs, prepares the local stores/output directory and
checks all original anchors/current macros. It declares zero files or mutable sources and invokes
no native child or source/output write. A copied entry accepts an input with three hard links and
leaves unused fault/restoration aliases untouched. Two complete captures, six physical refusals and
thirteen independent source/file/mutable/mode/store/restoration omissions watch these plans.
Captures certify dispatch; the real native replay supplies separate behavior evidence.

A failed body's thread panic path, line and byte-column must identify a current assertion macro.
Custom messages qualify. Nonthread macro mentions, compiler/linker noise, expect-only panics and
passed test names refuse. D185's measured nonthread weakness is repaired and independently watched
by the actual legacy-regex body fault. All seventeen real cases match the corrected source-derived
failing-test sets; every failed body reaches a current macro. One prior prediction omitted the
privacy test's optional eps_phys collision; its actual failure and source cause are recorded.
Seventy-two inputs, sixty-three unchanged production Rust files, the unchanged public tests,
eighteen local logs and Cargo's restored full ten-test executable remain exact. Run the native
command alone and consume restoration before another build or probe. These probes change no
metadata API, grammar, numeric execution or geometry authority.


## Exact declared-name reads

FormulaNamespace::resolve accepts a validated MachineToken and returns the exact existing
FormulaDeclaration. Lookup uses the authored spelling directly. The returned declaration retains
its original name and canonical source borrows; it is independent of the query token and namespace
allocation. Reading metadata neither copies canonical state nor fetches a numerical value.

An unknown measurement still has a known declared kind. The unknown body_width namespace in the
[initial namespace example](#checked-initial-namespace) resolves to the same borrowed record:

```rust
# use sc_core::{name::MachineToken, ontology::EntityId, recipe::{FormulaDeclaration,
# FormulaInitialDeclaration, FormulaInputOrigin, FormulaKind, FormulaNamespace},
# value::{LengthDeclaration, LengthDeclarationDefinition, LengthState}};
# let name = MachineToken::new("body_width").unwrap();
# let record = LengthDeclaration::new(LengthDeclarationDefinition { id: EntityId::from_bits(1),
# source: EntityId::from_bits(2), state: LengthState::Unknown { observation: EntityId::from_bits(3) } }).unwrap();
# let namespace = FormulaNamespace::new([FormulaInitialDeclaration::try_from(
# FormulaDeclaration::length_input(&name, FormulaInputOrigin::Measurement, EntityId::from_bits(4), &record)).unwrap()]).unwrap();
let query = MachineToken::new("body_width").unwrap();
let declared = namespace.resolve(&query).unwrap();
assert_eq!(declared.kind(), FormulaKind::Length);
assert_eq!(declared.name(), "body_width");
```

Reserved names likewise retain their kinds when their runtime providers are absent: eps_fmt is a
length with export context, eps_phys is a length with profile context, and is_base_size is Boolean
with size context. Static resolution supplies no provider-availability or canonical registry proof.
A runtime read of an unknown fact remains formula_unknown; a missing valid tolerance provider
remains formula_tolerance_unbound. Canonical length authored reads now have an [explicit adapter](formula-runtime-validation.md#canonical-length-authored-reads).
Complete runtime resolution and execution checks remain separately
owned. This API resolves the initial namespace before statement one; [ordered metadata scopes](formula-name-scopes.md) retain prior
recipe visibility. Expression kind checks and complete recipe proofs are available through their
separate validation APIs; an initial read alone proves neither.

An absent declaration returns opaque FormulaUnboundName. name() borrows the exact query and
origins_searched() exposes all nine flat namespace domains in contract order: measurement, ease,
parameter, profile, material, geometry, recipe, size and tolerance. Recipe has no entries in the
initial namespace. No spelling is repaired, alias accepted, value substituted or recipe reordered.
The error carries no guessed statement index or canonical expression; later whole-recipe composition
adds only context it actually owns. Default Debug omits the name; Display is the stable internal
token, which the command layer must localize with the explicit arguments.

```rust
use sc_core::{name::MachineToken, recipe::{FormulaNamespace, FormulaOrigin}};
let namespace = FormulaNamespace::new([]).unwrap();
let query = MachineToken::new("missing_width").unwrap();
let error = namespace.resolve(&query).unwrap_err();
assert_eq!(error.token(), "formula_unbound_name");
assert_eq!(error.name(), "missing_width");
assert_eq!(error.origins_searched(), &FormulaOrigin::ALL);
```

Public read and compile-fail contracts cover exact source/name/domain behavior, missing-name
privacy and borrow lifetimes. Historical fault sets, dispatch controls and exact restoration
receipts are retained in `docs/history/stitchcad-devnotes-part509.md`, retrievable without Git:

```bash
python3 -I -B scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part509.md
cargo test -p sc-core --test formula_name_read_contract
python3 -I -B docs/tasks/artifacts/formula_structure/name_read_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/name_read_mutations.py
```

Run mutations alone and consume restoration before another build/probe. Classifier-only plans no
native child/source/output writes. Dispatch captures certify effects separately from native proof.

General Input metadata can now be matched to a supplied actual canonical scalar declaration through
the [general authored-read adapter](formula-runtime-validation.md#general-scalar-authored-reads).
It validates source/identity/kind and preserves state; static metadata construction still fetches no
value or registry proof. The canonical ScalarInputKind also converts explicitly to FormulaBindingKind.


## Actual mixed-program initial namespace

After [complete initial association](drafting-context-sources.md#bind-every-extra-initial-source),
`BoundDraftingProgramInitials::bind_initial_namespace()` derives a private initial name map from
that exact programme's containing Root. Complete held-content and prospective name bounds have
already passed. The existing complete Root current-input check runs first,preserving its original
parameter,canonical,Material,source and reference causes. It computes no value or readiness.

The factory inserts all eight existing reserved declarations,then actual Root parameters,selected
host measurement/Ease rows,and original bound extra initials. Parameters borrow their actual
metadata tokens and canonical targets. Selected rows come from `DesignFormulaDeclarationSource`'s
allocation-free iterator,not the older allocating declaration-vector factory. The immutable host
contract retains actual selected table rows followed by Ease rows,including original fallible
items;no new source registry or totals are accepted. Extra inputs keep their prior source bindings.

Each insertion reuses `FormulaNamespace::with_initial()` and its original collision rules. There
is no shadowing,repair,deduplication or successful returned prefix. A parameter/measurement clash
retains Parameter then Measurement declarations;two host rows or repeated extras retain their two
original occurrences. A reserved spelling retains fixed reserved metadata and the actual attempted
source. The first refusal follows parameter,selected-row,extra order;later rows are not consumed
after an earlier failure. Current-input validation precedes this streaming construction.

```rust
use sc_core::{construction::{BoundDraftingProgramInitials,DraftingInitialInputProvider},
    design::DesignFormulaDeclarationSource,name::MachineToken,recipe::FormulaKind};
fn initial_kind<H:DesignFormulaDeclarationSource,S:DraftingInitialInputProvider<H> + ?Sized>(
    inputs:&BoundDraftingProgramInitials<'_, '_, '_, '_, '_, '_,H,S>,name:&MachineToken)
    ->Result<FormulaKind,String> {
    let names=inputs.bind_initial_namespace().map_err(|error|error.to_string())?;
    names.visible_names().resolve(name).map(|declaration|declaration.kind())
        .map_err(|error|error.to_string())
}
```

`DraftingProgramInitialNamespace` owns the private map and borrows the complete original initial
associations. `inputs()` reaches those exact roles,positions,bounded work,input and selected policy.
`visible_names()` returns only a readonly scope borrowing the map allocation. Actual Unknown
Length/Count metadata remains available by kind while its value stays unread. A bound existing
point can be visible initially;the programme's own scalar lets and named native outputs are absent.
For example,`let later:length=parameter` leaves `later` unbound in this initial scope,even when
`parameter` is available. Earlier-only mixed visibility follows in a separate stage.

`DraftingProgramInitialNamespaceError` retains complete original associations and the existing
`DesignFormulaError`:original current-input/host cause,invalid initial declaration,or reserved/
ambiguous-name sources. Debug omits authored names. The held drafting provider is not queried again
for currentness or geometry during map construction. The metadata scope proves no whole programme
kinds,canonical dependencies,argument execution,geometry values or release readiness. Logical name
bounds are separate from allocator bytes/capacity and CPU work;coupled verification remains owned.


## Bound complete mixed-order checker work

`DraftingProgramInitialNamespace::admit_order_work(explicit_policy)` now counts every actual mixed
scalar operand and native argument before any child kind checker or proof-vector reservation.
`DraftingProgramOrderLimits` requires separate expression,node and retained-record allowances;
there is no Default. Earlier normalized syntax and the private initial map supply no checker grant.
Their original held-content,namespace,output and initial-binding policies remain required separately.

The producer traverses genuine original mixed slots. A scalar let contributes its actual RHS;
an assertion contributes both original operands. Point contributes two actual normalized arenas,
center-arc three,and slash/spread its opening arena. Other native inputs retain their slot with no
scalar expression. Every branch,call argument and repeated native occurrence counts. Arena
`node_count()` reads the actual retained node-vector length;normalization estimates and caller
hints do not replace these fields. Separately identified Root formula-source bodies retain their
own later source-checking policy and are outside this mixed scalar/native cohort.

For complete steps M,scalar statements S,expressions E and semantic nodes N,the conservative
additional retained-checker bound is **M + S + E + 2N**:outer step slots,statement/expression proof
owners,node certificates and at most one retained name dependency per node. Inline owners can be
counted conservatively twice. This is a logical successful-proof envelope,not checker scratch,
allocator bytes/capacity,CPU or a runtime policy. Complete checked arithmetic precedes quotas;
expression allowance is checked before nodes,then retained records.

The mixed fixture has seven slots,two scalar statements,three scalar operands/eight scalar nodes
and eight native argument occurrences/twelve native nodes,including the repeated Point. It needs
11 expressions,20 nodes and **7 + 2 + 11 + 40 =60** additional records. Allowances10 expressions,
19 nodes or59 records refuse the actual complete requirement in that order,even though a later
Point name would fail static checking. No child checker or drafting callback runs here. Existing
Root source bodies remain separately owned. An empty programme accepts0/0/0 checker work while
its initial namespace still required the earlier reserved-name allowance.

```rust
use sc_core::{construction::{DraftingProgramInitialNamespace,DraftingInitialInputProvider,
    DraftingProgramOrderLimits,DraftingProgramOrderShape},design::DesignMetadataHost};
fn order_shape<H:DesignMetadataHost,S:DraftingInitialInputProvider<H> + ?Sized>(
    names:&DraftingProgramInitialNamespace<'_, '_, '_, '_, '_, '_, '_,H,S>,
    policy:DraftingProgramOrderLimits)->Result<DraftingProgramOrderShape,String> {
    names.admit_order_work(&policy).map(|work|work.shape()).map_err(|error|error.to_string())
}
```

`AdmittedDraftingProgramOrderWork` privately pins the original initial namespace,selected policy
and complete shape. `inputs()`,`limits()` and `shape()` expose those originals without allocating,
advancing metadata or checking a kind. `DraftingProgramOrderWorkError` retains that same owner/
policy and PopulationOverflow,Expressions,Nodes or RetainedRecords with actual required counts.
Diagnostics omit names. The complete mixed checker below performs proof reservation and earlier-only
checking under this envelope. Dependencies,allocator faults and coupled verification retain their
separate owners;bounded work alone grants no accepted programme prefix.


## Check complete mixed name and kind order

`DraftingProgramInitialNamespace::check_order(explicit_order_limits)` consumes the actual private
initial map. It invokes the same complete order-work admission above,then reserves the entire
mixed proof vector before checking a header or child expression. The original TryReserveError
stays inline on allocation refusal. No metadata cursor,successful local proof prefix or cloned
final namespace escapes any failure.

The cursor traverses the programme's original mixed slots. A scalar slot must match the same
actual normalized statement and genuine recipe ordinal;native slots never advance that scalar
ordinal. Existing current-statement checking validates headers and every original operand before
recording let metadata. An assertion checks both operands and their complete comparison kind;
its label adds no name. Original source spans,statement pointers and argument arenas stay intact.

For a native slot,the controller preflights all actual named outputs against visible declarations
using the existing namespace collision rules **without insertion**. Earlier same-step output names
are also compared in original output-vector order;equal names retain their exact two geometry
sources. Reserved,prior-scalar,prior-geometry and same-step clashes refuse before child arguments.
The actual Point/Arc/Slash checker then checks every original scalar argument against initial and
strictly earlier declarations. Native inputs without scalar documents retain their actual input.
Only after complete argument success are that slot's actual bound outputs inserted. Self and
future outputs remain absent from arguments,even when the raw output-name vector lists them first.

For example,the focused programme follows this order:

1. `let width:length=1 mm` adds only `width`.
2. Point `anchor` checks `width` and `1 mm/0`,then adds `anchor`. Static checking performs no division.
3. `let second:length=x(anchor)` resolves the actual earlier point and adds `second`.
4. Center-arc `future_edge` checks Radius Length and Start/Sweep Angle,then adds its actual edge name.
5. Slash separation checks the earlier `second` Length;no named output was supplied for this slot.
6. Walk retains its actual no-scalar-document input;spatial/semantic dependencies remain separate.
7. `assert closure:eps_geo=second==width` checks both operands and declares no name.

The original output vector can list `[future_edge at4,anchor at2]`;visibility still follows actual
mixed positions. `x(anchor)` in step1 or in Point's own arguments refuses the original unbound name.
A later wrong assertion kind remains a static refusal at mixed position7/scalar ordinal3,even when
Point's earlier argument contains runtime division by zero. A scalar header clashing with prior
`anchor` retains Geometry then the actual Recipe declaration,with genuine scalar ordinal2
alongside mixed position3. Same-step duplicate names retain their original output indexes and owners.

```rust
use sc_core::{construction::{DraftingProgramInitialNamespace,DraftingInitialInputProvider,
    DraftingProgramOrderLimits},design::DesignMetadataHost};
fn checked_steps<H:DesignMetadataHost,S:DraftingInitialInputProvider<H> + ?Sized>(
    names:DraftingProgramInitialNamespace<'_, '_, '_, '_, '_, '_, '_,H,S>,
    policy:DraftingProgramOrderLimits)->Result<usize,String> {
    names.check_order(&policy).map(|proof|proof.steps().len()).map_err(|error|error.to_string())
}
```

`CheckedDraftingProgramOrder` privately retains the original bound initial/output associations,
selected policy,complete shape and every original-order `DraftingProgramOrderStep`. Each step has
a genuine mixed `position()` and immutable Scalar,Point,CenterArc,SlashSpread or Other `kind()`.
Scalar components retain their original ordinal/statement and both assertion proofs;native
components retain actual normalized schema owners and complete argument proofs. These proofs
borrow original declarations/arenas,so the final cursor allocation is dropped. Individual enum
components cannot manufacture the private complete owner. The empty programme accepts zero
additional checker work and returns an empty complete vector without callbacks.

`DraftingProgramOrderError` retains original associations/policy and the first Work,Allocation,
Header,Scalar,Point,CenterArc or SlashSpread refusal. Headers include genuine mixed position and
original output index,or None for scalar headers. Child errors preserve their existing actual
statement/argument role,node,span,name/kind and source witnesses. Diagnostics omit authored source.

This proves complete mixed name/header and scalar/native argument-kind order. Separately identified
Root formula-source bodies,actual canonical/current geometry/frame and semantic dependencies,
Pocket/Buttonhole obligations,allocator fault tests and coupled verification remain owned later.
No geometry/numeric provider is called here,and no values or executable whole programme follow.


## Complete dependency composition

**Root source bridge,canonical joins and complete geometry supplier binding are implemented
with focused checks;actual garment source content admission is also available. Semantic
dependency composition and whole acceptance remain planned.**
The scoped source review separates canonical relations,geometry and semantic sources. The mixed
proof above does not itself certify those stages. The [actual Root source bridge](formula-checked-recipes.md#root-sources-in-the-actual-mixed-scope)
now invokes the existing complete source factory before mixed checking consumes the initial map.

The actual containing initial map is the right boundary for Root formula-source checking. Its
`FormulaNormalizedSources` already comes from this programme's actual Root. The existing
`check_sources(namespace,FormulaSourceCheckLimits)` factory checks every source,including unused
bodies and all branches,canonical input joins and Derived consumer kinds. Invoke it against the
private initial map before mixed checking consumes that map. Returned proofs borrow actual
normalized sources and declaration owners;they need no cloned namespace. The implemented bridge
retains those exact owners,explicit allowances and original quota/allocation/static causes.

Reuse `FormulaCheckedSources::prepare_dependencies(FormulaSourceDependencyLimits)` and
`FormulaSourceDependencies::admit_acyclic(FormulaSourceCycleLimits)` for the actual canonical
Derived relation. These separate native factories already preserve original edges,source-proof
identity,complete bounds and concrete cycles without reading values. Their relation is not a
replacement for mixed scalar/native dependencies,which must also join every original checked
operand to its actual canonical source/family/state. A copied inventory or equal IDs cannot replace
those owners. The full feature still owes the combined no-prefix acceptance boundary.

The closed native input enum currently has13 families:Point,Line,Arc,Bezier,Offset,SlashSpread,
CloseDart,AddNotch,Grainline,Walk,Truing,Mirror and Join. Geometry checking must cover their actual
published dependency fields plus geometry declarations used by checked selectors. Existing-only
references reuse the context's exact reference/frame/snapshot binding. Programme creators must be
classified across the complete original sequence:ambiguous creators,self and future uses refuse
before live lookup. Earlier fixed outputs come from their actual schemas;variable outputs require
authentic family fields/manifests and original containing frame/snapshot records.

The named planned-source port takes a genuine **named-output** position request. A dependency
can refer to an earlier output that has no global name. The implemented geometry controller therefore uses
an actual creating-step/reference request and authentic planned-family sources,with complete
bounds before queries. It fabricates no global name and never treats planned geometry as live.
Existing named-output binding remains its own actual catalogue/definition/tag association.

Semantic source ownership requires the actual garment. `DesignGarment<H>` owns sewing, Pocket,
notion,Closure,layer,Hem and construction inventories;its `inputs()` getter,under the published
`DesignReferenceHost` bound,borrows its sole nested DesignInputs. The [implemented garment source facet](drafting-context-sources.md#admit-actual-garment-source-content)
binds that actual input object to the drafting Root and admits complete fresh garment/provider
content before currentness or declaration queries. Public `charge_design_garment_resources` and the original
inventory producers supply existing exhaustive field walks. Original declaration adapters preserve
actual objects/canonical loans;their allocating vector factories need bounded composition before
use. No empty garment,matching-id copy or id-only payload can establish an unprovided owner.

The semantic slice owns native Dart/Notch/Grainline/edit/source relations as well as the mandatory
Pocket and Buttonhole checks. A Pocket opening keeps its actual logical declaration or Profile
origin,kind/domain/state and component recipe dependency. A Buttonhole keeps its actual canonical
button-size declaration and derivation operation;there is no second authored hole-length input.
Actual readonly logical/Profile/operation source facets and their complete bounds are required
where current intent adapters expose references only. Missing or unsupported sources retain their
original typed refusal. Physical Pocket construction/opening scope and executed positive hole
length retain their G2/G3 owners;logical checks cannot be replaced by a physical approximation.

Each stage will keep actual source/context identity,original first causes and a complete result
before it can join the final programme acceptance. The task tree owns the source composition,
canonical graph,geometry and semantic implementations separately;coupled allocator/lifetime/
source/order verification remains a later feature-completion task. This plan adds no fabricated
source record,context,logical state,geometry or value to the implementation.


### Geometry work before binding

Implemented `admit_geometry_work` charges complete original content afresh,then derives E,
E+E records and E*S creator inspections with checked arithmetic before work quotas. Refusal order
is content,population overflow,occurrences,records,inspections. The error retains actual canonical
inputs,selected policy and original `refusal()`;only the semantic diagnostic is boxed.

A rich fixture has13 formula uses plus1 native field across4 mixed slots:E14,records28,inspections56.
Limits `(13,0,0)` refuse E14;`(14,27,0)` refuse28 records;`(14,28,55)` refuse56 inspections.
`(14,28,56)` passes. A scalar-only programme permits all zero geometry allowances after content
passes. Native fixtures retain20 repeated/direct fields across17 slots,requiring40 records/340
inspections. No successful admission establishes a supplier,frame,currentness,geometry or value.

Reproduce the scoped family/public-owner controls with the tracked probe:

```bash
python3 -I -B scripts/local_environment.py -- python3 -I -B docs/tasks/artifacts/drafting_dependencies/plan_sources.py
```

It builds the current sc-core library,then uses the public Garment/input getter with the actual
DesignReferenceHost bound. The borrowed return compiles;the otherwise identical lifetime escape
must fail for that lifetime cause. Compiler scratch stays in the repository and rejects linked or
nested-repository output paths. This control establishes the source interface/lifetime boundary;
it does not implement the planned dependency stages.


## Bind complete geometry suppliers

After `admit_geometry_work`,call `work.bind_geometry()` with a source implementing both
`DraftingInitialInputProvider` and `DraftingGeometryStepProvider`. The controller reserves the
complete E request slots and E supplier slots before any source lookup. It then classifies every
original creator across the whole mixed sequence before binding a supplier. A later self/future
or ambiguous creator therefore refuses before even an earlier existing reference is queried.
Original merged occurrence order remains Root sources first,then mixed positions,with formula
uses before direct native fields at the same position. Repeated uses retain separate slots.

There are three supplier variants:

| Variant | Source and guarantee |
| --- | --- |
| Existing | Original opaque geometry,with exact reference and held frame/snapshot record binding |
| Fixed | Exact strictly earlier Point/Line/Arc/Bézier schema output;no provider lookup |
| Planned | Authentic strictly earlier variable family field and original context records;output remains unissued |

A variable request calls `planned_geometry` once. The response is checked against the actual
creating input object,then exact Point/Edge reference,then the original frame record,then original
snapshot record. Equal IDs or equal-looking foreign inputs/records do not satisfy identity.
Authentic family parents establish active role/manifest membership. An unnamed dependency has no
independently authored catalogue to compare;the separately implemented named-output controller
still checks its actual catalogue/definition/tag identity. Neither path fabricates a global name.

For example,an Offset at mixed position1 consumes an existing source edge. A Point at2 declares
its fixed output. A Line at3 uses that point and an existing point. Offsets at4 and5 both consume
the unnamed active edge from position1. The complete result has five requests and suppliers in
order:Existing,Fixed,Existing,Planned,Planned. Only the two Existing slots query live geometry;
only the two Planned slots query authentic family fields. A Point argument `1 mm/0` remains
statically valid and unevaluated. Missing geometry and unavailable planned fields remain errors.

```rust
use sc_core::{construction::*,design::DesignMetadataHost};
fn geometry_count<H:DesignMetadataHost,
    S:DraftingInitialInputProvider<H> + DraftingGeometryStepProvider<H> + ?Sized>(
    work:&AdmittedDraftingProgramGeometryWork<'_, '_, '_, '_, '_, '_, '_, '_, '_,H,S>)
    ->Result<usize,String> {
    let proof=work.bind_geometry().map_err(|error|error.to_string())?;
    assert_eq!(proof.requests().len(),proof.suppliers().len());
    assert!(std::ptr::eq(proof.work(),work));
    Ok(proof.suppliers().len())
}
```

`BoundDraftingProgramGeometry` privately owns both complete vectors and borrows the exact admitted
`work()`,which reaches the original canonical/source/order owners and selected geometry policy.
`requests()` and `suppliers()` expose readonly complete slices. Empty occurrence streams succeed
with empty slices and no source callbacks. Individual inspectable supplier variants cannot create
this private complete owner or replace its slots;the proof cannot outlive its original work.

`DraftingProgramGeometryError` retains that same actual work. `refusal()` distinguishes an inline
original reservation cause from a boxed `Binding` refusal. Binding variants retain the zero-based
original occurrence index and exact request:Creator,Existing,PlannedSource or PlannedMismatch.
Existing errors retain their original reference/frame/snapshot witnesses and provider error chain.
PlannedSource preserves its original typed cause. PlannedMismatch retains the entire returned
family field and actual records,plus the first Input,Reference,Frame or Snapshot issue. A later
failure drops all private partial vectors;it exposes no accepted prefix or successful supplier.
Diagnostics omit authored names and opaque payloads. This provides complete static supplier
association,without reading values,issuing identities or executing geometry. Semantic targets,
whole-programme acceptance,allocator faults and coupled verification retain their separate owners.


## Logical source implementation contract

This is the design for `G1-SLICE.5f.2.dependencies.semantic.logical.sources`;the logical models,
semantic operation forms and source facets below retain their individual implementation stages.
Finite logical declarations,inventories and canonical Design attachment are
[implemented](logical-inputs.md). The [standalone Buttonhole operation record](semantic-recipes.md)
and [standalone logical derivation sources](semantic-recipes.md#standalone-logical-derivation-operation)
are implemented. [Raw component origin carriers](semantic-recipes.md#raw-original-component-origins)
retain genuine native declarations or imported-source loans. The
[standalone ordered semantic owner](semantic-recipes.md#original-ordered-semantic-recipe) is
implemented and [attached to the actual Root](semantic-recipes.md#attach-the-original-semantic-cohort-to-design).
Original semantic mixed ordinals,complete coverage and scalar-name order metadata are implemented.
[Actual Design semantic source scopes](semantic-recipes.md#actual-design-semantic-source-scope)
now lend Root logical/operation members after fresh complete Root/provider accounting.
The [selected-Profile source facet](semantic-recipes.md#bind-the-original-selected-profile-source)
now preserves explicit pin/Root/snapshot association after fresh complete content checks.
Complete semantic dependency binding,component binding and Profile parameter/rule facets remain **planned**.
[Raw original Profile/parameter source records](semantic-recipes.md#raw-original-profile-and-parameter-records)
and exhaustive selected-pin text accounting are implemented;Profile parameter binding remains next.
Garment source-content admission does not implement those remaining stages.
The existing numeric formula targets,scalar formula statements and
mixed ordinal metadata cannot complete source binding merely because an identifier matches.
The tracked dependency-plan probe reports those actual producer enum variants before building
its current-library public-owner controls. This separates the source interface that exists from
the records and complete acceptance that still have to be implemented.

The implementation sequence is:

1. **Standalone records implemented:** immutable logical declaration records in core. They retain an explicit semantic role,
   original declared domain,authored state and provenance. Domain members are authored data;core
   supplies no list of physical Pocket opening types,hardware sizes or selected defaults. A finite
   logical selection can name a stable member without interpreting a vendor code as dimensions.
   Unsupported representations retain an explicit source refusal rather than a scalar substitute.
2. **Canonical Design attachment implemented:** integrate those records into actual canonical ownership and identity/resource traversal. Design
   declarations must resolve to their original canonical object. Equal IDs on another inventory
   cannot provide a substitute. Existing numeric-only construction remains an explicitly empty
   logical cohort;it never manufactures an Unknown or default declaration. Logical declarations
   add no formula operand kind,string literal or enum-valued `let` to the existing formula grammar.
3. **Buttonhole/logical records and raw component carriers implemented;binding planned:** represent actual semantic recipe operations for logical derivation,component
   sources and Buttonhole dependencies. They need their own immutable typed input and authentic
   ordered owner before any public source facet can return one. A Buttonhole operation retains
   the canonical button-size declaration and explicit policy dependencies. Its physical formula,
   clearance,geometry and positive executed length stay G3 work;there is no authored hole length.
4. **Semantic owner/Root identity,content and mixed ordinal metadata implemented;semantic binding planned:**
   semantic operations retain genuine original recipe ordinals under complete held-field,shape
   and coverage checks. Normalized/scalar-name-order views loan actual Root members without new
   formula arenas. Typed rule/ID/dependency checking remains separate. A new semantic operation is not one of
   the current direct Point/Edge drafting schemas. It cannot acquire a synthetic native step,
   scalar ordinal,global geometry name or live output. The original thirteen drafting families
   keep their distinct authentic geometry contracts. All new bounds precede child queries or
   result allocation,and all original occurrences remain visible on refusal.
5. **Design member/selected-Profile source scopes implemented;parameter/rule facets planned:** return actual canonical declaration,
   domain/state and operation loans under original Root/snapshot ownership. A Profile source also
   retains its explicit selected profile/schema/content pin and actual canonical inventory;
   another profile or matching parameter ID is insufficient. The source's exhaustive resource
   walk includes all those records,unused entries,duplicated caches and represented counters.
6. Bind complete semantic occurrences only after their complete source/content/work allowances.
   Keep the original Pocket opening and all component source Pieces/copies. A recipe-built
   component retains its actual original recipe/creator;explicit imported primitives retain their
   real source without fabricated history. Unknown state stays Unknown,Derived retains its sole
   operation source,and neither becomes a selected opening or hardware value during static checks.

Logical state must retain the same distinctions already made by canonical scalar inputs:claimed
Known with scoped-evidence references,explicit Assumed with a human-assumption reference,Unknown
with an observation request,Preference with provenance,and Derived with its sole computation
source. Known/Assumed/Preference selections must belong to their actual declared domain. Unknown
and Derived have no authored selection or cached result field. A source's claimed state does not
prove evidence truth,human approval,Profile readiness or physical feasibility. A selectable design
choice keeps an explicit category and selection/provenance,distinct from an unknown fact;no
choice/default supplies a missing observation. It does not add an enum-valued scalar formula.

The logical model slice owns complete domain/state/provenance structural validation and explicit
construction bounds before later inventory attachment. The canonical inventory slice owns actual
membership,identity disjointness and current Root/resource integration. The operation slice owns
typed operation inputs and genuine complete ordered ownership. The port slice owns original
source loans and typed causes. Subsequent semantic work/checking owns complete currentness
workspace,canonical joins,operation existence/kinds/dependencies and complete no-prefix results.
Allocator failures,source lifetimes and coupled acceptance remain `.5f.2.verify` after the agreed
integration set. G4 still owns Profile registry/schema/evidence policy and G3 physical execution;
these logical interfaces cannot manufacture either approval.


## Finite logical declaration records

The [logical input reference](logical-inputs.md#finite-logical-declaration-records) documents
implemented finite domains,authored states,constructor bounds and remaining integration.

## Standalone logical inventory

The [standalone logical inventory reference](logical-inputs.md#standalone-logical-inventory)
documents original membership/kind loans and complete constructor limits. The
[canonical Design attachment](logical-inputs.md#attach-to-the-original-canonical-design) now includes
all-family identity checks and actual/prospective resource walks. Profile/source binding and
semantic operation acceptance retain their separate stages.
