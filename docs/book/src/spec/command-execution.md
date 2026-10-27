# Implemented command execution

The [command-layer contract](command-layer.md) defines the complete intended surface. This page
describes the implemented private owner, metadata query and Material inspect-preview subset.

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
unnecessary. is_preview_available reports AssignMaterial only, separately from is_available.
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
let report = bus.preview(&request)?;
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

