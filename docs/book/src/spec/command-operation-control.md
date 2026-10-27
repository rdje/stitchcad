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
G1-SLICE.6.8.2.2.2. This native source-copy case brings focused bus coverage to46 cases; it is not a
complete resource, performance or integration certificate.

Focused runtime evidence comprises37 earlier bus cases, eight runtime-control cases and one actual
source-copy case:46 passing cases. The evidence-input slice established six pure admission/fraction
cases and control privacy. The copy slice passes strict selected lint/formatting and11 affected
compiler privacy cases, including refusal of all three private consuming helpers. Full command
integration and deeper source/allocation acceptance remain separately owned. These input units and
copy counts do not establish total model/work/memory or future numeric/file/decompression/geometry/
solver bounds; that remaining resource scope is G1-SLICE.6.8.2.2.2.
