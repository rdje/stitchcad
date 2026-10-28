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

Whole0444–0449 component implementation journals retain exact source intervals in
[part605](../history/stitchcad-devnotes-part605.md); current component contracts/examples remain in the book.

Whole0451–0455 implementation journals retain exact source intervals in
[part610](../history/stitchcad-devnotes-part610.md); current component contracts/examples remain in the book.

## Retained charts and standalone profile input0456

answers: "are profile resolution intents owned by the current source host?"

Native source-owner definitions retain exactly measurements/sets, metadata/observations/chart and
metadata/chart. Separate SizeSetResolutionIntent owns transformation declarations, Known evidence
and LabelMapping vectors; G4-PROFILES.1.resource-admission is a P1 prerequisite before schema/editor/
transform input exposure, complementing G1.9 raw transport bounds. No G4 implementation is implied.
.6.8.2.2.2.2.3.2.2.1 charges actual garment registry/chart fields and MTM's owned Ease snapshot tokens,
common metadata separately. Source resource19/strict lint/fmt pass:4/14,2/66,partial1/33,combined6/80,
exact original owner/index and held missing-context state. Actual token spellings plus independent
Python/OS oracles corrected the initial64-byte fixture expectation to66. Source examples moved into
a bounded chapter; original whole bytes retained608. Family/host/higher-root/runtime/deeper/full
proof remains; component demand grants no currentness, physical or allocator memory/time certificate.

## Complete external set-family resources0457

answers: "how does source accounting preserve actual family origin and authored refusal order?"

.6.8.2.2.2.2.3.2.3 precharges the actual set vector and matches each explicit Garment/Mtm variant,
composing complete common/chart fields in order. Exhaustive private inventory/family views and enum
matching expose stored-field/variant changes without allocating a census, querying contexts or reading
values. Source resource24/strict lint/fmt pass:20/239/0, prefix22/241, exact original nested refusals,
reversed order/same totals/different first chart owner, singleton garment origin, empty/prior errors
and held missing-context state. One shared test-fixture module fixes first duplicate-loading lint.
External .3.2 implementation has focused acceptance; full host .3.3/higher root .2.5.2/runtime/deeper/
full work remains owned. Logical content grants no currentness, physical or allocator memory/time proof.

## Required actual source-resource port0458

answers: "how does core account for an actual immutable higher source host without supplied totals?"

DesignResourceSource extends DesignReferenceHost with required readonly charge_resources and only a
sealed sink. No default implementation, policy setter/reset/finish or publication/authority method is
provided; exhaustive honest foreign-host implementation is a trusted integration obligation. Actual
DesignSources matches private owner/two-root definition and visits MeasurementInputs then SizeSets,
without clone/current/census/context callbacks; existing meter code is unchanged. Source resource28/
strict core-lib/source lint/fmt pass:39/628/0,prefix41/630/1, exact first witnesses, held missing-context
and empty/prior failures. Python/OS field census389+239 corrects stale626 test expectation before
acceptance. Higher-root cached entity census/refs .2.5.2 and runtime .4/deeper/full proof remain owned;
source demand is no semantic/currentness, dishonest-host security, whole-model or allocator memory/time proof.

## Complete actual root-resource composition0459

answers: "does source accounting include the core owner's separately retained metadata census?"

The complete private Inputs/Sewing/Garment producers exhaustively borrow each wrapper/definition,
charge every actual component once in schema order and retain the caller's one sink. DesignMetadata
owns both H and a captured identity vector: charge actual H first, then the stored vector, without
regenerating the semantic census. Copy guards cover captured records and fixed provided refs.
Actual source resource33/strict core-lib/source-test lint/fmt pass:80/650/129, prefix82/652/136;
cache52+26, landmark78+1, notion79+1 and original source/revision refusals preserve unchanged data.
Real deletion refuses current validation but held accounting stays80/650/129; empty full draft0/8/0
retains canonical recipe. Incorrect expected static token-field spelling corrected against producer.
Whole0458 scope/lesson/ledger retained611. Runtime .4 must install explicit policy, bind initial
exact authority/snapshot before private walks and admit prospective shapes before costly preparation.
Deeper/full proof remains owned; model demand grants no authority/currentness, dishonest-host security,
allocator memory/time, retained-journal, physical or release certificate.
