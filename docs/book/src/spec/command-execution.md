# Implemented command execution

The [command-layer contract](command-layer.md) defines the complete intended surface. This page
describes the private owner, metadata query and individual or grouped Measurement/Material/Notch
metadata inspect-previews and atomic in-memory metadata-group commits.

## First executable read and private ownership

CommandBus now owns one actual DesignGarment and the trusted application's CommandAuthoritySource.
The submitted request cannot replace that provider. There is no public model getter, mutable model
borrow, replacement method or commit token. Construction moves the original aggregate without
reconstructing held references. This is an in-process owner; it does not authenticate a transport.

execute currently implements GetPieceList; is_available describes that entry point. commit_group
implements the three metadata mutations with finite edit capacity and installed operation policy. Handler availability
is independent of the17-command typed catalog. Each execute call freshly authorizes, then checks
the requested Design and exact revision against the private owner. Authorization failures preserve
their original typed cause. DesignMismatch and RevisionMismatch retain the command and both actual
and requested values; counters beyond128 bits remain exact. Other authorized, correctly bound
requests return HandlerUnavailable, including mutations. No command is silently treated as applied.

With an actual garment, installed trusted provider and exact snapshot claims:

```rust
let bus = CommandBus::new(garment, trusted_authority);
let request = CommandRequest::Query(CommandQuery::new(
    CommandSnapshotContext::new(design_id, actor, exact_revision),
    QueryPayload::GetPieceList,
));
let CommandResult::PieceList(list) = bus.execute(&request)?;
assert_eq!(list.design(), design_id);
let authored_pieces = list.pieces();
```

CommandPieceList borrows the complete original Piece slice in authored order and its represented
revision. Piece identities, multiplicity, pairing and assigned or unresolved Material intent remain
unchanged. The result cannot outlive the owner. This metadata read changes no model, cache, revision
or identity allocator. A later query rechecks grants, including revocation; a previous result is no
authorization token. Already delivered read data does not become a revocable capability.

Inspection remains available for modelled physical deferrals and held topology repairs. Reading the
authored inventory does not validate those sources or certify geometry, envelope support or release
readiness. Individual/group metadata previews and commits are described below. Other handlers,
persistence and runtime/adapter parity remain separately owned implementation work.

## Trusted metadata operation policy

Before costly metadata previews, commits or history traversal, the host installs explicit positive
CommandOperationLimits exactly once. Authorized GetPieceList reads need no policy. For example,
with host-chosen NonZeroUsize maxima:

```rust
bus.install_operation_limits(CommandOperationLimits::new(
    maximum_members, maximum_metadata_targets, maximum_assignment_reason_bytes,
    maximum_measurement_evidence_references,
))?;
```

Missing policy refuses with CommandControlError::LimitsNotInstalled; attempted replacement returns
LimitsAlreadyInstalled and retains the original policy. Pure submitted-input admission precedes
private authority/model work. Actual progress, observer methods, cancellation/retry semantics and
the precise limits of these metadata bounds are documented in [Operation limits and progress](command-operation-control.md).

## Actual Material assignment preview

CommandBus::preview now accepts AssignMaterial under fresh inspect authority; commit permission is
unnecessary. With installed operation policy, is_preview_available reports the three metadata handlers,
separately from is_available. All three prepare metadata; recipe/geometry execution remains separate.
The request carries the actual Design, actor, exact revision, retry key and explicit Piece selection.
Constructing it grants no authority. With a supported actual garment and trusted provider:

```rust
let assignment = AssignMaterial::new(
    PieceSelection::new(vec![back_piece, front_piece])?,
    MaterialAssignment::Assigned(material_id),
)?;
let request = CommandRequest::Mutation(CommandMutation::new(
    CommandMutationContext::new(
        design_id, actor, exact_revision.clone(), retry_key, None,
    ),
    MutationPayload::AssignMaterial(assignment),
));
let CommandPreview::Material(report) = bus.preview(&request)? else {
    unreachable!("this typed request is AssignMaterial");
};
assert_eq!(report.revision(), &exact_revision);
for change in report.changes() {
    // Original selected Piece identity and exact assignment values, in selection order.
    let (piece, before, after) = (change.piece(), change.before(), change.after());
}
```

After pure input admission, authorization and exact Design/revision binding precede private work.
Unsupported preview handlers refuse.
The explicit garment execution envelope then runs before complete current-source validation and
target lookup: modelled Lining retains its original env_lining/G7 refusal. Every selected Piece must
exist; MissingPiece retains its identity and zero-based selection index, including later targets.
The bus replaces only Material assignment on copied held Pieces, preserving labels, topology
references, pairing and multiplicity. It rebuilds the actual structural/input/sewing/garment owner
and rechecks its current registry and sources. An absent assigned Material returns the original
DesignInputsError → MaterialInventoryError::MissingMaterial chain. Unknown Material flags remain
unknown, and an Unresolved assignment preserves the exact supplied nonblank reason, including spaces.

CommandMaterialPreview contains the actual Design/base revision and exact before/after assignment
changes. Exact no-ops are omitted; changed targets retain request-selection order even when it differs
from inventory order. The private candidate is discarded. The original Piece inventory and revision
remain unchanged on success and refusal; no identity, cache, history or publication is created.
The report contains no candidate model or commit capability. execute still refuses AssignMaterial;
commit_group recomputes and reauthorizes at its own effect boundary.

This is current metadata/source validation. It does not evaluate geometry, certify complete unused
Piece topology, resolve physical properties or approve a release. Focused native tests cover the real
sc-measure aggregate and installed inspect-only provider, order/no-op/reason semantics, original
error chains, revocation and unchanged owned Pieces. Full bus integration remains G1-SLICE.6.9.

## Private identity reserve and actual Notch preview

Creation uses identities reserved by the trusted application configuration. CommandBus::with_identities
owns the actual garment, trusted authority provider and explicit unused identity vector. The bus
rejects non-increasing supplied order, including duplicate identities, then the first identity already
owned anywhere in the garment, including retired creating operations. Errors retain the supplied
identity, zero-based reserve index and original owning position. No sorting or replacement occurs.

The composition root fills the reserve from its injected IdGenerator: clock/entropy for production,
deterministic allocation for replay/tests. Reserve management must ensure global nonreuse across
buses and restarts. The implemented checks establish local disjointness, not cross-process persistence.
The reserve has exactly the supplied length; there is no hidden refill or fallback sequence.
CommandBus::new has an empty reserve. Reads and Material previews need none; creation reports the
original IdError::Exhausted. Persistent issuance, bounded refill and production host integration remain
with the later publication/runtime/storage owners; this API does not claim those workflows exist.

Each creation preview borrows an independent iterator over immutable reserved values. Advancing or
exhausting that iterator cannot advance another iterator or the owned reserve. It cannot call a clock,
entropy provider or arbitrary Clone implementation. The request cannot supply a new Notch identity:

```rust
let bus = CommandBus::with_identities(garment, trusted_authority, reserved_unused_ids)?;
let intent = AddNotch::new(piece_id, exact_edge_anchor, explicit_profile_bindings);
let request = CommandRequest::Mutation(CommandMutation::new(
    CommandMutationContext::new(design_id, actor, exact_revision.clone(), retry_key, None),
    MutationPayload::AddNotch(intent),
));
let CommandPreview::Notch(report) = bus.preview(&request)? else {
    unreachable!("this typed request is AddNotch");
};
let proposed_notch = report.notch();
assert_eq!(report.revision(), &exact_revision);
```

Fresh inspect authorization, actual Design/revision, envelope and complete current-source checks
precede target lookup and staged allocation. A missing owning Piece refuses before exhaustion.
Notch::new validates the actual current edge and anchor ownership against that Piece and its ledger;
its original typed cause survives. The candidate appends the actual Notch after existing semantic
landmarks, rebuilds the sewing/garment owners and validates the complete current candidate.

CommandPreview::Notch contains only the actual Design/base revision and proposed immutable Notch,
including its staged identity, exact anchor and original sample/production/style/encoding bindings.
Profile binding validation remains DeferredToG4. No style, physical dimensions or geometry is invented.
The candidate and iterator are discarded. Repeating a preview at the same base returns the same next
reserved identity; a refused anchor consumes none. The proposed identity is tentative and grants no
reservation or later publication capability. execute still refuses AddNotch. Model/revision/owned
reserve stay unchanged, and no history, artifact or approved release is produced.

## Exact candidate differences

Reports are derived from the actual validated candidate before that candidate is discarded.
The bus compares original garment metadata, sewing graph, input registries/recipe/higher sources,
Design identity/revision, canonical declarations, cut plan and topology history. The comparisons use
exhaustive owner-definition patterns: adding a field requires explicitly including it in this coverage.
Preview hosts therefore implement Clone and PartialEq in addition to DesignReferenceHost; the
installed sc-measure source owner supplies those value semantics.

For Material assignment, Piece count and identity order must match. Every unselected Piece must be
identical. A selected Piece may change only its exact Material assignment, which must equal the
explicit intent; labels, quantities, pairing and held references are compared too. Existing semantic
landmarks must be identical. Before/after values come from the original and candidate, with changed
targets in request order and exact no-ops omitted.

For AddNotch, the complete Piece inventory must remain identical. The candidate's landmark inventory
must contain the original exact prefix followed by one actual Notch. The appended Notch must match
the requested Piece, exact anchor, all symbolic profile bindings and privately staged identity.
The report reads that actual appended object. Changes to existing landmarks or other owner content
refuse. CommandPreviewError::Difference preserves a structured CommandDifferenceError cause identifying
the failed comparison; an unexpected internal mutation cannot become a successful preview report.

Focused difference checks use real canonical Pieces, ledger and Notches, including unrelated valid
label changes, reordered Pieces/landmarks, unselected edits and mismatched assignment or Notch identity.
The existing real-aggregate bus cases still pass. These checks establish the implemented
metadata subset; other mutation differences and full integration retain their later owners.

## Actual Measurement binding and metadata preview

ChangeMeasurement now prepares an actual private metadata candidate. Fresh inspect authorization,
exact Design/revision binding, explicit envelope and complete current sources precede Measurement
lookup. The original DesignMetadata owner resolves the actual Measurement role and its borrowed
canonical dedicated LengthDeclaration. A missing Measurement or wrong metadata role preserves the
original DesignMetadataError chain. MeasurementDeclarationMismatch refuses a replacement naming
another declaration, retaining the Measurement, actual bound id and requested replacement id.

```rust
let intent = ChangeMeasurement::new(measurement_id, replacement_declaration);
let request = CommandRequest::Mutation(CommandMutation::new(
    CommandMutationContext::new(design_id, actor, exact_revision.clone(), retry_key, None),
    MutationPayload::ChangeMeasurement(intent),
));
let CommandPreview::Measurement(report) = bus.preview(&request)? else {
    unreachable!("this typed request is ChangeMeasurement");
};
assert_eq!(report.measurement(), measurement_id);
for change in report.changes() {
    assert_eq!(change.before().id(), change.after().id());
    let authored_state = change.after().state();
}
```

The bus replaces only the bound dedicated declaration at its original position, retaining exact
supplied source, authored state and provenance references. It rebuilds the actual structural/input/
sewing/garment owner and validates the complete current candidate. Shared consumers see that same
canonical declaration: for example, replacing a Measurement also used as a Material width with a
known zero value refuses with the original MaterialError::NonPositiveWidth in its source chain.
An Unknown observation or Derived formula remains a reference without a numeric fallback.

CommandPreview::Measurement carries actual Design/base revision, original Measurement id and zero
changes for an exact no-op or one CommandLengthChange with actual before/after declarations. The
bus checks every other canonical record by restoring only the original target in a copied candidate
inventory and comparing the complete owner. All other metadata, Pieces, landmarks, ordering, recipe,
cut plan and ledger must remain identical. Unexpected changes retain Difference causes and refuse.

Source, evidence, observation and formula references remain authored claims. This metadata preview
does not verify their external truth or execute the recipe, regenerate geometry or produce an Instance.
Actual expression/recipe execution remains G1-SLICE.5e/.5f; source truth remains its evidence owners.
Inspect-only preparation grants no commit permission. Success and refusal leave the original model,
canonical values, revision and identity reserve unchanged; execute still refuses ChangeMeasurement.

## Private preparation before group staging

The three metadata handlers now return one private prepared object containing their actual validated
candidate model and its derived report. The bus's private preparation entry point retains fresh
authorization, exact binding, explicit envelope/current-source checks and operation-specific difference
checks. No caller can obtain that object or invoke preparation directly; a compiler privacy check
rejects that access using the same valid host/authority bounds as the public API.

Public preview consumes the private prepared object, discards its model and returns only the owned
report. Preparation now operates through a private immutable view of the actual model and accepts
one concrete private identity cursor. The three handlers share this view; they cannot substitute an
authority provider or call a caller-supplied identity generator. A single preview borrows the original
model and creates an independent reserve cursor, preserving its existing refusal order and reports.
Group staging borrows each preceding validated candidate and passes the same cursor onward:
two successful Notch steps must use successive identities, while another preview starts at the
original reserve again. The sequential group API and focused proof are described below.

This internal refactor does not publish a model, expose a replacement capability, advance
revision/identity allocation. Paired seam splitting and Dart execution
retain their actual recipe/geometry dependencies; sewing ease realization remains deferred to G2/G3.

Whole-group preparation also retains its actual final candidate, derived group report and concrete
cursor consumption count in a private PreparedGroup. Public preview consumes it, discards the model
and allocation information, and returns only the report. A compiler privacy check rejects direct
preparation, including requests for Execute mode. No caller-supplied prepared object or mode can
be submitted for publication. commit_group performs fresh effect authorization, revision/replay
checks, atomic model/reserve transition and append-only in-memory audit recording.

## Typed mutation-group inputs

CommandMutationGroup::new(group_id, mutations) owns a nonempty original Vec of CommandMutation
values. Every member must explicitly name that group, and all must share the first command's exact
Design, complete actor claim and revision. A standalone context with group=None is refused rather
than rewritten. Human and Agent claims remain distinct even when they name the same human principal.
Revision comparisons retain exact counters beyond128 bits.

```rust
// Both original mutation contexts already name Some(group_id).
let group = CommandMutationGroup::new(group_id, vec![first_mutation, second_mutation])?;
let common_claims = group.context();
for mutation in group.mutations() {
    assert_eq!(mutation.context().group(), Some(group.id()));
    assert_eq!(mutation.context().expected_revision(), common_claims.expected_revision());
}
```

The constructor checks membership, Design, actor then revision for each command in submitted order.
NoMutations retains the requested group id. GroupMismatch, DesignMismatch, ActorMismatch and
RevisionMismatch retain the first failing zero-based index, payload-derived command identity and
exact expected/requested data. No sorting, deduplication or identity minting occurs. Payloads and
retry keys remain exact, including repeated keys as submitted data; execution must determine their
scope, payload equality and replay outcome. This input validation does not decide that policy.

id(), context() and mutations() expose immutable original data. A compiler check rejects direct
member reordering. Any typed mutation payload may be represented, including a handler that is not
available yet. This is a group input schema, not a new command class or authenticated transaction.
Constructing it neither grants permission, looks up model targets, proves current revision, returns
a preview nor executes a group. [Whole-group authority](command-scopes.md#whole-mutation-group-authorization)
is a separate core check. The bus combines it with actual private staging as described below.

## Sequential metadata-group preview

CommandBus::preview_group(&group) accepts the same three implemented metadata mutations. One fresh
trusted authority snapshot and one complete Inspect grant cover every original member, including
retries. It then binds the common Design and exact revision against the actual private owner.
DesignMismatch and RevisionMismatch preserve group identity and exact requested/current values.
An unsupported member remains an actual HandlerUnavailable refusal.

Each first occurrence runs against the preceding actual validated candidate. Every step retains the
original envelope, full current-source, target, candidate and semantic-difference checks. One independent
private reserve cursor spans the entire preview; distinct successful AddNotch members use successive
tentative identities. The original reserve, model and revision remain unchanged, including after a
later member fails. Candidate models never appear in the returned report.

```rust
let preview = bus.preview_group(&group)?;
assert_eq!(preview.group(), group.id());
assert_eq!(preview.revision(), group.context().expected_revision());
for (index, step) in preview.steps().iter().enumerate() {
    // An alias reports its first occurrence; it does not repeat the edit.
    if let Some(first) = step.replay_of() {
        assert!(first < index);
        assert_eq!(step.report(), preview.steps()[first].report());
    }
}
let net_changes = preview.difference();
```

steps() preserves every submitted position, payload-derived command identity and retry key. A key's
first request is compared with any later request using complete CommandMutation equality. An exact
repeat returns the original report and replay_of=Some(first_index), even after intervening commands;
it does not reapply the edit or allocate a second identity. A different request under the same key
returns RetryConflict with group, key, first occurrence, conflicting index and command. The alias table
exists only for this preview call. A separate preview starts again from the actual original owner;
this is no persisted retry history or cross-group replay guarantee.

difference() compares the actual original and final candidate, rather than concatenating step reports:

- lengths() returns exact changed dedicated declarations in original canonical inventory order,
  retaining source and Unknown/Derived state. Inventory identities/order and general scalars remain exact.
- materials() returns exact changed assignments in original Piece inventory order, independently of
  each request's selection order. All other Piece fields and held references remain exact.
- notches() returns actual appended Notches in append order, with exact owner, anchor, bindings and
  tentative identity. Original landmarks remain an exact unchanged prefix; retries add no duplicate.

For example, an assignment changed from A to B and then back to A has two real step reports and no
net assignment change. The native mixed-group case likewise restores a Measurement declaration while
adding two distinct Notches and replaying the first Notch request once. Its net delta contains only
those two Notches, and repeating the entire preview returns the same tentative identities.

A failed member returns CommandGroupPreviewError::Mutation with the original group, zero-based index,
payload-derived command and complete original CommandPreviewError in the standard error source chain.
All preceding candidates and reports are discarded. A later invalid anchor or exhausted reserve cannot
consume an original identity. Final-model integrity failures retain Difference and its original cause.
The six native group cases and existing individual cases are focused checks; complete bus integration
remains separately owned. Group preview supplies no commit permission, approval token, published state,
revision advance, audit/undo history, recipe evaluation, geometry or physical-property certificate.

## Atomic in-memory metadata-group commit

CommandBus::with_edit_capacity installs the actual owner, trusted authority source, unused identity
reserve and an explicit NonZeroUsize maximum accepted-group count. new and with_identities retain
read behavior; previews require installed operation policy. With that policy, owners without edit
capacity refuse commit_group with EditsNotConfigured. is_commit_available requires both policy and
configured metadata handlers; it supplies no authority, current revision, journal room or valid candidate.
The host chooses the finite capacity. Construction mints no grant or implicit commit permission.

```rust
let mut bus = CommandBus::with_edit_capacity(
    garment, trusted_authority, reserved_identities,
    NonZeroUsize::new(config.maximum_committed_groups).ok_or(configuration_error)?,
)?;
bus.install_operation_limits(limits)?;
let result = bus.commit_group(&group)?;
let accepted = result.entry();
assert_eq!(accepted.actor(), group.context().actor());
assert_eq!(accepted.request(), &group);
assert!(accepted.commit().revision() > group.context().expected_revision());
let actual_changes = accepted.commit().changes();
```

A new group requires one complete explicit Commit grant, independently of Inspect. Initial fresh
authorization and actual Design binding precede private journal lookup. Exact accepted group replay
returns the first immutable audit/result with is_replay=true, even after later commits advanced the
current revision or filled capacity. It changes no model, revision, reserve or audit entry. Returned
grant/time/diagnostics remain those of the original acceptance; they do not certify current sources.
Every replay refreshes permission. Revocation can refuse a retry that previously succeeded.

Replay identity is the original explicit group id plus complete CommandMutationGroup equality,
including actor, exact revision, every payload and retry/group context. A different request under an
accepted group id returns GroupConflict. Across different groups, a key is scoped to this Design
owner and complete initiating actor/principal. Reusing that actor's accepted key returns KeyConflict
with requested group/index/key and the previous group; overlapping groups are never partly replayed
or applied. Another actor has its own key namespace. Group ids remain unique within this owner.
Within-group exact aliases retain the preview semantics above and consume no extra identity.

For a new request, the exact revision precondition precedes capacity and candidate work. Precondition
retains the original requested/current CommandGroupPreviewError and available actual state-change
events after that snapshot's original creation. Their common audit sequence orders fresh authoring,
Undo and Redo even when counters restore or trusted timestamps repeat. Each witness retains group,
resulting revision, direction and effective original commands. history_from names the owner's initial
revision; earlier activity is not fabricated. A future requested revision has no intervening events.
These records describe this in-memory owner, without a persisted history claim.

New authoring issues the exact successor of the separate maximum-issued counter. Undo/redo restore
the original snapshot counter while leaving issuance intact; a new branch therefore skips previously
issued snapshot counters. The actual candidate is rebuilt at its freshly issued revision through the original
structural/input/sewing/garment constructors and complete current validation. An exhaustive context
comparison permits only that revision change; canonical values, Pieces, landmarks and all other
fields remain equal to the prepared candidate. Unknown/Derived declarations remain authored state,
and their source/recipe/physical realization receives no new certificate from publication.

The bus constructs the remaining reserve and preflights space for one audit entry before effects.
It then refreshes whole-group Commit authority immediately before publication. Under its exclusive
mutable owner, it moves the validated model, remaining reserve and prepared audit entry together,
with no fallible operation or host callback between those moves. The old source host is retained
until all fields and the audit record are installed. A later candidate failure or effect-boundary
revocation leaves the original represented model, revision, unused identities and accepted records intact.

CommandAuditEntry retains its exact chronological effect sequence, original request, initiating actor/principal, actual accepted
step/net report, resulting revision, final complete grant and trusted observation time. It is append-only
for this owner's lifetime. The authorized commit/replay result returns its own immutable entry;
there is no unauthenticated general history/model getter. The audit and replay registry are not undo
history. A new accepted group, including a net no-op, advances exactly once and adds one record.

At the configured count bound, JournalFull refuses a new group before staging and drops no old
audit or replay data. Exact replay remains available after fresh authorization. Capacity bounds
accepted-group count; installed operation policy separately bounds declared metadata input units
and supports cancellation. Total model/work/memory and allocation fault injection remain their
resource/acceptance owners. JournalAllocation retains an actual preflight allocation error.
The six native commit cases cover actual metadata/revision/reserve publication, original actor/result,
later failure, retry/capacity/conflicts, permission separation and revocation at the second check.
Full command integration, persistence/reopen, other mutation handlers, adapters and actual
recipe/geometry/physical evaluation remain pending.

## Typed history operation inputs

CommandHistoryRequest::new(context, original_group, direction) owns a current operation context and
the exact original CommandMutationGroup. CommandHistoryDirection is Undo or Redo. The context
retains an explicit current operator, exact expected revision, retry key and Some(original_group.id()).
The constructor checks group membership first, then Design consistency. GroupMismatch and
DesignMismatch retain the original expected/requested values; absent membership is not rewritten.

```rust
let request = CommandHistoryRequest::new(
    current_operation_context, original_accepted_group,
    CommandHistoryDirection::Undo,
)?;
let operator_claim = request.context().actor();
let original_author_claim = request.original_actor();
let exact_original_members = request.original().mutations();
```

The current operator and revision may differ from the original author and base revision. For example,
a human may request traversal of a group originally authored by an agent. Both complete claims remain
distinct; the agent's principal is not substituted for its identity. The original group keeps its exact
actor, revision, order, payloads and retry aliases. Undo and Redo remain different intents even if a
caller submits the same operation key. Runtime must decide key conflicts and exact result replay.

context(), original(), original_actor() and direction() expose immutable submitted data. A compiler
check rejects replacing the original group directly. Constructor success does not authenticate either
claim, bind an accepted audit record, establish a current revision, grant authority or traverse history.
Runtime must authenticate the current operator over all original command scopes, check the exact
accepted original group, current revision and operation key, then enforce bounded atomic traversal.
The three native schema cases/privacy check are focused input proof. Actual bounded traversal and
native D285 branch checks are described below; storage/reopen proof remains pending.

## Bounded actual-snapshot undo and redo

with_edit_history installs explicit NonZeroUsize bounds for accepted authoring groups, reversible
history depth and accepted history actions. History starts empty. with_edit_capacity supports authoring
without traversal; with installed operation policy, apply_history then returns NotConfigured.
The host chooses each finite bound and installs metadata operation policy separately.

```rust
let mut bus = CommandBus::with_edit_history(
    garment, trusted_authority, reserved_identities,
    maximum_authoring_groups, undo_depth, maximum_history_actions,
)?;
bus.install_operation_limits(limits)?;
// Commit returned the exact original group and its accepted revision.
let undo = CommandHistoryRequest::new(
    current_operator_context, original_group.clone(), CommandHistoryDirection::Undo,
)?;
let result = bus.apply_history(&undo)?;
assert_eq!(result.entry().original_actor(), original_group.context().actor());
assert_eq!(result.entry().operator(), undo.context().actor());
let restored_revision = result.entry().revision();
```

Each committed group moves its actual predecessor model into the bounded undo stack. Undo and Redo
swap whole actual owned alternatives between the current owner and the two stacks. They preserve the
complete original model, held references, host, semantic identities and represented revision. Native
roundtrip checks verify original and accepted Piece-slice addresses as well as canonical declarations,
assignment values and counters. No inverse is reconstructed from a display diff. Issued entity ids and
the maximum-issued authoring counter never rewind, and a redo is not a new authoring act.

The supplied original group must equal the complete accepted authoring record and be the next group
on the selected stack. Unknown, changed and out-of-order groups retain GroupNotAccepted, GroupConflict
and NotCurrent; an empty selected stack retains Empty. Current revision must match the operation
context exactly. Precondition carries the original mismatch and common ordered state-change witnesses.
No partial traversal or record is accepted when any check refuses.

The current operator is authenticated against the actual installed source for every original command
scope, initially and at the final effect boundary. The authorization-only group keeps original command
payloads and uses the current operator context; the full supplied original is then bound to the actual
accepted record. A human with complete Commit scope can traverse an agent-authored group. The original
agent author/principal remains unchanged in the result/audit. A grant covering only Material cannot
also undo a Notch in that group. Partial grants cannot combine coverage.

Before effects, the bus checks the explicit current and alternative envelopes/current sources and
preflights the destination stack and immutable action-record allocation. Exclusive model/stack/action
moves contain no fallible operation or host callback. Original authoring audit/replay records remain
intact. History keys share the Design/complete-operator namespace with authoring keys: exact operation
retry returns its first immutable result with is_replay=true, while different direction/context/group
under the same accepted key returns KeyConflict. Exact replay refreshes authority before current
revision, stack or capacity checks and changes no model or audit sequence.

CommandHistoryAuditEntry retains the exact operation, current operator, original author, before/restored
revisions, effective original commands, final complete grant/time and common exact audit sequence.
Authoring and history sequences advance only for new accepted effects, independently of wall clocks
and restored model counters. A full action journal refuses a new traversal with ActionLimit; existing
records are never dropped to make room, and exact authorized replay still works.

When a new authoring act exceeds undo depth, commit.history().dropped_undo() names the oldest lost
undo group. A new branch also reports discarded_redo() in redo application order. Dropped alternatives
are discarded only after the complete model/reserve/journal/stacks transition. Their original authoring
audit/replay and accepted history-action records remain available to exact authorized retries.

The native D285 case commits snapshot1, undoes to snapshot0, then creates different content at freshly
issued revision2 and the next unused entity id. An old request for snapshot1 refuses; its witnesses
include Undo and the new branch in exact audit order. Storage must persist/restore issuance and audit
ordering independently of noncanonical undo history; actual reopen proof remains G1-SLICE.7 work.

Seven native history cases plus the prior30 bus cases pass focused checks. Compiler privacy rejects
direct stack/action-journal erasure. Persistence/reopen, complete model/work/memory bounds, deeper
source/allocation fault acceptance, full command integration and recipe/geometry/physical proof remain
separately owned; metadata history supplies no broader certificate.

[Operation limits and progress](command-operation-control.md) describes installed immutable metadata
input policy, actual pre-effect progress and atomic cooperative cancellation. The coupled bus now has
46 focused native cases. Submitted Measurement evidence references have their own explicit quota;
eight bus privacy boundaries and control privacy were previously checked. Full integration remains deferred.

Candidate reconstruction now copies the trusted source host once per actual non-alias member, plus
once for a new commit's fresh revision. Exact replay and actual Undo/Redo require no reconstruction
copy. The [resource-control guide](command-operation-control.md#source-host-copy-work) records the
native copy-count proof and its limits. There are46 focused bus cases; total resource proof remains owned.
