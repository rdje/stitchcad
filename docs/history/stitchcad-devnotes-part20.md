# Sealed archive — StitchCAD physical sewing intervals

Immutable historical segment, sealed by leaf `G1-SLICE.3c.4c.2` on `2026-10-01`.

- **Sealed identity:** 15 lines, 1375 bytes, `sha256:34867dc9d36aa4f7d59904e66875a19ac19675c849068140862c5c88caa9d56b`
- **Coverage:** physical sewing-interval lesson, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## _(2026-10-01)_ — sewing joins physical material intervals, not pattern names

- D57's copy identities make two copies of one Piece distinct sewing domains. D35 allows a physical
  copy to join itself only where current positive-length interiors are disjoint. Comparing held edge
  names cannot prove that: a merge can make formerly different names share a current frame. The
  constructor compares resolved fragments and checks every owned portion, including hidden interiors.
- Correspondence, journal traversal and reflection remain separate authored facts. Stops use semantic
  Notches/TurnPoints rather than physical notch representation. Ease is a signed declared source and
  explicit allocation; structural domains are checked, actual walking/arc lengths and resolved values
  are later obligations. A typed failure never silently stretches or reassigns material.
- Eighteen tests cover copies with different neighbours, partial/one-to-many spans, self-seams after
  merge/reversal, interior ownership/repairs, endpoint choices, stop/ease domains and removed targets.
  Disabling self-overlap and interval ownership refusals each produces a red regression. Existing
  notches retain behavior after extracting shared anchor validation; strict checks and WASM pass.
- promotion: promoted by `decision_sewing-spans-address-copies-and-permit-disjoint-self-seams.md`.
