# The command layer

> **Status:** normative specification, gate **G0** (roadmap §4.4, which requires undo/redo semantics to be
> defined *at G0*, and §7.8 for the authority levels). Implemented in `sc-core` from gate G1
> (leaf `G1-SLICE.6`); `sc-api` adapts transports, with workflow parity proven at G5. Terms are in the
> [glossary](glossary.md); the diagnostics follow the [internationalization](i18n-architecture.md)
> chapter's identity rule.

One surface, three front-ends. A UI panel, a CLI invocation and an MCP tool call are **adapters** over the
same typed commands, and the command layer is where every domain invariant lives. The property that makes
the three front-ends trustworthy is not that they are similar but that **there is no other way in**: no
mutation path bypasses the bus, which G1 enforces by module privacy and by a test that the domain types
expose no public mutators.

## 1. The command set

Five classes. The roadmap names one command per class as an example, and the census in §11 checks that all
five are carried here.

| Class | What it does | Authority needed | Reversible |
| --- | --- | --- | --- |
| `query` | reads the model and changes nothing | `inspect` | `not applicable` |
| `mutation` | changes the design, through the recipe | `commit` | `yes` |
| `evaluation` | computes an instance from the recipe | `generate` | `discarded` |
| `artifact` | produces files from an evaluated instance | `generate` | `not applicable` |
| `release` | prepares or approves a package | `generate`, or `approve` to approve | `no` |

The Reversible column is a closed vocabulary, and §1.1 draws its values from it:

| Reversibility | What it means |
| --- | --- |
| `not applicable` | nothing changed, or an artifact is immutable once written |
| `yes` | undo reverses the group, restoring semantics and not only geometry (§3) |
| `discarded` | a derived result is thrown away rather than undone; recomputation is its inverse |
| `no` | an approval is a record, and withdrawing it is a new record (§5) |

Three rules bind the classes:

- **A mutation is expressed in the recipe, never as a geometry edit.** `SplitSeamSpan` splits a sewing correspondence and the
  identity contract (ontology §1.1) says what happens to the references; a command that wrote coordinates
  directly would create geometry no operation owns, which is what the formula language refuses too
  ([formula language §2](formula-language.md)).
- **A class determines the authority, the reversibility and the undo granularity.** An adapter may not
  re-classify a command to make it convenient, and a new command arrives in one of these classes or the
  table gains a row with a recorded reason.
- **Queries never mutate, not even a cache an observer can see.** A query that changed a revision counter
  would make read-only front-ends unsafe, and `inspect` is the one authority an agent always holds.

### 1.1 The commands

One row per command, and the row is the declaration: the census in §10 reads this table, so a command
nobody wrote down cannot be issued and a command written down without a class cannot be reasoned about.
The roadmap names five of them (§4.4) and all five are carried here.

| Command | Class | Authority | Reversible | Granularity | What it does |
| --- | --- | --- | --- | --- | --- |
| `GetPieceList` | `query` | `inspect` | `not applicable` | `not applicable` | the pieces, their multiplicity, pairing and material |
| `WalkSeamSpan` | `query` | `inspect` | `not applicable` | `not applicable` | a span's length differential and its attribution to ease (ontology §3.2) |
| `GetUncertaintyClosure` | `query` | `inspect` | `not applicable` | `not applicable` | the unknowns that affect one requested artifact ([release §8](release-contract.md)) |
| `ChangeMeasurement` | `mutation` | `commit` | `yes` | one group per measurement changed | sets a measurement, which re-evaluates the recipe |
| `AssignMaterial` | `mutation` | `commit` | `yes` | one group | assigns a material to one or more pieces |
| `SplitSeamSpan` | `mutation` | `commit` | `yes` | one group, however many references move | splits a sewing correspondence at explicit cuts on both sides, preserving reference obligations |
| `CloseDart` | `mutation` | `commit` | `yes` | one group: centre, two legs, intake, truing | closes a dart, conserving its intake (ontology §4.3) |
| `AddNotch` | `mutation` | `commit` | `yes` | one group | adds a notch at an edge and a parameter (ontology §4.5) |
| `EvaluateInstance` | `evaluation` | `generate` | `discarded` | `not applicable` | evaluates the recipe for one size |
| `GradeSize` | `evaluation` | `generate` | `discarded` | `not applicable` | instantiation path B for one size ([paths §3](instantiation-paths.md)) |
| `ResolveProfile` | `evaluation` | `generate` | `discarded` | `not applicable` | resolves a Factory Profile's parameters against its constraints |
| `ExportDxf` | `artifact` | `generate` | `not applicable` | `not applicable` | writes one registered target ([interchange §2](interchange-dialects.md)) |
| `ExportPlt` | `artifact` | `generate` | `not applicable` | `not applicable` | writes the plotter path with its pen map |
| `ExportPdf` | `artifact` | `generate` | `not applicable` | `not applicable` | writes A0 or tiled pages with a scale square |
| `GenerateTechPack` | `artifact` | `generate` | `not applicable` | `not applicable` | generates the tech pack from canonical content |
| `PrepareRelease` | `release` | `generate` | `not applicable` | `not applicable` | builds a candidate package and its manifest ([release §2](release-contract.md)) |
| `ApprovePackage` | `release` | `approve` | `no` | `not applicable` | binds a human approval to a package identity |

## 2. The shape of a command

Every command is a typed value, not a string and not a method call:

| Field | Carries |
| --- | --- |
| `command_id` | the stable name, which is also the message id of its diagnostics ([i18n §3](i18n-architecture.md)) |
| `arguments` | typed values with kinds, so a wrong unit is refused before anything is computed ([formula language §2](formula-language.md)) |
| `precondition` | the revision it is valid against (§5), and any structural fact it needs (an edge that exists, a piece that is not folded) |
| `authority` | the level it requires (§7) |
| `effect` | what it may change: the design, an instance, an artifact, a package |
| `group` | the atomic group it belongs to, if any (§3) |
| `actor` | who initiated it — a named human, or an agent acting for one (§7) |

Validation happens once, in the command layer, and every adapter gets it for free. That is the argument
against per-front-end validation: three validators drift, and the drift is visible only to the user whose
front-end has the looser one.

### Implemented typed catalog

The sc-core command module now exposes CommandId::ALL and exact from_token lookup for the17
declared commands in §1.1. Each identity supplies its class, required authority, reversibility and
undo granularity as typed metadata. Stable tokens are independent of localized labels. Lookup
does not trim, case-fold or invent aliases; unknown names return None.

Catalog membership neither advertises a handler nor constructs payloads, grants authority or
performs transactions. Approve is required metadata for
human approval, never an agent-held grant. Mutation previews still need only inspect; the catalog
reports the permission required to issue the declared command itself. Typed inputs are available below;
authenticated actors/scopes, structural checks, revisions, transactions and workflow parity remain G1.6.

Historical focused verification receipts remain complete in `docs/history/stitchcad-devnotes-part477.md`.

### One typed request surface

The following names denote the existing core input API; construction grants no authority.

| API token | Meaning |
| --- | --- |
| `CommandRequest` | One of the five typed command families |
| `CommandQuery` | Snapshot-bound read intent |
| `CommandMutation` | Authored change with retry/group context |
| `CommandEvaluation` | Snapshot-bound derived-result intent |
| `CommandArtifact` | Snapshot-bound artifact intent |
| `CommandRelease` | Snapshot-bound preparation or approval intent |
| `CommandSnapshotContext` | Exact Design, actor claim and requested revision |
| `CommandQueryContext` | Compatibility alias for the shared snapshot context |
| `DesignRevision` | Exact unbounded Design revision counter |
| `QueryPayload` | Concrete piece-list, seam-span or artifact-closure read input |

`CommandRequest` retains one original `CommandQuery`, `CommandMutation`, `CommandEvaluation`,
`CommandArtifact` or `CommandRelease`. `command_id()` derives identity through the concrete payload;
`design()`, `actor()` and `expected_revision()` expose original claims. Revisions are borrowed unchanged.
`mutation_context()` borrows retry/group metadata only for mutations; other families return `None`.

```rust
use sc_core::{command::{CommandRequest, CommandQuery, CommandSnapshotContext,
    CommandActorClaim, DesignRevision, QueryPayload, CommandId}, ontology::EntityId};
let request = CommandRequest::Query(CommandQuery::new(CommandSnapshotContext::new(
    EntityId::from_bits(1), CommandActorClaim::Human(EntityId::from_bits(2)),
    DesignRevision::initial()), QueryPayload::GetPieceList));
assert_eq!(request.command_id(), CommandId::GetPieceList);
assert_eq!(request.mutation_context(), None);
```

All17 declared commands have typed inputs; evaluation/artifact/release details remain in their
linked chapters. Constructing this shared in-process input supplies neither a wire codec nor execution.
Authenticate and validate grants/current sources before handlers. Within authorized execution,
scope precedes geometry; reauthorize at publication. Agent approval intents stay untrusted; only
authenticated humans may accept.

### Implemented execution

The [execution guide](command-execution.md) covers the private actual garment owner, authorized
GetPieceList read, Measurement/Material/Notch previews, atomic metadata-group commit/replay/audit
and bounded actual-snapshot undo/redo. Other handlers, persistence and adapter parity remain pending.

### Implemented initial query requests

`CommandQueryContext` aliases the shared `CommandSnapshotContext`, which carries the explicit Design identity, unauthenticated actor/principal claim
and exact requested `DesignRevision`. The revision is mandatory, including revision0 and exact values
beyond128 bits; no query silently chooses a latest snapshot. Currentness and scoped inspect authority
must be checked by execution. Queries carry no mutation replay key or undo-group membership.

`CommandQuery` binds this immutable context to `QueryPayload`. `GetPieceList` requests the Design's
canonical piece list, including multiplicity, pairing and material. `WalkSeamSpan { span }` names the
canonical sewing correspondence whose length differential/ease attribution is requested; copies,
ranges, direction, ease and stops come from that actual target, rather than duplicated arguments.
The concrete variant determines the command identity and its catalog metadata remains query/inspect.

```rust
use sc_core::{command::{CommandActorClaim, CommandQuery, CommandQueryContext,
    DesignRevision, QueryPayload, CommandId}, ontology::EntityId};
let id = EntityId::from_bits;
let request = CommandQuery::new(CommandQueryContext::new(id(1),
    CommandActorClaim::Human(id(2)), DesignRevision::from_counter(7)),
    QueryPayload::WalkSeamSpan { span: id(3) });
assert_eq!(request.command_id(), CommandId::WalkSeamSpan);
```

Artifact-specific closure now carries the [same concrete artifact request](artifact-requests.md).
See [evaluation requests](evaluation-requests.md) for shared snapshots and both instantiation paths.

### Implemented mutation context

CommandMutationContext retains a target design identity, CommandActorClaim, expected DesignRevision,
mandatory CommandIdempotencyKey and optional CommandGroupId. Fields are private and exposed through
read-only getters. Retry and group identities have different Rust types; neither is silently minted
from a design or actor identity. None for the group requests the standalone group-of-one case.

An actor claim distinguishes a human initiator from an agent acting for a declared human principal.
Both initiator and principal survive unchanged for later audit and grant validation. A claim is
caller-supplied data: constructing Human or reading principal() authenticates nobody and grants no
permission, including approval. Core authorization must bind it to verified identity and scope.

```rust
use sc_core::command::{CommandActorClaim, CommandIdempotencyKey, CommandMutationContext, DesignRevision};
use sc_core::ontology::EntityId;
let agent = EntityId::from_bits(7);
let principal = EntityId::from_bits(17);
let revision = DesignRevision::initial();
let context = CommandMutationContext::new(EntityId::from_bits(21),
    CommandActorClaim::Agent { identity: agent, principal }, revision.clone(),
    CommandIdempotencyKey::from_entity_id(EntityId::from_bits(31)), None);
assert_eq!(context.actor().initiator(), agent);
assert_eq!(context.actor().principal(), principal);
assert_eq!(context.expected_revision(), &revision);
assert_eq!(context.group(), None);
```

DesignRevision is an exact nonnegative counter with private integer workspace. initial()/default()
represent zero; from_counter preserves an explicit u128 value; successor() computes the next counter
without changing the source or committing a command. It continues beyond u128::MAX without wrap or
a new revision limit. Decimal Display retains the exact value; the wire/persistence schema is still
owned by its later implementation. Undo's revision restoration remains §3, not an automatic increment.

### Implemented initial mutation payloads

MutationPayload carries ChangeMeasurement, AssignMaterial, SplitSeamSpan, CloseDart and AddNotch. CommandMutation binds that
concrete payload to CommandMutationContext. Its command_id() is derived from the variant; there is
no separate caller-supplied command identity that could misclassify the request. These are typed
request shapes. The [execution guide](command-execution.md) distinguishes implemented queries and
metadata preview/atomic publication and bounded history from other pending handlers and integration.

ChangeMeasurement carries the measurement metadata identity and an immutable replacement
LengthDeclaration. Measurement and declaration identities are distinct. The replacement retains
its own declaration/source identity and exactly one authored state with its original provenance:
known, assumed, unknown, preference or derived. Constructing the request reads no numeric value,
so an observation request or formula reference survives without a fabricated cached result.
Execution must check the actual measurement-to-declaration binding, stable declaration identity
and proposed source/provenance against current records before accepting the replacement.
Changing the supplied source is not implicitly forbidden; the payload preserves it for validation.

AssignMaterial carries a PieceSelection and the canonical MaterialAssignment. Selection requires
one or more distinct identities, retaining authored order. NoPieces refuses an empty list;
DuplicatePiece retains the actual identity and zero-based first/repeated positions at the first
duplicate. It neither silently deduplicates nor sorts the request.

```rust
use sc_core::command::{AssignMaterial, PieceSelection, MutationPayload, CommandId};
use sc_core::ontology::{EntityId, MaterialAssignment};
let pieces = PieceSelection::new(vec![EntityId::from_bits(9), EntityId::from_bits(4)])?;
let intent = AssignMaterial::new(pieces,
    MaterialAssignment::Unresolved("factory selection pending".to_owned()))?;
assert_eq!(intent.pieces().pieces(), &[EntityId::from_bits(9), EntityId::from_bits(4)]);
assert_eq!(MutationPayload::AssignMaterial(intent).command_id(), CommandId::AssignMaterial);
# Ok::<(), Box<dyn std::error::Error>>(())
```

An assignment preserves the assigned material identity or explicit unresolved reason. The same
local reason predicate used by canonical Piece construction rejects blank unresolved text;
AssignMaterialError retains the original whitespace. A nonblank reason is kept exactly. Material
existence, kind and truth still need the actual registry and provenance checks.

CloseDart and AddNotch payloads are also available. CloseDart names the canonical dart: the bus
resolves its held closing operation, intake and references rather than accepting copied geometry
or a replacement intake. The target's existence/currentness and the operation's kind remain
execution checks, and no closure or conservation result is produced by constructing the intent.

AddNotch carries the requested owning piece, persistent EdgeAnchor and complete symbolic
NotchProfileBindings. Style/encoding and separate sample/production dimension references survive
exactly; no physical profile value or default is supplied. The request carries no new notch id.
definition_with_id composes a canonical NotchDefinition with an explicit identity supplied by the
executing bus. It allocates nothing: the bus must allocate a fresh id and validate the actual
piece, current anchor and profile obligations before accepting the new entity.

Director ruling2026-10-26 resolves D226: SplitSeamSpan splits the paired sewing correspondence
at explicit cuts on both sides. G1-SLICE.6.2.2.b.2.b implements `SplitSeamSpan::new(span, side_a, side_b)`.
The target is a canonical SeamSpan identity. Both cuts are explicit `EdgeAnchor` values retaining their
original persistent edge and exact rational source-frame parameter. `cut(SeamSideId::A/B)` returns the
named cut unchanged; unequal parameters are allowed in the request, and swapping sides changes intent.
No missing cut is synthesized, mirrored, projected or inferred from the other side.

For example, an A cut at2/3 on one source edge and B cut at1/5 on another retains both fractions;
only actual sewing/geometry checks can establish whether they correspond. Equal source anchors do not
establish equal physical copies: the canonical span supplies the copy identities. Construction also
retains endpoint parameters without claiming they are valid interior cuts of the actual span.

The actual target supplies copy/range ownership, direction, signed ease/distribution and semantic
stops. The request neither duplicates those records nor supplies new result identities. Before any
mutation, execution must resolve the current span and cuts, establish strict interior paired cuts,
preserve reference/ease/stop obligations and allocate lineage-preserving outputs atomically. The payload
creates no split result or geometry edit. All five mutation schemas are available. Three metadata handlers
execute through the [native bus](command-execution.md); paired seam and Dart execution remain pending.
[Source-cut binding](command-seam-splits.md) checks the actual held span and strict interior cuts.

## 3. Atomic groups, and the granularity of undo

**One undo reverses one atomic group, never part of one.** A group is the unit a user means: "add a dart"
is a centre line, two legs, an intake and a truing of the waist edge, and undoing three of the four leaves
a piece nobody drafted.

| Property | Rule |
| --- | --- |
| granularity | the atomic group; a single command is a group of one |
| all-or-nothing | a group either commits whole or leaves the design exactly as it was, and a failure names the command that broke it |
| undo | reverses the group as one step, restoring **semantics and not only geometry** — the recipe, the identity of every entity and the revision counter |
| redo | re-applies the same group; a redo is not a new authoring act and carries the original actor |
| depth | a declared bound, not an unbounded history; exceeding it drops the oldest group and reports that it did |
| across instances | an evaluated instance and its artifacts are discarded, not undone — recomputation is the inverse of evaluation |
| across a save | undo history is not canonical content (ontology §7), so reopening a project starts a fresh history at the saved revision |

The last three rows are the ones an implementation gets wrong by accident, so they are stated as rules:
undo is a property of the *design*, and neither a derived instance nor a reopened file inherits a history.

Restoring a revision counter must also restore the exact snapshot that counter names. A new edit
after undo must never reuse a counter that named a different snapshot: an old optimistic precondition
would then match different content. Undo must preserve a separate maximum-issued counter for fresh
authoring; redo restores the original snapshot/counter. Storage/reopen must retain that issuance
state independently of the noncanonical undo stack. Metadata undo/branch native checks pass under
G1-SLICE.6.7.1; D285 storage/reopen acceptance remains owned and pending in G1-SLICE.7.

The [typed group-input guide](command-execution.md#typed-mutation-group-inputs) covers implemented
request consistency, sequential metadata previews, atomic metadata-group publication and bounded
actual-snapshot undo/redo. Persistence, other handlers and broader integration remain later work.

## 4. Preview and commit

A mutation is offered in two phases, because a patternmaker must see the result before accepting it:

- **preview** runs the command against a candidate state and returns the diff — the entities added, removed
  and changed, the measurements that moved, and the diagnostics the commit would raise. It changes nothing
  and needs no authority beyond `inspect`.
- **commit** applies it, atomically, and advances the revision.
- A preview that succeeds and a commit that fails are both possible and both honest: the commit re-checks
  the precondition (§5), because another actor may have moved the revision in between.
- A preview is never cached as an approval, and a UI that shows a preview must show that it is one. The
  distinction is a safety property: a previewed allowance is not an applied allowance.

## 5. Revision preconditions, idempotency and the audit trail

- **A mutation carries the revision it was authored against.** A stale revision is refused with
  `command_revision_stale`, naming both revisions and the commands that moved it. This is optimistic
  concurrency, and it is the only way three front-ends and an agent can edit one design without silently
  overwriting each other.
- **A retry is not a second edit.** Every mutating command carries an idempotency key, so a transport retry
  after a timeout applies once; a second submission of the same key returns the first result and says it did.
- **Every mutation is in the audit trail with its actor** (roadmap §10): the command, its arguments, the
  actor, the resulting revision and the diagnostics. The trail is append-only and is not the undo history —
  undo restores state, the trail records what happened, and a design whose undo history is empty still has a
  complete trail.
- **Imported files are data, never instructions** (§7.8): no command is constructed from file content
  without an actor, and an agent may not launder a file's contents into a mutation.

## 6. Structured errors and progress

- **Errors are structured**: a stable token, typed arguments with units, and the clause that was broken —
  never prose an agent must parse ([i18n §3](i18n-architecture.md)). A localized sentence is a presentation
  field over the same record.
- **A failure names the command and the argument.** `command_argument_invalid` carries the argument, the kind
  wanted and the kind given, so a front-end can highlight a field instead of showing a dialog.
- **Long operations report progress** — recipe evaluation over a size run, an offset over a pathology case,
  an export of a full size set — as a declared fraction with the step it is on, and they are **cancellable**:
  a cancelled operation leaves the design exactly as it was, which is the same guarantee an atomic group
  gives (§3).
- **A refused command produces no partial state and no partial artifact.** The envelope's rule
  ([envelope §10](feature-matrix.md)) holds here too: no geometry accompanies a refusal.

## 7. Agent authority as a command-layer concept

Five distinct command permissions imply no hierarchy. The census keys names, not descriptions.
Core enforces each permission and scope; a tool manifest cannot widen them.
Native mutation preview requires Inspect alone. Propose is separate proposal-workflow metadata.
Typed requests grant no execution authority.

| Level | May issue | May not |
| --- | --- | --- |
| `inspect` | every `query` | change anything |
| `propose` | proposal workflows, where implemented; native preview uses `inspect` | imply `inspect` or `commit` |
| `commit` | `mutation` commands, within a scope a human granted | approve a release, or exceed the granted scope |
| `generate` | `evaluation` and `artifact` commands, and `PrepareRelease` | approve |
| `approve` | `ApprovePackage` | be held by an agent at all |

- **`approve` is human-only** and no graph mutation can manufacture it
  ([release §7](release-contract.md)): an approval record whose approver is not an authenticated human is invalid, and an
  agent holding every other level still cannot produce one.
- **Authority is scoped, and a scope narrows.** A grant names the commands, the design and the duration;
  an agent acting under it carries the granting human's identity in the audit trail as the actor's
  principal, so responsibility is never ambiguous.
- **MCP is stdio-first with per-profile admission evidence** (roadmap §7.8/§10), and its tool count is curated: a tool is a
  coherent workflow, a batch operation or a query resource, never one tool per getter. A façade that
  mirrors the command set one-for-one is an API surface nobody can review.

### 7.1 Connection profiles and admission

Roadmap v0.4 permits the following connection profiles to land incrementally from G1. The admission
gate follows the actual trust boundary; it does not wait for G5. Local stdio is the G1 baseline.
Additional network and relay access is opt-in. No profile bypasses the command bus or core authority.

| MCP profile | Exposure | Evidence owner |
| --- | --- | --- |
| `local-stdio` | Owned child process and pipes | `G1-SLICE.9.transport.stdio` |
| `local-http` | Explicit loopback listener and client origin | `G1-SLICE.9.transport.http` |
| `remote-http` | Authenticated network caller over TLS | `G1-SLICE.9.transport.remote` |
| `private-bridge` | Authenticated relay and its private target | `G1-SLICE.9.transport.bridge` |

Every profile needs a documented threat model and executable admission tests before enablement.
Its record names the tested host/version, caller identity, permission and project/filesystem scopes,
credential issue/revocation where applicable, binding/origin rules, resource bounds and lifecycle.
Tests exercise denied access, scope escape, human-only approval, injected data, artifact overwrites,
disconnect/retry and command-state parity. A bridge includes both its remote caller and local target;
stdio at its inner hop grants no exemption. G5 proves host UI/workflow parity and G6 hardens security
and reliability. Policy admission and platform compatibility alone certify no implemented profile.

The command-layer census checks profile coverage against the roadmap and rejects the retired
stdio-only/G5+ admission rule in these normative sections. Runtime evidence remains with the owners.

The core now provides [exact command scopes and checked narrowing](command-scopes.md).
Core actor/permission/revocation checks use a [trusted host snapshot](command-scopes.md#core-authorization).

## 8. Workflow parity, and the table that proves it

**UI ↔ API ↔ MCP workflow parity is the invariant**: a workflow a user can complete in the UI can be
completed through the API and through MCP, and the three produce the same state. Parity is proven by a
**generated** coverage table, not by hand-written triplication, because a table three people maintain is
three tables.

The table's columns, so G1 can populate it mechanically:

| Column | Source |
| --- | --- |
| workflow | a named sequence of commands, declared once, in the workflow registry |
| command class | §1, derived from the commands in the workflow |
| authority required | §7, the complete exact permission set required across workflow steps |
| ui | the adapter entry point, or `absent` with the reason |
| api | the `sc-api` method, or `absent` with the reason |
| mcp | the tool name, or `absent` with the reason |
| verified by | the test id that completes the workflow through each present adapter |
| state parity | the digest of the resulting design, identical across adapters or the difference explained |

Generation rule: **every row comes from the workflow registry and the command table**, and a workflow whose
commands are all present in an adapter is marked present only when a test id exists for it. An `absent` cell
must carry a reason and an owning leaf, which is the same shape the [envelope](feature-matrix.md) uses for a
deferred row — a gap with a name is a plan, and a blank cell is not.

A cell carries one of two things, and the vocabulary is closed:

| Cell value | Meaning |
| --- | --- |
| `absent` | the adapter has no entry point for this workflow, with a reason and an owning leaf beside it |
| an entry-point name | the adapter exposes it, and the next column names the test that proves it |

The parity claim is about **workflows**, not commands: three adapters may expose different groupings and
still be at parity, and an adapter that exposes a command no workflow uses has added surface without adding
capability.

The director reaffirmed comprehensive external-agent control on 2026-10-01: supported inspection,
authoring, preview/commit, evaluation, recovery and artifact workflows must be reachable through
the API/MCP adapters with discoverable typed contracts, units, current identities and actionable
diagnostics. The shared command bus supplies behavior; tools expose coherent workflows. Discovery,
invalid-input correction, interruption/resume and provenance explanation need agent evaluation
and independent artifact checks (roadmap §7.8). MCP access alone is not measured stitching expertise.
The authority contract in §7 and parity evidence requirements above still apply.

### Host connection options under review

DBIQNP research reviewed on 2026-10-24, owned by G1-SLICE.9.plugin, identifies candidates for
the proposed conversational frontend. These are host capabilities, not implemented StitchCAD
integrations. The profile admission contract in §7.1 governs their implementation.

| Host or connection path | Documented option |
| --- | --- |
| Local Codex and ChatGPT desktop | stdio or Streamable HTTP; [client documentation](https://learn.chatgpt.com/docs/extend/mcp?surface=cli) |
| ChatGPT web | Remote streaming HTTP or SSE; [connection documentation](https://developers.openai.com/api/docs/guides/custom-mcp-server) |
| Private server through Secure MCP Tunnel | Outbound HTTPS bridge to a local stdio or HTTP server, without public ingress; [tunnel documentation](https://developers.openai.com/api/docs/guides/secure-mcp-tunnels) |
| Desktop-only plugin distribution | Local stdio packaging; public submissions require approved-partner access; [submission documentation](https://developers.openai.com/plugins/deploy/desktop-only) |

The local desktop and tunnel-to-stdio paths can preserve StitchCAD's stdio server. This corrects
the earlier proposal's overly broad network-transport dependency. A tunnel adds a remote call
path, credentials and lifecycle obligations; its permissions and workspace access need verification.
Private testing and public directory distribution have different requirements. Host UI rendering
and workflow parity also need separate tests. Assess these candidates before deciding whether
StitchCAD needs an HTTP listener. Every selected profile needs the admission evidence in §7.1;
the roadmap no longer delays a ready profile until G5 because of its transport.

### UI surfaces and presentation quality

The director's DBIQNP discussion on2026-10-25 confirms the quality bar for every selected platform:
sleek, smooth, beautiful, efficient, engaging and precise, state-of-the-art and production-grade.
These are future acceptance requirements; no StitchCAD application UI is implemented yet.

The roadmap already plans a full native application on Windows/macOS/Linux and a WASM web app.
ADR-0002 recommends Tauri with TypeScript/React chrome and a shared Rust domain core; the canvas
hosting topology remains subject to G1's realistic native/browser spike. G5 owns the complete2D
product UX. V1 adds staged3D assembly views. The web profile has an explicit capability matrix;
quality must hold within each admitted profile, with required workflows visibly routed to an
appropriate product surface when a host cannot support them.

Current host documentation shows richer options than text-only tools. OpenAI documents interactive
[iframe web components](https://developers.openai.com/plugins/build/app-quickstart) and
[fullscreen editing canvases](https://developers.openai.com/plugins/build/chatgpt-ui). Anthropic
provides [sandboxed interactive connectors](https://support.claude.com/en/articles/13454812-use-interactive-connectors-in-claude)
and lists a [Three.js3D viewer](https://claude.com/marketplace/connectors/threejs-3d-viewer).
These reviewed examples establish interactive host surfaces. StitchCAD's renderer, GPU access,
input/focus behavior, persistence, large-project performance and full workflow parity still need
per-host evidence; a generic3D demo does not establish those product capabilities.

The director accepted this direction on2026-10-25: a full standalone StitchCAD interface is the product baseline,
with chat integrations sharing its core, commands, revisions and authority. Embedded previews and
controls can provide another access path. Host restrictions should lead to a documented fallback
or opening the complete product view, while the shared command API retains supported operations.
The adopted decision at docs/decisions/decision_stitchcad-ui-baseline-and-quality.md records
the baseline and quality bar. Stack/canvas selection and host admission remain with their existing
spikes; the implementation frontier is unchanged.

Translate the quality bar into observable design and interaction criteria:

- A coherent visual system: typography, spacing, hierarchy, color, legible units and accessible states.
- Smooth pan/zoom, picking and3D camera movement, with interaction latency/frame pacing measured on
  representative projects and declared hardware; clear progress/cancellation for expensive work.
- Efficient keyboard/direct manipulation, predictable selection/snapping, undo/redo and recovery.
- Precise coordinates, dimensions, tolerances and feedback; rendering precision demonstrated at
  intended working scales, with visually clear previews and committed state.
- Engaging, calm workflows with useful progressive disclosure and polished loading/error/empty states.

The director also makes full AI coding-agent control critical across desktop, web and2D/3D graphics.
Include view/camera/pan/zoom, picking/selection, layers/overlays, tool modes, measurement/editing,
files/export, undo/redo and recovery. Agents must inspect document and presentation state, identify
objects/views, discover valid actions and verify completion, diagnostics and visual/artifact results.
Typed API/MCP/client control adapters must cover supported workflows; expose presentation controls
alongside shared domain commands. A supported operation left reachable only through an unexposed GUI
action does not satisfy this bar. Existing authority/revision/approval contracts still apply.

G1-SLICE.9.plugin retains this direction; G1.9/.13/G5/V1 own full control contracts, state visibility
and applicable end-to-end UI-agent workflow parity on each admitted surface.
No UI implementation or extra broad verification is started by this DBIQNP note.

## 9. Diagnostics

| Token | Raised when | Required arguments |
| --- | --- | --- |
| `command_unknown` | no command of that name exists | the name given, the nearest declared names |
| `command_argument_invalid` | an argument's kind or value is outside its declaration | the command, the argument, the kind wanted, the kind given |
| `command_revision_stale` | the precondition's revision is not the current one | both revisions, the commands that moved it |
| `command_precondition_unmet` | a structural fact the command needs does not hold | the command, the fact, the entity it concerns |
| `command_authority_insufficient` | the actor's level is below the command's | the actor, the level held, the level needed |
| `command_group_failed` | an atomic group broke part way and was rolled back | the group, the command that failed, the commands rolled back |
| `command_not_cancellable` | cancellation was requested where the command cannot be interrupted | the command, the phase it was in |
| `command_idempotency_replay` | a key was resubmitted; not an error, and reported as a replay | the key, the first result, the resubmission's actor |

## 10. Verification status of the claims in this chapter

- **The five command classes, the shape of a command, the undo granularity, the two-phase preview, the
  revision and idempotency rules, the five authority levels and the parity table's columns are project
  decisions**, recorded in `docs/decisions/decision_command-layer-contract-and-undo-granularity.md`. The
  roadmap fixes the requirements (§4.4, §7.8, §10); this chapter fixes the contract that satisfies them.
- **The five commands the roadmap names are carried, derived rather than read**:
  `docs/tasks/artifacts/command_layer/run_command_layer_census.sh` parses §4.4's own example list and
  refuses a command the roadmap names that §1.1 does not carry; checks every §1.1 row's class, authority
  and reversibility against the vocabularies §1 and §7 declare; compares §7's five levels with the five
  roadmap §7.8 names, in both directions; and refuses a parity column with no declared source.
  `run_command_layer_probes.sh` is its ground truth.
- **Typed catalog metadata, GetPieceList and three metadata handlers with preview/atomic in-memory
  group publication are implemented; full execution remains pending.** The set is a contract for
  `G1-SLICE.6`. The census checks specification closure; the native catalog target compares metadata.
  Neither establishes command execution. A new command is a row here first.
- **The parity table stays empty until adapter evidence exists.** G5 owns its generated rows.
  [Native workflow declarations](command-workflows.md) describe tested core sequences; they establish
  no UI/API/MCP presence or resulting-state digest parity.
- **The undo depth bound is declared but not numbered here.** It is a resource limit whose value belongs
  with the resource bounds roadmap §10 requires at G1, and a number written at G0 would be a guess; what is
  normative is that the bound exists, that exceeding it drops the oldest group, and that dropping is
  reported rather than silent.

## 11. What must be true in tests

- No mutation path bypasses the bus: the domain types expose no public mutator, and a test that reaches into
  one fails to compile rather than failing an assertion.
- Every command the roadmap names is carried, and every command carried declares a class, an authority, a
  reversibility and a granularity — both directions, on every build.
- An atomic group that fails part way leaves the design byte-identical to its prior state, and the
  diagnostic names the command that broke it and the commands rolled back.
- One undo restores semantics and not only geometry: the recipe, every entity's identity and the revision
  counter, verified by re-evaluating the recipe rather than by comparing contours.
- A stale revision is refused naming both revisions; a resubmitted idempotency key returns the first result
  and reports the replay; a cancelled long operation leaves no partial state.
- A preview changes nothing observable, including the revision counter.
- An agent holding `inspect`, `propose`, `commit` and `generate` still cannot produce an approved package,
  and an approval record whose approver is not an authenticated human is refused.
- The parity table is generated from the registry and the command table: a workflow with no test id is not
  marked present, and an `absent` cell with no reason and no owning leaf fails the generation.
