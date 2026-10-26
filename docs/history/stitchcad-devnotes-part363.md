# Sealed archive — completed lazy conditional value scope

Immutable historical segment, sealed by leaf `G1-SLICE.5e.2.2` on `2026-10-26` (UTC).

- **Sealed identity:** 20 lines, 1847 bytes, sha256:d9a1cc91c1306d50e2ad8810ad5a3aed37c0f62e72cf11cfee5ed5d3539fba91
- **Coverage:** whole committed source interval from 570d5b52c4576ff935b7ea0fde6eff748bc1d909.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 34377–36224, 1847 bytes, sha256:d9a1cc91c1306d50e2ad8810ad5a3aed37c0f62e72cf11cfee5ed5d3539fba91
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5e.2.1`
  Status: `done`
  Goal:0273 lazy conditional value control with both branch kinds checked before callbacks.
  Paths: crates/sc-core/src/recipe/evaluated.rs;crates/sc-core/src/recipe/evaluated/conditional.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_conditional_value_contract.rs
  Boolean condition, equal numeric branch metadata, selected callback once, untouched untaken
  callback, actual body failure and output-kind validation; merge condition and taken sources only.
  Full checked-AST names/structure/branch proofs must precede this supplied-value primitive.
  Retain361 whole0272 task,362 oldest whole numeric-domain/binary lesson group,245 ledger.
  Native/selected lint and book/live sync; full expression/statement/atomic execution stays open.
- [x] REPRODUCE:0273 f67ed36 has sourced values but no lazy value conditional; source read rc=0.
- [x] ROOT CAUSE:0273 evaluating both callbacks would read untaken Unknowns and contaminate
  sources; supplied kinds must be checked first, then only one callback. Native4/0, rc=0.
- [x] FIX: actual Boolean/equal-numeric metadata guard, selected callback/result kind and source union.
- [x] ADDRESSED:0273 python3 -I -B scripts/local_environment.py -- cargo test --offline -p sc-core
  --test formula_conditional_value_contract:4 pass/0 fail, rc=0.
- [x] NO REGRESSION:0273 full kind matrix/counts/actual Unknown/error/output/source controls pass;
  cargo clippy --offline -p sc-core --test formula_conditional_value_contract -- -D warnings, rc=0.
  Full AST static names/structure and expression/statement/atomic .5e.2/.4 remain owned prerequisites.
- [x] LOCKSTEP:361/362/245 whole Git/hash/public records and book render rc=0; live/frontier align.
  Promotion declined: existing grammar7/D121; no supplied metadata promoted to a whole AST proof.
