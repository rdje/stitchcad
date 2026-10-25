# Sealed archive — completed signed sums and differences scope and proof

Immutable historical segment, sealed by leaf `G1-SLICE.5c.1.3` on `2026-10-25` (UTC).

- **Sealed identity:** 25 lines, 2064 bytes, sha256:f48003d8968b13b27e905f7efa04b1ffa62c0604688714865d04df1543dbf1da
- **Coverage:** whole committed source interval from a814dd9409cce141037541e874d914c4da0a4ce4.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 22734–24798, 2064 bytes, sha256:f48003d8968b13b27e905f7efa04b1ffa62c0604688714865d04df1543dbf1da
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5c.1.2.2`
  Status: `done`
  Goal: signed sums/differences with bounded wider intermediate reduction before128-bit judgment.
  Acceptance: cancellation of wide intermediate products stays exact, typed width errors reflect
  the fully reduced result; one focused check and later .5c.1.3 oracle integration.
  Paths: crates/sc-core/src/recipe/rational.rs;crates/sc-core/tests/formula_rational_contract.rs;crates/sc-core/Cargo.toml;Cargo.lock
  Own0246: these implementation/contract/dependency files; private
  cached num-bigint0.4.6 intermediates, published MIT OR Apache-2.0/Rust1.60/std metadata verified.
  Admitted operands bound the raw numerator to257 bits/denominator256. Reduce before narrowing;
  preserve canonical sign/zero and exact measured error witnesses. No numeric quantity/evaluator claim.
  Update existing edge-rational decision with workspace separation; native one-target proof and
  offline dependency resolution, platform/full campaigns deferred .5c.1.3. Retain completed task/
  policy/ledger316/317/218 under fixed caps and commit before oracle integration.


- [x] REPRODUCE / ISSUE: 6d96bc7 lacks sum/difference methods; source inspection confirms, rc=0.
- [x] ROOT CAUSE: raw257-bit sums can reduce to admitted128-bit values; cargo test --offline -p
  sc-core --test formula_rational_contract proves wide cancellation,3 passed/0 failed, rc=0.
- [x] FIX: private bounded bigints reduce before narrowing; exact measured bits/sign/canonical zero.
- [x] ADDRESSED: same focused native target covers signed differences and129/257/256-bit refusals,
  output3 passed/0 failed, rc=0. Offline metadata resolves four MIT/Apache licensed packages, rc=0.
- [x] NO REGRESSION: same target retains constructor/product/quotient checks,3/0, rc=0;
  broader/platform verification and independent oracle remain .5c.1.3.
- [x] LOCKSTEP: retained316/317/218 preserve whole Git intervals/public reads, rc=0; book render0/live/frontier synced.
  promotion: existing edge/formula rational decision records bounded workspace/dependency choice.
