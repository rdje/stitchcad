# Owned Design inputs and sources

`DesignMetadata<H>` owns a DesignMetadataHost and its identity census; CanonicalInputs owns values.
MeasurementInputs and DesignSources implement this core port. Full Design assembly
follows; admission selects no table, SizeSet or Factory Profile.

## Admission and current reads

`DesignMetadata::new(host, &canonical)` checks stable census, duplicate metadata ids and both
canonical families' collisions, then the host's actual current validator. Empty hosts are legal;
input/evidence references stay uses. Collision witnesses retain family/authored positions without
deduplicating records.

`measurement_declaration(measurement, &canonical)` rechecks whole scope, requires actual Measurement
metadata and resolves its explicit host binding to the supplied inventory's actual dedicated Length.
Detached equal copies/bare canonical ids cannot supply metadata/bindings. Unknown/Derived stays
unchanged; replacement inputs are rechecked. The borrow retains host/canonical context.

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

Hosts supply complete immutable snapshots/semantic validation. Core checks observable ids/object
membership, not arbitrary provider semantics. No self-reference/value/state cache is stored;
host()/entities() are shared, Debug exposes entity count only.

## Immutable source snapshots

DesignMetadata owns H through a private Arc. Its manual Clone shares the exact source instance and
copies only the captured entity vector; it does not require or invoke H.clone. Model reconstruction
can preserve honestly charged source/cache content even when an independent semantic-equal H.clone
uses a different cache representation. Each logical snapshot still charges the full source content
and its own census. Shared access must preserve source immutability.

```rust
let snapshot = metadata.clone();
assert!(core::ptr::eq(metadata.host(), snapshot.host()));
```

host() remains a readonly borrowed &H accessor, now an ordinary method rather than const fn.
No Arc handle, mutable source or replacement method is exposed. Cross-thread sharing requires
H:Send+Sync; single-thread hosts remain supported. See [retained publication](command-retained-publication.md#immutable-source-ownership)
for the D293 cache regression and exact publication-bound examples.

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

Other Design ids, SizeSets, formula composition and readiness have separate owners. Profile ids/
labels/result claims resolve no physical field despite numeric-id matches; [G4 boundaries](material-inputs.md)
apply. Native/privacy checks cover this port; full Design execution/integration remains unverified.
Use the local producer profile:

```bash
cargo test -p sc-core --test design_metadata_contract
```

## Actual measurement host

MeasurementInputs lists owned landmarks/procedures/measurements/tables/Ease mappings/sets in family/
authored order, excluding canonical/evidence uses. Whole current checks include unused mappings and
compression permission. Bindings use actual metadata/borrowed record context; general scalars cannot
replace dedicated lengths.

| API or variant | Meaning |
| --- | --- |
| `MeasurementHostError` | Original typed sc-measure port refusal |
| `Inputs`, `RecordContext`, `Measurement` | Whole inventory, target context or explicit binding cause |
| `MissingMeasurement` | Actual measurement metadata is absent |

Direct trait binding checks only its selected declaration/context; DesignMetadata adds whole-scope
validation/canonical membership without storing a context.

```bash
cargo test -p sc-measure --test measurement_inputs_contract
```

## Owned common SizeSet metadata

[Held resource accounting](source-resource-accounting.md#actual-common-sizeset-metadata) counts
actual member/axis/point/break vectors and labels/tokens without rebuilding contexts or reading values.

SizeSetMetadata owns members, axes/member points, required dimensions/available breaks and Design
table id, reused by chart families. Values/measurement/Ease retain their owners; empty break drafts
prove no coverage or full Design composition.

new(definition, &canonical, &inputs) checks canonical/measurement/Ease ids, then authored set/member/
axis/value/break ids, including unselected tables. References stay uses. Actual current input/context/
axes/break validators require the exact Design table; equal-content peers cannot replace missing ids.
Replacement canonical inputs are rechecked.

with_context checks whole scope before temporary SizeBreakContext: only owned results escape,
Unknown/Derived stay borrowed, with no self-reference/value/state cache. Failure skips the callback,
retaining original concrete SizeBreakError (including MissingDelta), unboxed as source (D255).

| API or variant | Meaning |
| --- | --- |
| `SizeSetMetadata`, `SizeSetMetadataDefinition` | Private common owner and explicit editable construction input |
| `SizeSetMetadataPosition` | Original ownership witness: Inputs, Set, Member, Axis, AxisValue, Break |
| `SizeSetMetadataError` | DuplicateIdentity, Inputs, MissingDesignTable, RecordContext, TableContext, Sources, BreakContext or Breaks |

identities() lists common owned ids, definition() is shared, Debug shows counts. validate_current
checks structure; chart/grade coverage, evaluation and readiness stay separate. A required waist
break without a delta may be saved, but require_grade_coverage refuses the missing cell without zero
substitution.

```bash
cargo test -p sc-measure --test size_set_metadata_contract
```

## Owned garment SizeSet sources

[Retained chart resource accounting](source-resource-accounting.md#actual-retained-garment-and-mtm-charts)
includes unused observations, saved chart bindings and MTM's owned Ease snapshot without value reads.

GarmentSizeSet owns common metadata, observations and chart. Admission checks source ids/common
sources/all observations/composite; missing unused observation tables refuse. Design references this
external object without copying values or storing contexts.

with_inputs revalidates before temporary GarmentSizeSetInputs. Shared chart_context()/break_context()
supply chart.declaration(member, pom, view.chart_context()) and break.delta(view.break_context()) with
original canonical data. Only owned results escape the callback.

| API or variant | Meaning |
| --- | --- |
| `GarmentSizeSet`, `GarmentSizeSetDefinition` | Private garment source owner and exact construction input |
| `GarmentSizeSetPosition` | Common, Observation or Chart original ownership witness |
| `GarmentSizeSetError` | DuplicateIdentity, Metadata, Context, Observation, CollectionContext or Inputs cause |

Partial chart/break drafts remain valid; validate_inputs_for requires Regeneration/GradeRules
coverage without computing geometry/unknown quantities. reference() retains set/revision, identities()
owned ids, Debug observation count. MTM/unified hosts follow; full Design/execution remains later.

```bash
cargo test -p sc-measure --test garment_size_set_contract
```

## Owned MTM SizeSet sources

MtmSizeSet owns singleton common metadata/MtmChart; MeasurementInputs keeps body/Ease/table/set
ownership. Admission checks input/common/chart ids through original validators and temporary actual
membership/record/Design-table contexts. No body-plus-Ease value, profile selection or cache is created.

with_inputs revalidates before temporary MtmSizeSetInputs. Shared chart_context()/break_context()
allow quantity/coverage queries; chart.body_declaration(pom, context)/ease_declaration(pom, context)
borrow separate canonical records. Only owned results escape; missing replacement sources skip callbacks.

| API or variant | Meaning |
| --- | --- |
| `MtmSizeSet`, `MtmSizeSetDefinition` | Private external MTM source owner and exact construction input |
| `MtmSizeSetPosition` | Common or Chart ownership witness |
| `MtmSizeSetError` | DuplicateIdentity, Metadata, Eases, Context or Inputs original cause |

Partial mappings stay drafts. Regeneration needs coverage; GradeRules retains GradeRulesUnsupported
chart/member refusal. SizeSetInputError exposes concrete chart/MTM/break causes rather than boxes
(D256). No numeric execution/readiness follows.

```bash
cargo test -p sc-measure --test mtm_size_set_contract
```

## Unified external SizeSet inventory

[Complete retained set resources](source-resource-accounting.md#actual-external-family-and-inventory-composition)
compose actual family metadata and charts in authored order, preserving explicit Garment/MTM provenance.

SizeSet retains Garment/Mtm family (also singleton garment provenance), lends SizeSetInputs and
preserves actual family causes.

SizeSetInventory owns authored order and checks shared/complete set ids before current validators.
Distinct sets cannot reuse owned ids; shared canonical refs stay uses. All sets, including unselected,
validate; explicit empty inventories are legal.

size_set(id) inspects shared content; validate_reference(expected, &canonical, &inputs) rechecks all
sets and borrows exact id/revision/current inputs. Labels/latest revisions give no fallback: an
unselected garment's lost required axis blocks a valid selected MTM set.

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

[Readonly resource accounting](source-resource-accounting.md#complete-immutable-source-host-port)
implements the required core DesignResourceSource port over both actual retained roots, without current queries.

DesignSourcesDefinition supplies owned MeasurementInputs/SizeSetInventory to DesignSources, without
stored canonical values/contexts. new(definition, &canonical) repeats complete inventory/current
checks; DesignSourcesError retains typed SizeSets/Measurement causes. definition()/measurements()/
size_sets() are shared, without mutators.

The catalog lists measurement/Ease then each set's common/chart sources, excluding canonical/evidence
uses. Category indices count authored host records; enum order/numeric type selects no role.
DesignMetadata checks whole-host currentness/object membership before lending canonical measurement
records; direct binding stays targeted. Replacement inputs recheck unselected sources.

DesignSizeSetHost::validate_size_set requires current sources/exact external id/revision, never bare
numeric ids. This readonly facet proves no path coverage, Profile values or full Design integration;
physical Profile resolution stays G4 despite numeric-id matches.

| API | Meaning |
| --- | --- |
| `DesignSources`, `DesignSourcesDefinition` | Private actual combined owner and exact editable construction input |
| `DesignSourcesError` | Original SizeSets or Measurement cause through core ports |

```bash
cargo test -p sc-measure --test design_sources_contract
```

## Owned core input composition

DesignInputs owns structure, parameters, exact recipe, Materials and checked metadata host; only
structure owns CanonicalInputs. Provided refs/selected names need explicit checks below;
sewing/garment currentness/readiness remain separate.

Ownership visits structure (Design/canonical/Pieces/copies/operations), parameters, Materials, then
higher metadata. Retired creators still own ids; evidence/formula/material refs remain uses.
DuplicateIdentity retains both original DesignInputsPositions.

Current parameter targets, all Material properties/Piece assignments/sources are rechecked against
this canonical owner; past checks cannot substitute. Unresolved assignments/uncertainty/held Piece
repairs survive edited history without reconstruction. Recipe text/canonical syntax stays exact;
unbound names need binding/checking.

| API or variant | Meaning |
| --- | --- |
| `DesignInputs`, `DesignInputsDefinition` | Private implemented-component owner and exact construction input |
| `DesignInputsPosition` | Structure, Parameter, Material or Source original ownership position |
| `DesignInputsError` | DuplicateIdentity or original Parameters, Materials, Sources, References typed cause |

Shared getters/identities() expose owners; validate_current() repeats checks. id()/revision() retain
structure identity/counter. Debug omits customer content; no context is stored:

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

Composition checks cover selected declarations/origins/canonical pointers, incoherent table/Ease/
SizeSet scope and held Piece references after deletion; namespace native/privacy checks retain original
objects/shared access. A deleted held edge keeps its unresolved point/uncovered intervals: ownership/
current inputs grant no topology readiness or implicit repair/registration/reattachment. Point/whole-range
readiness uses the separate structural report.

This closes source/input components; full Design sewing/garment readiness, runtime, geometry,
physical validation, storage, integration/release and owned D154 reference-ordinal/D250 vocabulary
repairs remain open.

## Actual sewing amount declarations

DesignInputs.sewing_amount_declarations(&graph) checks all current inputs before lending each
authored span's DesignSewingAmount over the actual canonical owner. Views borrow graph/root;
no span or allocation is rewritten. Retained views also borrow the exact portion mapping;source
metadata denotes the full original declaration. Copy/range/stop validity,global ids,numeric/physical ease
and release are separate checks.

Explicit signed amounts borrow the span's Length. Formula(id) needs actual DesignParameter metadata
bound to current Length: parameter5001 can target dedicated length10; Formula(5001) keeps both ids
and the exact record, while bare Formula(10) refuses. General Length uses its existing identity/kind
adapter. Unknown/Derived sources and states stay unchanged; no value/replacement scalar is created.

Profile(ref) remains logical: profile_validation() returns Some(DeferredToG4), even for Profile10
beside length10. Other origins return None without proving numeric/physical completeness. Profile
grammar, type, state and physical resolution stay G4; explicit/formula/profile origins stay distinct.

| API or variant | Meaning |
| --- | --- |
| `DesignSewingAmount` | Actual span/canonical borrow and amount metadata |
| `DesignSewingAmountSource` | Explicit signed Length, formula metadata or logical Profile |
| `DesignSewingDeclarationError` | Inputs cause, missing parameter or FormulaKind mismatch |
| `MissingFormulaParameter` | No actual Design parameter metadata for the formula id |

Count returns FormulaKind with actual kind/span/parameter; late errors return no prefix and retain
original root causes. SewingGraph checks sources; DesignSewing below checks owned joins.

## Private sewing/input ownership

DesignSewing::new(DesignSewingDefinition { inputs, graph, landmarks }) owns the exact input root,
graph and complete landmarks. Shared definition()/inputs()/graph()/landmarks() expose no mutator.
Only the root owns canonical inputs; no context or replacement graph is stored.

Before root validation,the identity census visits inputs,graph,landmarks,authored spans,then unique
retained original sources. Their identities remain owned even when the original span is retired.
Graph5000 cannot alias Design, span5001 parameter5001, or landmark90000 retired operation90000.
DuplicateIdentity keeps both DesignSewingPositions. Copy/parameter/anchor refs remain uses; labels
create no identities or ownership.

| API | Meaning |
| --- | --- |
| `DesignSewingDefinition` | Exact authored root, graph and landmarks |
| `DesignSewing` | Private immutable component owner |
| `DesignSewingPosition` | Original input/graph/landmark/span/source location |
| `DesignSewingError` | Original ownership or current-root cause |

Admission preserves held references for inspection/explicit repair. Replacing copy96000 with96010
leaves96000 inspectable but current graph validation returns MissingCopy, even if Piece topology is
ready. No seam is reassigned or graph/landmark rebuilt against later history. Ownership certifies
no current sewing, numeric/Profile/geometric/execution/release result.

## Current sources over the owned sewing state

DesignSewing.validate_current() checks owned ids/current root, every landmark Piece/anchor, original
graph against actual copies/Pieces/landmarks/ledger, then all amounts. amount_declarations() repeats
this preflight before borrowing objects; late failures expose no partial views.

Unused landmarks retain AnchorError; absent Piece yields MissingLandmarkPiece before graph lookup.
Renamed copies retain SewingError despite healthy amounts. After graph validation, Count retains
DesignSewingDeclarationError. `Error::source()` exposes original concrete causes.

| Variant | Meaning |
| --- | --- |
| `MissingLandmarkPiece` | An owned landmark's held Piece is absent |

Raw anchor/graph choices and repairs preserve data, registrations, ranges and allocations. Source
success resolves no numbers or Profiles; amounts remain DeferredToG4 where applicable. Whole-Piece
reports, geometry, full Design execution/assembly/release remain separate. Focused checks cover
owner aliases, unused landmark repairs and amount/Profile borrows.

## Joined Gather declarations

```rust,ignore
let views = sewing.gather_declarations(&constructions)?;
```

Whole-current sewing root/landmarks/graph/amounts precede all construction sources, even without
Gathers. Views borrow authored Gather/position/side/span DeclaredEase, DesignSewingAmount and canonical
owner. closing_operation() stays logical despite numeric-id matches; Debug shows side only.

A-minus-B remains signed: +5 mm permits gathering A; −3 mm permits B. No negated amount/allocation
copy, fallback or resolved sign is produced. Derived/formula and Profile origins remain unread/
DeferredToG4. A later same-Piece copy retarget refuses after an earlier healthy Gather, without partial
views; unused bad landmarks/amounts/constructions also block empty scope. Original concrete cause
chains remain. Operation checks, global garment/root ids, intake conservation and physical/release
proof remain separate. Missing exact span metadata refuses instead of substituting or omitting a view.

| API token | Meaning |
| --- | --- |
| `DesignGatherDeclaration` | Actual Gather/span/side/amount/closing intent over borrowed current contexts |
| `DesignGatherDeclarationError` | Original Sewing/Constructions cause or missing exact span amount |

## Private garment ownership

```rust,ignore
let garment = DesignGarment::new(DesignGarmentDefinition {
    sewing, pockets, notions, closures, layers, hems, constructions,
})?;
```

This privately owns exact DesignSewing and Pocket/NotionPlacement/Closure/Layer/Hem/Construction
inventories; shared getters retain original objects/sole canonical owner. identities() visits sewing,
Pockets, notions, Closures/instances, layers, Hems, constructions in family/authored order. Instances/
retired creators own ids; Piece/copy/parameter/Profile/operation references stay uses. DuplicateIdentity
retains first/repeated locations before root-input checks; Inputs keeps its original cause chain.

Admission retains drafts/damage/modelled Lining unchanged. Fly refuses env_fly/G7 in Closure::new
before birth. Debug shows counts; admission certifies no source/declaration joins, execution scope,
geometry, resolved physical values, completeness or release.

| API token | Meaning |
| --- | --- |
| `DesignGarmentDefinition` | Exact seven-component owned input |
| `DesignGarment` | Private garment owner over one sewing/input root |
| `DesignGarmentPosition` | Original family/nested owner location |
| `DesignGarmentError` | Original ownership/input/family cause |

### Complete current garment sources

```rust,ignore
garment.validate_current()?;
```

Checks visit ownership/inputs, sewing, Pockets, notions, Closures, layers/Materials, Hems, constructions,
including complete empty/unused context. Errors retain locations/raw repairs/concrete standard causes
without partial success, retargeting or cache. Parameter ids cannot supply missing Materials; layer
sources precede assignment checks. Later Hem damage refuses after earlier success. Unknown/unresolved
content stays held; Lining sources may pass while execution refuses. Declaration roles, envelope,
geometry, resolved physical values and release remain separate.

### Complete garment declaration views

```rust,ignore
let views = garment.declarations()?;
for view in views {
    let canonical = view.canonical_inputs(); // sole root
    let original_role = view.source();       // original adapter
}
```

Whole-current garment validation precedes even empty views. Order: sewing amounts, Pocket openings,
Closure fields, paired layer Material/offset, Hem depth/fold, Dart/Tuck/Pleat intake/closing, joined
Gathers, grain/allowance roles. Adapters borrow original objects/positions/roles/canonical owner;
notions supply placement sources, Closures hardware sizes. Each layer pairs its exact assigned Material
or unresolved reason with offset intent; missing adapters refuse truncation. Debug shows origins only.

Late failures keep concrete causes, no prefix: allowance Formula(2000) refuses bare Count2000;
current sewing damage wins first. Unknown/Derived/logical ids/G3/G4 deferrals stay unchanged/unread.
Domain/operation resolution, execution scope, geometry/conservation/physical/release remain separate.

| API token | Meaning |
| --- | --- |
| `DesignGarmentDeclaration` | Original role/canonical borrow |
| `DesignGarmentDeclarationSource` | Typed family; paired layer views |
| `DesignGarmentDeclarationError` | Current/metadata cause or missing layer pair |

### Requested explicit execution scope

```rust,ignore
garment.require_in_scope()?; // original Closure guards, then distinct layer families
garment.declarations()?;    // complete current sources/metadata; separate obligations
```

Original [layer](ontology-constructions.md#owned-layer-families-and-complete-current-sources) and
[Closure](ontology-closures.md) guards retain named causes/context despite damage; Fly refuses
before birth. Scope certifies no currentness (even empty scope/host drift), domains/operations,
geometry/physical support/completeness/release. Inspection remains available.

| API token | Meaning |
| --- | --- |
| `DesignGarmentEnvelopeError` | Original Closure/layer envelope cause and authored context |

Garment17 native/3 privacy-lifetime and aggregate2 native/12 privacy-lifetime pass. Apply the
[whole-Piece report](ontology-implementation.md): unused interior loss can outlive healthy garment
sources/live points. Runtime/physical/release proof remains pending.
