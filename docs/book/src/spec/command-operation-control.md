# Operation limits and progress

The [command-layer contract](command-layer.md#6-structured-errors-and-progress) requires bounded
inputs, declared progress and atomic cancellation. The [execution guide](command-execution.md)
describes implemented metadata operations. This page currently covers input admission and progress
data/observer contracts. Runtime installation, emission and cancellation wiring remain pending.

## Explicit finite metadata admission

CommandOperationLimits::new takes three explicit NonZeroUsize maxima. There is no guessed default.
check_mutation and check_group borrow original typed inputs and return CommandOperationDemand or
the first CommandOperationLimitError. They perform no model, registry, authority or history lookup.

| Unit | What counts |
| --- | --- |
| members | every original submitted member, including an exact retry alias |
| targets | one Measurement, selected Material Pieces, or one Notch-owning Piece |
| reason_bytes | exact UTF-8 unresolved reason bytes for each proposed selected Piece assignment |

These are declared metadata input units. They are not observed CPU time, total model memory,
temporary allocation peaks, numeric complexity, file/decompression bounds or geometry/solver work.
Unavailable SplitSeamSpan/CloseDart handlers contribute no implemented metadata targets or reason
bytes; their submitted members still count. Admission supplies no handler availability or execution.

```rust
// The host supplies all three validated positive maxima explicitly.
let limits = CommandOperationLimits::new(
    maximum_members, maximum_metadata_targets, maximum_assignment_reason_bytes,
);
let demand = limits.check_group(&group)?;
let submitted_members = demand.members();
let primary_target_selections = demand.targets();
let proposed_assignment_reason_bytes = demand.reason_bytes();
```

For example, assign the nonblank reason `é` to two Pieces, change one Measurement, then submit an
exact alias of that first assignment. The three submitted members contain five target selections
and eight proposed reason bytes: the reason has two UTF-8 bytes, copied into two proposed assignments
per submitted assignment member. The alias still counts for input admission; runtime replay semantics
remain separate. The native contract case admits exactly 3 members, 5 targets and 8 reason bytes.

Member-count excess is checked first. Then each original member checks targets before reason bytes.
Members retains maximum/requested counts. Targets retains maximum, preceding count, this member's
count, original zero-based index and payload-derived command. ReasonBytes retains maximum, preceding
bytes, selected copy count, exact bytes per copy, original index and command. The exact multiplicative
components remain available even when a requested total would exceed the platform count range.

The check compares against remaining room before adding or multiplying. It neither wraps, saturates,
clips input nor silently deduplicates members. Original contexts, payloads, identities and order remain
unchanged. Private limit fields expose immutable getters; compiler privacy rejects direct widening.
An admitted demand is data, not authority or a commit capability.

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
present in progress data. Runtime must emit actual phases before effects and honor cancellation atomically.

The four native admission/fraction cases and compiler privacy check pass focused validation, with
strict selected lint/format checks. Existing bus source is unchanged by this leaf. Installation of
immutable trusted limits, real progress checkpoints, atomic cancellation/refusal and coupled runtime
acceptance remain G1-SLICE.6.8.1.2 work. These contracts do not claim that runtime cancellation is wired.
