# Sealed engineering continuity — part673

- **Identity:** 51 lines, 5701 bytes, sha256:bc11c49551dbc35068766b724a006e01a2d9b8b3a588cd575ee5596af8c4534f
- **Coverage:** whole0514 task scope, lesson and changelog from 3e3e061d5b943c77dab5908bf2fcfcf649313e03.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 16918–20387, 3469 bytes, sha256:f295ce23c05215c267c9c9ba285f4f163a700666e6e0a9155a466ce9013a72d8
- Whole source interval: DEV_NOTES.md bytes 193–1491, 1298 bytes, sha256:fdf58a333fdc63f774a275a2ac0f413b10d9f3ceea69b730a408c1c99d350919
- Whole source interval: CHANGELOG.md bytes 207–1141, 934 bytes, sha256:43462547b0beeda13e9217ae0b40cd27be775b76e55dfb3a6e29f07e03d4059b
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.3.2.3.2.4.3.3.3.3`
  Status: `done`
  Goal:0514 guarded private exact original-coordinate/Param ordering under admitted widths;no
  bounded view narrowing,absolute direction filtering or new public API/integer type. Actual .4
  context must still bind ledger/source-creation frames and unique candidates before scalar use.
  - [x] REPRODUCE / ISSUE: valid original coordinates can refuse stored Param/u128 views without
    losing their value. python3 -I -B docs/tasks/artifacts/seam_partition/original_comparison_oracle.py
    --check →6 frozen Fraction cases/3 Param refusals/1 beyond-u128,rc=0. These actual projected
    originals need exact bound/interval ordering,not narrowing or a binary64 substitute.
  - [x] ROOT CAUSE: 0514 cargo test -p sc-core --lib original_point::query::contracts →2 passed,
    rc=0. Test drives6 actual declared/offset/reversed ledgers,real admission/projection and exact
    same source/creation evidence. Cross-products require at most2W;reference fields add63≤W.
    Existing2W+128 reservation covers both. Mapping direction and numerical order remain distinct.
  - [x] FIX: shared private compare-parts/wide/reference helpers admit operands and each product
    before allocation. Reuse in actual inverse merge-side comparison and explicit normalized unit-end
    invariant. Avoid recursive normalization in reference comparison. Workspace retains actual static
    caller operation;all producer construction sites explicit. Private typed zero-denominator/unit/
    integer-quota failures keep concrete values/order signs;no public projection candidate chosen.
  - [x] ADDRESSED: 0514 python3 -I -B scripts/local_environment.py -- cargo test -p sc-core --lib
    original_point::query::contracts →2 passed,rc=0:existing actual wide6-row and new actual comparison
    6-row controls. New frozen Fraction rows include equality/source-bound equality,64/188-bit
    originals,reversed5/8 versus3/8,and optional view refusals. Exact conservative private quota
    accepts;one bit below refuses before product allocation. cargo test -p sc-core --lib
    original_point::query::wide::contracts →1 passed,rc=0:normalization/unit-order and comparison/
    normalization zero-denominator guards. No new source/semantic provider or public scalar order.
  - [x] NO REGRESSION: 0514 cargo test -p sc-core --test original_point_projection_contract →6
    passed,rc=0;cargo test -p sc-core --test sewing_contract notch_context_recovery →4 passed,rc=0.
    Scoped Clippy --lib -D warnings/fmt --all --check →pass0. Original comparison oracle6/width
    oracle6+960 Fraction steps pass0. Receipt672 →3 exact Git spans,rc=0. Book/publication90/204/
    1572/2703,glossary0,map unchanged,retention1126 logical/31 working/2442415 residentB →pass0.
    Broader source/context/paired/mutation/physical proof retains .3.3.acceptance/.6.5.3.2.acceptance.
  - [x] LOCKSTEP: current task/source decision/MEMORY/DEV/CHANGE/book/D301/task index align;
    whole0513 retained672. LIVE/README reviewed unchanged,G1 6/18. Next .4 actual source-bound
    original-notch context and frame/candidate/domain binding. Private numeric success does not
    authorize that pending semantic context,amount evaluation,current Design or physical matching.
    promotion: declined (method-specific private comparison/invariant implementation under indexed exact arithmetic/frame/source contracts;no new public API or general policy).
## _(2026-10-28 UTC)_ — guarded private original-coordinate ordering

0514 shares guarded wide/reference cross-products with inverse merge comparison and explicit normalized unit-end checks. Actual stage operation labels remain concrete;each operand/result is admitted before allocation. Existing2W+128 covers2W products and W+63 reference comparison;no narrowing or public API.
Six frozen Fraction orderings drive actual ledger/admission/projection values,including64/188-bit originals,3 Param refusals/1 beyond-u128,equality/source bounds and reversed5/8 versus3/8. Exact private quota passes;one bit below refuses. Unit and zero-denominator guards retain typed actual causes.
Private2+guard1/public inverse6/actual Notch4/scoped Clippy/fmt/ordering6/width6+960 pass0. Numeric order and mapping direction remain separate. Actual context must bind matching ledger/source frames and unique candidates before using these private scalars;no semantic quantity or physical proof follows.
[Whole0513 validation](docs/history/stitchcad-devnotes-part672.md) retains3 exact Git spans. Book90/204/1572/2703,glossary0/map/retention1126 pass. LIVE/README unchanged,G1 6/18;actual original-notch context .4 next. Promotion declined:method-specific implementation of indexed exact arithmetic/frame contracts.
## STITCHCAD-G1-0514 — compare exact original coordinates without bounded views

- Private guarded wide/reference ordering reuses cross-products in inverse merge and normalized unit-domain checks. Concrete caller operations and actual numeric/quota causes survive;existing2W+128 covers products. No new public ordering/integer type,reference narrowing or
  direction filter.
- Six independent Fraction cases drive actual projection values:64/188-bit originals,3 Param refusals/1 beyond-u128,equality/source bounds/reversed order. Private2+guard1/public inverse6/Notch4/scoped Clippy/fmt/width6+960 pass. Exact quota accepts;one bit below and invalid
  unit/denominator refuse.
- Whole0513 retained672/three exact Git spans;book90/204/1572/2703/glossary0/map/retention1126 pass. LIVE/README unchanged,G1 6/18. Actual frame-bound original-notch context next;quantities/physical matching/paired effects/full acceptance remain owned.


