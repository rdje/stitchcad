# Sealed engineering continuity — part588

- **Identity:** 48 lines, 3999 bytes, sha256:2a0714f01de90aef38d806f3cf2d28596cd3d5012d8a87d8c9b421ce03429fe9
- **Coverage:** whole0436 task/lesson/ledger and D285 diagnosis ledger from ce5fce9f436c5a813a1aa22aad197ca3ec4aaed7.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 45143–47019, 1876 bytes, sha256:a1f88d1f76e394c09a1f3a43ceb5f734cf1a8994e6bb8da494b2d87b146a06a6
- Whole source interval: DEV_NOTES.md bytes 193–911, 718 bytes, sha256:a0340916b8a4d2dbb73f28fdf0ce27c9972d6363e95a1f71def720e7ef32849c
- Whole source interval: CHANGELOG.md bytes 207–713, 506 bytes, sha256:03e6b7774bfbbe502cb96ef0db216dfd2a1f20a769bc4838c227c7f08cde6697
- Whole source interval: docs/tasks/PLANNING.md bytes 246–1145, 899 bytes, sha256:63e8f0d8aca6935883a14eb11a98586b66680047291d00154becff369f56dbfa
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.7.1.2`
  Status: `done`
  Goal:0436 typed immutable history direction/current operator context/exact original group,
  explicit membership and Design consistency; original author/order/revision/keys never rewritten.
  - [x] REPRODUCE / ISSUE: git show HEAD:crates/sc-core/src/command.rs | rg -n prints original group
    exports rc=0; operator/current-operation metadata must remain separate from original authoring data.
  - [x] ROOT CAUSE (WHY + WHERE): cargo test -p sc-core --test command_history_contract:3 passed rc=0;
    original group authority/revision cannot stand in for the current operation's actor/revision/key.
  - [x] FIX: cargo clippy -p sc-core --test command_history_contract -- -D warnings:finished rc=0;
    private owned original/context/direction, explicit original group membership then exact Design check.
  - [x] ADDRESSED: cargo test -p sc-core --test command_history_contract:3 passed rc=0; full role/
    principal distinction, exact counters beyond128 bits, original order/repeated keys and typed refusals.
  - [x] NO REGRESSION: cargo test -p sc-core --doc command::history:1 passed rc=0; selected strict
    Clippy/fmt --check rc=0. Direction/key differences remain input data, not accepted replay/effects.
  - [x] LOCKSTEP: transition_receipts.py --record docs/history/stitchcad-devnotes-part587.md:3 exact
    Git spans rc=0; whole0435 diagnosis retained with native-runtime proof still explicitly pending.
  Cargo commands use scripts/local_environment.py. Promotion: indexed command-layer decision.
  Native publication80 chapters/159 scoped API rows/1466 source links/2449 rendered links rc=0;
  retention1041/global71 fits rc=0.
  Constructor grants no authority/current snapshot/audit binding/model/replay or undo capability;
  actual bounded traversal, action audit and D285 fix/native acceptance remain .6.7.1.3/.7.
## _(2026-10-27 UTC)_ — current history operator and original authoring intent

0436 .6.7.1.2 owns current operator/revision/key separately from exact original group and direction.
Explicit original membership precedes Design consistency; full actor/principal, original revision,
order/payloads/aliases stay exact. Immutable getters/private fields confer no authority or effect.
Native history3/privacy1/selected strict lint/fmt pass. Promotion: command-layer decision. Runtime
traversal/action audit and D285 maximum-issued revision/native branch proof remain .6.7.1.3; reopen .7.

[Whole0435 D285 diagnosis scope/lesson/ledger](docs/history/stitchcad-devnotes-part587.md) retains the model/native-proof boundary.

## STITCHCAD-G1-0436 — preserve current history operator and original author

- Typed Undo/Redo intent owns current operator/revision/key and exact original authoring group separately.
- Validate explicit membership then Design; retain original author, exact revision, member order and keys.
- Native history3/privacy1/selected strict lint/fmt pass; no runtime traversal/authority or D285 fix claimed.
- Whole0435 diagnosis scope, lesson and ledger retained587 with three exact native source intervals.

## D285 — restored revision reused by a different undo branch
P1 latent integrity risk; owner G1-SLICE.6.7.1, in_progress. Restore counter then current.successor
can admit an old snapshot precondition against different content. Maximum-issued revision must survive undo.
- [x] ROOT CAUSE: python3 -I -B docs/tasks/artifacts/command_layer/revision_branch_probe.py: pinned
  native contract model reuses1, stale precondition matches; separate maximum-issued gives2, rc=0.
- [x] ADDRESSED: diagnosis scope only: revision_branch_probe.py records1/2 counterexample rc=0; runtime fix .6.7.1.3 pending.
- [x] NO REGRESSION: producer AST parsed rc=0; git diff --quiet HEAD -- crates rc=0; product code unchanged.
Persistence must restore maximum-issued revision; owned with actual storage/reopen in G1-SLICE.7.
D283/D284 remain fixed; [whole correction/proofs](../history/stitchcad-devnotes-part581.md).

