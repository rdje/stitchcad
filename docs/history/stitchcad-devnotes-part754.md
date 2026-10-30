# Sealed engineering continuity — part754

- **Identity:** 104 lines, 10922 bytes, sha256:b01a8da14bfb7e1237d57616c5dca1434dc25e01d401b1aac050d8454ae72c90
- **Coverage:** whole0595 task scope, lesson and changelog from ef197008fc52d295025c255b9df41d8706ce25ec.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 15682–20256, 4574 bytes, sha256:87dd479df61d01067908ce972be31117c9488870f35f32a3db4910026718c124
- Whole source interval: DEV_NOTES.md bytes 193–1501, 1308 bytes, sha256:8dd500ace47b749325c770ae434845407e00fe502df4eebc2cea0db64d9dd77e
- Whole source interval: CHANGELOG.md bytes 207–897, 690 bytes, sha256:2e16ab7d6c9c9bda9784e2b0a17dfbf5ca605ccbc5f3b2c6d1423a788f8750f8
- Whole source interval: docs/tasks/PLANNING.md bytes 90576–94926, 4350 bytes, sha256:72482904104b2b0c6497d6793e631ee7db44d65df888b6cdca514afc3197e208
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.5e.2.5.derived.3.3.3.2`
  Status: `done`
  Goal: private resumable actual-node machine for a real unresolved Derived read. Preserve pending
  tasks/actual operand values and original read order across supplier evaluation;no restart or
  fabricated read/error/value. Existing expression/recipe execution must remain exact. Runtime
  graph source cache can be earned only by actual source execution,never caller callbacks/seeds.
  Concrete scope:private Ready/AwaitDerived read control and Complete/AwaitDerived progress only.
  Start pins actual root/context and node allowance;resume retains pending tasks,actual completed
  operands and selected conditional contributions. A real unresolved name is requeued before
  yielding,with its original node/canonical join;no value/read recorded until actual supplier
  completion. Existing one-shot expression/recipe adapters retain original RequiresEvaluation.
  Failed machines cannot resume/restart;successful scratch remains reusable. No new public
  callback,source cache,value-result constructor,rounding or numeric dispatch. Focused private
  tests use actual source definitions,canonical Derived records and actual executed supplier roots,
  never arbitrary AST values. Existing expression/recipe native results/oracles must remain exact.
  Completion0595: private actual-node continuation with original pinned context and unbound roots.
  - [x] REPRODUCE / ISSUE: a real Derived supplier needs to run between consumer steps without
    restarting earlier actual reads/operands or introducing a source-edge storage round. Existing
    expression/recipe one-shot execution must retain original requirements and exact outcomes.
  - [x] ROOT CAUSE (WHY + WHERE): machine.rs held root/context only in run() and could return only
    a final V/error. Original Sources/read/provider/dispatch and real recipe store→publish paths
    inspected;rg -n 'ReadValue|Progress|State|start|resume|Task::Enter' machine.rs → private control,
    original-node restoration and pinned context,rc=0. No numeric kernel or binding change.
  - [x] FIX: private Ready/AwaitDerived reads,Complete/AwaitDerived progress and Idle/Running/
    Failed lifecycle. Pin original root/context;requeue exactly the popped unresolved name within
    existing N reservation,retain real values/tasks/conditional contributions. Failed machines
    cannot restart/resume;successful scratch reuses buffers. Old adapters wrap successful reads
    only;one-shot pause fallback retains actual canonical RequiresEvaluation. No public seeds/API.
  - [x] ADDRESSED (verified): cargo test -p sc-core --lib recipe::execution::machine::tests --
    --nocapture →three pass,rc=0. Actual source registry/proofs/Derived record and executed supplier
    roots cover pending arithmetic/calls,condition contributions,repeated suspension,once-only
    earlier reads,untaken Derived laziness,unchanged buffer pointers/state and original failures.
    Unbound1000/3 survives supplier continuation;result1001 and sin/cos contributions remain exact.
    Caught private guards reject failed resume/restart. Initial fixture E0599 used nonexistent span
    range();actual start()/end() accessors fix it. No manually supplied AST result is accepted.
  - [x] NO REGRESSION: cargo test -p sc-core --test formula_scalar_execution_contract --test
    formula_scalar_recipe_execution_contract -- --nocapture →11/9 pass with independent49/18-row
    value/source/read/refusal fixtures exact,rc=0. Scoped cargo clippy -p sc-core --lib -- -D warnings
    passes,rc=0;inline context enum allowance documents why boxing adds an unreserved allocation
    after initial large_enum_variant refusal. Changed rustfmt/diff pass,rc=0. Positive book
    97/335/1629/2865/glossary310/9/158/0 and753 four exact Git spans/prior live ids/order pass,rc=0.
    Initial commit hook refused unrecognized promotion label;required declined syntax repaired.
    No full CI/matrix campaign;new whole-source result oracle/worker remains owned by next .3.
  - [x] LOCKSTEP: private checkpoint/examples/boundaries in book,task/index/MEM/DEV/CHANGE/LIVE
    align. Next .3.3.3.3 actual lazy bounded source worker/earned read results,then .4 real recipe/
    Design joins. Full graph/currentness/resources/platform/physical and D334/D156/D223/PLANNING.5
    owned. G1 6/18/G0 unapproved;full/hosted79bd6f6a SHA-scoped. D339 captures untouched.
    promotion: declined (private continuation applies the existing bare-root/real-let decision;no new cross-cutting rule or public source-value authority).
## _(2026-10-30 UTC)_ — retain actual scalar continuations without restarting reads

0595 pins original root/context and restores only an unresolved Derived name. Private control keeps completed operands/tasks/selected condition contributions within N reservations. Successful scratch is reusable;failed machines reject restart/resume. Existing adapters retain original causes;no new rounding.
Machine3/expression11+oracle49/recipe9+oracle18/scoped strict lib lint/fmt pass. Real source/proof/supplier execution checks1000/3 unbound,1001 result,actual sin/cos sources,buffer pointers and read order. Initial fixture span API and inline-context enum lint corrected;no hidden failing verdict remains.
Book97/335/1629/2865/glossary310/9/158/0 pass;[whole0594 source plans](docs/history/stitchcad-devnotes-part753.md) retains four exact Git spans/live ids. Private checkpoint only:actual graph worker/earned results/read witnesses pending. G1 6/18/G0 unapproved;full-hosted79bd6f6a SHA-scoped. D339 proofs protected.
Next .3.3.3.3 actual bounded lazy source worker/independent results,then .4 real recipe/Design bridge. Bare roots stay unbound;actual lets store once. Full integration/currentness/resources/platform/physical and D334/D156/D223/PLANNING.5 owned. Full PNT;commit each slice,push400 after gates.
## STITCHCAD-G1-0595 — retain actual scalar continuations and original one-shot behavior

- Pin actual root/context;requeue only unresolved Derived names and retain operands/tasks/conditional contributions. Private lifecycle rejects failed reuse;old readers retain original causes. No public value seed,source cache or new binding round. Book checkpoint/examples and task ownership align.
- Machine3/expression11+oracle49/recipe9+oracle18/scoped lint/fmt/book97/335/1629/2865/glossary0 pass. Whole0594 retained753/four spans/live ids. Next .3.3.3.3 actual graph worker,then .4 real bridge. G1 6/18/G0 unapproved;full-hosted79bd6f6a SHA-scoped. Full integration/platform remains pending.

## D298/D299 — verified glossary/index repair0484

Done;owner G1-SLICE.6.vocabulary.acceptance. [Whole causes/reproductions/repairs](../history/window23.md#stitchcad-devnotes-part639md)
retain obsolete secondary pointer,definition-only use and token/chapter-pair assumptions;focused4/full41 pass0.

## D300 — corrected execution scope0485

Done;owner .6.5.3.2.1. [Whole reproduction/source proof](../history/window23.md#stitchcad-devnotes-part642md) retains the exact original ledger.

## D301 — parent-aware ease conservation before paired effects

P1 paired prerequisite;owner G1-SLICE.6.5.3.2,in_progress before effects. Partition/lineage complete0574:47 native/21 doc scoped checkpoint. Next .5e.2.5.derived.3.3.3.3 lazy worker after0595. Real providers/physical joins .2.4/trusted atomic effects .2.5 remain pending before whole acceptance. Current repair/authority/runtime proof stays owned;local/hosted79bd6f6a stays SHA-scoped.
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

D308–D333 fixed0537–0563;[prior ledger](../history/window24.md#stitchcad-devnotes-part726md). D335–D339 fixed;D339 native chapter search0586 preserves full words/terms,normal warnings retained. Next .5e.2.5.derived.3.3.3.3 actual lazy worker. D334 P2:G0-CONTRACT.15f after G1/before G0;D223/PLANNING.5 open. G1 6/18/G0 unapproved;full/hosted79bd6f6a SHA-scoped.
