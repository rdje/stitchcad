# Actual command data resource accounting

Command inputs, reports and retained execution state own dynamic data in addition to the actual
[model](model-resource-accounting.md) and [source host](source-resource-accounting.md). They use the
same explicit logical units and sealed sink from the [resource contract](command-operation-control.md#logical-dynamic-content-accounting-contract).
CommandState locations describe these objects; a resource result is data, not authority or a model
snapshot stamp. Actual private-state composition and runtime retained admission remain owned work.

## Actual typed mutation-group inputs

charge_command_mutation_group_resources borrows an actual immutable CommandMutationGroup. It
charges the common snapshot context's represented revision bits, then precharges the entire owned
mutation vector. Each member contributes its own original context revision and actual payload.
The common context and member contexts retain separate revision values, even when equal. An exact
retry alias also owns another context/payload copy here; runtime execution deduplication does not
remove those stored fields.

| Actual payload | Additional owned dynamic content |
| --- | --- |
| ChangeMeasurement | actual replacement declaration's Known evidence vector |
| AssignMaterial | actual ordered Piece selection vector; one exact unresolved reason string, if provided |
| AddNotch | none; Piece, anchor and symbolic profile references remain fixed values |
| SplitSeamSpan | none; original span and explicit anchors remain fixed references |
| CloseDart | none; original Dart target remains a fixed reference |

The five variants are explicit and exhaustive. Private group/context/mutation/payload/selection and
LengthDeclaration views match every owner field. Copy guards cover fixed contexts and construction
intents, so an owned-field change requires an accounting decision. The declaration snapshot walker
is shared with canonical model accounting; Known/Assumed/Preference/Unknown/Derived states retain
their actual stored form, with no numerical availability or inferred evidence.

```rust
let mut meter = DesignResourceMeter::new(explicit_resource_limits);
charge_command_mutation_group_resources(&actual_group, &mut meter)?;
let owned_request = meter.finish()?;
```

This creates no replacement model, copy of the group, grant, context lookup or execution result.
It calls no model/source census, currentness, numerical or authority provider. Typed references can
still be stale or belong to unavailable handlers. Accounting preserves the original object and does
not establish currentness, handler availability, source truth or permission. Initial transport decoding
and allocations needed to construct typed inputs are separate; this is not a raw-ingress limit.

## Owned strings and declared operation demand

An AssignMaterial payload stores one reason string beside its selected Piece IDs. If it selects two
Pieces with reason ` é `, that request owns4 UTF-8 bytes once. The separate CommandOperationDemand
counts8 proposed reason bytes because applying the operation can copy that string onto both Pieces.
Actual candidate/report/journal owners must charge their own retained fields. These are distinct
questions; neither count silently substitutes for the other.

The native mixed fixture uses all five typed variants plus an exact ChangeMeasurement alias. The
replacement owns2 evidence references, and the alias owns another2. The Material selection owns2
Piece IDs. Six member slots plus those vectors total12 collection entries. The single reason owns4
bytes. Common revision3 and each of six member revision3 values use2 bits apiece:14 magnitude bits.
The fixture deliberately supplies a revision claim without asking a model to validate it; source
telemetry remains0 clones/0 resource calls/0 semantic callbacks.

A prior2 collection entries/2 UTF-8 bytes/1 counter bit composes to14/6/15 under the same meter.
A latched prior failure survives. A group containing just the three fixed construction intents at
revision0 counts3/0/0. A single explicit Unknown Measurement replacement at revision0 counts1/0/0;
it does not acquire a Known evidence vector or a numeric value.

## Original first refusals

Locations use CommandState scope, the actual containing group ID, a static field and the original
zero-based member index. A nested replacement is owned data inside that request group; its referenced
canonical declaration ID is not promoted into a separate containing owner. Whole-group fields use
no member index. No private reason or source text is copied into a location.

| Maximum | Original first refusal |
| --- | --- |
| 1 counter bit | common context0+2, requests.group.context.revision |
| 5 entries | mutation vector0+6, no member index |
| 7 entries | first replacement evidence6+2, member0 |
| 9 entries | Material selection8+2, member1 |
| 11 entries | alias replacement evidence10+2, member5 |
| 3 text bytes | unresolved reason0+4, member1 |
| 13 counter bits | final member context12+2, member5 |

Four native request cases verify exact counts, every listed first witness, unchanged input objects,
prior-prefix/first-error behavior and fixed/Unknown payloads. The33 existing source resource cases
and6 cases selected by the canonical filter pass, with strict core-library/affected-test lint and
formatting. Public constructors and command-bus code remain byte-exact in this slice. No fake host,
supplied resource totals, model clone or current/source/authority callback supplies the evidence.

## Actual preview reports and net differences

charge_command_preview_resources borrows a real CommandPreview. Its three exhaustive variants
charge their own represented base revision and actual owned fields. Measurement reports precharge
their change vector and count each before/after declaration's Known evidence separately. Material
reports precharge their change vector and count each before/after unresolved reason separately;
Assigned references remain fixed IDs. A Notch report owns fixed identity, anchor and symbolic profile
bindings, plus its revision; it gains no evidence, strings, inferred dimensions or collection entry.
An empty change vector contributes zero entries. Revision0 contributes zero magnitude bits.

charge_command_group_preview_resources charges the actual common report revision, then the entire
step vector before walking members in original order. Each step owns its full preview, including
another revision. An exact alias owns another full report snapshot even though it applies no second
edit. After the steps, the reader precharges the net difference's lengths, materials and notches
vectors in that order, walking their actual members. Net before/after snapshots are owned copies
beside the step copies, so both count. The reader never reconstructs a model or deduces a report
from a request or an operation-demand total.

```rust
let mut meter = DesignResourceMeter::new(explicit_resource_limits);
charge_command_group_preview_resources(&actual_bus_report, &mut meter)?;
let owned_report = meter.finish()?;
```

Standalone locations use the actual Design owner. Their revision/vector fields have no index;
before/after fields use the original change index. Group step locations use the actual group owner
and original step index, including aliases. Net-difference fields use that group owner and their
original change index, with no index for the whole vector. Locations carry static field names and
no private reason text. The reader does not promote a referenced Piece/declaration into a containing
owner, authorize access, validate currentness or acquire publication capability.

The native fixture reconstructs a real revision3 model with two Pieces explicitly assigned an actual
available Material, then asks the inspect-authorized bus for these five actual sequential steps:

1. ChangeMeasurement replaces Known2 evidence with Known3 evidence at the same positive width.
2. AssignMaterial selects both Pieces in reverse order with reason ` é `.
3. AddNotch prepares one actual semantic mark using the private staged identity reserve.
4. An exact alias of step0 owns another original Measurement report.
5. AssignMaterial changes both reasons to ` β `.

The single-report demands in that order are6/0/2,2/8/2,0/0/2,6/0/2 and2/16/2
(entries/UTF-8 bytes/counter bits). The group owns5 step slots and2 common revision bits. Its net
length difference owns1 slot plus2+3 evidence entries, its net Material difference owns2 slots and
8 reason bytes, and its net Notch difference owns1 slot. The complete group report is30/32/12.
Both Unicode reasons are independently verified as4 bytes. A prior2 entries/2 bytes/1 bit composes
to32/34/13 without replacing or restarting the sink.

| Maximum | Actual original first refusal |
| --- | --- |
| 4 entries | reports.steps,0+5, no member index |
| 7 entries | reports.measurement.before.evidence,6+2, step0 |
| 18 entries | reports.measurement.after.evidence,16+3, alias step3 |
| 26 entries | reports.difference.lengths.after.evidence,24+3, net change0 |
| 29 entries | reports.difference.notches,29+1, no member index |
| 15 text bytes | reports.material.after.reason,12+4, step4 |
| 27 text bytes | reports.difference.materials.after.reason,24+4, net change0 |
| 11 counter bits | reports.material.revision,10+2, step4 |

A seven-step variant restores the original Measurement and both Material assignments. Those net
vectors become empty; the existing Notch remains a net addition. The intermediate and restoring
step snapshots still count:32/32/16. A separate actual revision0 no-op followed by an explicit
Unknown Measurement produces8/0/0 for the group,0/0/0 for the no-op report and3/0/0 for the
Unknown report. Unknown adds no invented evidence or numeric value; its actual Known before
snapshot remains owned. Prior failures survive every variant, including the fixed Notch report.

Five native report cases and the four existing request cases pass with strict affected lint and
formatting. Original report objects are unchanged; source telemetry and authority reads stay zero
during accounting. Every private report owner and change/Notch definition is matched exhaustively.
The original runtime/report implementations are byte-exact after removing the added readonly views;
execution, history, preparation, operation and identity files are byte-exact in this slice.

Accepted authoring/history record readers remain G1-SLICE.6.8.2.2.2.2.4.3.2.2; private retained
model/reserve/journal composition and prospective runtime retained admission remain .4.3.3–.4.3.4.
Coupled resource acceptance .4.4 and deeper/full verification remain deferred. Logical field counts
do not bound allocator peaks, host algorithm time, raw ingress, numeric/file/decompression/geometry
work or physical/release acceptance.
