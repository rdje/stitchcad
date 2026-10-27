# Sealed engineering continuity — part583

- **Identity:** 161 lines, 12908 bytes, sha256:bfff72b7916ace90d54b0f8b7c815e579088e2b263edaa4a4005a28b5aeabb07
- **Coverage:** whole0431 task/lesson/ledger and command implementation continuity0291–0430 from 20cf99a379ae36a508334b8a936252a5f4972e27.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 43152–45011, 1859 bytes, sha256:167d5b3d66ab714c2d9207115d1f953107462f277a60be265995bfe9710dc7d6
- Whole source interval: DEV_NOTES.md bytes 193–968, 775 bytes, sha256:afbc802513548faf620f219209659e03ad75a0011c196dfca835c8fec2598b36
- Whole source interval: CHANGELOG.md bytes 207–705, 498 bytes, sha256:d7443ebe72aa487b89bdc1a78e4a895962a1797df6fdec760d0ce8dce3c648b8
- Whole source interval: docs/decisions/decision_command-layer-contract-and-undo-granularity.md bytes 6465–16241, 9776 bytes, sha256:ef53b3be83b6ca63ea612272e2a6ba8b14943b69364dda793cc169122144cae8
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.4.2.2.2`
  Status: `done`
  Goal:0431 private preparation base/cursor sharing for successive actual staged models without
  authority substitution, reserve advancement or public model handle; original single-preview behavior.
  - [x] REPRODUCE / ISSUE: git show HEAD:crates/sc-core/src/command/bus/notch.rs | rg -n
    'impl<|self.identities.stage': original handler-bound bus impl/cursor reset printed rc=0.
  - [x] ROOT CAUSE (WHY + WHERE): native HEAD Notch source rc=0; private handlers depend on original
    bus owner and reset their own reserve cursor rather than accepting a preceding staged model/cursor.
  - [x] FIX: cargo clippy -p sc-measure --test design_inputs_contract -- -D warnings:finished rc=0;
    borrowed actual PreparationBase, shared original guards, explicit IdentityStage and private into_parts.
  - [x] ADDRESSED: cargo test -p sc-measure --test design_inputs_contract command_bus:18 passed rc=0;
    all three native preview handlers compile and preserve actual reports, refusal ordering and original owner.
  - [x] NO REGRESSION: cargo test -p sc-core --lib command::bus::identity:1 passed rc=0; cargo test
    -p sc-core --doc command::bus:4 passed rc=0; selected fmt --check rc=0. Full integration remains .6.9.
  - [x] LOCKSTEP: transition_receipts.py --record docs/history/stitchcad-devnotes-part582.md:3 exact
    Git spans rc=0; whole0430 scope/lesson/ledger retained. Book/pointer/frontier/lesson aligned.
  Native publication80 chapters/159 scoped API rows/1464 source links/2444 rendered links rc=0;
  maintained-reference81 files/17594 lines/1228233 bytes rc=0; retention1036/global66 fits rc=0.
  Cargo commands use scripts/local_environment.py. No group success, publication, replay or model
  replacement is exposed. Promotion declined: routine internal refactor under existing command contracts.
## _(2026-10-27 UTC)_ — private model base and identity cursor

0431 .6.5.4.2.2.2 moves the three metadata handlers onto an immutable private PreparationBase.
The bus keeps fresh authority/binding; the base keeps original availability/envelope/current checks.
Notch receives the concrete independent reserve cursor rather than starting one inside its handler.
Prepared state/report can be consumed internally for later staging; no public model handle exists.
Native bus18, identity1, compiler privacy4 and selected strict lint/fmt pass. Actual sequential group
preview remains the next leaf. Promotion declined: routine internal refactor under existing contracts.

[Whole0430 group-authority lesson](docs/history/stitchcad-devnotes-part582.md) retains its original proof.

## STITCHCAD-G1-0431 — share the private preparation base and identity cursor

- Three metadata handlers borrow the actual model through one private view; fresh authority stays in the bus.
- Pass the concrete staged identity cursor explicitly; original single-preview ordering/effects stay unchanged.
- Native bus18/identity1/privacy4/selected strict lint/fmt pass; sequential group preview follows.
- Whole0430 scope, lesson and ledger retained582 with three exact native Git source intervals.

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

answers: "who can retain a validated prepared command model?"
0427 .6.5.4.1 retains actual model/report in a private PreparedCommand; private prepare performs the
same original checks. Public preview consumes/discards the model; compiler privacy4/real bus18/lint
pass. This is the existing transaction prerequisite, not publication or completed group execution.

answers: "what does a typed mutation group validate?"
0428 .6.5.4.2.1 requires nonempty original order and exact explicit group/Design/actor/revision claims;
preserves payloads/retry keys and first failing index/context. No sorting, grant, replay or execution
follows. Native group4/privacy1/lint/fmt cover schema scope; sequential private staging follows.

answers: "how is an entire mutation group authorized?"
0430 .4.2.2.1 requires one fresh trusted snapshot/complete grant for all members, retaining original
first grant/member refusals. Decisions borrow the group, grant no effect capability and must refresh
at effects. Single-command checks retain their original order; scope/permission unions refuse.
