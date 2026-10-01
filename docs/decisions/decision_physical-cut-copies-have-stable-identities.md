# Physical cut copies have stable identities

- **Type:** `decision`
- **Date:** `2026-10-01`
- **Status:** `active`
- **Owner / source:** `G1-SLICE.3c.2b`, director ruling answering D57, ontology §4.1–§4.2.

answers: "how does a seam distinguish copies of a cut-two piece?" · "are cut-copy ids derived from order?" · "how do mirrored copies appear in a cut plan?" · "how do closure notions distinguish physical copies?"

## Context

A pattern Piece can request several physical copies. Its copies may have different seam neighbours;
a pattern-only sewing graph cannot distinguish them. The director chose a stable identity for every
physical copy, rather than implicit expansion by the assembly renderer.

## Decision

An immutable `CutPlan` holds explicit `CutCopyDefinition`s, each with its own stable entity id,
a pattern Piece id and authored/reflected orientation. The caller supplies ids through the project's
injected identity workflow; validation never mints ids, and an index or ordinal is not an identity.
The copy owns no duplicated pattern geometry: its edge references use the Piece's source frame.
Reflected orientation is separate from journal reversal and will be consumed by G2/V1 transforms.

Validate the complete plan against the supplied Piece collection. Piece ids and copy ids are unique
and disjoint; every copy names an existing Piece; counts exactly equal each Piece's cut quantity.
`MirroredPairs` requires half authored and half reflected copies. `Single` and separate `PairMember`
definitions require authored copies, because their own geometry already carries their handedness.
Companion existence/reciprocity remains the design-collection obligation owned by `.6`.

The sewing graph will name copy ids plus edge ranges. Existing identities survive plan reordering;
quantity edits require an explicit replacement that retains surviving identities. A removed copy
cannot silently transfer its spans to another copy: graph validation must expose the missing target.
The command bus owns atomic replacements and global design-level identity/reference checks.

## Consequences

- Two copies of one Piece can have different neighbours without duplicating drafting recipes.
- A complete CutPlan is structurally checked, not a cutting/geometry or release certificate.
- Empty plans are legal only for empty Piece collections. No missing copy is silently generated.
- G2/V1 must consume copy orientation; the G1 plan neither reflects coordinates nor expands seams.
- D57 closes after copy addressing is exercised by the sewing graph, not merely after this record.

- Implementation prose must declare API names in the chapter's local vocabulary table, and garment
  concepts in the glossary. Run the glossary census on every ontology implementation slice: D59
  reproduced 15 undeclared names at the committed baseline, despite other focused gates being green.

## Physical notion placement (`G1-SLICE.3c.4c.1a`)

Closure components attach to physical material, so their placements name stable CutCopy identities,
not implicit ordinal copies. Each immutable placement retains its own id, held anchor and directed
orientation range; its private born Piece id detects later copy reassignment. Birth validates copy,
source owner, live uniquely owned anchor and complete owned direction. Current validation accepts
uniquely resolved historical anchors while refusing split choices, missing copies, changed source
owners and lost/foreign material. No geometry is copied or reflected at G1; G2/V1 consumes copy
orientation separately from journal direction. Raw queries do not grant release readiness.
