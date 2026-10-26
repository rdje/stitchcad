# Sealed archive — completed implemented numeric call provenance scope

Immutable historical segment, sealed by leaf `G1-SLICE.5e.3.4` on `2026-10-26` (UTC).

- **Sealed identity:** 19 lines, 1693 bytes, sha256:1a4f318b55a4034d0b5b2fa677cf0f3d001376c9b28af2e305cd3d3c4495c2f1
- **Coverage:** whole committed source interval from a88c7b7bb05bbb866265128b39c1143967cf35fd.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 35701–37394, 1693 bytes, sha256:1a4f318b55a4034d0b5b2fa677cf0f3d001376c9b28af2e305cd3d3c4495c2f1
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5e.3.3`
  Status: `done`
  Goal:0270 implemented numeric built-in calls compute and retain sources automatically.
  Paths: crates/sc-core/src/recipe/evaluated.rs;crates/sc-core/src/recipe/evaluated/calls.rs;crates/sc-core/tests/formula_evaluated_calls_contract.rs
  Own sqrt/hypot/abs/min/max/clamp/round_to/arc_length wrappers, all evaluated input unions,
  exact-valued call contributions and first actual typed argument/domain/arity refusals.
  Reuse numeric kernels; no missing trig, geometry, AST, conditional or recipe execution claim.
  Retain354 whole0269 task,355 oldest full product/quotient lesson,242 ledger; book/live sync.
- [x] REPRODUCE:0270 1e836c1 requires manual source attachment for numeric calls; source read rc=0.
- [x] ROOT CAUSE:0270 returned/winning numeric values alone omit executed call/candidate/bound
  sources. Actual eight-family wrapper native5/0 verifies automatic unions/call sources, rc=0.
- [x] FIX: eight existing numeric kernels wrapped with complete input unions and required own sources.
- [x] ADDRESSED:0270 python3 -I -B scripts/local_environment.py -- cargo test --offline -p sc-core
  --test formula_evaluated_calls_contract:5 pass/0 fail, rc=0.
- [x] NO REGRESSION:0270 all family/input/unchanged/perfect/storage/arity/refusal controls pass;
  cargo clippy --offline -p sc-core --test formula_evaluated_calls_contract -- -D warnings, rc=0.
  Missing trig .5d, full .5e execution and broad .5g/pre-push remain owned; no absent call stubbed.
- [x] LOCKSTEP:354/355/242 whole Git/hash/public records and book render rc=0; live/frontier align.
  Promotion declined: existing numerical/D121 contracts; no new language/precision limit.
