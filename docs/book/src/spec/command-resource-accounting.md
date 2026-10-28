# Actual command data resource accounting

Command inputs, reports and retained execution state own dynamic data in addition to the actual
[model](model-resource-accounting.md) and [source host](source-resource-accounting.md). They use the
same explicit logical units and sealed sink from the [resource contract](command-operation-control.md#logical-dynamic-content-accounting-contract).
CommandState locations describe these objects; a resource result is data, not authority or a model
snapshot stamp. Actual current private-state composition/admission is implemented; prospective retained
authoring/history projections are implemented with focused checks. P0 D293 source-clone ownership
repair and coupled/full verification remain owned work.

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

## Actual accepted authoring and history records

These readers inspect immutable data returned by real execution. They do not accept a request as
proof that an action occurred, look up a private journal or perform a new effect.

| Reader | Actual owned fields |
| --- | --- |
| charge_command_group_commit_resources | original group report, issued revision, discarded-redo ID vector |
| charge_command_audit_entry_resources | effect sequence, exact original request copy, accepted commit |
| charge_command_group_commit_result_resources | one accepted-record copy; fixed replay flag |
| charge_command_history_request_resources | current operator context revision, complete original authoring-group copy; fixed direction |
| charge_command_history_audit_entry_resources | effect sequence, effective-command vector, exact history request, before/restored revisions |
| charge_command_history_result_resources | one accepted history-action copy; fixed replay flag |

Every owner is matched exhaustively, including grant/time, replay, direction and oldest-undo ID
fields guarded as fixed values. SystemTime here is the original trusted final authorization observation;
reading it calls no clock or authority provider. Audit sequence and every owned DesignRevision count
their actual represented magnitude bits, including duplicate values and zero. An authoring entry
walks sequence, request and commit in schema order. A commit walks its report, issued revision and
then precharges its complete discarded-redo vector. A history action walks sequence, precharges
its stored effective-command vector, walks the full history request, then before/restored revisions.
An original retry alias stays in the copied request; the separate stored effective-command vector
omits execution aliases. Neither reader reconstructs or deduplicates either owner.

```rust
let mut meter = DesignResourceMeter::new(explicit_resource_limits);
charge_command_audit_entry_resources(actual_accepted.entry(), &mut meter)?;
let owned_record = meter.finish()?;
```

The result reader charges its one owned entry once. Calling an entry reader and its containing
result reader on the same sink would deliberately walk that entry twice; choose the containing
owner being measured. Exact replay creates no additional private accepted record, but its returned
owned result still contains a complete copied record and has the same demand as the original result.
No model, journal lookup, fresh grant, currentness or publication capability is carried by a count.

The native accepted fixture starts with a real revision0 Known2 model and two actual Piece reasons
`explicit selection pending`, independently verified as26 UTF-8 bytes each. Its four submitted
members are a Known3 Measurement replacement, a reversed two-Piece Material reason ` é `, an
actual AddNotch, and an exact alias of the Measurement request. The inspect/commit-authorized bus
actually accepts the group. Its request is12/4/0; the report plus issued revision is27/120/1.
The report text includes each old26/new4 reason for both Pieces in the step and again in the net
difference. Effect sequence1 adds1 bit: the complete authoring entry and each returned result copy
are39/124/2. A prior2 entries/2 bytes/1 bit composes to41/126/3.

A distinct human operator with a real complete grant performs Undo and Redo. The original agent
and principal stay unchanged inside the copied authoring group. The actual effective command vector
has3 entries, omitting the alias. The Undo request is12/4/1; sequence2, before revision1 and restored
revision0 make its action15/4/4. Redo's sequence3, operator/before revision0 and restored revision1
make its action15/4/3. Exact Undo replay after Redo returns the original action; exact authoring
replay after the round trip returns its original record. Accounting these copies calls neither
source nor authority, and changes neither original object nor accepted metadata.

Original locations stay under the actual containing authoring group. Record-specific fields include
records.authoring.sequence, records.commit.revision, records.commit.discarded_redo,
records.history.sequence, records.history.commands, records.history.before_revision and
records.history.revision. Nested requests/reports preserve their existing original fields/positions;
requests.history.context.revision identifies the current operator's own revision. A record sequence
limit can refuse before walking requests or reports. A full effective-command vector is refused
before its members. No private reason text is copied into a witness. The first prior failure survives
every record, result, commit and history-request reader.

The branch case accepts a second group, undoes both groups, then accepts a new branch. The actual
commit's discarded-redo vector preserves both original group IDs in redo order and contributes2
entries. That accepted entry is7/64/5: sequence5 uses3 bits while newly issued revision3 uses2,
independently of the restored revision0. At6 entries, records.commit.discarded_redo refuses5+2;
at2 bits, records.authoring.sequence refuses0+3. After two more no-op commits at bounded undo depth2,
the oldest-undo ID is present but fixed, the discarded vector is empty, and the actual final entry
is3/4/18. No lost-history event is fabricated from a count.

Four actual record cases, five report cases and four request cases pass with strict affected lint
and formatting. All original execution/history/request implementations are byte-exact after removing
the new exhaustive readonly views; preparation, operation, identity and group implementations are
byte-exact. Existing request/report readers are byte-exact. This implements accepted data accounting
with focused checks. Current private retained model/reserve/journal composition is admitted; prospective
authoring admission is implemented; D293 source ownership repair remains; coupled acceptance .4.4 and deeper/full verification stay deferred. Logical counts do not bound allocator peaks, host algorithm time, raw
ingress, numeric/file/decompression/geometry work or physical/release acceptance.


## Explicit immutable retained-policy configuration

The composition root calls install_retained_limits with explicit positive DesignResourceLimits for
logical collection entries, UTF-8 bytes and represented unbounded-counter magnitude bits. This is a
third independent policy beside operation-input limits and per-model limits. None has a default.
new, with_identities and edit/history constructors start without retained limits. Installation does
not walk private models, reserve, journal or history, validate currentness, clone sources or consult
authority. It changes only the private policy slot. Even a tiny explicit policy can be installed on
an existing owner; construction/initial allocation and admission are separate concerns.

```rust
bus.install_operation_limits(explicit_input_limits)?;
bus.install_model_limits(explicit_per_model_limits)?;
bus.install_retained_limits(explicit_retained_limits)?;
```

Costly implemented single/group previews, commits and history traversal require all three installed
policies. Missing retained configuration returns CommandControlError::RetainedLimitsNotInstalled
after pure operation admission and the existing model-policy presence check, before authority,
progress, private traversal or preparation. Repeat installation, with either the same policy or a
larger one, returns RetainedLimitsAlreadyInstalled and preserves the original policy. Requests and
progress observers cannot replace, widen or reset it; direct private-field reset fails compilation.

is_preview_available requires all three policies and an implemented handler; is_commit_available
also requires edit capacity. Availability is no authority, currentness, quota-room or validity
certificate. Authorized GetPieceList remains independent of every policy and does no private source
walk, even with tiny retained configuration or missing edit capacity.

Two native configuration cases cover plain/reserve/edit-history constructors, once-only installation,
missing-policy refusal across all four costly entry points and cheap reads before/after setup.
Source telemetry stays0/0/0 and configuration/setup refusals read authority0 times. The11 bus
compile-failure contracts use the bounded space-free command::bus::CommandBus doctest filter. The existing
operation-policy and model-policy setup cases retain their original precedence and behavior; strict
affected lint/formatting pass. Test helpers install explicit permissive retained limits; the helper
that deliberately omits model limits is named without_model_limits and still supplies retained policy.

Installation alone certifies no retained content or quota room. Actual current private admission now
runs as described below; [prospective authoring](command-retained-publication.md) also admits exact
evictions/redo discard before publication allocations/effects. [History projection](command-retained-publication.md#exact-prospective-history-actions) is implemented; D293 ownership repair remains. Coupled/deeper/full resource acceptance stays .4.4.

## Actual current private retained composition

Before preparing a supported metadata operation or traversing a new history action, the bus admits
its complete actual retained artifact state under the installed retained policy. Fresh authority,
Design and exact snapshot binding precede this private walk. Accepted authoring/history replays
return their original records before walking private models, even when current retained content is
over the installed bound. Ordinary authorized GetPieceList remains a borrowed policy-free read.
There is no public private-state meter, model getter, replacement token or caller-supplied census.

The exhaustive internal bus view charges these actual owners, using one cumulative sealed meter:

1. The complete current model, including its actual source host and separately stored entity census.
2. The full immutable identity-reserve backing, including the consumed prefix. Its finite consumed
   cursor is fixed data; the backing does not shrink after a successful Notch allocation.
3. The journal's initial revision, independent maximum-issued revision and current effect sequence,
   followed by its whole accepted-entry vector and every actual authoring record in stored order.
4. The whole undo deque, each actual model alternative front to back; the whole redo vector and
   each actual alternative in stored vector order; then the whole chronological action vector and
   each actual accepted history-action record. Redo application order is a separate runtime concern.

Each collection is precharged before its members. Model and record producers preserve their native
original locations; no private text or invented containing identity is added to a refusal. Reserve,
journal and stack/vector fields use CommandState scope and the actual Design owner. Static fields
are retained.identities, retained.journal.initial_revision, retained.journal.maximum_revision,
retained.journal.sequence, retained.journal.entries, retained.history.undo, retained.history.redo
and retained.history.actions. Multiple owned model alternatives count separately even when revisions
or source values match. Their immutable source contract remains honest/exhaustive; no semantic census,
validation, formatting or clone is used by runtime accounting.

Finite count limits, configured policies, grant/time and cursor/identity fields stay fixed and are
guarded accordingly. The trusted foreign authority service A is outside this retained artifact scope;
the walk neither inspects nor calls it. Every represented model's owned source H remains included.
This bounds logical artifact content, not arbitrary authority-service allocation or host algorithms.

The whole-owner RetainedValidation phase emits0/1 before and1/1 after actual admission, with no member
index. It follows SnapshotBound and precedes member preparation/current validation; history also checks
its actual target and action room first. Cancellation at0 makes no resource call. Cancellation at1
has completed the retained walk but performs no semantic validation, candidate clone or effect.
Retained refusal preserves CommandControlError::Retained and the original resource error through the
standard source chain. A prior failure is never reset. Each actual retained model contributes one
source resource call to this phase, separately from later per-model current/prospective admission.

The native oracle independently rebuilds and validates actual Material models at revisions0,1,2
through public constructors. Real buses produce the two accepted records, Undo of group2, Undo of
group1 and Redo of group1. The final current model is revision1, the undo alternative is revision0,
the redo alternative is revision2, the maximum-issued revision is2 and effect sequence is5. Both
accepted records and all three history-action records remain. The reserved three-ID backing remains.

Let M and B be the independently measured revision0 model entries and text bytes. Each actual model
has M entries; the two edits replace26-byte reasons with4-byte reasons. Complete retained demand is
3M+29 entries,3B+74 bytes and36 magnitude bits. This is re-derived from the independently retained
actual models and returned immutable records, not from private bus statistics. Exact limits reach
RetainedValidation1 with0 clones/3 source calls/0 semantic callbacks. One-short entry, text and bit
policies preserve the same first error as that independent composition. Exact authoring/history
replays still invoke0 clones/0 resource calls/0 semantic callbacks.

A separate actual Notch commit consumes one of two reserved IDs. Both backing slots still count.
The [prospective authoring proof](command-retained-publication.md#native-exact-limit-examples) now
refuses a later one-short publication before growth. Cancellation before/after current retained
admission leaves the original group/key/capacities available for a later actual commit. Unauthorized,
wrong-Design and stale single/group/commit/history paths still cause no private resource walk.

Four current-retained cases and the affected model-admission cases pass with strict affected
lint/formatting. The extra current-retained pass changes source-walk telemetry, while candidate-copy
counts remain unchanged. [Prospective authoring admission](command-retained-publication.md) now
prevents new group publication from exceeding the exact future retained bound, including undo eviction
and redo discard. Prospective history projection is implemented; P0 D293 source-clone ownership repair blocks signoff,
and coupled .4.4/deeper/full verification stays deferred. Raw ingress, allocator peaks, arbitrary host time, numeric/file/
decompression/geometry work and physical/release acceptance retain their separate owners.
