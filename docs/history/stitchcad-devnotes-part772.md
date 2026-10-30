# Sealed engineering continuity — part772

- **Identity:** 86 lines, 9163 bytes, sha256:610129190d5e02a85421e344ca076768881617f63f5e063066b5015d11350384
- **Coverage:** whole0613 task scope, lesson and changelog from 0f385afbacc7e67e966197575e103c1dd0fdda4f.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 30507–33344, 2837 bytes, sha256:81085267cc8c9777074da74140779420e0d8a79d7cb93be5af912c7c7a145d75
- Whole source interval: DEV_NOTES.md bytes 193–1518, 1325 bytes, sha256:d568cc3dc079ebabc2020a8ac4cdf559d90e7b1ee2155be96aae7a1a4236f383
- Whole source interval: CHANGELOG.md bytes 207–866, 659 bytes, sha256:070f1c889b26353c27de2aeb40ecd3364afad5a361ac0e104524a95867444656
- Whole source interval: docs/tasks/PLANNING.md bytes 90576–94918, 4342 bytes, sha256:34dc055d3bf5cc5cfe54466a0a9afa996b383b2c2acd6c1e8fcb7491f066bfa0
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.5d.1.2.2.2.d.2.2.1`
  Status: `done`
  Goal:rigorous unrounded per-precision tangent enclosures/convergence and iteration arithmetic.
  - [x] REPRODUCE / ISSUE:fixed sine/inverse and640-bit pi certificates do not license an arbitrary
    tangent precision or quotient. Need fresh proven enclosures without an invented value/refusal.
  - [x] ROOT CAUSE (WHY + WHERE): rg -n 'def pi_interval|def series_interval|undecided|integer_limit'
    docs/tasks/artifacts/formula_structure/tangent_reduction_plan.py →fresh precision-specific pi,
    unrounded component series,positive-denominator conditioning and certificate-only decision,
    rc=0. NIST4.19/4.24 and Calcut tangent Corollary1 independently reopened;all bounds are our proof.
  - [x] FIX:B>=128 uses fresh directed Machin pi width20(B+1),radian width5B+7,square10B+16,
    term10B+20 and component(10B^2+20B+2)*2^-B. Analytic floors make quotient width tend tozero;
    original midpoint theorem proves eventual separation. At most4B series terms/constant integer
    slots and16B+bit_length(2B+1)+16 candidate operand bits per iteration. Whole endpoints must
    round alike;else undecided. Actual poles/whole-interval width excess remain distinct,no kernel.
  - [x] ADDRESSED (verified): python3 -I -B docs/tasks/artifacts/formula_structure/tangent_reduction_plan.py
    →three independent alternate-identity Fraction pi bounds/42 direct-power factorial component/
    54 coupled exact endpoint,pole,width controls pass,rc=0. Original29928 finite/372 poles/3642
    reciprocal/92 signed boundary witnesses/144 rectangles/two controls retained. Actual legal
    near-midpoint input247385123034593398372138492992328993860/8635369834810561370044731507908276509
    microdegrees is undecided at128/256,certified zero at512;independent atan/pi proves its side.
  - [x] NO REGRESSION: producer_census.py →344 sources/33815 calls/73 heredocs/2892 embedded,
    rc=0. Existing effect/file controls →109/48 and231/89/38,23 actual data refusals/19 guard reds
    pass,rc=0. bash -n run_formula_structure_probes.sh/git diff --check/generated map compare,
    book98/365/1645/2903/glossary310/9/158/0/history771 four Git spans/prior live ids/order/uniqueness
    pass,rc=0. Native APIs unchanged;full CI/probes/latency/platform not inferred.
  - [x] LOCKSTEP:fact/retrieval answer/index/book/task/index/MEM/DEV/CHANGE align. Whole0612
    retained771. Next P1 .d.2.2.2 practical admission before kernel;no universal512-bit claim,
    numerical-domain cutoff,allocator-byte/latency/full-runtime admission. LIVE G1 6/18/G0
    unapproved unchanged;D341 fixed/D156/D223/PLANNING.5/D334 owned. Protected evidence retained.
    promotion: promoted (fact_trig-midpoints-and-resource-bound.md gains convergent unrounded enclosure answer/proof and actual boundary witness).
## _(2026-10-30 UTC)_ — prove convergent unrounded tangent enclosures at each precision

0613 adds fresh directed Machin pi and unrounded sine/cosine intervals. Component width<=(10B^2+20B+2)*2^-B and positive denominator floors imply quotient convergence;the rechecked rational-angle theorem supplies eventual midpoint separation. Decisions require whole-interval agreement or actual width/pole proof.
Independent alternate-pi3/direct-power factorial42/coupled54 controls pass. A legal near-midpoint Angle remains undecided at128/256 bits;512 certifies zero,independent atan/pi confirms. Original reduction/conditioning controls retained. Per-iteration4B terms/constant slots/explicit linear bit extents proved.
Book98/365/1645/2903/glossary0/inventory344/file231/effect109/history771 four spans/map/live ids/diff pass. [Whole0612](docs/history/stitchcad-devnotes-part771.md) retains exact prior proof/scope. Fact gains a convergent-enclosure answer;primary series/theorem rechecked. Native APIs unchanged.
Next P1 .d.2.2.2 must admit practical production resources before kernel. Convergence supplies no uniform512-bit,allocator-byte or latency guarantee;undecided is no returned value/numerical refusal. LIVE G1 6/18/G0 unapproved unchanged;full-hosted79bd6f6a scoped,D341 fixed/other owners retained. Full PNT/push400.
## STITCHCAD-G1-0613 — prove convergent per-precision unrounded tangent enclosures

- Fresh pi/unrounded sine/cosine bounds converge with explicit per-iteration term/integer extents. Whole endpoints must round alike;undecided stays separate from actual pole/width refusals. Independent pi3/component42/coupled54 controls and a genuine near-midpoint witness pass.
- Existing reduction controls/inventory344/file231/effect109/book98/365/1645/2903/glossary0/history771/map pass. Actual128/256-bit undecided input certifies zero at512;no universal precision/resource guarantee. Next P1 production admission before kernel. LIVE G1 6/18/G0 unapproved unchanged.

## D298/D299 — verified glossary/index repair0484

Done;owner G1-SLICE.6.vocabulary.acceptance. [Whole causes/reproductions/repairs](../history/window23.md#stitchcad-devnotes-part639md)
retain obsolete secondary pointer,definition-only use and token/chapter-pair assumptions;focused4/full41 pass0.

## D300 — corrected execution scope0485

Done;owner .6.5.3.2.1. [Whole reproduction/source proof](../history/window23.md#stitchcad-devnotes-part642md) retains the exact original ledger.

## D301 — parent-aware ease conservation before paired effects

P1 paired prerequisite;owner G1-SLICE.6.5.3.2,in_progress before effects. Partition/lineage complete0574:47 native/21 doc scoped checkpoint. Next .5e.2.5.derived.3.3.3.4.2 owned sources. Real providers/physical joins .2.4/trusted atomic effects .2.5 remain pending before whole acceptance. Current repair/authority/runtime proof stays owned;local/hosted79bd6f6a stays SHA-scoped.
Actual native structural_child_graph_acceptance_is_not_parent_ease_conservation passes:copied10+10 gives20 for parent10;independently authored5+5 also passes structural validation with DeferredToG2AndG3. Root:per-span fields/validation lack an original-parent relation. No split result is published;handler unavailable.
Implement coupled parent-aware sources/quantization/provenance for explicit/formula/profile and distributions;standalone structural acceptance is not conservation proof.
Independent Fraction±1 diagnostic:rounded-child orders[0,1,0] versus[1,0,0];root boundaries[0,1,0],signed symmetry. Native .2 root-query watch passes with original domains/direction;nonuniform/context/stops/lineage/effects and full paired acceptance remain pending.

## D302 — aggregate count mistaken for active archive pressure

Done0490;owner G1-SLICE.6.5.3.2.retention. [Whole cause/reproduction/verified fix](../history/window23.md#stitchcad-devnotes-part659md).

## D303 — repeated retained-source interval claims

Done0492;owner G1-SLICE.6.5.3.2.3.2.3.2. [Whole red/fix/verification](../history/window23.md#stitchcad-devnotes-part659md).

## D304 — equal source content retained as multiple actual bodies

Done0492;owner G1-SLICE.6.5.3.2.3.2.3.2. [Whole red/fix/verification](../history/window23.md#stitchcad-devnotes-part659md).

## D305 — premature bounded intermediates in reference recomputation

P1 fixed0503;owner G1-SLICE.6.5.3.2.3.2.4.3.numeric. Independent Fraction14/public point+range native watch first failed1/rc101:nine representable final results falsely repaired. Split1/3 held1537228672809129301/4611686018427387902 now correctly resolves1/9223372036854775804.
Root:bounded differences/products before complete ratios in split-second/offset/merge point+range maps. Reuse exact complete relative progress;share one complete arc-length merge kernel. Reduce before narrowing;no reference width increase or rounding.
Focused range20/notch4 and oracle14 pass0;five genuinely wide final fractions retain original reference/range and actual numeric cause. Topology unit/scoped lint results belong owning leaf;projection/context and deferred broader paired acceptance remain separate.

## D306 — independent seam oracles absent from automated gates

Done0528;owner G1-SLICE.6.5.3.2.oracle-watch. Baseline9-producer fix retained687. Current gate10 producers/12 fixtures/9 finite guards pass;29 suites discovered. New stop oracle registered0529;missing/unknown/status/mismatch/wiring controls retained. Hosted verdict follows push400. Stop partition .3.3 continues.

## D307 — retained test block precedes later production items

Done0536;owner G1-SLICE.6.5.3.2.prepush-400.layout. Full make check at51e421cf rc2 exposed0471 production append after0470 tests. Existing test block moved to EOF;production/test content exact. Actual library-test Clippy/native boundary1/fmt pass0. D308 fixture lint and parent prepush-400 full gates/push/hosted verdict remain pending.

## D308–D341 — runtime continuity,source ownership and producer coverage

D308–D340:[whole prior ledger](../history/stitchcad-devnotes-part769.md). D334 P2 G0-CONTRACT.15f after G1/before G0;D223/PLANNING.5 open. D341 P1 fixed0611:ten source/entry mismatches;owner G1-SLICE.5d.1.2.2.2.d.2.1.inventory. Census343/independent405/effect+file controls pass0;D156 runtime audit separate. Full/hosted79bd6f6a SHA-scoped.
