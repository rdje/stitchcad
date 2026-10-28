# Prospective retained publication

Authoring commits now admit their exact future private retained artifact state before publication
construction. This extends [current retained admission](command-resource-accounting.md#actual-current-private-retained-composition)
and the [per-model projections](command-operation-control.md#exact-prospective-measurement-resources).
The authoring implementation has focused checks. Prospective history-action admission remains
G1-SLICE.6.8.2.2.2.2.4.3.4.2; coupled and deeper/full verification remain separately owned.

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
Prospective history .4.3.4.2 and coupled/deeper/full acceptance remain pending. This supplies no
raw-ingress, allocator-peak, arbitrary-time, numeric/file/decompression/geometry or physical/release
certificate.
