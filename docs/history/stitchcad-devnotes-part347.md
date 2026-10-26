# Sealed archive — completed core size context frame scope

Immutable historical segment, sealed by leaf `G1-SLICE.5e.1.5.2` on `2026-10-26` (UTC).

- **Sealed identity:** 16 lines, 1406 bytes, sha256:bf3935e3f89011cb891a758b3dd7612bb240dbfda1ef0994e87fb0298dad61cc
- **Coverage:** whole committed source interval from bf8f4c23885e99200cb71bd7839e88fe6603fdfe.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 32713–34119, 1406 bytes, sha256:bf3935e3f89011cb891a758b3dd7612bb240dbfda1ef0994e87fb0298dad61cc
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5e.1.5.1`
  Status: `done`
  Goal:0265 core size source/range/value frame and unsigned Count bridge.
  Paths: crates/sc-core/src/recipe/context.rs;crates/sc-core/src/recipe/context/size.rs;crates/sc-core/src/recipe/semantic.rs;crates/sc-core/src/recipe/numeric.rs;crates/sc-core/src/recipe/mod.rs
  Additional paths: crates/sc-core/tests/formula_size_context_contract.rs
- [x] REPRODUCE:0265 5b8af44 has size metadata without a value context; git source read, rc=0.
- [x] ROOT CAUSE:0265 supplied source/range facts differ from canonical membership/currentness;
  one-based range and identity base plus wide Count avoid inference/narrowing. Native5/0, rc=0.
- [x] FIX: closed size roles; immutable checked frame, exact Count/Boolean/source and named absence.
- [x] ADDRESSED:0265 python3 -I -B scripts/local_environment.py -- cargo test --offline -p sc-core
  --test formula_size_context_contract:5 pass/0 fail, rc=0.
- [x] NO REGRESSION:0265 role closure/false/missing/pins/invalid/full-width/binding controls pass0;
  cargo clippy --offline -p sc-core --test formula_size_context_contract -- -D warnings, rc=0.
  Canonical membership bridge .2; full registry/axes/broad .5g/pre-push remain owned.
- [x] LOCKSTEP:345/346/237 whole Git/public records and book render rc=0; live/book align.
  Promotion declined: existing formula3.1/membership/binding contracts; no actual membership claim.
