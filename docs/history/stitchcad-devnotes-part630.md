# Sealed engineering continuity — part630

- **Identity:** 37 lines, 4453 bytes, sha256:2ae709045ca474d6270ddcd113e9ba0b1e59ff00b558e5daf0335aa5a0eba764
- **Coverage:** whole0475 discarded-preparation scope, lesson, changelog and D295/D296 closure from b0ee34d0ec212db816bd462f778abde460d34fb5.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 52990–54845, 1855 bytes, sha256:da0977779aa992e96debbeeadfa3f90620c5d3d72147003faa07662b8b250303
- Whole source interval: DEV_NOTES.md bytes 193–1335, 1142 bytes, sha256:92e7cd53c62a2edf8a6614173bd7a42dfb2bf9db99c493997d742a1341a297ab
- Whole source interval: CHANGELOG.md bytes 207–1001, 794 bytes, sha256:f95a51251d3a3b96ed704f01f87d3e34eb5249934b0666d526f9c40bea88afbd
- Whole source interval: docs/tasks/PLANNING.md bytes 246–908, 662 bytes, sha256:4cd589dac6c015a82508665a32c74b0cd432a78c9f5fe14778cd443cb6739a59
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.9.1`
  Status: `done`
  Goal:P1 D295 complete failed-group discarded-preparation diagnostics from actual successful first
  occurrences; preserve original index/command/key order, aliases, native source chain and atomic retry.
  Own P3 D296 stale G0 command-census header claiming no command code; correct its spec-only scope.
  - [x] REPRODUCE / ISSUE: cargo test -p sc-measure --test design_inputs_contract failed_group_diagnostic
    →0 passed/1 failed rc=101;command_group_failed contract requires missing discarded prior commands.
  - [x] ROOT CAUSE: cargo test -p sc-measure --test design_inputs_contract failed_group_diagnostic
    →0 passed/1 failed rc=101;actual error names only group817000/index4/AddNotch;D296 header stale.
  - [x] FIX: error-only actual-step producer/fixed Copy index-command-key DTO for Mutation/RetryConflict/
    Difference;exclude aliases/failing/unexecuted,keep native chain. D296 comment only,executable exact.
  - [x] ADDRESSED: cargo test -p sc-measure --test design_inputs_contract failed_group_diagnostic →1 passed rc=0;
    transaction3 covers positions0/2/3,keys/native/conflict/empty prefix and same-key/two-ID/one-slot retry.
  - [x] NO REGRESSION: cargo test -p sc-measure --test design_inputs_contract command_bus::group_preview:: →6 passed rc=0;
    group_commit6/strict core+test Clippy/fmt pass0;lifecycle/accounting unchanged,D296 census17/5/5/0 rc=0.
  - [x] LOCKSTEP: book85 chapters/159 API rows/1545 source/2601 rendered links rc=0;live docs/decision/
    index updated,LIVE D295/D296 closed,README correct unchanged;whole0474/decisions629,4 Git spans rc=0.
  Verification: above focused pass0;retention1083/2425686residentB rc=0;workflow/full bus/publication pending.
  Commit: STITCHCAD-G1-0475;next .6.9.2 actual native workflow evidence,absent adapters remain unclaimed.
## _(2026-10-28 UTC)_ — exact discarded-preparation diagnostics

0475 .6.9.1 fixes D295:Mutation/RetryConflict/Difference carry actual successful first-occurrence index/command/key descriptors; aliases/failing/unexecuted members omitted. Fixed Copy context only; native source chain unchanged.
Native D2951 fails before/passes after;transaction3/group-preview6/group-commit6/strict lint/fmt pass0. Preview/commit yield positions0/2/3 and original keys;empty-prefix,key conflict and same-key/two-ID/one-slot corrected retry hold.
Error-only borrowed-step producer leaves lifecycle/accounting untouched. D296 stale census header corrected;17 commands/5 classes/5 levels/0 failures,non-comment bytes unchanged. Source/accounting/physical proof is not expanded.
Promotion:indexed decision. Book85/159/1545/2601 and retention1083/2425686B pass0;whole0474 and0471–0474 decisions retained629(4 Git spans). LIVE D295/D296 closed;README unchanged;next .6.9.2 native workflow evidence before full bus.
[Whole0474 transaction/decision continuity](docs/history/stitchcad-devnotes-part629.md) retains exact native proofs and compatibility boundaries.
## STITCHCAD-G1-0475 — report exact discarded group preparations

- Fix D295:member, retry-conflict and final-difference failures retain actual successful first-occurrence positions/commands/keys; aliases and failed/unexecuted members add no entry.
- Preserve original native source chains and same-key/two-ID/one-slot corrected retries. Native diagnostic1 red→green;transaction3/group-preview6/group-commit6/strict lint/fmt pass.
- Fix D296's stale census header; executable bytes stay exact and the17-command/5-class/5-level census passes0. Lifecycle/accounting producers remain unchanged.
- Whole0474 and0471–0474 decision continuity retained629; book/live docs align. Next native workflow evidence, then broader bus/publication acceptance; no additional authority or physical proof.

## D295 — failed-group diagnostics omitted discarded prior commands
P1 fixed0475, owner G1-SLICE.6.9.1; native diagnostic1 failed before/passed after;transaction3/group-preview6/group-commit6/strict lint/fmt pass0.
Mutation/RetryConflict/Difference now carry actual first-occurrence index/command/key context; aliases/failing/unexecuted members excluded. Preview/commit preserve positions0/2/3, original native cause and same-key/two-ID/one-slot corrected retry; empty prefix stays empty.
D296 P3 fixed0475 same owner:corrected stale command-census header against actual schemas/bus;non-comment source unchanged,17 commands/5 classes/5 levels/0 failures rc=0.
