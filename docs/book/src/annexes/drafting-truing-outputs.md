# Native truing output and source contracts

This companion follows the [constrained seam-truing input contract](drafting-edit-inputs.md#native-constrained-seam-truing-inputs).
That input chooses an adjusted side and pattern Piece,retains the actual span and selected copy,
and borrows an explicit sourced nonnegative T2 length-fit requirement. Its endpoint/stop locks
specify permissions,not a unique fitted curve. Actual fitting and complete variant coverage remain
separate obligations.

## Associate the actual complete copy and Piece inventory

`BoundConstrainedTruing::bind_copy_inventory` borrows the complete actual `PieceInventory` under
`TruingCopyInventoryLimits::new(C,I,W)`. It uses the exact CutPlan already retained by the actual
input source;no second supplied plan can replace that parent. `BoundTruingCopyInventory` retains
the actual source,plan,whole inventory and original explicit limits.

```rust
use sc_core::{construction::*,ontology::PieceInventory};
fn bind<'a>(source:&'a BoundConstrainedTruing<'a>,inventory:&'a PieceInventory,
    limits:&'a TruingCopyInventoryLimits)
    ->Result<BoundTruingCopyInventory<'a>,TruingCopyInventoryError<'a>> {
    source.bind_copy_inventory(inventory,limits)
}
let limits=TruingCopyInventoryLimits::new(2,2,8);
assert_eq!(limits.workspace_entries(),8);
```

C counts every retained-plan copy;I every supplied Piece,including unrelated records. Checked
I+3C precedes whole C,I and workspace quotas,before scans or allocation. Canonical `CutPlan::new`
then validates copied definitions against this exact inventory,including complete quantities,
orientations and pair/fold policy. Its temporary proof is dropped before authentic A/B records are
cached. A plan's earlier birth against another Piece collection is not proof for this inventory.

The transient bound counts logical entries,not allocator bytes,prior immutable storage or a whole
execution budget. Association takes O((C+I)log(C+I+2)) work with constant cached side references.
It scans no nested geometry,registry or observers and solves no fit. Quota equality is accepted.
With C2/I2,the bound is8;with C2/I1,it is7. W7 refuses complete required8 for a two-Piece context
before any canonical quantity mismatch can be inspected.

After canonical validation,both original span sides are resolved in A/B order. `copy(side)` lends
the actual retained-plan physical copy;`piece(side)` lends its actual supplied pattern owner.
Neither owner is inferred from an edge id or array position. `selected_piece()` follows the
original explicit adjusted side;`counterpart_piece()` follows the other original side. These are
actual borrows,not equal-looking reconstructed Pieces. `source()` preserves the original graph,
span,direction,ease,stops,selected copy and quantitative provider,including its actual tolerance
reference and derivation. `plan()`, `inventory()`, `limits()` and `workspace_bound()` expose the
whole actual context and conservative temporary bound.

The input binding checks the selected copy only. If an otherwise canonical retained plan contains
A200 but omits the graph's B201,the new association refuses `MissingCopy { side:B, copy:201 }`.
Selecting B with A absent still refuses original A first. No partial side context escapes.
An unrelated supplied Piece102 without its required plan copy also causes canonical refusal;
filtering to the selected Piece would hide that obligation. Errors preserve the whole actual
source,plan,inventory,limits and first resource,canonical or original A/B cause.

## Shared pattern ownership is a coupled refit

Two physical copies may share actual Piece100,including authored A200 and reflected B201.
The retained plan keeps its original order,for example `[201,200]`;the A/B cache still follows
actual source identities. `same_pattern_piece()` reports shared ownership,not an independent
counterpart or geometric fit. Both selected/counterpart getters then lend the same actual Piece.
The original `affected_copies()` inventory still returns both physical copies.

A refit edits the pattern Piece,so every physical copy and any other observing seam,mark or
constraint may be affected. Shared ownership cannot silently freeze the counterpart,fork a copy,
move endpoints/stops or replace ease. Complete current Design/graph/plan/ledger ownership,
held-reference repair,observer/all-copy effects,deterministic output/curve policy,actual fit and
length-error evidence,authoring authority and atomic publication remain required. This stage
constructs no output topology and certifies no endpoint,stop,geometry or physical result.

## Bind explicit edge-output roles

`BoundTruingCopyInventory::bind_edge_roles` associates the actual complete source/copy/Piece
context with an `AdmittedEdgeOutputRoleCatalog`. It produces `BoundTruingEdgeRoles` under explicit
`EdgeOutputRoleCatalogLimits`. These are edge-role declarations;Point and semantic-Notch
catalogues are different types and cannot supply this binding.

```rust
use sc_core::{construction::*,name::MachineToken,ontology::{EntityId,LocalTag}};
fn bind<'a>(copies:&'a BoundTruingCopyInventory<'a>,catalog:&'a AdmittedEdgeOutputRoleCatalog<'a>,
    limits:EdgeOutputRoleCatalogLimits)->Result<BoundTruingEdgeRoles<'a>,TruingEdgeRoleBindingError<'a>> {
    copies.bind_edge_roles(catalog,limits)
}
let raw=EdgeOutputRoleCatalog::new(EntityId::from_bits(1000),
    [(9,"refit"),(2,"retained"),(7,"inactive")].into_iter().map(|(tag,key)|
        EdgeOutputRoleDefinition::new(LocalTag::new(tag),MachineToken::new(key).expect("literal key"))
    ).collect(),vec![LocalTag::new(2),LocalTag::new(9)]);
let catalog=raw.admit(EdgeOutputRoleCatalogLimits::new(3,2,21)).expect("bounded literal metadata");
assert_eq!(catalog.key_bytes(),21);
```

The whole D-definition,A-active-occurrence and B-key-byte quotas precede operation equality.
D includes inactive definitions;A counts every original active occurrence;B is the catalogue's
complete cached original machine-key byte count. A valid catalogue for operation1001 still
refuses binding to actual truing operation1000. D2 refuses required3 before A1,B20 or operation
inspection;with D3/A2,B20 refuses required21 before that owner mismatch.

Association reads cached counts and original operation fields only. It takes constant work and
space,scans no keys,allocates nothing and mutates neither original input. Earlier catalogue
admission and owned storage retain their separate full-population/workspace obligations.
`copies()`, `catalog()` and `limits()` lend the actual complete parents and original explicit
policy. Catalogue definition/active order,machine keys,local tags and authentic member borrows
stay unchanged. Errors retain the same whole copy/Piece/source and catalogue context,policy and
first complete quota or operation cause;no accepted prefix escapes.

Empty metadata and a catalogue containing only inactive definitions can both bind. They do not
prove an empty refit result or erase either physical copy,shared-Piece coupling,stops or the actual
fit provider. Active membership likewise supplies no evaluated curve or issued identity. Complete
output manifests,reference/endpoint/stop mappings,deterministic variant/refit policy,actual history,
geometry/length-fit evidence,current Design,all-copy effects and atomic authoring authority remain
required. A caller-supplied role tag cannot grant topology issuance or canonical publication.
