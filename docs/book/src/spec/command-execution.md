# Implemented command execution

The [command-layer contract](command-layer.md) defines the complete intended surface. This page
describes the private owner, metadata query and Measurement/Material/Notch metadata inspect-previews.

## First executable read and private ownership

CommandBus now owns one actual DesignGarment and the trusted application's CommandAuthoritySource.
The submitted request cannot replace that provider. There is no public model getter, mutable model
borrow, replacement method or commit token. Construction moves the original aggregate without
reconstructing held references. This is an in-process owner; it does not authenticate a transport.

Only GetPieceList currently executes. is_available reports that exact subset independently of the
17-command typed catalog. Each execute call authorizes against a fresh trusted snapshot, then checks
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
readiness. AssignMaterial inspect-preview is described below. Mutation publication, other handlers,
idempotency, audit, undo and runtime/adapter parity remain separately owned implementation work.

## Actual Material assignment preview

CommandBus::preview now accepts AssignMaterial under fresh inspect authority; commit permission is
unnecessary. is_preview_available reports ChangeMeasurement, AssignMaterial and AddNotch,
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

Authorization and exact Design/revision binding run first. Unsupported preview handlers refuse.
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
a future commit must recompute and reauthorize at its own effect boundary.

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
report. This preserves the existing single-command behavior while providing internal ownership for
later sequential group staging. It does not publish a model, expose a replacement capability, advance
revision/identity allocation or implement group success. Paired seam splitting and Dart execution
retain their actual recipe/geometry dependencies; sewing ease realization remains deferred to G2/G3.

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
a preview nor executes a group. Sequential private staging and whole-group failure handling follow.

