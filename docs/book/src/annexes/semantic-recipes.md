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

This is a standalone source record and exhaustive content walk. Genuine ordered semantic recipe
ownership, component origin binding, Root/program attachment,
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
graph acceptance or execution; those remain tracked alongside component origin binding and source ports.

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
