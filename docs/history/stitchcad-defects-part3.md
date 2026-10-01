# Sealed archive — StitchCAD defect census, closed D56

Immutable historical segment, sealed by leaf `G1-SLICE.3c.1a` on `2026-10-01`.

- **Sealed identity:** 15 lines, 1425 bytes, `sha256:84eb40141b0e8711f03e40ba3387708f8b8c8b87fb43700eb281d9a86353972c`
- **Coverage:** `D56`; discovery record preserved above its closure evidence.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D56** — `.3c.1`'s `Mirroring::MirroredPairs` requires an even total on one Piece and cannot
  represent the canonical fixture's two distinct cut-once left/right members. Found `2026-10-01`.
  - Reproduce: reference skirt §6 declares `skirt_back_right` and `skirt_back_left`, quantity one
    each and reciprocal pairing; `piece_contract::cut_quantity_and_mirroring_cannot_make_an_empty_or_incomplete_pair`
    exercises the constructor's odd-quantity refusal. `PieceDefinition` has no separate handedness
    or companion identity, so the existing enum cannot print the fixture's L/R pairing faithfully.
  - Impact: the first G2 fixture would need to discard pairing or misstate its cut quantities.
  - Owner/schedule: **`G1-SLICE.3c.1a`, immediately** — explicit L/R member plus companion identity,
    with a fixture-shaped regression test. `.6` validates companion existence and reciprocal metadata
    in the design's collection; G2 verifies geometric mirroring.
  - **Closed:** `G1-SLICE.3c.1a` adds `PairMember { handedness, companion }`; the canonical
    fixture-shaped cut-once L/R test passes. `cargo test -p sc-core --test piece_contract` →
    `14 passed`, `rc=0`. Reinstating an even-total restriction on every pair mode makes that
    regression fail (`rc=101`); restored source passes. Companion-collection validation is owned
    explicitly by `.6`, and geometric mirroring remains G2's predicate.
