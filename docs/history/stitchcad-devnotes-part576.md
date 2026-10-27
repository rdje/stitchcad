# Sealed engineering continuity — part576

- **Identity:** 38 lines, 3130 bytes, sha256:e64d6680e857c3d998a655720dde5c46c907fbc65bd0af033b3c2e258c7cae96
- **Coverage:** whole0424 actual-difference task/lesson/ledger from c027bc918d0a5099b7ca2388fd005aa6424667f0.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 40496–42341, 1845 bytes, sha256:0d510442fcbfc5863203d71a084c2126b97b88bae580fe80f9e7b72ae2fb3b27
- Whole source interval: DEV_NOTES.md bytes 193–1007, 814 bytes, sha256:2cde8f00a89540d31f7bfc832d8279c278d4c059dc51ea28c75f65c3a9ba16f3
- Whole source interval: CHANGELOG.md bytes 207–678, 471 bytes, sha256:3486e2b07e6c8177637809166935d95773fd90738b206230de5b76558e57b337
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.2.c`
  Status: `done`
  Goal:0424 actual Material/Notch candidate differences, closed unchanged-owner coverage and
  structured refusal for unrelated context/Piece/order/landmark/intent changes. Publication remains .4.
  - [x] REPRODUCE / ISSUE: git show c665276:crates/sc-core/src/command/bus/material.rs prints
    report values copied from original/request before candidate construction, rc=0; whole diff was absent.
  - [x] ROOT CAUSE (WHY + WHERE): cargo test -p sc-core --lib command::bus::difference:3 passed
    rc=0; independently valid label/ordering/unselected edits must not be reported as only Material intent.
  - [x] FIX: cargo clippy -p sc-measure --test design_inputs_contract -- -D warnings:finished rc=0;
    exhaustive unchanged-owner definition comparison plus actual candidate assignment/append extraction.
  - [x] ADDRESSED: cargo test -p sc-core --lib command::bus::difference:3 passed rc=0;
    no-op/reason bytes, wrong intent, labels/order/unselected changes and actual appended Notch/prefix checked.
  - [x] NO REGRESSION: cargo test -p sc-measure --test design_inputs_contract command_bus:14 passed
    rc=0; cargo test -p sc-core --doc command::bus:3 passed rc=0; selected fmt --check rc=0.
  - [x] LOCKSTEP: transition_receipts.py --record docs/history/stitchcad-devnotes-part575.md:3 exact
    Git spans rc=0; whole0423 scope/lesson/ledger and pointers retained. Book/pointer/decision aligned.
  Book build/publication80 chapters/159 tables/1461 rows/2437 local links rc=0; maintained-reference
  candidate81 files/17436 lines/1217605 bytes rc=0; verify-retention OK rc=0; git diff --check rc=0.
  Cargo commands use scripts/local_environment.py. Actual host requires value equality for preview;
  queries retain original host bound. No public candidate/model/replacement capability is added.
## _(2026-10-27 UTC)_ — actual candidate semantic differences

0424 .6.5.2.c derives reports from the validated candidate, checks unchanged garment/input/sewing/
structure context and permits only selected Material fields or one exact appended Notch. Exhaustive
owner-definition patterns force comparison coverage when fields are added; source hosts must support
value equality. No-op omission/order/reason bytes stay exact. Unexpected candidate differences retain
structured causes and refuse before a report. Focused diff3/bus14/privacy3/lint/fmt check this scope.
Promotion: command-layer decision; publication and full bus/evaluator/runtime proof remain owned.

[Whole0423 private identity task/lesson/ledger](docs/history/stitchcad-devnotes-part575.md) retains exact proof and predecessor lesson pointers.

## STITCHCAD-G1-0424 — derive exact differences from actual candidates

- Compare original and validated candidate across complete implemented owner definitions.
- Derive exact Material/Notch reports; refuse unrelated context/Piece/order/landmark or intent changes.
- Exhaustive field patterns require new fields to enter comparison coverage; original state unchanged.
- Focused diff3/bus14/privacy3/lint/fmt; publication and full integration remain separately owned.

