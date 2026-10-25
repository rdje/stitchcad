# Sealed archive — completed canonical scalar state and lint-repair scope

Immutable historical segment, sealed by leaf `G1-SLICE.5e.1.3` on `2026-10-25` (UTC).

- **Sealed identity:** 19 lines, 1730 bytes, sha256:2ebed868df7ce608655863fa012ec178ee8d247e5506e5f8e57d1ec021f57d38
- **Coverage:** whole committed source interval from 0ce3e010e96fa8614f125a9a15b74bc8263a2e19.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 31754–33484, 1730 bytes, sha256:2ebed868df7ce608655863fa012ec178ee8d247e5506e5f8e57d1ec021f57d38
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5e.1.2`
  Status: `done`
  Goal:0262 canonical scalar state/value content and focused lint repairs.
  Paths: crates/sc-core/src/value.rs;crates/sc-core/src/value/scalar.rs;crates/sc-core/tests/scalar_value_contract.rs;crates/sc-core/tests/formula_length_input_contract.rs
  Additional paths: crates/sc-core/src/recipe/input.rs;crates/sc-core/src/recipe/declaration.rs;crates/sc-core/src/recipe/numeric.rs;crates/sc-core/src/recipe/roots.rs
- [x] REPRODUCE: db8aa3c lacks scalar states; selected cargo clippy --test scalar_value_contract
  --test formula_length_input_contract -- -D warnings returns101 for3 library lint failures.
- [x] ROOT CAUSE: Length-only state model; general metadata enum enlarges read error. Clippy output
  identifies source/idioms, rc=101; committed test also has explicit panic against deny rule.
- [x] FIX: typed raw scalar values, state/kind/evidence checks, Derived-only Area; compact sealed
  length view, standard integer methods and asserted test variant; no boxing/lint suppression.
- [x] ADDRESSED:0262 python3 -I -B scripts/local_environment.py -- cargo test --offline -p sc-core
  --test scalar_value_contract --test value_contract --test formula_length_input_contract
  --test formula_numeric_contract --test formula_root_value_contract:5/8/3/6/4 pass, rc=0.
- [x] NO REGRESSION: same native26/0; selected cargo clippy --offline -p sc-core
  --test scalar_value_contract --test formula_length_input_contract -- -D warnings, rc=0.
  Broad .5e.2/.5g/pre-push; general registry adapter remains .5e.1.
- [x] LOCKSTEP:339/340/234 whole Git/public records and book render pass0; live/book align.
  Promotion declined: implementation of existing canonical state/kind/lint contracts.
