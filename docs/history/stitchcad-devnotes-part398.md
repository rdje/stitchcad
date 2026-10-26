# Sealed archive — whole0289 sourced inverse scope

Immutable historical segment, sealed by leaf `G1-SLICE.5d.1.2.2.2.d.1` on `2026-10-26` (UTC).

- **Sealed identity:** 15 lines, 1465 bytes, sha256:655ba069112ba6b7f0e6f92e8e6c82739ca8cf0ecb6c0064680e7b3a4ea9e458
- **Coverage:** whole committed source interval from cbbfb7c1167d49a00ed2f7120e41910deb1edf99.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 11145–12610, 1465 bytes, sha256:655ba069112ba6b7f0e6f92e8e6c82739ca8cf0ecb6c0064680e7b3a4ea9e458
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5e.3.7`
  Status: `done`
  Goal:0289 sourced atan/atan2 retain every actual operand/call source and exact refusal priority.
  Paths: crates/sc-core/src/recipe/evaluated/calls.rs;crates/sc-core/tests/formula_evaluated_calls_contract.rs
  Own396 whole0288 native task,397 oldest0264 lesson,261 whole0288 ledger before live growth.
  Focused supplied-value/native/source/storage/T1 checks; book/live sync, broad .5d.4/pre-push.
  Acceptance0289:
  - [x] REPRODUCE: native inverse values existed without supplied-scalar call/source adapters.
  - [x] ROOT CAUSE0289: read numeric/provenance/calls contracts; cargo test -p sc-core --test formula_evaluated_calls_contract:8/0, rc=0, confirms source attachment is the missing boundary.
  - [x] FIX: delegate actual kinds/values to native inverses; union both operands and own call source, even exact axes.
  - [x] ADDRESSED0289: same focused target actual round_to/sqrt/hypot inputs, cancellation/storage/read/T1 and6x6 kind priority pass, rc=0.
  - [x] NO REGRESSION0289: existing six call tests plus two inverse tests8/0; selected lib/test strict Clippy and fmt pass, rc=0. No native/certificate source edits; broad .5d.4/pre-push.
  - [x] LOCKSTEP:396/397/261 whole Git/hash/public exact, Python comparisons rc=0; book build rc=0; live/decision align, G1still5/18.
  Promotion declined: existing numerical/provenance contracts; no new cross-cutting rule.
  Verification: rc=0; Commit: STITCHCAD-G1-0289.
