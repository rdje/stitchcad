# Sealed engineering continuity — part803

- **Identity:** 69 lines, 7285 bytes, sha256:834add120e18bb1d30d09b233105164c8261873f375cf5c1a738061a7bb61c42
- **Coverage:** whole0643 task scope, lesson and changelog from 12d79c08b127ff8a199359ae8c7089d2f15ab3a6.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 36284–37260, 976 bytes, sha256:ce09dc20c49b7aaeef86efb643d87cbb2b47f79bc71e3fe3fdd8b1819498bd7a
- Whole source interval: DEV_NOTES.md bytes 193–1600, 1407 bytes, sha256:1a50b95e386df247273e7e0a9413f960f34bb9e05a55136a8309c21dac57bd00
- Whole source interval: CHANGELOG.md bytes 207–759, 552 bytes, sha256:aeb5aacdd90c837628080c678814428b3349e481418992762f3a26c51e73f8a5
- Whole source interval: docs/tasks/PLANNING.md bytes 90576–94926, 4350 bytes, sha256:0c8b284e485512f38f0dec65d3fbc37791a166921bf34c999f09bfac25ccb8a1
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.5d.4.acceptance.gates.copy-lint`
  Status: `done`
  - [x] ISSUE:D346 full make check exits2 before tests on clone_on_copy.
  - [x] ROOT CAUSE (WHY + WHERE):Tolerance derives Copy;recipe_runtime/tests.rs used clone.
    scripts/run_make.sh check →Clippy clone_on_copy at268,rc=2;full tests not run.
  - [x] FIX:Copy assignment retains distinct equal provider storage and all original assertions.
  - [x] ADDRESSED (verified):local_environment.py -- cargo test -p sc-core --lib
    paired_runtime_requires_original_borrowed_provider_and_label_sources →1 pass,rc=0.
  - [x] NO REGRESSION:same launcher cargo clippy -p sc-core --lib --tests -- -D warnings and
    cargo fmt --all -- --check pass,rc=0;book101/374/1670/2978/glossary0/history802/IDs+2/map/diff0.
  - [x] LOCKSTEP:book/live/notes sync;full gates rerun next;promotion declined(existing identity).
  Commit:STITCHCAD-G1-0643;Verification:actual identity watcher/strict lint/fmt;full gates pending.
## _(2026-10-30 UTC)_ — preserve provider identity under strict Copy lint

First deferred full check stopped before tests:make check exits2 on clone_on_copy in recipe_runtime/tests.rs:268. Tolerance has derived Copy. D346 fixed0643 by plain Copy assignment;the existing test still checks equal fields/distinct addresses and rejects foreign provider/label borrows.
The actual private identity watcher passes1;strict sc-core lib/tests Clippy and workspace formatting pass. The original provider still executes the paired recipe. Production implementation/quantitative semantics are unchanged. Full native/Wasm/doctrine/probe gates must rerun next.
Book101/374/1670/2978/glossary0/history802 four Git spans/IDs+2/map/diff pass. Book clarifies borrowed provider identity even for Copy values. No full integration verdict or daily cleanup. Original failed/full and focused check logs remain distinct.
D346 receipt target/irrational_platform_certificate/coupled-0643-copy-20261030T182217444630Z-15087/receipt.json SHA f538d53145670f48e8c021f982b9b83dec679861dba0c86763d471b8c23cbe88 retains five exact original logs.
[Whole0642](docs/history/stitchcad-devnotes-part802.md) retains D345 capture preservation. Promotion declined:existing identity contract. LIVE marks D346 fixed;G1 6/18/G0 unapproved unchanged. Next .acceptance.gates.run;remaining application/resource/release owners remain. Full PNT continues.
## STITCHCAD-G1-0643 — retain borrowed-provider identity with Copy assignment

- Fix D346 strict lint failure in the original private provider/label identity watcher:copy Tolerance directly. Its equal-value/distinct-address and foreign-provider/label assertions still pass1;strict affected lint/workspace formatting pass. Production arithmetic unchanged.
- Book101/374/1670/2978/glossary0/history802/IDs+2/map/diff pass. First full gate exited2 before native tests;full integration must rerun next. G1 6/18/G0 unapproved unchanged;no daily cleanup.

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

## D308–D346 — prior closures and strict provider-identity lint

D308–D345:[whole prior ledger](../history/stitchcad-devnotes-part802.md). D334 P2 after G1/before G0;D223/PLANNING.5 open. D346 P1 fixed0643,owner G1-SLICE.5d.4.acceptance.gates.copy-lint:Copy assignment preserves original provider/label identity watcher;test1/strict affected lint/fmt pass. First full make check exited2 before tests;full gates rerun next.
