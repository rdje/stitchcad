# Sealed engineering continuity — part640

- **Identity:** 23 lines, 3647 bytes, sha256:662192cf3ea3211aeabe9103b7f21729947a6f5cb66f966e6a3fa07117ef9655
- **Coverage:** whole0485 task scope, lesson and changelog from 44534dc86d929fead2942538426afeaf2f723346.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 38967–40710, 1743 bytes, sha256:1a8fb036338fc4076f6c50239063f1931d0dfec09015dfc7a15df694aca035d5
- Whole source interval: DEV_NOTES.md bytes 193–1501, 1308 bytes, sha256:808729b8c85c5bc6df47e7676173fa6df82a651cafc40a97266040c22d22c023
- Whole source interval: CHANGELOG.md bytes 207–803, 596 bytes, sha256:1646227562ba3eda2e34b90c201a2f54aae3a50e63b6f4844479f595c84ebe6f
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.3.2.1`
  Status: `done`
  Goal: exact held source-frame interval split_at prerequisite;D300 scope correction and closed-proof retirement.
  - [x] REPRODUCE / ISSUE: git show HEAD:ontology/range.rs/interface review →positive-range constructor only,no reusable strict partition;paired handler needs exact original-frame child intervals,rc=0.
  - [x] ROOT CAUSE: source review/actual core range-point fold →copying exact ascending bounds suffices without ratio recomputation or ledger edit;preparation.rs three-handler match proves D300 all-execution-pending prose stale,rc=0.
  - [x] FIX: add EdgeRange::split_at and Copy RangeSplitError retaining original interval/cut;strict interior guard,unchanged edge/frame and fixed array. Correct D300 to implemented metadata versus pending paired/Dart execution.
  - [x] ADDRESSED: cargo test -p sc-core --test range_contract →17 passed rc=0;four new cases cover148 exact sample memberships,8 endpoint/exterior refusals,full-width/unknown references and actual later ledger edits.
  - [x] NO REGRESSION: cargo test -p sc-core --doc ontology::range::EdgeRange::split_at →1 passed;clippy --lib --test range_contract -- -D warnings/fmt pass0;glossary310/9/158/0 and final book87/162/1556/2641 rc=0. D300 actual handler source bytes unchanged,scope predicate passes0.
  - [x] LOCKSTEP: MEMORY/DEV/CHANGELOG/D300/indexed decision/index/LIVE/book/API-status updated;README unchanged. transition_receipts.py --record part639 →4 exact Git spans rc=0;history_archive.sh verify-retention →1093/2477796 residentB rc=0.
  Commit: STITCHCAD-G1-0485;next .2.2 binds actual original A/B source cuts;paired physical/ease/stop/lineage/admission/transaction execution remains parent work.
## _(2026-10-28 UTC)_ — exact held-range partition prerequisite

0485 .6.5.3.2.1 adds EdgeRange::split_at with strict original-frame interior bounds and Copy RangeSplitError retaining original interval/cut. Fixed-array children preserve edge/endpoints without arithmetic rescaling,allocation,identity issuance or topology edits.
Four new cases cover148 exact membership samples,eight endpoint/exterior refusals,full-width parameters,unknown-source repairs and later actual ledger split mapping. Initial native16/1 caught Param Display's t= prefix;format exact rational evidence directly. Final range17 and method rustdoc1 pass0.
Selected strict library/range-target Clippy and fmt pass. Real glossary310/9/158/0 and final book87/162/1556/2641 pass0. D300 stale all-mutation-execution-pending sentence corrected from unchanged actual three-handler bytes;paired/Dart execution remains pending.
Promotion:indexed exact-edge-parameter decision;whole0484/D298/D299639 verifies4 Git spans,retention1093/2477796 residentB passes0. LIVE adds D300 scope correction,G1still6/18. README unchanged. Next .2.2 actual A/B binding;paired-feature acceptance remains separately deferred.
[Whole0484 glossary acceptance/defects](docs/history/stitchcad-devnotes-part639.md) retains full41 and exact D298/D299 causes/repairs.
## STITCHCAD-G1-0485 — partition exact held source-frame ranges

- Add EdgeRange::split_at and structured RangeSplitError;strict interior cuts return two fixed-array intervals with original edge/frame and exact bounds.
- Native range17/method rustdoc1/selected strict Clippy/fmt pass;four new cases cover exact membership,refusals,full-width/unknown references and actual later topology edits.
- D300 corrects stale command execution scope. Final glossary0 failures/book87/162/1556/2641 passes;whole0484/D298/D299 retained639. Actual paired handler binding and broader execution remain owned.

