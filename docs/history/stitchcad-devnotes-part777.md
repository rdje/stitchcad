# Sealed engineering continuity — part777

- **Identity:** 90 lines, 9364 bytes, sha256:e8c7bf91c32bbe93187f777e291dba1eee8685a89874b7c0e683bcd087b0ee4b
- **Coverage:** whole0617 task scope, lesson and changelog from 9a5f49f86f047cc15c54e45a4ab8df7097700c44.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 31419–34591, 3172 bytes, sha256:312b99990d49c84173c245fafc09b670982aa7408128f7976b04d99c552fa785
- Whole source interval: DEV_NOTES.md bytes 193–1443, 1250 bytes, sha256:9a5aaa95c94328fda6e87e7578a08c407b07f465cbb49806f37452ef1e967adc
- Whole source interval: CHANGELOG.md bytes 207–807, 600 bytes, sha256:d09de9db87090a6a499fbc6c1086a550b37e2e6a6388e8e11634a299d6428730
- Whole source interval: docs/tasks/PLANNING.md bytes 90576–94918, 4342 bytes, sha256:34dc055d3bf5cc5cfe54466a0a9afa996b383b2c2acd6c1e8fcb7491f066bfa0
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.5d.3.2.2.3`
  Status: `done`
  Goal:actual supplied-scalar tangent computation with original input/policy/native metadata and
  successful-call contributions,including exact zero/unit and original typed failures.
  - [x] REPRODUCE / ISSUE:native values0616 carried no supplied scalar's input contributions.
    Dropping Tan for exact zero/unit would let a completed approximation-producing dependency
    silently pass T1. Failure also needs original input/policy without a fabricated successful value.
  - [x] ROOT CAUSE (WHY + WHERE): cargo test -p sc-core --test formula_evaluated_tangent_contract
    →five actual source/binding/class/first-kind/prep/native-error tests pass,rc=0. Existing
    FormulaApproximationSources defines contributions for successful executed calls. Attempted
    operation metadata on failure must stay distinct;actual prior input contributions remain real.
  - [x] FIX:checked_tan calls actual native preparation/evaluation with explicit limits. Private
    result owns actual sourced input/native execution;value unions input sources with Tan even
    exact zero/unit. Private error retains actual input/limits/attempted Tan and original prep or
    native error chain. No caller result/source seed,no fabricated output or successful-call flag
    on failure;earlier successful Tan on the input survives a later refused attempt.
  - [x] ADDRESSED (verified): cargo test -p sc-core --test formula_evaluated_tangent_contract
    →five pass using real upstream round_to/cos kernels,not manually attached sources,rc=0.
    Exact zero/unit through cancellation,false comparison/storage/read preserve RoundTo/Tan and
    T1 refusal/T2 source admission;actual negative tan30 preserves Cos/Tan. Boolean false/prior
    successful Tan retain kind-before-empty-budget priority. Original fold/pole/late resource/
    actual width errors preserve input/policy/progress and exact native source-chain pointer.
  - [x] NO REGRESSION: cargo test -p sc-core --test formula_evaluated_calls_contract →eight
    existing supplied numeric-call tests pass,rc=0. cargo clippy -p sc-core --lib --test
    formula_evaluated_tangent_contract -- -D warnings / selected rustfmt --check / cargo test -p
    sc-core --doc recipe::evaluated::tangent:: →strict lint/fmt/one new positive doc pass,rc=0.
    Positive book98/374/1646/2908,glossary310/9/158/0,history776 four exact Git spans,all prior
    IDs/order/map/diff pass,rc=0. Input/source/native policies unchanged;no full CI/platform claim.
  - [x] LOCKSTEP:book/API map/native status facts/decision/task/index/MEM/DEV/CHANGE/LIVE align;
    whole0616 retained776. Next .4 independent combined native acceptance,then actual AST/source/
    recipe/Design policy adoption .5e.2.4.integration.tangent. LIVE source-bearing tangent values,
    G1 6/18/G0 unapproved unchanged. Broader provider/heap/CPU/platform/physical/release owners remain.
    promotion: declined (existing successful-call provenance and tangent-resource decisions own these semantics;this is scoped native implementation).
  Commit:STITCHCAD-G1-0617;Verification:focused checks pass;independent combined acceptance pending.
## _(2026-10-30 UTC)_ — retain actual tangent input and call contributions

0617 adds actual supplied-scalar checked_tan with explicit native limits. Successful wrapper retains original sourced operand/native execution;value unions every input dependency and Tan even exact zero/unit. It computes through the real native plan,without caller-seeded result or contributions.
Failure retains original input/limits/attempted operation and original prep/native error chain. Existing input contributions survive;no successful output or call contribution is invented. Prior completed Tan stays on a later wrong-kind input. No Boolean coercion or resource/domain substitution.
Five new wrapper tests use real round_to/cos kernels through false storage/T1/T2,kind/pole/fold/later quota/width failures and original chain identity. Existing numeric calls8/new doc1/scoped strict lint/fmt pass;book98/374/1646/2908/glossary0/history776 four spans/IDs/map/diff pass.
[Whole0616](docs/history/stitchcad-devnotes-part776.md) retained. Next .4 independent combined native acceptance,then real AST policy adoption. LIVE source-bearing values,G1 6/18/G0 unapproved unchanged. No full CI/platform/physical/release approval;full PNT/push400/protected proofs retained.
## STITCHCAD-G1-0617 — retain actual tangent operands and successful call contributions

- Add supplied-scalar checked_tan with original input/explicit policy/native execution. Successful zero/unit values retain Tan;failures preserve input contributions/attempted operation/original typed cause without a fabricated output or call flag.
- Wrapper5/earlier numeric calls8/doc1/scoped lint/fmt pass. Book98/374/1646/2908/glossary0/history776/IDs/map/diff pass. LIVE source-bearing tangent values,G1 6/18/G0 unapproved unchanged. Next independent combined acceptance,then actual AST policy adoption.

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
