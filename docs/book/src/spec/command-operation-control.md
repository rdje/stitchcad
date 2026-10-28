# Operation limits and progress

The [command-layer contract](command-layer.md#6-structured-errors-and-progress) requires bounded
inputs, declared progress and atomic cancellation. The [execution guide](command-execution.md)
describes implemented metadata operations. Installed input policy, real pre-effect progress and
cooperative atomic cancellation cover individual/group metadata previews, commits and history traversal.

## Explicit finite metadata admission

CommandOperationLimits::new takes four explicit NonZeroUsize maxima. There is no guessed default.
check_mutation and check_group borrow original typed inputs and return CommandOperationDemand or
the first CommandOperationLimitError. They perform no model, registry, authority or history lookup.

| Unit | What counts |
| --- | --- |
| members | every original submitted member, including an exact retry alias |
| targets | one Measurement, selected Material Pieces, or one Notch-owning Piece |
| reason_bytes | exact UTF-8 unresolved reason bytes for each proposed selected Piece assignment |
| evidence_references | Known-state evidence references in each submitted Measurement replacement, including aliases |

These are declared metadata input units. They are not observed CPU time, total model memory,
temporary allocation peaks, numeric complexity, file/decompression bounds or geometry/solver work.
Unavailable SplitSeamSpan/CloseDart handlers contribute no implemented metadata targets or reason
bytes; their submitted members still count. Admission supplies no handler availability or execution.

```rust
// The host supplies all three validated positive maxima explicitly.
let limits = CommandOperationLimits::new(
    maximum_members, maximum_metadata_targets, maximum_assignment_reason_bytes,
    maximum_measurement_evidence_references,
);
let demand = limits.check_group(&group)?;
let submitted_members = demand.members();
let primary_target_selections = demand.targets();
let proposed_assignment_reason_bytes = demand.reason_bytes();
let proposed_measurement_evidence_references = demand.evidence_references();
```

For example, assign the nonblank reason `é` to two Pieces, change one Measurement, then submit an
exact alias of that first assignment. The three submitted members contain five target selections
and eight proposed reason bytes: the reason has two UTF-8 bytes, copied into two proposed assignments
per submitted assignment member. The alias still counts for input admission; runtime replay semantics
remain separate. The native contract case admits exactly 3 members, 5 targets, 8 reason bytes and no evidence references.

Member-count excess is checked first. Then each original member checks targets, reason bytes and
evidence references in that order.
Members retains maximum/requested counts. Targets retains maximum, preceding count, this member's
count, original zero-based index and payload-derived command. ReasonBytes retains maximum, preceding
bytes, selected copy count, exact bytes per copy, original index and command. The exact multiplicative
components remain available even when a requested total would exceed the platform count range.
EvidenceReferences retains the configured maximum, exact preceding count, this original member's
count, index and command. For example, two references in one Known replacement and an exact alias
have cumulative evidence demand4; maximum3 refuses at index1 with consumed2/requested2.

Known-state evidence is counted by the original vector length, without copying, deduplication or
source lookup. Assumed, Unknown, Preference and Derived replacements contribute no evidence-vector
entries. This limit certifies neither existence nor truth of evidence. A corrected request may reuse
the refused keys because admission adds no authoring/history record or effect. The native runtime
case refuses this dimension on all four costly entry paths before any authority read or progress.

This API admits already constructed typed inputs. It does not bound raw transport decoding, the
initial LengthDeclaration structural constructor or allocation before the caller submits the input.
Those ingress and whole-model dimensions remain separate work; no raw untrusted API is exposed here.

The check compares against remaining room before adding or multiplying. It neither wraps, saturates,
clips input nor silently deduplicates members. Original contexts, payloads, identities and order remain
unchanged. Private limit fields expose immutable getters; compiler privacy rejects direct widening.
An admitted demand is data, not authority or a commit capability.

## Install trusted policy once

Every bus starts without operation policy. The trusted composition root must install explicit limits
before costly metadata operations; no request, observer or constructor supplies a hidden default.

```rust
let mut bus = CommandBus::with_edit_capacity(
    garment, trusted_authority, reserved_identities, maximum_committed_groups,
)?;
bus.install_operation_limits(CommandOperationLimits::new(
    maximum_members, maximum_metadata_targets, maximum_assignment_reason_bytes,
    maximum_measurement_evidence_references,
))?;
let result = bus.commit_group_with_progress(&group, &observer)?;
```

All five maxima in this example are host-chosen positive NonZeroUsize values. Installation changes
no model, grant, revision, reserve, audit or history. A second installation returns
CommandControlError::LimitsAlreadyInstalled and retains the original policy. Compiler privacy also
rejects direct policy reset. Missing policy returns LimitsNotInstalled on costly metadata paths.
Authorized GetPieceList inspection remains available without operation policy.

Pure submitted-input admission runs before authority lookup and private model work. It therefore
can refuse oversized input without consulting the installed source or emitting progress. Within
policy, original authority, binding, envelope, registry and target failures retain their typed causes.
An unauthorized request emits no progress. History admission covers the complete submitted original
group before constructing its current-operator authorization context. Exact aliases count in admission.

The existing preview, preview_group, commit_group and apply_history methods use the unit observer;
they enforce the same installed policy. Their explicit observer counterparts are preview_with_progress,
preview_group_with_progress, commit_group_with_progress and apply_history_with_progress. Availability
queries for preview/commit handlers require installed policy; availability supplies no permission,
remaining quota or candidate validity.

## Declared progress fractions

CommandProgress::new takes a CommandProgressPhase, completed count, positive total and optional
original member index/CommandId. A completed count above the total returns CommandProgressError
with the exact supplied counts. Zero total cannot be supplied through NonZeroUsize. Valid fractions
include `0/3` and `3/3`; `4/3` is refused rather than clipped.

```rust
let progress = CommandProgress::new(
    CommandProgressPhase::Preparing, completed_members, positive_member_total,
    Some((original_index, command_id)),
)?;
let declared_fraction = (progress.completed(), progress.total().get());
```

Fractions are local to their named phase. Authorized, SnapshotBound, Preparing, CurrentValidation,
CandidateValidation, AllocationPreflight, ReadyToPublish and Replaying are declared pre-effect steps.
ReadyToPublish means preparation is ready; it does not report a published model. The eventual success
response acknowledges completion. Constructing phase data proves no actual source check or effect.

## Data-only continuation decisions

CommandProgressObserver receives readonly CommandProgress and returns CommandProgressDecision::Continue
or Cancel. It cannot return a candidate model, replace an authority provider or mint a publication token.
The unit observer ignores progress and continues. No measurement values, labels or model handles are
present in progress data. Actual validation phases surround actual current/candidate validators;
Preparing counts original submitted members, including exact aliases. Fractions are phase-local,
not an overall time estimate. Observers must return promptly: checkpoints are cooperative, not
preemption of a running validator or a wall-clock watchdog.

Returning Cancel produces CommandControlError::Cancelled with the exact observed progress. Private
staged candidates are discarded. The original model/revision, reserved identities, audit records,
history stacks, issued counters and accepted keys remain unchanged. Retrying the same refused keys
can succeed. A cancellation after real candidate validation or after the first staged Notch therefore
consumes no original identity. Internal reserved collection capacity may grow during allocation preflight;
it is not published semantic state.

Closed audit records and return payload copies are prepared before the last ReadyToPublish checkpoint.
Fresh complete effect authorization follows that checkpoint, so observer-triggered revocation refuses
publication. No observer or large payload preparation runs after that final authorization. Publication
uses exclusive infallible state moves; cancellation cannot roll back an already published operation.
ReadyToPublish is preparation progress, not completion; only the successful response confirms effects.

Exact replay refreshes authority and emits Replaying before returning the original record. Cancelling
that retrieval changes no state and does not undo its previously accepted effect. Replay neither
advances audit sequence nor traverses history again.

## Source-host copy work

Private candidate reconstruction requests one copy of the trusted source host for each actual
non-alias metadata candidate. It consumes the copied nested owners through crate-private helpers
and rebuilds them through the same validating constructors. Public callers receive no extraction
method, mutable bus model or replacement capability. Complete source validation and final effect
authorization remain in place; moving a copied component is not a validation shortcut.

For n actual candidate members, grouped preview needs n reconstruction copies and a new commit
needs n+1, including its fresh-revision rebuild. Exact aliases add no candidate copy. The explicit
submitted-member maximum bounds n. Exact accepted replay and actual snapshot Undo/Redo request no
host reconstruction copy. The native case wraps the actual immutable DesignSources, forwarding
all public census/binding/reference/current-validation methods and observing only host copies.
It measures individual counts1/1/1, group preview3, commit4 and replay/Undo/Redo0; Undo also restores
the original Piece-slice address. Prior native individual counts were3/3/2.

This is a bound on bus-requested source-host copy invocations. It does not bound bytes or time inside
the trusted host's Clone/validation implementation, remaining core collection copies, total model
memory, raw input decoding or retained audit/history bytes. Remaining implementation is
G1-SLICE.6.8.2.2.2.2. This native source-copy case brings focused bus coverage to46 cases; it is not a
complete resource, performance or integration certificate.

## Identity-reserve transition work

Reserved identities have one immutable shared backing and a private consumed prefix. Preflight adds
the exact staged consumption with checked arithmetic, refuses an out-of-range prefix, then shares the
existing backing. It performs no unused-ID copy, including on zero-creation authoring. Publication
installs this prepared handle with the model and record at the same atomic boundary. This is
constant-sized reserve-transition metadata; other model/record preparation can still allocate.

Independent borrowed stage cursors start at the current prefix and count only their own consumption.
No arbitrary Clone/provider can share or advance those cursor positions. Exhaustion, including the
last128-bit identity, preserves the original IdError::Exhausted. A refusal or inspect-preview does not
advance the original prefix; Undo/Redo and exact replay never rewind or reuse issued IDs.

The initial reserve is validated and converted to shared storage once. Its entire supplied backing,
including consumed IDs, remains allocated for this bus lifetime. There is no hidden refill or reserve
getter. This trades retention of the finite initial backing for removal of reserve-size-dependent
publication copies. It does not certify initial conversion peaks or total memory/time. Focused proof
is the expanded native identity-cursor case plus the46 actual bus cases, with strict selected lint
and formatting. Initial/model/retained-state bounds remain G1-SLICE.6.8.2.2.2.2 work.

## Logical dynamic-content accounting contract

DesignResourceLimits and DesignResourceMeter live in the Design domain, independent of command
authority. They provide a data-only accounting contract with implemented component producers below.
Complete root/source accounting and runtime enforcement remain pending; the meter alone is not
a runtime model-admission gate.

| Unit | Exact declared accounting |
| --- | --- |
| collection_entries | entries in logical owned dynamic collections, including nested collections and reference vectors |
| text_bytes | exact UTF-8 bytes of logical owned text fields, without shared-backing deduplication |
| counter_bits | actual magnitude bits of logical unbounded counters; fixed-width numeric values are outside this unit |

The host supplies explicit positive maxima: NonZeroUsize for collection entries/text bytes and
NonZeroU64 for counter bits. There is no default. These are logical content units, not allocator RSS,
capacity overhead, a wall-clock deadline or evidence that the model is valid. A model producer must
precharge a collection's exact length before walking nested entries, charge every logical text field
and account for actual unbounded-counter magnitudes without serializing or cloning the counters.

For example, these are three demonstration charges; no actual Design is inspected:

```rust
let mut meter = DesignResourceMeter::new(DesignResourceLimits::new(
    maximum_collection_entries, maximum_text_bytes, maximum_counter_bits,
));
{
    let sink: &mut dyn DesignResourceSink = &mut meter;
    let location = DesignResourceLocation {
        scope: DesignResourceScope::SourceHost,
        owner: None,
        field: "example.entries",
        index: None,
    };
    sink.charge_entries(2, location)?;
    sink.charge_text("é", DesignResourceLocation {
        field: "example.label", index: Some(0), ..location
    })?;
    sink.charge_counter_bits(129, DesignResourceLocation {
        field: "example.revision", ..location
    })?;
}
let declared = meter.finish()?;
assert_eq!(declared.collection_entries(), 2);
assert_eq!(declared.text_bytes(), 2);
assert_eq!(declared.counter_bits(), 129);
```

DesignResourceScope identifies Structure, Inputs, Sewing, Garment or SourceHost. Locations retain a
optional actual owner identity, static schema field token and original zero-based authored index.
Owner identity is location data, never permission or a model handle. Locations contain no copied
user label/value or model handle. Count refusals retain CollectionEntries or TextBytes, the explicit
maximum, successful preceding count, exact attempted amount and location. CounterBits retains its
original u64 components and location. Remaining-room comparisons precede addition, so even an
attempt whose sum would exceed the platform/u64 range remains an exact typed refusal.

The first refusal is latched. Every later charge, including an empty/zero charge in another unit,
returns that same original error; finish also refuses even if a producer ignored an earlier error.
No partial successful demand escapes on finish after failure. Empty charged demand remains data;
it does not certify that a model was traversed. The meter retains counts/locations and no model text.

A producer receives only a mutable DesignResourceSink trait object. The sink is sealed, with charging
methods and no policy setter, reset, finish or model/publication operation. The concrete meter's policy
and state are private. This prevents replacement through the supplied charging capability; an honest,
exhaustive producer remains an integration obligation, not a security claim against dishonest host code.

The meter contract established six native cases and four compiler privacy/sealing cases, with strict selected lint and
formatting. The actual core visitor is G1-SLICE.6.8.2.2.2.2.2, the actual immutable source-host producer
is .3, and prospective shape/runtime installation is .4 under the same parent. Source truth, raw
input decoding, allocator peaks, host algorithm time and future solver/file/decompression/geometry
work remain distinct obligations. The existing metadata bus has not changed its runtime behavior.

## Actual canonical-input accounting

charge_canonical_input_resources borrows an actual CanonicalInputs and charges its existing inventory
and Known-state evidence vectors into the supplied sink. It creates no replacement declarations,
looks up no registry, extracts no numeric availability and changes no value/state/provenance/order.

```rust
let mut meter = DesignResourceMeter::new(resource_limits);
charge_canonical_input_resources(&canonical_inputs, &mut meter)?;
let canonical_component = meter.finish()?;
let actual_entries = canonical_component.collection_entries();
```

The order is exact: charge the length inventory's full length, visit lengths in authored order and
charge each Known evidence vector, then do the scalar inventory and its Known evidence vectors.
Static fields are canonical.lengths, canonical.lengths.evidence, canonical.scalars and
canonical.scalars.evidence in Structure scope. Inventory charges have no index; evidence charges
retain the actual declaration owner identity and original zero-based declaration index. Reused evidence identities in different declarations
still occupy entries in both original vectors; accounting does not deduplicate them.

For example, the native actual inventory has five Length declarations covering Known, Assumed,
Unknown, Preference and Derived, plus six Scalar declarations covering those states and a computed
Area. The Known length and Known scalar each hold two references, reusing the same identities.
Demand is15 collection entries (5+2+6+2), zero text bytes and zero unbounded-counter bits.
Maximum14 refuses at canonical.scalars.evidence, index0, consumed13/requested2. Maximum4 refuses
first at the full length inventory (consumed0/requested5), before inspecting nested evidence.
Maximum6 admits those five records then refuses their first two-reference evidence vector.

An exhaustive internal owner view and exhaustive declaration/state patterns require accounting
updates when a new owned field/state is added. Present fixed value payloads must remain Copy; a new
owned dynamic payload requires revisiting the producer. Unknown/Derived states remain authored
states, with no value fallback. Empty actual canonical scope contributes zero entries; an earlier
meter refusal stays the exact original error, even if this producer's error is ignored.

Four actual canonical cases plus the previous six meter cases pass:10 total, with strict selected
lint and formatting. The command-bus source remains unchanged. This is canonical-component proof;
Pieces/topology, parameters/recipe/Materials, sewing/garment, root counter bits, source-host traversal
and runtime model admission remain separately owned. Finishing this component's meter supplies no
whole-model, source-truth, allocator RSS/time or publication certificate.

## Actual Piece and cut-plan accounting

charge_piece_inventory_resources borrows the actual Piece inventory. It precharges Pieces, then each
Piece's boundary, outer hole list, each hole's edge vector, construction lines and fold references in
schema/authored order. It charges exact unresolved Material-reason bytes, then name/size/fabric/colorway
label bytes. Assigned Material identity contributes no reason text; no registry lookup or physical
property inference occurs. charge_cut_plan_resources charges the actual physical-copy vector.

```rust
let mut meter = DesignResourceMeter::new(resource_limits);
charge_piece_inventory_resources(&pieces, &mut meter)?;
charge_cut_plan_resources(&cut_plan, &mut meter)?;
let represented_components = meter.finish()?;
```

The producers do not rebuild a Piece or copy against a later topology ledger. Actual held references,
assignment/label text and physical-copy order remain unchanged after success/refusal. Exhaustive
private owner/definition/label views and fixed Copy-reference constraints expose new owned fields.
This is content accounting, not current-reference, Material truth, geometry or release validation.

Nested locations now carry optional actual owner identity as well as scope/field/index. For a hole-edge
vector, pieces.holes.edges retains its Piece owner and original hole index. Other Piece member fields
retain owner and original Piece index. Whole inventories/copy vectors have no standalone entity owner.
Canonical evidence locations likewise carry actual declaration identity. No dynamic path string or
copied user label/value enters an error; identities supply location, not authority.

The native fixture holds two Pieces, each with boundary3, two hole vectors containing1 and2 edges,
construction1 and fold1. Piece entries total22, and two physical copies bring the total to24. Each
label is7 UTF-8 bytes; the first unresolved reason ` é ` adds4 bytes, while the second Piece is Assigned.
Text demand is18 bytes, with no trimming/deduplication. Maximum18 entries refuses the second Piece's
second hole vector with consumed18/requested2 and that actual Piece owner/hole index1. Maximum17
text bytes refuses its colorway with consumed16/requested2. Maximum23 entries admits Pieces, then
refuses cut_plan.copies with consumed22/requested2.

After a real split retires a held boundary source, rebirth would refuse that original Piece; accounting
still succeeds without repair, rewriting or changed ledger. Empty actual Piece/copy scopes contribute
zero entries and preserve an earlier meter refusal. Five new actual cases plus prior10 resource cases
pass:15 total, with strict selected lint/formatting and four meter privacy/sealing checks rechecked.
Topology history/index/registration, remaining model/source/runtime admission and total RSS/time proof
remain separately owned. The command bus has not changed its runtime behavior.

## Actual topology accounting

charge_topology_resources borrows the actual IdentityLedger. It charges the journal's complete
length before visiting entries in append order. Each Declared entry charges its held edge vector;
each Offset entry charges its fragment vector. Split, Merge, Reverse and Delete carry fixed Copy
payloads and add no nested collection entries. Their original values remain unchanged.

```rust
let mut meter = DesignResourceMeter::new(resource_limits);
charge_topology_resources(&identity_ledger, &mut meter)?;
let actual_topology = meter.finish()?;
```

After journal payloads, the producer charges the maintained live-edge set and registered-reference
map by their actual stored lengths. It does not replay the journal, recompute an index, resolve a
reference, rebuild repairs, mint an identity or rewrite a saved split choice. Exhaustive ledger/
journal/edit views and Copy index-key/value constraints require an update for new owned payloads.
The internal view's debug output contains counts, not journal values.

Whole fields are topology.journal, topology.live_edges and topology.registrations. Nested fields
are topology.journal.declared.edges and topology.journal.offset.fragments, with actual operation
owner and original zero-based journal index. Registration choices and retired-source references
remain part of the held model; accounting does not require them to resolve or certify integrity.

The native fixture performs Declare, Split, Merge, Reverse, three-fragment Offset and Delete, with
two registered references and a saved split choice. It retains6 journal entries,5 declared edges,
3 offset fragments,6 live-index entries (including an unused live edge) and2 registrations:22 logical
entries, zero text bytes and zero unbounded-counter bits. Maximum5 refuses the full journal6;
maximum10 refuses declaration6+5 at index0; maximum13 refuses offset11+3 at index4 with its actual
operation owner. Maximum19 refuses the live index14+6; maximum21 refuses registrations20+2.

Four new actual topology cases plus prior15 resource cases pass:19 total, with strict selected lint
and formatting. They retain the exact original ledger/choices on success/refusal, and empty actual
scope preserves a prior meter error. This closes Piece/copy/topology component accounting. Remaining
parameter/recipe/Material, sewing/garment, root composition/counter bits, source-host and runtime
admission work stay owned; no release, physical, integrity, allocator RSS/time or authority proof follows.

## Actual input metadata accounting

charge_parameter_resources precharges the actual parameter vector, then charges each held machine
token's exact bytes in authored order. Target references remain fixed metadata; it performs no
canonical target lookup or numeric-availability read. Refusals retain actual parameter owner/index.

charge_recipe_document_resources borrows both retained fields: exact original source, then actual
canonical syntax text. It does not call the parser, normalize literals again, regenerate identity or
evaluate operators. Both logical strings count even when their text could coincide. An empty source
still retains the eight-byte canonical identity `(recipe)`; it contributes8 bytes, not zero.

charge_material_inventory_resources precharges the actual Material vector, then each exact name
and provided composition. None contributes no composition text and supplies no inferred composition.
Property targets and optional shrinkage are fixed Copy metadata, with no duplicate canonical values/
evidence or applied physical scale. It performs no property, source or registry lookup. Locations
retain actual Material owner and authored index. Exhaustive private owner/text/definition views and
Copy guards expose future owned-field obligations, including canonical reference-vector payloads.

```rust
let mut meter = DesignResourceMeter::new(resource_limits);
charge_parameter_resources(&parameters, &mut meter)?;
charge_recipe_document_resources(&recipe_document, &mut meter)?;
charge_material_inventory_resources(&materials, &mut meter)?;
let actual_input_metadata = meter.finish()?;
```

The native fixture has two parameters with tokens `a` and `bb` (3 bytes). The original recipe text
is24 bytes including outer whitespace; its retained canonical identity is36 bytes. Two Materials
have names of3 and4 bytes; the first provided composition ` é ` adds4 bytes and the second is None.
Combined demand is4 collection entries,74 UTF-8 bytes and zero unbounded-counter bits. Actual
Unknown/Derived width/flag/shrinkage references remain unchanged, without inferred property values.

Maximum2 text bytes refuses the second token at consumed1/requested2. Maximum23 refuses recipe.source
at0+24; maximum62 after tokens refuses recipe.canonical at27+36. Maximum6 on Materials refuses the
first composition at3+4; cumulative maximum73 refuses the second Material name at70+4. Collection
maximum3 after parameter entries refuses materials at2+2. Owners/indices distinguish member fields;
recipe strings and whole inventories have no standalone entity owner.

Five new actual cases plus prior19 resource cases pass:24 total, with strict selected lint/formatting.
They preserve canonical inputs, parameter order/targets, both recipe fields and complete Material
intent. Empty parameter/Material drafts plus empty recipe source demand0 entries/8 bytes/0 bits;
a prior meter refusal stays original across all three producers. Remaining sewing/garment, root/
source and runtime admission work is separately owned; this is no binding, recipe evaluation,
Material truth/physical, whole-model or allocator RSS/time certificate.

## Actual sewing accounting

charge_sewing_graph_resources precharges the actual graph's span vector, then each span's weighted
region vector (when Weighted) and stop list in authored/schema order. Uniform and BetweenNotches
carry no region vector. Fixed side, direction and Explicit/Formula/Profile amount metadata stay
unchanged; the producer does not read a symbolic value, look up a copy/landmark, resolve an anchor,
validate current contexts or realize ease/geometry.

charge_sewing_landmark_resources charges the provided actual Notch/TurnPoint registry slice. Root
composition must supply its exact owned vector. Exhaustive private views and fixed Copy-payload
constraints cover each held piece/anchor/id/profile representation; bindings remain symbolic, with
no inferred notch style, sample/production dimensions or encoding. No landmark is reborn or resolved.

```rust
let mut meter = DesignResourceMeter::new(resource_limits);
charge_sewing_graph_resources(&sewing_graph, &mut meter)?;
charge_sewing_landmark_resources(&landmarks, &mut meter)?;
let represented_sewing = meter.finish()?;
```

The native graph has3 spans covering Uniform/Weighted/BetweenNotches and Explicit/Profile/Formula
amounts. Its weighted span holds2 regions, the spans retain1/2/2 stops, and the registry holds two
Notches plus one TurnPoint:13 collection entries, zero text bytes and zero unbounded-counter bits.
Maximum2 refuses sewing.spans (requested3, actual graph owner). Maximum3 refuses the first stop list
at3+1, span owner/index0; maximum5 refuses the weighted vector at4+2, weighted span owner/index1.
After10 graph entries, maximum12 refuses the3-entry sewing.landmarks field. Empty actual graph and
registry contribute zero and cannot clear an earlier meter refusal.

After a real deletion retires a held boundary source, the same graph/landmark accounting still succeeds
without rebirth or resolving the now-orphaned anchors. Exact graph/landmarks/ledger and profile intent
remain unchanged on success/refusal. Four new actual cases plus prior24 resource cases pass:28 total,
with strict selected lint and formatting. The first compile caught a missing public re-export, which
was corrected before these checks passed. Garment families, root/source composition and runtime
model admission remain owned; these counts supply no binding, realized ease, geometry, physical,
release, whole-model or allocator RSS/time certificate.

Focused runtime evidence comprises37 earlier bus cases, eight runtime-control cases and one actual
source-copy case:46 passing cases. The evidence-input slice established six pure admission/fraction
cases and control privacy. The copy slice passes strict selected lint/formatting and11 affected
compiler privacy cases, including refusal of all three private consuming helpers. Full command
integration and deeper source/allocation acceptance remain separately owned. These input units and
copy counts do not establish total model/work/memory or future numeric/file/decompression/geometry/
solver bounds; that remaining resource scope is G1-SLICE.6.8.2.2.2.2.
