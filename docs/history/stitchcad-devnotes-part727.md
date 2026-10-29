# Sealed engineering continuity — part727

- **Identity:** 115 lines, 11822 bytes, sha256:e1407225c9d7e1aa89552fb527b9eb8d351abbdbe7c021e2893564bac967c869
- **Coverage:** whole0568 task scope, lesson and changelog from 9858935f9c166ec9f69cb11b31cff2ec66296ddd.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 23717–29245, 5528 bytes, sha256:3cf5d7c321e89b753f71362bdc30b19ad47b6145e43e1dc9d7ae7d5fad4c17dd
- Whole source interval: DEV_NOTES.md bytes 193–1488, 1295 bytes, sha256:e8467c95a3f9c8084c986399cd6dab74e3f3221d0bbb08f385bd10583e896ca7
- Whole source interval: CHANGELOG.md bytes 207–864, 657 bytes, sha256:60b8135ac9be35e018bff2448c66c4afce6926983f7a9274512aaf2a39f02135
- Whole source interval: docs/tasks/PLANNING.md bytes 90576–94918, 4342 bytes, sha256:30a1cc42c9eef296009a4ba5df50c7e0695ca90f9df66b8f6fafdb13acc14146
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.3.2.3.4.2`
  Status: `done`
  Goal: bind complete span history to actual SewingGraph and Design identity/resource ownership;
  reconcile retained original source IDs with canonical retired owners exactly once. Keep legacy held
  drafts inspectable without inventing historical splits. Admission/lookup must expose actual live/
  retired state and reject inconsistent graph/issued identities before candidate effects.
  Scoped source recovery:actual sewing constructor/current validator,Design sewing/garment identity
  walks and complete resource composition;only library graph constructor is sewing.rs,new private
  graph resource view has one caller. Standard graph Clone must retain history. Gather keeps its
  actual old target and existing current MissingSpan boundary;no automatic child transfer.
  Implement optional private immutable history with pure identity binding to exact live graph leaves
  and retired source roots;reject replacement,missing/retired live IDs,unrepresented history leaves,
  graph-ID collisions and missing/live source roots. Actual graph lookup borrows exact live member
  or exact immediate retirement children;legacy provenance alone supplies no invented split.
  Include complete retired IDs in current graph external-namespace checks and Design ownership;
  source/live aliases owned once in history birth order. Exhaustive resource views charge actual
  history vector and fixed Copy fields on every snapshot;legacy None accounting unchanged.
  Paths:ontology/sewing.rs and new sewing/history.rs,seam_span_history.rs private views,exports;
  design/sewing_state.rs,design/resource/sewing.rs;focused core/actual Design/resource/native
  privacy tests,book lineage/resource sections/scoped map and task/live docs. No bus effect/new ID
  generator or current physical/formula/Profile certificate. Scoped lint/fmt/tests;full paired
  verification stays deferred. Actual pressure63/64 active,84/192 aggregate;one record726 fits,
  then pack before any further retirement. Never confuse aggregate and active slot counts.
  CLAIM CLARIFICATION0567:commit body before-allocation wording means complete entry/issued-count
  admission;chronological validation then uses the bounded map. Source/book already say this exactly.
  D335,P1 owned/fix now:history stores selected runtime admission limits;derived Eq would make
  canonical graph equality depend on caller policy after attachment. Reproduce with identical
  valid Root records under1/1 versus8/16 allowances;expected semantic equality fails. Root is the
  retained limits field in seam_span_history.rs,not different span content. Remove runtime policy
  from the owned history,retain explicit constructor limits and complete pre-allocation admission;
  update obsolete getter/tests/docs and preserve private-field doctest's intended refusal.
  Completion0568:actual graph/Design history identity binding and resource integration complete.
  - [x] ROOT CAUSE (WHY + WHERE): actual native D335 equality test under identical Root7
    data/limits1/1 versus8/16 failed the intended equality assertion,rc=101. Retained limits plus
    derived Eq in0567 made policy part of owned history. Remove stored policy/getter,retain
    explicit constructor caps and complete admission;native same regression now passes,rc=0.
    Private forge doctest updated to actual fields,not masked by obsolete-field compilation noise.
  - [x] ADDRESSED (verified): local_environment.py cargo test history contract →6 pass;
    core sewing_contract span_history →5 pass;actual sc-measure design_inputs_contract
    span_history →2 pass,all rc=0. Exact live leaves/retired source ancestors,unrelated-family
    refusal,borrowed pointers,history cloning,old provenance absence,typed identity refusal and
    policy-independent actual graph/Design equality pass. Complete retired IDs participate in
    current external namespaces and actual Design inputs;source/live aliases owned once by births.
    Resources charge actual history vector after source bodies;graph5+history4=9,limit8 refuses
    exact5+4 field/owner;two snapshots18. Actual Design adds3 items with text/counters unchanged.
  - [x] NO REGRESSION: four existing sewing resource cases and29 actual stop cases pass,rc=0.
    Relevant primitive4/graph4 doctests pass,rc=0;scoped strict Clippy core library/three test
    targets and measure library/design-input target plus fmt --all --check pass,rc=0. Initial
    fixture clone/two unwraps/export wrapping corrected;affected namespace case rechecked1/0.
    Both book examples byte-exact to passed doctests;positive book94/255/1595/2787 and actual
    glossary310/9/158/0 pass,rc=0. Receipt726/four exact Git spans and actual generation-pressure
    census pass,rc=0:active64/64,aggregate85/192. Full paired verification remains deferred.
  - [x] LOCKSTEP: book lineage/resource/source-binding and two scoped API rows,MEM/DEV/CHANGE/
    LIVE/index/D301/D335 align. Whole0567/prior ledger retained726. D335 fixed;D334 P2/D156/
    D223/PLANNING.5 remain owned;G1 6/18/G0 human unapproved. Old full/hosted79bd6f6a verdict
    stays exact-SHA scoped. Pack window24 prerequisite next,before more retirement;then actual
    paired candidate/reference joins .3.4.3. No actual command/physical/provider/effect proof.
    promotion: declined (application of indexed canonical-content versus runtime-policy separation,actual identity ownership and exhaustive logical accounting;D335 root/reproduction/fix retained here and in the book).

## _(2026-10-29 UTC)_ — bind span history to actual graph ownership

0568 binds history to actual graph leaves/retired source ancestry;lookup lends exact live members or immediate retirement records. Current namespaces and Design own every issued ID once,including retired intermediates. Exhaustive resource views charge each history vector on every snapshot;legacy None stays exact.
D335 native equality red101:identical Root7 under1/1 versus8/16 differed because limits were retained. Remove runtime policy/getter from owned data,keep explicit complete admission. Local/actual graph/Design equality now passes. Private forge doctest matches actual fields;no unrelated-field masking.
Native history6/graph5/Design2/legacy resource4/stop29 and doctest8 pass;scoped strict lint/fmt pass after fixture clone/unwrap/export fixes. Book94/255/1595/2787,glossary310/9/158/0 and receipt726/four spans pass. LIVE G1 6/18/G0 human unapproved;D335 fixed,D334 P2/D156/D223/PLANNING.5 owned.
[Whole0567 history proof](docs/history/stitchcad-devnotes-part726.md). Promotion declined:indexed canonical policy separation/ownership/resource contracts. Active64/64 requires window24 packing before more retirement,then candidate/reference joins .3.4.3. No physical/provider/authority/effect/full paired claim.
## STITCHCAD-G1-0568 — bind span history to actual graph ownership

- Bind actual live leaves/retired source ancestry and explicit borrowed lookup;own every birth once through Design and charge each history vector per snapshot. Fix D335:runtime admission policy is not canonical identity content. History6/graph5/Design2/resource4/stop29/doc8,scoped lint/fmt pass.
- Book94/255/1595/2787/glossary310/9/158/0 pass;whole0567 retained726/four spans. Active64/64 needs window24 packing before next candidate/reference work. G1 6/18/G0 human unapproved;D334 P2/D156/D223/PLANNING.5 owned. Physical/provider/authority/effect/full paired proof remains pending.

## D298/D299 — verified glossary/index repair0484

Done;owner G1-SLICE.6.vocabulary.acceptance. [Whole causes/reproductions/repairs](../history/window23.md#stitchcad-devnotes-part639md)
retain obsolete secondary pointer,definition-only use and token/chapter-pair assumptions;focused4/full41 pass0.

## D300 — corrected execution scope0485

Done;owner .6.5.3.2.1. [Whole reproduction/source proof](../history/window23.md#stitchcad-devnotes-part642md) retains the exact original ledger.

## D301 — parent-aware ease conservation before paired effects

P1 paired prerequisite;owner G1-SLICE.6.5.3.2,in_progress before effects. Ease0527/stop0566/history0567/actual graph-Design binding0568 implemented with scoped checks. Full local/hosted79bd6f6a passed0564. Next pack active64/64,then actual candidate/reference joins .3.4.3;real formula/Profile/geometry,physical/authority/aggregate effects/full acceptance stay owned.
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

## D308–D335 — prepush repairs and identity policy

D308–D333 fixed0537–0563;[prior ledger](../history/stitchcad-devnotes-part726.md). Local/hosted79bd6f6a checks pass0564;cleanup/stop/history checkpoints follow. D335 fixed0568:runtime limits excluded from canonical history. D334 P2:G0-CONTRACT.15f after current G1/before G0 closure. D223/PLANNING.5 open. Next pack active64/64,then candidate joins. G1 6/18;G0 unapproved.
