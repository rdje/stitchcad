# Sealed archive — completed canonical length authored-read scope

Immutable historical segment, sealed by leaf `G1-SLICE.5e.1.2` on `2026-10-25` (UTC).

- **Sealed identity:** 16 lines, 1418 bytes, sha256:875187a4a70cabe5dd90c08c4a0017781ae9a1c8b401a02344d9a1c5a4971568
- **Coverage:** whole committed source interval from db8aa3c4d59224b11744543d6aee55ea06603757.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 32054–33472, 1418 bytes, sha256:875187a4a70cabe5dd90c08c4a0017781ae9a1c8b401a02344d9a1c5a4971568
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5e.1.1`
  Status: `done`
  Goal:0261 exact canonical length authored reads with source/state retained.
  Paths: crates/sc-core/src/recipe/input.rs;crates/sc-core/src/recipe/numeric.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_length_input_contract.rs
- [x] REPRODUCE:9cd7936 has canonical authored lengths and metadata but no sourced numeric read;
  git source inspection rc=0. Unknown/derived must not gain independent numeric values.
- [x] ROOT CAUSE: actual LengthInput borrow supplies authoritative state/value; existing
  LengthDeclaration::authored_value and typed Length domain verified by focused native3/0, rc=0.
- [x] FIX: immutable adapter/results/refusals retain actual metadata/canonical borrow; total Length import.
- [x] ADDRESSED: python3 -I -B scripts/local_environment.py -- cargo test --offline -p sc-core
  --test formula_length_input_contract:3 passed/0 failed,25 origin/state pairs, rc=0.
- [x] NO REGRESSION: same command verifies signed limits/zero, all other source variants and source
  immutability, rc=0. Broad evaluator/platform proof .5e.2/.5g/pre-push; other adapters .5e.1.
- [x] LOCKSTEP:338/233 Git/public bytes and book render pass0; live/book align. Promotion declined:
  implementation of existing canonical length/state contract; no new cross-cutting policy.
  Book warned on bare From<Length>; code spans remove the HTML warning, render rc=0.
