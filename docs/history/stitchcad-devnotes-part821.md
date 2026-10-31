# Sealed engineering continuity — part821

- **Identity:** 31 lines, 3919 bytes, sha256:7e2ecea7ece2c0c8463ca7b1565735975962ec3eeac305df5e6f7dc9a949ef93
- **Coverage:** whole0661 task scope, lesson, changelog and defect ledger from 22b8983c1806f41bb7719db6418b04f748afd9e9.
- Whole source interval: docs/tasks/G1-SLICE-producers.md bytes 61432–62831, 1399 bytes, sha256:599954add9610fee303d040f0883e2b373751f77a0d57172e144c2e2a4ecaaad
- Whole source interval: DEV_NOTES.md bytes 193–1724, 1531 bytes, sha256:1e5a0a41e64e4670d124992c2012645af378c92d3f72486b29c11cdcf3bd5c3a
- Whole source interval: CHANGELOG.md bytes 207–765, 558 bytes, sha256:a09122fc8dd3f2854ff311587bc2214a405a15ffbd2436d2f87fa7b46244d008
- Whole source interval: docs/tasks/PLANNING.md bytes 94497–94928, 431 bytes, sha256:cf7c1de7be5a6683d16da4b14146342a02c2c40d4f6af83bb61d9e4921ad1e49
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.5b.4c.h2.b.p.o.f.n16.2.ownership`
  Status: `done`
  - [x] ISSUE:D361 P1 platform freshness check can adopt a concurrent run and amend its receipts.
  - [x] ROOT CAUSE (WHY + WHERE):actual old main's nonexclusive prepare adopts competing inode;
    write_failure reaches trapped foreign failure publication;sentinel exact/no compiler/browser,rc=0.
  - [x] FIX:reset stale owner before preflight,reserve run/temp exclusively,retain temp identity
    until sealing;create report/failure/pending files exclusively and retain atomic latest rename.
  - [x] ADDRESSED:actual run collision refused before receipt/browser/compiler;actual temp
    collision leaves competing bytes exact and failure only in newly owned run;pending overwrite
    refused. Read-only watcher rejects three actual directory/file/reset omissions,rc=0.
  - [x] NO REGRESSION:four native plans/545 files/six reds and platform31/405/seven reds stay
    green;book101/374/1684/3002/history820 four Git spans/census/map/diff pass,rc=0.
  - [x] LOCKSTEP:book/live/notes/registry/frontier sync;D361 fixed. Promotion declined(existing
    immutable ownership/file contracts). n16/n16.2 active;effects/consumer/a41 pending.
    No fresh compiler/browser verdict;full D156/.h2.b.v/pre-push/app/release remain owned.
  Commit:STITCHCAD-G1-0661;Verification:actual controlled collisions and independent pure ownership guards.
## _(2026-10-31 UTC)_ — earn exclusive platform capture ownership

0661 closes n16.2.ownership. D361 P1:actual old prepare admits a concurrent run and reaches trapped foreign failure publication. Reset stale owner before preflight;reserve run/temp exclusively. Report/failure/pending files use exclusive creation;temp identity survives sealing and latest rename stays atomic.
Actual run collision refuses before receipt/browser/compiler. Actual temp collision preserves competing sentinel;failure stays in the new owned run. Existing pending pointer refuses overwrite. Independent pure helper/reset controls reject three actual body omissions. No source/capture bytes overwritten.
Four plans/current545/six reds and platform31/405/seven reds remain green. Book101/374/1684/3002/history820 four exact Git spans/census/map/diff pass. LIVE notes:D361 fixed;states/G1 6/18/G0 unapproved unchanged. Promotion declined:existing immutable ownership contracts. No fresh native/browser verdict.
[Whole0660](docs/history/stitchcad-devnotes-part820.md) retains D360 inventory proof. n16/n16.2 active;next effects,then actual consumer;a41/D156/.h2.b.v/pre-push remain owned. Session health flagged:long review/fresh context recommended;durable pointer names next slice. No user pause inferred.
Evidence:target/platform-0661-ownership.log plus actual controlled run/temp/pending red→green. Shared read-only watcher remains nonproducing. Native/Chrome execution stays separate;original reports and historical capture roots remain immutable.
## STITCHCAD-G1-0661 — earn exclusive platform capture ownership

- D361 fixes concurrent run/temp adoption and stale ownership. Actual collisions preserve competing bytes;failure stays in new owned run;pending overwrite refuses. Exclusive directory/file/reset body controls reject3 omissions;latest rename stays atomic.
- Inventory4/current545/six reds/platform31/405/seven reds/book101/374/1684/3002/history820/census/map/diff pass. No fresh runtime verdict;next effects/consumer. a41/D156/full integration remain owned;G1 6/18/G0 unapproved unchanged.

## D308–D361 — exclusive platform capture ownership earned

D308–D360:[whole prior ledger](../history/stitchcad-devnotes-part820.md). D361 P1 fixed0661:actual competing run adopted by nonexclusive prepare;exclusive roots/receipts/reset/temp identity controls pass,three body omissions rejected;competing bytes preserved. D334 P2 after G1/before G0;D223/PLANNING.5 open. Next effects/consumer;a41/D156/full integration owned.
