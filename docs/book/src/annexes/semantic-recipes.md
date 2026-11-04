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
retains original declaration order and is attached to the actual Root below. Mixed-program metadata,
component origin binding and readonly source ports are implemented below;complete Closure-to-operation
consumer binding follows the planned whole semantic contract at the end of this chapter. It creates
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
component origin binding and source ports are implemented below. Whole consumer binding remains
planned under the contract at the end of this chapter.

## Raw original component origins

Component origin carriers preserve actual original loans before query/binding validation.
`DraftingComponentCreator` has Mirror and Join variants holding their real
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
source provider must freshly account every actual additional recipe input, manifest,
primitive-source payload, duplicated record and cache before bounded binding queries; earlier
native constructor proofs provide no whole-source credit. Complete Root Piece/copy membership,
actual recipe/native-input association,frame/snapshot identity and original first causes are
checked by the implemented work/association stages below. Other creation forms
need genuine supported source records; these carriers supply no synthetic native history for them.

## Component source binding contract

The readonly query port,fresh current copy/Piece work stage and complete original source/creator
association binder are implemented below. Raw carriers and program-record accounting are also
implemented. These APIs add no geometry algorithm,importer or history
reconstruction. All original source records must already be held before lookup.

A component target is an explicit `PocketPieceRef {copy,piece}`:a stable physical cut-copy ID and
its expected pattern Piece ID. A list position,quantity or equal-looking Piece cannot supply either
identity. Binding must retain the actual current Root's CutPlan copy and Piece inventory member,
including original copy orientation and Piece metadata. The copy must name that expected Piece,
and the returned origin's Piece must be that exact original member. These checks create no
reflected geometry or inferred Cartesian transform.

Imported origin requires its actual primitive Piece and original import-source record. The
payload may be opaque,but its complete fields still need an integration-owned resource producer.
Missing/unprovided/retired/unsupported source data returns the original typed cause. An unknown
recipe does not become an imported success,and an imported primitive does not acquire a fabricated
recipe,creator,format or creation position.

Recipe origin retains the actual complete original `DraftingProgramInputs` owner and genuine
Mirror/Join Piece-identity declaration. Those declarations are locally checked metadata facts;
they issue no fresh/current identity and prove no executed creation or history. The binder must
check the declared result Piece ID against the actual selected Piece and match the exact creator
input loan to its original native program member. Scalar and semantic slots cannot supply native
creation. No original occurrence is skipped:missing or repeated creator membership refuses rather
than choosing a position. A successful position comes from the actual mixed sequence,never from
a numeric operation ID,source label or reconstructed ordinal.

The source provider must return the recorded original recipe association under its actual
immutable snapshot contract. Original program context,Root,frame and snapshot loans remain
original;do not replace them with the current containing context or infer ancestry from equal
IDs. Native source Piece/metadata associations must be checked against their actual original
program owner. The component header's frame/snapshot must match the containing context's actual
records,and recipe frame compatibility uses original record identity with no implicit transform.
Captured original program snapshot provenance remains separately inspectable. Matching these
metadata fields does not prove execution,geometric correctness or historical source truth.

Complete fresh current garment/provider/request content and prospective copy/member/creator work
must be admitted before query,source scans or result reservation. The provider's exhaustive walk
includes every actual additional original program,its complete native inputs/manifests,import
payloads,retained contexts,unused records,copies/caches and represented counters. Constructor
policies and previous demand totals give no credit. Precharge all collections before traversal;
all original occurrences and first field causes remain inspectable on refusal.

Program-record accounting needs a separate composition boundary. Existing
`DraftingProgramInputs::charge_resources` walks the Root,then calls its provider,then walks steps.
A provider accounting its own held recipe must not recursively invoke itself through that whole
walk. The record producer below provides an exhaustive step/native-field walk without Root/provider,
currentness or value callbacks. Integrations then account original context bodies through their
actual owners,while the existing whole-program method composes the same record walk and keeps
its complete first-refusal contract. This helper grants no whole-source admission itself.

The readonly source port must return original source-owned loans independently of a by-value
copy/Piece request. Query loads,clones and evaluates nothing. All raw responses remain inspectable
before core checks them;only a private complete source binding may report successful association.
Original missing/unsupported/provider errors and actual ownership/frame/snapshot/creator witnesses
survive. No successful Piece/copy or creator prefix is published on failure.

Current supported recipe creator carriers are Mirror and Join. Other creation forms need genuine
supported source records and explicit future implementation;they receive no synthetic native
history. Whole Pocket/semantic dependency order,canonical logical input/Profile parameter binding,
G3 physical effects,G4 source/evidence truth and execution remain separate. Coupled verification
stays `.5f.2.verify` after the agreed integration set.

## Original program record accounting for source providers

`DraftingProgramInputs::charge_record_resources(sink)` appends only the actual original mixed step
vector and complete native input fields to the caller's cumulative meter. It requires only
`DesignMetadataHost` and `DraftingContextSource`;it makes no Root/provider resource callback,
currentness/member/value query or normalization call. A provider holding its original recipe can
compose this walk without reentering itself through whole-program accounting.

```rust
use sc_core::{construction::{DraftingContextSource,DraftingProgramInputs},
    design::{DesignMetadataHost,DesignResourceSink,DesignResourceLimitError}};
fn records<H:DesignMetadataHost,S:DraftingContextSource<H>+?Sized>(
    program:&DraftingProgramInputs<'_,H,S>,sink:&mut dyn DesignResourceSink)
    ->Result<(),DesignResourceLimitError> {
    program.charge_record_resources(sink)
}
```

The method exhaustively matches the private original program owner and every step variant.
Context and scalar/semantic ordinals have fixed Copy guards. It precharges the complete vector
at `CommandState`,actual original Root ID,`drafting.program.steps`,no invented index. Scalar and
semantic ordinals remain data;even an invalid ordinal is charged without coverage/order lookup.
Every native occurrence uses the existing exhaustive native-field producer,including repeated
references to the same input. Original/canonical argument buffers and actual nested collections
keep their original operation owners,field names and indices;there is no deduplication,cloning,
normalization or supplied demand credit.

The integration separately accounts its actual original Root,frame/snapshot,source payloads,
retained copies/caches and represented counters through their real owners. This record walk
does not replace complete source/program admission or supply allocator capacity,CPU or execution
credit. A captured program context remains its original borrowed context;the record method uses
its actual Root ID for locations without requerying that provider or reconstructing provenance.

The existing complete `charge_resources` method still walks Root,then provider,then a zero-entry
fence retaining ignored provider errors,and finally this same record producer. Complete program
accounting therefore keeps its earlier field order and first-cause behavior. Normalized arenas
and later proof buffers remain separate resource scopes.

The focused fixture has four raw slots:invalid scalar ordinal0,a Point input,invalid semantic
ordinal999,and the same original Point input again. If T is that Point's actual original plus
canonical argument bytes,record demand is4/(2T)/0. Prior2 entries/3 text bytes/1 bit compose to
6/(2T+3)/1. A one-bit maximum succeeds despite Root/snapshot revisions needing three bits each,
and the provider call trace stays0. This checks that record accounting does not walk those bodies.

Maximum1 entry refuses a two-slot vector before native traversal. Text maximum12 refuses the
first native `drafting.point.x.source` field's13-byte `private_width` at operation100,before its
canonical or later arguments. Empty records contribute0/0/0;empty and nonempty walks preserve a
previously latched cause. The caller must finish its meter. These checks accept no scalar order,
native kind/value,geometry,source association or executable program. The component source query
and complete association binder are implemented below. Coupled verification stays deferred to `.5f.2.verify`.

## Readonly original component query port

`DraftingComponentProvider` extends the existing actual garment/context ports. Its `Imported`
associated type is the actual opaque import payload,with no Clone/Debug/size requirement.
`component_source` takes an explicit by-value `PocketPieceRef {copy,piece}` and returns original
Piece/import or recipe/creator/frame/snapshot loans. The request cannot substitute a borrowed
source record,and numeric IDs cannot manufacture original data or creation positions.

```rust
use sc_core::{construction::*,design::DesignReferenceHost,ontology::PocketPieceRef};
fn original<'a,H:DesignReferenceHost,S:DraftingComponentProvider<H>+?Sized>(provider:&'a S,
    requested:PocketPieceRef)->Result<DraftingComponentResponse<'a,H,S>,S::Error> {
    provider.component_source(requested)
}
```

`DraftingComponentResponse` is a transparent alias using that provider's actual context,Frame,
Snapshot and Imported types. Both raw source lifetimes are tied to the provider borrow. An
original program context may live longer and be shortened into this view without cloning,
rebinding or replacing any original field. API examples check that conversion and refuse a
temporary request-backed import record. Existing source/creator/data-owner escape refusals remain.

Query returns already-held original records and loads,allocates,clones and evaluates nothing.
Missing,reassigned,retired,ambiguous or unsupported sources preserve original typed provider causes.
An unprovided source cannot return an empty import or reconstructed recipe. Original program,
native creator and context fields stay in recipe origin;import origin retains its actual record.
No source/currentness/creation-history or executable acceptance follows from constructing a raw
response. Direct port calls perform no automatic resource preflight or complete binding checks.

Inherited complete fresh accounting covers every actual original import/program/native field,
creator manifest,retained context,unused record,copy/cache and counter before lookup. Providers
holding recipes compose `charge_record_resources` without recursively reentering themselves and
account real context bodies through their original owners. The honest immutable integration
contract forbids fabricated source metadata and omitted resources;it is no security boundary
against a provider violating that contract.

The maintained fixture returns actual Root Piece100 and import record for copy10002,including
original frame/snapshot records. The opaque payload needs no Clone/Debug,and copying the view
preserves its exact Piece/source pointers. It separately holds unsupported copy10001's original
record and an unused cache;all are charged. Original import names occupy18/22 UTF-8 bytes;their
byte vectors have3/2 entries and their actual revision counters need3/129 bits.

Fresh garment/provider admission totals138 counter bits:Root3,snapshot3 and both imports3+129.
Maximum137 refuses the unused second import at9+129 before a component query. Provider-only
entry maximum4 refuses the complete two-record vector at3+2 before traversal. An ignored return
error still leaves the sink's original cause latched. Raw missing copy404,copy10002 with expected
Piece999,and unsupported copy10001 retain their exact original requests and typed causes.

These are source-port/accounting checks. The fresh work stage below binds current actual
CutPlan/Piece membership. The source binder below binds exact original source/frame/snapshot
owners and validates original recipe/native creator membership without a successful prefix. Original
source truth,execution/history,physical geometry and whole semantic dependencies remain separate;
coupled verification stays `.5f.2.verify` after the agreed integration set.

## Fresh current component work

An `AdmittedDraftingGarmentSource` now exposes `charge_resources(sink)` for fresh composition.
It walks the entire actual garment and provider again,without using its earlier stored demand
as credit or calling the garment getter,currentness,member,value or component queries. A zero-entry
fence preserves an ignored provider failure. The caller owns and finishes the cumulative meter;
an earlier latched cause remains the first cause. Existing garment admission uses the same walk
and retains its Root-before-content-before-snapshot order.

`prepare_component_binding(target,limits)` first performs that complete fresh walk,then derives
the checked prospective matching bound `(E+8)²`,where E is the actual complete held collection
population. `DraftingComponentBindingLimits::new(content,matching_inspections)` has no default.
Content failure precedes arithmetic/work refusal,which precedes current copy/Piece lookup.
The conservative bound covers original member/creator and fixed-header inspections. It supplies
no string-byte,provider-internal CPU,allocator,geometry or execution allowance.

```rust
use sc_core::{construction::*,design::{DesignReferenceHost,DesignResourceSource},
    ontology::PocketPieceRef};
fn work<'g,'a,H:DesignReferenceHost+DesignResourceSource,S:DraftingComponentProvider<H>>(
    parent:&'g AdmittedDraftingGarmentSource<'a,H,S>,target:PocketPieceRef,
    limits:DraftingComponentBindingLimits)
    ->Result<AdmittedDraftingComponentWork<'g,'a,H,S>,DraftingComponentWorkError<'g,'a,H,S>> {
    parent.prepare_component_binding(target,limits)
}
```

After work admission,the actual current CutPlan must contain `target.copy`,that original copy
must name `target.piece`,and the actual current Piece inventory must contain that Piece. The
private work result retains the exact original parent,target,copy and Piece loans,selected limits,
fresh demand and derived shape. Copy orientation remains unchanged metadata. No transform or
identity is produced. Missing-copy,reassigned-copy and missing-Piece refusals retain the original
request and policy;reassigned-copy diagnostics also lend the exact actual current copy.

Work acceptance makes no component query and accepts no import,recipe,creator,frame/history or
executable source association. A current copy whose original source is unsupported can pass this
stage;its later source binding must still refuse. Recharging a work scope walks all its original
parent fields again,without credit for the stored demand or duplicate charging of borrowed views.

Focused checks cover both copies10001/10002 of Piece100 and exact member pointers/orientations,
one fresh provider walk per preparation,and zero component queries. Prior2 entries/3 text bytes/
1 counter bit compose with the full current demand,whose represented counters total138 bits.
Maximum137 refuses the unused import at9+129 before work or missing-copy checks,including when
the provider ignores its return error. One inspection below `(E+8)²` refuses before missing404;
the exact bound permits that missing-copy diagnostic and original copy10002→expected999 refusal.
Checked arithmetic refuses addition/square overflow. API examples refuse private-field mutation,
work escaping its parent and default policy construction. Coupled verification remains deferred.

## Bind the complete original component association

`AdmittedDraftingComponentWork::bind_source()` repeats complete fresh garment/provider content,
checked prospective matching work and current copy/Piece membership before making one exact
`component_source` query. Earlier demand grants no credit. A fresh content/work refusal makes no
query and retains the original nested work cause. An original provider refusal retains its typed
cause,request and fresh admission metadata. No successful source prefix is returned.

```rust
use sc_core::{construction::*,design::{DesignReferenceHost,DesignResourceSource}};
fn bind<'w,'g,'a,H:DesignReferenceHost+DesignResourceSource,S:DraftingComponentProvider<H>>(
    work:&'w AdmittedDraftingComponentWork<'g,'a,H,S>)
    ->Result<BoundDraftingComponentSource<'w,'g,'a,H,S>,DraftingComponentSourceError<'w,'g,'a,H,S>> {
    work.bind_source()
}
```

Every returned origin must lend the exact actual current Piece accepted by the work scope.
The common frame and containing snapshot must be the context's original records. Equal Piece
IDs,matching payload bytes or equal frame coordinates do not substitute for those owners.
The first mismatch order is Piece,frame,then snapshot. Refusals retain the entire raw response,
including mismatched fields,plus the original work parent and complete fresh demand/shape.

An imported origin then retains its actual opaque import record and has no creator position.
The source's immutable integration contract owns the truth of that recorded import association;
the binder creates no importer,format inference or historical certificate.

A recipe origin must preserve its original program/context and snapshot. Its original frame
must be the actual compatible component frame;no implicit transform is inferred. The genuine
creator declaration must name the actual current result Piece. The source Piece of a Mirror
must be the exact original member of that captured program Root. A Join additionally requires
the exact original Root Piece inventory,source Piece slice and CutPlan,then exact A/B source
Piece members. Equal-data replacement owners refuse. These checks retain the original metadata
parents and grant no globally fresh identity or geometric result.

The binder scans the complete captured mixed sequence for the creator's exact original native
input loan. An equal-ID Copy of that input is another record and refuses. Scalar and semantic
slots supply no native creator. No occurrence is skipped:missing membership refuses,and a second
matching occurrence reports the first two genuine one-based mixed positions. A complete binding
reports the unique actual position through `creator_position()`. It never derives a position
from a scalar/semantic ordinal or operation ID. The captured program snapshot remains separately
inspectable and may differ from the current component's containing snapshot.

The successful private `BoundDraftingComponentSource` retains its exact work parent,complete raw
source,optional creator position and fresh demand/shape. Its `charge_resources` method rewalks the
whole original garment/provider content. Returned loans and fixed metadata contribute no duplicate
body ownership or stored-demand credit. The binder performs no additional currentness,value or
geometry query,source/body clone,transform,identity issuance or result-vector reservation.

Focused fixtures check imported original record loans and ordered Piece/frame/snapshot refusals.
Actual Mirror and Join creators occur at mixed position2 between raw scalar0 and semantic999
requests;those requests remain data and this association proves no whole-program coverage/order.
Equal-ID native copies refuse,as do repeated creator positions2/4,foreign original source owners,
incompatible recipe frames,wrong declared result Piece and equal-data foreign CutPlans/inventories.
Successful captured program snapshots remain distinct from the current containing snapshot.

The provider accounts complete original garments,context bodies,program records,creator inputs,
unused import/cache fields and represented counters before lookup. Fixture creator catalogues are
actually empty and selected metadata policies borrow their original source fields. A deliberate
negative control grows a held cache after work admission:the old content maximum refuses first;
with one spare entry,the larger fresh matching bound refuses instead. Both make zero component
queries,including an ignored provider return. Original typed missing-source causes also survive.
These fixtures model the immutable contract's violation only to falsify cached admission credit.

Complete source association is implemented with focused checks. Whole semantic consumer binding,
dependency order,source truth,execution/history and physical geometry remain separate. Coupled
verification stays `.5f.2.verify` after the agreed integration set.

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

Pinned Profile sources and original rule/policy signatures have their own implemented sections
below. G4 retains Profile schema/registry/evidence/physical resolution;G3 retains physical recipe
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
and their resource producer,raw original body/origin source records and providers are implemented
below;original rule source association and complete local signature/output binding are also
implemented. Actual canonical input/Profile parameter and whole dependency binding remain separate.
Existing operation records retain
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
binding,value/domain truth and execution remain separate proof obligations. Raw source facets
and local signature binders follow below;coupled verification remains deferred to `.5f.2.verify`.

## Raw original rule body and origin facets

`DraftingRuleSource` loans an actual immutable `SemanticRuleDeclaration`,an original opaque body
`DraftingSourceRecord` and a `DraftingRuleOrigin`. A Design origin holds the actual containing
Root and original snapshot record. A Profile origin holds the complete original
`DraftingProfileSource`:Root,selected profile/schema/content pin,canonical inventory,Profile body
and snapshot. `origin.root()` and `origin.snapshot()` return those exact loans. Copying a carrier
copies references only;the host,snapshot,Profile and body need neither Clone nor Debug,and body
payloads may be unsized. Debug output omits body,signature role and pin text.

These helpers expose only already-held original fields. They neither create a body from an
operation nor choose a Profile. Their public construction permits mismatched or equal-data
foreign fields for later inspection and typed binding refusal:

```rust
use sc_core::{construction::*,design::{DesignInputs,DesignMetadataHost},
    recipe::{SemanticOperationKind,SemanticRuleDeclaration},ontology::EntityId};
fn design_rule<'a,H:DesignMetadataHost,S:?Sized,P:?Sized,B:?Sized>(
    root:&'a DesignInputs<H>,snapshot:&'a DraftingSourceRecord<S>,
    declaration:&'a SemanticRuleDeclaration,body:&'a DraftingSourceRecord<B>)
    ->DraftingRuleSource<'a,H,S,P,B> {
    DraftingRuleSource {declaration,body,origin:DraftingRuleOrigin::Design {root,snapshot}}
}
fn profile_rule<'a,H:DesignMetadataHost,S:?Sized,P:?Sized,B:?Sized>(
    profile:DraftingProfileSource<'a,H,S,P>,declaration:&'a SemanticRuleDeclaration,
    body:&'a DraftingSourceRecord<B>)->DraftingRuleSource<'a,H,S,P,B> {
    DraftingRuleSource {declaration,body,origin:DraftingRuleOrigin::Profile(profile)}
}
fn original<'a,H:DesignMetadataHost,S:DraftingRuleProvider<H>+?Sized>(provider:&'a S,
    rule:EntityId,kind:SemanticOperationKind,origin:DraftingRuleSelection<'_>)
    ->Result<DraftingRuleResponse<'a,H,S>,S::Error> {
    provider.rule_source(DraftingRuleRequest {rule,kind,origin})
}
```

`DraftingRuleProvider` extends the existing immutable context and selected-Profile ports. Its
`RuleBody` associated type is the actual opaque body payload;the Profile type comes from the
same selected-Profile provider. A Design-only integration refuses unsupported Profile selection
through its original typed cause. It supplies no invented Profile or empty body success.

`rule_source` receives an exact rule ID,expected operation family and explicit
`DraftingRuleSelection::Design` or `Profile(&SelectedProfilePin)`. Numeric rule/parameter IDs do
not select a factory,latest revision or default policy. `DraftingRuleResponse` is a transparent
alias using the provider's original snapshot,Profile and body types. Response loans belong to
the provider independently of the request pin's lifetime;that pin is selection intent and cannot
replace the original source pin. Missing,retired,ambiguous,unsupported-family and Profile failures
retain original typed provider causes and their source chains.

The inherited complete resource walk must cover every actual declaration/signature,opaque body,
origin,pin,canonical inventory,unused record,retained copy/cache and represented counter before
lookup. Use `charge_semantic_rule_resources` for each declaration and actual integration-owned
producers for bodies. Precharge collections before traversal,without cloning,normalizing or
supplied totals. An opaque body still has real accounting obligations. The provider query loads,
allocates and evaluates nothing;the integration must expose the original declaration/body
association honestly. Signature/body conformance is a separate execution obligation.

The maintained fixture accounts for three rule records,three one-input signatures,all body/role
text,an unused cache and a 129-bit retained revision. These add 6 entries,100 UTF-8 bytes and129
counter bits to its existing complete source walk. The containing Root/provider demand is
13/T/138;prior2 entries/3 text bytes/1 bit compose to15/(T+3)/139. Maximum9 entries refuses the
three-record vector at7+3 before traversal;T−1 text bytes refuses the final18-byte unused cache.
An ignored provider refusal remains the sink's original first cause and no rule query runs.
These are source-walk conformance checks,not rule-binding or execution acceptance.

Direct port calls and raw carriers perform no automatic content admission or association check.
The existing fresh Design semantic scope can walk the complete Root/provider,and work admission
below now bounds prospective rule comparisons. The source binder below checks actual Root/snapshot/origin,
selected Profile owners and rule identity/family;result and local input signature matching follow. Public raw fields
prove none of those matches. Canonical domain/state,value/evidence truth,G3 physical policy
execution and complete program acceptance remain separate;`.5f.2.verify` owns deferred coupling.

## Fresh rule binding work admission

`DraftingContext::prepare_rule_binding` freshly walks the complete actual Root,all held provider
content and the explicit requested Profile pin when present. Earlier context,member and signature
constructor admissions grant no content credit. The request supplies an operation ID,expected
semantic family and `DraftingRuleSelection`;the operation's actual rule/policy ID is read only
after complete work admission and original Root member lookup.

`DraftingRuleBindingLimits::new(content,matching_inspections)` selects both allowances explicitly.
Let E be the newly walked total collection entries,including all unused signatures,rule records,
copies and caches. Original operation input count P and any honest held source signature count Q
are each at most E. A complete local-role search therefore requires at most E² role inspections.
The conservative bound W=(E+8)² additionally allows16E linear inspections and64 fixed header/control
inspections. It covers prospective rule association/signature matching and original operation/output
member work. Whole semantic input/dependency binding has its own later complete policy;provider
internal algorithms,string-comparison byte work,content traversal,CPU time,allocator capacity and
execution are not certified by W.

Both E+8 and its square are checked before the explicit W quota. Only then does the method resolve
the exact original Root semantic operation and expected family. Thus whole content refusal comes
before arithmetic/work refusal,and work refusal comes before missing/wrong-kind member refusal.
An ignored provider sink failure remains the original first cause. No Profile source,rule body,
currentness or value query runs,and no request/result vector is allocated. Refused diagnostics
are boxed;the immutable successful scope retains original loans and fixed metadata only.

```rust
use sc_core::{construction::*,design::{DesignResourceLimits,DesignResourceSource},
    ontology::EntityId,recipe::SemanticOperationKind};
fn prepare<'a,H:DesignResourceSource,S:DraftingRuleProvider<H>+?Sized>(
    context:DraftingContext<'a,H,S>,operation:EntityId,kind:SemanticOperationKind,
    selection:DraftingRuleSelection<'a>,content:DesignResourceLimits,inspections:usize)
    ->Result<AdmittedDraftingRuleWork<'a,H,S>,String> {
    context.prepare_rule_binding(operation,kind,selection,
        DraftingRuleBindingLimits::new(content,inspections)).map_err(|e|e.to_string())
}
```

`AdmittedDraftingRuleWork` exposes its actual `context()`,original recipe/member/index through
`operation()`,exact requested `selection()`,`limits()`,fresh `demand()` and derived `shape()`.
`request()` derives the rule/policy ID and family from that actual operation while retaining the
original source-selection intent. It does not query or certify that source. A requested Profile
pin may be created after the context;the returned scope remains tied to that shorter borrow.
Private fields and the pin lifetime prevent mutation or escape into a longer-lived certificate.

The maintained fixture has E=21 and W=841. Root logical domain/semantic role text adds24 bytes to
the prior source fixture's text T;an explicit Profile intent adds its separately owned16 pin bytes.
Complete demand is21/T′/138 and prior2 entries/3 text bytes/1 bit compose to23/(T′+3)/139.
Maximum20 entries refuses the unused signature at20+1 before either W or missing operation999.
W maximum840 refuses required841 before that missing member;841 permits the original missing or
wrong-kind cause. A Profile text maximum T′−1 refuses the final one-byte content-version field
before work/member/source lookup. Every case keeps zero rule queries. Checked arithmetic also
refuses E+8 and square overflow without constructing fake source populations.

`charge_resources` rewalks every actual Root/provider/request-pin field in the caller's cumulative
sink;cached demand/shape grant no fresh content credit. It preserves earlier/ignored first errors
and performs no source/member/value query or matching scan. `DraftingRuleWorkError` retains the
exact context,requested operation/family,selection,policy and original resource/arithmetic/quota
or lending member cause. Resource causes retain their standard error chain;member causes retain
their actual original operation loan through the typed refusal getter.

This scope accepts complete prospective work and original Root membership. Actual selected
Profile/rule/body/origin association,complete local signature and logical Derived-output relation
are implemented below. This work scope alone supplies no canonical input value,physical Buttonhole
result,source/schema/evidence truth,whole dependency order or executable program. Coupled
verification remains `.5f.2.verify` after the agreed integration set.

## Bind the original rule source association

`AdmittedDraftingRuleWork::bind_source` loans its exact original work parent and repeats complete
fresh Root/provider/request-pin content,W and member admission. An earlier demand/shape gives no
credit. For explicit Profile intent,it then uses the existing `bind_profile_source` method,which
performs its own fresh content walk before selected-source lookup. Both readonly lookups follow
the complete prospective matching preflight;the immutable provider contract keeps original
semantic records stable. Design selection performs no Profile lookup.

The method resolves one exact rule request derived from the actual Root operation. It checks
returned Root identity,snapshot record identity,Design/Profile origin,actual Profile pin/inventory/
body owners,rule ID and family in that order. Profile header fields must be the very originals
accepted by the selected-Profile binder. A distinct equal-label pin,equal-data canonical inventory
or equal-payload Profile record cannot substitute for those owners. Requested pins remain separate
selection intent;the existing Profile binder compares their exact labels before the rule-header
comparison requires original source identities.

```rust
use sc_core::{construction::*,design::DesignResourceSource};
fn original_source<'w,'a,H:DesignResourceSource,S:DraftingRuleProvider<H>+?Sized>(
    work:&'w AdmittedDraftingRuleWork<'a,H,S>)
    ->Result<BoundDraftingRuleSource<'w,'a,H,S>,String> {
    work.bind_source().map_err(|e|e.to_string())
}
```

`BoundDraftingRuleSource::work()` loans the exact original parent. `source()` returns the original
declaration/body/origin view;`profile_source()` returns the accepted original Profile header when
requested. Both are borrowed views of the provider's already-accounted records. `demand()` and
`shape()` retain this bind's fresh content and prospective work metadata. The private proof cannot
outlive its work parent or replace a source field. No body,domain,state or signature buffer is
cloned or allocated,and the opaque body must remain the actual source's original record. Core
does not prove that an opaque body implements its declaration;that remains the honest immutable
integration and execution/conformance contract.

The maintained Design/Profile fixture retains exact21/T′/138 and W841. Source binding adds one
fresh work walk;Profile selection also adds the selected-Profile binder's independent walk.
There is one rule query. A subsequent `charge_resources` walk composes prior2 entries/3 text
bytes/1 bit to23/(T′+3)/139 without a new lookup or cached demand credit.

Eight direct mismatch controls preserve first-cause priority and complete original returned
witnesses:foreign Root with the same ID,foreign equal-payload snapshot,wrong origin,foreign equal-label
pin,foreign equal-data inventory,foreign equal-payload Profile record,wrong rule ID and wrong
family under the same ID. Several controls also have a later bad field,so the priority is checked.
The source proof deliberately accepts only source association;the fixture's local roles still
need the separate signature checker. No argument/output or executable prefix is certified.

An additional complete source holds33 collection entries,W1681 and every unused alternative
record. A negative control grows its actual separately held test cache by one after initial work.
Repeating admission refuses33+1 at the original cache field when maximum33 is selected,including
an ignored provider failure. With content maximum34 but W maximum1681,new required1764 refuses
before source lookup. These controls falsify cached-credit reuse;they do not relax the immutable
source contract. A failed Profile admission makes zero rule queries;an original rule-provider
failure retains its typed rule request and standard error cause chain.

`DraftingRuleSourceError` retains `work()`,fresh `demand()`/`shape()` when work succeeded,accepted
`profile_source()` when available,and the complete original `returned()` rule source when lookup
succeeded. `refusal()` distinguishes the original work error,selected-Profile error,provider error
and exact first `DraftingRuleSourceIssue`. Nested lending causes remain available through typed
getters;underlying resource/provider causes retain their standard source chains. Failure exposes
no private successful source proof.

`charge_resources` exhaustively rewalks the actual work parent and guards the borrowed source/
Profile views and fixed fresh metadata. It performs no source,currentness,member,value or local
signature query. Complete named input/result matching and the actual canonical logical
Derived-output relation are implemented below. Canonical input binding,source/schema/evidence
truth,G3 physical results,whole dependency order and execution remain separate obligations;
coupled verification is deferred to `.5f.2.verify`.

## Bind complete local rule signature and logical output

`BoundDraftingRuleSource::bind_signature` repeats complete fresh content,W and Root member
admission before inspecting the original operation/signature metadata. It performs no additional
Profile,rule,currentness or value query. The accepted source declaration and original operation
remain immutable loans;no signature,domain,state or body is cloned or selected.

The checker distinguishes a supplied reference's declared kind from its actual canonical source
binding. A Design logical target or Profile parameter retains its explicit `LogicalInputKind` and
original representation. Numeric targets retain dedicated Length versus Scalar with the precise
`ScalarInputKind`;a general Scalar(Length) cannot satisfy a dedicated Length requirement. Matching
these metadata fields does **not** prove actual canonical input membership,Profile parameter
metadata or source/evidence truth. Those dependencies still require the whole semantic checker.

After fresh admission,the refusal order is:

1. Actual operation/signature family and logical result-kind agreement.
2. Complete supplied/required named input counts,including missing or extra requirements.
3. Each original operation input's local role,then exact kind,in original operation order.
4. Actual Root logical output membership and kind,when the operation is logical.
5. That actual canonical record must be `Derived` and name this exact semantic operation.

Both immutable constructors already establish unique local roles. Equal counts plus complete
name membership therefore establish a bijection;no positional meaning or reverse scan is needed.
For example,operation roles `[source_a,source_b]` match signature order `[source_b,source_a]` as
original index pairs `(0,1)` and `(1,0)`. Equal kinds never excuse repeated roles at construction.
All prospective work is bounded before a missing role,kind or output diagnostic is inspected.

```rust
use sc_core::{construction::*,design::DesignResourceSource};
fn signature<'b,'w,'a,H:DesignResourceSource,S:DraftingRuleProvider<H>+?Sized>(
    source:&'b BoundDraftingRuleSource<'w,'a,H,S>)
    ->Result<BoundDraftingRuleSignature<'b,'w,'a,H,S>,String> {
    source.bind_signature().map_err(|e|e.to_string())
}
```

`BoundDraftingRuleSignature::parent()` preserves the exact original source proof. `input_count()`
is the original complete named population. `input_match(index)` returns a private
`DraftingRuleInputMatch` for one valid original operation-input position,or None for an invalid
index. It exposes both original owners through `operation()`/`rule()`,both authored positions
through `input_index()`/`requirement_index()`,and original field loans through `pair()`.
`DraftingRuleInputPair::Logical` retains the exact Design/Profile/numeric input and corresponding
logical-rule requirement;`Buttonhole` retains the exact numeric policy input/requirement. The
public pair enum is a raw view;constructing one cannot forge the private match or signature proof.
No local token buffer is copied. Each getter search is one bounded allocation-free scan;repeated
caller inspections need their own aggregate allowance. Local matches remain tied to their original
data owners and prove no actual input source or value binding.

`output()` is None for Buttonhole. Its mandatory canonical button source remains in the original
operation,and its physical Length result is still G3 execution's responsibility. An empty named
policy signature removes neither obligation. Logical operations return the actual Root
`LogicalInputBinding`:original inventory/member/index,domain,state and provenance. The checker
refuses Unknown,Known and every other non-Derived output instead of upgrading or overwriting it.
It also refuses a Derived record naming another operation. The exact operation output reference
and canonical member are joined;no logical domain member is computed or validated as a result.
Both logical and Buttonhole named signatures may be explicitly empty.

The maintained fixture covers Design and Profile logical input representations,reversed logical
and numeric policy requirement order,and actual Root output90/index0/Derived80/domain `output_a`.
The same-ID Profile declaration remains separately owned and cannot supply that Root output.
Native positive demand is26/T′/9,W1156;prior2 entries/3 text bytes/1 bit compose to28/(T′+3)/10.
Five focused tests cover complete named matches,result/count/extra/missing/type refusals,actual
missing/wrong-kind/non-Derived/other-operation output causes,empty signatures and fresh admission.
No additional rule query runs after source association.

A deliberate actual cache-growth control26→27 refuses26+1 under maximum26,including an ignored
provider sink error. With content maximum27 but W maximum1156,new required1225 refuses before
the known bad local role. These controls falsify cached-credit reuse without relaxing the
immutable source contract. Missing output90 and wrong-kind output retain original lending
causes;Unknown/Known or other Derived records remain unchanged and inspectable.

`DraftingRuleSignatureError` preserves `parent()`,fresh `demand()`/`shape()` when work succeeded,
the original raw `output()` record when encountered,and the exact `refusal()`. Typed refusals
retain original work errors,first `DraftingRuleSignatureIssue`,lending output lookup cause,
non-Derived state or expected/actual Derived operation IDs. No partial successful signature or
accepted input vector is published. Underlying resource causes keep their standard source chain;
lending records remain available through typed getters.

`charge_resources` freshly rewalks the original complete parent content and guards output loans
and fixed demand/shape metadata. It performs no matching or source/value query. This is local
signature and one canonical output-relation acceptance. Actual canonical input/Profile metadata,
whole source/order/graph proof,body conformance and result-domain/value validation,physical
Buttonhole length,G4 truth and execution remain separate. Coupled verification stays deferred to
`.5f.2.verify` after the agreed integration set.

## Whole semantic consumer binding contract (planned)

The source primitives above are implemented with focused checks. The aggregate semantic consumer
binder described here is **planned**. This contract owns the next implementation slices;it grants
no new public proof,selected logical value,whole-program acceptance or physical execution.
Roadmap §2 requires replayable recipe semantics and explicit uncertainty;§8.2 makes dependency
closure specific to the requested artifact. Complete static checking preserves all occurrences
in its selected source universe. A later artifact closure may use that graph without silently
omitting inputs during admission or resolving an unrelated Unknown.

The whole binder must join the actual admitted garment/context with the existing private
program source/order/acyclic/canonical/geometry owners. Independently recomputed equal-looking
proofs cannot substitute. The original mixed sequence,scalar recipe and semantic recipe remain
separate owners with their genuine ordinals and positions. Existing
`DraftingProgramDependencyInputs` already joins source/order/acyclic owners;canonical and geometry
proofs add their own complete obligations. Semantic composition must retain those owners and
boundaries rather than rebuilding a second source graph from IDs.

### Original consumers and order

Root-owned semantic records and ordered recipe consumers have different provenance:

| Consumer | Original witnesses retained | Order rule |
| --- | --- | --- |
| Pocket | Actual Pocket,original inventory index,opening field,served copy/Piece and every original component index/reference | No mixed position is authored on this Root record;none is invented |
| Closure | Actual Closure/index,kind,canonical size fields,physical instances and original derivation field | No invented native creator or mixed position |
| LogicalDerivation | Actual Root operation and input/role index,output and rule source request | Genuine checked semantic ordinal and mixed consumer position |
| Buttonhole | Actual Root operation,canonical button,policy and every original named numeric input | Genuine checked semantic ordinal and mixed consumer position |
| Native drafting input | Actual checked mixed step and original typed semantic target fields | Genuine original mixed position;family proofs preserve actual canonical parents |

The first occurrence view must stream original loans and locations without source queries,value
reads,result buffers or fabricated positions. Complete counts precede traversal in the later work
stage. Repeated references remain separate occurrences;an empty iterator for one dependency class
cannot certify that a native family has no obligations in other classes.

A logical Declaration source must bind its actual Root member and expected logical kind. A Profile
source must bind an explicitly selected original pin,schema/content source,canonical inventory and
actual parameter/member. Complete source-selection metadata must be accounted before queries;
missing,duplicate,extra or foreign selection records refuse. A parameter ID,matching pin text or
shared domain token cannot choose an owner. Rule/policy origin is also explicit. No implicit Design
first lookup,Profile fallback or replacement source is allowed.

Known,Assumed,Unknown,Choice,Preference and Derived retain their original domain,state and
provenance. Static binding selects no domain member and supplies no physical opening/hardware
vocabulary. A local rule signature specifies kinds and named roles;it does not authorize domain
conversion or evidence acceptance. Numeric targets retain dedicated Length versus general Scalar
representation and exact scalar kind. Numeric Profile fields need their genuine numeric source
contract;the logical Profile port cannot be coerced into one.

For each Design/Root Derived logical input,bind its sole original operation,the exact logical
output member and kind,then its genuine semantic/mixed position. An ordered recipe consumer requires that
supplier strictly earlier;self/future/missing/foreign suppliers refuse before value access. The
complete relation must cover all original operation inputs,not only a successful prefix. Root
Pocket/Closure consumers instead retain their Root location and link to the actual operation;
their absent position cannot be replaced with a fabricated end-of-program sentinel.

A Profile member's Derived state remains attached to its actual external Profile source. Its
operation reference is not a Root operation merely because the IDs match. Profile-internal
recipe/order acceptance requires genuine Profile metadata and its stated proof scope;this G1
consumer join cannot invent that program or a current mixed position. It retains the external
source obligation and states that boundary explicitly rather than claiming its body was checked.

For example,the following describes metadata relations,not new formula syntax:

```text
Root Pocket P: opening -> declaration90
Root declaration90: PocketOpening, Derived(operation80)
Mixed position2: semantic operation80 -> declaration90
```

The Pocket remains a Root consumer with no invented mixed position. If operation80 itself reads
a logical declaration Derived from a supplier at position4,its ordered input is a future dependency
and must refuse. If that input is Unknown,static association can retain the Unknown without
choosing a member;later execution/export still needs the relevant resolution.

### Garment and operation joins

A Pocket's opening must resolve as PocketOpening under its actual source owner. Its served copy
and every component reference retain both copy and expected-Piece identity,actual current members
and original orientation. Component source acceptance reuses the complete original association
binder above. Captured recipe creator positions belong to that captured original program;they
cannot become current-program order evidence merely because operation IDs match. An imported
component keeps its actual primitive record and has no fabricated creating recipe or position.

A ButtonAndButtonhole Closure must bind the actual typed Buttonhole operation named by its
original `ButtonholeDerivation`. The Closure's canonical button and that operation's button must
resolve to the same actual declaration/Profile member and selected source owner. Equal parameter
IDs under different original Profiles are insufficient. Its complete named numeric policy inputs
and local policy signature must also bind. No independently authored hole length,default clearance,
physical formula or positive-length proof is introduced. HookAndBar retains both actual logical
size origins;CentredZipper retains its original length origin and numeric provenance. Unsupported
kinds retain their explicit envelope refusal.

Every LogicalDerivation must reuse complete original rule-source/signature/output association,
then bind each actual named logical/Profile/numeric input. Output must be the actual Root member
whose Derived state names that exact operation. Signature equality alone certifies no canonical
input membership,source selection,dependency order,domain truth or executable body.

The existing thirteen native families keep all their direct geometry and numeric obligations.
Their additional semantic targets are classified below;this table is an implementation contract,
not a claim that those whole joins are already implemented:

| Original native variant | Additional semantic association to preserve |
| --- | --- |
| `ConstructPoint` | Original numeric arguments;no new garment target is invented |
| `DrawLine` | Original point references;no new garment target is invented |
| `DrawArc` | Original point/numeric fields;no new garment target is invented |
| `DrawBezier` | Original control references;no new garment target is invented |
| `OffsetEdge` | Actual SeamAllowance,width/corner/inclusion provenance and original owning Piece/range |
| `SlashSpread` | Actual named source Piece and original cut/bank/motion fields |
| `CloseDart` | Actual canonical Dart and its own closing-operation/intake/source fields |
| `AddNotch` | Actual requested Piece/anchor and original symbolic sample/production/Profile bindings |
| `PlaceGrainline` | Actual canonical Grainline,Piece and original arrow/alignment/print fields |
| `WalkSeam` | Actual span/copies/Pieces and every original balance pair/notch/Profile field |
| `TrueSeam` | Actual span,adjusted side,Piece and exact typed fit-tolerance source |
| `MirrorPiece` | Actual source Piece,axis and genuine existing family source/metadata parents |
| `JoinPieces` | Actual span,A/B Pieces,copies/inventory/CutPlan and genuine family source parents |

Actual geometry reached through a semantic target still needs the existing frame/snapshot and
supplier/currentness contracts. A direct geometry view with zero entries does not discharge that
work. Each family must either expose its genuine original source association or return an actual
typed missing/unsupported cause. Numeric/Profile/physical facts remain separate from their symbolic
references. No empty target,authored geometry copy,inferred frame transform or made-up native role
can fill a missing source. A context without an actual garment cannot manufacture one to satisfy
this binder;the existing narrower formula/geometry proofs retain their stated scopes.

### Complete bounds and acceptance

Fresh aggregate accounting must include the actual whole garment/Root/provider,original program
and held sources,all explicit Profile/rule selection requests,unused records/caches/copies and
represented counters. Prospective occurrence/comparison work and any result reservation have
explicit checked policies before child queries or allocation. Earlier source proofs,constructor
shapes,stored demand and individually successful lookups grant no aggregate credit. All original
collections are precharged before traversal,including refused/repeated/unused occurrences.

The implementation must bound repeated child-factory walks and matching work explicitly when
composing them. A per-component or per-rule bound alone cannot justify a whole consumer cohort.
Resource arithmetic/quotas precede child work;original first field/provider/owner/order/argument
causes and all raw witnesses survive. The final private semantic cohort exists only after every
required join succeeds. No successful Piece,rule,creator or consumer prefix becomes whole proof.
Physical source truth,evidence approval,G3 policy/geometry execution,allocator capacity and CPU
remain outside this static logical contract.

Implementation order is original occurrence views,complete explicit source-selection requests,
fresh aggregate work admission,logical/rule joins,garment/component/Closure joins,native target
joins,then the complete semantic cohort. Whole `.5f.2.check` composes it with existing owner/order/
context/argument proofs. Coupled `.5f.2.verify` activates after that agreed feature set is implemented;
focused checks remain the verification level for each preceding slice.

## Original Pocket and Closure dependency fields

`DesignGarment::pocket_closure_dependencies()` now streams original Pocket/Closure field views.
This first occurrence stage is implemented with focused checks;the planned whole semantic contract
above still needs ordered operation/native occurrences,complete source-selection requests,fresh
aggregate work and complete source binding. This method covers Pockets/Closures specifically;
it certifies no other garment family,program slot or whole dependency cohort.

Each private `DraftingGarmentSemanticUse` retains the actual garment,original member/index and
`DraftingGarmentSemanticField`. `owner()` returns either the actual Pocket and its original inventory
index,or the actual Closure and its original inventory index. Those are zero-based metadata
locations. There is no fabricated mixed-program position or creating operation. Copying a view
copies only original loans and fixed indices,without a Clone/Debug requirement on the host.

Pockets stream in authored inventory order. Within each Pocket,the fields follow its original
definition:served copy/expected-Piece reference,attachment position,directed orientation,logical
opening,and every component reference in original component order. Component fields retain their
actual zero-based index and exact original borrowed `PocketPieceRef`. Source-frame geometry and
copy orientation are not transformed or evaluated.

Closures follow the Pockets,in original Closure inventory order. A CentredZipper contributes its
original `ClosureLength` field. HookAndBar contributes distinct Hook and Bar source fields,even
when their authored `NotionSize` values are equal. ButtonAndButtonhole contributes the original
Button source and `ButtonholeDerivation`. Each kind is followed by every original physical instance
in instance order,with its original index and complete instance loan. Stable instance identity
remains its own field;the array index cannot replace it. The raw classifier explicitly retains the
unsupported Fly gap field;normal Closure construction already refuses that envelope.

The implementation exhaustively matches Pocket and Closure definitions,all Closure kinds and
instance identity/placement fields. Growing these records requires a fresh classification.
The fused iterator borrows original buffers and uses constant-size inline field views. It allocates,
clones,queries and selects no value;Debug prints roles/indices without authored length content.
It needs the model's existing `DesignReferenceHost` getter bound and no `DesignResourceSource` or
provider. Raw iteration grants no resource,comparison,source/currentness or executable allowance.
The future aggregate binder must preflight complete content and occurrence work before matching.

The focused fixture's one Pocket yields five fields:served,position,orientation,opening and
component0. Every field and owner pointer is the original actual object. A second fixture adds
three zipper origins (Explicit,Formula,Profile),HookAndBar with equal-but-distinct Declaration
fields,and ButtonAndButtonhole with a Profile button. Their original kind and instance fields
add twelve occurrences,for seventeen total. All Closure inventory indices and instance0 pointers
stay exact. Exhausted iterators continue returning None. API examples compile the narrow generic
getter contract and refuse private field mutation or original-owner lifetime escape.

## Original semantic operation dependency fields

`NormalizedDraftingProgram::semantic_dependencies()` now streams every original Buttonhole and
LogicalDerivation field at its genuine mixed step. This implemented raw occurrence stage follows
complete original program coverage and normalization. Namespace/kind/source/dependency checks,
explicit Profile/rule requests,aggregate work and whole semantic binding remain separate.

Each private `DraftingProgramSemanticUse` retains the exact actual normalized program,its genuine
`DraftingProgramStepView`,original semantic ordinal,actual Root operation and typed
`DraftingSemanticField`. `program()` keeps the actual source owner inspectable;an independently
normalized program over the same Root cannot replace it merely because IDs/positions match.
`position()` reports the genuine one-based mixed position. `ordinal()` reports the separate
one-based semantic recipe ordinal. Nested input indices remain zero-based original locations.

Fields stream in original mixed order. A Buttonhole contributes its original canonical button
origin,explicit policy reference,and every original named numeric policy input. A LogicalDerivation
contributes its typed output,explicit rule reference,and every original named logical/Profile/
numeric input. The complete input records retain their actual role buffers and typed sources.
Repeated targets remain separate named occurrences. There is no independently authored hole length,
source selection,canonical value,domain copy or fabricated formula/native operation.

Scalar/native slots contribute no semantic-operation fields to this scoped view. Their own
numeric,geometry and semantic target obligations remain separate,as do Root Pocket/Closure fields.
The producer exhaustively matches both operation variants,definitions and input records. Fixed
Copy guards cover typed source/target representations;role buffers stay borrowed. The fused
iterator uses constant-size inline header views and original input vectors,without queries,
clones,new arenas or result-vector allocation. It requires no new resource-source bound and grants
no content/comparison/source/currentness or executable allowance. Debug omits authored role text.

The focused mixed fixture has semantic1 at position1,scalar1 at2,a Point at3 and semantic2 at4.
Its Buttonhole headers plus two distinct roles sharing the same numeric target yield four fields.
Its logical headers plus Logical,Profile and Numeric inputs yield five more. All nine retain exact
Root operation/header/input/role/typed-source pointers and the actual program owner. A second
normalized program over the same Root and sequence remains a different owner. No provider callback
occurs during either stream. An actual empty program yields no fields and remains exhausted.

The logical input fixture intentionally retains a self-referential Derived target as raw metadata;
this view establishes no dependency-order acceptance. The future bounded binder must refuse that
relation when composing a whole proof. API examples compile the generic getter and refuse private
ordinal mutation or normalized-owner lifetime escape. Native target occurrences remain the next
slice;complete request/work/source/cohort and coupled verification remain tracked.

## Original raw native semantic fields

`DraftingOperationInput::semantic_dependencies()` now classifies all thirteen native schemas and
streams actual semantic target and symbolic Notch Profile field loans. This implemented raw stage
retains the exact original native input. Its program-owner/mixed-position wrapper is implemented below;
source selection,canonical association,currentness,aggregate work and executable acceptance remain
separate. No mixed position is inferred from an operation ID or local field index.

Each private `DraftingNativeSemanticUse` retains its original `DraftingOperationInput` and a
`DraftingNativeSemanticField`. Target fields retain their actual `EntityId` borrow and explicit
`DraftingNativeSemanticTargetRole`. Notch Profile fields retain their actual `ProfileParameterRef`
borrow and `DraftingNotchProfileRole`. Copying a view copies those loans and fixed metadata;it
constructs no canonical object,source body or selected Profile value.

| Original family | Fields in this semantic view |
| --- | --- |
| ConstructPoint,DrawLine,DrawArc,DrawBezier | No additional semantic targets;their original numeric/direct geometry obligations remain |
| OffsetEdge | Actual SeamAllowance target |
| SlashSpread | Actual source Piece target |
| CloseDart | Actual canonical Dart target,with the original request and explicit history identity retained |
| AddNotch | Actual owning Piece,then all six original symbolic Profile fields |
| PlaceGrainline | Actual canonical Grainline target |
| WalkSeam | Actual span,then every original pair/A/B/Profile field |
| TrueSeam | Actual span,adjusted Piece and fit-tolerance source;the original input retains its selected side |
| MirrorPiece | Actual source Piece;original axis remains on the native input/direct geometry view |
| JoinPieces | Actual span,source Piece A and source Piece B |

The six Profile roles are Style,SampleDepth,SampleWidth,ProductionDepth,ProductionWidth and
Encoding,in original schema order. Equal references remain separate authored occurrences. A Walk
streams its span first,then original pair order,A before B,and all six roles on each side. It
retains every original zero-based pair index and explicit `SeamSideId`;repeated balance pairs are
not deduplicated. With N pairs,the semantic stream has one span plus12N Profile occurrences.
An actual empty pair vector still contributes its original span and no invented Profile fields.

Independent exhaustive native-owner guards classify schema growth separately from numeric content
and direct geometry producers. Walk pair/notch and Profile binding/dimension records are also
matched exhaustively. The raw iterator borrows original target fields and buffers,uses constant-size
inline views,is fused,and allocates,clones,queries and evaluates nothing. It supplies no content,
comparison,Profile pin,source/frame/currentness or physical allowance. Complete aggregate preflight
and actual target/source/family joins remain mandatory before whole acceptance.

The focused fixture covers all thirteen families in both quadratic/cubic Bézier,rotation/separation
SlashSpread and point/range Mirror-axis modes. Four direct point/curve families contribute zero
fields to this class. Two original Walk pairs contribute25 fields. Across all families,the stream
contains43 occurrences:13 target fields,six Notch Profile fields and24 Walk Profile fields. Every
emitted view keeps the actual original input. Walk fields retain exact original nested Profile
pointers,pair indices,sides and role order. Equal-ID Mirror inputs and equal Notch Profile values
in separate original inputs retain different actual field loans. An empty Walk retains its span
and remains exhausted after it. Deferred invalid numeric expressions are not evaluated by this view.

API examples compile the raw getter and refuse private-field mutation or original-data lifetime
escape. These are original-field classification checks. They establish no existence or liveness of
the referenced Dart,Piece,span,allowance,grainline,tolerance or Profile declaration. The wrapper below
keeps these original raw fields under the exact normalized program and genuine mixed step;
complete semantic source/work/cohort and coupled verification remain tracked.

## Original native semantic fields in the mixed program

`NormalizedDraftingProgram::native_semantic_dependencies()` now wraps every original raw native
semantic field with the exact actual normalized program and genuine mixed step. It reuses the
complete thirteen-family producer above. This completes the planned original occurrence-view
stage;explicit source-selection requests,aggregate work and actual source/cohort binding remain
pending. Original coverage/normalization does not supply namespace,kind,dependency-order,currentness
or executable acceptance.

Each private `DraftingProgramNativeSemanticUse` retains `program()`,`step()` and `native()`.
`position()` comes from its actual original `DraftingProgramStepView`. The raw native view keeps
its exact input,target/Profile reference and original pair/A/B/role metadata. No scalar or semantic
recipe ordinal is fabricated for a native operation. A separately normalized program over equal
source data remains a different owner;later proof joins must use this exact actual program.

Iteration follows genuine mixed order,then the raw producer's original local field order. Scalar
and semantic-only slots contribute no fields to this specific stream and keep their own dependency
obligations. Four point/curve families have zero additional semantic targets;their numeric/direct
geometry still belongs to the existing producers. There is no deduplication,body clone,new arena,
result-vector reservation,provider callback or value read. The fused wrapper borrows all original
owners and grants no new content/comparison/source-work allowance.

The focused fixture inserts scalar1 at mixed position1 and scalar2 at10 around thirteen native
inputs. Semantic fields occur at positions6/7/8/9,then11/12/13/14/15. The Walk at12 preserves all25
of its raw fields,including both original pairs and every A/B/Profile role. All43 fields retain
exact original raw target/Profile pointers,input loans and local locations. A second normalized
program shares the same source data but cannot replace the actual program owner. Both curve/
motion/axis modes pass,and iteration leaves the provider trace at0.

An actual scalar-only program produces no native semantic fields and stays exhausted. Its deferred
invalid numeric expression remains unevaluated. The shared raw thirteen-family fixture still
passes all target/Profile/pair and equal-ID owner checks. API examples compile the generic wrapper
and refuse private raw-use replacement or normalized-owner lifetime escape. Root Pocket/Closure,
ordered semantic operation and native occurrence views are now implemented with focused checks;
complete request/work/source/family/cohort and whole/coupled verification remain separate.

## Explicit semantic source-selection request contract (planned)

Original occurrence views,raw selection request records and record-only accounting are implemented.
Local admission described here is **planned**. Raw records below hold authored intent and create no actual
source,member,dependency,Profile value or whole binding proof.

A request owner will retain one original ordered row vector. A Profile row supplies a fixed typed
relative occurrence location and an original explicit `SelectedProfilePin` loan. A rule row supplies
a requested one-based semantic ordinal and explicit `DraftingRuleSelection::Design` or Profile pin.
Design is an authored choice;missing rows cannot silently become Design or a latest/default Profile.
Rule/policy IDs and expected families come from the actual Root operation during binding,never
from a replacement source declaration supplied in the request.

Relative locations preserve the three existing occurrence classes:

| Request class | Coordinates and field identity | Accepted provenance comes from |
| --- | --- | --- |
| Garment | Original zero-based Pocket/Closure index and typed original field/nested role | Actual current garment/member/field view |
| Semantic operation | Requested one-based semantic ordinal and typed header/input index | Actual Root semantic member and genuine normalized mixed step |
| Native input | Requested one-based mixed position and actual target or Notch/Profile/pair/A/B role | Exact normalized program/step and original raw native field |

These integers are requested coordinates,not issued positions or identity certificates. Zero,
missing,out-of-range or wrong-kind coordinates remain inspectable data until the actual-scope
factory checks them. A scalar ordinal,operation ID,Profile parameter ID or pointer alone cannot
manufacture a consumer location. Reusing the same native input twice produces two distinct mixed
consumers,even though their raw field pointers match. Requests must distinguish those positions.
Raw location keys contain fixed metadata and do not hide borrowed foreign programs,garments or
source bodies. Actual proof owners are supplied and joined by the later whole work factory.

For example,these are request descriptions rather than new Rust API or formula syntax:

```text
Native position12 / pair0 / sideA / SampleDepth -> explicit pin P
Native position12 / pair1 / sideA / SampleDepth -> explicit pin P
Semantic ordinal2 / logical input1 -> explicit pin Q
Semantic ordinal1 / rule origin -> explicit Design
```

The two Walk rows remain separate even when their parameter IDs and selected pin agree. The
semantic input and rule-origin requests keep their own coordinate classes. Matching IDs or
labels do not collapse these rows or imply that P and Q are physically compatible.

Direct Profile fields use their actual field location. A native canonical target may itself
retain Profile references,for example an allowance or grainline descriptor. An explicit target
context selection applies to the actual Profile references reached through that authentic target
association;it does not create those fields or prove that the target needs a Profile. Actual target
metadata and family proof must be inspected under complete bounds before those needs are known.
If no such need exists,the context row cannot quietly count as a consumed selection. Missing
native source support returns its original typed cause,not an invented descriptor or empty source.

Every selected pin preserves its original profile/schema/content strings and lifetime. The records
parse no version,trim no label,select no value and copy no domains. The raw row vector moves into
its owner unchanged,including invalid coordinates,duplicate keys and extra rows. Inspection retains
original row order and indices. A separate private local admission borrows that exact raw owner.
It freshly accounts complete record content and checked conservative duplicate-key comparison
work before scanning. Duplicate selection keys refuse regardless of differing pins/origins.
No result map,index buffer,source lookup or successful source prefix is needed for local admission.

Record-only accounting must precharge the complete original vector before traversal,then charge
all three actual pin strings in each Profile row and Profile-origin rule row. Repeated or unused
pin loans still count as original row occurrences. Design-origin rule rows and fixed location/kind
metadata own no dynamic body. Exhaustive row/key/origin views and the existing selected-pin producer
must preserve first cumulative field failures,including an earlier latch on an empty vector.
The containing caller supplies the real scope/owner;opaque labels cannot manufacture entity IDs.
Root/provider/program bodies are accounted separately through their actual owners. Local shapes,
previous admission and borrowed record views give no later whole-work credit.

Coverage is deliberately staged. After fresh aggregate owner/content/work preflight,the coverage
factory matches direct keys to actual original occurrences and complete direct Profile/rule needs.
Local record admission alone proves none of that. Native target-internal Profile requirements are
confirmed by actual target/family association;final cohort acceptance also checks that no supplied
selection row remains unconsumed. All candidate rows and pins are accounted before any child query,
and all child-walk/comparison/result bounds are explicit before work. No partial consumer/source
success can hide a missing,extra,foreign or unsupported selection.

Request records,record-only accounting and local structural admission are separately owned next
slices. Aggregate `.work.coverage`,native-family binders and the final cohort own their respective
actual-source coverage stages. G4 registry/schema/evidence and Profile-internal body acceptance,
physical compatibility,execution and coupled verification remain separate.

## Original typed semantic selection rows

`DraftingSemanticSelectionInputs::new(rows)` now owns the caller's exact original ordered
`Vec<DraftingSemanticSelectionRow>` without validation,cloning,lookup or source selection. The raw
owner,typed coordinates and record accounting are implemented with focused checks. Local admission,
actual scope/coverage,source binding and aggregate work remain the next stages.

A Profile row holds its requested `DraftingSemanticSelectionLocation` and original explicit pin
loan. A Rule row holds a requested semantic ordinal and explicit Design/Profile origin. All three
pin labels remain in their actual original `SelectedProfilePin`;the row copies no strings or
values. `rows()` lends the original immutable slice,including duplicates,extra rows and invalid
coordinates. Construction moves the existing vector buffer unchanged and creates no result map
or replacement source. An explicitly empty vector is raw request data,not a complete binding.

Locations use four typed variants: Pocket,Closure,Semantic and Native. The first two belong to
the same garment occurrence class. Pocket and Closure coordinates use requested zero-based
inventory indices and distinct typed field selectors. Semantic coordinates use a requested
one-based semantic ordinal plus a typed header/input selector. Native coordinates use a requested
one-based mixed position plus a target or Notch/Profile/pair/A/B selector. Nested indices remain
zero-based. The field selector families preserve the existing5/5/6/3 raw field classes;no arbitrary
string path,parameter ID or operation ID substitutes for those coordinates.

`DraftingSemanticSelectionRow::key()` projects only the fixed structural key. Profile keys use
the complete requested location. Rule keys use their requested semantic ordinal. Choosing another
pin or changing Design to Profile does not create another key for the same requested target.
This getter performs no matching or duplicate validation. The future local admission must refuse
duplicate keys;the raw owner deliberately retains every original conflicting row for inspection.

The focused fixture retains eight rows in the original vector allocation,with exact first/second
pin pointers and authored order. It includes a zero rule ordinal,a zero semantic ordinal with
input999,two Walk pair locations,duplicate Profile and Rule keys with different selections,and
a nonexistent Closure99 request. None is silently normalized,dropped or accepted. Walk keys at
positions12/13,pairs0/1 and sidesA/B stay distinct even with the same selected pin and field role;
a semantic ordinal12 is a different coordinate class from native mixed position12.

Debug omits pin-label content. API examples compile the original-row constructor and refuse private
row mutation or selected-pin lifetime escape. These records contain no foreign program/garment
body and claim no actual owner,issued position,source existence,Profile compatibility,domain/value,
local/whole quota or default origin. Their literal coordinates are resolved only against the actual
scope at the later bounded factory,which retains both the original requests and genuine data views.

## Account for original semantic selection records

`DraftingSemanticSelectionInputs::charge_record_resources(sink,scope,owner)` now appends complete
original request content into the caller's cumulative sink. The caller supplies the actual
containing `DesignResourceScope` and optional entity owner. The producer does not manufacture an
entity from an opaque Profile label or infer a Root/program association from requested coordinates.
The caller owns and finishes the meter.

The complete original row vector is charged before any traversal,at static field
`drafting.semantic.selections.rows`,with index None. Every Profile row then charges its original
selected pin using the existing exhaustive pin producer. A Profile-origin Rule row does the same;
a Design-origin Rule row adds only its already-counted fixed row metadata. Every pin contributes
all three actual profile/schema/content strings in original order,with the actual original row
index. Repeated pins,unused selections,invalid coordinates and duplicate keys still count.

The method exhaustively matches its private owner,rows,origins,locations and nested typed field
selectors. Fixed metadata has Copy guards;no logical counter is manufactured from a position or
ordinal. It performs no request lookup,duplicate/coverage validation,Profile/source/currentness/
value query,Root/provider/body callback,clone or result reservation. Cached constructor metadata
provides no credit. This record-only walk does not account the actual containing program/garment/
provider bodies,which belong to their real owners and later aggregate admission.

The focused fixture contains four rows:Profile P,explicit Design rule,Profile P again,and a
Profile-origin rule using Q. Coordinates deliberately include zero and maximum requested indices;
the producer charges them as fixed data without indexing a real scope. P's three original strings
occupy6/3/3 UTF-8 bytes and Q's occupy3/4/3. Exact record demand is4 entries/34 text bytes/0 counter
bits. Prior2 entries,3 text bytes and1 bit compose to6/37/1. No source or physical value is read.

Entry maximum3 refuses the complete four-row vector before pin traversal. Text maximum33 refuses
the unused final rule pin's original content-version field at row3,after31 bytes with3 requested.
The exact caller scope/owner and original row index survive. Empty and nonempty walks preserve an
earlier latched cause;an actual empty row vector contributes0/0/0. The generic API example compiles
without a host/provider or source association. Local structural admission is next;complete actual
scope/target/source coverage,aggregate work and whole/coupled verification remain pending.
