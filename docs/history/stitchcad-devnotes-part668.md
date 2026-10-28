# Sealed engineering continuity — part668

- **Identity:** 52 lines, 6009 bytes, sha256:df543277b27ce1411b7c991f7740cef6a9a6964cc91cc040a6c9a18f884527e2
- **Coverage:** whole0510 task scope, lesson and changelog from 463838f40ac76b7f3fdb282835428213ea850284.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 15876–19740, 3864 bytes, sha256:808675758a811e45310aff40b5fd8a9e2769434e029b4d5fc0236c3a06253674
- Whole source interval: DEV_NOTES.md bytes 193–1473, 1280 bytes, sha256:e0298c742b6a33d9426031376b35b031b45d84e8ba9967cec1e4feeb9dcebe17
- Whole source interval: CHANGELOG.md bytes 207–1072, 865 bytes, sha256:7da2aaefb488c0c4cef728a649b18ab1613651cc717018d143e0c5327abb9b0c
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.3.2.3.2.4.3.3.1`
  Status: `done`
  Goal:0510 bounded complete positive-interval coverage without endpoint choice trees;shared exact
  per-edit mapping with legacy range resolution. Complete input and peak portion limits precede
  derived collection growth;held/source/edit/numeric causes,order,repairs and direction survive.
  - [x] REPRODUCE / ISSUE: actual source validation needs whole intervals,not point uniqueness.
    rg -n 'ledger.resolve_range|piece.range_resolutions|start: self.resolve|end: self.resolve'
    crates/sc-core/src/ontology/range.rs crates/sc-core/src/ontology/sewing.rs →5 actual callsites,
    including source side and Piece ownership queries and both legacy endpoint folds,rc=0.
    Dependency owned here;legacy collection meter remains correct within its documented scope.
  - [x] ROOT CAUSE: 0510 cargo test -p sc-core --test range_coverage_contract
    reversed_split_keeps_original_first_failure_precedence →draft failed101:direct reversed emission
    evaluated second affine before first division. Independent Fraction control confirms both branches
    overflow for symmetric held1/MAX..(MAX-1)/MAX at2/5. Historical mapping computes first branch
    before reversing output. Preserve that order and per-input all-or-repair semantics.
  - [x] FIX: stream shared fixed-size mapping in original arithmetic order;reverse only successful
    input emissions with mutable iterators,no indexing or extra collection. Borrow actual journal;
    admit all entries/nested fields before source lookup. Charge pending+next+active+one requested
    output before append;truncate failed input emissions before retaining actual whole repair.
    RangeCoverage/limits/fixed boxed refusal are public;PointJournalAdmissionIssue restricts shared
    preflight to actual population failures. Initial Clippy rejected256B mapped outcome/error and
    unchecked suffix indexing;precise input issue plus iterator reversal fix both without waivers.
  - [x] ADDRESSED: 0510 python3 -I -B scripts/local_environment.py -- cargo test -p sc-core --test
    range_coverage_contract --test range_contract --test unique_point_contract →6+20+6 passed,rc=0.
    Exact quota boundaries/complete unrelated inputs/unknown source/reversed gaps/deletion/numeric
    rollback/first-cause precedence pass. Eight cycles:upper interval1 under peak2,whole2 under3;
    legacy lower endpoint256 leaves unchanged. No partial coverage or mutated ledger. All14 frozen
    Fraction rows feed new coverage;reference_recomposition_oracle.py --check →14/9 valid/5 genuine
    width refusals/reversed dual-failure control1,rc=0. No input/CPU/RSS/global-authority conflation.
  - [x] NO REGRESSION: 0510 python3 -I -B scripts/local_environment.py -- cargo test -p sc-core
    --test sewing_contract notch_context_recovery →4 passed,rc=0;actual original source coverage
    joins actual Notch/Graph projections without a fake provider. cargo test -p sc-core --doc
    RangeCoverageLimits →1 passed,rc=0. Scoped Clippy four affected targets/fmt --all --check →pass0.
    Book build/publication →89/195/1567/2688,rc=0;glossary →0 failures,rc=0;receipt667 →3 exact Git
    spans,rc=0;retention →1121 logical/26 working/2395133 residentB,rc=0. Broader paired/mutation/
    physical and bus verification remain .6.5.3.2.acceptance/pre-push after feature implementation.
  - [x] LOCKSTEP: current task/next children/MEMORY/DEV/CHANGE/book/API routes/D301/task index align.
    Whole0509 retained667. LIVE/README reviewed unchanged,G1 6/18. Next .3.2 actual complete source
    dependency admission;whole source/Piece ownership/endpoint/currentness and quantities remain owned.
    promotion: declined (implementation of already indexed explicit input/derived-work admission and exact source/reference contracts;no new policy or roadmap direction).
## _(2026-10-28 UTC)_ — bounded complete interval coverage

0510 streams shared exact range mapping without endpoint choice trees. Explicit complete journal/nested and peak portion budgets guard each append;unknown/deleted/gapped/numeric repairs retain held/edit/cause/order/direction. Peak counts queues+active+output;whole8-cycle2 portions needs3 slots.
Draft reverse traversal changed first arithmetic failure;targeted native failed101. Original forward arithmetic plus iterator-only successful output reversal fixes precedence and preserves per-input rollback. Precise PointJournalAdmissionIssue fixes initial256B Clippy error;no lint waiver or indexing.
Native coverage6/range20/current6/actual Notch4/doc1/scoped Clippy/fmt pass0. Fraction14 plus dual-failure control pass0. Upper8-cycle interval fits2 slots;old endpoint256 leaves stays. Actual Notch/Graph source coverage joins original point evidence;whole source/ownership/currentness remains next.
[Whole0509 acceptance](docs/history/stitchcad-devnotes-part667.md) retains3 exact Git spans. Book89/195/1567/2688,glossary0,retention1121 pass. LIVE/README unchanged,G1 6/18;actual source dependency admission .3.2 next;context/quantities/paired acceptance remain owned. Promotion declined:existing admission contracts.
## STITCHCAD-G1-0510 — bound complete interval coverage before source context

- Shared streamed range mapping preserves exact positive intervals/repairs/original numeric precedence without endpoint choice trees. Explicit complete input and peak portion admission;fixed actual held/operation refusal,precise PointJournalAdmissionIssue;no partial quota result.
- Native coverage6/range20/current6/actual Notch4/doc1/scoped Clippy/fmt and Fraction14+dual-failure control pass. Eight cycles:whole2 portions/peak3,upper1/peak2;old endpoint256 leaves persists. Draft reverse-order defect reproduced/fixed before commit.
- Whole0509 retained667/three exact Git spans;book89/195/1567/2688/glossary0/retention1121 pass. LIVE/README unchanged,G1 6/18. Actual complete source dependency admission .3.2 next;context/quantities/paired effects/full acceptance retain owners.

