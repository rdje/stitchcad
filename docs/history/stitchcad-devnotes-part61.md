# Sealed archive — unsigned magnitude rounding lesson

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3c.4` on `2026-10-02`.

- **Sealed identity:** 15 lines, 1322 bytes, `sha256:90999a24ccc28b56d425dea1a921336683ddaba0651cc45c86881741a3cd844b`
- **Coverage:** unsigned magnitude rounding lesson; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## _(2026-10-02)_ — unsigned canonical magnitudes need a full-width rounding result

- D95 permits128-bit positive literal children before signed binding. Existing signed rounding
  correctly returns i64; add a separate unsigned result sharing the same private magnitude rule.
  Compare r >= d-r so full-u128 remainders cannot overflow; preserve caller operation and signed
  MIN/overflow/sign reconstruction. No new dependency, scalar/domain rule or formula execution.
- Five public contracts consume138 Decimal120-digit oracle rows and check wide/tie/zero/signed
  boundaries. Nine actual compiled debug faults and one release wrapped-remainder fault fail
  assertions, restore exact bytes; all five existing signed faults remain discriminating.
- Strict native494 (units46), release signed/unsigned public tests and three real WASM builds pass.
  Book adds a progressive unit API/example and indexed expert proof annex; language/publication pass.
  Literal normalization/arena/serializer/binding/evaluation remain separate product leaves.
- Prior protocol/checklist/oldest ledger bytes retain exact histories. Correct another existing D34
  stale execution-order pointer; PLANNING.5 still owns derived synchronization.
- promotion: declined (routine full-width rounding prerequisite under the received D95 contract).
