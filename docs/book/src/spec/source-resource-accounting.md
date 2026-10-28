# Actual source resource accounting

These producers charge actual retained source objects through the
[logical resource contract](command-operation-control.md#logical-dynamic-content-accounting-contract).
[Kernel and structural root accounting](model-resource-accounting.md) composes separately.

The actual source families retain common metadata, observations and charts. Standalone resolution
intents hold transformations/evidence/label mappings outside these source owners. Their finite input
admission is G4-PROFILES.1.resource-admission, required before future profile exposure; current model
accounting supplies no certificate for that separate input.

## Actual measurement and Ease sources

sc-measure's charge_measurement_input_resources borrows actual MeasurementInputs. It precharges
Landmark, procedure, Measurement and table vectors, each table's entry vector, then Ease and EaseSet
vectors and each set's entry vector. Schema/member order stays unchanged. It charges every exact
name/documentation/token string, including each saved MeasurementBinding's owned token in tables,
Ease body/garment bindings, and EaseSet entries. Matching spellings never merge distinct owned fields.

Fixed domains, entered units, source/declaration/procedure ids, fit/compression and the inline pair
of landmark ids add no dynamic entries. Canonical value/state/source/evidence remain owned by core;
they are neither charged twice nor queried here. Exhaustive private owner/definition/binding views
and Copy constraints cover fixed fields. No context construction, semantic/provenance validation,
unit reconversion, Ease evaluation, physical fit or standards conformity follows from these counts.

```rust
let mut meter = DesignResourceMeter::new(resource_limits);
sc_measure::charge_measurement_input_resources(&measurement_inputs, &mut meter)?;
let represented_measurement_sources = meter.finish()?;
```

The native fixture has2 landmarks,2 procedures,2 Measurements,1 table/2 entries,1 Ease and1 set/1
entry:12 entries and30 UTF-8 bytes, zero counter bits. Names/documents retain `é`, `猫` and ` é `;
body token `b` and garment token `gg` count in each owned binding, plus set token `e`. Procedure
documentation preserves surrounding spaces. Both Measurement landmark pairs stay inline references.

Maximum8 entries refuses measurements.tables.entries at7+2 with the actual table/index0; maximum11
refuses measurements.ease_sets.entries at11+1 with the actual set/index0. Every other collection
precharge also has a focused refusal case. Text maxima9/18/22/25/29 retain documentation6+4,
Measurement token17+2, table entry token21+2, Ease garment token24+2 and set garment token28+2.
Whole member text keeps its original owner/member index; entry text uses the actual table/set and
original entry index. No private name/document/token text is copied into the error location.

Six actual cases pass with strict selected lint/formatting: exact demand, vector/text witnesses,
unchanged source/canonical data, prior prefix14/32, empty0/0/0 and original prior refusal. Replacing
the canonical context with an empty draft makes current validation refuse; accounting held source
metadata still succeeds unchanged without rebinding. This is the measurement/Ease component only.
Higher-root composition and runtime admission remain owned; these counts supply no source-truth, physical, whole-host/model or allocator time/memory proof.

## Actual common SizeSet metadata

charge_size_set_metadata_resources charges actual common membership, axes and breaks without
rebuilding their current contexts. It precharges members; axes and each value catalog; points and
each coordinate vector; required break dimensions; then available break records. Exact member/axis/
value labels and saved dimension-binding tokens count per owned field. Base member, system, Design
table, numeric source/delta references and fixed Count revisions add no unbounded counter bits.

Exhaustive private owner/definition/label views and Copy guards cover fixed fields. Label, Length and
Scalar axis sources retain their authored form; an unknown numeric source remains unread. No label
normalization, revision advance, coordinate resolution, Cartesian grid, adjacent-cell expansion,
missing-delta synthesis, coverage or grading execution occurs. A point has a member reference rather
than its own identity: coordinate refusals keep the actual Set owner and original point index.

```rust
let mut meter = DesignResourceMeter::new(resource_limits);
sc_measure::charge_size_set_metadata_resources(&common_metadata, &mut meter)?;
let represented_common_sources = meter.finish()?;
```

The native fixture has2 members,3 axes/6 values,2 points/6 coordinates,1 required dimension and1
available break:21 entries. Member labels ` é ` and `猫` retain7 UTF-8 bytes; axis names `a`, `é`, `猫`
retain6, label coordinates `S`/`é` retain3, and the required/available dimension tokens `gg` each add2:
20 bytes total. Dedicated Length and general Scalar coordinates remain references. Unused catalog
values still count; a reversed coordinate order and explicit second-member base stay unchanged.
The terminal Count revision remains at its bound and contributes zero unbounded magnitude bits.

Entry maxima1/4/6/8/10/12/15/18/19/20 retain original refusals0+2/2+3/5+2/7+2/9+2/11+2/13+3/16+3/
19+1/20+1. Nested value vectors retain actual axis/index; coordinates retain Set/point index. Text
refusals preserve actual member/value/axis/Set/break ownership, including member4+3, value9+2,
axis13+3, required dimension16+2 and available break18+2. No private label/token is copied into locations.

With no available breaks, the same actual draft retains its required dimension:20 entries/18 bytes,
without manufacturing a delta. Six new cases plus prior6 pass:12 source resource cases, with strict
selected lint/formatting after correcting the new test's missing trait import. Prior2 entries/2 bytes
compose to23/22; first errors remain original. An empty replacement canonical context refuses current
validation, while held accounting preserves exact metadata/canonical/measurement objects. Higher-root composition and runtime admission remain owned; these counts grant no currentness, grading/physical, whole-host/model or allocator memory/time proof.

## Actual retained garment and MTM charts

charge_garment_size_set_chart_resources takes an actual GarmentSizeSet and charges its complete
observation registry, including unused observations, then the chart's POM and saved-observation
binding vectors. Every original observation retains two owned MeasurementBinding token fields;
every saved chart binding owns another two. Matching spelling does not merge these fields.
Whole registry refusals use the actual Set owner; observations use their own ids, while saved chart
binding refusals use the owning chart id and original entry index, not the referenced observation id.

charge_mtm_size_set_chart_resources takes an actual MtmSizeSet and charges the chart's saved
EaseSetDefinition entry vector. Each saved entry owns its set token and body/garment tokens, even
though the saved set id references an authoritative Ease set elsewhere. The snapshot is retained
owned data, without copied numeric values or state. Count its vector/text here and the authoritative
owner through its own producer. No authoritative lookup or body-plus-Ease result is computed.

Both producers explicitly exclude common SizeSet metadata; compose that actual owner separately.
Exhaustive private family/chart/observation/definition/binding views and individual Copy constraints
cover fixed membership/member/table/provenance/source fields without current-context reconstruction,
canonical reads, completeness or instantiation checks. Standalone profile intents remain separate
inputs under G4-PROFILES.1.resource-admission; the actual source-family field sets are2/3/2 for
DesignSourcesDefinition/GarmentSizeSetDefinition/MtmSizeSetDefinition, with no resolution-intent field.

```rust
let mut meter = DesignResourceMeter::new(resource_limits);
sc_measure::charge_size_set_metadata_resources(&garment_sources.definition().metadata, &mut meter)?;
sc_measure::charge_garment_size_set_chart_resources(&garment_sources, &mut meter)?;
let represented_garment_family = meter.finish()?;
```

The native garment fixture has2 observations,1 POM target and1 saved chart binding; the second
observation is unused by the chart but still counts. Seven owned `gg` token fields give4 entries/
14 bytes/0 counter bits. Adding its actual common owner21/20 gives25/34. An explicit empty chart
component has0/0/0 while common metadata remains owned separately; that is no empty-model claim.
Entry maxima1/2/3 refuse0+2/2+1/3+1 at the actual Set/chart fields; all seven token fields have exact
first-owner/index refusal cases, ending with saved measurement token12+2.

The native MTM snapshot has two entries with token byte counts10/10/13 and14/10/9:2 entries/66 bytes/
0 bits. A partial one-entry snapshot contributes1/33. Expected spellings are asserted against actual
objects; independent Python UTF-8 lengths and OS wc agree33+33=66. The first run caught a manual
fixture expectation of64 and it was corrected before acceptance. Snapshot vector maximum1 refuses
0+2; token witnesses retain exact0+10/10+10/20+13/33+14/47+10/57+9, actual chart owner and entry0/1.

Seven new cases plus prior12 pass:19 source resource cases, with strict selected lint/formatting.
Both actual chart components share one meter at6/80, preserve original data and prior errors, and
stay inspectable when an empty replacement canonical context makes current validation refuse.
Higher-root composition and runtime admission remain owned; these component counts grant no currentness, physical, whole-host/model or allocator memory/time proof.

## Actual external family and inventory composition

charge_size_set_inventory_resources precharges the actual SizeSetInventory vector, then matches
each original Garment/Mtm variant in order. It composes that family's complete common metadata and
retained chart producers under the same sink. A singleton garment source keeps Garment provenance;
member count and matching labels never retag its instantiation path. Shared measurement/canonical
sources are not embedded in each set and are charged through their actual owners separately.

Exhaustive private inventory/family views and enum matching expose future stored fields or variants.
No identity census allocation, semantic/reference validation, current context, coverage query, numeric
read, grading or instantiation is performed. Component refusals retain their original exact owner,
field and local index; the anonymous outer vector uses size_sets with no invented owner/index.

```rust
let mut meter = DesignResourceMeter::new(resource_limits);
sc_measure::charge_size_set_inventory_resources(&size_sets, &mut meter)?;
let represented_external_sets = meter.finish()?;
```

The native mixed fixture has these owned components:

| Component | Entries | UTF-8 bytes |
| --- | --- | --- |
| Outer ordered set vector | 2 | 0 |
| MTM common metadata plus saved chart | 7 | 102 |
| Singleton garment metadata plus observations/chart | 11 | 137 |
| Total | 20 | 239 |

The metadata names/labels are asserted against actual objects; independent Python/OS byte counts
agree on Client A8, Client axis11, Client coordinate17 and Explicit count axis19. MTM common5/36
plus chart2/66 gives7/102. Garment common5/27 plus chart6/110 gives11/137. Both source families
keep their explicit provenance, authored references and zero unbounded counter bits.

Maximum1 entry refuses the outer vector at0+2. With MTM first, maximum19 refuses the garment
chart's saved bindings at18+2; maximum238 bytes refuses its last measurement token at230+9, actual
chart/index1. Reversing the actual set order preserves20/239 but the same entry maximum refuses
the MTM snapshot at18+2 with its own chart owner. Prior2 entries/2 bytes compose to22/241.

Five new cases plus prior19 pass:24 source resource cases, with strict selected lint/formatting.
The first lint caught duplicate test-fixture loading, an unused import and two disallowed panic
assertions; sharing one fixture module and using the established assertion style resolved them.
Actual empty inventory contributes0/0/0 and preserves first errors. Missing replacement canonical
sources make current validation refuse; held accounting preserves exact inventory/canonical/inputs.
Higher-root composition, runtime policy and deeper/full verification remain
owned; these counts grant no currentness, readiness, physical or allocator memory/time certificate.

## Complete immutable source-host port

Core's DesignResourceSource extends the readonly DesignReferenceHost contract with a required
charge_resources(&self, &mut dyn DesignResourceSink) method. There is no default implementation or
supplied totals carrier. The actual sc-measure DesignSources adapter exhaustively matches its private
owner and two-root definition, then charges MeasurementInputs followed by SizeSetInventory through
the same restricted sink. The port performs no clone, canonical/current validation, identity-census
allocation or context construction. Canonical values remain singly owned by the kernel.

Implementors must cover every owned dynamic field, including saved copies/caches, preserve original
schema/member order and precharge vectors. The sealed sink exposes no policy replacement/reset,
finish, model publication or authority method. This is a trusted integration contract: it cannot force
honest exhaustive behavior in arbitrary foreign host code. The actual adapter uses compiler-checked
owner/definition patterns and the tested component producers; no invented host or request statistics
replace actual data. A resource success supplies no semantic/currentness, evidence or authorization.

```rust
use sc_core::design::DesignResourceSource;
let mut meter = DesignResourceMeter::new(resource_limits);
actual_sources.charge_resources(&mut meter)?;
let represented_host = meter.finish()?;
```

The native complete host holds19 measurement/Ease entries and20 external-set entries:39 entries,
zero unbounded counter bits. Independent Python/OS UTF-8 counts for actual fixture fields give
landmarks40, procedures110, Measurements77, tables54, Ease42 and EaseSet66:389 bytes. Adding actual
sets239 gives628. Both declarations and saved snapshots retain their own logical text fields.
The initial test carried a stale626 total; the independent field census and actual refusals exposed
it, and corrected totals passed before acceptance.

Entry maxima3/20/38 preserve0+4 at landmarks,19+2 at size_sets and37+2 at the final chart bindings.
Maximum9 bytes refuses the first landmark name0+10;627 bytes refuses the final saved measurement
token619+9 with its actual chart owner/index1. Prior2 entries/2 bytes/1 counter bit compose to41/630/1.
An ignored host refusal cannot clear a prior counter error. An actual explicitly empty host has0/0/0
source demand; this says nothing about separately owned kernel recipe or root fields.

Four new actual-host cases plus prior24 pass:28 source resource cases, with strict core-library and
source-test lint/formatting. A moved test assertion was corrected to borrow preserved values. Missing
replacement canonical sources still refuse current validation; generic port accounting preserves the
exact held host/canonical data. Kernel higher-root composition must additionally charge its retained
DesignMetadata entity catalog and fixed provided-reference fields. That composition, runtime policy
and deeper/full verification remain owned; source demand is no whole-model/allocator time/memory certificate.
