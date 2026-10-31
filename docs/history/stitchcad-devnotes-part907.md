# Sealed engineering continuity — part907

- **Identity:** 18 lines, 1386 bytes, sha256:eb721735de8bdba408ba46b288861ba367f6ff4d4d351b49d98561c67126d319
- **Coverage:** whole completed0015 Tuck/Pleat task protocol;historical proof retained without rerun from 3a942a80274725bc25276af4d4db7681fd380ca3.
- Whole source interval: docs/tasks/G1-SLICE-constructions.md bytes 1670–3056, 1386 bytes, sha256:eb721735de8bdba408ba46b288861ba367f6ff4d4d351b49d98561c67126d319
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.3c.4a.2a`
  Status: `done`
  Goal: immutable, distinctly typed Tuck and Pleat with nonempty directed fold-line geometry,
  intake origin, explicit folding direction and closing-operation identity.
  Acceptance: all lines/direction are owned complete ranges with unique endpoints; empty or duplicate
  held fold intervals and negative explicit intake refused; symbolic intake supplies no default;
  queries preserve direction/repairs; physical fold shape/conserved intake remains G2/G3.
  Design before code: shared FoldDefinition input/validation, separate immutable Tuck/Pleat wrappers
  preserve semantic kind. A nonempty list supports one or multiple authored fold lines without
  inventing a physical pleat-count rule. Exact duplicate held intervals ignore traversal direction.
  Reuse IntakeAmount and whole-interval ownership; folding operation existence/kind is `.5`/`.6`.
  Broader actual geometry tests remain G2/G3, including coincidence between distinct held names.
  Extend the existing structural/geometric record before code. Live-window rollover is part of
  this slice's doc sync if the next entry crosses CHANGELOG/DEV_NOTES health targets.
  Verification: nine shared tuck/pleat contracts + two privacy doctests; ownership mutation red;
  strict Rust, wasm, book, focused censuses, ledger and staged doctrine gate green.
  Commit: `STITCHCAD-G1-0015`

