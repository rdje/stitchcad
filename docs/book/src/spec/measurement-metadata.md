# Measurement metadata: body quantities and garment POMs

sc-measure implements standalone immutable Measurement metadata and current references at
G1-SLICE.4a.2b, consuming [core length inputs](measurement-inputs.md). The named MeasurementTable,
per-POM Ease and SizeSet follow separately; this partial family does not close G1-SLICE.4.
[Ontology §2.1](ontology.md) remains the normative field contract. These libraries compile on the
native and WASM targets; the command/API/MCP facade follows in its owned G1 slices.

Runtime review captured at bf29b03: the [Rust run](https://github.com/rdje/stitchcad/actions/runs/36921077740)
and [doctrine run](https://github.com/rdje/stitchcad/actions/runs/36921077711) completed successfully;
job/step queries confirm every step, including strict lint/tests and the three-crate WASM build.
This is structural/runtime evidence for metadata, with source/physical/release proof still deferred.

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
