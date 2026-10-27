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

Direct trait binding checks only its selected declaration/context; DesignMetadata adds whole-scope
validation/canonical membership without storing a context.

```bash
cargo test -p sc-measure --test measurement_inputs_contract
```

## Owned common SizeSet metadata

SizeSetMetadata owns membership, axes/member points, required dimensions/available breaks and Design
table id. Chart families reuse it; values/measurement/Ease metadata keep their original owners.
Empty break drafts do not establish coverage; full Design composition remains separate.

new(definition, &canonical, &inputs) checks canonical/measurement/Ease ids, then authored set/member/
axis/value/break ids, including unselected tables. References stay uses. Actual current input/context/
axes/break validators require the exact Design table; equal-content peers cannot replace missing ids.
Replacement canonical inputs are rechecked.

with_context validates whole scope before a temporary SizeBreakContext callback. Only owned results
may escape; no self-reference/value/state cache is stored. Original Unknown/Derived declarations stay
borrowed. Failure skips the callback with original cause; break errors expose concrete SizeBreakError
(including MissingDelta), not its allocation wrapper (D255).

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

DesignInputs owns structure, parameters, exact recipe document, Materials and checked metadata host;
structure alone owns CanonicalInputs. Provided refs are checked below, selected names explicitly.
Sewing/garment currentness and readiness remain separate.

Ownership visits structure (Design/canonical/Pieces/copies/operations), parameters, Materials, then
higher metadata. Retired creators still own ids; evidence/formula/material refs remain uses.
DuplicateIdentity retains both original DesignInputsPositions.

Then actual parameter targets, all Material properties/Piece assignments and sources are rechecked
against this canonical owner; past standalone success cannot substitute. Unresolved assignments/
uncertainty and held Piece repairs survive without reconstruction against edited history. Recipe
text/canonical syntax stays exact; unbound names need later binding/checking.

| API or variant | Meaning |
| --- | --- |
| `DesignInputs`, `DesignInputsDefinition` | Private implemented-component owner and exact construction input |
| `DesignInputsPosition` | Structure, Parameter, Material or Source original ownership position |
| `DesignInputsError` | DuplicateIdentity or original Parameters, Materials, Sources, References typed cause |

Shared getters/identities() expose actual ownership; validate_current() repeats scoped checks.
id()/revision() preserve the structure's identity/counter. Debug omits customer content; composition
stores no borrowed context:

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

DesignInputs.sewing_amount_declarations(&graph) validates the whole current root, then borrows
DesignSewingAmount per actual authored span/canonical owner. Metadata checks supply no graph copy/
range/stop, global id, numeric, physical ease or release proof.

Explicit signed amounts borrow the span's Length. Formula(id) requires actual DesignParameter/
current Length binding: parameter5001 may target dedicated length10; Formula(5001) retains both
identities/the exact record, while bare Formula(10) refuses. General Length works through its existing
identity/kind adapter. Unknown/Derived source/state stays unchanged, without values/replacement scalars.

Profile(ref) stays logical with profile_validation() Some(DeferredToG4); length10 cannot resolve
Profile10 by numeric coincidence. Other branches return None without numeric/physical completeness.
Explicit/formula/profile origins remain distinct.

| API or variant | Meaning |
| --- | --- |
| `DesignSewingAmount` | Actual span/canonical context and borrowed amount metadata |
| `DesignSewingAmountSource` | Explicit signed length, actual formula parameter or deferred profile ref |
| `DesignSewingDeclarationError` | Current Inputs cause, missing parameter or FormulaKind mismatch |
| `MissingFormulaParameter` | Formula identity has no actual Design parameter metadata |

Count returns FormulaKind with actual kind/span/parameter. Late failures return no prefix; current
root errors retain concrete causes. Views borrow graph/root lifetimes without rewriting spans/
allocations. SewingGraph checks sources; DesignSewing below checks owned joins. Profile grammar/type/
state/physical resolution remains G4.

## Private sewing/input ownership

DesignSewing::new(DesignSewingDefinition { inputs, graph, landmarks }) owns exact DesignInputs,
original SewingGraph and complete SewingLandmarks. Private storage/shared definition()/inputs()/
graph()/landmarks() provide no mutator. The root alone owns canonical inputs; no context/replacement
graph is stored.

Before root validation, ownership visits original input positions, graph, landmarks and authored
spans. Graph5000 cannot alias Design, span5001 parameter5001, or landmark90000 retired operation90000.
DuplicateIdentity retains both DesignSewingPositions. Copy/parameter/anchor references are uses;
labels provide no identity/ownership.

| API | Meaning |
| --- | --- |
| `DesignSewingDefinition` | Exact authored input root, graph and landmark objects |
| `DesignSewing` | Private immutable owner of those implemented components |
| `DesignSewingPosition` | Original input, graph, landmark or span ownership position |
| `DesignSewingError` | Complete ownership or original current-root cause |

Admission retains held references for inspection/explicit repair. Replacing copy96000 with96010
leaves96000 inspectable; current graph validation returns MissingCopy. Piece topology may remain ready
in its own scope. No seam is reassigned or born-valid graph/landmark reconstructed against later
history. Ownership grants no current sewing, numeric, Profile, geometric, execution or release proof.

## Current sources over the owned sewing state

DesignSewing.validate_current() checks owned ids/current root, every landmark Piece/anchor, original
graph against actual copies/Pieces/landmarks/ledger, then all amounts. amount_declarations() applies
the same checks before borrowing exact objects, without partial views after late failure.

Unused landmark loss retains AnchorError; absent Piece returns MissingLandmarkPiece before graph
lookup. Renamed copies retain SewingError despite healthy amount metadata; Count amounts return
DesignSewingDeclarationError after graph validation. Wrappers expose original concrete causes through
`Error::source()`.

| Variant | Meaning |
| --- | --- |
| `MissingLandmarkPiece` | Actual owned landmark's held source Piece is absent |

Existing raw anchor/graph choice/repair rules preserve stored data, registrations, ranges and
allocations. Profile amounts stay DeferredToG4; source success resolves no numbers/profiles. Whole
Piece topology has its separate report; geometry, full Design execution, assembly and release remain
separately owned.

Focused checks cover owner aliases, unused landmark repairs and amount/Profile borrows;
numeric/geometric/runtime/release proof remains separate.

## Joined Gather declarations

```rust,ignore
let views = sewing.gather_declarations(&constructions)?;
```

Whole-current sewing root/landmarks/graph/amounts precede all supplied construction sources, even
without Gathers. Authored views borrow actual Gather/position/side/span DeclaredEase, existing
DesignSewingAmount and canonical owner. closing_operation() stays logical despite numeric-id matches;
Debug shows side only.

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

This owns exact DesignSewing and Pocket/NotionPlacement/Closure/Layer/Hem/Construction inventories.
Shared getters preserve actual immutable objects/sole canonical owner. identities() visits sewing,
Pockets, notions, Closures/instances, layers, Hems, constructions in authored/family order. Closure instances and retired root creators
own ids; Piece/copy/parameter/Profile/operation uses never create owners. DuplicateIdentity returns
first/repeated locations before current root-input validation; Inputs retains its original error chain.

Admission keeps empty drafts, damage and modelled Lining without rebuilding objects. Fly refuses
env_fly/G7 in Closure::new before an object exists. Debug shows counts; admission proves no source/
declaration joins, execution scope, geometry, resolved physical values, completeness or release.

| API token | Meaning |
| --- | --- |
| `DesignGarmentDefinition` | Exact seven-component owned input |
| `DesignGarment` | Private garment owner over one sewing/input root |
| `DesignGarmentPosition` | Original sewing/family/nested-instance ownership location |
| `DesignGarmentError` | Ownership/input or original sewing/garment family cause |

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
