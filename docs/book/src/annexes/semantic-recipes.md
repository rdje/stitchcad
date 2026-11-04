# Semantic recipe sources

## Standalone Buttonhole operation

`ButtonholeOperation` now retains an immutable original semantic recipe declaration. Its
`ButtonholeOperationDefinition` supplies the operation ID, exact `NotionSize` button origin,
explicit physical-policy source reference, and complete ordered `Vec<ButtonholePolicyInput>`.
The button stays either Declaration or Profile. A hardware designation is not interpreted as a
diameter, and a Profile parameter reference supplies no selected-profile or schema approval.

There is no independently authored hole-length field and no cached or executed result. The
physical policy must later resolve from its explicit source. Core supplies no formula, clearance,
hardware vocabulary or policy default. G3 executes the actual policy against resolved inputs and
proves a positive physical length. The Closure's existing `ButtonholeDerivation` names this kind
of operation; matching IDs alone proves no membership, kind, dependencies or currentness.

Each policy input retains an exact local `MachineToken` role and numeric `CanonicalInputTarget`.
Dedicated Length and general Scalar targets stay distinct, including a general Length-kind scalar.
These are canonical references, not separately owned values, copied uncertainty states or global
formula/geometry names. Construction performs no target lookup or policy-signature check. Distinct
roles may share a target; repeated roles refuse even for identical targets. Core infers no role
meaning from its spelling. For example, `policy_a` is a neutral role and supplies no physical rule:

```rust
use sc_core::{name::MachineToken,ontology::{EntityId,NotionSize},recipe::*,value::*};
# fn main()->Result<(),Box<dyn std::error::Error>> {
let button=NotionSize::Declaration(EntityId::from_bits(30));
let operation=ButtonholeOperation::new(ButtonholeOperationDefinition {
    id:EntityId::from_bits(10),button,policy:EntityId::from_bits(900),
    inputs:vec![ButtonholePolicyInput {
        role:MachineToken::new("policy_a")?,
        target:CanonicalInputTarget::Length(EntityId::from_bits(20)),
    }],
},ButtonholeOperationLimits::new(1,8,1))?;
assert_eq!(operation.definition().button,button);
assert_eq!(operation.shape().policy_inputs(),1);
assert_eq!(operation.shape().role_text_bytes(),8);
# Ok(()) }
```

Select `ButtonholeOperationLimits::new(policy_inputs,role_text_bytes,validation_inspections)`
explicitly; there is no Default. Core derives complete original P, role bytes and checked P*P
comparison bound first. Arithmetic overflow refuses before quotas. Complete input, text and
comparison quotas are checked in that order, before duplicate-role validation. Invalid repeated
roles count in every population. These are logical counts, bytes and conservative comparisons,
not allocator capacity, string-comparison byte work, CPU time or execution limits. Caller-owned
vectors and strings already exist before construction.

With `[policy_b,policy_a]`, exact allowances2/16/4 pass and preserve order. Allowance1 refuses
complete inputs2; text15 refuses16; comparisons3 refuses4. With `[policy_b,policy_b]`, the same
quota failures precede DuplicateRole with first0/duplicate1. An explicit empty vector with0/0/0 is
legal. It still needs original button and policy fields; it invents neither a selected size nor
a physical policy with no dependencies.

Validation uses borrowed prefix scans and allocates no workspace. The successful operation moves
the original input vector and role strings unchanged. `definition()` returns that immutable data;
`id()`, `limits()` and `shape()` retain original identity and factory metadata. Authored equality
compares definitions, so changing constructor allowances cannot change semantic content. Debug
omits role text. `ButtonholeOperationError` retains operation ID, selected bounds and typed refusal.
Limit retains resource/allowed/required; DuplicateRole retains both original positions without
copying authored role text into diagnostics.

## Account for actual Buttonhole recipe content

`charge_buttonhole_operation_resources(&operation,sink)` precharges the actual complete policy-input
vector, then charges each original role token in authored order. It invents no containing recipe
entry or position. Button origin, policy reference, numeric targets, constructor policy and factory
shape are fixed Copy fields; cached counts supply no demand credit. Exhaustive original owner,
definition and input views expose accounting obligations when fields change. The producer performs
no source query, currentness check, value read or clone. The caller owns and finishes the meter.

```rust
use sc_core::{design::*,recipe::ButtonholeOperation};
fn operation_content(operation:&ButtonholeOperation,limits:DesignResourceLimits)
    ->Result<DesignResourceDemand,DesignResourceLimitError> {
    let mut meter=DesignResourceMeter::new(limits);
    charge_buttonhole_operation_resources(operation,&mut meter)?;
    meter.finish()
}
```

Locations use Inputs scope and the original operation ID. `semantic.buttonhole.inputs` has index
None and charges the complete vector; `semantic.buttonhole.inputs.role` retains each original
input index. The two-role example needs2 entries/16 text bytes/0 counter bits regardless of wider
constructor allowances. Prior1 entry,3 text bytes and2 bits remain3/19/2. Entry allowance1 refuses
vector2; text15 refuses second role8 after8 bytes. Empty and nonempty producers preserve the same
earlier latched refusal.

This is a standalone source record and exhaustive content walk. The semantic owner below now
retains original declaration order and is attached to the actual Root below; component origin
binding, combined-program integration,
Closure-to-operation dependency binding and readonly source ports remain tracked work. It creates
no scalar statement, native drafting slot or geometry. Profile pins, currentness, physical execution
and the deferred coupled verification remain separate.

## Standalone logical derivation operation

`LogicalDerivationOperation` now retains an immutable original typed logical recipe declaration.
`LogicalDerivationDefinition` supplies operation ID, explicit rule-source reference, a logical
output target and complete ordered named inputs. It stores no copied domain, uncertainty state,
selected member or cached result. The output's actual canonical declaration owns its original
domain and Derived operation reference. Later binding must check that actual member, expected
kind and operation association; matching IDs alone supply no original-source proof.

`LogicalInputTarget {declaration,kind}` is a raw logical reference and expected role, separate from
numeric `CanonicalInputTarget`. Resolve it through the actual `LogicalInputs` owner using
`resolve(target.declaration,target.kind)`. The raw target itself grants no membership or value.

Each `LogicalDerivationInput` has a local `MachineToken` role and one typed source:

| Source | Original fields |
| --- | --- |
| Logical | Design logical target with declaration ID and expected logical kind |
| Profile | Original Profile parameter reference and expected logical kind |
| Numeric | Original dedicated Length or general Scalar canonical target |

Profile input IDs supply no selected-profile, schema or content pin. Numeric inputs do not turn
logical selections into scalar operands. The original rule reference does not imply a rule body,
signature, default selection or new language syntax. Actual rule/policy source loans and typed
signatures must be checked separately before complete dependency acceptance; execution remains
outside this standalone constructor. No enum-valued formula `let` or native drafting slot is added.

For example, a neutral local role references a logical source without copying its domain or value:

```rust
use sc_core::{name::MachineToken,ontology::EntityId,recipe::*,value::*};
# fn main()->Result<(),Box<dyn std::error::Error>> {
let output=LogicalInputTarget {
    declaration:EntityId::from_bits(70),kind:LogicalInputKind::PocketOpening,
};
let operation=LogicalDerivationOperation::new(LogicalDerivationDefinition {
    id:EntityId::from_bits(10),output,rule:EntityId::from_bits(900),
    inputs:vec![LogicalDerivationInput {
        role:MachineToken::new("source_a")?,
        source:LogicalDerivationInputSource::Logical(LogicalInputTarget {
            declaration:EntityId::from_bits(20),kind:LogicalInputKind::PocketOpening,
        }),
    }],
},LogicalDerivationLimits::new(1,8,1))?;
assert_eq!(operation.definition().output,output);
assert_eq!(operation.shape().inputs(),1);
# Ok(()) }
```

Select `LogicalDerivationLimits::new(inputs,role_text_bytes,validation_inspections)` explicitly.
There is no Default. Complete actual P, role text and checked P*P comparison arithmetic precede
input, text, then comparison quotas. Only then does the borrowed prefix scan reject the first
repeated role. No validation workspace is allocated. Input buffers already exist; counts and
comparisons do not bound allocator capacity, string-comparison byte work, CPU or runtime execution.
Distinct roles may share a source. An explicit empty vector with0/0/0 retains its original output
and rule references, without inventing a zero-dependency rule body or selected result.

For four roles `source_d,source_c,source_b,source_a`, exact4/32/16 passes. Inputs3 refuses complete4,
text31 refuses32, comparisons15 refuses16, before any duplicate-role test. Repeating the first role
at the final position with full allowances refuses DuplicateRole first0/duplicate3, even when those
entries use different source kinds. Original order, vector and role buffers survive successful
construction unchanged. `definition()`, `id()`, `limits()` and `shape()` retain original data and
factory metadata. Authored equality ignores selected constructor allowances. Diagnostics omit role
text; `LogicalDerivationError` retains operation ID, limits and typed original quota/role cause.

## Account for actual logical recipe content

`charge_logical_derivation_resources(&operation,sink)` precharges the actual complete typed-input
vector, then every original role's bytes in authored order. It charges no invented containing recipe
entry. Output/rule references, all three source variants and constructor metadata are fixed Copy
fields. Exhaustive owner/definition/input/source views require a new accounting decision when held
fields change. Cached shapes supply no demand credit. No canonical/domain/value, rule signature,
Profile/currentness or graph query occurs; no clone, accepted source prefix or selected result appears.

```rust
use sc_core::{design::*,recipe::LogicalDerivationOperation};
fn logical_operation_content(operation:&LogicalDerivationOperation,limits:DesignResourceLimits)
    ->Result<DesignResourceDemand,DesignResourceLimitError> {
    let mut meter=DesignResourceMeter::new(limits);
    charge_logical_derivation_resources(operation,&mut meter)?;
    meter.finish()
}
```

Locations use Inputs scope and the original operation ID. `semantic.logical.inputs` charges the
whole vector with index None; `semantic.logical.inputs.role` retains the original input index.
The four-role fixture costs4 entries/32 text bytes/0 counter bits. Prior2 entries,3 bytes and1 bit
finish6/35/1. Entry allowance3 refuses the original vector4; text31 refuses final role8 after24 bytes
at original index3. Empty and nonempty walks preserve an earlier latched cause. The caller finishes
the same cumulative meter. Standalone accounting grants no ordered owner, Root/Profile/rule binding,
graph acceptance or execution. The semantic owner below retains original declaration order;
component origin binding and source ports remain tracked separately.

## Raw original component origins

Component origin carriers now preserve actual original loans before any provider query or binding
API is introduced. `DraftingComponentCreator` has Mirror and Join variants holding their real
`DeclaredMirrorPieceIdentity` or `DeclaredJoinPieceIdentity` factory result. A geometry role catalog
or matching ID cannot construct either variant. `piece_id()` borrows the exact declared identity
field; `input()` follows the published family parents to the actual original `MirrorPieceCopy` or
`JoinPiecesAtSeam` input. It performs no lookup, cloning, identity issuance or position inference.
These are native declaration witnesses, not evidence of executed creation, history or geometry.

`DraftingComponentRecipeSource` retains the actual supplied Piece, original `DraftingProgramInputs`
owner and that native creator witness. The complete scalar recipe and native sequence stay in their
original owner. No extracted source, synthetic ordinal, reconstructed operation or replacement
recipe is created. Construction is raw: a foreign program, wrong Piece or mismatched creator can
still be represented for later typed refusal. This record itself certifies no program membership,
current Root, accepted creation history, output geometry or valid original source position.

```rust
use sc_core::{construction::*,design::DesignMetadataHost,ontology::Piece};
fn recipe_source<'p,'a,H:DesignMetadataHost,S:DraftingContextSource<H>+?Sized>(
    program:&'p DraftingProgramInputs<'a,H,S>,piece:&'p Piece,creator:DraftingComponentCreator<'p>)
    ->DraftingComponentRecipeSource<'p,'a,H,S> {
    DraftingComponentRecipeSource {program,piece,creator}
}
```

`DraftingImportedComponentSource` instead retains an actual primitive Piece and original
`DraftingSourceRecord<I>` containing its explicit import-source payload. The integration supplies
that real record; an empty substitute or made-up recipe violates the source contract. Core infers
no importer, file format, imported geometry or historical creator from this raw carrier. The payload
may be opaque or unsized and need not implement Clone or Debug. Copying the carrier copies loans.

```rust
use sc_core::{construction::*,ontology::Piece};
fn imported_source<'a,I:?Sized>(piece:&'a Piece,source:&'a DraftingSourceRecord<I>)
    ->DraftingImportedComponentSource<'a,I> {
    DraftingImportedComponentSource {piece,source}
}
```

`DraftingComponentOrigin` selects Imported or Recipe explicitly; `piece()` loans the Piece retained
by that original branch. `DraftingComponentSource` adds the actual original Cartesian frame and
snapshot records, each with the provider's associated type. Those record loans retain identity
even for zero-sized payloads. Debug omits primitive/import payloads, names and authored data. None
of the generic context or payload types needs Clone or Debug for these carrier operations:

```rust
use sc_core::{construction::*,design::DesignMetadataHost};
fn copy_source<'p,'a,H,S,I>(source:DraftingComponentSource<'p,'a,H,S,I>)
    ->(DraftingComponentSource<'p,'a,H,S,I>,DraftingComponentSource<'p,'a,H,S,I>)
where H:DesignMetadataHost,S:DraftingContextSource<H>+?Sized,I:?Sized {
    (source,source)
}
```

All carrier operations are constant-work borrowed field access. They allocate no collection or
accepted prefix, compute no geometry and query no source. They grant no content allowance. The
future source provider must freshly account every actual additional recipe input, manifest,
primitive-source payload, duplicated record and cache before bounded binding queries; earlier
native constructor proofs provide no whole-source credit. Complete Root Piece/copy membership,
actual recipe/native-input association, frame/snapshot identity and original first causes remain
with the tracked component binding task after source ports/accounting exist. Other creation forms
need genuine supported source records; these carriers supply no synthetic native history for them.

## Original ordered semantic recipe

`SemanticRecipe` now owns the exact supplied `Vec<SemanticOperation>` in authored order.
The implemented variants are Buttonhole and LogicalDerivation, each retaining its original
immutable operation record and constructor metadata. `SemanticOperation::id()` and `kind()`
report that actual original declaration. Scalar statements and native drafting inputs retain their
separate original owners. Raw component recipe carriers continue to borrow genuine native families;
they do not become invented semantic/native slots in this vector.

Select `SemanticRecipeLimits::new(operations,validation_inspections)` explicitly; there is no
Default. `SemanticRecipe::empty()` explicitly selects the same empty cohort with exact0/0 bounds;
it supplies no policy for nonempty input and invents no declaration. The factory derives complete
original N and checked conservative N*N identity comparisons
before operation-count then comparison quotas. Only after both pass does it scan for the first
repeated owned identity in authored order. Identical definitions and different-family records
with the same ID both refuse. Rule, policy, output and input IDs are references, not additional
owned operations. Validation uses borrowed prefix scans without workspace allocation. Input buffers
already exist; these counts/comparisons do not bound allocator capacity, CPU, queries or execution.

For N3, exact3/9 passes. Count2 refuses complete3 before duplicate scans; comparison8 refuses9.
IDs `[90,90,80]` with full allowances refuse DuplicateIdentity first0/duplicate1 even when the
first two operations have different families. IDs `[90,70,90]` retain first0/duplicate2. An empty
vector with explicit0/0 is legal and creates no unknown, default or placeholder operation.

The original vector and each operation's authored content remain unchanged. `operations()` returns
the complete readonly slice; `operation(id)` loans only an actual owned member. A shared rule ID900
does not become a member merely because two operations reference it. `limits()` and `shape()`
retain the original selected bounds and complete population; authored equality compares operations
and ignores constructor allowances. Debug omits authored local role text.

`resolve(id,expected_kind)` returns a private `SemanticOperationBinding` retaining the actual
recipe/member loans and original zero-based semantic-vector index. That index is neither a scalar
ordinal, native step nor combined-program position. Missing retains the requested identity/family;
Kind retains the expected family and actual original member loan. No matching-kind alternative,
copied definition or fallback substitutes. Equal-data recipes can compare equal while their bindings
still refer to different owners and members. Root/source consumers must bind that exact owner
separately. A caller returning an owned static error may render the native lending error explicitly:

```rust
use sc_core::{ontology::{EntityId,NotionSize},recipe::*};
# fn main()->Result<(),Box<dyn std::error::Error>> {
let operation=ButtonholeOperation::new(ButtonholeOperationDefinition {
    id:EntityId::from_bits(90),button:NotionSize::Declaration(EntityId::from_bits(903)),
    policy:EntityId::from_bits(900),inputs:vec![],
},ButtonholeOperationLimits::new(0,0,0))?;
let recipe=SemanticRecipe::new(vec![SemanticOperation::Buttonhole(operation)],
    SemanticRecipeLimits::new(1,1))?;
let binding=recipe.resolve(EntityId::from_bits(90),SemanticOperationKind::Buttonhole)
    .map_err(|e|e.to_string())?;
assert!(std::ptr::eq(binding.recipe(),&recipe));
assert_eq!(binding.index(),0);
# Ok(()) }
```

## Account for the complete semantic recipe

`charge_semantic_recipe_resources(&recipe,sink)` precharges the actual complete operation vector,
then invokes the exhaustive actual Buttonhole/logical member walks in original order. The whole
vector uses Inputs scope, field `semantic.operations`, owner None and index None. Nested fields
retain each original operation ID and its actual local-input index; no containing mixed position
is invented. Exhaustive owner/variant/member views expose new held-field obligations. Fixed Copy
constructor metadata supplies no cached-shape credit. The caller owns and finishes the same meter.

```rust
use sc_core::{design::*,recipe::SemanticRecipe};
fn semantic_content(recipe:&SemanticRecipe,limits:DesignResourceLimits)
    ->Result<DesignResourceDemand,DesignResourceLimitError> {
    let mut meter=DesignResourceMeter::new(limits);
    charge_semantic_recipe_resources(recipe,&mut meter)?;
    meter.finish()
}
```

For Buttonhole90 with two eight-byte roles, Logical70 with one eight-byte role, then Buttonhole80
with explicit empty dependencies, actual demand is6 entries/24 text bytes/0 bits:3 operations plus
3 member input entries. Prior2 entries/3 bytes/1 bit finish8/27/1. Entry allowance2 refuses complete
operations3. Allowance4 reaches Buttonhole90 inputs2 after3 entries; allowance5 reaches Logical70
inputs1 after5 entries. Text23 refuses its role8 after16 bytes at owner70/index0. Every original
first field survives; empty and nonempty recipes preserve any earlier latched sink refusal.

The owner/producer also compose into the actual Root below. Combined program order, rule/policy
signatures, complete source/graph binding and execution remain separate
tracked work. No record acquires a fabricated mixed/native/scalar position or whole-source allowance.

## Attach the original semantic cohort to Design

`DesignInputsDefinition::semantic_recipe` is now a required original component. Omitting it fails
compilation; no constructor fills an empty/default cohort for the caller. Numeric-only roots
explicitly supply `SemanticRecipe::empty()` with exact0/0 bounds. `DesignInputs::semantic_recipe()`
loans the actual retained owner, including every original operation, role and constructor record.
Moving the cohort into the Root preserves its operation buffer. No equal-data replacement is
manufactured, and member bindings continue to loan this exact original owner.

This complete construction example supplies the actual semantic owner alongside the other components:

```rust
use sc_core::{design::{DesignInputs,DesignInputsDefinition,DesignInputsError,DesignMetadata,
    DesignReferenceHost,DesignSourceReferences,DesignParameters,DesignStructure},
    ontology::MaterialInventory,recipe::{FormulaRecipeDocument,FormulaSources,SemanticRecipe}};
fn combine<H:DesignReferenceHost>(structure:DesignStructure,parameters:DesignParameters,
    recipe:FormulaRecipeDocument,formula_sources:FormulaSources,semantic_recipe:SemanticRecipe,
    materials:MaterialInventory,host:H,references:DesignSourceReferences)
    ->Result<DesignInputs<H>,DesignInputsError<H::Error>> {
    let sources=DesignMetadata::new(host,structure.inputs())
        .map_err(|issue|DesignInputsError::Sources(Box::new(issue)))?;
    DesignInputs::new(DesignInputsDefinition {structure,parameters,recipe,formula_sources,
        semantic_recipe,materials,sources,references})
}
```

The complete ownership census visits structural entities, parameters, formula sources, semantic
operations, Materials, then higher metadata. Every semantic operation ID is an owner even when
unused. `DesignInputsPosition::SemanticOperation {index}` retains its original zero-based semantic
position. Rule, policy, output and input references remain uses. For example, operation10 aliasing
dedicated length10 refuses at Structure(Length0)/SemanticOperation0. Operation90 aliasing logical
declaration90 refuses at Structure(Logical0)/SemanticOperation0. Formula source70 and operation70
refuse at FormulaSource0/SemanticOperation0; parameter70 precedes that same operation as Parameter0.
These are actual original positions, not a synthetic scalar/native/mixed index.

All actual and prospective input/garment resource paths compose the complete semantic walk after
scalar recipe and formula-source content. The source host and later fields remain in the same
cumulative sink. Detached candidate reconstruction preserves the complete semantic cohort.
Existing metadata command difference guards also compare all authored semantic content;
measurement/material/notch edits cannot silently change it while reporting only allowed fields.

For an explicitly empty scalar recipe, canonical Unknown length10 and Boolean20, logical Derived90
with domain `option_a`, then Buttonhole70 and LogicalDerivation80 each holding one eight-byte role,
the represented Inputs Root requires8 entries/32 text bytes/3 revision bits at revision7. The
canonical domain owns8 bytes, retained scalar-recipe canonical identity8, and semantic roles16.
Entry allowance6 reaches Buttonhole70 inputs1 after6 entries. Text31 reaches Logical80 role8 after24
bytes. The original owner/field/index survives both direct accounting and formula-scope resource
refusal, before later source queries.

A separate prospective fixture has one Known length/evidence1 and Buttonhole70/input1. Actual
Root demand is4/16/3; replacing only that length with Known evidence2 requires5/16/3. Entry allowance4
refuses `semantic.buttonhole.inputs` after4 entries, proving the unchanged semantic content remains
in the projected walk and old/new evidence is not double-counted. The original Root stays unchanged.

This attachment establishes owned data,identity disjointness across represented components and complete held-field
accounting. Semantic rule/policy existence/signatures,logical domain/dependency binding,Profile pins,
source currentness and graph acceptance remain with later bounded source integration. Genuine
combined-program semantic ordinals are implemented below. No source values or
physical buttonhole/logical results are computed by Root construction or resource accounting.

## Original semantic positions in the mixed program

`DraftingProgramStep::Semantic {ordinal}` now retains an exact requested one-based ordinal in
the actual context Root's complete `SemanticRecipe`. `DraftingProgramInputs::semantic_recipe()`
lends that same owner. Raw construction preserves zero,out-of-range,repeated,reordered or missing
requests without queries or acceptance. A semantic ordinal is independent of scalar recipe
ordinals,native family identities and the containing one-based mixed position.

Whole input admission freshly charges every Root semantic body,including unused operations,
then context source,mixed vector and native inputs. Complete expression/node/record bounds pass
before either recipe's order check. Semantic bodies introduce no scalar expressions or normalized
argument arenas;they stay in the actual Root. Each requested semantic slot counts as a mixed step.
`DraftingProgramShape::semantic_operation_count()` reports the complete actual Root population,
separate from `statement_count()`. The retained bound remains
`steps + scalar_statements + expressions + 2*nodes` because semantic slots borrow Root members.

Admission requires scalar ordinals1..S and semantic ordinals1..N independently,each exactly once
in its original relative order. Their interleaving with native requests is authored data. It
checks requests in genuine mixed order and reports the first ScalarOrder or SemanticOrder with
position,expected ordinal and original request. Expected None means that entire cohort already
occurred. After scanning,missing scalars precede missing semantics. There is no accepted prefix.

For a Root with one scalar statement and two semantic operations,this sequence passes recipe coverage:

| Mixed position | Original source |
| --- | --- |
| 1 | Semantic operation1 in the actual Root semantic recipe |
| 2 | Scalar statement1 in the actual complete scalar recipe |
| 3 | Native Point declaration,with two original scalar argument documents |
| 4 | Semantic operation2 in the actual Root semantic recipe |

For one-node scalar and Point arguments,this requires4 steps,1 scalar statement,2 semantic
operations,3 expressions,3 nodes and14 prospective records. Record limit13 refuses before order
checking. Replacing the final semantic ordinal2 with1 refuses at mixed position4,expected2/requested1.
Ending after semantic1 reports MissingSemantics required2/provided1. Requesting semantic1 from an
explicitly empty semantic Root refuses with expected None;no operation is synthesized.

Normalization reserves the complete mixed slot vector before child factories. Its `operations()`
slice contains None for scalar and semantic positions,and Some for native owners. `steps()` keeps
the distinction: `DraftingProgramStepKind::Semantic {ordinal,operation}` loans the exact immutable
Root member. Direct ordinal lookup preserves that pointer under nth,skip,reverse and interleaved
iteration. No semantic body is cloned into an arena. Debug exposes ordinal/kind without local role
text. For example,this helper returns all three distinct pieces of source information:

```rust
use sc_core::{construction::*,design::DesignMetadataHost,recipe::SemanticOperation};
fn original_semantic<'a,H:DesignMetadataHost,S:DraftingContextSource<H>+?Sized>(
    program:&NormalizedDraftingProgram<'_, 'a,H,S>,mixed_index:usize)
    ->Option<(usize,usize,&'a SemanticOperation)> {
    let step=program.steps().nth(mixed_index)?;
    match step.kind() {
        DraftingProgramStepKind::Semantic {ordinal,operation} =>
            Some((step.position(),ordinal,operation)),
        DraftingProgramStepKind::Scalar {..} | DraftingProgramStepKind::Drafting(_) => None,
    }
}
```

The scalar namespace/order checker retains `DraftingProgramOrderProofKind::Semantic` with that
same original ordinal/member. It adds no formula declaration,FormulaKind,global name or metadata
cursor advance. Formula-name dependency iteration continues to stream only actual scalar/native
formula uses;semantic role/ID inputs have no fabricated AST span. Their typed rule,logical,Profile,
numeric and operation dependency binding remains a later source-integration stage. Passing this
scalar name check does not establish semantic dependency order or a complete executable program.

Semantic slots cannot issue native geometry output names:output-position binding returns
SemanticStep with the original semantic ordinal. Extra initial Point/Edge references using a
semantic operation ID refuse as a planned creator before geometry lookup. Complete geometry creator
classification includes semantic IDs:one semantic candidate yields SemanticCreator with its actual
member/mixed position;an equal-ID native candidate yields Ambiguous with both original positions.
The ID cannot fall through to an existing geometry provider. No Point/Edge output,fixed native
family,role manifest,frame,physical length or executed logical selection is invented.

## Actual Design semantic source scope

`DraftingContext::prepare_design_semantic_sources(content_limits)` now creates a private
`DraftingDesignSemanticScope` for individual readonly Root member lookups. It freshly charges
every actual Root field,including unused logical declarations and semantic operations,then every
held provider field in one cumulative meter. Earlier Root/context demands provide no credit.
The provider's exhaustive resource walk runs;Root,currentness,member,geometry and value queries do
not. A zero-entry fence and the final meter preserve an ignored first provider-sink refusal before
any later returned cause. No result vector or accepted prefix is allocated.

The scope retains the exact actual context,Root and original snapshot record,selected `limits()`
and fresh complete `demand()`. The original immutable context's current-source association is
retained;preparing this scope performs no additional currentness validation. This covers Root
and provider content,not an actual garment's separately represented inventories. It grants no
aggregate query/comparison work,Profile/rule binding,semantic dependency order,evidence truth,
selected value,physical result or complete program acceptance. A complete consumer must bound
all its requests/comparisons before using these individual lookup primitives.

`logical_source(LogicalInputTarget)` resolves only the actual Root logical inventory and exact
expected `LogicalInputKind`. Its private `DraftingDesignLogicalSource` loans the actual scope and
existing inventory/member/kind `binding()`. `declaration()` returns the same canonical record,
including original domain,state,source and provenance. Unknown stays Unknown;Derived retains
its operation reference without supplying a selection. A dedicated numeric declaration with a
matching ID is not logical membership. No provider declaration lookup or copied domain is used.

```rust
use sc_core::{construction::*,design::{DesignResourceLimits,DesignResourceSource},
    value::LogicalInputTarget};
fn logical_domain_size<'a,H:DesignResourceSource,S:DraftingContextSource<H>+?Sized>(
    context:DraftingContext<'a,H,S>,content:DesignResourceLimits,target:LogicalInputTarget)
    ->Result<usize,String> {
    let sources=context.prepare_design_semantic_sources(content).map_err(|e|e.to_string())?;
    let source=sources.logical_source(target).map_err(|e|e.to_string())?;
    assert!(std::ptr::eq(source.scope(),&sources));
    assert!(std::ptr::eq(source.binding().inputs(),context.scope().root().inputs().logicals()));
    Ok(source.declaration().definition().domain.len())
}
```

`operation_source(id,SemanticOperationKind)` similarly loans the actual Root semantic recipe
member and its original zero-based semantic index. `DraftingDesignOperationSource::operation()`
returns that exact immutable body. Its index is not a scalar ordinal,native family or mixed
position. Neither this lookup nor a logical lookup binds rule/policy existence/signatures,
validates their mutual Derived/output relationship or checks whole semantic dependency order.
Both member views copy only loans and fixed joins;host/provider payloads need neither Clone nor
Debug. Views cannot outlive their actual source-scope loan. Debug omits domains,roles and opaque
source payloads.

```rust
use sc_core::{construction::*,design::DesignMetadataHost,ontology::EntityId,
    recipe::SemanticOperationKind};
fn original_operation_index<H:DesignMetadataHost,S:DraftingContextSource<H>+?Sized>(
    sources:&DraftingDesignSemanticScope<'_,H,S>,id:EntityId,kind:SemanticOperationKind)
    ->Result<usize,String> {
    let source=sources.operation_source(id,kind).map_err(|e|e.to_string())?;
    assert!(std::ptr::eq(source.binding().recipe(),sources.context().scope().root().semantic_recipe()));
    Ok(source.binding().index())
}
```

Admission refusal retains actual `context()`,selected `limits()` and the original first resource
`cause()`;its standard error chain retains that concrete resource cause. Lookup refusal instead
retains exact `scope()`,original typed `request()` and lending `cause()`:Logical or Operation.
Missing preserves requested ID/kind;Kind preserves expected kind and the actual original member
loan. Those lending causes remain available through typed access rather than a `'static` error
chain. No same-ID foreign owner or alternate family substitutes for the actual Root member.

For the Root8/32/3 fixture above,a provider with three cache entries,X UTF-8 label bytes and a
revision7 needs a combined11/(32+X)/6. Entry limit7 refuses Root Logical80 inputs1 after7 entries,
before the provider walk. Entry limit10 reaches provider cache3 after8 entries. A bit limit5
reaches its snapshot revision3 after Root revision3. An ignored cache failure still refuses with
the original cache location;the later zero fence cannot replace it. Freshly charging an already
prepared scope into a meter containing1 entry,2 text bytes and1 bit yields12/(34+X)/7.
`charge_resources(sink)` exhaustively walks actual Root/provider fields again;cached scope demand
and constructor policies never replace held-content accounting or clear an earlier sink failure.

Pinned Profile source records and original rule/policy signatures remain the following source-port
leaves. G4 retains Profile schema/registry/evidence/physical resolution;G3 retains physical recipe
execution. The Design source scope creates none of those proofs or results.

## Raw original Profile and parameter records

`DraftingProfileSource` now retains explicit original loans for the containing Design `root`,the
actual `selected` profile/schema/content pin,the Profile's canonical `inputs`,its original
`profile` payload record and the containing `snapshot` record. The Profile inventory is a separate
source owner;it is not implicitly the Design's canonical inventory. `SelectedProfilePin` keeps all
three authored labels distinct and unchanged,including nonblank whitespace and opaque version
spellings. Its existing validation boundary remains DeferredToG4. A raw source chooses no latest
version,default profile or physical parameter.

```rust
use sc_core::{construction::{DraftingProfileSource,DraftingSourceRecord},
    design::{DesignInputs,DesignMetadataHost},ontology::SelectedProfilePin,value::CanonicalInputs};
fn raw_profile<'a,H:DesignMetadataHost,S:?Sized,P:?Sized>(root:&'a DesignInputs<H>,
    selected:&'a SelectedProfilePin,inputs:&'a CanonicalInputs,
    profile:&'a DraftingSourceRecord<P>,snapshot:&'a DraftingSourceRecord<S>)
    ->DraftingProfileSource<'a,H,S,P> {
    DraftingProfileSource {root,selected,inputs,profile,snapshot}
}
```

`DraftingProfileLogicalSource` additionally retains the original `parameter` field loan,actual
canonical logical `declaration` and original `metadata` record exposing that association. The
parameter ID and canonical declaration ID are distinct roles:for example,parameter600 can bind
declaration90. The actual declaration retains its domain,state,source and provenance;the carrier
clones none of them. A Derived operation reference stays a reference,without a selected result or
proof that its rule exists. Metadata/profile payloads may be opaque or unsized and need neither
Clone nor Debug. Manual carrier Copy/Clone copies only loans;Debug omits pin/domain/body text.

```rust
use sc_core::{construction::{DraftingProfileSource,DraftingProfileLogicalSource,DraftingSourceRecord},
    design::DesignMetadataHost,ontology::ProfileParameterRef,value::LogicalInputDeclaration};
fn raw_parameter<'a,H:DesignMetadataHost,S:?Sized,P:?Sized,M:?Sized>(
    profile:DraftingProfileSource<'a,H,S,P>,parameter:&'a ProfileParameterRef,
    declaration:&'a LogicalInputDeclaration,metadata:&'a DraftingSourceRecord<M>)
    ->DraftingProfileLogicalSource<'a,H,S,P,M> {
    DraftingProfileLogicalSource {profile,parameter,declaration,metadata}
}
```

These are raw responses,not public accepted-source constructors. Foreign equal-data Root/pin/
inventory records,another snapshot or a detached equal-data declaration remain inspectable for
later typed refusal. Matching labels or IDs alone do not establish original Profile metadata,
canonical membership,currentness,domain acceptance,evidence truth or physical resolution. The
selected-source binding below checks explicit requested labels,Root and snapshot;the following
parameter binding below checks original profile/canonical/member association. The honest immutable source contract requires actual original
records;echoing a request or supplying a fabricated empty body cannot satisfy it.

### Complete selected-pin text accounting

`charge_selected_profile_pin_resources(pin,sink,scope,owner,index)` now charges the exact original
profile_id,schema_version and content_version strings in that order. Both the retained pin owner
and its definition are exhaustively classified;adding an owned field requires new accounting.
The caller supplies its original containing scope,optional entity owner and optional authored
collection position. The producer supplies static fields `selected_profile.profile_id`,
`selected_profile.schema_version` and `selected_profile.content_version`. It derives no entity or
index from an opaque profile label. Earlier pin construction supplies no content-budget credit.

For profile `atelier é`,schema ` 2.0 ` and content `7`,UTF-8 text demand is10+5+1=16 bytes,with no
collection entries or counter bits. Text9 refuses profile_id10 after0;text14 refuses schema5
after10;text15 refuses content1 after15. Original scope/owner/index survive the first refusal.
A prior1 entry,2 bytes and1 bit composes to1/18/1. A prior sink failure stays first.

```rust
use sc_core::{design::*,ontology::{SelectedProfilePin,SelectedProfilePinDefinition}};
use std::num::{NonZeroU64,NonZeroUsize};
# fn main()->Result<(),Box<dyn std::error::Error>> {
let pin=SelectedProfilePin::new(SelectedProfilePinDefinition {
    profile_id:"atelier é".into(),schema_version:" 2.0 ".into(),content_version:"7".into(),
})?;
let limits=DesignResourceLimits::new(NonZeroUsize::new(1).ok_or("entries")?,
    NonZeroUsize::new(16).ok_or("text")?,NonZeroU64::new(1).ok_or("bits")?);
let mut meter=DesignResourceMeter::new(limits);
charge_selected_profile_pin_resources(&pin,&mut meter,DesignResourceScope::SourceHost,None,None)?;
let demand=meter.finish()?;
assert_eq!((demand.collection_entries(),demand.text_bytes(),demand.counter_bits()),(0,16,0));
assert_eq!(pin.definition().schema_version," 2.0 ");
# Ok(()) }
```

This helper does not look up or validate a Profile,parse versions or create a meter/allowance.
The complete provider walk must also charge all actual canonical fields,Profile bodies,metadata,
duplicated records/caches and represented counters,including unused entries,before queries.
Existing canonical and logical producers compose those represented fields;opaque payloads need
their actual integration-owned exhaustive producer. Selected-source admission and typed provider
causes and parameter binding are implemented below. G4 still owns schema/registry/evidence/physical
resolution;these original source loans and text charges grant none of those proofs.

## Bind the original selected Profile source

`DraftingProfileProvider` now extends the immutable containing context with one readonly
`profile_source(requested_pin)` query. Its response loans come from the provider independently
of the request pin's lifetime. The requested profile/schema/content labels select an actual
already-held source;they are not a replacement original source pin. The port's compiled lifetime
controls permit a short-lived selection claim and refuse returning that claim as the original
longer-lived pin. The query loads/clones/allocates no body and reads no parameter value.

The provider's inherited exhaustive content walk must charge every actual Profile pin,canonical
inventory,body,metadata record,retained copy/cache and represented counter,including unused
sources,before lookup. It exposes the actual original Profile/body/canonical association under
the honest immutable integration contract. Core's explicit field checks complement that contract;
they do not establish G4 registry/schema/evidence truth or defend against invented source data.
Missing,retired or unsupported exact selections retain the provider's original typed cause.
There is no default/latest/single-candidate fallback or manufactured empty success.

`DraftingContext::bind_profile_source(requested_pin,content_limits)` freshly accounts for the
complete Root,all held provider content,then every requested pin string in one cumulative meter.
The request fields have CommandState scope,actual containing Root owner and no invented index.
The existing zero fence and final meter retain an ignored first provider-sink failure. Only after
all content passes does core issue one Profile query. Earlier context/source demands supply no
aggregate credit. The pin borrow may be shorter than the original containing context borrow.

After a response,core compares the actual returned Root pointer,then exact profile_id,
schema_version and content_version text in that order,then the original snapshot record identity.
Equal-ID/equal-revision foreign Roots refuse. Labels are not trimmed,parsed or normalized;
schema `2.0` differs from ` 2.0 `. The caller's selection pin and provider's actual original pin
may be distinct equal-data objects:the request is intent,the returned pin belongs to the source.
The actual Profile/canonical association stays with the original provider contract;the following
parameter binder below uses that same original source and inventory,not equal-looking substitutes.

Successful `BoundDraftingProfileSource` privately retains original `context()`,requested
`selected()` pin,actual returned `source()`,explicit `limits()` and fresh `demand()`. Its fields
cannot be replaced,and it cannot outlive the requested pin or original context/provider records.
Host/provider payloads need neither Clone nor Debug. Debug omits pin labels and opaque data.
The raw returned source alone cannot construct a private accepted binding.

```rust
use sc_core::{construction::*,design::{DesignResourceSource,DesignResourceLimits},
    ontology::SelectedProfilePin};
fn selected_source<'p,'a,H:DesignResourceSource,S:DraftingProfileProvider<H>+?Sized>(
    context:DraftingContext<'a,H,S>,selected:&'p SelectedProfilePin,limits:DesignResourceLimits)
    ->Result<BoundDraftingProfileSource<'p,H,S>,String> where 'a:'p {
    let bound=context.bind_profile_source(selected,limits).map_err(|e|e.to_string())?;
    assert!(std::ptr::eq(bound.selected(),selected));
    assert_eq!(bound.source().selected.definition(),selected.definition());
    Ok(bound)
}
```

Failure preserves the exact original `context()`,requested `selected()` pin and selected
`limits()`. `refusal()` distinguishes Resources,original Source error and Mismatch. Mismatch
identifies Root,the first SelectedPin field,or Snapshot. `returned()` retains the whole actual
raw response for mismatches,including foreign records;it is None before a successful response.
The standard error chain preserves concrete resource/provider causes. No partial accepted
source or substituted diagnostic/value escapes failure.

The focused fixture has an empty Design Root,a provider cache3,a separate Profile canonical
inventory4,an additional unused equal-ID Root,actual pin/body and alternate snapshot records.
Three separately represented revision7 counters require9 bits;all actual text,including both
the provider-owned pin and distinct requested pin,requires T bytes. Exact7/T/9 passes. Entry2
refuses provider cache3 after0,before lookup,even if its error is ignored. Text T-1 reaches the
request's final content_version byte after T-1 bytes,also before lookup. A returned foreign Root
wins over wrong pin labels/snapshot;profile ID precedes schema,then content,then snapshot.

`BoundDraftingProfileSource::charge_resources(sink)` walks those actual Root/provider/request
fields again. The returned header is a borrowed view of the original provider-owned data;cached
demand never replaces content and no new Profile lookup occurs. A meter with prior1 entry,
2 bytes and1 bit composes the fixture to8/(T+2)/10.

```rust
use sc_core::{construction::*,design::*};
fn selected_content<H:DesignResourceSource,S:DraftingProfileProvider<H>+?Sized>(
    bound:&BoundDraftingProfileSource<'_,H,S>,limits:DesignResourceLimits)
    ->Result<DesignResourceDemand,DesignResourceLimitError> {
    let mut meter=DesignResourceMeter::new(limits);
    bound.charge_resources(&mut meter)?;
    meter.finish()
}
```

This is one selected-source association under fresh logical content bounds. It does not grant
aggregate lookup/comparison work,parameter/canonical member or kind binding,additional source
currentness validation,garment content,semantic dependency graph,value resolution,physical
execution or whole-program acceptance. Those stages retain their original tracked owners.

## Bind actual Profile logical parameter metadata

`DraftingProfileLogicalProvider` adds readonly `profile_logical_source(original_profile,parameter)`
to the selected-Profile facet. It returns the actual original metadata record,parameter field,
canonical declaration and Profile header. The parameter request is a value,not a substitute
metadata-field loan;the compiled E0515 control refuses returning its stack address. The source
contract requires the parameter→declaration association exposed by that original metadata.
Core checks its returned header,parameter ID and actual canonical member/kind under that contract.
The query allocates/clones no data,loads no body and reads no parameter value.
`DraftingProfileLogicalResponse<'a,H,S>` names that same raw response using the provider's actual
associated snapshot,Profile and metadata types;the alias grants no additional source proof.

All held metadata vectors,text,source copies/caches and counters,including unused records,must be
part of inherited exhaustive provider accounting. `BoundDraftingProfileSource::bind_logical_parameter`
freshly walks the whole Root/provider/request content under its explicit selected limits before
issuing one metadata query. Earlier parent demand gives no credit. It does not repeat the selected
Profile lookup. Requested parameter ID and expected `LogicalInputKind` are fixed metadata,not
formula names,caller resource totals or selected values.

After a response,core requires the same original source header in this order:Root,pin owner,
canonical inventory owner,Profile payload record,snapshot record. Unlike the initial selection
claim,these returned header fields must identify the exact source already bound by the parent.
Another equal-label pin or equal-data canonical inventory refuses. Core then compares the actual
returned metadata parameter ID with the request,joins the returned declaration ID and expected
kind against the parent's actual Profile logical inventory,and requires that exact canonical
declaration pointer. A detached equal-data record refuses after the real kind/member join.
No numeric declaration or different logical family substitutes for the requested source.

`BoundDraftingProfileLogicalSource` privately retains the exact original `parent()`,requested
`parameter()` and `kind()`,returned `source()`,actual canonical `binding()`,explicit `limits()`
and fresh `demand()`. `declaration()` loans the real canonical record,including its untouched
domain,state,source and provenance. Parameter600 can map to canonical declaration90 at original
logical inventory index0;none of those identities is a mixed/scalar/native position. Derived80
remains a reference,without a selected result or a rule/graph proof. The proof cannot outlive its
parent's loan,and its fields cannot be replaced. Debug omits opaque metadata/domain/pin text.

```rust
use sc_core::{construction::*,design::{DesignResourceSource,DesignResourceLimits},
    ontology::ProfileParameterRef,value::LogicalInputKind};
fn original_parameter<'b,'a,H:DesignResourceSource,S:DraftingProfileLogicalProvider<H>+?Sized>(
    parent:&'b BoundDraftingProfileSource<'a,H,S>,parameter:ProfileParameterRef,
    kind:LogicalInputKind,limits:DesignResourceLimits)
    ->Result<BoundDraftingProfileLogicalSource<'b,'a,H,S>,String> {
    let source=parent.bind_logical_parameter(parameter,kind,limits).map_err(|e|e.to_string())?;
    assert!(std::ptr::eq(source.parent(),parent));
    assert!(std::ptr::eq(source.binding().inputs(),parent.source().inputs.logicals()));
    Ok(source)
}
```

Refusal preserves original parent,parameter,kind and limits. `returned()` retains the entire
actual raw response,including wrong/detached metadata and declarations,once a response exists.
`refusal()` distinguishes original Resources,typed Source error,Mismatch and lending Canonical
cause. Mismatch identifies the first Root,SelectedPin,Inventory,Profile,Snapshot,Parameter or
Declaration issue. Canonical retains the original `LogicalInputLookupError`:Missing preserves
returned declaration ID/expected role;Kind loans the actual original canonical member. Concrete
resource/provider causes remain in the standard error chain;borrowed member causes stay available
through typed access. No accepted parameter prefix or fabricated replacement source escapes.

The focused fixture extends the selected-source fixture with two original metadata records,
an unused equal-data canonical inventory,an independently held missing declaration and alternate
pin/body/snapshot records. All count before query. Exact14/T/9 passes;one prior entry,2 bytes and
1 bit compose to15/(T+2)/10. Entry8 refuses metadata vector2 after7 entries,before a missing
parameter lookup,even if the provider ignores the sink error. A missing parameter404 instead
preserves its typed provider cause when complete content is admitted. All seven association
refusals retain original responses;wrong logical kind and missing canonical999 retain their real
inventory causes. Successful lookup uses one fresh resource walk,zero repeat Profile queries
and one metadata query.

`charge_resources(sink)` freshly walks actual complete parent/provider content again. The
metadata/member/header are original borrowed views,not newly owned copies of bodies or domains.
Cached demand and earlier limits never replace accounting;no metadata/value/source query occurs.
This remains individual source/member/kind binding. Aggregate query/comparison work,currentness
revalidation,semantic dependency graph,Profile schema/evidence truth,physical/value resolution
and whole-program execution retain their separate owners. Original rule/policy source signatures
are the next tracked source-port stage.

## Rule and policy source contract

This is the implementation contract for the rule-source leaves. Immutable signature declarations
and their resource producer are implemented below;original body/origin source records,providers
and accepted source bindings are **planned**. Existing operation records retain
rule/policy IDs and local inputs;they do not establish that an actual rule exists or has the
required signature. The implemented Root/Profile member facets supply original data loans,
not a fabricated rule body or evaluator.

An actual rule source must retain three separate things:its immutable declaration identity and
typed signature,its original body record,and its containing origin/Root/snapshot. The body may be
opaque to this G1 integration. It must be the actual already-held source,with an exhaustive
integration-owned resource producer;an empty placeholder or a copied operation definition cannot
stand in for it. Static metadata does not prove that an opaque implementation honors its signature.
That association relies on the honest immutable source contract;execution/conformance is separate.

The signature distinguishes logical derivation from Buttonhole policy. A logical rule declares
its logical result kind and complete named input requirements. Each input requirement preserves
LogicalInputKind or the actual numeric canonical representation:dedicated Length versus Scalar
with its precise ScalarInputKind. Logical source origin stays separately explicit as Design or
Profile;it is not coerced into a numeric operand. A Buttonhole policy has its mandatory canonical
button-size source and complete named numeric policy-input requirements. Its result is the later
physical buttonhole length,not a new authored/cached result field on ButtonholeOperation.

Local roles remain original MachineTokens,not global formula names,spatial output names or a
new executable language. Distinct roles may use the same actual source. Repeated signature roles
refuse even when their kinds match. Construction must derive complete input count,text and
conservative comparison bounds before quotas,then reject the first duplicate in original order.
Every allowance is explicit;an empty signature is explicit and cannot remove the Buttonhole's
required button source. Constructor policy/shape metadata gives no later whole-source credit.

Source origins are explicit. A Design source retains the actual containing Design owner. A
Profile source retains the actual selected Profile/pin/source/canonical/snapshot association,
including separate profile/schema/content labels. Matching a rule ID or a parameter ID does not
select a factory or a latest revision. Missing/unprovided/retired/unsupported/ambiguous sources
retain original typed causes rather than a default policy or reconstructed history.

The provider must expose original declaration/signature/body/origin records after complete content
admission. Every owned signature role/vector,body field,unused declaration,retained copy/cache and
represented counter belongs in the actual source walk. Opaque bodies do not waive accounting.
Collections are precharged before traversal,and original text is charged without normalization,
cloning or supplied demand totals. Borrowed views of already-accounted records do not create
copies of their domains or bodies. Exhaustive owner/definition/variant views and fixed Copy fields
must make new retained fields an explicit accounting obligation.

The later binder admits all held content and complete prospective operation/role comparisons
before provider queries,result reservation or role matching. It retains the original operation,
requested rule/policy ID and every supplied source occurrence. Matching then checks actual
Root/snapshot/origin,rule identity/family,result kind and exact local-role/input requirements.
Roles match their original names,not a guessed positional meaning;both original orders remain
inspectable. Missing,extra,repeated or wrong-kind inputs refuse without exposing an accepted
prefix. Equal-data foreign records cannot replace the selected original source or canonical member.

Logical input/output declarations retain their actual canonical domains,state,source and
provenance. Matching a logical signature does not compute a result,select a domain member or
upgrade Unknown/Derived to Known. The original Derived operation/output relationship is checked
against actual operation and canonical owners;body evaluation and result-domain/value validation
remain with execution. G3 owns physical Buttonhole policy execution and positive length;G4 owns
Profile schema/registry/evidence/policy truth. No hardware dimensions,clearance,formula,logical
default,geometry output,allocator/time budget or whole-program acceptance is inferred here.

Implementation proceeds through immutable bounded signature records/accounting,original body and
origin facets,then fresh bounded binding. Whole source/order/graph/allocator/lifetime integration
verification remains the separately tracked `.5f.2.verify` leaf after the agreed integration set.

## Immutable rule signature declarations

`SemanticRuleDeclaration::new` moves a complete `SemanticRuleDefinition` into an immutable owner.
The definition retains its original rule ID and `SemanticRuleSignature`. `Logical` declares its
`LogicalInputKind` result and a vector of `SemanticRuleInput` requirements. Each requirement is
either an exact logical kind or a `CanonicalNumericInputKind`:dedicated `Length`,or `Scalar` with
the precise `ScalarInputKind`. Thus dedicated Length and Scalar(Length) remain distinct even when
their physical dimensions agree. Input origin,canonical identity,domain and state stay elsewhere.

`Buttonhole` contains only `ButtonholePolicyRequirement` numeric requirements. The policy family
always requires the operation's canonical button-size source and has a later physical Length
result. An empty requirement vector removes neither obligation. It adds no hole-length field,
physical formula,button dimensions,clearance or evaluator. A logical requirement cannot occupy a
numeric policy slot:

```compile_fail,E0308
use sc_core::{name::MachineToken,recipe::*,value::LogicalInputKind};
let input=ButtonholePolicyRequirement {
    role:MachineToken::new("clearance").unwrap(),
    kind:SemanticRuleInputKind::Logical(LogicalInputKind::NotionSize),
};
```

Every constructor allowance is explicit through `SemanticRuleLimits::new(P,T,I)`. Construction
derives the complete input count P,all separately owned role bytes T and conservative P×P
comparison bound before checking the input,text and inspection quotas in that order. It then
rejects the first repeated role in original declaration order,retaining both zero-based indices.
Equal kinds do not excuse repeated roles;distinct roles may require the same kind. Zero allowances
admit an explicitly empty signature. Validation queries nothing,allocates no workspace and retains
the original vector and role buffers. Input allocation already happened in the caller;these
logical bounds do not bound buffer capacity,string-comparison byte work,CPU or body execution.

This declaration requires an original logical input,a dedicated length and a general Length
scalar. Three eight-byte roles require exactly 3 inputs,24 text bytes and 9 inspections:

```rust
use core::num::{NonZeroU64,NonZeroUsize};
use sc_core::{design::*,name::MachineToken,ontology::EntityId,recipe::*,value::*};
# fn main()->Result<(),Box<dyn std::error::Error>> {
let rule=SemanticRuleDeclaration::new(SemanticRuleDefinition {
    id:EntityId::from_bits(900),
    signature:SemanticRuleSignature::Logical {
        result:LogicalInputKind::PocketOpening,
        inputs:vec![
            SemanticRuleInput {role:MachineToken::new("source_a")?,
                kind:SemanticRuleInputKind::Logical(LogicalInputKind::NotionSize)},
            SemanticRuleInput {role:MachineToken::new("source_b")?,
                kind:SemanticRuleInputKind::Numeric(CanonicalNumericInputKind::Length)},
            SemanticRuleInput {role:MachineToken::new("source_c")?,
                kind:SemanticRuleInputKind::Numeric(CanonicalNumericInputKind::Scalar(ScalarInputKind::Length))},
        ],
    },
},SemanticRuleLimits::new(3,24,9))?;
assert_eq!(rule.kind(),SemanticOperationKind::LogicalDerivation);
assert_eq!(rule.shape().role_text_bytes(),24);
let mut meter=DesignResourceMeter::new(DesignResourceLimits::new(
    NonZeroUsize::new(3).ok_or("entries")?,NonZeroUsize::new(24).ok_or("text")?,NonZeroU64::MAX));
charge_semantic_rule_resources(&rule,&mut meter,DesignResourceScope::SourceHost)?;
let demand=meter.finish()?;
assert_eq!((demand.collection_entries(),demand.text_bytes(),demand.counter_bits()),(3,24,0));
let policy=SemanticRuleDeclaration::new(SemanticRuleDefinition {
    id:EntityId::from_bits(901),signature:SemanticRuleSignature::Buttonhole {inputs:vec![]},
},SemanticRuleLimits::new(0,0,0))?;
assert_eq!(policy.kind(),SemanticOperationKind::Buttonhole);
# Ok(()) }
```

`definition()`,`id()`,`kind()`,`limits()` and `shape()` expose the exact immutable metadata.
Equality compares original definition content,independent of selected constructor allowances.
Diagnostics retain identity,family,shape and refusal without local role text.

`charge_semantic_rule_resources` freshly walks actual fields rather than trusting shape totals.
The caller supplies the actual containing scope;the owner is the rule ID. It precharges the whole
`semantic.rule.inputs` vector with no fabricated containing collection index,then charges each
original `semantic.rule.inputs.role` at its zero-based declaration index. All fixed kinds/results,
constructor policy and shape are guarded as Copy fields;exhaustive owner/definition/variant views
expose future accounting obligations. It uses the caller's cumulative sink and preserves its first
latched refusal,including for an empty signature. The caller must finish that sink and separately
account for the original body,all held declarations,unused records,copies,caches and counters.

This API validates a declaration's complete local shape and role uniqueness. Original body,
Design/Profile origin,Root/snapshot membership,operation signature matching,canonical source
binding,value/domain truth and execution remain separate proof obligations. The source facets
and binder above remain planned;complete coupled verification is deferred to `.5f.2.verify`.
