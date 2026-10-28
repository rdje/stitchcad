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

[Whole0437–0442 snapshot/admission/cancellation/copy/reserve decisions](../history/stitchcad-devnotes-part596.md)
retain their complete answers, native proofs, original boundaries and retention tradeoffs.

## First-refusal logical dynamic-content contract0443

answers: "how does a resource producer charge work without replacing the active policy?"

G1-SLICE.6.8.2.2.2.2.1 defines positive collection-entry/UTF-8-text/unbounded-counter-bit limits and a
constant-state first-refusal meter in the Design domain. Precheck remaining room before arithmetic;
retain exact first counts/location across ignored/later/zero charges and finish. Give producers only a
sealed charging trait object, with no concrete-policy setter/reset/finish/model operation. Contract6/
privacy4/lint/fmt pass. Actual core/source producers are implemented; current/prospective model admission is
implemented with focused checks under .4, with retained/coupled/deeper/full work owned;
this contract proves no model traversal, allocator RSS/time or dishonest-host security boundary.

Whole0444–0449 component implementation journals retain exact source intervals in
[part605](../history/stitchcad-devnotes-part605.md); current component contracts/examples remain in the book.

Whole0451–0455 implementation journals retain exact source intervals in
[part610](../history/stitchcad-devnotes-part610.md); current component contracts/examples remain in the book.

Whole0456–0460 source/root/runtime implementation journals retain exact source intervals in
[part614](../history/stitchcad-devnotes-part614.md); current contracts/examples remain in the book.

[Whole0461–0466 resource decisions](../history/stitchcad-devnotes-part620.md) retain exact prospective
model/Notch/revision and actual request/report ownership contracts and focused proofs.

[Whole0467–0470 resource decisions](../history/stitchcad-devnotes-part624.md) retain accepted/current/
prospective authoring ownership, immutable setup and bounded doctest contracts with focused proofs.

[Whole0471–0474 history/source-ownership/resource/transaction decisions](../history/stitchcad-devnotes-part629.md)
retain exact native proofs, D293/D294 closure, compatibility changes and pending boundaries.

## Discarded private preparation context0475

answers: "how does a failed group identify discarded preceding commands without exposing candidates?" · "does the command census prove execution behavior?"

D295 reproduced natively:the mixed prefix/alias/failing-Notch error named only the failing AddNotch,
although command-layer §9/§11 require preceding discarded commands. Mutation/RetryConflict/Difference
now carry fixed Copy CommandGroupDiscardedStep data:index, command and key from actual successful
first-occurrence steps. Preserve original positions/order and omit aliases/failing/unexecuted members.
The producer copies no report/model and calls no authority/source provider; original error sources stay
unchanged. Diagnostic construction occurs only on these error paths, within admitted member counts.
Exhaustive external error patterns must add discarded or ..; existing wildcard patterns remain valid.

Native diagnostic1 fails before/passes after;transaction3/group-preview6/group-commit6/lint/fmt pass0.
Preview/commit witness positions0/2/3 and original keys, empty prefix, native Notch source and key-conflict
cause, then corrected same-key/two-ID/one-slot publication equal to a real reference. Lifecycle and
accounting sources stay unchanged; final Difference uses the same actual-step producer.
D296 corrected the G0-era census header:no executable bytes changed;17/5/5/0 native census passes.
Specification closure does not establish execution; native workflow evidence/full bus acceptance follow.

## Builtin workflow discovery and exact permission sets0476

answers: "where are actual native workflows declared?" · "why report a permission set instead of the strictest level?"

CommandWorkflowId/CommandWorkflowStep declare3 immutable native sequences once. Stable exact lookup
has no whitespace/case alias or default. Unique command/class metadata derives from catalog identities;
complete distinct rights derive from each purpose. Extract the existing group-purpose match unchanged
and share it with actual authorization; all other grant-check bytes/order and lifecycle code stay exact.
Discovery contains no model/session/policy/grant and is never runtime availability or authorization.
Non-exhaustive public enums permit future additions; actual new stages/workflows need native proof.

Native acceptance interprets every declared stage over real complete sources/models:inspection1,
edit/preview/replay5 and Undo/Redo7. It checks original/current reads, actual metadata differences,
identity issuance, capacity replay and restored snapshot pointers. A separate permission matrix proves
Inspect cannot commit and Commit cannot read/preview. Native2/metadata2/schema4/catalog2/group-auth5/
strict lint/fmt pass0; generated stdout table is watched by the core book-projection contract.
D297 clarified the misleading ordered/strictest wording. Named permissions stay distinct; native
preview requires Inspect alone, Propose is separate metadata, and partial grants never gain a union
through discovery. UI/API/MCP parity remains empty until actual adapter tests/digests exist. The new
book part and2 public API rows explain this boundary; broader .6.9.3/publication acceptance follows.

## Combined current native acceptance0477

answers: "what current native command populations passed together after implementation?"

All317 selected cases pass across21 targets:core command-contract66 in17 targets plus model-resource47
and metadata8;higher Design-input163 plus source-resource33. Completed output derives98 actual bus
cases within that163. Command15/Design34 doctests pass49, including privacy/lifetime/sealing controls.
Current workflow, discarded-error, resource, transaction and authority paths all participate. Production
source is unchanged in this acceptance leaf; source diagnostics/types and actual metadata handlers
are distinguished from still-unimplemented other runtime handlers. The book records native commands,
counts and limits. Full publication fixture probes follow before this metadata lane closes; workspace
CI/persistence/browser/adapter parity and physical/release evidence remain separate owners.
