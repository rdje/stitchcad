# Sealed archive — whole completed sewing structural contract

Immutable historical segment, sealed by leaf `G1-SLICE.4c.3d.0` on `2026-10-26` (UTC).

- **Sealed identity:** 22 lines, 1730 bytes, sha256:c022b1d9ee136eed33c351952dc4a00a7fcdbaf9920982ce1db4bb5cbab33ad6
- **Coverage:** whole committed source interval from e77742e88a485ab04137d7b8234482bb30fca29d.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 14559–16289, 1730 bytes, sha256:c022b1d9ee136eed33c351952dc4a00a7fcdbaf9920982ce1db4bb5cbab33ad6
- **Retrieval:** bytes below the first rule and blank line; original terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.3c.2b.2`
  Status: `done`
  Goal: immutable sewing graph whose sides name validated CutCopy identities and partial edge ranges;
  explicit correspondence direction, ease declaration and stop anchors. Settle D35 before code.
  Acceptance: partial and one-to-many spans distinguish physical copies of one Piece; graph references
  resolve through the cut plan and owned topology; missing/duplicate references refused. Same-copy
  seams have an explicit tested contract, positive-range interiors and endpoint choices stay visible.
  D57 closes after sewing integration; D35 closes after its self-seam regression and documentation.
  Verification: 18 sewing contracts + existing range/copy/notch suites, strict check, wasm,
  book, fixture/feature/glossary censuses, ledger probes and staged doctrines green. D35/D57 closed.
  Commit: `STITCHCAD-G1-0011`

  Design: `decision_sewing-spans-address-copies-and-permit-disjoint-self-seams.md`, before code.
  Same-copy contract (D35): legal for ranges whose current positive-length interiors are disjoint;
  shared endpoints are legal, an overlapping/identical material interval sewn to itself is refused.
  Different copies of the same Piece may correspond over the same source interval. Same-copy
  disjointness is checked in resolved topology, not only by authored EdgeRef inequality.
  Stop landmarks: semantic Notch or born-valid edge-anchored TurnPoint, named by stable id and
  attached to an explicit span side/copy. Stops must belong to that side's current interval.
  Ease: signed A-minus-B declaration (explicit length or symbolic parameter); uniform, weighted
  parameter regions or between explicit notch landmarks. G2/G3 owns geometric differential checks.

