# Sealed archive — completed general scalar authored-read scope

Immutable historical segment, sealed by leaf `G1-SLICE.5e.1.4` on `2026-10-26` (UTC).

- **Sealed identity:** 15 lines, 1400 bytes, sha256:c8166afe417ebd490b24975d11d162ffde8a1dd319bf057d76e8da881cfb731f
- **Coverage:** whole committed source interval from 72cc909312a25705fc67d84ca454fed6aa4bebe3.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 32098–33498, 1400 bytes, sha256:c8166afe417ebd490b24975d11d162ffde8a1dd319bf057d76e8da881cfb731f
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5e.1.3`
  Status: `done`
  Goal:0263 sourced general scalar reads with actual canonical identity/kind validation.
  Paths: crates/sc-core/src/recipe/input.rs;crates/sc-core/src/recipe/input/scalar.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/src/recipe/declaration.rs;crates/sc-core/tests/formula_scalar_input_contract.rs
- [x] REPRODUCE:0263 0ce3e01 supplies canonical scalar state but no metadata match/read adapter;
  git source inspection rc=0. Initial fixture extend has E0597/101; pushes fix its lifetime coercion.
- [x] ROOT CAUSE:0263 metadata claim cannot prove actual target/kind; the independent kind table/
  wrong-candidate fixtures and focused native5/3 establish match/read separation, rc=0.
- [x] FIX: source/id/kind checks in order; compact borrowed source/state, typed value/action results.
- [x] ADDRESSED:0263 python3 -I -B scripts/local_environment.py -- cargo test --offline -p sc-core
  --test formula_scalar_input_contract --test formula_length_input_contract:5/3 pass,78 reads, rc=0.
- [x] NO REGRESSION:0263 same native8/0; cargo clippy --offline -p sc-core
  --test formula_scalar_input_contract -- -D warnings, rc=0. Broad execution/platform .5e.2/.5g/pre-push.
- [x] LOCKSTEP:341/342/235 whole Git/public records and book render rc=0; live/book align.
  Promotion declined: existing metadata/canonical state contract; no registry/approval inferred.
