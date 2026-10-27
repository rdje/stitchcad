# Sealed engineering continuity — part577

- **Identity:** 37 lines, 3076 bytes, sha256:bb276d57b306e87e83d5e51945422cff99364aae355b2d5d203501820286091e
- **Coverage:** whole0425 canonical-replacement task/lesson/ledger from 411f74d529a15b5186ba13c65b736fb4ed7199da.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 41260–43150, 1890 bytes, sha256:1d724e409497bfd6faae5f5bfe149c044401de46680b21ceca40574275375e6e
- Whole source interval: DEV_NOTES.md bytes 193–909, 716 bytes, sha256:686cf96cdc2ad63e2a85fa93e738dbb6fda4b5278ca8c2b6a3e94ffd085b8883
- Whole source interval: CHANGELOG.md bytes 207–677, 470 bytes, sha256:5a86748c267626ca9c888ec705a2a5446a18c1dddd98b75d29e434246f3331a8
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.3.1.1`
  Status: `done`
  Goal:0425 immutable dedicated canonical length replacement, stable existing id/position, exact
  source/state preserved; original owner/other records unchanged. Actual binding/candidate .1.2 next.
  - [x] REPRODUCE / ISSUE: git show c027bc9:crates/sc-core/src/value/inventory.rs prints constructor/
    borrowed-only ownership with no stable replacement operation, rc=0; handler would duplicate copying.
  - [x] ROOT CAUSE (WHY + WHERE): cargo test -p sc-core --test canonical_inputs_contract:9 passed rc=0;
    dedicated lengths and general Length-kind scalars are distinct; replacement must target existing owner.
  - [x] FIX: cargo clippy -p sc-core --test canonical_inputs_contract -- -D warnings:finished rc=0;
    clone owner/replace matching dedicated record only; typed MissingLength preserves exact requested id.
  - [x] ADDRESSED: cargo test -p sc-core --test canonical_inputs_contract:9 passed rc=0; new2 cover
    original order/source/Unknown/Derived/no-op and missing/general-scalar targets, without numeric defaults.
  - [x] NO REGRESSION: canonical_inputs_contract9 rc=0 includes prior7; cargo test -p sc-core --doc
    value::inventory:1 passed rc=0; selected strict Clippy/fmt pass rc=0. No domain authority granted.
  - [x] LOCKSTEP: whole0424 task/lesson/ledger retained576; book/pointer/decision/task/ledger updated.
    transition_receipts.py --record docs/history/stitchcad-devnotes-part576.md:3 exact Git spans
    rc=0; retention OK rc=0; book build/publication80 chapters/159 tables/1461 rows/2437 links rc=0.
    Maintained-reference candidate81 files/17454 lines/1218767 bytes rc=0; integration remains .6.9.
  Cargo commands use scripts/local_environment.py. No Measurement binding/source truth/evaluation or
  committed mutation follows from detached value replacement; actual candidate .1.2 retains these duties.
## _(2026-10-27 UTC)_ — stable canonical length replacement

0425 .6.5.3.1.1 clones canonical owner and replaces only an existing dedicated declaration at its
original id/position. Exact supplied source/Unknown/Derived state survives; every other length/scalar
and original owner stay unchanged. Missing/general Length-kind scalar targets refuse MissingLength.
Focused canonical9/privacy1/lint/fmt check component behavior. Promotion: command-layer decision.
Actual Measurement binding/private candidate .1.2 follows; no authority, evaluation or truth claimed.

[Whole0424 candidate-difference task/lesson/ledger](docs/history/stitchcad-devnotes-part576.md) retains exact source proofs and prior lesson pointers.

## STITCHCAD-G1-0425 — preserve canonical identity during length replacement

- Detached replacement updates an existing dedicated declaration at its original position only.
- Preserve exact source/state/provenance, other lengths/scalars and the original canonical owner.
- MissingLength refuses absent/general Length-kind scalar targets; no append or numeric fallback.
- Focused canonical9/privacy1/lint/fmt; actual Measurement binding/candidate remains .6.5.3.1.2.

