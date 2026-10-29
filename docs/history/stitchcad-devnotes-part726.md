# Sealed engineering continuity — part726

- **Identity:** 98 lines, 10152 bytes, sha256:75c1222269e9423cf45a1355c9251939268e68e6154dfe97ad42b30ef7654735
- **Coverage:** whole0567 task scope, lesson and changelog from 2b1dde6058df0398e9b3cde196cbe9e992179470.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 23576–27452, 3876 bytes, sha256:8be6229db35e84e7b501cce50251452e2821314613bc07989fba3dcec07ef9b0
- Whole source interval: DEV_NOTES.md bytes 193–1507, 1314 bytes, sha256:23c9ecc87adf87e80e6423fae8619bb1fa193fe33b40a0ad628346f16e0346f1
- Whole source interval: CHANGELOG.md bytes 207–862, 655 bytes, sha256:16321a3b816590c02f09b5ef1085f72296cec43e87cf9fd5c647b50062eb2e5d
- Whole source interval: docs/tasks/PLANNING.md bytes 90576–94883, 4307 bytes, sha256:cc999e2e9d6882ef0f19cb566b0b70e42dd971089be4a3cde6380d4e4197cdf4
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.3.2.3.4.1`
  Status: `done`
  Goal: bounded immutable ordered span identity history. Explicit Root births/Split(parent,two
  A-ordered children);each ID born once,only a currently live parent may split. Retired lookup keeps
  immediate children even after nested splits,never silently selects a live descendant. No allocator,
  clock,geometry/value/current graph check or command capability;records are validated supplied data.
  Explicit positive entry/issued-identity limits;count complete input before validation-map allocation.
  Validate parent first,then children in authored order;preserve original entry/identity/first birth/
  prior split in typed errors. Private immutable owner retains original entries;fixed Debug and
  allocation-free local state lookup. Empty histories valid under explicit limits;no default policy.
  Paths:crates/sc-core/src/ontology/seam_span_history.rs and mod.rs;focused public contract test,
  command-seam-lineage book/SUMMARY/scoped API map and live/task docs. Focus nested/sibling states,
  quota/refusal precedence/nonreuse plus independent chain identities;privacy/API doctests and scoped
  lint/fmt. No broad CI or whole paired mutation campaign. Graph integration remains next .2.
  Completion0567:bounded immutable supplied span birth/split history implemented.
  - [x] ROOT CAUSE (WHY + WHERE): roadmap4.1/nonreuse and paired .3.4 require explicit old
    parent/two-child identity history;actual SewingGraph live spans/retained-root ownership is
    separate integration. Native seam_span_history_contract →5 actual cases establish supplied
    ordered data/nonreuse/quota/state contracts,rc=0;no current graph or global issuance inferred.
  - [x] ADDRESSED (verified): local_environment.py -- cargo test -p sc-core --test
    seam_span_history_contract →5 pass/0 fail,rc=0. Exact nested record borrows/unrelated roots/
    unknown/empty/full128-bit IDs,complete entry/issued quotas,parent/child/retired reuse priority
    and independent chain depths1/2/17/32/257 preserve immediate children and sibling leaves.
    All complete counts precede map allocation;private owner keeps original entries and explicit
    limits;lookup allocates nothing and never silently returns a chosen live descendant.
  - [x] NO REGRESSION: cargo test -p sc-core --doc seam_span_history →four pass/0 fail,
    rc=0:actual public example/private owner/no Default/borrow lifetime. Scoped cargo clippy
    -p sc-core --lib --test seam_span_history_contract -- -D warnings and cargo fmt --all
    -- --check pass,rc=0. Initial format check requested export order;corrected and final scoped
    lint/format pass. Book example byte-exact to compiled doctest;actual glossary310/9/158/0 and
    positive book94/253/1593/2784 pass,rc=0. Initial publication check named missing topic-index
    entry;added it and positive check passed without changing checker rules. Broader graph/
    physical/command/full paired verification remains deferred to its separate owners.
  - [x] LOCKSTEP: new history book/SUMMARY/topic index/five scoped API rows and MEM/DEV/CHANGE/
    LIVE/index/D301 align. Whole0566/prior ledger retained725/four Git spans. G1 6/18/human G0
    unapproved remain;old full local/hosted79bd6f6a verdict stays exact-SHA scoped. Next actual
    graph/Design owner/resource integration .3.4.2,then candidate/reference joins .3.4.3;
    no graph installation,ID allocator,provider value,physical match or bus command effect.
    Retention1179 and receipt725/four exact Git spans pass,rc=0. Live-doc proposal guards refused
    MEMORY51 lines/DEV328-byte line before tracked writes;tightened to existing caps. No cap change.
    promotion: declined (application of indexed immutable identity/nonreuse,explicit logical admission and metadata-versus-current-source boundaries;canonical APIs are retrievable in the book).

## _(2026-10-29 UTC)_ — retain ordered span identity births and retirements

0567 implements bounded immutable Root/Split history:complete entry/issued count before map allocation,parent live before child nonreuse,all retired IDs retained. Fixed Debug and allocation-free state lookup lend immediate children from the actual record without descendant selection. No global/current graph authority.
Native5 covers exact nested pointer/order/full128-bit/empty/quota/refusal causes and independent chain depths1/2/17/32/257. Doctest4/scoped strict lint/fmt pass;initial export order corrected. Book example exactly matches compiled snippet. Glossary310/9/158/0 and positive book94/253/1593/2784 pass.
Initial publication guard named missing topic-index entry;added it and passed with checker unchanged. Whole0566/prior ledger retained725/four spans;book/index/five API rows/live docs align. LIVE G1 6/18/G0 human unapproved;old full/hosted79bd6f6a verdict scoped. D334 P2/D156/D223/PLANNING.5 owned.
[Whole0566 stop proof](docs/history/stitchcad-devnotes-part725.md). Promotion declined:indexed immutable identity/explicit admission/currentness boundaries. Next actual graph/Design ownership/resource integration,then paired candidate/held reference joins;physical/authority/effects/full proof stay owned. Full PNT.
## STITCHCAD-G1-0567 — retain bounded span identity histories

- Add immutable supplied Root/Split histories with explicit complete input/issued-ID limits,typed chronological refusals and borrowed live/retired lookup. Never reuse a retired ID or silently select a descendant. Native5/doctest4/scoped lint/fmt and independent chain expectations pass.
- Book94/253/1593/2784 and glossary310/9/158/0 pass;five API rows/index/whole0566 record725 retained. G1 6/18/G0 human unapproved;D334 P2/D156/D223/PLANNING.5 owned. Next actual graph/Design owner/resource binding;global freshness,current sources,physical/effects and full paired proof remain pending.

## D298/D299 — verified glossary/index repair0484

Done;owner G1-SLICE.6.vocabulary.acceptance. [Whole causes/reproductions/repairs](../history/window23.md#stitchcad-devnotes-part639md)
retain obsolete secondary pointer,definition-only use and token/chapter-pair assumptions;focused4/full41 pass0.

## D300 — corrected execution scope0485

Done;owner .6.5.3.2.1. [Whole reproduction/source proof](../history/window23.md#stitchcad-devnotes-part642md) retains the exact original ledger.

## D301 — parent-aware ease conservation before paired effects

P1 paired prerequisite;owner G1-SLICE.6.5.3.2,in_progress before effects. Ease0527/stop component0566 complete with scoped actual callers;full prepush/pushed79bd6f6a local+hosted checks passed0564. Bounded history0567 implemented;next graph/Design owner/resource integration .3.4.2. Real formula/Profile/geometry,physical/authority/aggregate effects/full acceptance remain owned.
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

## D308–D334 — prepush repairs

D308–D333 fixed0537–0563;[prior ledger](../history/stitchcad-devnotes-part725.md). Full clean/pushed79bd6f6a checks pass0564;cleanup0565/stop checkpoint0566 complete. Bounded history0567;graph integration next. D334 parse-only verdict P2:G0-CONTRACT.15f,after current G1/before G0 closure. D223/PLANNING.5 open. LIVE G1 6/18;G0 human unapproved.
