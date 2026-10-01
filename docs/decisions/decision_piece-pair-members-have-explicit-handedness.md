# A cut-once L/R member carries handedness and a companion Piece identity

- **Type:** `decision`
- **Date:** `2026-10-01`
- **Status:** `active`
- **Owner / source:** `G1-SLICE.3c.1a`, defect D56; reference skirt §6 and the mirrored-pair glossary.

answers: "how does a cut-once pair member print L/R?" · "can a mirrored pair have separate Piece identities?"

## Context

The reference skirt names separate right and left back pieces, each cut once, with reciprocal pairing.
The first `Piece` implementation represented only one definition requesting an even total of mirrored
copies; it omitted the fixture's separate-member form. Inferring L/R from a name would silently turn
print prose into identity and cannot enforce a complete label.

## Decision

Retain `Mirroring::MirroredPairs` for a single definition whose total cut quantity includes both hands.
Add `Mirroring::PairMember { handedness, companion }` for separately identified members. `Handedness`
is explicitly `Left` or `Right`; `companion` is a distinct Piece identity. Quantity counts this
member's physical copies. The complete print view carries this metadata directly from the cut plan.
A member cannot name itself as its companion.

## Consequences

- A cut-once right/left fixture pair is expressible without changing its geometry, cut quantities or
  Piece identities. Existing even-total pair requests retain their original meaning.
- Companion existence, reciprocal identity, opposite handedness and equal member quantities are
  design-collection invariants. `G1-SLICE.6` validates them; a standalone constructor can prove only
  that the companion differs from itself. G2 still verifies actual geometric mirroring.
- This corrects pair metadata; it does not settle the separate D57 question of how the sewing graph
  addresses multiple physical copies. That decision remains with the director, asked explicitly.
