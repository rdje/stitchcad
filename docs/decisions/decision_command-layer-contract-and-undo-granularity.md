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
privacy4/lint/fmt pass. Actual exhaustive core/source producers and runtime model admission remain .2/.3/.4;
this contract proves no model traversal, allocator RSS/time or dishonest-host security boundary.

## Actual canonical resource producer0444

answers: "how does logical accounting reach actual canonical declarations without changing state?"

G1-SLICE.6.8.2.2.2.2.2.1 precharges actual canonical length/scalar vectors, then each original Known
state's evidence-vector length. Exhaustive owner/state patterns and Copy-payload constraints expose
future dynamic-field obligations. Preserve original order/field/index; do not read availability, clone,
normalize or inspect registry truth. Native resource10 (prior6/actual4)/lint/fmt verify15/0/0 demand,
exact first refusals, original owner and empty/prior-error composition. This is a canonical component;
remaining core/source/runtime, allocator RSS/time and evidence/publication proof stay separately owned.

## Actual Piece/copy resources and nested owner0445

answers: "how do resource refusals identify a nested hole without rebirthing its Piece?"

G1-SLICE.6.8.2.2.2.2.2.2.1 precharges actual Piece/copy vectors and exact reason/label bytes through
exhaustive held views. Add optional actual owner identity to static scope/field/index so nested holes
retain Piece owner/hole index, and canonical evidence retains declaration owner. No path-string
allocation, rebirth, repair, registry lookup or geometry proof. Resource15/privacy4/lint/fmt verify24/18/0,
original nested/UTF-8/copy witnesses, actual split-held content and no mutation. D292 corrects one stale
ontology metadata-command clause; whole-model/source/runtime and physical/RSS/time proof remain owned.

## Actual held topology resource producer0446

answers: "does resource accounting replay topology to estimate its held indices?"

G1-SLICE.6.8.2.2.2.2.2.2.2 charges actual journal/per-edit vectors and stored live/registration lengths.
Exhaustive ledger/journal/edit views plus Copy index-key/value constraints expose owned-field changes.
Precharge before traversal; retain actual operation/index locations without replay, resolution, repair,
mutation or extra model copies. View debug shows counts only. Resource19/lint/fmt verify22/0/0 across
all six edits, vector/index/registration witnesses and original ledger/choice preservation. This completes
Piece/copy/topology component accounting; whole-root/source/runtime and RSS/time/readiness remain owned.

## Actual retained input metadata0447

answers: "does a recipe resource check reparse the source or omit its canonical identity?"

G1-SLICE.6.8.2.2.2.2.2.3 borrows both original/canonical recipe strings, parameter tokens and actual
Material names/provided composition. Precharge vectors; preserve authored owner/index/field under one
meter. Exhaustive held views and fixed Copy guards expose field evolution without lookup, regeneration,
value inference or applied shrinkage. Resource24/lint/fmt verify4/74/0 and exact cumulative witnesses;
empty source still owns canonical(recipe)8 bytes. Original data stays exact. Remaining model/source/
runtime, binding/evaluation, truth/physics and whole-model/RSS/time proof remain separately owned.

## Actual held sewing resources0448

answers: "does accounting resolve sewing anchors or instantiate symbolic ease/profile bindings?"

G1-SLICE.6.8.2.2.2.2.2.4.1 precharges actual graph/spans/weighted-region/stop/landmark collections.
Exhaustive held views and fixed Copy guards preserve all ease sources/distributions and Notch/Turn
profile/anchor payloads, without lookup, rebirth, resolution, inferred values or realized geometry.
Resource28/lint/fmt verify13/0/0, exact owner/index limits, actual deletion-held content and no mutation;
initial public-export gap was corrected before acceptance. Garment/root/source/runtime, currentness,
binding, selected profile/ease values and physical/RSS/time proof remain separately owned.

## Actual held garment composition resources0449

answers: "what does actual Pocket/notion/Closure resource accounting measure?"

.6.8.2.2.2.2.2.4.2 charges actual Pocket/component, notion and Closure/instance vectors in authored
order. Exhaustive owner/definition views and Copy guards preserve fixed original source/copy/anchor,
opening, size, operation and derived Count payloads without lookup, rebirth, resolution or execution.
Native resource32/strict selected lint/fmt pass:29/0/0, unused notions, all supported Closure intents,
exact cumulative owner/index refusals and real deletion-held content unchanged. Component reference
moved byte-exact to model-resource-accounting.md; operation policy example now correctly says four.
Layers/Hems, construction, root/source/runtime and deeper/full acceptance remain separate. This is
no current binding, physical readiness, whole-model or allocator memory/time proof. Active raw64
requires owned archive maintenance before another retirement; aggregate85 alone is not the trigger.

## Actual held layers and Hems0451

answers: "how are held layer/Hem resources charged without granting Lining execution?"

.6.8.2.2.2.2.2.4.3 charges actual Facing/Lining/Interfacing vectors, offset-source vectors and exact
unresolved Material reason bytes, then Hems. Exhaustive private owner/wrapper/content/definition views
and Copy guards retain original sources/operation/depth/fold/method intent. Faced ids add no duplicate
Facing content. Native resource37/strict selected lint/fmt pass:13/9/0, exact family/owner/index first
refusals, unchanged env_lining, empty/prior-error and real deletion-held state without lookup/repair.
Construction/root/source/runtime/deeper/full work remains; no current binding, execution/physical,
whole-model or allocator memory/time certificate follows from component accounting.
