# Native slash/spread Piece reconciliation

This companion continues the [output and reference-disposition contracts](drafting-slash-output-contracts.md).
Native declarations retain original owners and request evidence. They do not create a geometrically
valid replacement Piece,change canonical metadata or authorize publication. The nonempty changed
contour path is available;actual copy reconciliation,effect and zero/no-new-edge outcomes remain
separately owned native work. Actual geometry/current Design/semantic repair stays G2/G3/Design/bus.

## Canonical source-metadata preservation

`SlashSpreadPieceMetadataRequirement::preserve_source(fold,fold_correspondence,all_copy_effects)`
explicitly requests preservation of the actual source Piece's identity,total cut quantity,mirroring
and companion metadata,cutting side,material assignment,assembly layer and authored label. It stores
no cloned metadata,second cut quantity or independent label-derived hand/fold authority. There is
no default preservation request. Unrelated authored metadata changes keep their canonical owners.

The two required evidence-source identities request actual fold correspondence and effects on
every physical-copy/semantic/sewing consumer. They are original caller fields,not provider lookup,
accepted evidence or a certificate that copies or consumers have been reconciled.

`required_boundary.preserve_piece_metadata(&request)` requires the actual complete boundary/
consumer/source/contour context. A successful private `BoundSlashSpreadPieceMetadata` borrows that
original source Piece and exposes its canonical metadata by reference:

| View | Authoritative original field |
| --- | --- |
| `piece_id()`, `quantity()`, `mirroring()` | Actual source identity,total cut copies and handedness/companion metadata |
| `cut_on_fold()`, `cutting_side()` | Actual source fold state and fabric cutting side |
| `material()`, `layer_index()`, `label()` | Exact source assignment/reason,layer and authored label text |
| `printed_label()` | Source label with quantity,L/R and fold indicators derived from the same canonical cut metadata |

Borrowing preserves exact unresolved material reason text and label spelling. A cut-four mirrored
source remains cut-four with paired metadata. No label text can silently replace that quantity or
handedness. `source_piece()` exposes the original owner;its old contours are context,not the proposed
output contours. The boundary owner retains the actual proposed contour declaration separately.

```rust
use sc_core::construction::*;
fn preserve_metadata<'a>(boundary:&'a RequiredSlashSpreadBoundary<'a>,
    request:&'a SlashSpreadPieceMetadataRequirement)
    ->Result<BoundSlashSpreadPieceMetadata<'a>,SlashSpreadPieceMetadataError<'a>> {
    boundary.preserve_piece_metadata(request)
}
```

## Explicit fold association

The request must explicitly choose a `SlashSpreadFoldIntent` matching the original cut-on-fold state:

| Intent | Structural association required |
| --- | --- |
| `NoFold` | Original Piece is not cut on fold;an existing fold cannot be removed through this preservation request |
| `RetainOriginal` | Original Piece is cut on fold and its exact original fold identity is retained on the proposed outer boundary |
| `Derived { tag }` | Original Piece is cut on fold;tag names an actual admitted Source-provenance candidate from its exact original fold,used on the proposed outer boundary |

A bridge or another source edge cannot become the original fold by occupying a boundary position.
A candidate used only in a hole or construction line cannot be the outer fold. Original traversal
and candidate source direction remain explicit;tag is persistent output identity,not a contour index.
A derived candidate may declare a source subrange;actual whole fold-domain coverage and physical
fold correspondence still require the requested evidence. Provenance alone cannot prove them.

Admission first checks fold-state agreement,then the required original fold field. RetainOriginal
then checks actual outer retained membership. Derived checks candidate existence,outer-boundary use
and exact original-fold source provenance,in that order. Typed errors retain the original complete
request and actual whole boundary context. No missing fold or substitute candidate is manufactured.

`source_fold()` borrows the original canonical fold field when present. `fold_output()` exposes the
authentic admitted candidate only for a derived fold. `fold_edge()` returns the declared retained
or new logical fold identity;that new identity is still unissued. No global source retirement or
physical fold truth follows from this association. The original Piece and ledger stay unchanged.

The scan uses the already admitted complete N candidates and M contour occurrences and allocates
nothing. Earlier source/contour bounds retain their scope;this grants no whole-operation CPU/memory
or currentness guarantee. Actual fold geometry,copy inventory/copy effects,current semantic ownership,
material registry,identity history and atomic publication remain separate obligations. Physical-copy
binding,effect derivation and explicit zero/no-new-edge outcomes follow before native family closure.
