# Owned Design inputs and sources

`DesignMetadata<H>` owns a DesignMetadataHost and its identity census; CanonicalInputs owns values.
MeasurementInputs and DesignSources implement this core port. Full Design assembly
follows; admission selects no table, SizeSet or Factory Profile.

## Admission and current reads

`DesignMetadata::new(host, &canonical)` checks the host's stable census, duplicate metadata ids and
collisions with either canonical declaration family, then calls the host's actual current validator.
An explicitly empty host is legal. Referenced input/evidence ids are uses, not metadata owners.
Family and authored positions are retained in collision witnesses; records are never deduplicated.

`measurement_declaration(measurement, &canonical)` repeats whole-scope validation, requires actual
Measurement metadata, then resolves its explicit binding through the host. The returned object must
be the actual dedicated canonical length in this supplied inventory. A detached equal copy refuses.
A canonical id cannot invent metadata or bindings. Unknown/Derived stays unchanged. Replacement
inputs require current validation. The returned borrow retains host and canonical context.

```rust
use sc_core::{
    design::{DesignMetadata, DesignMetadataError, DesignMetadataHost},
    ontology::EntityId,
    value::CanonicalInputs,
};
fn inspect<H: DesignMetadataHost>(host: H, canonical: &CanonicalInputs, measurement: EntityId)
    -> Result<(), DesignMetadataError<H::Error>>
{
    let metadata = DesignMetadata::new(host, canonical)?;
    let declaration = metadata.measurement_declaration(measurement, canonical)?;
    assert!(canonical.length(declaration.id()).is_some_and(|actual|
        core::ptr::eq(actual, declaration)));
    Ok(())
}
```

The host must supply an immutable complete snapshot and actual semantic validation. Core checks
observable identities/object membership; it cannot prove arbitrary provider binding semantics.
There is no stored self-reference or copied value/state cache. host() and entities() are shared views.
Default Debug exposes only the entity count.

## Public vocabulary and boundary

| API or variant | Meaning |
| --- | --- |
| `DesignMetadataHost` | Readonly higher metadata contract with original typed Error |
| `DesignSizeSetHost` | Additional readonly actual external set id/revision validation facet |
| `DesignMetadata` | Private checked host owner; admission is scoped to these canonical inputs |
| `DesignHostEntity` | Actual owned id, family and original within-family index |
| `DesignHostEntityKind` | Landmark, Procedure, Measurement, MeasurementTable, Ease, EaseSet, SizeSet, SizeMember, SizeAxis, SizeAxisValue, SizeBreak, SizeChart, SizeChartObservation |
| `DesignMetadataError` | Current scope or explicit binding refusal |
| `DuplicateIdentity` | Two actual metadata positions claim one id |
| `CanonicalIdentityCollision` | Metadata claims a canonical input id |
| `ChangedEntities` | Provider violated the stable census contract |
| `Host` | Original typed provider refusal, also retained as error source |
| `MissingMeasurement`, `WrongEntityKind` | Absent metadata or an actual different role |
| `NoncanonicalDeclaration` | Returned declaration is detached from the supplied canonical inventory |

Other Design identities, SizeSet metadata, formula source composition and whole readiness retain
separate owners. Profile ids, labels and result claims cannot resolve physical fields, including when
an id coincides with a numeric declaration; [G4 profile boundaries](material-inputs.md) still apply.
Focused native and privacy checks cover this port; full Design execution and integration remain
unverified. Run the focused contract through the local producer profile:

```bash
cargo test -p sc-core --test design_metadata_contract
```

## Actual measurement host

MeasurementInputs implements the port with owned landmarks, procedures, measurements, tables,
Ease mappings and sets, in that order and their authored order within each family. Referenced
canonical/evidence ids are excluded. Whole validation uses its actual current validators, including
unused mappings and compression permission. Measurement bindings use actual metadata and the
borrowed record context; a general scalar peer cannot replace a dedicated length target.

| API or variant | Meaning |
| --- | --- |
| `MeasurementHostError` | Original typed sc-measure port refusal |
| `Inputs`, `RecordContext`, `Measurement` | Whole inventory, target context or explicit binding cause |
| `MissingMeasurement` | Actual measurement metadata is absent |

The direct trait binding method checks the selected declaration/context only. DesignMetadata adds
whole-scope validation and canonical object membership. No borrowed context is stored.

```bash
cargo test -p sc-measure --test measurement_inputs_contract
```

## Owned common SizeSet metadata

SizeSetMetadata owns exact membership, axes/member points, required dimensions/available breaks
and an explicit Design table id. Chart-family owners below reuse this common source component;
full Design composition follows. Canonical values and measurement/Ease metadata stay
in their original supplied owners. Empty break inventories can be valid drafts, without coverage.

new(definition, &canonical, &inputs) checks all owned ids across canonical/measurement/Ease records,
then the set, members, each axis/its values and breaks in authored order. Even an unselected table
cannot reuse a member/axis/value/break id. Source references are uses, not extra declarations.
It then invokes actual current input/context/axes/break validators and requires the exact Design
table. Equal-content tables cannot replace a missing id. Replacement canonical inputs are rechecked.

with_context validates that whole scope before visiting a temporary SizeBreakContext. The callback
cannot retain the local contexts; owned results may be returned. No self-reference or value/state
cache is stored. Original canonical declarations are borrowed, including Unknown/Derived. Failures
skip the callback and preserve the typed cause. Break declaration error sources expose the concrete
SizeBreakError, including MissingDelta, rather than its allocation wrapper (D255 correction).

| API or variant | Meaning |
| --- | --- |
| `SizeSetMetadata`, `SizeSetMetadataDefinition` | Private common owner and explicit editable construction input |
| `SizeSetMetadataPosition` | Original ownership witness: Inputs, Set, Member, Axis, AxisValue, Break |
| `SizeSetMetadataError` | DuplicateIdentity, Inputs, MissingDesignTable, RecordContext, TableContext, Sources, BreakContext or Breaks |

identities() lists only common owned ids; definition() is shared. Debug exposes counts only.
validate_current checks current structure; chart coverage, grade coverage, evaluation and readiness
remain separate. For example, an authored required waist break with no available delta can be saved;
require_grade_coverage still refuses its missing cell. No zero delta is invented.

```bash
cargo test -p sc-measure --test size_set_metadata_contract
```

## Owned garment SizeSet sources

GarmentSizeSet owns common metadata, all observations and the chart. Construction checks source ids,
actual common sources, every observation and the original composite. A missing unused observation
table refuses. Design references this external object; no values/contexts are copied or stored.

with_inputs revalidates before visiting temporary GarmentSizeSetInputs. Readonly chart_context()/
break_context() expose actual shared contexts. Inside the callback, the chart's declaration(member,
pom, view.chart_context()) and a break's delta(view.break_context()) borrow original canonical data.
Owned results may return; local views/contexts cannot escape.

| API or variant | Meaning |
| --- | --- |
| `GarmentSizeSet`, `GarmentSizeSetDefinition` | Private garment source owner and exact construction input |
| `GarmentSizeSetPosition` | Common, Observation or Chart original ownership witness |
| `GarmentSizeSetError` | DuplicateIdentity, Metadata, Context, Observation, CollectionContext or Inputs cause |

Partial chart/break drafts remain valid. validate_inputs_for requires the original structural
coverage for Regeneration or GradeRules; it computes no geometry or unknown quantity. reference()
retains the actual set/revision, identities() lists owned ids, and Debug reveals observation count.
MTM and unified source hosts have separate contracts below; full Design/execution remain later owners.

```bash
cargo test -p sc-measure --test garment_size_set_contract
```

## Owned MTM SizeSet sources

MtmSizeSet owns singleton common metadata and the actual MtmChart. MeasurementInputs retains actual
body/Ease/table/set ownership. Construction checks complete input/common/chart ids and reuses the
original validators through temporary contexts sharing actual membership, records and Design table.
No body-plus-Ease value, profile selection or context cache is created.

with_inputs revalidates before visiting MtmSizeSetInputs. Its readonly chart_context()/break_context()
permit explicit quantity/coverage queries. The chart's body_declaration(pom, context) and
ease_declaration(pom, context) borrow separate actual canonical records. Local views cannot escape;
owned callback results may return. Missing replacement sources refuse before the callback.

| API or variant | Meaning |
| --- | --- |
| `MtmSizeSet`, `MtmSizeSetDefinition` | Private external MTM source owner and exact construction input |
| `MtmSizeSetPosition` | Common or Chart ownership witness |
| `MtmSizeSetError` | DuplicateIdentity, Metadata, Eases, Context or Inputs original cause |

Partial mappings stay drafts; Regeneration requires coverage, while GradeRules keeps the original
GradeRulesUnsupported chart/member refusal. Composite SizeSetInputError sources expose concrete
chart/MTM/break causes rather than boxes (D256 correction). No numeric execution/readiness follows.

```bash
cargo test -p sc-measure --test mtm_size_set_contract
```

## Unified external SizeSet inventory

SizeSet owns the explicit Garment/Mtm family and lends SizeSetInputs for current queries.
Singleton garment charts retain that provenance; errors retain actual family causes.

SizeSetInventory owns authored sets in order. It checks all shared and complete set ids before
current validators. Distinct sets cannot reuse owned ids; shared canonical references remain uses.
All sets, including unselected ones, validate. Empty explicit inventories are legal.

size_set(id) inspects a shared source. validate_reference(expected, &canonical, &inputs) revalidates
all sets and borrows the exact id/revision target, retaining current input borrows. Equal labels or
latest revisions supply no fallback. A selected valid MTM set therefore refuses when an unselected
garment set has lost its required axis declaration.

| API or variant | Meaning |
| --- | --- |
| `SizeSet`, `SizeSetPosition`, `SizeSetError` | Exact Garment/Mtm owner, original family position and typed family cause |
| `SizeSetInventory` | Private ordered external source inventory; sets() is shared |
| `SizeSetInventoryPosition` | Inputs or Set index with exact original nested source position |
| `SizeSetInventoryError` | DuplicateIdentity, Inputs, InvalidSet, MissingSet or RevisionMismatch |

Exact source admission and structural path coverage are separate checks:

```rust
use sc_core::{design::DesignSizeSetReference, value::CanonicalInputs};
use sc_measure::{MeasurementInputs, SizeInstantiationPath, SizeSetInventory};
fn require_regeneration(inventory: &SizeSetInventory, pin: DesignSizeSetReference,
    canonical: &CanonicalInputs, inputs: &MeasurementInputs) -> Result<(), Box<dyn std::error::Error>>
{
    let set = inventory.validate_reference(pin, canonical, inputs)?;
    set.with_inputs(canonical, inputs, |view|
        view.validate_inputs_for(SizeInstantiationPath::Regeneration))??;
    Ok(())
}
```

Unknown/Derived values still require their own observation/evaluation. This checks structural inputs,
not numeric execution, full Design readiness or release. Debug reveals only inventory count.

```bash
cargo test -p sc-measure --test size_set_inventory_contract
```

## Actual combined Design sources

DesignSources owns MeasurementInputs and SizeSetInventory through DesignSourcesDefinition, with no
canonical values or borrowed contexts stored. new(definition, &canonical) reuses complete actual
inventory/current checks. DesignSourcesError preserves SizeSets or Measurement typed causes.
Shared definition()/measurements()/size_sets() access supplies no mutator.

The core metadata catalog lists actual measurement/Ease records, then each set's common/chart sources.
Canonical and evidence references are excluded. Each category's index counts authored records across
the host; no enum ordering or numeric type selects a role. Core DesignMetadata adds its original
whole-host/current/object-membership guards before returning actual canonical measurement records.
Direct measurement binding remains targeted. Replacement inputs recheck unselected sources too.

DesignSizeSetHost::validate_size_set requires actual current sources and the exact external id/
revision. A present numeric id cannot replace missing set metadata. This readonly facet adds no
path coverage or profile value resolution. Physical Profile resolution retains its G4 owner,
including coincident numeric ids. Full Design composition and integration remain later work.

| API | Meaning |
| --- | --- |
| `DesignSources`, `DesignSourcesDefinition` | Private actual combined owner and exact editable construction input |
| `DesignSourcesError` | Original SizeSets or Measurement cause through core ports |

```bash
cargo test -p sc-measure --test design_sources_contract
```

## Owned core input composition

DesignInputs owns the implemented structure, parameters, exact recipe document, Materials and checked
metadata host. Its structure is the single CanonicalInputs owner. Provided source references are
checked below; selected input names are checked explicitly. Sewing/garment state and readiness remain separate.

Construction checks identities in order: structure (Design, canonical records, Pieces, copies,
operations), parameters, Materials, then higher source metadata. Even a retired creating operation
cannot share an id with higher metadata. Referenced evidence/formula/material ids are uses, not new
owners. DuplicateIdentity retains both actual original DesignInputsPosition witnesses.

It then rechecks actual parameter targets, whole Material properties/Piece assignments, and source
currentness against its own canonical records. Prior standalone validation cannot substitute for
that context. Unresolved assignments and uncertain states remain intact. No Piece is reconstructed
against an edited ledger, so held repair state remains inspectable. Recipe ownership preserves
original text/canonical syntax; unbound names still require the later binding/checking phase.

| API or variant | Meaning |
| --- | --- |
| `DesignInputs`, `DesignInputsDefinition` | Private implemented-component owner and exact construction input |
| `DesignInputsPosition` | Structure, Parameter, Material or Source original ownership position |
| `DesignInputsError` | DuplicateIdentity or original Parameters, Materials, Sources, References typed cause |

Component getters are shared. identities() lists actual owned ids; validate_current() repeats the scoped
checks. id()/revision() retain the actual structure's identity/counter without advancing it. Debug
exposes no customer content. Combining existing components requires no borrowed context storage:

```rust
use sc_core::{design::{DesignInputs, DesignInputsDefinition, DesignInputsError,
    DesignMetadata, DesignReferenceHost, DesignSourceReferences, DesignParameters, DesignStructure},
    ontology::MaterialInventory, recipe::FormulaRecipeDocument};
fn combine<H: DesignReferenceHost>(structure: DesignStructure, parameters: DesignParameters,
    recipe: FormulaRecipeDocument, materials: MaterialInventory, host: H, references: DesignSourceReferences)
    -> Result<DesignInputs<H>, DesignInputsError<H::Error>>
{
    let sources = DesignMetadata::new(host, structure.inputs())
        .map_err(|issue| DesignInputsError::Sources(Box::new(issue)))?;
    DesignInputs::new(DesignInputsDefinition {structure, parameters, recipe, materials, sources, references})
}
```

```bash
cargo test -p sc-core --test design_structure_contract
cargo test -p sc-measure --test design_inputs_contract
```

## Provided source references and joint scope

DesignSourceReferences owns three explicit Option fields: measurement_table, ease_set and size_set.
None means unprovided in a draft. No Default, singleton inference, label match or latest revision
fills a field. DesignInputs stores the exact carrier; references() returns its copy for inspection.
A copied carrier alone proves no currentness or readiness.

DesignReferenceHost validates actual current sources and each provided identity/pin. DesignSources
uses actual table/Ease/set metadata, then checks joins among provided fields: an Ease set's garment
table and a SizeSet's logical table must match a provided Design table. A selected MTM chart must
name the exact provided Ease set. A same-content peer with a different id cannot replace it.
Unprovided companion fields stay unprovided, even when the selected object names an obvious target.

For example, the actual MTM source may name table51/Ease60/SizeSet40 revision2. Providing all three
accepts structurally; selecting table50 or peer Ease61 refuses. Providing only the SizeSet leaves the
other two fields None. Numeric declaration2000 cannot stand in for any absent source metadata.

| API or variant | Meaning |
| --- | --- |
| `DesignSourceReferences` | Exact provided/unprovided metadata references; no declaration ownership |
| `DesignReferenceHost` | Readonly actual presence and joint-scope validation facet |
| `MissingMeasurementTable`, `MissingEaseSet` | Provided id has no actual record of the requested role |
| `EaseTableMismatch`, `SizeTableMismatch`, `MtmEaseSetMismatch` | Individually present records disagree in selected scope |
| `References` | DesignInputsError preserves the original actual host reference cause |

Whole ownership/current checks precede reference admission. Reference ids are uses, excluded from
the owned-id census. Presence and coherent scope still supply no complete namespace, path coverage,
numerical evaluation, physical Profile resolution or release readiness. Partial drafts remain valid.

## Formula names from actual selected inputs

DesignInputs.formula_namespace() first rechecks complete current input ownership and references.
It borrows parameter declarations in authored order, then the selected table's measurements and
selected Ease set's mappings. DesignFormulaHost supplies actual selected metadata through the
one-way port; DesignSources validates every source and exact saved binding before borrowing
canonical records. Local measurement contexts never escape or become stored self references.

```rust
use sc_core::{design::{DesignFormulaError, DesignFormulaHost, DesignInputs}, recipe::FormulaNamespace};
fn names<'a, H: DesignFormulaHost>(state: &'a DesignInputs<H>)
    -> Result<FormulaNamespace<'a>, DesignFormulaError<'a, H::Error>> {
    state.formula_namespace()
}
```

For example, parameter authored_body can borrow body declaration10 while selected table51 supplies
measurement2/garment_waist from declaration11 and Ease60 supplies mapping30/waist_ease from13.
The names preserve Parameter/Measurement/Ease origins and metadata identities. Canonical Unknown/
Derived states stay exact; general scalar parameters retain their explicit kind and target.
Unselected body table50 contributes no body_waist. With both source fields None, only parameters
and the eight reserved metadata names appear, even if a selected SizeSet names a table/Ease set.

If parameter5001 is instead named garment_waist, the namespace returns formula_ambiguous_name
with the original parameter and measurement declarations, in that order. Neither shadows the other.
A selected Ease entry named eps_num returns formula_rebinding with reserved and attempted sources.
No accepted namespace escapes a collision. The namespace and its diagnostics borrow the actual
state, so neither can outlive it. Borrowed collision payloads are inspected through the Namespace
variant; they cannot be exposed as static std::error::Error sources. Current input errors retain
their concrete error source chains.

| API or variant | Meaning |
| --- | --- |
| `DesignFormulaHost` | Readonly actual selected table/Ease declaration facet |
| `DesignFormulaError` | Current input or original borrowed initial-name refusal |
| `InvalidInitial` | Host supplied metadata that cannot seed an initial namespace |
| `Namespace` | Original reserved/ambiguous-name error with actual borrowed sources |
| `FormulaEase` | Original selected canonical Ease declaration refusal |

Input admission remains distinct from name checking. This namespace supplies implemented input
metadata; Profile/Material/geometry/size/tolerance values, recipe statement checking, graph execution,
physical claims and readiness retain their existing owners. Human labels never become formula names.

## Implemented input composition acceptance

The focused composition review checks actual selected declarations/origins/canonical pointers,
incoherent table/Ease/SizeSet scope, and held Piece references after deletion. Source objects and
private/shared access remain unchanged from the namespace implementation's native/privacy checks.
Input ownership and source currentness do not imply that topology is ready: a deleted held edge
remains inspectable with its original unresolved point and uncovered interval evidence. Construction
never repairs, registers or reattaches it implicitly. Point/whole-range readiness uses the separate structural report.

This closes the implemented source/input component scope. Full Design assembly still needs current
sewing/garment sources and readiness; runtime commands, geometry, physical validation, storage,
integration and release retain their task owners. D154's reference ordinal and D250's baseline
vocabulary repairs remain open under their existing owners; this review closes neither.

## Actual sewing amount declarations

DesignInputs.sewing_amount_declarations(&graph) rechecks the whole current root, then returns one
borrowed DesignSewingAmount per actual span, in authored order. Each retains the actual span and
canonical owner. This checks amount metadata only; graph copy/range/stop and global identity joins
are separate, as are numeric evaluation, physical ease and release readiness.

An explicit signed amount borrows the original Length from the span. Formula(id) requires actual
DesignParameter metadata with that id and its current canonical binding, then requires Length kind.
For example, parameter5001 may target dedicated length10; Formula(5001) retains both identities and
borrows that exact record. Formula(10) refuses when10 is only a canonical declaration. A general
scalar Length target is also supported after its existing identity/kind adapter succeeds. Unknown/
Derived states and sources stay unchanged; no formula value or replacement scalar is generated.

Profile(ref) stays a logical target-profile role. profile_validation() returns Some(DeferredToG4)
for this branch; canonical length10 cannot resolve profile reference10 merely because numbers match.
Other branches return None from that accessor, without asserting numeric or physical completeness.
The source view preserves the exact explicit/formula/profile distinction.

| API or variant | Meaning |
| --- | --- |
| `DesignSewingAmount` | Actual span/canonical context and borrowed amount metadata |
| `DesignSewingAmountSource` | Explicit signed length, actual formula parameter or deferred profile ref |
| `DesignSewingDeclarationError` | Current Inputs cause, missing parameter or FormulaKind mismatch |
| `MissingFormulaParameter` | Formula identity has no actual Design parameter metadata |

A Count parameter refuses with FormulaKind and its actual kind, span and parameter identity. A late
failure returns no accepted prefix of amount views. Current root errors retain concrete error chains.
Views cannot outlive the actual graph/root objects, and never rewrite spans or allocations. Use the
separate current SewingGraph validator for structural sources and DesignSewing below for owned joins. Profile declaration grammar/type/state/physical resolution remains G4.

## Private sewing/input ownership

DesignSewing::new(DesignSewingDefinition { inputs, graph, landmarks }) owns the exact implemented
DesignInputs root, original SewingGraph and complete semantic SewingLandmark inventory. Private
storage and shared definition()/inputs()/graph()/landmarks() getters provide no mutation path.
The root remains the only canonical owner; no borrowed context or replacement graph is stored.

Before current root validation, the complete ownership census visits original input positions,
graph identity, landmarks and spans in authored order. Graph5000 cannot alias the containing Design;
span5001 cannot alias parameter5001; landmark90000 cannot alias a retired creating operation90000.
DuplicateIdentity retains both actual DesignSewingPosition values. Referenced copy/parameter/anchor ids
remain uses and never become extra owners. No label supplies identity or ownership.

| API | Meaning |
| --- | --- |
| `DesignSewingDefinition` | Exact authored input root, graph and landmark objects |
| `DesignSewing` | Private immutable owner of those implemented components |
| `DesignSewingPosition` | Original input, graph, landmark or span ownership position |
| `DesignSewingError` | Complete ownership or original current-root cause |

Ownership admission preserves held sewing references for inspection and explicit repair. For example,
replacing copy96000 with96010 leaves the original graph's96000 reference inspectable; the separate
current graph validator returns MissingCopy. Structural Piece topology may still be ready in its own
scope. Nothing reassigns the old seam or reconstructs born-valid graph/landmark objects against a
later ledger. Current sewing/source queries are separate from ownership; construction
alone supplies no sewing, numeric, Profile, geometry, execution or release readiness certificate.

## Current sources over the owned sewing state

DesignSewing.validate_current() checks complete owned ids/current root, then every landmark's actual
current Piece and anchor, the original graph against current root copies/Pieces/landmarks/ledger,
and every actual amount declaration. amount_declarations() performs the same checks before returning
borrowed views over those exact objects. No partial declaration collection escapes a late failure.

Unused landmarks remain owned obligations: deleting material under an unselected turn point returns
its original AnchorError; removing its Piece returns MissingLandmarkPiece before graph lookup.
A renamed copy returns the original SewingError even if pure amount metadata still validates. A Count
amount parameter returns its original DesignSewingDeclarationError after graph validation. Landmark,
graph and amount wrappers expose the concrete original error through std::error::Error::source().

| Variant | Meaning |
| --- | --- |
| `MissingLandmarkPiece` | Actual owned landmark's held source Piece is absent |

These queries reuse the existing raw current anchor/graph choice and repair rules. Stored data,
registrations, ranges and allocations stay unchanged. Profile roles remain visibly DeferredToG4 in
returned amount views; current source success does not resolve numeric values or physical profiles.
Whole Piece topology readiness remains the separate structural report. Geometry, full Design
execution, assembly completeness and release remain separately owned obligations.
