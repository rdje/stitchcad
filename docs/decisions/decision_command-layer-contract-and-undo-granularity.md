# The command layer: five classes, one undo granularity, and authority as a permission on a class

- **Type:** `decision`
- **Date:** `2026-09-30` (absolute); roadmap §4.4 requires undo/redo semantics to be defined **at G0**, and
  §7.8 requires the authority levels to be scoped from day one
- **Status:** `active`; all17 typed inputs, trusted-host authorization, Piece-list read and Measurement/Material/Notch metadata preview/atomic commit/bounded actual undo-redo implemented; persistence/full bus pending
- **Owner / source:** leaf `G0-CONTRACT.17`, from roadmap §4.4, §7.8, §10 and §11's G0 and G1 exit clauses;
  the normative text is `docs/book/src/spec/command-layer.md`

answers: "what is the undo granularity?" · "why is an evaluation not undone?" · "what does a preview guarantee?" · "can an agent approve a release?" · "how is UI/API/MCP parity proved?" · "why is the parity table empty?" · "what would reopen the command-layer contract?"

## The decision

**Everything mutates through one typed command layer, and the contract is fixed at G0 in five parts:** the
command set is partitioned into five classes (`query`, `mutation`, `evaluation`, `artifact`, `release`);
**the undo granularity is the atomic group** — one undo reverses one group, never part of one; mutations
carry a revision precondition and an idempotency key; the five authority levels of roadmap §7.8 are
permissions on *classes* of commands, enforced in the core, with `approve` unholdable by an agent; and
UI↔API↔MCP parity is proved by a **generated** coverage table whose columns and generation rule are
normative now and whose rows are G5's evidence.

## The decisions, and the alternative each rejected

| Decision | Rejected | Why |
| --- | --- | --- |
| undo granularity is the atomic group | per-command undo | "add a dart" is a centre, two legs, an intake and a truing; undoing three of the four leaves a piece nobody drafted |
| an undo restores semantics — recipe, identities, revision — not contours | contour-level undo | two designs with identical contours and different recipes are different designs (ontology §9), so a contour undo can silently change which one is on screen |
| evaluations and artifacts are discarded, not undone | undoing a generated instance | a derived result has no history to reverse; recomputation is its inverse, and an undo stack full of instances hides the design's own |
| undo history is not canonical content | persisting the stack | a reopened project would inherit a history nobody present can vouch for; the trail (§5) is what persists |
| preview needs only `inspect` | preview as a privileged operation | a patternmaker must see the result before accepting it, and an agent must be able to propose without being able to commit |
| a commit re-checks the precondition a preview passed | trusting the preview | three front-ends and an agent edit one design; the preview is a statement about a revision, not a reservation |
| idempotency keys on every mutation | retry-by-repeat | a transport timeout must not become a second edit, and a replay is reported as a replay rather than silently deduplicated |
| authority is a permission on a class, enforced in the core | per-tool permission lists | a tool manifest is data an adapter ships; the core is the only place three adapters cannot disagree |
| the parity table is generated from a workflow registry and the command table | a hand-written table per adapter | three people maintaining one table produce three tables, and the drift is invisible until a user finds it |
| parity is claimed over **workflows**, not commands | per-command parity | three adapters may group commands differently and still be at parity; an adapter exposing a command no workflow uses has added surface, not capability |

## Why the parity table is empty

Its columns and generation rule are normative, and its rows are evidence about software that does not exist
yet: the adapters land at G1 (`sc-api`, `sc-mcp`) and G5 (the shells). A table filled in at G0 would be a
claim about unwritten code, and an `absent` cell would be indistinguishable from a guess. So the chapter
declares the shape, the closed cell vocabulary (`absent`, or an entry-point name with a test id), and the
rule that a workflow is marked present only when a test completes it through that adapter — and G5 populates
it mechanically. The same reasoning left the canvas spike's `results.tsv` empty at `G0-CONTRACT.11`.

## The one number deliberately not written

The undo **depth** is declared to exist, to be bounded, and to report when it drops the oldest group — but
its value is not set here. It is a resource limit, and roadmap §10 requires resource bounds (solver work,
file sizes, decompression) at G1, where a measurement exists to derive one from. A number written at G0
would be a guess in a normative font, and a guess that later has to move is a migration.

## How to apply

- `G1-SLICE.6` implements the bus against this chapter and enforces "no mutation path bypasses it" by
  module privacy plus a test that the domain types expose no public mutator.
- A new command arrives as a row in §1.1 first: class, authority, reversibility, granularity. The census
  refuses a row whose values are not in the vocabularies §1 and §7 declare, so the table cannot drift into
  prose.
- A new authority level is a governance change, not a chapter edit — the census refuses a sixth.
- The parity table is generated; an adapter gap is an `absent` cell with a reason and an owning leaf, which
  is the envelope's shape for a deferred row.

## What would reverse it

The transport timing in condition (3) below is superseded by the director-requested roadmap v0.4
amendment in [decision_mcp-transports.md](decision_mcp-transports.md). Profile-specific evidence may
admit a connection from G1; the authority levels and core enforcement above remain normative.

Three conditions, each needing a recorded decision: (1) a real workflow that cannot be expressed as an
atomic group — collaborative editing with per-field concurrency, for instance — which would make the group
the wrong undo unit and needs its own contract; (2) evidence from G1 that a bounded undo depth cannot be
chosen without a persistence-format change, which moves the bound into the storage contract; (3) a
threat model (roadmap §10, G5+) that permits a non-stdio MCP surface, which re-opens how authority is
carried across a network boundary but not what the levels are.

## Native implementation continuity

[Whole0291–0430 implementation records](../history/stitchcad-devnotes-part583.md) retain their original answers and proof boundaries.

## Sequential metadata group preview0432

answers: "how do metadata groups stage actual models and retries?" · "how is the net group delta derived?"

G1-SLICE.6.5.4.2.2.3 uses one actual trusted authority snapshot/complete Inspect grant, then binds
actual Design/exact revision. Each first request consumes the preceding validated candidate and one
independent concrete reserve cursor. Per-member original guards and typed error sources remain;
a failure retains original group/index/command and discards every staged candidate/report.
Within one preview only, complete CommandMutation equality under a repeated key returns the first
report with replay_of; conflicting reuse refuses before reapplication. No persistent replay is claimed.
The final diff reads original/final actual models, permits only successful nonalias semantic footprints,
preserves complete other owner fields/order and omits restored values. Native mixed progression,
restoration/nonempty deltas, exact aliases/conflicts, later anchor/exhaustion, source refresh and
binding order pass in six group cases. Publication/audit/undo/evaluation remain their later owners.

## In-memory metadata publication0434

answers: "when is a metadata group published?" · "how does exact retry work after revisions advance?" · "what bounds the in-memory audit?"

G1-SLICE.6.6.1 requires explicit finite accepted-group capacity. Complete Commit authority and Design
binding precede journal lookup. Exact group id plus complete original group equality returns the first
immutable result after fresh authorization, before revision/capacity checks. Changed same-id groups
refuse; keys belong to this Design/complete-actor namespace, and cross-group overlap refuses rather
than replaying or publishing part. Journal entries are never discarded at capacity; new groups refuse.
New edits require exact current revision; original mismatch carries actual available advances/commands
and initial journal revision, with no fabricated earlier trail. Prepare/rebuild/current-check the actual
successor model and reserve/allocation preflight, then refresh complete Commit authority immediately
before infallible exclusive model/reserve/audit moves. Retain old host through the complete transition.
One accepted group adds one immutable original actor/request/result/grant/time entry and advances once,
including net no-op. Exact replay adds neither. Native commit6/previous24/diff5/lint/fmt are focused
proof. Persistence/reopen, bounded undo, byte/work/cancellation limits and full integration remain owned.

## Revision identity across undo branches — D285

answers: "how can undo restore counters without accepting stale branch requests?"

0435 .6.7.1.1 pins the native57767b0 contract/code and reproduces a counter model: restore0 after
snapshot1, then current.successor gives a distinct snapshot1 that accepts the old revision precondition.
Undo must preserve a separate maximum-issued counter; new authoring allocates above it, while undo/
redo restore original snapshots/counters. Original counter/identity restoration remains unchanged.
The model probe is diagnosis, not runtime proof. Runtime fix/native branch refusal is verified0437
under .6.7.1.3; storage/reopen .7 must still retain issuance independently of undo history.
The read-only model probe remains diagnosis. Native in-memory traversal/branch acceptance passes0437; storage/reopen proof remains pending.

## History intent and operator0436

answers: "how do current history operator and original author remain distinct?"

G1-SLICE.6.7.1.2 owns current operation context/direction and exact original CommandMutationGroup.
Explicit membership then Design must match; operator/revision may differ from original author/base.
Both complete actor/principal claims remain untrusted and exact; original payloads/keys/order remain.
Native schema3/privacy1/lint/fmt pass. Runtime must authenticate current operator for every original
command scope, bind the exact accepted record/current revision/key and enforce bounded atomic effects.
Input construction is no history capability; apply_history separately enforces traversal/action audit and D285 issuance.

## Actual bounded snapshot history0437

answers: "how does undo preserve complete original semantics?" · "how are history effects authorized and ordered?"

G1-SLICE.6.7.1.3 moves actual predecessor/current models through bounded alternatives; no inverse is
reconstructed. Explicit finite depth/action limits, exact operation replay and shared complete-actor
key scope preserve append-only original/action records. Bind the complete submitted original to its
accepted record and selected stack top, authenticate current operator across every original command
scope, validate actual current/alternative models, preflight allocations and refresh authority at effects.
Original author remains distinct from current operator. Report oldest undo eviction/discarded redo.
Model counters restore, while maximum-issued authoring revision/entity ids never rewind. Global exact
audit sequence orders authoring/Undo/Redo independently of timestamps and restored counters; stale
witnesses share actual event ordering. Native bus37 (prior30/history7), privacy7/lint/fmt cover this
metadata subset, including D285 undo0/new2/old1 refusal and pointer/canonical roundtrip. Storage/reopen
must persist issuance/order; work/cancellation and deeper/full acceptance remain separately owned.

## Explicit metadata admission/progress contracts0438

answers: "what do initial operation limits measure?" · "does progress data itself cancel or authorize work?"

G1-SLICE.6.8.1.1 admits original submitted members, primary metadata selections and cumulative proposed
UTF-8 assignment-reason bytes under explicit positive host limits. Aliases still count. Member excess
precedes per-member target/byte excess; guard remaining room before arithmetic and retain exact factors.
Typed phase-local fractions require positive total/completed<=total. Data-only observer returns Continue/
Cancel and carries no model/authority capability. Native control4/privacy1/lint/fmt pass; runtime bus is
unchanged. Immutable installation/actual emission/atomic cancellation follow .6.8.1.2. These inputs do
not prove CPU time, total model memory, allocation peaks, numeric/file/decompression or solver bounds.

## Actual immutable operation control0439

answers: "where does metadata cancellation become effective?" · "may a request widen installed limits?"

G1-SLICE.6.8.1.2 installs policy exactly once with no default or request override. Admit original typed
metadata input before private source/model work, then emit actual phase-local data-only checkpoints.
Cancel discards private work without changing semantic model/reserve/audit/history/issuance/keys;
internal reserved capacity may grow. Prepare closed audit/result copies before ReadyToPublish and
fresh final complete effect authority. No observer/large preparation follows that authority check.
Exact replay refreshes entry authority and emits Replaying without new effects. Cheap authorized Piece
reads require no policy. Native bus44/privacy8/lint/fmt pass, including observer revocation at Ready.
Input counts and cooperative checkpoints do not prove total model/work/memory or wall-clock bounds;
.6.8.2 owns the remaining resource implementation, .6.5.5/.6.9 deeper/full acceptance.
