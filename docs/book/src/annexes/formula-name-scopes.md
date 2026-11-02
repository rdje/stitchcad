# Ordered formula name scopes

> **Status:** implemented metadata foundation, G1-SLICE.5b.2d.2. FormulaNameCursor and
> FormulaStatementNameScope enforce authored declaration order and retain actual binding sources.
> Initial,current-statement and [whole recipe proofs](formula-checked-recipes.md) are available,as is
> separate admitted scalar execution. This chapter builds on [initial declarations and exact reads](formula-declarations.md).

A formula statement sees the initial namespace and declarations above it. A current or future let
cannot supply its own RHS, and an assertion label never declares a scalar value. The cursor derives
that scope from an actual [normalized recipe](formula-recipe-inputs.md), preserving its owner,
statement order and original global spans. Callers cannot supply an ordinal, inject a recipe
binding or obtain a final initial namespace from the cursor.

## Visit one actual statement at a time

FormulaNameCursor::new consumes a checked FormulaNamespace and borrows one immutable
FormulaNormalizedRecipe. The initial namespace contains authored sources and all eight fixed
reserved names. current() returns the current FormulaStatementNameScope, or None at the end.
The scope exposes statement_index(), the actual one-based ordinal, and statement(), the original
normalized statement with its name, spans, annotation and expression operands.

resolve() accepts a validated MachineToken and returns an original initial or earlier-let
FormulaDeclaration. Its exact-name behavior and missing-name arguments are the same as the
[initial read API](formula-declarations.md#exact-declared-name-reads). Unknown canonical records and
absent export/profile/size providers retain their declared kinds. The cursor reads no numerical
value, canonical state, provider availability or physical geometry.

```rust
use sc_core::{name::MachineToken, recipe::{
    FormulaKind, FormulaNameCursor, FormulaNamespace, FormulaRecipe,
}};
let source = "let width:length=25 cm let hem:length=width";
let recipe = FormulaRecipe::parse(source).unwrap().normalize_literals().unwrap();
let mut cursor = FormulaNameCursor::new(FormulaNamespace::new([]).unwrap(), &recipe);
let width = MachineToken::new("width").unwrap();
let first = cursor.current().unwrap().unwrap();
assert_eq!(first.statement_index(), 1);
assert!(first.resolve(&width).is_err()); // current let is absent from its RHS scope
assert!(cursor.advance_metadata().unwrap());
let second = cursor.current().unwrap().unwrap();
assert_eq!(second.statement_index(), 2);
assert_eq!(second.statement().name(), "hem");
assert_eq!(second.resolve(&width).unwrap().kind(), FormulaKind::Length);
assert!(cursor.advance_metadata().unwrap());
assert!(cursor.current().unwrap().is_none());
assert!(!cursor.advance_metadata().unwrap());
```

advance_metadata() records only the current let's actual authored annotation and location, then
moves one statement forward. It returns true for an advance and false at the fused end. An assert
moves the cursor but adds no binding, even if its label repeats a value name or reserved spelling.
Assertion positions count toward subsequent let ordinals. Empty recipes and the fixed maximum
of 4,096 statements have the same end behavior.

This is metadata staging. A normalized RHS such as missing or a dimensionally invalid operator is
still unchecked here. Recording its annotation supplies no proof that the expression has that kind
or can execute. The current-statement checker below checks every operand before a validator uses
metadata advance; whole-recipe acceptance remains separate. No accepted expression, recipe graph, numeric
binding or partial geometry is returned by this API.

## Borrow visible metadata without the map

`FormulaNamespace::visible_names()` and `FormulaNameCursor::visible_names()` now return a borrowed
`FormulaNameScope`. It exposes exact `resolve()` and `check_expression()` through the same existing
name/signature checker. The cursor view includes initial and earlier declarations,even after the last
scalar statement. Its private map cannot be extracted or cloned. Copying a scope copies only its loan;
the scope cannot survive mutation/advancement of its original cursor. Debug output omits names/values.

Point and center-arc normalized owners accept `check_arguments_in_scope(scope)`;a normalized
slash/spread opening accepts `check_argument_in_scope(scope)`. These delegate to their original
complete argument factories,retaining actual owners,roles,spans,dependencies and first failures.
Proofs borrow the original expression/declaration owners,so they can survive the cursor allocation.
No source kind,value,provider availability or geometry is inferred from the scope itself.

`FormulaGeometryDeclaration::try_from(actual_declaration)` admits only explicit Point/Edge metadata;
scalar,canonical length,recipe and reserved declarations return unchanged as conversion errors.
`cursor.declare_geometry(metadata)` uses the existing reserved/cross-origin collision rules and
preserves scalar position on both success and refusal. It does not itself prove that the output
is earlier,current,from an actual manifest or in the containing frame. The mixed controller must
bind those sources and order before acceptance;this metadata port supplies no executable prefix.

```rust
use sc_core::{construction::{PointConstruction, PointConstructionNormalizationLimits, PointCoordinate},
    name::MachineToken, ontology::{EntityId, LocalTag, PointRef}, recipe::{FormulaDeclaration,
    FormulaExpressionDocument, FormulaGeometryDeclaration, FormulaKind, FormulaNameCursor,
    FormulaNamespace, FormulaRecipe}};
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let recipe=FormulaRecipe::parse("let width:length=25 mm")?.normalize_literals()?;
let name=MachineToken::new("p")?;
let mut cursor=FormulaNameCursor::new(FormulaNamespace::new([])?,&recipe);
cursor.advance_metadata().map_err(|e|e.to_string())?;
// This explicitly supplied reference is metadata, not a live geometry/source certificate.
let geometry=FormulaGeometryDeclaration::try_from(FormulaDeclaration::point(&name,
    PointRef::new(EntityId::from_bits(80),LocalTag::FIRST))).map_err(|_|"point metadata")?;
cursor.declare_geometry(geometry).map_err(|e|e.to_string())?;
let input=PointConstruction::new(EntityId::from_bits(100),
    FormulaExpressionDocument::new("width".into())?,
    FormulaExpressionDocument::new("1 mm/0".into())?);
let normalized=input.normalized(PointConstructionNormalizationLimits::new(4,10))
    .map_err(|e|e.to_string())?;
let proof=normalized.check_arguments_in_scope(cursor.visible_names()).map_err(|e|e.to_string())?;
drop(cursor);
assert_eq!(proof.coordinate(PointCoordinate::X).kind(),FormulaKind::Length);
# Ok(())
# }
```

`FormulaNamespace::with_initial(extra)` consumes an existing initial map and adds explicit admitted
initial declarations in supplied order,using the same collision rules. Reserved names and original
source pairs stay intact. A late failure returns no extended namespace prefix. This neither admits
additional source content nor certifies its containing owner:complete program/name/source admission
remains the containing controller's responsibility. No recipe declaration can enter as an initial.

## Check the actual current statement

FormulaStatementNameScope::check_kinds takes no replacement statement, namespace or ordinal.
It checks the scope's actual normalized statement against the initial and earlier-let metadata.
The returned FormulaCheckedStatement privately owns component proofs, borrows the same statement
and records its genuine one-based statement_index. It certifies local kinds and dependencies;
it reads no values, canonical states, tolerance availability or physical geometry.

```rust
use sc_core::recipe::{FormulaCheckedStatementKind, FormulaNameCursor,
    FormulaNamespace, FormulaRecipe};
let recipe = FormulaRecipe::parse(
    "let width:length=25 cm assert closure:eps_fmt=width==width"
).unwrap().normalize_literals().unwrap();
let mut cursor = FormulaNameCursor::new(FormulaNamespace::new([]).unwrap(), &recipe);
let first = cursor.current().unwrap().unwrap().check_kinds().unwrap();
cursor.advance_metadata().unwrap();
let second = cursor.current().unwrap().unwrap().check_kinds().unwrap();
drop(cursor);
assert_eq!(first.statement_index(), 1);
assert_eq!(second.statement_index(), 2);
if let FormulaCheckedStatementKind::Assert { left, right } = second.kind() {
    assert_eq!(left.dependencies()[0].declaration().name(), "width");
    assert_eq!(right.dependencies()[0].declaration().name(), "width");
}
```

A let checks its complete RHS and then compares its kind with the declared annotation. An assert
checks the left operand, the right operand and then the closed equality signature. Both must share
one of length, angle, area, ratio or count; Boolean/PointRef/EdgeRef comparisons are refused.
All branches and arguments retain static dependencies, including untaken conditional branches.
Every valid tolerance class remains symbolic metadata. An absent eps_fmt or eps_phys value does
not prevent static acceptance; later runtime context and contribution rules remain required.

FormulaCheckedStatementKind exposes immutable Let or Assert component proofs. Their original
normalized expressions, kinds and ordered sourced dependencies cannot be substituted in the private
enclosing owner. They retain repeated names, canonical record borrows and actual prior let ordinals.
Proofs/errors can survive cursor advancement or its destruction because their declaration locators
are copied and their statement borrow belongs to the recipe. They cannot outlive the normalized
recipe or any borrowed canonical record. A scope itself still borrows cursor metadata.

FormulaStatementCheckError retains statement(), statement_index(), refusal(), token() and span().
FormulaStatementCheckRefusal distinguishes Expression, BindingDimension and AssertionDimension:

| Case | Available arguments and span |
| --- | --- |
| Expression | Binding/AssertionLeft/AssertionRight, original normalized operand and nested node/name/call/dimension refusal; nested node span |
| BindingDimension | FormulaBindingDimensionRefusal with genuine declared_kind, expression_kind and singleton wanted_kind; annotation span |
| AssertionDimension | FormulaDimensionRefusal with actual == operation, both immediate kinds/direct class roles and all wanted signatures; complete original statement span |

The assertion separator has no independently retained span, so the error uses the actual statement
span. No combined normalized comparison expression is invented. Direct grouped tolerance-name
operands retain their class role; computed lengths such as min(eps_num) remain ordinary lengths.
canonical_statement() invokes the exact normalized owner's factory; nested expression errors retain
their own actual expression identity. Debug omits customer text and Display contains only the token.

For let width:boolean=missing, the missing RHS keeps its expression error before the header kind
comparison. For assert gap:eps_num=point_input==missing, the unresolved right child wins before
the complete comparison tuple is available. A known child's own earlier dimension error keeps
precedence over a later unresolved child. Header collisions are refused by current() before any
scope is exposed. A check does not advance metadata, publish a binding or return a checked prefix.

Earlier annotations remain metadata. A caller may stage an unchecked let first:length=missing,
then locally check second:length=first against that annotation. This certifies the second statement's
scope and kinds, and supplies no whole-recipe acceptance. The [whole recipe validator](formula-checked-recipes.md) checks each
statement successfully before staging it and returns only a complete immutable proof.

Current role faults target the certified-root helper expression. Lost role returns its plain
Value kind;forged computed role restores the original first-dependency classification under its
original if-true fault. That forged body is token-identical to the frozen original mutant helper.
All22 case identities/20 other tuples and all other program AST/plans/CLI/classifier/restoration
stay exact. Current22 compiled assertion reds and all7 restored tests pass;all3 sources restore.

```bash
cargo test -p sc-core --test formula_checked_statement_contract -- --nocapture
python3 -I -B docs/tasks/artifacts/formula_structure/statement_owner_contract.py
python3 -I -B docs/tasks/artifacts/formula_structure/checked_statement_mutations.py
```

Seven public contracts exercise48 let pairs,320 class/comparison pairs, source/child priority,
canonical/unknown-record ownership, direct/computed class roles, runtime-invalid static cases and
privacy. Maximum4,096 statements,256-node operands,16 nested conditionals and4,000 grouping pairs
pass on a64KiB stack. Five precise compiler guards in statement_owner_contract.py compile the actual documentation
against Cargo's reported current artifact and require only E0451/E0515 for privacy/recipe/record borrows.
That compiler entry declares its one read-only implementation source, output directory and all
seventeen destinations before fixture reads, directory creation, writes or Cargo dispatch. These
include artifacts.json/build.log and each of five guard source/log/binary paths. The binary slots
remain declared even though an expected compile refusal leaves them absent. Invalid output types,
symlinks and hard links, or a missing source, refuse with status2 before reads or store activation.
Cargo and Rustc preserve prepared stores. Public Cargo JSON selects the current library and every
reported rlib/rmeta input. A pure plan validates those files and dependency search directories
before Rustc; it creates no directories and grants no source-write authority.
The named profile is the unique top-level load of this repository's local_environment.py and must
precede activation. Independent controls require all seventeen files, the sole read-only input,
current library plan and directory plans. They intercept the original six child calls and fixture
bytes. Captures prove dispatch; separate native execution verifies the five precise diagnostics.
The direct entry declares all four inputs, twenty-three ordinary diagnostic files and all three
mutable sources before reads or effects. It requires twenty-two source bindings and count-limited
replacements, including both canonical-owner sites. Children retain prepared stores and the full
seven-test argv. Every source restores exactly before the current full target is consumed.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/checked_statement_mutations.py --classifier-only
```

Classifier-only declares zero files/mutables/native effects. All three input sources may be read-only
hard-linked files; unused fault/restoration aliases are outside that mode's declared file plan.
Current thread panic path/line/byte-column must identify an assertion macro. Custom messages pass;
nonthread/compiler/expect/test-name noise refuses. Current two complete captures,ten physical
refusals,twenty mode faults and an independent legacy-regex fault discriminate omissions,partial
mutable plans,wrong replacement counts and restoration loss. Every failed test body in current
native22 replay originates at a current assertion macro. The known parent join site only propagates
a child assertion;it is not separate valid assertion evidence. Four acceptance/refusal preconditions in three public
tests prevent extraction panics from hiding rejection. Seventy-two frozen inputs/sixty-three unchanged
production Rust files/twenty-three local logs/full seven-test artifact were exact;full
formatting/strict Clippy/seven-hundred-three native tests passed at that historical snapshot.
Current native22/mode proof is complete;broader pre-push verification remains separate. Run mutations exclusively. Existing
18 scope and16 expression fault controls remain required when their implementation changes.

## Binding refusals retain real sources

current() checks a let header before exposing its scope, and advance_metadata() applies that same
check before insertion. A refusal leaves the cursor at the same actual statement with the same
prior declarations. Repeating either call repeats the refusal; a caller cannot skip it, silently
replace an earlier entry or reorder the recipe to make a forward name visible.

FormulaNamespaceError now represents these three binding cases:

| Case | Token | Binding sources |
| --- | --- | --- |
| ReservedBinding | formula_rebinding | fixed reserved metadata, actual attempted declaration |
| AmbiguousName | formula_ambiguous_name | earlier authored input, actual attempted let |
| RecipeRebinding | formula_rebinding | earlier and attempted lets in this same actual recipe |

The initial namespace also uses AmbiguousName for colliding authored pairs, including equal origins.
An assertion label introduces no binding collision. name() and binding_sources() explicitly expose
the refusal's spelling and ordered source pair; default Debug omits authored payload and Display
contains only the stable internal token. The command layer localizes it using these typed arguments.

Recipe sources include both actual one-based statement indices, authored kinds, whole-statement
spans and binding-name spans. Reserved metadata and initial inputs have no recipe ordinal; none is
invented. Canonical record identities/borrows and geometry references remain intact. This follows
[contract5.2.1](../spec/formula-language.md#521-binding-refusal-sources) and the delegated
[D131 reference evidence](formula-static-validation.md#reserved-name-diagnostic-sources).

```rust
use sc_core::recipe::{FormulaDeclarationSource, FormulaNameCursor,
    FormulaNamespace, FormulaNamespaceError, FormulaRecipe};
let source = "let width:length=25 cm assert gap:eps_fmt=width==width let width:angle=90 deg";
let recipe = FormulaRecipe::parse(source).unwrap().normalize_literals().unwrap();
let mut cursor = FormulaNameCursor::new(FormulaNamespace::new([]).unwrap(), &recipe);
assert!(cursor.advance_metadata().unwrap());
assert!(cursor.advance_metadata().unwrap());
let error = cursor.current().unwrap_err();
assert!(matches!(error, FormulaNamespaceError::RecipeRebinding { .. }));
assert_eq!(error.token(), "formula_rebinding");
let [prior, attempt] = error.binding_sources();
assert!(matches!(prior.source(), FormulaDeclarationSource::Recipe { statement_index: 1, .. }));
assert!(matches!(attempt.source(), FormulaDeclarationSource::Recipe { statement_index: 3, .. }));
assert!(cursor.advance_metadata().is_err());
assert_eq!(cursor.current().unwrap_err().name(), "width");
```

The absent export tolerance in that assertion has no effect on metadata staging; its value is a
later runtime obligation. Likewise a reserved-name let refuses against the fixed metadata source
without fetching its provider. Header refusals precede later RHS name/type inspection.

## Ownership and verification

Cursor fields are private. A scope borrows its cursor and actual statement owner, so it cannot
survive an advance or escape either allocation. Successful declaration metadata retains its
original canonical/recipe source lifetime; missing-name diagnostics retain the exact query lifetime.
Default cursor/scope Debug exposes only the position/ordinal. No source text or authored name is
included by default.

Public native and compile-fail contracts check actual owner pointers, ordered visibility, header
refusals, immutable advances, bounded traversal/drop and formatting privacy. Complete historical
fault sets, dispatch controls and restoration receipts remain in
`docs/history/stitchcad-devnotes-part510.md`, retrievable without Git:

```bash
python3 -I -B scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part510.md
cargo test -p sc-core --test formula_ordered_names_contract
python3 -I -B docs/tasks/artifacts/formula_structure/ordered_name_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/ordered_name_mutations.py
```

Classifier-only plans no native child/source/output writes. Run mutations alone and consume both
source restorations before another build/probe. Dispatch captures certify planned effects separately
from actual native behavior; full runtime/geometry acceptance remains a separate owner.

## Mixed drafting names and admission

### Next namespace stages — planned

The mixed namespace/controller is **planned**. The existing actual `DesignInputs::formula_namespace()`
factory checks current inputs and obtains actual parameter declarations,then selected measurement/Ease
host declarations. It supplies reserved metadata and checks collisions. This contract does not silently
supply a complete Profile/Material or geometry-name environment. Additional always-visible sources
and per-operation output names must remain explicit actual inputs with complete retained-content admission.

The existing scalar `FormulaNameCursor` retains its actual whole recipe and genuine scalar ordinal.
Its current-statement scope invokes the existing statement checker with original spans. Native
argument checkers now also accept the [borrowed visible-name scope](formula-name-scopes.md#borrow-visible-metadata-without-the-map)
and delegate to their same kernels. The cursor map stays private,including
against cloning a final namespace out of a prefix,and retain actual scalar declarations. Earlier
geometry insertion remains metadata until the complete controller checks every kind/dependency.

Planned output names must borrow their real creating step and manifest. Point,line,center-arc and
Bézier schemas expose fixed outputs through their actual public schema methods. Variable point/edge
roles have separate typed active-role catalogue views retaining original definitions and membership.
Local catalogue keys are not global formula names. Names and explicit tags are never generated from
vector order,guessed from an operation family or treated as live geometry in the current snapshot.
Catalogue metadata alone grants no canonical history/frame/currentness or evaluated-presence proof.

Borrowed visible-name/kernel adapters,raw initial/output-name owners and complete held-content/
prospective name-work admission are implemented;source/position and ordered metadata integration follow. Complete
program and name/source/catalogue fields,including all retained definitions,active lists,indexes and
text,are admitted before namespace callbacks or child checking. Existing geometry still requires the
actual context's exact source binding;planned geometry stays tied to its earlier original step/manifest.
The resulting metadata stage is not an accepted program. Complete kinds,dependencies and source truth
remain in their separately owned checking stages before values,followed by coupled feature verification.

### Explicit raw name and output sources

`DraftingProgramNameInputs::new(program,initial,outputs)` now retains the actual normalized program
loan and both complete original vectors. `initial()` borrows explicit extra `FormulaInitialDeclaration`
locators;the Root-derived parameter/measurement/Ease environment and reserved metadata compose later.
These remain metadata claims with their original names and source records;construction does not
validate canonical targets,existing geometry,source truth or collisions. Borrowed tokens/records
cannot be replaced while the names are used. `outputs()` borrows all original named output occurrences.

Each `DraftingOutputName::new(position,name,source)` owns an explicit global `MachineToken`,keeps
the requested one-based combined creating position and borrows a `DraftingGeometryNameSource`:
point,line,center-arc,Bézier,active point role or active edge role. Fixed variants retain the actual
original schema and use its published `output()`;variable variants retain authentic typed active
catalogue membership,its complete catalogue and actual definition/tag. `source().reference()` exposes
that exact persistent reference without allocating an identity or constructing geometry.

A global name such as `front_waist` can deliberately differ from a local catalogue role key such as
`waist_corner`;neither spelling is generated from the other. `declaration()` borrows the actual global
name and carries its source's PointRef/EdgeRef as raw formula metadata. No anonymous caller-provided
output reference replaces the original schema/catalogue handle. The role catalogue's metadata
admission still proves neither actual creating-step association nor canonical history/frame/presence.

```rust
use sc_core::{construction::{DraftingContextSource, DraftingGeometryNameSource, DraftingOutputName,
    DraftingProgramNameInputs, NormalizedDraftingProgram, PointConstruction},
    design::DesignMetadataHost, name::{MachineToken, MachineTokenError}};
fn raw_names<'n,'p,'a,H:DesignMetadataHost,S:DraftingContextSource<H> + ?Sized>(
    program:&'n NormalizedDraftingProgram<'p,'a,H,S>,point:&'n PointConstruction)
    -> Result<DraftingProgramNameInputs<'n,'p,'a,H,S>,MachineTokenError> {
    let output=DraftingOutputName::new(2,MachineToken::new("front_waist")?,
        DraftingGeometryNameSource::Point(point));
    let names=DraftingProgramNameInputs::new(program,vec![],vec![output]);
    assert!(std::ptr::eq(names.program(),program));
    Ok(names)
}
```

The helper above requests position2 explicitly;it does not check whether that program position is
this point. Zero,repeated,out-of-range,colliding or mismatched records remain inspectable in raw inputs
for the later complete admission/association stages. Empty vectors still pin the actual program.
Constructors clone neither vectors nor documents,create no namespace,perform no callbacks or values,
and expose no accepted name/program prefix. Default Debug shows counts/positions/source families,
without authored names or formulas. Complete held-content producers and prospective name-work admission are available below;
source/position association and ordered visibility remain separate implementation stages.

### Additional held-field charges

`DraftingProgramNameInputs::charge_extra_resources(sink)` now charges its complete additional input
component. The initial vector precedes original name text and actual canonical Length evidence
vectors;fixed declaration/source identifiers carry no extra dynamic content. The output vector
precedes each explicit global token and its original source. Fixed schemas reuse their existing
complete held-field producers,including both buffers of every Point/Arc argument document.

A typed active-role source charges its whole actual catalogue,including inactive definitions and
keys,original active list,retained tag index,active index and active associations. Complete collection
lengths are charged before nested keys;prior peak admission-work counts do not substitute for actual
held fields. For two definitions and one active role,the actual raw2+1 and retained2+1+1 populations
are seven entries. Reusing that catalogue twice counts both logical source occurrences;no address or
identity deduplication removes input. Point/edge domains retain distinct static field paths.

For example,one initial Length name with two evidence entries,then two output names using that same
catalogue,requires `1 + 2 + 2 + 7 + 7 = 19` entries. All inactive and repeated key text counts too.
Resource locations preserve actual metadata/canonical/operation owners and collection-specific
original indexes. Empty components preserve an earlier latched refusal. The walk clones/formats
nothing,looks up no source,reads no numeric availability and grants no whole-name/program acceptance.

```rust
use sc_core::{construction::{DraftingContextSource, DraftingProgramNameInputs},
    design::{DesignMetadataHost, DesignResourceDemand, DesignResourceLimitError,
    DesignResourceLimits, DesignResourceMeter}};
fn extra_demand<H:DesignMetadataHost,S:DraftingContextSource<H> + ?Sized>(
    names:&DraftingProgramNameInputs<'_, '_, '_,H,S>,policy:DesignResourceLimits)
    -> Result<DesignResourceDemand,DesignResourceLimitError> {
    let mut meter=DesignResourceMeter::new(policy);
    names.charge_extra_resources(&mut meter)?;
    meter.finish()
}
```

This demand covers additional inputs;the actual containing Root/context/raw program and retained
normalized arenas compose through the complete producer below. Prospective name-work admission is
also available;creating-step associations remain the next stage before namespace construction/checking.

### Complete retained program charges

`NormalizedDraftingProgram::charge_resources(sink)` composes a fresh walk of the actual Root,
context source,mixed vector and native fields with all retained source,scalar and native arenas.
It precharges each actual source/statement/slot/node vector and each nested call-index vector.
Borrowed names/source text and fixed Copy literals own no additional text or counter buffers.
Repeated native steps count their separate arenas;unused sources and untaken branches count too.

For the [complete seven-step program example](drafting-context-sources.md#complete-content-shape-and-original-order),
the actual extra population is source owners2,source
nodes11,call indices2,scalar statements2,scalar nodes8,mixed slots7 and native nodes12:44 entries.
The conservative84-record normalization allowance is a different quantity and never substitutes
for this held-content walk. Source locations retain source identity;native locations retain actual
operation identity and original zero-based mixed index. A nested call-vector location uses its
semantic node index. Empty programs still charge their actual Root/context and preserve first refusal.

```rust
use sc_core::{construction::{DraftingContextSource, NormalizedDraftingProgram},
    design::{DesignResourceDemand, DesignResourceLimitError, DesignResourceLimits,
    DesignResourceMeter, DesignResourceSource}};
fn program_demand<H:DesignResourceSource,S:DraftingContextSource<H> + ?Sized>(
    program:&NormalizedDraftingProgram<'_, '_,H,S>,policy:DesignResourceLimits)
    -> Result<DesignResourceDemand,DesignResourceLimitError> {
    let mut meter=DesignResourceMeter::new(policy);
    program.charge_resources(&mut meter)?;
    meter.finish()
}
```

This producer performs no normalization,source currentness,geometry lookup or numeric evaluation.
The additional-name producer composes separately into the same cumulative sink;complete content
and prospective name-work bounds must still precede namespace construction and ordered checking.

### One cumulative complete name-content walk

`DraftingProgramNameInputs::charge_resources(sink)` now composes the two complete producers in
order:actual normalized program first,additional initial/output names and their original sources
second. The same sink retains all earlier consumption and its original first refusal. Repeated
source content counts in both logical occurrences;no old admitted demand provides credit.

For the six-source example,the empty program's normalized vectors add zero entries but its actual
Root/context remain. The17 additional entries and all name/document/key bytes follow those fields.
A budget one entry below the complete sum refuses the final edge-role association at output index5
with the original operation owner;its consumed count includes the earlier Root/context entries.
A Root text refusal precedes context charging and every additional-name field. Empty inputs still
preserve an earlier cumulative failure.

```rust
use sc_core::{construction::{DraftingContextSource, DraftingProgramNameInputs},
    design::{DesignResourceDemand, DesignResourceLimitError, DesignResourceLimits,
    DesignResourceMeter, DesignResourceSource}};
fn complete_name_demand<H:DesignResourceSource,S:DraftingContextSource<H> + ?Sized>(
    names:&DraftingProgramNameInputs<'_, '_, '_,H,S>,policy:DesignResourceLimits)
    -> Result<DesignResourceDemand,DesignResourceLimitError> {
    let mut meter=DesignResourceMeter::new(policy);
    names.charge_resources(&mut meter)?;
    meter.finish()
}
```

Success accounts held logical content only. It performs no namespace construction,position/source
association,currentness check or value query. Prospective namespace work is admitted below;
source/position association and whole checking remain later stages.

### Admit complete prospective name work

`DraftingProgramNameInputs::admit_name_work(&limits)` freshly admits complete held content first.
Only then does it count original Root parameters,actual selected measurement/Ease rows,extra initial
names,output names and scalar statements. Every addition is checked. The conservative namespace
bound includes all eight reserved names and a slot for every statement,including assertions whose
labels never declare names. The retained-record bound adds one association slot per output name.
`DraftingProgramNameLimits` selects cumulative content,namespace entries and retained records
explicitly;it has no Default. Zero name limits refuse reserved metadata even for empty input.

`DesignFormulaDeclarationSource` supplies the actual selected host population through a borrowed
exact-size,fused iterator. Its creation,length and drop allocate nothing and validate/consume no
row. Core counts its actual length before namespace work;it never calls the older allocating
`DesignFormulaHost::formula_declarations()` factory on this bounded path. Each original row remains
an item even when its canonical binding fails. Later iteration returns its actual borrowed
Measurement/Ease token,metadata identity and dedicated canonical Length record,or original typed
host cause. Unknown values remain Unknown;no general scalar Length or numeric fallback substitutes.
Unprovided selections have no rows;missing selected objects remain later Root-reference refusals.
The host's immutable exhaustive integration contract is required;this is no dishonest-host defense.

For example,reserved8 + Root parameters1 + selected host rows2 + extra initial1 + outputs2 + scalar
statements2 =16 namespace entries. The two output associations give18 prospective retained records.
The assertion's slot is conservative. Limits15/18 refuse the complete16 namespace population;
limits16/17 refuse the complete18 retained population. Neither attempt advances the host iterator,
validates current sources,builds a namespace or returns a prefix. A content refusal precedes even
iterator construction and zero name quotas. Population overflow precedes both name quotas.

```rust
use sc_core::{construction::{DraftingContextSource, DraftingProgramNameInputs,
    DraftingProgramNameLimits, DraftingProgramNameShape},design::{DesignFormulaDeclarationSource,
    DesignResourceDemand, DesignResourceSource}};
fn bounded_names<H:DesignResourceSource+DesignFormulaDeclarationSource,
    S:DraftingContextSource<H> + ?Sized>(names:&DraftingProgramNameInputs<'_, '_, '_,H,S>,
    policy:DraftingProgramNameLimits)->Result<(DesignResourceDemand,DraftingProgramNameShape),String> {
    let admitted=names.admit_name_work(&policy).map_err(|error|error.to_string())?;
    Ok((admitted.demand(),admitted.shape()))
}
```

`AdmittedDraftingProgramNameWork` has private fields and retains the actual original names/program,
exact policy,fresh demand and complete shape. Providers need neither Clone nor Debug;diagnostic
formatting omits names. The owner cannot outlive its input loan. Invalid requested output positions
and unbound extra sources remain data here:source/position association,collisions,ordered visibility,
complete kinds and dependency truth require their separately owned stages. These bounds grant
logical namespace/association records only;allocator capacity,comparison CPU,child proof work and
runtime/geometry budgets are separate.
