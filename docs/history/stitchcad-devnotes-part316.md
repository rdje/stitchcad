# Sealed archive — completed exact products and quotients scope and proof

Immutable historical segment, sealed by leaf `G1-SLICE.5c.1.2.2` on `2026-10-25` (UTC).

- **Sealed identity:** 20 lines, 1668 bytes, sha256:45485330eed645e3195988bc2de82d828e2e45dd6225e59635a49b2185240f6c
- **Coverage:** whole committed source interval from 6d96bc722934641cac010f7b5dce836dee214d34; live ownership retained.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 22399–24067, 1668 bytes, sha256:45485330eed645e3195988bc2de82d828e2e45dd6225e59635a49b2185240f6c
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5c.1.2.1`
  Status: `done`
  Goal: STITCHCAD-G1-0245 delivers exact products/quotients and measured reduced-width refusals.
  Paths: crates/sc-core/src/recipe/rational.rs;crates/sc-core/tests/formula_rational_contract.rs
  Acceptance: cross-cancel before multiplication; preserve full magnitudes/sign/canonical zero,
  refuse zero divisors before zero shortcut. Fully reduced components above128 bits report exact
  numerator/denominator width, fixed bound/token and formula_domain; no partial value or rounding.
  Bounded partial products establish overflowing product width without dependencies or allocation.
  One affected public contract target; broader/oracle/platform work stays .5c.1.3. Update book/live,
  retain whole0244 task/ledger314/217 and completed proof nodes315 under fixed caps; commit before sums.

- [x] REPRODUCE / ISSUE: predecessor67e0a41 rational.rs supplies construction only; source inspection0.
- [x] ROOT CAUSE: cross-cancellation is required before width judgment; affected native target proves
  admitted cancellation and exact129/256-bit refusals:2 passed, rc=0.
- [x] FIX: checked products/quotients, zero-divisor priority and bounded exact width witness; focused
  formula_rational_contract passes2/0, rc=0.
- [x] ADDRESSED: primary profile cargo test -p sc-core --test formula_rational_contract:2 passed/0 failed, rc=0.
- [x] NO REGRESSION: same focused target retains constructor cases, rc=0; broad/platform/oracle work stays .3.
- [x] LOCKSTEP: three retained source/public intervals exact, rc=0; book/live/frontier sync.
  promotion: declined (existing numeric/focused-check policy). Hooks required before commit.
