# Sealed engineering continuity — part581

- **Identity:** 79 lines, 6583 bytes, sha256:c617daa2a541494dc3df67e5f2616e94acb749fafdbf8af6c865556a9bf38acc
- **Coverage:** whole0428 group-schema and0429 D284 tasks/lessons/ledgers plus D283 correction from d9037780c08d56ca898b68c602bad56cb19d1d5a.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 42580–44318, 1738 bytes, sha256:a935575dfa4653696b762f58bd37d2ab359e4fed05ba59e7ce9c0663ac71791c
- Whole source interval: docs/tasks/G1-SLICE.md bytes 44564–46399, 1835 bytes, sha256:109a845c74757f874a050210727b967f6ab0c1c14aabd9af210164b720ab2597
- Whole source interval: DEV_NOTES.md bytes 193–1122, 929 bytes, sha256:00ed5d6e1284b408fa07a7036e85cecf67a13298676cef4131f6974ebca5bc47
- Whole source interval: CHANGELOG.md bytes 207–1011, 804 bytes, sha256:d0c6450ca8fabc1c73cd4be0524ecde00bac1e4aff38f7b767f83bbd5911261b
- Whole source interval: docs/tasks/PLANNING.md bytes 246–1523, 1277 bytes, sha256:25601264301334c9dadfc3788771b940620c89f8cb6336be627e7f5b1a4c98a7
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.4.2.1`
  Status: `done`
  Goal:0428 exact nonempty typed mutation-group inputs/common claims, no authority or execution.
  - [x] REPRODUCE / ISSUE: git show 12b23f6:crates/sc-core/src/command/context.rs prints optional
    membership per standalone context without a validated group owner, rc=0.
  - [x] ROOT CAUSE (WHY + WHERE): cargo test -p sc-core --test command_group_contract:4 passed rc=0;
    explicit original membership/order and full common actor/Design/revision must remain consistent.
  - [x] FIX: cargo clippy -p sc-core --test command_group_contract -- -D warnings:finished rc=0;
    immutable group/common snapshot, per-command checks in input order, exact original error fields.
  - [x] ADDRESSED: cargo test -p sc-core --test command_group_contract:4 passed rc=0; no mutation/key
    sorting/dedup, empty/None/wrong membership, mixed Design/actor/revision and first invalid index checked.
  - [x] NO REGRESSION: cargo test -p sc-core --doc command::group:1 passed rc=0; selected fmt --check/
    strict Clippy rc=0. All typed mutation payloads remain representable; availability/replay separate.
  - [x] LOCKSTEP: whole0427 private preparation scope/lesson/ledger retained580; book/pointer/decision
    updated. transition_receipts.py record580:3 exact Git spans rc=0; retention1034 records/64 working OK rc=0.
    Native publication80 chapters/159 scoped API rows/1463 source links/2442 rendered links rc=0;
    maintained-reference81 files/17549 lines/1224998 bytes rc=0. Full bus proof remains .6.9.
  Cargo commands use scripts/local_environment.py. Consistency confers no authenticated actor,
  current model/revision, grant, lookup, replay result, preview or execution. .4.2.2 staging follows.
- ID: `G1-SLICE.6.5.4.retention1`
  Status: `done`
  Goal:0429 D284 scope diagnosis/correction; original planned packing was unnecessary. Restore
  group staging .4.2.2, with original archive bytes and fixed limits unchanged.
  - [x] REPRODUCE / ISSUE: bash scripts/history_archive.sh verify-retention prints64 working Markdown
    rc=0; incorrect0427/0428 resume instructions treated that aggregate as the active group's64 bound.
  - [x] ROOT CAUSE (WHY + WHERE): read-only history_generations.describe/decode_group: generation1
    working21/decoded3984766B and generation2 working43/decoded303365B, rc=0; inventory sums to64.
    Each generation is bounded64; the aggregate192. Public source/decision already enforce this.
  - [x] FIX: git diff --quiet HEAD -- docs/history .doctrine/history_archive rc=0; cancel false
    dependency, correct pointer/reader guide/decision/defect ledger. No authoring/installer invoked.
  - [x] ADDRESSED: independent rg --files docs/history -g 'stitchcad-*-part*.md' census43 equals actual
    active raw set, rc=0. Original logical1034/global64 remains correct; pressure inference was wrong.
  - [x] NO REGRESSION: bash scripts/history_archive.sh verify-retention:1034 records/4288131 decoded
    bytes OK rc=0; git diff --check rc=0; per-generation/index caps and every archive byte unchanged.
  - [x] LOCKSTEP: MEMORY/TASK_TREE/book annex/history decision/PLANNING D284/DEV_NOTES/CHANGELOG
    aligned; native publication80 chapters/159 scoped API rows/1463 source links/2442 rendered links
    rc=0; maintained-reference81 files/17554 lines/1225404 bytes rc=0. No retirement record or data change.
  D284 P2 is owned/fixed here. Global working64 comprises21 sealed catalogs and43 active raw records;
  compare actual group pressure before future capture. Do not change existing finite budgets.
D284 corrected0429: v3 aggregate64 is21 sealed catalogs+43 active records; bounds are64/group and192/aggregate.
No packing was performed or needed. Native decode/independent rg agree; group staging is the frontier.

## _(2026-10-27 UTC)_ — exact typed mutation-group metadata

0428 .6.5.4.2.1 owns a nonempty ordered mutation group, requiring explicit group membership and
common exact Design/actor/revision. No standalone rewrite, sorting, deduplication or key inference.
Errors retain first original index/command/expected/requested data; full Actor roles and beyond128
revision values survive. Native group4/privacy1/strict lint/fmt pass. Input consistency confers no
current model, grant, replay or execution. Promotion: command-layer decision; real staging follows.

[Whole0427 private-preparation task/lesson/ledger](docs/history/stitchcad-devnotes-part580.md) retains exact source proofs and prior correction pointers.

## STITCHCAD-G1-0429 — correct archive working-count scope

- D284: aggregate64 combines21 sealed catalogs/43 active records; per-group64 and aggregate192 remain fixed.
- Native decode/independent raw-file census agree; no packing or archive data change was needed.
- Correct resume/frontier and reader guidance; sequential private group staging is next.

## STITCHCAD-G1-0428 — own exact typed mutation-group metadata

- Nonempty ordered original mutations require explicit common group/Design/actor/revision claims.
- Preserve exact payloads/retry keys and original error index/context, including beyond128 revisions.
- Focused group4/privacy1/lint/fmt; no sorting, deduplication, implicit membership or authority.
- Sequential private staging, replay and group execution remain separately owned.

## D284 — aggregate archive count misread as active pressure
P2, fixed0429 by G1-SLICE.6.5.4.retention1: total64 is21 sealed+43 active; bounds64/group and192/aggregate.
- [x] ROOT CAUSE: native generation decode + independent rg census show21/43 and raw43 rc=0; v3 sums counters.
- [x] ADDRESSED: restore actual group staging frontier; no packing/data rewrite or cap change; git archive diff empty rc=0.
- [x] NO REGRESSION: public verify-retention1034 records validates all generations; named source/count checks rc=0.

## D283 — publication count labels
P2, fixed0426 by G1-SLICE.6.5.3.1.2: manual tuple labels confused scoped API rows/source links with tables/rows.
- [x] ROOT CAUSE: native411f74d formatter oracle agrees with named check summary rc=0; original labels were correct.
- [x] ADDRESSED: book_publication.py check().summary():80 chapters/159 scoped API rows/1462 source links/2439 rendered links rc=0.
- [x] NO REGRESSION: tuple compatibility/native original formatter equality rc=0; read-only archive census1031 records/3 affected rc=0.
Sealed575/576/577 remain exact; their159 means scoped API rows,1461 source links,2436/2437 rendered links.
Live0422 labels corrected; native producer now supplies named results/summary. Current task records full scope.

