# Sealed archive — completed correctly rounded hypot scope and proof

Immutable historical segment, sealed by leaf `G1-SLICE.5d.1.2.1` on `2026-10-25` (UTC).

- **Sealed identity:** 20 lines, 1562 bytes, sha256:ba361feba854120cc3b4268187b223e4889eaeb9c827b18f0e1287b09bbb2287
- **Coverage:** whole committed source interval from a67f4c30a34aca6ee3a32c1103e79085a1820c8d.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 29072–30634, 1562 bytes, sha256:ba361feba854120cc3b4268187b223e4889eaeb9c827b18f0e1287b09bbb2287
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5d.2.2`
  Status: `done`
  Goal:0256 exact rounded hypot without materializing an area; source provenance .5e.3.
  Paths: crates/sc-core/src/recipe/roots.rs;crates/sc-core/src/recipe/numeric.rs;crates/sc-core/tests/formula_root_value_contract.rs
  Own exact signed-length squared sum and reuse reviewed rounded-root helper. Validate actual
  kinds through existing signatures, then only completed rounded Length domains; no internal area
  or rational-width refusal. Replay ten retained hypot rows and preserve15 sqrt cases in one
  affected native target; broad/platform/mutation .5d.4/pre-push. Keep parent open for .5e.3
  source contribution proof. Retain whole sqrt task/ledger329/228; book/live/pointers before commit.


- [x] REPRODUCE / ISSUE:e646da3 has no production hypot; source inspection rc=0.
- [x] ROOT CAUSE: intermediate Area/width limits can falsely refuse an admitted rounded Length;
  cargo test --offline -p sc-core --test formula_root_value_contract:4 passed/0 failed, rc=0.
- [x] FIX: exact squared fractions/root reuse, metadata kinds and rounded-result scalar witnesses.
- [x] ADDRESSED: same native target consumes ten independent hypot rows/bad-kind cases,4/0, rc=0.
- [x] NO REGRESSION: same target retains all15 sqrt rows,25 total/4 tests, rc=0; source provenance
  .5e.3 and broad/platform/mutation .5d.4/pre-push remain open.
- [x] LOCKSTEP:329/228 whole Git/public records exact, rc=0; book/live/frontier aligned.
  promotion: numerical decision records hypot value-kernel scope; parent keeps provenance pending.
