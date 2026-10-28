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

## Exact prospective Measurement resources0461

answers: "does replacing evidence charge both the old and new vectors or allocate a candidate first?"

After actual current checks/Measurement binding, a private borrowed root projection selects the exact
replacement at its canonical slot and charges only resulting owned evidence, before candidate clone/
constructors. Public current wrappers use the unchanged None route; original index/order/other fields
and all source/census ownership remain. No supplied totals or public projection capability is exposed.
Projection3/old Measurement4/source33/canonical selection6/strict lint/fmt pass against actual rebuilt
validated-model oracle: +1 refusal/no host copy, no-op/same-size at N, Unknown N-2, sequential shrink/
restore/alias, unchanged original address/revision and refused group/key/capacity reused. E0599 uses
actual difference getters; zero shared Material-width fixture corrected to explicit positive1000um.
Whole0460 retained613. Material/Notch/revision .4.2.2–.4.2.4, retained state/deeper/full proof remain owned.

## Exact selected Material resources0462

answers: "does a proposed Material reason count in addition to the old reason or before Piece copies?"

Check every actual selected Piece first, preserving authored missing-target index; then borrow the
selection/assignment in a private exhaustive projection enum. At each original Piece slot charge its
resulting reason only when selected; retain unselected fields, owned labels/collections and fixed
Assigned refs. The full original schema walk precedes any Piece/reason/model clone, with no supplied
totals or mutable/public projection capability. Actual rebuilt-model oracle plus Material projection4/
old Material5/Measurement projection3/source33/Piece5/strict lint/fmt pass:4+3→6+6 adds5 bytes and
refuses before host copy; no-op/same-size/shrink/assigned removes7, missing index1, sequential restore/
alias and failed group/key/capacity preservation. Python/OS confirms exact UTF-8 lengths4/3/6.
Whole0461/0456–0460 journals614 retain4 Git spans. Notch/revision/retained/deeper/full work remains owned.

## Exact actual Notch-append resources0463

answers: "can prospective Notch admission preserve original identity/anchor failures without consuming the cursor?"

Readonly-peek the actual concrete borrowed iterator, build/validate only fixed Notch payload, then
admit the exact resulting root before staged consumption or registry/model copy. Target→exhaustion→
anchor precedence persists; no fake ID or external allocator is used. Charge the whole new registry
before members, including the actual appended fixed Notch; exhaustive/Copy guards and nonzero enum
size protect the shape/count. Native Notch projection3/old Notch4/source33/sewing4/cursor1/strict lint/
fmt pass against actual rebuilt model: +1 entries/unchanged text/bits; no-copy refusal, one-entry
boundary/aliases/later rollback and freed-evidence retry retains original ID/key/group/capacity.
Whole0462 retained615. Publication revision .4.2.4/retained/deeper/full work remains owned; no complete
retained-state, ingress, allocator memory/time, physical or release certificate.

## Exact maximum-issued publication resources0464

answers: "does undo lower the resource forecast for the next issued revision?"

After actual group metadata validation, project the complete candidate with the private maximum-issued
revision's successor magnitude, before successor/revision-model construction. Pinned public bits/
trailing_ones APIs forecast width readonly; no clone/format/counter advance. Original public successor
semantics persist. Checked magnitude-count overflow uses exact chunks/no wrap, clipping or counter cap.
Runtime revision3/forecast1/count-boundary1/source33/structure5/context2/D2851/copy1/strict lint/fmt pass:
actual rebuilt model oracle covers0/sparse/carry/u128MAX; carry refusal leaves bounded metadata copy1,
no final revision reconstruction, original revision/key/capacity. Max3/undo0 still forecasts4/3bits under
cap2; exact authoring/history replays0/0/0. Offline num-bigint0.4.6 public docs verified after web fetch
failed. Whole0463 and whole0388/0389 lesson616/617 retain4 Git spans, freeing DEV from401-line overflow.
All model projections implemented with focused checks; retained .4.3/coupled/deeper/full proof remains.

## Exact owned mutation-request resources0465

answers: "is one owned Material reason multiplied by selected Pieces, and do retry aliases own data?"

Typed group accounting charges common/member revision copies, member/selection/Known-evidence vectors
and one stored reason per payload. Exact aliases own another context/payload copy; runtime dedup does
not erase those fields. Declared operation work8 differs from one owned4-byte reason for two Pieces.
CommandState locations keep actual containing group/member position, not a promoted referenced
canonical ID. Exhaustive private owner views/Copy guards and shared actual declaration walker cover
all five variants without model/current/authority/numeric callbacks. New request4/source33/canonical6/
strict lint/fmt pass:12/4/14,prefix14/6/15, fixed3/0/0, Unknown1/0/0 and original first witnesses.
Whole0464 retained618; current constructors/bus code byte-exact. Bounded command-resource chapter
owns examples; report/record/private retained/runtime policy and coupled/deeper/full work remains.

## Exact owned preview-report resources0466

answers: "does a restored net value or retry alias remove owned report snapshots from resource demand?"

Actual single/group preview readers borrow every private owner exhaustively. Charge common/step revisions,
step and net vectors, before/after evidence/reasons and exact alias report copies in actual stored order.
No-op reports own no changes; fixed Notch payloads own no dynamic field. Net restoration removes only the
net snapshots: intermediate/restoring step copies remain. Standalone locations name actual Design/change;
group locations name actual containing group/step, net members their original change index. No referenced
Piece/declaration is promoted into a containing owner; no clone/model/source/authority lookup occurs.
Real inspect-authorized bus reports verify30/32/12,prefix32/34/13,restored32/32/16 and revision0
no-op/Unknown8/0/0; report5/request4/strict lint/fmt pass. Runtime bodies byte-exact. This is logical
owned report data only; accepted records/private aggregate/retained runtime policy and broader proof remain.
