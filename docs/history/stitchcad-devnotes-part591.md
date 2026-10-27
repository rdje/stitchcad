# Sealed engineering continuity — part591

- **Identity:** 58 lines, 5129 bytes, sha256:d042a55cd617f20b962490e0ed171710cab033890063805b45784d266dc51b8f
- **Coverage:** whole0439 scope/lesson/ledger and D285/D286 current ledger from 1437b493d97a3746b55ea081a7ed82ebf5e53fce.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 42910–45604, 2694 bytes, sha256:851f9e53c56c74fcfbe2cf3114c6f85e2472a18012035526a637d67ab274fca1
- Whole source interval: DEV_NOTES.md bytes 193–986, 793 bytes, sha256:801cf7c680485eefd8060fadcd6c6aa60167c4076fb4208d487805b5dc99e23a
- Whole source interval: CHANGELOG.md bytes 207–716, 509 bytes, sha256:39ca0489d8378354e27946dab70d343c7ddc55a2235c25600f68defe6fa645a7
- Whole source interval: docs/tasks/PLANNING.md bytes 246–1379, 1133 bytes, sha256:6702b3f11d23d5bfb4315e1cc2dfdc195722db7121a3517ef587f597d646b25d
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.8.1.2`
  Status: `done`
  Goal:0439 install immutable host limits once, admit costly metadata entry inputs before private work,
  emit real phase-local progress and honor cooperative cancellation with no partial semantic effects.
  Closed audit/result copies precede ReadyToPublish/final fresh authority; no observer or large payload
  preparation follows that final check. Default methods use unit observer with identical installed policy.
  - [x] REPRODUCE / ISSUE: python3 -I -B native HEAD bus assertions print no installed operation
    policy or runtime observer entry rc=0; prior data-only declarations do not control actual runtime.
  - [x] ROOT CAUSE (WHY + WHERE): transition_receipts.py --record docs/history/stitchcad-devnotes-part590.md:
    3 exact Git spans rc=0; whole0438 scope/lesson/ledger preserves its explicit runtime-pending boundary.
  - [x] FIX: cargo clippy -p sc-measure --test design_inputs_contract -- -D warnings:finished rc=0;
    one-time private policy, original-input admission, real checkpoints and prepared closed final payloads.
  - [x] ADDRESSED: cargo test -p sc-measure --test design_inputs_contract command_bus:44 passed rc=0;
    seven control cases cover setup/cheap read, target/byte bounds before source work, single/group
    preview and commit cancellation after actual staging, alias fractions, Ready cancellation/revocation,
    history cancellation/retry/replay, unchanged issuance/keys and original unauthorized/target causes.
  - [x] NO REGRESSION: cargo test -p sc-core --doc command::bus:8 passed rc=0; selected cargo fmt
    --package sc-core --package sc-measure --check rc=0. Prior37 native bus cases retained explicitly
    configured test-host policy; no hidden production default. Initial test tuple lint/constructor mismatch
    corrected with named Stop fixture; no suppression or changed runtime acceptance.
  - [x] LOCKSTEP: bash scripts/history_archive.sh verify-retention:1044 logical/74 aggregate working
    Markdown rc=0; active generation remains within64/group, aggregate192. Pointer/frontier/book/README/
    live status/lesson/indexed decision aligned. Full .6.9 and deeper .6.5.5 acceptance remain deferred.
  Native book build/publication:81 chapters/159 scoped API rows/1472 source links/2466 rendered links
  rc=0; maintained-reference candidate82 files/18015 lines/1256963 bytes owner .6.8.1.2 rc=0.
  Cargo invocations use scripts/local_environment.py. Promotion: indexed command-layer decision.
  Input admission does not prove total model/work/memory bounds; collection reserved capacity may grow
  without changing published semantic state. Storage/reopen D285 issuance/audit proof remains .7.
## _(2026-10-27 UTC)_ — actual immutable admission and cooperative cancellation

0439 .6.8.1.2 enforces one-time explicit metadata policy before private source/model work on all
costly metadata entry paths. Real phase checkpoints cancel before effects; default methods use unit
observer. Closed audit/result copies precede ReadyToPublish and final fresh authority; native observer
revocation there refuses publication. Bus44/privacy8/strict selected lint/fmt pass. Named Stop fixture
replaces initial complex tuple lint/constructor mismatch, without suppression. Cheap authorized reads
need no policy; no hidden production default. Promotion: command-layer record. Total model/resource
bounds .6.8.2 and deeper/full acceptance .6.5.5/.6.9 remain; whole0438 scope/lesson/ledger retained590.

## STITCHCAD-G1-0439 — enforce metadata admission and atomic cooperative cancellation

- Immutable one-time host policy controls costly metadata preview/commit/history; authorized reads need none.
- Actual data-only checkpoints preserve semantic state/keys/ids on Cancel; final authority follows Ready.
- Prepare closed audit/result copies before final boundary; native bus44/privacy8/strict lint/fmt pass.
- Book/setup/remaining resource scope aligned; whole0438 retained590 with3 exact Git source spans.

## D285 — restored revision reused by a different undo branch
P1 runtime portion fixed0437 (G1-SLICE.6.7.1.3); persistence/reopen remains owned/pending in G1-SLICE.7.
- [x] ROOT CAUSE: python3 -I -B docs/tasks/artifacts/command_layer/revision_branch_probe.py: pinned
  native contract model reuses1 for distinct content and accepts old1, rc=0; diagnosis retained588.
- [x] ADDRESSED: cargo test -p sc-measure --test design_inputs_contract command_bus:37 passed rc=0;
  actual undo0/new branch2/next unused id/old1 refusal and ordered Undo/branch witnesses verified.
- [x] NO REGRESSION: cargo test -p sc-core --doc command::bus:7 passed rc=0; strict selected Clippy/fmt rc=0.
Storage must restore maximum-issued revision/audit sequence independently of noncanonical undo history.
D283/D284 remain fixed; [whole correction/proofs](../history/stitchcad-devnotes-part581.md).

D286 P2 fixed0437, owner G1-SLICE.6.7.1.3: three native ce5fce9 module/book intro clauses falsely said metadata publication/history pending.
Scoped prior/current census3/0 rc=0; corrected actual metadata scope, native bus37/rendered publication verify it.

