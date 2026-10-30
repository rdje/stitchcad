# Sealed engineering continuity — part778

- **Identity:** 137 lines, 12531 bytes, sha256:95fd7ee057c0fb56b60247c31fd45ab3914728f852b14d73af809633c85c9091
- **Coverage:** whole0618 task scope, lesson and changelog from 51980f41c416ec7ed0f310080b06edb94fd9f125.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 29541–35808, 6267 bytes, sha256:4ef6ba12bc6915e4e1455b2ee5a5fd410edf52b0188ebf4e058c7c2da2980568
- Whole source interval: DEV_NOTES.md bytes 193–1478, 1285 bytes, sha256:17e918ca3ae0bdfafd9e8dc3316e0e07999d99f857ec63f402719679f438f168
- Whole source interval: CHANGELOG.md bytes 207–844, 637 bytes, sha256:b1e5f59bd0a70b8849bc9bd584621e784e8844c729dc4bda2fe3708619ea91ed
- Whole source interval: docs/tasks/PLANNING.md bytes 90576–94918, 4342 bytes, sha256:34dc055d3bf5cc5cfe54466a0a9afa996b383b2c2acd6c1e8fcb7491f066bfa0
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.5d.3`
  Status: `done`
  Goal: sin/cos/tan, atan, atan2 and arc_length with signed sweeps and reviewed rounding.
  Acceptance: exact odd-quarter-turn tan poles, signed principal branches, atan2(y,x)/(0,0),
  zero axis and near-axis rounded endpoints, complete turns and signed/multiple-turn sweeps;
  no blanket angle normalization, quantum/source-domain errors or libm-dependent result.
  Split functions into further owned children if this unit exceeds a safe implementation slice.
  Verification:native implementation/scoped acceptance0618;Commit:STITCHCAD-G1-0618.
  Closure0618:available native values/sources under explicit admitted plans;coupled irrational
  review .5d.4 and AST policy adoption .5e.2.4.integration.tangent remain separate. No platform claim.
- ID: `G1-SLICE.5d.3.1`
  Status: `done`
  Goal: exact signed/full-turn arc_length and23 independent cases verified0258.
  Paths: crates/sc-core/src/recipe/arc.rs;crates/sc-core/src/recipe/rational.rs;crates/sc-core/src/recipe/numeric.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_arc_value_contract.rs
  Additional paths: docs/tasks/artifacts/formula_structure/arc_value_oracle.py;crates/sc-core/tests/fixtures/formula_arcs.tsv
  Acceptance: [whole scope/six proof boxes](../history/window13.md#stitchcad-devnotes-part332md); provenance .5e.3.
- ID: `G1-SLICE.5d.3.2`
  Status: `done`
  Goal: correctly rounded sin/cos/tan under .5d.1.2.2 plan.
  Closure0618:available native values/sources under explicit admitted plans;coupled irrational
  review .5d.4 and AST policy adoption .5e.2.4.integration.tangent remain separate. No platform claim.
- ID: `G1-SLICE.5d.3.2.1`
  Status: `done`
  Goal:0280 native sin/cos verified3/0; original scope379, whole retention below.
  Acceptance: [whole summary/provenance](../history/window14.md#stitchcad-devnotes-part400md).
- ID: `G1-SLICE.5d.3.2.2`
  Status: `done`
  Goal: tangent only after its separate admitted all-input resource plan; no borrowed sin/cos certificate.
  Closure0618:available native values/sources under explicit admitted plans;coupled irrational
  review .5d.4 and AST policy adoption .5e.2.4.integration.tangent remain separate. No platform claim.
- ID: `G1-SLICE.5d.3.2.2.1`
  Status: `done`
  Goal:0615 completed scope; [whole scope/proofs](../history/stitchcad-devnotes-part775.md).
- ID: `G1-SLICE.5d.3.2.2.2`
  Status: `done`
  Goal:0616 completed scope; [whole scope/proofs](../history/stitchcad-devnotes-part776.md).
- ID: `G1-SLICE.5d.3.2.2.3`
  Status: `done`
  Goal:0617 completed scope; [whole scope/proofs](../history/stitchcad-devnotes-part777.md).
- ID: `G1-SLICE.5d.3.2.2.4`
  Status: `done`
  Goal:independent combined native tangent value/source/resource acceptance after implementation.
  - [x] REPRODUCE / ISSUE:focused native examples alone cannot independently establish actual
    transcribed values,original typed poles/widths or policy-independent source-bearing outcomes.
  - [x] ROOT CAUSE (WHY + WHERE): python3 -I -B docs/tasks/artifacts/formula_structure/tangent_value_oracle.py --check
    →139 independent exact Fraction rows:125 values/10 poles/four widths;7141B,rc=0. It uses signed
    rational centering,alternate atan(1/2)+atan(1/3) pi and direct factorial powers,not native absolute
    folding,Machin pi or fixed-point recurrences. No Rust/native output/frozen input is read.
  - [x] FIX:guarded read-only/stdout producer and frozen independent fixture;native watchers call
    actual preparation/worker and sourced wrapper under two explicit policies. Compare values,
    original operands/limits/progress,Tan contributions,exact poles and exact width bits. A separate
    watcher regenerates all frozen bytes;register producer/file ownership and existing probe suite.
  - [x] ADDRESSED (verified): cargo test -p sc-core --test formula_tangent_oracle_contract -- --nocapture
    →two pass:all139 rows under16412/7680 and70000/40000 through both actual entry points;all7141
    bytes regenerate exactly,rc=0. Full128-bit inputs,wide finite Ratios,complete turns,near-pole
    neighbors and both sides of the actual half-ppm boundary are present. Exact sqrt(3) inequality
    control proves tan30=577350 and rejects rounded-component577351. Fixture SHA256
    cf1a803c1efd405dcd84d6af73340b675824c979b793bfbb6112862218a39d7b.
  - [x] NO REGRESSION: cargo test -p sc-core --test formula_tangent_prepare_contract --test
    formula_tangent_value_contract --test formula_evaluated_tangent_contract →14 pass,rc=0.
    cargo test -p sc-core --lib recipe::tangent:: →two pass,including873 independent signed folds
    and excessive native extent guard,rc=0. Together with oracle two:18 selected native tests pass.
    cargo clippy -p sc-core --lib --test formula_tangent_oracle_contract -- -D warnings / selected
    rustfmt --check / bash -n docs/tasks/artifacts/formula_structure/run_formula_structure_probes.sh
    →strict lint/fmt/shell syntax pass,rc=0. Producer census346/33981 calls;file233/89/38 with15
    actual data refusals/12 guard reds pass,rc=0. Positive book98/374/1647/2910,glossary310/9/158/0,
    history777 four exact Git spans,prior IDs/order/map/diff pass,rc=0. No full CI/platform claim.
  - [x] LOCKSTEP:book/status/native facts/decision/task/index/MEM/DEV/CHANGE/LIVE align. Whole0617
    retained777. Native tangent/.5d.3.2/.5d.3 implementation parents close for scoped native values;
    coupled .5d.4 remains pending. Next real AST/source/recipe/Design policy integration owns
    implementation and paired acceptance. LIVE Tan139,G1 6/18/G0 unapproved unchanged. Existing
    full locality/provider/heap/CPU/platform/replay/physical/release owners remain separate.
    promotion: promoted (decision_tangent-resources.md gains independent native acceptance answer,fixture identity and explicit scope).
  Commit:STITCHCAD-G1-0618;Verification:18 selected native tests/139 independent rows;broader gates pending.
- ID: `G1-SLICE.5d.3.3`
  Status: `done`
  Goal: correctly rounded atan/atan2 under .5d.1.2.2 plan.
- ID: `G1-SLICE.5d.3.3.1`
  Status: `done`
  Goal:0288 native inverses verified3/0; original scope396, whole retention below.
  Acceptance: [whole summary/provenance](../history/window14.md#stitchcad-devnotes-part401md).
## _(2026-10-30 UTC)_ — independently accept scoped native tangent execution

0618 independent Fraction oracle uses signed rational centering,alternate atan1/2+atan1/3 pi and direct factorial powers. No Rust/native output/frozen input is read. It certifies139 rows:125 values/10 poles/four widths,including full128-bit,wide finite,near-pole and actual midpoint-neighbor cases.
Two native watchers compare actual preparation/worker and sourced wrapper under two explicit policies;original inputs/limits/progress/flags and exact causes survive. All7141 fixture bytes regenerate;exact sqrt(3) control rejects rounded-component tan30. Eighteen selected native tests pass.
Scoped strict lint/fmt/shell syntax/book98/374/1647/2910/glossary0/census346/file233+15/12/history777 four spans/IDs/map/diff pass. Existing probe suite watches oracle;full transitive locality review stays owned. [Whole0617](docs/history/stitchcad-devnotes-part777.md). Independent acceptance answer promoted.
Native tangent/.5d.3 implementation parents close;coupled .5d.4 pending. Next actual AST/source/recipe/Design tangent policies,original proof/context/error contracts and paired acceptance. LIVE Tan139,G1 6/18/G0 unapproved unchanged. No full CI/platform/production claim;full PNT/push400/proofs retained.
## STITCHCAD-G1-0618 — independently accept scoped native tangent values and sources

- Exact Fraction alternate-identity/direct-power oracle certifies139 rows. Actual native values/wrappers match125 values/10 poles/four exact widths under two policies;all7141 frozen bytes regenerate. Eighteen selected native tests pass,including real resource/source guards.
- Scoped lint/fmt/shell syntax/book98/374/1647/2910/glossary0/census346/file233/history777/IDs/map/diff pass. LIVE Tan139,G1 6/18/G0 unapproved unchanged. Native implementation closes;coupled/platform review and real AST policy adoption remain owned. Next AST integration.

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
