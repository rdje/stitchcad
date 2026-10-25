# Sealed archive — completed signed arc value scope

Immutable historical segment, sealed by leaf `G1-SLICE.5d.1.2.2.h` on `2026-10-25` (UTC).

- **Sealed identity:** 13 lines, 1182 bytes, sha256:61850e2f760d71b301fc054931c9cdc88603d199d518340ac19525c3c0ae6fb1
- **Coverage:** whole committed source interval from f168eec4e43cc794dc3e989706215a61f44cdec4.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 31003–32185, 1182 bytes, sha256:61850e2f760d71b301fc054931c9cdc88603d199d518340ac19525c3c0ae6fb1
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5d.3.1`
  Status: `done`
  Goal:0258 exact signed/full-turn arc_length using the certified pi endpoint.
  Paths: crates/sc-core/src/recipe/arc.rs;crates/sc-core/src/recipe/rational.rs;crates/sc-core/src/recipe/numeric.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_arc_value_contract.rs
  Additional paths: docs/tasks/artifacts/formula_structure/arc_value_oracle.py;crates/sc-core/tests/fixtures/formula_arcs.tsv
- [x] REPRODUCE: missing arc API at67f67d0, git source inspection0.
- [x] ROOT CAUSE: python3 -I -B docs/tasks/artifacts/formula_structure/pi_rounding_certificate.py
  reproduces640-bit midpoint-exclusion certificate, rc=0 (compiled certificate test).
- [x] FIX: exact signed product/rounding, actual kind/width/scalar errors; no dependency/modulo.
- [x] ADDRESSED: local_environment.py cargo test --offline -p sc-core --lib
  --test formula_arc_value_contract --test formula_numeric_contract:67/2/6 pass, rc=0;23 oracle rows.
- [x] NO REGRESSION: same command numeric6/0 and library67/0, rc=0; broad .5d.4/pre-push, provenance .5e.3.
- [x] LOCKSTEP:331/230 exact Git/public records, book/live align; decision implementation promoted.
