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

[Whole0432–0436 sequential preparation/publication/revision/operator decisions](../history/stitchcad-devnotes-part592.md)
retain their complete lessons, alternatives, focused proofs and pending boundaries.

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

## Measurement evidence admission0440

answers: "are submitted Measurement evidence vectors covered by target counts?"

D287 native baseline shows they are independent: a Known replacement carries two references while
member/target/reason demand stays1/1/0. G1-SLICE.6.8.2.1 adds a fourth explicit positive host maximum
and cumulative evidence-reference demand/refusal, including exact aliases. Preserve original vectors,
counts and first member/index/command; compare remaining room before adding, without source lookup.
All costly metadata entry paths share this admission before authority/model work. Refused keys remain
unused. No evidence truth/registry grant is implied. This bounds already typed input, not raw decoding,
initial construction, whole-model validation/cloning or retained-state memory; those remain separately owned.

## Bound source-host reconstruction copies0441

answers: "why consume copied inner Design owners instead of cloning them again?"

D289 native real-source instrumentation measured individual H copies3/3/2 in Measurement/Material/Notch
previews. G1-SLICE.6.8.2.2.1 consumes copied Structure/Inputs/Sewing definitions privately and rebuilds
through unchanged validators. Bus reconstruction now requests one H copy per non-alias candidate and
one extra for a new revision; replay and actual Undo/Redo request none. Explicit member policy bounds
candidate count. Native bus46 and privacy3/4/4 verify copy counts and no public extraction. This does
not bound trusted host-internal bytes/time, all core collection work or retained memory; .6.8.2.2.2 owns those.

## Shared reserve publication0442

answers: "does committing metadata require copying all unused reserved identities?"

D290 native source/caller census shows prior after_consuming recopied the entire unused Vec even for
zero creation. G1-SLICE.6.8.2.2.2.1 retains one immutable shared backing and publishes a checked forward
prefix. Independent borrowed cursors preserve local consumption/exhaustion; publication shares the
backing without copying unused IDs, and history never rewinds issuance. Cursor1/native bus46 and
selected lint/fmt pass. Initial backing including consumed IDs remains allocated for bus lifetime;
this trades finite initial retention for removal of per-commit backing copies, not a total resource,
initial-conversion or timing certificate. D291's standalone preview explicitly installs operation policy.
