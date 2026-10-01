# Executable closure intent

This companion records G1's structural closure APIs under ontology §4.7. Physical hardware geometry,
placement, fold/seam effects and derived buttonhole lengths remain G2/G3 obligations. Logical
parameter/operation/material registries and target profiles must validate their own declarations.
Zipper/hook-and-bar descriptors and button/buttonhole derivation follow their placement foundation
in G1-SLICE.3c.4c; a placement API alone is not a completed closure implementation.

## Physical notion placements

`NotionPlacement` gives each notion placement a stable semantic identity and names the **physical
cut copy** it attaches to. `NotionPlacementDefinition` carries id, copy id, an `EdgeAnchor` position
and a `DirectedRange` orientation reference in the pattern Piece's source frame. The immutable
placement retains its original source Piece identity to detect copy reassignment. It duplicates no
pattern geometry, reflected coordinates, material state or component size.

For example, two copies of one pattern Piece can carry different hook/bar or zipper-side placement
positions. Each placement keeps its own identity, anchor and orientation while naming its intended
copy. Reordering the CutPlan preserves bindings. Removing that copy refuses `MissingCopy`, even if
another copy of the same Piece remains. Reassigning the original copy to another source Piece is
also a typed refusal; neither operation transfers the notion to an arbitrary peer.

The constructor requires the original copy in the supplied plan, matching source Piece, a live
uniquely owned anchor, and complete owned orientation intervals with unique endpoints. Missing edges,
foreign material and split endpoint choices are refused. Current `validate_current()` checks the
copy/owner again and follows the identity journal for the anchor and directed range. A historical
anchor need not retain its original live edge: splitting and reversing it is valid when its current
resolved point remains uniquely owned. A split-point choice or deletion is `CurrentUnresolved` with
raw point evidence. Birth validation retains the existing live-edge requirement and its original
`BornUnresolved` errors; Notch and TurnPoint behavior remains unchanged.

`anchor_resolution()` and `direction_resolution()` expose independent raw point/range evidence
without choosing or rewriting input. Lost orientation interiors remain visible even when both
original endpoints survive. Complete live endpoints around a foreign merged middle do not establish
ownership. Directed fragment order composes authored traversal with journal reversal; reflected
CutCopy orientation is a separate G2/V1 transform, never an inferred reversal of the source frame.

`validate_current()` checks one supplied current plan/Piece context. Design validation must check all
current registries and every placement before closure execution; raw queries grant no release
readiness. Actual hardware shape, attachment coordinates and reflected geometry are unproved:
every placement reports `GeometricValidation::DeferredToG2`. Counts and sizes belong to the closure
kinds, not an implicit expansion or a default supplied by a placement.

| API token | Meaning in the physical-placement implementation |
| --- | --- |
| `NotionPlacement` | Immutable placement on one stable physical cut copy |
| `NotionPlacementDefinition` | Authored placement identity, copy, anchor and orientation range |
| `NotionPlacementError` | Typed copy/owner/anchor/direction refusal |
| `CurrentUnresolved` | Previously valid held anchor now needs repair or an explicit choice |
| `UnresolvedDirection` | Positive orientation content or an endpoint needs repair/choice |
| `DirectionOutsidePiece` | Current orientation material belongs outside the original owner |
| `EdgeAnchor` | Held source-edge identity and exact parameter for a notion attachment |
| `BornUnresolved` | Birth point requires unique resolution; its existing error contract is retained |
| `MissingCopy` | Original physical-copy target is absent; a peer is never substituted |
