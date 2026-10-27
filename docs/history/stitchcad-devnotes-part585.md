# Sealed engineering continuity — part585

- **Identity:** 38 lines, 3136 bytes, sha256:f8e993e64e7979e4a4ec6a62ebef2189cbc1227dbcd1d1b22dd46143cc64fd3f
- **Coverage:** whole0433 private group-preparation task/lesson/ledger from 46b47c1dd3e6aa4fc12b8c90df3a6e3435e88689.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 43446–45422, 1976 bytes, sha256:77fe2886a988ade0611c1ac66d0b515c4ebfda3a9006e0829bae00e4f1564d3e
- Whole source interval: DEV_NOTES.md bytes 193–893, 700 bytes, sha256:6a552e84c7ee5165cc030e99189643fe37ada392da5f20388584ea64afefe7ce
- Whole source interval: CHANGELOG.md bytes 207–667, 460 bytes, sha256:2b1ccaae6333c3b61c4150da3960754c5e8bde56950a325c1c13a388a380e2b7
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.4.3`
  Status: `done`
  Goal:0433 private actual final prepared group model/report and concrete consumed cursor count;
  public preview discards publication-side ownership, without a caller-held mode/model capability.
  - [x] REPRODUCE / ISSUE: git show HEAD:crates/sc-core/src/command/bus/group.rs | rg -n prints
    direct Ok(CommandGroupPreview) and dropped local candidate rc=0; identity source has no consumed count.
  - [x] ROOT CAUSE (WHY + WHERE): native HEAD group/identity source rc=0; later atomic publication
    needs actual final ownership and concrete consumed reserve prefix, rather than a reconstructed report.
  - [x] FIX: cargo clippy -p sc-measure --test design_inputs_contract -- -D warnings:finished rc=0;
    private PreparedGroup/prepare_group retains actual state/report/count and trusted-only mode selection.
  - [x] ADDRESSED: cargo test -p sc-measure --test design_inputs_contract command_bus::group_preview:
    6 passed rc=0; actual sequential mixed models, net delta, retry aliases and later failure still pass.
  - [x] NO REGRESSION: cargo test -p sc-core --lib command::bus::identity:1 passed rc=0, exact cursor
    counts/final128-bit id/exhaustion/independence; cargo test -p sc-core --doc command::bus:5 passed rc=0;
    selected fmt --check rc=0. New compiler case refuses direct group preparation with valid generic bounds.
  - [x] LOCKSTEP: transition_receipts.py --record docs/history/stitchcad-devnotes-part584.md:3 exact
    Git spans rc=0; whole0432 scope/lesson/ledger retained. Book/pointer/frontier/lesson aligned.
  Native publication80 chapters/159 scoped API rows/1465 source links/2446 rendered links rc=0;
  maintained-reference81 files/17663 lines/1232927 bytes rc=0; retention1038/global68 fits rc=0.
  Cargo commands use scripts/local_environment.py. Promotion declined: existing private ownership,
  preview and authority contracts; actual publication/replay/audit follows .6.6.1. Full .6.9 deferred.
## _(2026-10-27 UTC)_ — private whole-group preparation ownership

0433 .6.5.4.3 retains actual final model/report/concrete consumed identity count in PreparedGroup.
Public preview consumes/discards model and allocation information. Private mode selection cannot be
called by request submitters; native group6/identity1/compiler privacy5 and selected strict lint/fmt pass.
Cursor count changes only on successful yields, including final128-bit id once; exhaustion adds none.
Promotion declined: existing private ownership/preview/authority contracts; actual publication next.

[Whole0432 group-preview scope/lesson/ledger](docs/history/stitchcad-devnotes-part584.md) retains exact native sources.

## STITCHCAD-G1-0433 — retain prepared whole groups privately

- Keep actual final candidate/report and concrete cursor consumption together inside the bus.
- Public preview discards publication-side ownership; direct preparation/mode selection stays private.
- Native group6/identity1/privacy5/selected strict lint/fmt pass; no publication or persisted replay yet.
- Whole0432 scope, lesson and ledger retained584 with three exact native source intervals.

