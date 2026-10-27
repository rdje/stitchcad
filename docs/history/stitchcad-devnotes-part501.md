# Sealed archive — whole0349 Design parameter ownership

Immutable historical segment, sealed by leaf `G1-SLICE.6.4.2.d.2.a` on `2026-10-27` (UTC).

- **Sealed identity:** 48 lines, 4317 bytes, sha256:4731b306d2f4842e056afdec19e4ac70c19442a8720224a7c4403690b1c2ebb5
- **Coverage:** complete committed parameter task/lesson/ledger from a21bbfd9b0b9db93fc266508d694085f56cb27d3.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 47517–49706, 2189 bytes, sha256:b6911960f23b459cea8f0da067ce69d82092fe4ee5bfdf049c5ca13dd98e5f2f
- Whole source interval: DEV_NOTES.md bytes 193–1589, 1396 bytes, sha256:70d55366523ef637cbff49d6b06ae6064bd9883f48f8bf6beb55dc1858be9105
- Whole source interval: CHANGELOG.md bytes 207–893, 686 bytes, sha256:5c08c85c38dfa79b2217c7b39aed2b47e71032035ed5e16726e0007a6d938bac
- **Retrieval:** bytes below the first rule and blank line; original source terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.6.4.2.d.1`
  Status: `done`
  Goal: immutable owned named Design parameters pin exact canonical target family/id/kind. Reuse
  MachineToken/reserved-name and formula input adapters; unique metadata ids/tokens and canonical-id
  disjointness. Current lookups borrow actual CanonicalInputs, retain Unknown/Derived state and
  refuse missing/changed kinds without retargeting or source/evidence truth claims. No reverse dependency.
  Own native/privacy/lint/book/API sync, whole0348 continuity500 and exact old book proof retirement
  before cap growth; retain public retrieval instructions and current functional contracts/examples.
  - [x] REPRODUCE / ISSUE: formula input metadata claims alone do not own named Design parameters;
    native design_parameters_contract exercises missing/changed targets and duplicate bindings9/0, rc=0.
  - [x] ROOT CAUSE (WHY + WHERE): FormulaDeclaration::input carries ids/kind, while the scalar adapter
    checks supplied canonical objects. New metadata pins family/id/kind and reuses this actual adapter;
    cargo test -p sc-core --test design_parameters_contract passes9/0, rc=0, target/*native-0349.log.
  - [x] FIX: immutable ordered parameters, unique metadata ids/tokens, reserved-name/canonical-id
    refusals and shared current-target queries. Unknown/Derived states/source remain canonical, uncached.
  - [x] ADDRESSED: python3 -I -B scripts/local_environment.py -- cargo test -p sc-core --test
    design_parameters_contract gives9/0, rc=0; exact role/errors, current false state and family pins.
  - [x] NO REGRESSION: privacy2/strict selected Clippy/fmt/book/publication77/108/1444/2359 pass0,
    target/design-parameters-*-0349.log. D250132 rows exactly baseline, delta0; full glossary rc=1.
  - [x] LOCKSTEP: API vocabulary/status/index/pointer/notes aligned; continuity500 compares6 exact Git
    source spans0, including both complete old book proof sections. Functional examples/commands retained;
    caps unchanged, G1still6/18. promotion: declined (canonical source/name/identity contracts). Commit0349;
    .2.d.2 measurement/Ease follows. Full host composition/Design/readiness/bus/integration/CI remains later.
## _(2026-10-27 UTC)_ — owned named Design parameters

0349 DesignParameters owns immutable metadata id/token/pinned canonical target family and scalar kind.
Identity/name checks run before target resolution; ids cannot alias canonical declarations, reserved
inputs cannot bind, and duplicate tokens refuse even for one shared target. Distinct names sharing a
target and empty drafts remain legal. Current declaration queries reuse the actual scalar adapter;
dedicated lengths retain their original borrowed representation. Same-id unknown→false preference
edits read current state/provenance; changed kinds or missing ids refuse rather than retarget peers.
Complete current validation rechecks metadata/canonical identity disjointness after replacement.
No value/state cache, source-id equality inference, evidence truth or cross-domain completeness follows.
Native9/privacy2/strict selected lint/fmt/book/publication pass0. D250132 glossary rows remain baseline,
delta0. Whole0348/D253 continuity500 matches6 Git spans, including complete historical book identity/input
proof sections; retrieval instructions and current functionality/examples/commands remain in the book.
Promotion: existing canonical source/name/identity contracts. Caps unchanged; G1still6/18; measurement/Ease
inventories .6.4.2.d.2 next. Full host composition/Design/readiness/execution/integration/CI remains later.

## STITCHCAD-G1-0349 — own named Design parameters with current canonical targets

- Add immutable DesignParameters: unique ids/tokens, reserved-name/canonical-id refusals and pinned target family/id/kind.
- Resolve actual current declarations through existing adapters; preserve state/source without cached values or retargeting.
- Native9/privacy2/strict selected lint/fmt/book/publication pass0. D250132 glossary rows remain exactly baseline.
- Whole0348/D253 and complete old book input proofs500 match6 Git spans; retrieval/current contracts remain public.
- Caps unchanged; G1still6/18. Measurement/Ease inventories next; full host/Design/readiness/bus/integration/CI pending.

End of whole0349 Design parameter continuity.
