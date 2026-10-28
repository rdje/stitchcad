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

Actual report/difference/accepted-record walkers, private retained model/reserve/journal composition
and prospective runtime retained admission remain G1-SLICE.6.8.2.2.2.2.4.3.2–.4.3.4. Coupled resource
acceptance .4.4 and deeper/full verification remain deferred. Logical field counts do not bound
allocator peaks, host algorithm time, raw ingress, numeric/file/decompression/geometry work or physical/
release acceptance.
