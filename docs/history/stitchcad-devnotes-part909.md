# Sealed engineering continuity — part909

- **Identity:** 29 lines, 2673 bytes, sha256:bcc51ffc015450777aff9bed80f101eaa59b1cd2326add4f41ec90a25be0d8f4
- **Coverage:** whole0735 scope,lesson and changelog from 431f1fd33b3ddc97891d5d534a1c07b9921b539a.
- Whole source interval: docs/tasks/G1-SLICE-mirror.md bytes 5832–6938, 1106 bytes, sha256:4366eacdcf44b57400144dcbc1ae2fd27baa43684286344fab7ea3cb60c332f0
- Whole source interval: DEV_NOTES.md bytes 193–1159, 966 bytes, sha256:f15aaecd7df1920feb5e7cb3de70ff611738c933e8efc0332acf7dea5b591182
- Whole source interval: CHANGELOG.md bytes 217–818, 601 bytes, sha256:fea70d516dbee6b3b14cab3010c7612c6494792d8b1fe26360340cbb771071c6
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.5f.1.c.mirror.outputs.copies.plan.entries`
  Status: `done`
  Goal:explicit complete result-copy choices;no applied effects.
  Design:Retain/Retire borrow actual result copies;Issue borrows raw definitions,not issued CutCopy.
  Every old result copy is explicit;source/other copies cannot transfer or retire. No id reuse.
  - [x] ISSUE:result-copy quantities need complete explicit choices.
  - [x] ROOT CAUSE (result copies):actual copy/cut APIs reviewed;whole N/C/2N before scans;
    actual result quantity/mirroring used;old kernels unchanged;rc=0.
  - [x] FIX:complete Retain/Retire/Issue plan with authentic original fields/order.
  - [x] ADDRESSED (result copies):cargo test --test mirror_piece_result_copy_contract
    --test mirror_piece_copy_identity_contract:6+4 pass;rc=0. Full invocation in receipt.
  - [x] NO REGRESSION (result copies):API4/copy book3,lint/fmt/build/map/render/908 spans pass;rc=0.
  - [x] LOCKSTEP:book/live align;promotion: declined (existing identity/cut/effect boundary).
  Proof:target/history-proof-0735-mirror-result-copies/;Commit:STITCHCAD-G1-0735.
## _(2026-10-31 UTC)_ — declare complete mirror result-copy intents

0735 admits whole N/C/2N before operation/intent scans. Every actual old result copy requires explicit Retain/Retire;Issue retains raw definitions with explicit result Piece/id/orientation.
Source/other copies cannot transfer or retire;known ids and retirement cannot permit reuse. Authentic fields/order retained;active counts obey actual result cut policy,independent of original source copies.
Native6+4/API4/copy book3,selected lint/fmt/build/map/render pass0;old kernels unchanged,whole0734 including window27 dependency spans908 exact0. Source/plan/input definitions remain unchanged;no immutable copy minted.
Promotion: declined (existing identity/cut/effect boundary). Native result plan closes;complete companion association next. Actual currentness/global reservation/issuance/retirement/consumer/physical/geometry effects and full integration remain owed. G1still6/18/G0 unapproved.

## STITCHCAD-G1-0735 — declare complete mirror result-copy intents

- Whole N/C/2N precede operation/intent validation;actual old results require explicit retention or retirement.
- Proposed new definitions use explicit ids/result Piece/orientation;known or retired ids cannot be reused.
- Authentic original fields/order retained;actual result cut quantity/orientations govern active proposed copies.
- Native10/API4/copy book3 and focused lint/fmt/build/map/render/history908 checks pass.
- Native result plan closes;actual model/physical effects and complete companion association remain owned.

