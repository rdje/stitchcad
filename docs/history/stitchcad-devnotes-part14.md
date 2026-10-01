# Sealed archive — StitchCAD dev notes, persistent identity

Immutable historical segment, sealed by leaf `G1-SLICE.3c.4a.2a` on `2026-10-01`.

- **Sealed identity:** 24 lines, 2230 bytes, `sha256:2b6aebd33130c47504ea8a35a89c24cf522e4699e7974577d08a4c47ead5d2e0`
- **Coverage:** the oldest persistent-identity lesson, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## _(2026-10-01)_ — the persistent-identity contract: a reference is never rewritten, the journal folds

- `G1-SLICE.3b` landed the persistent-identity contract — `sc_core::ontology::topology`'s `IdentityLedger`:
  an append-only journal of typed edits and a pure fold resolving a held `(EdgeRef, Param)` through split,
  merge, reverse, delete and offset-fragmentation. 62 unit tests + 8 recorded-seed properties green, and
  `sc-core` still cross-builds to wasm.
- The design point worth keeping: **an edit never rewrites a stored reference.** The obvious implementation
  — mutating every consumer of the edited edge in place — is the silent reassignment §1.1 forbids, and a
  missed consumer is silent corruption. Instead the reference is immutable and resolution is a fold of the
  journal, so no consumer can be missed: the answer is computed *from the reference the consumer holds*.
  Repair state (`open_repairs`, `release_readiness`) is **derived** on every query, never stored, so it cannot
  drift from the journal — the discipline every derived-vs-hand-kept count here enforces. Undo (`.6`) then
  becomes journal algebra, not consumer archaeology.
- The exact-arithmetic choice paid off in the property suite: a split↔merge round trip returns the IDENTICAL
  reduced `Rational` (zero drift), and merge's arc-length recomputation is checked against its defining
  proportion cross-multiplied in `i128` — an oracle sharing no code with the fold. A fixed-point parameter
  would drift under the same round trip.
- One contract subtlety the tests pin: at a split point the reference resolves to BOTH fragments and the
  ledger never picks — the consumer states a `SplitSide`. If one side is later deleted, the fold still offers
  the survivor and shows the task on the dead side; whether it is an open repair depends on the stated side.
  "No silent reassignment" holds on every path.
- promotion: promoted by `decision_reference-resolution-journal-fold.md` (which carries `answers:`) — the
  durable boundary (immutable references, fold resolution, derived repair state, offset-consumes-declared-
  intervals until G2 geometry) is recorded there so `.3c`/`.6`/`.7` inherit it rather than re-litigate.
