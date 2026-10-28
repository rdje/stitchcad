# Sealed engineering continuity — part677

- **Identity:** 55 lines, 5972 bytes, sha256:c7b51a8d881bf1fbdbf8a00a5e897813b72ca0bed19e3c480d320f191bee55e2
- **Coverage:** whole0518 task scope, lesson and changelog from 00e1d74ae785cf64458efe6b8ee95fa497a285ef.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 17773–21525, 3752 bytes, sha256:b53b00c327e0d3d2a39f30d0ee7bc6f2ff5513aee0de2fe2ac3ec84edc5745cc
- Whole source interval: DEV_NOTES.md bytes 193–1502, 1309 bytes, sha256:cd5957d113a792999d2b73ea8179c5f9cfd95174aca51d2718a75ae0bd6d1236
- Whole source interval: CHANGELOG.md bytes 207–1118, 911 bytes, sha256:1d78f6e9bad5142d30034eceec574fb68a2793926f84ffbc3b0f690c6d03947d
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.3.2.3.2.4.3.4.2`
  Status: `done`
  Goal:0518 exact original-notch CDF/interval queries implemented with focused checks. Preserve
  full actual source/ledger/Notches,clamping,canonical A orientation and original signed rounding.
  - [x] REPRODUCE / ISSUE: 0518 python3 -I -B docs/tasks/artifacts/seam_partition/
    notch_quantity_oracle.py --check →6 bound rows/302 raw-width controls/380 exact query rows,rc=0.
    Actual64-bit lower1/(2MAX),upper1/4,query1/8 is below half but binary64 gives half. A188-bit
    actual partial-offset pair also has a below-half query;Param/u128 narrowing cannot supply it.
  - [x] ROOT CAUSE: 0518 cargo test -p sc-core --test sewing_contract notch_quantities →2 passed/
    81 filtered,rc=0,all380 frozen rows. Correct CDF is clamped original notch progress,not current
    local parameters or child totals. Complement B/Opposite BEFORE rounding;subtracting rounded
    prefix changes ties. Actual original frames,source domains and named/authored order stay exact.
  - [x] FIX: 0518 separately admitted quantity methods preflight full original edge/both bounds,
    including zero;private semantic projection port guards actual same immutable ledger/source
    creation and unique increasing originals. Guard raw ratio/complement/source multiplication
    under2W+93;shared half-away magnitude helper rounds once. Interval amounts difference original
    rounded boundaries in canonical A order. Concrete standard numeric source and original actual
    frame/bounds/interval evidence survive;no Big type,projection clone or partial amount escapes.
    Focused quantity2/private actual-projection1 →passed,rc=0.
  - [x] ADDRESSED: 0518 cargo test -p sc-core --test sewing_contract notch_quantities →2 passed,
    rc=0. All380 actual ledger/retained source/local validation/context queries match independent
    Fraction outputs:A/B x Same/Opposite x5 signed/zero/maximal literals,clamps/partial domains,
    reversed current frames,64/188-bit originals. Exact interval sums conserve original source;
    both full bounds validate before quantities;zero keeps wrong-edge/interval/outside refusals.
    Initial test1 passed/1 failed,rc=101:interior offset Piece coverage quota retained3/requested1/
    limit3 correctly refused. Fixture now explicitly admits peak4 for3 output portions+active;
    full Piece repairs remain present,no production quota or semantics changed to pass.
  - [x] NO REGRESSION: 0518 cargo test -p sc-core --lib admitted_original_coordinate_ordering
    →1 passed,rc=0:6 actual Fraction projection rows plus quantity complement,actual operation/
    limit1 numeric refusal,other equal-content ledger refusal before numeric work and non-increasing
    bound refusal. Native admission2 still passes;scoped Clippy/fmt →pass,rc=0;cargo test -p sc-core
    --doc NotchEase →4 passed,rc=0. All native tools use local_environment.py. Book/publication
    90/211/1572/2706,glossary310/9/158/0,map unchanged,676 receipt3 Git spans,retention1130 logical/
    35 working/63372 decodedL/4891369 decodedB/2468706 residentB →pass,rc=0. Actual Design quantity
    integration/full paired/mutation/physical verification stay .4.acceptance/.3.acceptance/
    .6.5.3.2.acceptance/pre-push;no deferred verification claimed passed.
  - [x] LOCKSTEP: 0518 MEMORY/DEV/CHANGE/book/API row/canonical decision/task index/D301 align.
    Whole0517 retained676/3 exact Git spans. LIVE/README reviewed unchanged,G1 6/18. Next .3.4.
    acceptance actual Design/retained-source quantities;no symbolic/physical/effect readiness.
    promotion: declined (implementation of indexed exact original interval/resource/rounding contracts;no new policy,implicit value or physical assumption).
## _(2026-10-28 UTC)_ — exact original-notch cumulative and interval queries

0518 clamps actual original notch progress and complements B/Opposite before shared signed rounding;intervals difference original rounded boundaries. Preflight full original edge/both bounds even for zero. Private closed projection port guards common immutable frame/unique ordered originals;no Big API or narrowing.
Native2 consumes380 Fraction rows across A/B,directions,reversed frames,clamps/partial domains,5 signed/zero/maximal literals and64/188-bit originals;all conserve source. Initial interior-offset fixture correctly refused peak3 for3 portions+active;explicit4 passes and preserves surrounding Piece repairs.
Private actual-projection1/6 rows+quota/frame/order controls,admission2,doc4,scoped Clippy/fmt/oracle6+302+380 pass0. Book90/211/1572/2706,glossary0/map/retention1130 and676 receipt3 exact Git spans pass0. Separate2W+93 guards actual integer operands/results;not allocator/runtime/whole Design proof.
[Whole0517 admission](docs/history/stitchcad-devnotes-part676.md) retains exact scope/lesson/changelog. LIVE/README unchanged,G1 6/18. Actual Design quantity integration next;paired/mutation/physical/effects remain owned. Promotion declined:implementation of indexed original interval/rounding contracts.
## STITCHCAD-G1-0518 — query exact original-notch cumulative quantities

- Actual original clamped progress,exact B/Opposite complement before shared signed rounding and original rounded boundary differences. Preflight full original edge/both bounds even for zero;closed private projection port and separate2W+93 guards retain actual
  frame/operation/quota evidence.
- Native2/380 Fraction rows preserve signs/ties/clamps/partial domains/reversed frames and64/188-bit originals;all intervals conserve original source. Private projection1/admission2/doc4/scoped Clippy/fmt/oracle6+302+380 pass. Interior-offset fixture peak4 retains3 Piece portions
  plus active input.
- Whole0517 retained676/three exact Git spans;book90/211/1572/2706,glossary0/map/retention1130 pass. Actual Design quantity integration next;paired/mutation/physical/effect/full verification remains owned. LIVE/README unchanged,G1 6/18.


