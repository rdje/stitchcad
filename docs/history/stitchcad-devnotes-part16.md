# Sealed archive — StitchCAD dev notes, interval coverage

Immutable historical segment, sealed by leaf `G1-SLICE.3c.4b.1` on `2026-10-01`.

- **Sealed identity:** 13 lines, 1183 bytes, `sha256:fcf7c4759b090289851f4a71a769e31a3179a82f3ccb222a3699fab8c86cb2f4`
- **Coverage:** the oldest interval-coverage lesson, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## _(2026-10-01)_ — interval coverage and endpoint identity answer different questions

- `G1-SLICE.3c.2a` fixes D55 with a pure exact whole-interval journal fold. Split and offset partition
  ranges by intersection; merge rescales by declared lengths; reverse reflects bounds and direction;
  deleted or trimmed positive-length content becomes a visible repair. No sampling can certify an
  interval: a nanowide gap escapes a hundredths grid and is still detected by the interval fold.
- Coverage, point ambiguity and geometry remain separate. The result retains endpoint point queries
  alongside ordered interval portions. A split-boundary choice is not silently picked just because
  a positive-length interval maps uniquely. G2 still owns continuity and geometric closure.
- Thirteen range tests include a differential comparison against the already-tested point fold,
  exact integer-length merge expectations, terminal repairs and a deliberately disabled delete arm
  observed red. Piece range queries now catch the original counterexample. Existing suites stay green.
- promotion: promoted by `decision_range-resolution-preserves-entire-interval.md`, recorded before code.
