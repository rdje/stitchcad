# Sealed engineering continuity — part627

- **Identity:** 38 lines, 4321 bytes, sha256:28f0d0456fa312ff6f1d4102751b188b1cb6ecf05d18d60c89ac8e4e0cb77b50
- **Coverage:** whole0472 source ownership scope, lesson, changelog and D293 closure from ad143a4e9d7c9fa5122a80e0516aa2b4b4bf6655.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 52476–54435, 1959 bytes, sha256:4f08e765dd1b114951f653351752ecd36d9fe41d5fc775ec255ec2e9759e2c7c
- Whole source interval: DEV_NOTES.md bytes 193–1281, 1088 bytes, sha256:681e910da925a181c6e37879a45a039d06e172e5feaff68ec89dc95419e95a60
- Whole source interval: CHANGELOG.md bytes 207–930, 723 bytes, sha256:15b553f165b430452b0c8482b2d11e83950fcd505104597ae4205b8c52761de1
- Whole source interval: docs/tasks/PLANNING.md bytes 246–797, 551 bytes, sha256:9bb0402cfd6257f34143ab237474d8fc126f04020a174157857bca7068a121c4
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.8.2.2.2.2.4.3.4.3`
  Status: `done`
  Goal:P0 D293 freeze actual immutable source ownership across model snapshots; eliminate foreign Clone
  footprint assumptions; honest charged-cache regression, borrowed host/semantic contracts and logical counts.
  Decision:private Arc<H>, manual metadata Clone shares source; host()->&H becomes non-const; cross-thread
  ownership requires H:Send+Sync, single-thread hosts stay legal. Correct source-copy telemetry claims.
  - [x] REPRODUCE / ISSUE: original honest plain source diagnostic0471 accepted907B/retained908B;
    replacement regression2 both fail against inline H rc=101, not a fake source or fabricated total.
  - [x] ROOT CAUSE: native ownership check confirms inline derived H.clone; custom semantic Eq permits
    cache differences. cargo test --test design_inputs_contract command_bus::source_clone_footprint:: red2.
  - [x] FIX: private Arc<H>/exhaustive manual Clone preserves actual source and separately copies census;
    no mutable/public Arc capability, source shared-access immutability remains an integration obligation.
  - [x] ADDRESSED: same regression2 passed rc=0;actual source address/logical double charge,905/906/907B
    witnesses/current check;explicit foreign Clone control still grows cache. Metadata Clone needs no H:Clone.
  - [x] NO REGRESSION: affected command25/metadata8/doctest4, cargo clippy core lib/measure test -D warnings
    and cargo fmt --check pass0;native original one-root/consuming candidates unchanged0;source observer control1.
  - [x] LOCKSTEP: native book85 chapters/159 API rows/1535 source/2587 rendered links rc=0;
    live docs/decision/index close D293,README correct unchanged;whole0471 retained626,retention1080/2404556B rc=0.
  Verification: focused above rc=0;host() non-const/cross-thread Send+Sync documented;coupled/full pending.
  Commit: STITCHCAD-G1-0472; next .4.4 coupled resource acceptance after completed implementation.
## _(2026-10-28 UTC)_ — freeze actual immutable source ownership (D293)

0472 .6.8.2.2.2.2.4.3.4.3 closes P0 D293:private Arc<H>/manual metadata Clone retain the same source instance; captured entity vectors remain separate and every logical model charges full source content.
Regression2 first failed, now passes:exact906B/current retained check;905B preserves880+26 refusal;old907B also passes. Explicit honest foreign Clone still grows cache; actual model snapshots no longer invoke it.
Foreign-copy observer control1, all bus foreign clones0; core model copies remain. Metadata8/doctest4/affected command25/strict lint/fmt pass0. host()->&H now non-const; cross-thread H:Send+Sync, single-thread supported and actual DesignSources Send+Sync compiles.
Promotion:indexed command decision; whole0471 scope/lesson/changelog retained626. Book/source/privacy checks scoped; coupled .4.4/deeper/full proof pending. README correct unchanged; LIVE closes D293 note.
[Whole0471 history/D293 diagnosis lesson](docs/history/stitchcad-devnotes-part626.md) retains exact before-fix reproduction.
## STITCHCAD-G1-0472 — freeze actual immutable source ownership (P0 D293)

- Share the actual source privately across snapshots; copy captured entity vectors separately and preserve complete logical resource charges.
- Honest semantic-equal cache-growth regression2 fails before/passes after; exact906B and old907B publication/current checks pass,905B retains original880+26 refusal.
- Affected command25/metadata8/doctest4/strict lint/fmt pass. Foreign Clone telemetry is0, independently controlled; it no longer measures core model copying.
- host()->&H becomes non-const; cross-thread sources require Send+Sync, single-thread supported. Whole0471 retained626; coupled/deeper/full resource acceptance remains pending.

## D293 — generic source Clone changed actual charged cache footprint
P0 fixed0472, owner G1-SLICE.6.8.2.2.2.2.4.3.4.3; whole diagnosis0471 retained626.
Native regression2 failed before/2 passed after rc=0:actual source snapshots share H; full logical charges remain; exact906B/current check pass,905B refuses880+26. Explicit foreign Clone still grows its own cache.
Repair:private Arc<H>, separate census copies; host()->&H becomes ordinary non-const; cross-thread H:Send+Sync, single-thread legal. Coupled .4.4/deeper/full resource proof pending.
