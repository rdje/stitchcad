# Prospective retained publication

Authoring commits now admit their exact future private retained artifact state before publication
construction. This extends [current retained admission](command-resource-accounting.md#actual-current-private-retained-composition)
and the [per-model projections](command-operation-control.md#exact-prospective-measurement-resources).
Authoring and history-action projections now have focused checks. Coupled/deeper/full verification
remains separately owned.

**P0 D293 — source ownership repair required before resource signoff.** A reproducible custom source
adapter preserves semantic Clone equality while its honestly charged immutable cache grows in each
new clone. Its forecast admits907 bytes but the final model retains908; the next current meter refuses
882+26. This is a defect, not dishonest resource totals. The next owned leaf .4.3.4.3 must freeze source
ownership across model copies and close this case before .4.4 acceptance. The concrete fixture checks
below establish the projection logic with stable source copies; they do not close D293.

## What is admitted, and when

A new group first passes explicit input policy, fresh complete authority, exact Design/revision
binding, finite journal room and current retained admission. It prepares the actual sequential
metadata candidates and original reports under the existing per-model limits. Exact aliases reuse
the first execution result, while their owned request/report copies still count. The final prepared
candidate, actual group report and original request are borrowed by the private publication projection.
No caller-supplied totals, model replacement, grant, shadow report or admission token is accepted.

The existing maximum-issued model projection runs first. Then RetainedPublication emits0/1 before
and1/1 after the exact future retained check. Only after that check may the runtime construct the
successor revision/effect sequence, build the final revision model, prepare reserve/journal/history
allocation, clone retained requests/reports and prepare the returned result. The final complete Commit
grant is refreshed after all preparation, immediately before the existing atomic effect. Resource
admission cannot substitute for that fresh authority check.

```rust
// The composition root installs all three explicit immutable policies beforehand.
let accepted = bus.commit_group_with_progress(&actual_group, &observer)?;
let original_record = accepted.entry();
```

Temporary metadata preparation is already bounded by submitted operation and per-model limits; it
can construct temporary candidates and reports before this publication gate. These are not added to
the final retained-state total. The gate bounds the owner's future retained artifacts and precedes
publication-side growth. Caller-delivered result copies are separate owned data, measured by the
published result readers; they are not a second private journal entry. Allocator peaks and arbitrary
host algorithm time remain outside the logical-content guarantee.

## Exact future composition

The projection preserves the [current composition's schema order](command-resource-accounting.md#actual-current-private-retained-composition):

1. Charge the actual prepared candidate with its future revision, forecast from the journal's
   independent maximum issued. The restored current revision never supplies the next issue value.
2. Charge the complete immutable identity backing once. Cursor advancement changes no backing length.
3. Charge the actual journal initial revision, forecast next maximum and next effect sequence.
   Precharge the whole prospective accepted-entry vector, walk all existing records unchanged,
   then charge the new record's forecast sequence, exact original request, actual prepared report,
   forecast committed revision and actual redo-discard ID vector.
4. If history is configured, precharge the resulting undo deque. At depth, omit exactly its actual
   oldest entry; retain the other alternatives in order and append the actual previous current model.
   Charge an empty redo vector because new authoring discards that branch. Keep all existing action
   records unchanged and in chronological order. Without history, no old model snapshot is retained.

The new commit's discarded-redo IDs still count, in redo application order, even though their model
alternatives leave the owner. The fixed oldest-undo ID contributes no dynamic entry. The projection
neither adds discarded models to the future budget nor subtracts unrelated existing records. Thus
exact fitting publication is not rejected by a conservative old-plus-new total. All represented
counters and every owned request/report/model copy remain separate charges, even when values match.

Private bus, journal, history and history-step fields are matched exhaustively. Fixed identities,
limits and configuration remain guarded. The source H in each actually retained model is charged
through the required honest/exhaustive readonly port; authority service A remains outside the
artifact scope and is never called by accounting. Current model, request and report producers retain
their original field locations. New and existing record fields use the same static group/Design
owners and original member coordinates. Retained refusal preserves CommandControlError::Retained
and its original resource cause through the normal error source chain.

## Counters and collection boundaries

Successor magnitude is read without advancing, cloning or formatting the actual counter. Both the
maximum-issued DesignRevision and the independent CommandAuditSequence use the exact pinned integer
API forecast already used by model admission. If the magnitude count needs2^64 units, the shared
producer charges two exact chunks at the original location; no finite u64 policy admits both.
The counter value itself gains no hard cap, wrap, saturation or replacement policy.

Appended journal/undo vectors normally precharge their complete prospective length in one call.
The platform count boundary likewise uses exact chunks at the same location rather than wrapping
or clipping. A primitive boundary test exercises this refusal without inventing an actual enormous
vector or model. Audit-counter tests compare forecast with actual successor arithmetic at0, carries,
u128::MAX and the following represented value. These primitive checks are not huge-workload claims.

## Native exact-limit examples

Let M and B be the independently measured entries and text bytes of the real revision0 model used
by the native fixtures. Both Pieces initially retain26-byte reasons. They are ordinary actual models
and source records, independently rebuilt and validated through public constructors.

A two-member group assigns reason ` é ` to one Piece and submits an exact alias. The accepted record
owns both original payloads and both report snapshots. With two reserved IDs and undo enabled, future
retained demand is2M+13 entries,2B+76 bytes and5 counter bits. Exact limits accept the same immutable
record as an unrestricted reference owner. One-short entry, text and bit limits retain the same first
error as an independent composition of actual models and returned records. Count/text refusals leave
the original group/key available for a smaller actual no-op retry. A policy below the minimum first
publication counter demand can admit no first effect; it is never silently widened.

In these single-distinct-member fixtures, retained quota refusal or cancellation has1 bounded
preparation source-host copy and no final publication-model copy. Successful authoring has2 copies:
that temporary candidate and the final revision model. Larger groups retain the existing one-copy
per distinct prepared member behavior; aliases do not prepare another candidate. No general allocator
peak or physical byte size is inferred from these logical/copy counts.

Three one-Piece assignments with undo depth1 produce a final retained demand2M+21 entries,2B+60
bytes and30 bits. The exact final budget admits the third act while reporting the second group's
oldest-undo eviction. The projected future traverses only its two retained models; it does not keep
the dropped alternative in the budget. Source calls at this third commit are7: two current-retained,
three individual current/metadata/revision projections and two prospective-retained calls.

The branch fixture uses an actual authored Known512-evidence vector, shrinks it to explicit Unknown,
changes a Material, then undoes both groups. A new Material branch restores the original Known form
as part of its real base snapshot. It discards both redo alternatives while retaining both group IDs,
all original accepted records and both history-action records. If K=512, its future entry demand is
2M+2K+29, text demand2B+110, and counter demand36. The evidence size makes the exact final entry
maximum also admit the earlier redo state; text policy explicitly admits both earlier and final
states. One-short entry and bit limits refuse with the original independent-oracle error. The new
revision is3 and sequence5 despite the current revision having been restored to0. Source calls are8:
three current-retained, three individual model projections and two prospective-retained calls.

A no-history owner starting at revision u128::MAX needs1029 total future retained magnitude bits
for a one-member no-op publication. The actual returned record alone owns642 bits, independently
charged by the public record reader. At1028, records.commit.revision refuses with consumed900 and
requested129; the original revision and model remain. No issued value or final model is constructed
before this refusal. These are represented magnitudes, not counter-value ceilings.

After a real Notch consumes one of two reserved IDs, a following no-op would retain three models
and require3M+14 entries. A bound3M+13 refuses before publication, proving that both backing slots
still count rather than only the unused suffix. The earlier current-only later-over-bound proof has
been replaced by this early-growth-refusal case.

RetainedPublication cancellation at either endpoint preserves the original group/key/capacities and
snapshot. Revocation after successful resource admission still fails the fresh final authority check;
the completed candidate/final models are discarded without an effect. A later authorized retry can
use the original group/key. Exact accepted replays retain their original result and bypass private
resource walks, preserving the existing audit/retry contract.

Five authoring cases, the three affected current-retained cases, one actual revision-successor case,
one audit-successor case and the two primitive count-boundary cases pass with strict affected lint
and formatting. Publication-order/source checks verify that counter construction, final model,
reserve/history/request/report allocations, fresh authority and effects follow the new gate.
D293 ownership repair and coupled/deeper/full acceptance remain pending. This supplies no
raw-ingress, allocator-peak, arbitrary-time, numeric/file/decompression/geometry or physical/release
certificate.

## Exact prospective history actions

After complete current-operator authority, exact original accepted-record binding, current revision,
stack target and finite action-room checks, the history path admits current retained content and
validates both actual model snapshots. RetainedPublication then checks the exact future owner before
cloning its before/restored counters, issuing the next sequence, collecting effective commands,
cloning requests/results or reserving stack/action storage. Initial operator-authorization metadata
is separate bounded input preparation. History never constructs another model or invokes H.clone.
The fresh final complete grant still follows all record preparation and precedes the actual move.

The private projection borrows the actual target, request, accepted record, journal and history.
It charges the restored target as the new current model and the complete identity backing. Journal
initial revision and maximum issued remain unchanged; only the effect sequence is forecast. Existing
accepted authoring records remain unchanged. The source stack omits exactly its actual last step;
the destination stack keeps its original order and appends the previous current model. Undo uses
undo-back to redo-back; Redo uses redo-back to undo-back. There is no inverse edit, clipping to depth,
second copy of the restored target or new authoring revision.

The whole action vector is precharged at its new length, followed by all original actions and the
exact new action. New data is the forecast sequence, actual effective-command vector, complete
original history request, actual before revision and actual restored revision. Commands come from
the accepted report's first-occurrence steps; aliases remain in the copied original request. The
read-only count/filter borrows already admitted accepted steps before the new vector is allocated.
Actual source H is charged once per retained model, in the same schema order and original locations
as current admission. No replacement model, supplied total, caller token or authority callback occurs.

A real agent authoring group with a Material assignment and an exact alias is undone by a separately
authorized human. With two reserved IDs and model demand M/B at revision0, the future owner is
2M+19 entries,2B+84 bytes and10 bits. The copied request retains both payloads, while the effective
command vector has one entry. Exact entry/text/bit maxima match an independent composition of real
models and returned immutable records. Each one-short policy returns the same original error and
source chain before publication copies. Repeating a refused request returns that original cause;
its key/action slot/counter and current Piece address remain unchanged. Exact authorized replay
still returns the first action at action capacity, with zero private resource calls.

A second fixture uses three unequal actual Material snapshots. Its second reason is64 bytes, so
incorrectly retaining the wrong redo member changes real counted text. After Undo group2, Undo
group1 and Redo group1, the future owner is3M+28 entries,3B+352 bytes and36 bits. The current
snapshot is revision1, undo contains revision0 and redo still contains revision2. Exact limits and
one-short text/bit refusals match the independent actual-owner oracle, with0 H clones. The final
Redo uses8 source calls: current retained3, individual current/target2 and prospective retained3.

A no-op authoring act starts at u128::MAX−1 and issues the all-one maximum u128::MAX. Undo restores
the former value while preserving that maximum. It needs1797 future retained bits; its actual new
action owns642 bits. At1796, the original error refuses before retained counter/request/command
copies. Advancing or forecasting another maximum-issued revision during history would incorrectly
reject the exact case. The actual values are large representations, not numeric counter ceilings.

RetainedPublication0/1 cancellation preserves model addresses, stack membership, the original key
and the sole action slot. Revocation after resource admission still fails the fresh final authority
check without any move. A later authorized retry succeeds. The projection invokes no H.clone,
including successful history: existing model owners move between current and stack positions.

Four new history cases, four current-retained cases and four affected model-admission cases pass
with strict affected lint/formatting. Initial over-bound owners are refused by current admission;
accepted history growth is now checked before publication. Logical composition is implemented with
focused checks, while the separately reproduced D293 source-clone ownership defect remains P0 and
blocks feature signoff. Coupled/deeper/full verification follows that repair. These results establish
no raw-ingress, allocator-peak, arbitrary-time, geometry/physical or release certificate.
