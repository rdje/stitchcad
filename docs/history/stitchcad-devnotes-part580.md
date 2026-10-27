# Sealed engineering continuity — part580

- **Identity:** 39 lines, 3222 bytes, sha256:556a2815505558b36c1d6ea5cb8435658312ff3633309c4ae3679fee8e1f4e72
- **Coverage:** whole0427 private-preparation task/lesson/ledger from 12b23f6930dd1d104f6b3a040b0f012c715072ee.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 42187–44146, 1959 bytes, sha256:586d97d27ee99011b2f39dd2a8852d6ed13d9518fd6f09bc55f857bedbcff2bd
- Whole source interval: DEV_NOTES.md bytes 193–984, 791 bytes, sha256:60650198b25c4101909019ced81c4787c833098adcc0cee3366857bcb5d75895
- Whole source interval: CHANGELOG.md bytes 207–679, 472 bytes, sha256:60c4de1f1cb40b900e9f56d3168a4e6fb6a0f3ce82171010abbe00adf8683397
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.4.1`
  Status: `done`
  Goal:0427 actual private validated model/report ownership through preparation; public preview
  consumes/discards the model. No public prepared handle, publication or group success claimed.
  - [x] REPRODUCE / ISSUE: git show 25f29c8:crates/sc-core/src/command/bus/measurement.rs prints
    only report return after actual candidate construction, rc=0; group staging cannot retain that model.
  - [x] ROOT CAUSE (WHY + WHERE): cargo test -p sc-measure --test design_inputs_contract command_bus:
    18 passed rc=0; retain same validated state/report internally without exposing a replacement capability.
  - [x] FIX: cargo clippy -p sc-measure --test design_inputs_contract -- -D warnings:finished rc=0;
    private PreparedCommand owns state/report, private prepare dispatches all three actual handlers.
  - [x] ADDRESSED: cargo test -p sc-core --doc command::bus:4 passed rc=0; external generic code with
    correct host/authority bounds cannot call prepare. Actual public preview still discards the private state.
  - [x] NO REGRESSION: cargo test -p sc-measure --test design_inputs_contract command_bus:18 passed
    rc=0; selected fmt --check/strict Clippy rc=0. No original state/revision/reserve changes or new public API.
  - [x] LOCKSTEP: whole0426 Measurement/D283 scope/lesson/ledger retained579; book/pointer/decision
    synchronized. transition_receipts.py record579:3 exact Git spans rc=0; retention OK rc=0.
    Native publication:80 chapters/159 scoped API rows/1462 source links/2440 rendered links rc=0.
    Maintained-reference81 files/17515 lines/1222838 bytes rc=0; full bus .6.9 remains deferred.
  Cargo commands use scripts/local_environment.py. Native sewing EaseValidation::DeferredToG2AndG3
  remains explicit: paired split/Dart need real recipe/geometry. Existing transaction prerequisite is safe
  for the implemented three metadata handlers before that dependent work is executable.
## _(2026-10-27 UTC)_ — private prepared model/report ownership

0427 .6.5.4.1 retains each actual validated metadata candidate and its exact report in private
PreparedCommand. Private prepare retains original auth/binding/source/difference checks; public
preview consumes the object, drops the model and returns the report. No candidate/prepare handle
is public. Real bus18/privacy4/strict selected lint/fmt pass; group staging/publication remain pending.
Native sewing ease is explicitly deferred to G2/G3; paired split/Dart keep their real dependencies.
Promotion: command-layer decision. D283 labeled publication summaries remain canonical.

[Whole0426 Measurement/D283 task/lesson/ledger](docs/history/stitchcad-devnotes-part579.md) retains exact source proofs and correction scope.

## STITCHCAD-G1-0427 — retain validated candidates behind private preparation

- Private prepared object owns actual validated model and derived report for all three metadata handlers.
- Public preview discards the model; callers cannot invoke preparation or obtain a model handle.
- Focused real bus18/privacy4/lint/fmt; existing semantics and original causes remain unchanged.
- Group staging/publication and actual recipe/geometry execution remain separately owned.

