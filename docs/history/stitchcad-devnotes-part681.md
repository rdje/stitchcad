# Sealed engineering continuity — part681

- **Identity:** 54 lines, 5919 bytes, sha256:d61ce60b3dbd991358a524afc4bfb9325298cea2d52beb503f3dff8185646511
- **Coverage:** whole0522 task scope, lesson and changelog from 1accf6be137d8e35914d159ce2fbcd4b440e5ada.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 18959–22554, 3595 bytes, sha256:c53fd8dfe23698d9e71a2e665e7bc27973bef949e0da440dec79be91f51ca5ef
- Whole source interval: DEV_NOTES.md bytes 193–1525, 1332 bytes, sha256:970efa07ac576aac006f98a7679bb9071f0b3367309f3a5502d2745d554dfa29
- Whole source interval: CHANGELOG.md bytes 207–1199, 992 bytes, sha256:87c9e69e806ffbf38a35d8ea894fece1c6e8e700fc940f4082ee3450f58784e0
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.3.2.3.2.4.4.1`
  Status: `done`
  Goal:0522 actual bound source capture admitted before materialization;original request/span,
  full logical root payload and actual retained target intervals preserve provenance.
  - [x] REPRODUCE / ISSUE: 0522 cargo test -p sc-core --test sewing_contract command_ease_capture
    →3 passed/83 filtered,rc=0. retain_ease_source clones a declared original region/stop body;
    command callers need complete pre-clone admission,including original stops absent from children.
  - [x] ROOT CAUSE: 0522 same cargo test -p sc-core --test sewing_contract command_ease_capture
    →3 passed,rc=0. Actual direct source has1 root+2 regions+2 stops=5;retained child302 has empty
    child stops but still full source301 cost5. Existing graph resource meter independently agrees:
    declared graph5 entries;retained graph7 minus2 live spans=5. Arc sharing grants no logical discount.
  - [x] FIX: 0522 binding.admit_ease_capture takes positive no-default per-capture item limit;
    counts1 root,all original weighted regions,then original stops with checked additions. Borrowed
    Copy admission retains actual request/span/count/limits;exhaustive definition/source/portion
    views and fixed-field/slice guards prevent hidden owning fields. Explicit capture then clones
    declared metadata once or shares actual retained root with target intervals. No index/current
    queries/quantities/physical/effect capability. Focused native3 and doc4+1 →passed,rc=0.
  - [x] ADDRESSED: 0522 cargo test -p sc-core --test sewing_contract command_ease_capture
    →3 passed/83 filtered,rc=0. Direct signed/formula/profile sources keep exact body/stops/request/
    span pointers and state. Limit2 refuses regions consumed1/requested2;limit4 refuses original
    stops consumed3/requested2;exact5 captures independent owned direct region vector. Child302
    exact5 shares canonical source301 and keeps narrower target intervals;limit4 retains both IDs
    and original stop cost. Damaged held Uniform/profile source captures at minimum1 without
    resolving or repairing deletion. Existing graph meter agrees with all admitted payload costs.
  - [x] NO REGRESSION: 0522 cargo clippy -p sc-core --test sewing_contract -- -D warnings and
    cargo fmt --all --check →pass,rc=0;cargo test -p sc-core --doc CommandEaseCapture →4 privacy/
    lifetime/no-default compile-fail passed,rc=0;--doc admit_ease_capture →1 API example passed,rc=0.
    Native tools use local_environment.py. Book/publication91/216/1577/2719,glossary310/9/158/0,
    map unchanged,680 receipt3 exact Git spans,retention1134 logical/39 working/63629 decodedL/
    4917768 decodedB/2495105 residentB →pass,rc=0. Weighted/notch caller quantities,aggregate command
    accounting,actual source/value/physical authority and paired/full proof retain .2/.3/.4/
    .acceptance/.6.5.3.2.acceptance/pre-push;no later gate claimed passed.
  - [x] LOCKSTEP: 0522 five public types/API routes/new bounded capture chapter/SUMMARY/topic
    navigation/MEMORY/DEV/CHANGE/source decision/task index/D301 align. Whole0521 retained680/3
    exact Git spans. LIVE/README reviewed unchanged,G1 6/18. Next .4.4.2 exact weighted command
    target/projected-pair quantities from admitted capture;capture alone supplies no numeric or
    current/physical/authority certificate. Per-capture limits are not global allocator/CPU quotas.
    promotion: declined (mechanical implementation of indexed original-source capture/resource contracts;no new policy,implicit value or physical assumption).
## _(2026-10-28 UTC)_ — admit actual bound source capture before cloning

0522 CommandEaseCaptureLimits/Admission/Capture count1 original root+all original regions/stops before materialization. Actual request/span borrows,checked quota evidence,exhaustive/fixed Copy views preserve full payload. Declared capture clones once;retained capture shares original root and keeps target intervals.
Native3/scoped Clippy/fmt/doc4+1 pass0. Direct source5,retained child empty stops still5;existing graph meter independently agrees5 or7−2 spans. Limits2/4 preserve precise region/stop refusal;exact5 captures. Signed/formula/profile metadata and damaged held source remain exact;minimum1 does no current repair/query.
Book91/216/1577/2719,glossary0/map/retention1134/680 receipt3 exact spans pass0. New bounded capture chapter/routes/SUMMARY/topic navigation cover5 public types. Source decision199L stays bounded. Weighted command quantity views .4.4.2 next;no numeric/current/physical/authority/effect certificate from capture.
[Whole0521 checkpoint](docs/history/stitchcad-devnotes-part680.md) retains exact source/query scope. LIVE/README unchanged,G1 6/18. Per-capture bounds need separate aggregate accounting;consume every preparation terminal before stage. Promotion declined:implementation of indexed capture/resource contracts.
## STITCHCAD-G1-0522 — admit actual command source capture before cloning

- Positive explicit limits count1 logical original root+complete original weighted regions/stops before materialization. Private borrowed admission retains actual request/span;capture clones declared metadata once or shares retained root with target intervals. Precise
  target/source/collection/count/limit evidence survives.
- Native3/scoped Clippy/fmt/doc4+1 pass. Direct5 and retained root5 agree with independent graph resource meter,including empty child stops. Exact/one-below quotas,signed/formula/profile metadata,owned/direct vector and shared root provenance,damaged held references and minimum1
  stay exact.
- Whole0521 retained680/three exact Git spans;book91/216/1577/2719,glossary0/map/retention1134 pass. New capture chapter/API routes;weighted command quantities next. LIVE/README unchanged,G1 6/18;aggregate accounting,current/value/physical/authority/effects/full verification
  remain owned.


