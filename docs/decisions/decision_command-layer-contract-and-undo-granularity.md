# The command layer: five classes, one undo granularity, and authority as a permission on a class

- **Type:** `decision`
- **Date:** `2026-09-30` (absolute); roadmap §4.4 requires undo/redo semantics to be defined **at G0**, and
  §7.8 requires the authority levels to be scoped from day one
- **Status:** `active`; all17 typed inputs, trusted-host authorization, Piece-list read and Measurement/Material/Notch metadata previews implemented; publication pending
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

## First native command foundation0291

G1-SLICE.6.1 adds sc-core CommandId discovery with all17 declared identities and typed class,
required-authority, reversibility and undo-grouping metadata. Exact lookup has no trim/case aliases.
The focused native target compares every row against the normative book table and passes2/0;
strict selected Clippy/fmt and command census17/5/5/0 failures pass, rc=0.
Metadata confers no authenticated grant or handler availability. Payloads, scoped authority,
aggregate mutation, transactions, retry/audit, history and cancellation retain .6.2–.6.9 ownership.

0293 .6.2.2.a adds immutable mutation context with explicit design, untrusted actor/principal claim,
expected exact revision, typed retry key and optional group. Initial revision0 has exact successors
in private BigUint workspace, including beyond128 bits; no undocumented counter limit or clock.
No wire format, currentness, grant, replay or handler is implied by constructing metadata.
Focused native2/0 retains full identities/counters and standalone/explicit group semantics.

0294 .6.2.2.b.1 binds ChangeMeasurement/AssignMaterial payloads to that context, deriving command
identity from the concrete variant. Canonical LengthDeclaration retains all authored states and
source/provenance. Ordered nonempty distinct PieceSelection and canonical MaterialAssignment share
the existing nonblank unresolved-reason rule. Actual registry binding, auth and execution remain
later bus owners. The focused native3/0 target checks those payload boundaries.

0295 .6.2.2.b.2.a adds CloseDart's canonical target and AddNotch's anchor/profile creation fields,
with an explicitly bus-supplied new identity; native construction2/0 and prior payload3/0 pass.
Director2026-10-26 resolves D226: SplitSeamSpan splits the sewing correspondence at explicit cuts
on both sides.0299 .6.2.2.b.2.b supplies the canonical span identity and explicit named A/B EdgeAnchors.
Original edge/parameter frames survive without inferred correspondence or result identities. The target
supplies copies, direction, ease and stops; their current validation and lineage-preserving atomic split
remain execution obligations. Native construction4/0 plus prior payload3/0 pass. All five mutation
schemas are present; no handler, geometric conservation or authorization is claimed.

0300 .6.2.2.c.1 adds exact snapshot/actor claims for queries and concrete GetPieceList/WalkSeamSpan
payloads. Variant-derived identity stays query/inspect; no mutation replay/group fields or grants.
Native query2/0 plus catalog2/0 check original contexts/targets and metadata, including exact revision
values beyond128 bits. Artifact-specific uncertainty closure follows actual concrete artifact inputs;
this foundation produces no result, geometric walk, mutable cache or authenticated authority.

0335 .6.2.2.c.2.6 completes the shared in-process CommandRequest over five original family values.
Identity is payload-derived; Design/actor/revision and mutation-only retry/group metadata are preserved.
Core owns authorization and invariants; sc-api is an adapter. No wire codec, grant or handler follows
from constructing data. Evaluation/artifact/release schemas, including D239 detached approval intent,
are now present; actual source validation, execution and human approval remain .6.3+ obligations.

0337 .6.3.2 checks one trusted host snapshot, matching exact submitted actor/role/principal and one live
permission/Design/command/time grant. Permissions are explicit; grants never combine to widen coverage.
Agents cannot hold/issue approval; mutation preview needs inspect. Transport providers must authenticate
sessions and validate human grant issuance; submitted claims never become their own authority source.
Returned decisions describe a snapshot, not future execution or approval; effects must reauthorize and
validate actual sources/revisions, subjects/evidence/scope. No handler, human record or server is claimed.

answers: "which commands actually execute over the private owned Design?"

0419 .6.5.2.a installs CommandBus with the actual DesignGarment and trusted application authority
source, neither replaceable by a request. Only GetPieceList executes. Every read freshly authorizes,
then binds actual Design and exact revision; original typed auth causes and both context values
survive refusals. The original authored Piece slice/revision are borrowed, with no reconstruction,
normalization, defaults, observable model cache or revision changes. Privacy/lifetime controls prevent
replacement and escaping borrows; earlier results do not authorize a later call after revocation.
Inspection may show held repairs and modelled physical deferrals without certifying readiness.
Native5 on real sc-measure schemas/privacy2/strict selected lint/fmt pass0; installed actual03460b0
and41400cd book committed checks pass0. Whole book subtree/0418 evidence568 retains4 exact Git spans.
Candidates/allocation/semantic diffs follow .6.5.2.b/.c; handlers/evaluator dependencies, effect-boundary
publication, replay/audit/history, runtime and UI/API/MCP parity remain owned. No mutation executes.

answers: "what does an actual Material preview validate and preserve?"

0422 .6.5.2.b.2 prepares an actual private AssignMaterial candidate after fresh inspect authority,
exact Design/revision binding, explicit envelope and complete current-source checks. All selected
Piece identities resolve before replacement; original labels, held references and multiplicity remain.
The actual rebuilt aggregate rechecks current registry/source rules, including assigned Material
existence. Original typed causes survive. A report carries only Design/base revision and exact changed
assignments in selection order, omitting no-ops and preserving unresolved reason bytes. Candidate
discard leaves the original Piece inventory/revision unchanged; no new identity or publication occurs.
The report grants no later effect authority. Complete topology, geometry, physical readiness, other
handlers and atomic publication remain owned; focused native bus10/privacy2/lint/fmt establish this scope.

answers: "how are actual candidate differences kept within an operation's semantics?"

0424 .6.5.2.c compares original and validated candidate using exhaustive owner-definition patterns,
covering unchanged garment families, sewing graph, inputs/source host, recipe, structural identity/
revision, canonical declarations, cut plan and ledger. Preview hosts support value equality.
Material differences permit only selected exact assignment fields; count/order and every other Piece
field/landmark remain equal. Added Notch differences require the original exact landmark prefix and
one actual appended object matching intent/staged identity, with complete Piece inventory unchanged.
Reports read candidate values. Unexpected changes retain structured Difference causes and refuse.
Focused canonical diff3 and real bus14/privacy3/lint/fmt cover this subset; other mutation semantics,
publication and full bus/evaluator/runtime proof remain owned.

answers: "how does ChangeMeasurement prepare stable canonical declaration replacement?"

0425 .6.5.3.1.1 adds CanonicalInputs::with_length_declaration: existing dedicated id selects one
original position, retaining exact supplied source/state and all other owned records. MissingLength
refuses absent/general-scalar targets. The original owner stays unchanged; no binding, authority,
source truth or evaluation follows from detached replacement. Actual binding/candidate/diff .1.2 follows.

answers: "what does a real ChangeMeasurement metadata preview check?"

0426 .6.5.3.1.2 resolves the actual Measurement role/binding before requiring the original bound
declaration id. Exact source/state replacement rebuilds and validates the actual candidate; shared
consumers such as Material width retain their original current-property errors. Canonical diff
normalization reverts only the selected declaration and compares the full owner, preserving every
other value/order; other metadata/Pieces/landmarks remain identical. Reports read actual candidate
values, omit no-ops and confer no publication authority. Unknown/Derived gain no numeric fallback.
This is metadata preparation; actual recipe execution/source truth remain separately owned.

answers: "which book publication populations were actually checked?"
D283 correction0426: the producer reports chapters, scoped API rows, source links and rendered links.
Historical575/576/577 called the middle populations tables/rows; their tuples were correct, labels were not.
Named PublicationResult/summary now preserve native formatter semantics; no table count was measured.
