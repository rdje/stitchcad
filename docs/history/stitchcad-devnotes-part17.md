# Sealed archive — StitchCAD separate pair members

Immutable historical segment, sealed by leaf `G1-SLICE.3c.4b.2` on `2026-10-01`.

- **Sealed identity:** 12 lines, 1049 bytes, `sha256:140c4c41dd058717960cd08ba89536c9f52fda2a005d0f1449e5f6cef8f1e830`
- **Coverage:** separate-pair-member lesson, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## _(2026-10-01)_ — separate L/R members differ from one even-total pair request

- The canonical fixture's back pair is two Piece identities, each cut once. `.3c.1` enforced only
  an even-total pair mode, so it could not encode the fixture's L/R member labels. `PairMember`
  adds explicit handedness and a distinct companion identity; the old pair mode remains supported.
- A fixture-shaped test guards both cut-one quantities and reciprocal label metadata. Applying an
  even-total restriction to every pair mode turns that test red; self-companion is also a typed refusal.
  The constructor cannot prove companion existence or mirroring from one definition: `.6` owns the
  collection checks and G2 the geometry. These obligations are explicit rather than presumed.
- The related physical-copy addressing question (D57) is a separate, unspecified graph contract.
  The director's decision is pending; marking it explicit allows independent marks to proceed.
- promotion: promoted by `decision_piece-pair-members-have-explicit-handedness.md`.
