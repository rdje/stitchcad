# Sealed archive — completed exact named tolerance comparison scope

Immutable historical segment, sealed by leaf `G1-SLICE.5e.3.5` on `2026-10-26` (UTC).

- **Sealed identity:** 21 lines, 2000 bytes, sha256:d5b9d8e7f2a4c498018b6f6339cee86e8dbfd37e0431120eb0493620507617c5
- **Coverage:** whole committed source interval from 60bcd0ef86c6179d929af21229ee8a6522acb4db.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 36054–38054, 2000 bytes, sha256:d5b9d8e7f2a4c498018b6f6339cee86e8dbfd37e0431120eb0493620507617c5
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5e.3.4`
  Status: `done`
  Goal:0271 exact named tolerance comparison over sourced scalar values and actual context.
  Paths: crates/sc-core/src/recipe/rational.rs;crates/sc-core/src/recipe/evaluated.rs;crates/sc-core/src/recipe/evaluated/tolerance.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_tolerance_comparison_contract.rs
  Compare absolute rational difference inclusively without materializing an out-of-domain/wide
  numeric expression; bounded exact workspace. Actual class/provider/operands and sources retained.
  T1 contribution guard precedes threshold; no missing context/default or invented comparison label.
  Retain356 whole0270 task,357 oldest full sum lesson,243 ledger; book/live sync.
  Full assertion naming/AST/provider lookup/replay remains .5e.2/.3/.4; focused native/selected lint.
- [x] REPRODUCE:0271 a88c7b7 lacks product tolerance comparisons; exact subtraction can exceed
  width/domain despite a valid Boolean comparison. Source/native controls reproduce, rc=0.
- [x] ROOT CAUSE:0271 comparison intermediates are not numeric expression results; exact bounded
  absolute cross-products plus actual context and source guard solve it. Native5/0, rc=0.
- [x] FIX: inclusive rational comparison and sourced named result/refusal; boxed complete error.
- [x] ADDRESSED:0271 python3 -I -B scripts/local_environment.py -- cargo test --offline -p sc-core
  --test formula_tolerance_comparison_contract: final5 pass/0 fail, rc=0; Fraction witnesses rc=0.
- [x] NO REGRESSION:0271 kinds/classes/threshold/source/context/width/domain controls pass;
  cargo clippy --offline -p sc-core --test formula_tolerance_comparison_contract -- -D warnings,
  final rc=0 after large-error101 repair. Full assertions/AST and broad .5g/pre-push remain owned.
- [x] LOCKSTEP:356/357/243 whole Git/hash/public records and book render rc=0; live/frontier align.
  Promotion declined: existing exact numeric/comparison/context and formula4.2 provenance contracts.
