# Sealed engineering continuity — part626

- **Identity:** 32 lines, 3515 bytes, sha256:af089e3ffb757737a5ca5eac152e1c698b086fe943876638272c8815c8266639
- **Coverage:** whole0471 history/D293 diagnosis scope, lesson and changelog from a8eb3250a4ea695444b53d9ded760a0ef78c8488.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 52287–54093, 1806 bytes, sha256:40dda78485006fa6256f946514959b55111c299c83edd4ed75a9115394499a85
- Whole source interval: DEV_NOTES.md bytes 193–1206, 1013 bytes, sha256:b9dff49b9312591494c6aecb358e2d54816ffece60c668d3608160cdbc543f29
- Whole source interval: CHANGELOG.md bytes 207–903, 696 bytes, sha256:6106fb6d2a2beed3ab2e5992241608cb1ac5418b7cb17028f8a4445f265d7b3d
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.8.2.2.2.2.4.3.4.2`
  Status: `done`
  Goal: exact history retained projection before counter/request/command copies or stack/action growth;
  preserve actual model moves, current operator/original author, replay, fresh authority and cancellation.
  Own scoped D293 source-clone footprint diagnosis here; its P0 repair .3.4.3 precedes feature acceptance.
  - [x] REPRODUCE / ISSUE: python3 -I -B native source/order confirms prior history had no prospective
    check before publication construction; source_clone_footprint1 reproduces D293907/908B rc=0, open.
  - [x] ROOT CAUSE: native source/order rc=0:actual target is validated before projection; inline H
    derived Clone can alter charged cache despite semantic equality; D293 .3.4.3 owns repair.
  - [x] FIX: exact borrowed target/request/accepted projection retains max-issued, moves previous model,
    omits actual source top, forecasts only sequence/action; no counter/model/record allocation in walk.
  - [x] ADDRESSED: cargo test -p sc-measure --test design_inputs_contract command_bus::retained_history::
    →4 passed rc=0;Undo2M+19/2B+84/10,Redo3M+28/3B+352/36,max1797bits/action642;short/cancel/revoke/replay.
  - [x] NO REGRESSION: current4/resource-admission4 and cargo clippy core lib/measure contract -D warnings,
    cargo fmt -p sc-core -p sc-measure --check pass rc=0; native original producers unchanged rc=0.
  - [x] LOCKSTEP: native book publication85 chapters/159 API rows/1532 source/2582 rendered links rc=0;
    MEMORY/DEV/CHANGELOG/decision/index updated,LIVE adds D293;README correct unchanged;coupled/full pending.
  Verification: history4/current4/resource-admission4/D293 diagnostic1/lint/fmt/book/source/retention rc=0.
  Commit: STITCHCAD-G1-0471; next P0 D293 .3.4.3, parent remains in_progress.
## _(2026-10-28 UTC)_ — exact prospective retained history publication

0471 .6.8.2.2.2.2.4.3.4.2 borrows actual validated history target/request/accepted record; admits exact model moves, unchanged maximum-issued and next sequence/action before publication construction.
History4/current4/resource-admission4/strict lint/fmt pass. Real oracles:Undo2M+19/2B+84/10;unequal Redo3M+28/3B+352/36;all-one maximum1797bits/action642. Quota/cancel/revoke preserve state/keys; replay skips private walks.
P0 D293 plain real-source/cache diagnostic reproduces semantic-equal Clone cache1→2→3:forecast907B accepted but actual908B, then current refuses882+26. This passing diagnostic proves an open defect; frozen-source repair .3.4.3 precedes coupled .4.4/full proof.
Promotion:indexed command decision distinguishes projection correctness from generic Clone footprint. Whole0470 retained624; closed D292/G1.1 retained625; retention1079 records/2400192 residentB passes0. README unchanged; LIVE defect note adds D293.
## STITCHCAD-G1-0471 — admit exact prospective retained history publication

- Project actual Undo/Redo model moves, original author/current operator, aliases, unchanged maximum-issued and next action before publication copies or allocation.
- History4/current4/resource-admission4/strict lint/fmt pass against real oracles, one-short limits, cancellation, replay and late revocation.
- Plain real-source/cache diagnostic reproduces P0 D293:forecast907B accepted but actual908B; frozen-source repair is next, before coupled/full resource acceptance.
- Whole0470 and closed D292/G1.1 retained624/625; book documents exact history scope and prominently identifies the open D293 signoff blocker.

