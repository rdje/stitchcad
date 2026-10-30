# Sealed engineering continuity — part748

- **Identity:** 96 lines, 10186 bytes, sha256:d8be822bff7041806551d5abad7d916bda193f4062c32b219092cca19b821266
- **Coverage:** whole0589 task scope, lesson and changelog from 8c12b9e64efc8e63fa9951955feda2fdec0af10c.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 13373–17171, 3798 bytes, sha256:d5e334e7ccd87d4f166dacc12f0d833b4919101e48f4c337125e5c5a760559ad
- Whole source interval: DEV_NOTES.md bytes 193–1538, 1345 bytes, sha256:ec2fc9914f7525827c3ad03e5b7c10fc2db1225c496336a0507ff220de12e2d2
- Whole source interval: CHANGELOG.md bytes 207–874, 667 bytes, sha256:9180de2e4c1b408dc16aa88f19b00e62940f294b752589996f536d2c202656f5
- Whole source interval: docs/tasks/PLANNING.md bytes 90576–94952, 4376 bytes, sha256:65dfaffde190cafe173b443f02815c6f3ade65b378d905c0a331a0a8c940718e
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.5e.2.5.derived.3.1`
  Status: `done`
  Goal: allocation-free complete canonical-input/source join over both actual immutable owners.
  First reject source entity ids colliding with any owned canonical declaration;then validate every
  Derived formula reference in canonical length/general-scalar order,including unused targets.
  Publish no accepted partial scope. Lend actual source/declaration borrows with original family,
  declared kind,position and formula id;never use the separate source/provenance field as a formula
  reference,read values,clone state or infer a parameter. Preserve non-Derived zero/false/Unknown.
  Source existence alone supplies no namespace/kind/cycle/current Design/value proof. Existing
  canonical family contracts and source owners are the authority;focused native/compiler/lint/
  book checks then commit. Complete graph/resource/evaluation work stays .3.2 and later owners.
  Completion0589: complete original canonical/source joins available;graph work stays .3.2.
  - [x] REPRODUCE / ISSUE: existing canonical records retain Derived formula ids without source
    existence proof. Source owners0588 exist separately;actual join admission is required before
    any graph/value work. No authored result can be stored on an Unknown/Derived declaration.
  - [x] ROOT CAUSE (WHY + WHERE): rg -n 'LengthState::Derived|ScalarInputState::Derived|formula:
    EntityId|not a complete Design' value.rs/value/scalar.rs/value/inventory.rs → actual sole ids,
    original RequiresEvaluation and component scope,rc=0. Complete canonical getters/state-kind
    interfaces and existing source index/native owner contracts read before implementation.
  - [x] FIX: two allocation-free complete passes pin actual inventories:owned-id disjointness
    first,then every Derived reference. Borrow actual targets/sources without copied state or
    fabricated names/values. Exhaustive state matches require future variants to be considered.
  - [x] ADDRESSED (verified): cargo test -p sc-core --test formula_source_inputs_contract
    -- --nocapture →four pass,rc=0,including final exhaustive-arm rerun. Dedicated/all six general
    scalar kinds,shared-source actual pointers,original family/kind/positions,late missing formula,
    complete collision precedence and unchanged non-Derived/provenance/empty states retained.
  - [x] NO REGRESSION: cargo test -p sc-core --test formula_canonical_input_contract →five pass,
    rc=0;actual owner/family/kind/read boundaries remain. cargo test -p sc-core --doc
    recipe::source_inputs -- --nocapture →four pass,actual E0616/E0515 for both owners plus positive
    example,rc=0. Initial positive example incorrectly erased borrowed error into static Box and
    failed E0597;handle local join directly. Scoped strict lib/new-target Clippy/final changed
    rustfmt/diff pass,rc=0. Positive book96/310/1624/2845/glossary310/9/158/0 and747 four exact Git
    spans/all prior live ids/order pass,rc=0. Full graph/runtime/platform checks remain deferred.
  - [x] LOCKSTEP: book canonical/source join semantics/example/refusals and four API map rows,
    task/index/MEM/DEV/CHANGE/LIVE align. Next .3.2 actual normalized/static source proof phases,
    preserving real declaration-order/binding contracts before dependency/cycle/value work.
    Global Design,currentness,kind/graph/evaluation/resource envelope not certified. G1 6/18/G0
    unapproved;full/hosted79bd6f6a SHA-scoped. Full families/load/replay/physical and D334/D156/
    D223/PLANNING.5 remain owned;D339 proof untouched. No new language diagnostic/rounding rule.
    promotion: declined (existing sole formula reference,actual canonical ownership,complete admission and immutable borrowed-proof rules applied;no new language or authority policy).
## _(2026-10-30 UTC)_ — join actual canonical Derived references to original source owners

0589 checks complete canonical/source ownership before all Derived formula refs. Allocation-free borrowed scope lends actual family/kind/position/target/source without state clone,value read or invented name/rounding. Provenance refs are not owned declarations;source fields never replace formula fields.
Native4/canonical-owner5/doc4(E0616/E0515)/scoped strict lint/fmt pass. Initial positive example incorrectly boxed a borrowed error and failed E0597;handle the local join directly. Exhaustive state matches retain all five cases. Source existence alone supplies no output kind,graph,currentness or runtime proof.
Book96/310/1624/2845/glossary310/9/158/0 pass;[whole0588 original source inventory](docs/history/stitchcad-devnotes-part747.md) retains four exact Git spans/all prior live ids. G1 6/18/G0 unapproved/full-hosted79bd6f6a SHA-scoped. Existing canonical/source/borrow rules apply;promotion declined. D339 proof untouched.
Next .derived.3.2 actual normalized/static source phases and full resource/namespace/kind/graph admission,preserving real ordered-let binding contracts. Full families/load/replay/platform/physical and D334/D156/D223/PLANNING.5 owned. Full PNT;commit each slice,push400 after gates. Global Design proof remains open.
## STITCHCAD-G1-0589 — join actual canonical Derived formula sources

- Check complete source/canonical owned-id disjointness before all Derived references;lend actual target/source borrows with original family/kind/position. Preserve non-Derived states and provenance roles. No allocation,state/value cache or fabricated binding/graph/currentness proof.
- Native4/canonical-owner5/doc4/scoped lint/fmt/book96/310/1624/2845/glossary0 pass. Whole0588 retained747/four exact Git spans/live ids. Next .derived.3.2 normalized/static source proof phases. G1 6/18/G0 unapproved/full-hosted79bd6f6a SHA-scoped;full graph/evaluation/platform verification remains pending.

## D298/D299 — verified glossary/index repair0484

Done;owner G1-SLICE.6.vocabulary.acceptance. [Whole causes/reproductions/repairs](../history/window23.md#stitchcad-devnotes-part639md)
retain obsolete secondary pointer,definition-only use and token/chapter-pair assumptions;focused4/full41 pass0.

## D300 — corrected execution scope0485

Done;owner .6.5.3.2.1. [Whole reproduction/source proof](../history/window23.md#stitchcad-devnotes-part642md) retains the exact original ledger.

## D301 — parent-aware ease conservation before paired effects

P1 paired prerequisite;owner G1-SLICE.6.5.3.2,in_progress before effects. Partition/lineage complete0574:47 native/21 doc scoped checkpoint. Next .5e.2.5.derived.3.2 static sources after canonical joins0589. Real providers/physical joins .2.4/trusted atomic effects .2.5 remain pending before whole acceptance. Current repair/authority/runtime proof stays owned;local/hosted79bd6f6a stays SHA-scoped.
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

## D308–D339 — runtime continuity and book search

D308–D333 fixed0537–0563;[prior ledger](../history/window24.md#stitchcad-devnotes-part726md). D335–D339 fixed;D339 native chapter search0586 preserves full words/terms,normal warnings retained. Next .5e.2.5.derived.3.2 actual static source proof phases. D334 P2:G0-CONTRACT.15f after G1/before G0;D223/PLANNING.5 open. G1 6/18/G0 unapproved;full/hosted79bd6f6a SHA-scoped.
