# The command layer

> **Status:** normative specification, gate **G0** (roadmap §4.4, which requires undo/redo semantics to be
> defined *at G0*, and §7.8 for the authority levels). Implemented by `sc-api` from gate G1
> (leaves `G1-SLICE.6`, `G1-SLICE.10`), with workflow parity proven at G5. Terms are defined in the
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

- **A mutation is expressed in the recipe, never as a geometry edit.** `SplitSeamSpan` splits a span and the
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
| `SplitSeamSpan` | `mutation` | `commit` | `yes` | one group, however many references move | splits a span, resolving every reference by the identity contract |
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

Five levels, ordered, each a permission on a *class* of commands rather than a description of a tool. They
are enforced in the core, not in a tool manifest, so an adapter cannot widen one.

| Level | May issue | May not |
| --- | --- | --- |
| `inspect` | every `query` | change anything |
| `propose` | build a candidate group and request a preview | commit it |
| `commit` | `mutation` commands, within a scope a human granted | approve a release, or exceed the granted scope |
| `generate` | `evaluation` and `artifact` commands, and `PrepareRelease` | approve |
| `approve` | `ApprovePackage` | be held by an agent at all |

- **`approve` is human-only** and no graph mutation can manufacture it
  ([release §7](release-contract.md)): a package whose approver is not a human identity is invalid, and an
  agent holding every other level still cannot produce one.
- **Authority is scoped, and a scope narrows.** A grant names the commands, the design and the duration;
  an agent acting under it carries the granting human's identity in the audit trail as the actor's
  principal, so responsibility is never ambiguous.
- **MCP is stdio-only until a threat model exists** (§7.8), and its tool count is curated: a tool is a
  coherent workflow, a batch operation or a query resource, never one tool per getter. A façade that
  mirrors the command set one-for-one is an API surface nobody can review.

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
| authority required | §7, the strictest level the workflow needs |
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
- **No command in this chapter exists yet.** The set is a contract for `G1-SLICE.6`, and the census checks
  the *specification's* closure, not an implementation's; a command added at G1 is a row here first.
- **The parity table is empty on purpose**, as the canvas spike's results are: its columns and generation
  rule are normative now, its rows are G5's evidence, and a table filled in before the adapters exist would
  be a claim about software nobody has written.
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
  and a package whose approver is not a human identity is refused.
- The parity table is generated from the registry and the command table: a workflow with no test id is not
  marked present, and an `absent` cell with no reason and no owning leaf fails the generation.
