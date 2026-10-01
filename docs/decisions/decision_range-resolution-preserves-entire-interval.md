# Range resolution folds the entire interval, never just its endpoints

- **Type:** `decision`
- **Date:** `2026-10-01`
- **Status:** `active`
- **Owner / source:** `G1-SLICE.3c.2a`, defect D55; ontology §1.1 and §4.2.

answers: "how do seam ranges survive edits?" · "can endpoint repairs prove contour integrity?" · "how are deleted interior fragments represented?"

## Context

The point-reference fold is correct for individual parameters. It cannot prove coverage of a held
interval: split an edge into thirds and delete the middle; both original endpoints still resolve.
`piece_contract::endpoint_inventory_cannot_certify_the_interior_of_an_edge_range` is the tracked
counterexample. A fixed grid is not a remedy: an arbitrarily narrow deleted interval can escape it.

## Decision

A stored `EdgeRange` names an edge and exact ascending bounds `from < to` in `[0,1]`. Traversal
relative to the stored edge is explicit outside that range. Its resolution folds the **whole interval**
through the ledger's typed journal. Split partitions it by exact interval intersection; merge rescales
both ends by declared arc lengths; reverse reflects its bounds and reverses fragment direction;
offset intersects declared source intervals and retains uncovered gaps as visible repairs; delete
replaces the affected portion with a repair. Arithmetic refuses explicitly rather than rounding.

The output is ordered in the original range's forward traversal. Each portion is a surviving directed
range or a `RangeRepairTask`: held original range, affected range at the orphaning edit, and the typed
cause naming the edit operation. A repair is terminal; later unrelated edits never silently reattach
it. An undeclared source is a typed missing-source task, not an identity pass-through. Query results
are derived; the stored range remains unchanged.

**Coverage and points are separate questions.** Positive-length portions partition the interval's
interior. Their shared boundary is carried by both adjoining fragments, without choosing one point
anchor. Original endpoint queries use the existing point fold and retain split-point choices and offset
ambiguity; point consumers must still state their choice. `has_full_coverage` means no portion was lost
or failed arithmetic, not that endpoints have unique resolutions or that the resulting contour closes.
A zero-length interval is a point-reference concern and is rejected by the range constructor.

## Consequences

- `Piece` full-edge queries and sewing spans consume this fold rather than infer completeness from
  their endpoint inventories. The command bus (`.6`) must check range repairs alongside point repairs.
- G2 still proves geometry: orientation, endpoint coincidence, simplicity, containment, continuity
  across fragment joins and intake conservation. Interval coverage is not a release approval.
- Tests compare range fragments with the independent existing point fold away from partition
  boundaries, and use explicit narrow-gap and interior-deletion counterexamples against sampling.
- Range registration/persistence enters with the command bus/store. This slice provides the pure
  query and its typed evidence; the existing point ledger's registration verdict keeps its scope.
- Related: `decision_reference-resolution-journal-fold.md`,
  `decision_ontology-invariants-structural-g1-geometric-g2.md`.
