# Sealed engineering continuity — part763

- **Identity:** 100 lines, 10467 bytes, sha256:a4fb71e719ab19b5a1257612ece9714b3fa6ace62875db1f7e1619db4d109fe7
- **Coverage:** whole0604 task scope, lesson and changelog from 2faa36144631019f4e93bed2b9494e395c35d83e.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 18975–23160, 4185 bytes, sha256:1345d2e39afdc2f0b1039428b86e8e9243e6e76f64b4725df5f62d66b360d4cf
- Whole source interval: DEV_NOTES.md bytes 193–1487, 1294 bytes, sha256:c18ff3474e1a53f8a06e29f852000f12c61308b922bc1155f07d0df03cea9882
- Whole source interval: CHANGELOG.md bytes 207–873, 666 bytes, sha256:1474e4ca15082b26a00dd0f39ff5ec9dd36cf3c252c48d90f4d85df54ed86da8
- Whole source interval: docs/tasks/PLANNING.md bytes 90576–94898, 4322 bytes, sha256:ae187668c2d92f6a9154ee45120784c598f2b67d28072f4d357b881e1a3a9468
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.5e.2.5.derived.3.3.3.4.4.2`
  Status: `done`
  Goal: actual recipe Derived suspension/read adaptation using only the private earned source
  runtime. Preserve original direct and prior-let witnesses,source/canonical identities and
  standalone RequiresEvaluation behavior. No caller-seeded value/cache or synthetic statement.
  Completion0604:private recipe Derived suspension/earned-read adaptation;no combined public worker.
  - [x] REPRODUCE / ISSUE:recipe reads used only direct canonical/reserved or completed prior-let
    values;actual Derived names returned their original requirement. Shared source runtime0603
    needed an internal read path without inventing a value/cache or changing standalone semantics.
  - [x] ROOT CAUSE (WHY + WHERE):actual RecipeSources/RecipeReadData/worker and source completed
    witness construction inspected. rg -n 'completed_derived_read|read_derived|AwaitDerived|derived:
    None|original_source' on execution/{source_read,source_runtime,recipe_read,recipe_worker,
    recipe_result}.rs →shared earned helper,private yield/ready paths,three original-mode worker
    sites and original-source getter,rc=0. Initial unused Derived witness warning corrected by
    diagnostic Debug reading its actual kind;no dead-code suppression or discarded witness.
  - [x] FIX:central actual completed-source witness factory checks exact canonical owner before
    selecting the sole Derived supplier. Runtime offers readonly earned lookup;empty cache yields
    actual AwaitDerived with unread slot. Recipe adapter records only returned real source root/
    canonical join/supplier proof. Private read-source wrapper preserves original public recipe
    read-kind enum/getter;standalone worker explicitly selects original mode. Original direct/
    reserved/prior-let and class witnesses retain their paths;no new public API/value seed.
  - [x] ADDRESSED (verified): cargo test -p sc-core --lib recipe::execution::recipe_read::tests
    -- --nocapture →one pass/88 filtered,rc=0. Actual original two-statement proof:execute first
    RHS1 mm/3 and store its real let333. Second prior+width+width records actual prior binding,
    suspends unread Derived slot,then resumes after real source9000 produces1000/3. Final unbound
    RHS2999/3,three distinct reads,once source ordinal1 and original canonical RequiresEvaluation
    remain exact. No replay or manufactured prior-let result. Equal-content foreign canonical
    join triggers caught private owner guard before it can retrieve the cached root.
  - [x] NO REGRESSION: cargo test -p sc-core --test formula_scalar_recipe_execution_contract --
    --nocapture →nine pass,rc=0,including18-row/7-refusal independent whole-recipe/frozen-byte
    watchers and original Derived requirement/order. Source graph target →eight pass,rc=0,
    including27-row/8-refusal/frozen-byte watchers and512-source requested64KiB stack. Scoped core
    lib/both affected targets strict Clippy/changed rustfmt/diff →pass,rc=0. cargo test -p sc-core
    --doc recipe::execution::recipe_worker -- --nocapture →one pass,rc=0. Book98/357/1642/2895,
    glossary310/9/158/0,762 four Git spans/prior live ids/order/uniqueness pass,rc=0. Private test
    native checked;no all-unit Clippy/full CI campaign claimed. Full feature verification .4.5.
  - [x] LOCKSTEP:book explains actual suspension/earned witnesses,333 versus1000/3/RHS2999/3,
    original public mode and unfinished combined worker boundary. Task/index/MEM/DEV/CHANGE align;
    LIVE reviewed unchanged:G1 6/18/G0 unapproved. Whole0603 retained762/four spans. Next .4.4.3
    complete source/recipe worker/reservations/once storage,then .4.4.4 actual Design execution/
    .4.5 independent acceptance. No second-let storage/complete integrated result claimed here.
    Namespace/parser/provider/kernel/allocator/CPU/full resources/platform/physical remain owned;
    full local/hosted79bd6f6a only,D340 fixed,D334/D156/D223/PLANNING.5 owned.
    promotion: declined (uses existing actual continuation and earned-source/real-let boundaries;new adapter witnesses/order and compatibility are retrievable in source/book/native contracts).
## _(2026-10-30 UTC)_ — suspend recipe Derived reads for actual earned source roots

0604 adds private Derived recipe read mode and shared earned witness helper with exact canonical-owner guard. Unread names yield to actual machine continuations;only actual cached supplier roots record reads. Original direct/prior-let/class paths and public recipe read enum/getter/RequiresEvaluation remain.
Private native1/recipe9/source8/doc1/scoped lib/affected-target strict lint/fmt pass. Real first RHS/store333,source1000/3 twice,next RHS2999/3 and once ordinal1 verified;foreign equal inventory guard caught. Independent recipe18/7/source27/8 and frozen bytes retained. Unused witness warning fixed with kind-only Debug.
Book98/357/1642/2895/glossary0 pass. No supplied cache/value or complete integrated result/second-let storage. [Whole0603](docs/history/stitchcad-devnotes-part762.md) retains four spans;prior live ids/order checked. Original standalone worker explicitly selects original mode at all operand sites.
Next .4.4.3 complete worker/pre-reservation/real storage/.4.4.4 actual Design execution/.4.5 independent acceptance. LIVE reviewed unchanged:G1 6/18/G0 unapproved;full-hosted79bd6f6a scoped. D340 fixed;D334/D156/D223/PLANNING.5 owned. Full PNT/push400;protected evidence intact.
## STITCHCAD-G1-0604 — adapt recipe Derived reads to earned original source roots

- Private recipe names suspend until actual shared source runtime computes their roots;record original canonical/supplier witnesses without replay or caller values. Exact canonical owner guard and original public recipe enum/RequiresEvaluation preserved. Combined worker remains next.
- Native1/recipe9/source8/doc1/scoped lint/fmt/book98/357/1642/2895/glossary0 pass. Real stored333/unbound1000/3/RHS2999/3 and once ordinal covered;independent18/7 and27/8/frozen bytes retained. Whole0603 retained762/four spans/live ids. Next .4.4.3 worker;LIVE G1 6/18/G0 unapproved unchanged.

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

## D308–D340 — runtime continuity and source ownership

D308–D333 fixed0537–0563;[prior ledger](../history/window24.md#stitchcad-devnotes-part726md). D335–D339 fixed;D339 proof protected. D340 P2 fixed0599:actual source9003 and operation/source collision guard;owner .5e.2.5.derived.3.3.3.4.2. D334 P2:G0-CONTRACT.15f after G1/before G0;D223/PLANNING.5 open. Full/hosted79bd6f6a SHA-scoped.
