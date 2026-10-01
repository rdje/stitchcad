# Measurement metadata: body quantities and garment POMs

sc-measure implements standalone immutable Measurement metadata and current references at
G1-SLICE.4a.2b, consuming [core length inputs](measurement-inputs.md). G1-SLICE.4a.3 adds the
named MeasurementTable. [Per-POM Ease mappings/sets](ease-inputs.md) are implemented separately; SizeSet follows.
This family does not close G1-SLICE.4.
[Ontology §2.1](ontology.md) remains the normative field contract. These libraries compile on the
native and WASM targets; the command/API/MCP facade follows in its owned G1 slices.

Runtime review captured at bf29b03: the [Rust run](https://github.com/rdje/stitchcad/actions/runs/36921077740)
and [doctrine run](https://github.com/rdje/stitchcad/actions/runs/36921077711) completed successfully;
job/step queries confirm every step, including strict lint/tests and the three-crate WASM build.
This is structural/runtime evidence for metadata. Table contracts and cross-compilation are verified
locally at .4a.3; source/physical/release proof remains deferred.

## Content and references

Each MeasurementDefinition has identity, human-readable name, exact MachineToken, entered Unit,
MeasurementKind (Body or Garment), two landmark identities, a procedure identity and the sole canonical
length-declaration identity. Measurement validates this metadata and exposes it without unchecked
mutation. The display name can contain localized text; it never changes the token. A blank name and
any binding of the eight reserved formula inputs refuse. Units remain entered-unit metadata: changing
cm to in does not reconvert the internal value. For example, an entered 3/4 in length remains 19050 µm
in its declaration; readers can use sc-units to present the recorded unit exactly.

A Landmark has a nonblank name, explicit body/garment kind and source record. A MeasurementProcedure
has a nonblank name and canonical nonblank documentation, kind and source record. Metadata holds only
its procedure id, so multiple measurements share one current document rather than copied instructions.
Caller-authored records provide no standard vocabulary or standards conformance; see
[the adopted standards boundary](standards.md).

For a fixture waist girth, the authored pair may name the same waist-level landmark twice. This
retains level intent; it does not pretend a circumference is a straight segment or enforce distinct
endpoints. A waist-to-hem POM can name two garment landmarks in authored order and a garment procedure.
Supplying body landmarks or a body procedure for that POM refuses with the offending identity and
expected/current kind. The library imposes no invented subject vocabulary or physical dimension bound.

## Current records and actionable refusals

MeasurementContext borrows current declaration, landmark and procedure inventories. Duplicate
identities refuse before lookup, across both same-kind and cross-kind records. A Measurement cannot
reuse a supplied target record's identity. Missing referenced records refuse by identity; a remaining
peer is never selected automatically. The command layer will supply current Design revision/context;
a borrowed context is not independently a current-revision or release certificate.

Measurement.new checks all required targets. Measurement.validate_current checks them again against
supplied current records, preserving the saved metadata. Removing a landmark leaves the old id
inspectable and produces MissingLandmark; changing its kind in a same-id replacement produces
LandmarkKindMismatch. Equivalent procedure changes produce MissingProcedure or ProcedureKindMismatch.
No reference is silently retargeted. A cloned definition can form an explicit validated replacement,
leaving the original object unchanged.

Measurement.declaration borrows the current canonical length state/value/source by its held identity.
A same-id edit from Assumed to Unknown is visible immediately; RequiresObservation stays a refusal,
not a cached number. Derived retains its sole formula and requires evaluation. Landmark and procedure
queries likewise borrow current records and validate their own target domains. Each target query
proves only that target; validate_current checks the whole metadata object. Displaying a surviving
value while another reference is missing grants no validation or release permission.

## Named tables and stable bindings

MeasurementTableDefinition holds a stable table id, nonblank human-readable name and authored ordered
entries. Each MeasurementBinding captures a Measurement id, its exact token, Body/Garment kind and
canonical declaration id. MeasurementBinding.from borrows the immutable metadata to capture these
expected references; it copies no numeric value, state, source, landmark or procedure document.
An empty named draft is legal. Duplicate measurement ids or tokens within one table are refused;
duplicate display labels and multiple measurements sharing a declaration are legal.

For an illustrative table named “Skirt measurements”, the body-waist entry can bind measurement 1,
token body_waist, Body and declaration 10; the garment-waist POM binds measurement 2, token
garment_waist, Garment and declaration 11. A caller may record assumed 740000 µm and 780000 µm
respectively with their distinct assumption records. These are illustrative inputs, not prescribed
physical measurements or a computed Ease. The table keeps the four binding fields and each input's
required metadata remains on the current Measurement; the canonical declaration holds its value/state.

MeasurementTableContext borrows current measurements and the existing MeasurementContext. Repeated
measurement ids or collisions with supplied declaration/landmark/procedure ids refuse before lookup.
Two distinct tables may bind the same token spelling to different measurements in a shared inventory;
uniqueness is table-scoped. Recipe composition still owns its [single flat namespace](formula-language.md).
The table id cannot reuse any supplied metadata/target id. This is local structural identity checking,
not a global Design or source/evidence registry certificate.

MeasurementTable.measurement selects a saved binding by exact token; measurement_by_id selects its
saved metadata id. Both then resolve that id in current inventory and check expected token, kind and
canonical declaration, followed by all required Measurement current-target checks. MeasurementTable.declaration
borrows that selected input's current core declaration, never a cached numeric result.
Targeted queries validate just that entry; validate_current checks the entire table.

| Current edit | Table query result |
| --- | --- |
| Remove measurement 1; measurement 4 has the same token | MissingMeasurement for 1; no peer transfer |
| Rename measurement 1 token to renamed_waist | TokenMismatch with original/current spelling; new token is not bound by the old table |
| Replace measurement 1 with valid garment metadata | KindMismatch; no body/POM conversion |
| Reassign measurement 1 from declaration 10 to 11 | DeclarationMismatch; old scalar identity remains inspectable |
| Edit declaration 10 from assumed to unknown | Current Unknown is borrowed; authored_value returns RequiresObservation with its request id |
| Revise display label, entered unit or procedure documentation | Borrow current metadata/document; internal length is not reconverted |
| Remove a required procedure | InvalidMeasurement preserves table id, measurement id and underlying MissingProcedure |
| Reorder current inventories | Id/token selection unchanged; authored table order retained |

Rebinding is explicit: clone table.definition, replace the intended entry with a newly captured
MeasurementBinding, then call MeasurementTable.new against the current context. A successful validated
replacement leaves the prior table unchanged; a missing or incompatible target returns a typed error.
Birth success is not cached current approval. Private representation prevents unchecked edits.

A valid unknown input can be saved and inspected. An API or future MCP client can query its core state,
source and required observation without inventing zero or copying a peer's preference. A derived input
retains its formula and requires the evaluator. Full table structural success does not authorize
release, prove procedure repeatability or validate the source of a declared Known value.

## Proof boundaries

Nonblank procedure documentation proves content exists, not that a physical measurement is repeatable.
Landmark names/source ids and measurement state do not prove source truth, independent evidence or
standard content. Design/recipe owns global identity and source registries; G2/G3 owns executed
geometry/domain repeatability, and G4 owns scoped evidence and artifact policy. Every referenced
length declaration retains its explicit deferred provenance marker. Current structural success
cannot replace any of those proofs or the independent production review.

| API token | Meaning in measurement metadata |
| --- | --- |
| `MeasurementKind` | Explicit Body or Garment domain |
| `Body` | Anatomical measurement domain |
| `Garment` | Garment point-of-measure domain |
| `LandmarkDefinition` | Authored named kind/source record |
| `Landmark` | Immutable structurally named landmark |
| `MeasurementProcedureDefinition` | Authored named kind/source and documented procedure |
| `MeasurementProcedure` | Immutable canonical documented procedure |
| `MeasurementDefinition` | Entered unit/name/token and required stable references |
| `Measurement` | Immutable metadata with current-reference queries |
| `MeasurementContext` | Unambiguous borrowed canonical inventories |
| `MeasurementError` | Structural metadata/current-reference refusal |
| `EmptyName` | Named metadata lacks nonblank human-readable content |
| `EmptyProcedureDocumentation` | Procedure has no nonblank documented content |
| `ReservedInputToken` | Measurement attempts to rebind a built-in formula input |
| `DuplicateIdentity` | Metadata/context reuses a semantic identity |
| `MissingDeclaration` | Canonical length declaration is absent |
| `MissingLandmark` | Required landmark identity is absent |
| `MissingProcedure` | Documented procedure identity is absent |
| `LandmarkKindMismatch` | Current landmark belongs to the other domain |
| `ProcedureKindMismatch` | Current procedure belongs to the other domain |

| Table API token | Meaning |
| --- | --- |
| `MeasurementBinding` | Expected measurement/token/domain/declaration tuple; no scalar cache |
| `MeasurementTableDefinition` | Named stable table and ordered authored entries |
| `MeasurementTable` | Immutable validated table bindings |
| `MeasurementTableContext` | Borrowed unambiguous current metadata and canonical target context |
| `MeasurementTableError` | Structural table/current-binding refusal |
| `DuplicateToken` | Table repeats one machine spelling |
| `MissingToken` | Requested spelling is not bound by this table |
| `MissingBinding` | Requested measurement id is not bound by this table |
| `MissingMeasurement` | Bound metadata id absent; no same-token peer selection |
| `TokenMismatch` | Current token differs from saved binding |
| `KindMismatch` | Current body/POM domain differs from saved binding |
| `DeclarationMismatch` | Current metadata reassigned the canonical scalar id |
| `InvalidMeasurement` | Required current metadata/target failure retained as structured source |
