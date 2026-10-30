# Sealed engineering continuity — part776

- **Identity:** 90 lines, 9545 bytes, sha256:c1d16b24c2bb169d82937a9d013a211519196658d2c3ae14473ec5c3aa96a277
- **Coverage:** whole0616 task scope, lesson and changelog from 8516048f86a40e8016e437a4f03871d5b6585f05.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 31281–34651, 3370 bytes, sha256:12cd72bd456a0d2549cf019872ccbd8b6e965e5b0f6be155d1ee12441ab498c0
- Whole source interval: DEV_NOTES.md bytes 193–1408, 1215 bytes, sha256:cedc2a887c0f0e7e4659b91e33b9a088c57d8c2fae6506f36210498af48a4f09
- Whole source interval: CHANGELOG.md bytes 207–825, 618 bytes, sha256:548a228b99ac835c05eb6bcf9370cd26d7339feed0a35be1df953c9d81e27967
- Whole source interval: docs/tasks/PLANNING.md bytes 90576–94918, 4342 bytes, sha256:34dc055d3bf5cc5cfe54466a0a9afa996b383b2c2acd6c1e8fcb7491f066bfa0
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.5d.3.2.2.2`
  Status: `done`
  Goal:bounded native unrounded pi/sine/cosine/quotient worker and certified value execution.
  - [x] REPRODUCE / ISSUE:preparation0615 admitted no numerical value. Existing rounded sin/cos
    cannot implement tangent;wide-result model classification also supplies no exact native bit count.
  - [x] ROOT CAUSE (WHY + WHERE): python3 -I -B -c "import runpy;m=runpy.run_path('docs/tasks/artifacts/formula_structure/tangent_reduction_plan.py');a=m['A'];d=m['U']//a;print([(b,[m['round_positive'](v).bit_length() for v in m['enclose'](a*d-1,d,b).ppm]) for b in (128,256)])"
    →[(128,[142,148]),(256,[148,148])],rc=0. Returning a guessed width at128 would violate
    FormulaRationalError's exact measured_bits contract. Retain whole-interval certification.
  - [x] FIX:fresh directed Machin pi and both unrounded components at B128/doubling;exact fold
    chooses direct/reciprocal quotient and proved positive denominator floor. Preflight complete
    integer demand/native counters/factors/work before each allocation/charge;no precision ceiling.
    Whole rounded endpoints alone accept a value;whole equal excessive bit counts certify width.
    Results/errors retain original plan/input/policy/progress;exact endpoints need no series.
  - [x] ADDRESSED (verified): cargo test -p sc-core --test formula_tangent_value_contract
    →five exact/signed/reciprocal/tiny/fullwidth/refinement/late-resource/bit-count tests pass,rc=0.
    Exact square inequalities independently bound tan30;original inverse/alternate-pi control
    owns near-midpoint zero. Small budget refuses next4122 after512;larger certifies148-bit width
    after1536 at256. Actual near-midpoint limits8219/3584 certify zero at512;smaller bit/work
    quotas preserve next8219/2048 demand,prior4122/1536 charges and bit-first priority. No value
    on refusal;successful values agree under larger policies and preserve original signed input.
  - [x] NO REGRESSION: cargo test -p sc-core --test formula_tangent_prepare_contract →four pass,
    rc=0. cargo test -p sc-core --lib recipe::tangent:: →two pass,including873 independent signed
    folds and oversized native extent refusal before counter/allocation,rc=0. cargo clippy -p
    sc-core --lib --test formula_tangent_value_contract -- -D warnings / selected rustfmt --check
    / cargo test -p sc-core --doc recipe::tangent:: →strict lint/fmt/four docs pass,rc=0.
    python3 -I -B docs/tasks/artifacts/formula_structure/tangent_budget_plan.py →19 exact controls/
    three guard-body reds pass,rc=0. Positive book98/372/1645/2906,glossary310/9/158/0,census345/33905,
    history775 four Git spans,all prior IDs/order/map/diff pass,rc=0. No full CI/platform claim.
  - [x] LOCKSTEP:book/API map/native-status facts/index/decision/task/MEM/DEV/CHANGE/LIVE align;
    whole0615 retained775. Native .3 source contributions and .4 independent combined acceptance
    precede real .5e.2.4.integration.tangent policy adoption. LIVE bounded values,G1 6/18/G0
    unapproved unchanged. Full heap/CPU/provider/Design/platform/physical/release owners remain.
    promotion: promoted (decision_tangent-resources.md gains exact native-width refinement answer and original plan/progress contract).
  Commit:STITCHCAD-G1-0616;Verification:focused checks pass;independent combined feature proof pending.
## _(2026-10-30 UTC)_ — execute bounded certified native tangent values

0616 consumes actual tangent plans with explicit quotas. Fresh directed Machin pi/unrounded sine+cosine and exact direct/reciprocal quotient refine B128/doubling. Whole rounding agreement alone accepts Ratio;complete integer/work/native-factor/counter preflight precedes allocation/charge.
Results/errors retain original input/plan/policy/progress. Exact cases charge no series. Native width needs exact measured_bits:actual witness at128 spans142/148 bits,256 proves148. Refine or return resources instead of guessing width. Logical quotas imply no total allocator/CPU/Design certificate.
Worker5/prep4/private2(873 signed folds)/doc4/scoped lint/fmt pass;model19/3 guard reds. Book98/372/1645/2906/glossary0/census345/history775 four Git spans/IDs/map/diff pass. Width-refinement answer promoted;stale native status summaries corrected. [Whole0615](docs/history/stitchcad-devnotes-part775.md).
Next actual Tan source contributions .3,then combined independent .4 and AST policy adoption. LIVE bounded native values,G1 6/18/G0 unapproved unchanged. No full CI/platform/physical/release approval. Full PNT/push400;original proofs retained.
## STITCHCAD-G1-0616 — execute certified tangent values under explicit resources

- Add fresh directed pi/unrounded component/quotient refinement with complete iteration preflight. Retain original plan/policy/progress for certified Ratio or typed failure. Exact numerator width requires agreed bit count;no guessed diagnostic or uncertain value.
- Worker5/prep4/private2/doc4/scoped lint/fmt/model19+3 pass. Book98/372/1645/2906/glossary0/census345/history775/IDs/map/diff pass. LIVE bounded values,G1 6/18/G0 unapproved unchanged. Next source contributions;combined native/AST/platform acceptance remains pending.

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
