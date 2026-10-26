# Sealed archive — completed explicit tolerance context scope

Immutable historical segment, sealed by leaf `G1-SLICE.5e.1.5.1` on `2026-10-26` (UTC).

- **Sealed identity:** 16 lines, 1379 bytes, sha256:3cd8b253688c0e992991e64c495ef5d69892c17b063771184ee42eaa9741156b
- **Coverage:** whole committed source interval from 5b8af4440272ed1630ccb7551fea193be7ad5da5.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 32117–33496, 1379 bytes, sha256:3cd8b253688c0e992991e64c495ef5d69892c17b063771184ee42eaa9741156b
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5e.1.4`
  Status: `done`
  Goal:0264 intrinsic1/10 and explicit sourced optional tolerance contexts.
  Paths: crates/sc-core/src/recipe/context.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_tolerance_context_contract.rs
- [x] REPRODUCE:0264 72cc909 has reserved tolerance metadata but no explicit value-context adapter;
  git source inspection rc=0. Formula3.1 forbids optional export/profile defaults.
- [x] ROOT CAUSE:0264 static visibility differs from provider availability; actual Tolerance classes/
  borrowed source ids preserve the required separation. Focused native5/0, rc=0.
- [x] FIX: immutable class-checked optional providers; intrinsic units defaults; exact Length read,
  source/derivation/actual label and typed unbound refusal. No G4 registry/physical approval inferred.
- [x] ADDRESSED:0264 python3 -I -B scripts/local_environment.py -- cargo test --offline -p sc-core
  --test formula_tolerance_context_contract:5 pass/0 fail, rc=0.
- [x] NO REGRESSION:0264 same target covers masks/classes/zero/replacement; cargo clippy --offline
  -p sc-core --test formula_tolerance_context_contract -- -D warnings, rc=0; broad .5e.3/.5g/pre-push.
- [x] LOCKSTEP:343/344/236 whole Git/public records and book render rc=0; live/book align.
  Promotion declined: existing formula3.1/units3 provider contracts; size/full execution pending.
