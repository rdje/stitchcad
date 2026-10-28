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
// The host supplies all four validated positive maxima explicitly.
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
before costly metadata operations, alongside explicit model limits below. No request, observer or
constructor supplies a hidden default.

```rust
let mut bus = CommandBus::with_edit_capacity(
    garment, trusted_authority, reserved_identities, maximum_committed_groups,
)?;
bus.install_operation_limits(CommandOperationLimits::new(
    maximum_members, maximum_metadata_targets, maximum_assignment_reason_bytes,
    maximum_measurement_evidence_references,
))?;
bus.install_model_limits(model_resource_limits)?;
let result = bus.commit_group_with_progress(&group, &observer)?;
```

Operation/group-capacity maxima use explicit positive NonZeroUsize values; model counter bits use
NonZeroU64, with positive NonZeroUsize entry/text maxima. Installation changes
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
they enforce both installed policies. Their explicit observer counterparts are preview_with_progress,
preview_group_with_progress, commit_group_with_progress and apply_history_with_progress. Availability
queries for preview/commit handlers require both installed policies; availability supplies no permission,
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
Complete actual model roots compose their source content and retained census under one sink.
Current/preparation and history-alternative admission now uses the required readonly source port;
exact Measurement/Material projections are implemented; Notch/revision and retained command-state admission remain pending. The meter alone is not a runtime model-admission gate.

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

DesignResourceScope identifies Structure, Inputs, Sewing, Garment or SourceHost. Locations retain an
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

[Complete immutable source accounting](source-resource-accounting.md#complete-immutable-source-host-port)
uses actual retained host fields; it exposes no default, source-currentness or authority bypass.

Actual component examples and refusal witnesses are in
[Actual model resource accounting](model-resource-accounting.md).

Focused runtime evidence comprises37 earlier bus cases, eight runtime-control cases and one actual
source-copy case:46 passing cases. The evidence-input slice established six pure admission/fraction
cases and control privacy. The copy slice passes strict selected lint/formatting and11 affected
compiler privacy cases, including refusal of all three private consuming helpers. Full command
integration and deeper source/allocation acceptance remain separately owned. These input units and
copy counts do not establish total model/work/memory or future numeric/file/decompression/geometry/
solver bounds; that remaining resource scope is G1-SLICE.6.8.2.2.2.2.

## Installed current-model admission

CommandBus starts with no model limits. The trusted composition root calls install_model_limits
exactly once with explicit DesignResourceLimits: cumulative collection entries, exact logical UTF-8
bytes and represented unbounded-counter magnitude bits. The setter does not traverse the model or
call its source host. A second installation returns ModelLimitsAlreadyInstalled without replacing
the first policy. Missing policy returns ModelLimitsNotInstalled after pure input admission; authorized
GetPieceList inspection needs neither policy. Neither requests nor progress observers carry a policy.
Compile-time H: DesignResourceSource is required for costly metadata methods; readonly query paths
retain the narrower semantic host contract. Compiler privacy rejects direct model-policy reset.

For single/group metadata preparation, the complete actual current or previously staged garment is
charged under the installed model policy at CurrentValidation0, before envelope/current validation
or a model clone. The walk follows initial exact authority and Design/revision binding. Thus an
authority, Design or revision refusal never triggers a private resource walk or exposes a quota
witness. Runtime model refusal retains CommandControlError::Model and the original scope/owner/
field/index/consumed/requested/maximum resource error through standard error.source chains.

History traversal admits both the current actual model and the exact stored alternative before each
snapshot's semantic validation, after current-operator authority and exact snapshot binding. Successful
Undo/Redo moves existing owners. Exact accepted authoring/history replays keep their original result
and skip private model walks; accepted request identity and fresh authority checks still apply.
Cancellation remains at actual pre-effect checkpoints; no effect, reserve, revision, stack, key or
accepted record is published on refusal. is_preview_available and is_commit_available require both
policies; availability supplies no remaining-quota, permission or candidate-validity certificate.

Four new cases instrument a wrapper delegating to actual DesignSources, without invented model or
resource totals. Unauthorized and mismatched snapshots produce0 clones/0 resource calls/0 semantic
callbacks across single/group preview and commit; history denial/stale snapshot does likewise.
An actual one-byte-short model policy preserves the exact native resource refusal across all three
preparation entries, with0 clones/1 actual host resource call/0 semantic callbacks and unchanged
Piece address/revision. Setup/cheap reads invoke none. Actual history traversal charges2 snapshots
with0 host copies; exact history/authoring replays invoke none. The existing46 cases plus4 new cases
pass50, with strict core-library/affected-test lint and formatting; focused bus privacy/capability
examples also pass. The new test's missing explicit borrowed-error lifetime and one unused import
were corrected before acceptance. No semantic validation callback is replaced by accounting.

This implements current/preparation-root and stored history-alternative admission. Exact prospective
Measurement evidence and selected Material reason admission are implemented below; Notch/revision
growth remains .4.2.3/.4.2.4; retained
command audit/replay/history content and maximum-issued/audit counters remain .4.3. This partial
runtime stage does not certify every newly published model or total retained state fits the policy.
Raw ingress, allocator peaks, host algorithm time, solver/decompression/numeric work and physical/
release acceptance remain separate. Deeper/full verification stays deferred until feature completion.

## Exact prospective Measurement resources

ChangeMeasurement first retains current-model admission, original envelope/current validation and
actual Measurement binding. A mismatched canonical declaration still returns its original identity
error. Before the first candidate model clone, the runtime borrows the replacement declaration at
that exact existing canonical slot and charges the complete prospective garment under the same
installed model limits. The original slot/index and all other owned inventories, source snapshots,
retained census, text and revision remain part of the walk. No candidate data is installed or cloned.

For Known state, only the replacement's evidence vector is charged at that slot; old and new evidence
are not added together. Unknown/Derived and the other fixed states own no evidence vector there.
All unrelated canonical declarations keep their original authored order, ownership and charges.
Public current-model accounting still uses the same implementation with no replacement. The
projection is private to core; requests cannot supply resource totals or substitute another meter.
Subsequent actual candidate construction and full current validation still run after admission.

For example, let the actual current model have N collection entries, including2 evidence references
in the bound declaration, and set the model entry maximum to N. Replacing them with3 references
projects N+1 and refuses with the exact first error returned by a real rebuilt-model resource walk.
There is no candidate host copy. Replacing them with2 references or submitting the exact no-op fits N;
changing to Unknown projects N-2. All unchanged text and counter units retain their exact values.

Three new native cases compare the borrowed projection with independently rebuilt, validated actual
models. They verify the growth refusal across single/group preview and commit, unchanged Piece
address/revision and reuse of refused group/key/capacity by a valid same-size replacement. Current
and prospective source walks occur without a candidate copy on refusal. Admitted previews retain
one actual host copy. A group that shrinks then restores evidence has no net metadata difference;
an exact alias reuses the prior report and repeats neither candidate clone nor resource walks.

The first oracle fixture used zero for a Measurement that also supplies Material width; actual
Material validation refused it. An explicit positive1000 micrometre length makes the fixture isolate
evidence accounting while preserving that production validator. An assumed test convenience accessor
was replaced with the actual difference getters. The new3 and original4 Measurement cases,33 source
resource cases and6 cases selected by the canonical filter pass, with strict affected lint/formatting.
Authorization/group/effect/history implementations are byte-exact in this slice. Selected Material
projection follows below; Notch appends, publication revision growth and retained content remain owned;
this feature stage supplies no complete new-model/retained-state, raw ingress, allocator memory/time,
physical or release certificate. Deeper/full verification remains deferred until feature completion.

## Exact prospective selected Material resources

AssignMaterial retains current-model admission and original envelope/current validation. It checks
every actual selected Piece in the authored selection order; a missing later target keeps the same
MissingPiece identity/index before any prospective walk. It then borrows the selection and proposed
MaterialAssignment and charges the complete prospective garment before the first Piece/reason/model
copy. The private exhaustive projection enum carries borrowed data, without supplied totals or a
request-controlled callback, meter or public model replacement capability.

At each original Piece slot, the projection retains its labels, topology collections and original
owner/index. Selected Pieces contribute only their resulting unresolved reason text; unselected
Pieces retain their original assignment and exact text. Assigned references add no owned reason or
new Material object. Repeated proposed spelling counts per selected Piece; it is not deduplicated.
The full actual source host, captured metadata census and every other model field still count in
schema order. Subsequent actual Piece/candidate constructors and current validation remain required.

For example, two selected Pieces retain reasons ` é ` and `猫`:4+3 UTF-8 bytes. Replacing both with
`猫猫` gives6+6 bytes, so the prospective model has5 more bytes. A model text maximum equal to the
current demand refuses with the same first error as an independently rebuilt actual model, before
any candidate host copy. Replacing the first reason with `éé` preserves its4-byte demand; replacing
it with `猫` removes1 byte. Assigning both to an actual registry Material removes7 reason bytes.
An exact no-op fits its original limit. Unselected labels/reasons and other model units are unchanged.

Four new native cases compare with rebuilt, validated actual models. They cover growth refusal on
single/group preview and commit, unchanged original Piece address/revision/content and later reuse
of the refused group/key/capacity by a valid commit. They also cover no-op/same-byte/shrink/Assigned,
reversed selection/report order, missing target index1 before projection, and a staged shorter reason
followed by restoration and an alias. The alias repeats neither candidate copy nor resource walk;
restoration has no net metadata difference. Admitted previews retain one host copy and two source
resource walks. Python and OS byte counts independently confirm the exact4/3/6-byte strings.

The new4, original5 Material and3 Measurement-projection cases,33 source resource cases and5 Piece
cases pass, with strict affected lint/formatting. The execution guide's earlier semantic-only preview
host bound is corrected to the required DesignResourceSource contract. Notch appends, publication
revision growth, retained command state and deeper/full verification remain owned. These projections
supply no complete retained-state, raw ingress, allocator memory/time, physical or release certificate.
