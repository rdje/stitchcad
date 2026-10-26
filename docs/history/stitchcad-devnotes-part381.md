# Sealed archive — whole sourced sincos implementation scope

Immutable historical segment, sealed by leaf `G1-SLICE.5d.1.2.2.2.b.2.a` on `2026-10-26` (UTC).

- **Sealed identity:** 16 lines, 1566 bytes, sha256:702cb9c5f625c0f57f85559a74d4065c73f1c5c2dd2390082a18579e5d2dc267
- **Coverage:** whole committed source interval from 83de1978fab2c6881d697c65177de8301542d8a1.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 10407–11973, 1566 bytes, sha256:702cb9c5f625c0f57f85559a74d4065c73f1c5c2dd2390082a18579e5d2dc267
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5e.3.6`
  Status: `done`
  Goal:0281 sourced sin/cos calls retain real operand/call contributions, including exact axes.
  Own379 whole0280 task,380 oldest complete arc/UI lesson,253 ledger; recipes co-edit has retention receipt.
  Paths: crates/sc-core/src/recipe/evaluated/calls.rs;crates/sc-core/tests/formula_evaluated_calls_contract.rs
  Reuse native values; actual round_to Angle upstream, cancellation/storage/T1 and Boolean/numeric
  refusals. Focused native/lint only; book/live sync. Full AST and inverse/tangent remain open.
  Acceptance0281:
  - [x] REPRODUCE:0280 native sin/cos lacked automatic sourced adapters.
  - [x] ROOT CAUSE0281: cargo test -p sc-core --test formula_evaluated_calls_contract now executes actual sin/cos through supplied-value adapters,6/0, rc=0; old dispatch had no methods.
  - [x] FIX: delegate actual native kernels and existing call_result union, including exact axes.
  - [x] ADDRESSED0281: same target6/0, rc=0; real round_to Angle inputs/call sources survive cancellation/binding/read and T1 refusal; wrong Boolean/numeric input actuals retained.
  - [x] NO REGRESSION0281: same target includes existing eight families,6/0, rc=0; selected cargo clippy -p sc-core --lib --test formula_evaluated_calls_contract -- -D warnings passes, rc=0. Broad proof .5d.4/pre-push.
  - [x] LOCKSTEP:379/380/253 whole Git/hash/public bytes exact, book/live aligned. G1still5/18; full AST/inverse/tangent open; promotion: declined (existing source/class/native contracts).
  Verification: rc=0; Commit: STITCHCAD-G1-0281.

