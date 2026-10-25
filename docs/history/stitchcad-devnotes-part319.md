# Sealed archive — completed focused rational oracle scope and proof

Immutable historical segment, sealed by leaf `G1-SLICE.5c.2.1` on `2026-10-25` (UTC).

- **Sealed identity:** 22 lines, 1788 bytes, sha256:dbc9233085b988be1fe943c4723b83391de4cd2997a660833dd657007b3f2f72
- **Coverage:** whole committed source interval from 6b1de3d164e1f4c37cde3fdcfbe1c4ce132b8010.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 23145–24933, 1788 bytes, sha256:dbc9233085b988be1fe943c4723b83391de4cd2997a660833dd657007b3f2f72
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5c.1.3`
  Status: `done`
  Goal: independent rational oracle integration and focused primitive signoff; broad campaigns later.
  Acceptance: independent arbitrary-precision results at full-width/cancellation/refusal boundaries,
  stable diagnostics and original syntax/literal separation; no evaluator closure claim.
  Own0247: add pure stdout Fraction fixture oracle and immutable expected TSV, consumed by the
  existing public native target. Compare four-operator outputs/refusals without production bigint
  arithmetic or new evaluator scaffolding. One affected target; broad/platform work stays .5c.4.
  Paths: docs/tasks/artifacts/formula_structure/rational_value_oracle.py;crates/sc-core/tests/fixtures/formula_rationals.tsv;crates/sc-core/tests/formula_rational_contract.rs
  Own completed sum node retention318/ledger219 if needed for fixed caps; book/live/pointer sync.

- [x] REPRODUCE / ISSUE: a814dd9 contains example tests without independent arithmetic rows;
  source inspection confirms, rc=0.
- [x] ROOT CAUSE: independent Fraction arithmetic avoids sharing the production bigint/kernel;
  oracle emits34 exact/sign/zero/cancellation/refusal rows, rc=0.
- [x] FIX: pure oracle/TSV plus public four-operator replay; no production semantics changed.
- [x] ADDRESSED: cargo test --offline -p sc-core --test formula_rational_contract:4 passed/0 failed,
  including34 independent rows, rc=0.
- [x] NO REGRESSION: same target retains constructor and prior operator boundaries,4/0, rc=0;
  syntax/input code unchanged. Broad/platform/mutation campaigns remain .5c.4 and pre-push.
- [x] LOCKSTEP: book/live/frontier/pointer aligned; whole318/219 Git/public intervals exact, rc=0.
  promotion: declined (established independent-oracle and focused-check policies).
