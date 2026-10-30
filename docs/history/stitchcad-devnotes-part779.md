# Sealed engineering continuity — part779

- **Identity:** 99 lines, 10327 bytes, sha256:de02d19719c97cf1ea62d0d6c25aa13ae0930f454541eb8c621956919f51655f
- **Coverage:** whole0619 task scope, lesson and changelog from e1f0aa0f37db6bf4fb458b4090bcbea6c10e192e.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 8128–12147, 4019 bytes, sha256:047700ce938dab1206bbff1ea71a707773be6eb3c3fdc3b1471721759569c935
- Whole source interval: DEV_NOTES.md bytes 193–1510, 1317 bytes, sha256:326127eefea9c7a5a8025747efddf19207234f2775d0c468f7a9bc08ef4beb01
- Whole source interval: CHANGELOG.md bytes 207–856, 649 bytes, sha256:9daab39b403ee27938fa1d448b1e53a0278225759cd22ebf237637020f3b0c53
- Whole source interval: docs/tasks/PLANNING.md bytes 90576–94918, 4342 bytes, sha256:34dc055d3bf5cc5cfe54466a0a9afa996b383b2c2acd6c1e8fcb7491f066bfa0
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.5e.2.4.integration.tangent.expression`
  Status: `done`
  Goal:actual standalone expression policy-aware preparation/native Tan execution with original
  proof/input/context/error/read/flag contracts. No supplied AST values or new language code.
  - [x] REPRODUCE / ISSUE:native tangent0618 existed but actual expression preparation rejected
    every Tan and retained no explicit native policy. Runtime policy must also survive failures
    before Tan;missing configuration cannot become a fabricated numerical-domain refusal.
  - [x] ROOT CAUSE (WHY + WHERE): git show HEAD:crates/sc-core/src/recipe/execution/plan.rs
    →no explicit tangent expression factory/selected policy before0619,rc=0;current factory and
    field mechanically compared with HEAD. cargo clippy -p sc-core --lib --test
    formula_tangent_expression_contract --test formula_scalar_execution_plan_contract -- -D warnings
    →new shared native failure variant>=368B versus112B before allowances;keep full original
    cause inline until existing outer witness box,not another native diagnostic allocation.
  - [x] FIX:explicit per-call policy factory/common complete capability-before-source path;
    legacy MissingTangentPolicy retains actual node before source allocation/joins/values.
    Native sourced Tan dispatch uses real child values;selected policy survives every plan/result/
    prep/runtime error,including allocation or earlier unrelated failure. Exact/lazy flags and
    mathematical/resource causes stay original. Native budgets reset per actual call,not globally.
    Existing engine error witness remains outside logical quotas;four measured narrow lint
    allowances preserve original inline failures and the existing single outer error box.
  - [x] ADDRESSED (verified): cargo test -p sc-core --test formula_tangent_expression_contract
    →eight actual AST tests pass,rc=0:root/inventory identity,complete missing-policy precedence,
    selected-only zero-budget branches,per-call budgets,real Cos/Tan inputs,false conditions,
    T1/T2,original pole/first/later quotas,exact148-bit width/full128-bit literal Count division,
    original native chain and unrelated error policy. Initial test used a nonexistent normalized
    source getter;compiler identified it,retaining original source string fixes test ownership.
  - [x] NO REGRESSION: cargo test -p sc-core --test formula_scalar_execution_plan_contract --test
    formula_scalar_execution_contract →5+11 pass,including49-row reference/frozen regeneration,
    rc=0. cargo test -p sc-core --lib recipe::execution:: →nine private actual-context/poison/
    suspension/storage guards pass,rc=0. cargo test -p sc-core --doc recipe::execution::plan::
    →four positive/privacy docs pass,rc=0. Scoped strict Clippy/selected rustfmt pass,rc=0.
    cargo check -p sc-core --test formula_scalar_recipe_execution_contract --test
    formula_scalar_source_graph_execution_contract --test design_structure_contract →three affected
    targets compile,rc=0. Compiler caught three test-compatibility alias edits matched as suffixes;
    lexical full-alias correction restores all original fixture scopes,without invented error code.
    Positive book98/374/1648/2912/glossary310/9/158/0/history778 four exact Git spans/prior IDs/order/
    map/diff pass,rc=0. No broader-family runtime/full CI/platform verification inferred.
  - [x] LOCKSTEP:README/book/native-status facts/decision/task/index/MEM/DEV/CHANGE/LIVE align;
    whole0618/native subtree retained778;new integration children own sources/recipes/Design and
    independent paired acceptance. Next actual source graph policy adoption;legacy graph/recipe/
    Design scopes stay unchanged until their owners. LIVE standalone policies,G1 6/18/G0 unapproved.
    promotion: promoted (decision_tangent-resources.md gains actual expression policy retention/per-call scope answer).
  Commit:STITCHCAD-G1-0619;Verification:new8/existing16/private9/doc4/scoped lint/three consumer type checks.
## _(2026-10-30 UTC)_ — execute actual scalar expressions with explicit tangent policies

0619 policy-aware preparation selects explicit per-call native limits before complete capability/source admission. Legacy MissingTangentPolicy stays a setup refusal,not a language code. Actual sourced Tan uses completed children;plans/results/all prep/runtime failures retain policy,including earlier causes.
Exact zero/unit and selected-only branches preserve call/condition flags and T1/T2. Full128-bit Angle literal/Count arithmetic reaches actual near-midpoint/pole-width inputs. Native quotas reset per executed call;aggregate heap/CPU and existing boxed witnesses remain outside those logical limits.
New8/existing16/private9/doc4/scoped lint/fmt/three consumer type checks pass. Compiler caught normalized-source test getter and suffix-matched compatibility aliases;original sources/full enum aliases fix them. Measured shared failure>=368B stays inline under four narrow allows until existing outer box.
Book98/374/1648/2912/glossary0/history778 four spans/IDs/map/diff pass. [Whole0618](docs/history/stitchcad-devnotes-part778.md) retained. Next source graphs,then recipe/Design/paired acceptance. LIVE standalone policies,G1 6/18/G0 unapproved unchanged. Full PNT/push400/proofs retained;no full CI/platform claim.
## STITCHCAD-G1-0619 — execute actual scalar Tan expressions under explicit policies

- Add policy-aware preparation and real sourced Tan dispatch. Legacy missing policy precedes source admission;all plans/results/prep/runtime failures retain selected limits. Lazy branches,per-call budgets,full-width inputs,original typed causes and call flags remain exact.
- New8/existing16/private9/doc4/scoped lint/fmt/three consumer type checks pass. Book98/374/1648/2912/glossary0/history778/IDs/map/diff pass. LIVE standalone expression policies,G1 6/18/G0 unapproved unchanged. Next source graph adoption;recipe/Design/paired/full verification pending.

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
