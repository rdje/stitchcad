# Sealed engineering continuity — part762

- **Identity:** 102 lines, 10678 bytes, sha256:86dfc4b95fa4902bca05addb50505a373af4c9d24f9e387ab3484858c3078dba
- **Coverage:** whole0603 task scope, lesson and changelog from 15a52b747a566b9cfa28406641187cc8a96a8c6a.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 18821–23272, 4451 bytes, sha256:152b61c0f2e709b318f47a7f21395109af8091fa5221f158a6b144b802bd71d0
- Whole source interval: DEV_NOTES.md bytes 193–1483, 1290 bytes, sha256:2a126b7d2b626374d6ef1f8a4d8afb8ca4ba5afa3859fe49f6e17aeac7b9fc2f
- Whole source interval: CHANGELOG.md bytes 207–822, 615 bytes, sha256:5edbdd537b99355b5bd9c16b45ee2208d15b566e798a7d9634875089f3e70748
- Whole source interval: docs/tasks/PLANNING.md bytes 90576–94898, 4322 bytes, sha256:ae187668c2d92f6a9154ee45120784c598f2b67d28072f4d357b881e1a3a9468
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.5e.2.5.derived.3.3.3.4.4.1`
  Status: `done`
  Goal: extract shared private source runtime from existing worker. Full S/N reservations precede
  values;real completed roots/ordinals survive repeated distinct demands without re-evaluation or
  binding. Original one-shot selection/allocation/read/failure/result contracts remain exact.
  Commit integrity:first hook refused ROOT's missing diagnostic invocation/result within its own
  bullet. Recorded actual rg result there and refreshed book authority before restaging/retry.
  Completion0603:shared private actual source runtime;standalone public contract preserved.
  - [x] REPRODUCE / ISSUE:original source worker owns one local demand walk/cache and cannot be
    reused across actual ordered recipe requests without replay or a separate counterfeit cache.
    Shared source control must be reserved before values and retain only actual computed roots.
  - [x] ROOT CAUSE (WHY + WHERE):inspect complete source worker/read/result/failure,private shared
    machine and actual source graph plans. Original local machines/frames/outputs/ordinal become
    one private runtime;source lookup and public selected-entry error context remain in the adapter.
    rg -n 'struct SourceRuntime|fn (new|demand|finish)|SourceRuntime::new' on actual
    execution/source_runtime.rs and source_worker.rs →private new/demand/finish plus existing
    worker adapter invocation,rc=0. Full reservation is in new;real roots/poison guard are in demand.
    Initial extraction E0425 missing EntityId import and test E0609 public source-view/read-slot
    confusion corrected through actual import/private slot definitions;no new public data path.
  - [x] FIX:fallibly reserve complete S controls/frames/output slots and every source's N task/value
    buffers before starting machines. Private demand reuses actual successful roots/ordinals across
    entries without re-evaluation or storage. Failed demand poisons the whole runtime before work;
    neither reuse nor finish can publish an earlier prefix. Public exact-entry validation still
    precedes allocation and original graph/limits/selection/context/read/failure/result survive.
  - [x] ADDRESSED (verified): cargo test -p sc-core --lib source_runtime::tests -- --nocapture
    →one pass/87 filtered,rc=0. Actual source/namespace/canonical/graph factories,not seeded values:
    a=1000/3,b=a*3=1000,c=a+a=2000/3. Shared a stays ordinal1,cached repeat changes no ordinal;
    unused source stays empty,buffers retain pointers and canonical Derived stays RequiresEvaluation.
    Actual later division failure preserves original source3/cause. Two caught private lifecycle
    guard panics are expected:failed runtime cannot return an earlier root or publish its prefix.
  - [x] NO REGRESSION: cargo test -p sc-core --test formula_scalar_source_graph_execution_contract
    -- --nocapture →eight pass,rc=0 including actual512-source/64KiB-requested-stack case and both
    independent27-case/8-refusal/frozen-byte watchers. Original roots/ordinals/read traces/canonical
    states/errors remain exact. Scoped strict core lib/affected integration-target Clippy and
    changed rustfmt/diff →pass,rc=0. cargo test -p sc-core --doc
    recipe::execution::source_worker -- --nocapture →one pass,rc=0. Book98/357/1642/2895,
    glossary310/9/158/0,761 four exact Git spans/prior live ids/order/new uniqueness pass,rc=0.
    No broad CI/matrix;new private test is native checked,not a claimed all-unit Clippy campaign.
  - [x] LOCKSTEP:actual Design formula annex explains shared earned unbound roots/full reservation,
    unchanged public single-entry semantics and failed-prefix ownership. Task/index/MEM/DEV/CHANGE
    align;LIVE reviewed unchanged,G1 6/18/G0 unapproved. Whole0602 parent/child scope retained761/
    four spans;original child links preserve0600/0601 provenance. Next .4.4.2 actual recipe Derived
    adapter/.4.4.3 complete worker/.4.4.4 Design execution/.4.5 independent integration acceptance.
    Namespace/parser/provider/kernel/allocator/CPU/full resource/platform/physical boundaries remain
    owned. Full local/hosted79bd6f6a scoped;D340 fixed,D334/D156/D223/PLANNING.5 remain owned.
    promotion: declined (internal extraction applies existing earned-unbound cache and machine lifecycle rules;actual repeated-demand/poison evidence and public compatibility remain retrievable in source/book/native contracts).
## _(2026-10-30 UTC)_ — share actual earned source roots across private demands

0603 extracts fully reserved source controls/machines/frames/outputs into one private runtime. Actual roots/ordinals survive repeated demands without re-evaluation or binding;failure poisons reuse/publication. Existing public exact-entry selection/error/context/result adapter preserves its original contract.
Private native1/source8/doc1/scoped lib/affected-target strict lint/fmt pass. Actual a1000/3,b1000,c2000/3,once ordinals/stable buffers/later failure checked;two caught private guards expected. Independent27/8-refusal/frozen bytes and512-source requested64KiB stack pass. Missing import/public-view slot mistakes fixed.
Book98/357/1642/2895/glossary0 pass. Shared private cache/full reservation and public compatibility documented. [Whole0602](docs/history/stitchcad-devnotes-part761.md) retains parent/children and four spans;prior live ids/order/new uniqueness checked. No public recipe bridge or new execution API yet.
Next .4.4.2 real recipe Derived adapter/.4.4.3 complete worker/.4.4.4 Design bridge/.4.5 independent acceptance. LIVE reviewed unchanged:G1 6/18/G0 unapproved;full-hosted79bd6f6a scoped. D340 fixed;D334/D156/D223/PLANNING.5 owned. Full PNT/push400;protected evidence intact.
## STITCHCAD-G1-0603 — share actual earned source roots across private demands

- Extract fully reserved private source runtime with once-only unbound roots/ordinals,unchanged public one-entry behavior and failure-poisoned reuse/publication. No caller-seeded cache or new public recipe execution API.
- Private native1/source8/doc1/scoped lint/fmt/book98/357/1642/2895/glossary0 pass. Real repeated roots/buffer stability/failure guards and independent27/8-refusal/frozen bytes retained. Whole0602 parent/children retained761/four spans/live ids. Next .4.4.2 recipe adapter;LIVE G1 6/18/G0 unapproved unchanged.

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
