# Sealed engineering continuity — part590

- **Identity:** 39 lines, 3235 bytes, sha256:ffd7d9a31fd6f5e700b274d52e3762949df0009a471c2ae38b072e042b95e04c
- **Coverage:** whole0438 admission/progress task/lesson/ledger from 2cfaf4ded94a6fc6749a318a100f5d7fb3fab2a5.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 42885–44793, 1908 bytes, sha256:e26171005767d43c7c7d4b99c89e8bd1d24a1102e37c6a3ced8c5b05d7ac5e6b
- Whole source interval: DEV_NOTES.md bytes 193–1000, 807 bytes, sha256:0d0a90ea6fa01c5019feaa24cc1cae0fd5bf2b8dc0793a1341d0807231b99adf
- Whole source interval: CHANGELOG.md bytes 207–727, 520 bytes, sha256:93dfa508377cd1261491200a67d13041445354b66839973bcc056ced3e339413
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.8.1.1`
  Status: `done`
  Goal:0438 explicit finite metadata input demand/policy and typed phase fractions/observer decisions;
  data-only admission grants no authority/model/effect capability or wall-clock/total-memory bound.
  - [x] REPRODUCE / ISSUE: git show HEAD:crates/sc-core/src/command.rs | rg -n prints original
    context module rc=0; policy/count/phase declarations must be distinct from an execution result.
  - [x] ROOT CAUSE (WHY + WHERE): cargo test -p sc-core --test command_control_contract:4 passed rc=0;
    original aliases and UTF-8 field bytes are distinct from deduped effects, character counts or CPU work.
  - [x] FIX: cargo clippy -p sc-core --test command_control_contract -- -D warnings:finished rc=0;
    positive explicit limits, borrowed declared demand/refusals, guarded arithmetic and data-only observer.
  - [x] ADDRESSED: cargo test -p sc-core --test command_control_contract:4 passed rc=0; exact3/5/8
    member/target/UTF-8 units, member-first/per-member refusal fields, phase endpoints/overrun retained.
  - [x] NO REGRESSION: cargo test -p sc-core --doc command::control:1 passed rc=0; strict lint/fmt0;
    git diff --quiet HEAD -- crates/sc-core/src/command/bus.rs crates/sc-core/src/command/bus rc=0.
  - [x] LOCKSTEP: transition_receipts.py --record docs/history/stitchcad-devnotes-part589.md:4 exact
    Git spans rc=0; whole0437/0421–0422 proof retained without losing ids. New linked book guide,
    pointer/frontier/lesson/decision aligned. Runtime installation/emission/cancellation .1.2 remains.
  Native publication81 chapters/159 scoped API rows/1471 source links/2463 rendered links rc=0;
  retention1043/global73 fits rc=0.
  Cargo commands use scripts/local_environment.py. Promotion: indexed command-layer decision. Counts
  are declared metadata input units; no total model memory/CPU/numeric/file/solver or runtime cancel proof.
## _(2026-10-27 UTC)_ — explicit admission units and progress-data contract

0438 .6.8.1.1 adds host-chosen positive member/metadata-target/proposed-reason-byte limits, counting
original aliases and UTF-8 fields exactly. Remaining-room comparisons precede arithmetic and retain
unmultiplied excess components. Typed phase fractions refuse completed>total; data-only observer
returns Continue/Cancel, without model or authority handles. Native control4/privacy1/strict lint/fmt
pass. Runtime bus unchanged; installation/emission/cancellation .1.2 next. Promotion: command-layer
record. These units are neither CPU time nor total model memory/peak allocation bounds.

[Whole0437 and0421–0422 completed proofs](docs/history/stitchcad-devnotes-part589.md) retain exact native spans; graph ids remain live.

## STITCHCAD-G1-0438 — define explicit metadata admission and progress contracts

- Positive host policy for submitted members, primary metadata targets and proposed UTF-8 assignment reasons.
- Exact per-member refusals and guarded arithmetic; typed phase fractions and data-only continuation decisions.
- Native control4/privacy1/strict selected lint/fmt pass; runtime installation/cancellation remains .6.8.1.2.
- Dedicated book guide states data/runtime limits; whole0437/0421–0422 completed proofs retained589.

