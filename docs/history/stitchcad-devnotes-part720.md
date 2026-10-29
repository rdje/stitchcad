# Sealed engineering continuity — part720

- **Identity:** 102 lines, 10313 bytes, sha256:45224af15269223f46f7f09489ea3af42f8c98abc0680ac7906363cdc66be06a
- **Coverage:** whole0561 task scope, lesson and changelog from 3e01ef430aaf422529192943fb585eb0a018f948.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 28826–32834, 4008 bytes, sha256:093a39238eb387088176846b578eb839930fb3b78f1a67afdf32f51b264af962
- Whole source interval: DEV_NOTES.md bytes 193–1398, 1205 bytes, sha256:565654dedd271ae58f3b850e696ffb866f48cce983d0d596a8cab22758e3c964
- Whole source interval: CHANGELOG.md bytes 207–954, 747 bytes, sha256:f02cca3e03d30f11dbc343fb1044b6d7c27d8f8307eb0b7010d3411f3a73f188
- Whole source interval: docs/tasks/PLANNING.md bytes 90576–94929, 4353 bytes, sha256:25adf83d6ec1e8c7786fa3ed938fcccfab343f711542c00ee50786b83df6c08d
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.3.2.prepush-400.archive-client-fixture`
  Status: `done`
  Goal: D331,P1 current prepush blocker;packed_fixture computes a v1 window before checking
  whether any raw records need packing. Current verified v3 authority already packs all three
  fixed D222 records;none are raw. No new window is needed,yet unconditional windows key fails.
  Reproduce:clean407c3f22/derived425,published builder/map entries1–2 pass/source clean;archive
  client entry3 fails KeyError windows before output activation. Log:target/prepush-400-0560-
  environment/03-python_archive_client_contract.log. Public inventory confirms3 logical/no raw.
  Paths:docs/tasks/artifacts/ci_environment/python_archive_client_contract.py. Move only v1
  window/master/manifest construction under existing members condition;retain copied current
  packed authority unchanged when none. Explicitly bound optional raw packing to original v1
  fixture semantics;do not add a v3 archive writer or change record/source pins.
  Verify old failure/new default and actual --real eight provenance/read children,seven
  physical/seven omission/three content controls,whole predecessor preservation/source cleanup.
  Independently compare unchanged client and fixture algorithms outside conditional setup;
  book/live/whole prior receipt sync,commit0561,then remaining six published environment entries
  and fresh full clean prepush/normal push/actual jobs. No physical OS/runtime probe required.

  Completion0561:D331 conditional original v1 setup preserves current packed authority.
  - [x] ROOT CAUSE (WHY + WHERE): frozen407c3f22 packed_fixture computes windows before
    existing members condition. Public v3 inventory confirms all3 fixed records logical/no raw.
    target/archive_client_fixture_0561_focus.py reproduces old KeyError windows before output,
    rc=0;this fixture needs no new window when current production already packs all3 records.
  - [x] FIX: move only master/window/revision/manifest statements under members. Optional raw
    construction requires original v1 authority explicitly;already-packed data stays byte-exact,
    including v3 generation metadata. No generic v3 writer,record/source pin or client change.
  - [x] ADDRESSED (verified): actual prepared archive-client controller default and --real
    both pass original raw-only refusal/complete provenance/eight readonly children/seven physical/
    seven compiled omission/three content refusals/whole retained predecessor proof,rc=0.
    --real performs actual public read/Git/source-proof children;default intercepts archive
    responses. Original five task intervals4144B/followup8170B/whole ledgers3030B remain exact.
    Logs target/archive-client-fixture-0561-{default,real}.log;inputs exact/workspace removed.
  - [x] NO REGRESSION: independent focused whole controller AST equals frozen407c3f22 after
    moving exactly four original setup statements back and removing only explicit v1 guard,
    rc=0. Original client/tracked bytes exact;all3 packed/no raw;controlled non-v1 raw builder
    refuses before activation/no workspace,rc=0,target/archive-client-fixture-0561-focus.log.
    Book93/248/1590/2771,retirement719/four exact Git spans,retention1173 and diff --check pass0.
    No physical OS/runtime diagnostic activated;no whole environment/fresh full prepush claim.
  - [x] LOCKSTEP: whole0560/prior ledger retained719. Book explains preserved packed authority,
    optional original v1 builder and native-versus-intercepted proof scope. MEM/DEV/CHANGE/index/
    ledger align;LIVE G1 6/18/29 suites/G0 human unapproved unchanged,D331 fixed,D156 audits/
    D223/PLANNING.5 pending. Commit then published tail after archive client at reader controller,
    remaining six entries/fresh full clean gates/normal push/actual jobs. Full PNT continues.
    promotion: declined (application of indexed conditional fixture ownership,unchanged native provenance and exact legacy builder scope;no new archive or product authority).

## _(2026-10-29 UTC)_ — skip unnecessary legacy window setup for already-packed fixtures

0561 closes D331:archive-client fixture used v1 windows before checking members. All3 fixed records already packed in valid v3 authority. Move original master/window/revision/manifest setup under existing condition;explicitly bound optional raw builder to v1. Packed bytes stay exact.
Default/native --real each pass eight children/provenance/seven physical/seven omissions/three content controls. Native children use real read/Git/source-proof;fault dispatch remains intercepted. Whole controller AST except moved setup/new guard and actual client/tracked bytes exact0;no residue.
Whole0560/prior ledger retained719/four spans;book93/248/1590/2771 and retention1173 pass0. LIVE G1 6/18/29 suites/G0 human unapproved unchanged;D331 fixed,D156 audits/D223 continuity/PLANNING.5 pending. No whole environment/prepush/hosted verdict inferred.
[Whole0560 continuity proof](docs/history/stitchcad-devnotes-part719.md). Promotion declined:indexed conditional fixture/legacy scope/exact provenance. Next remaining six published environment entries,then fresh full clean gates/normal push/actual jobs;full PNT continues.
## STITCHCAD-G1-0561 — preserve already-packed archive client fixtures

- Close D331:construct original v1 fixture window only if raw members remain;already-packed v3 authority stays byte-exact. Optional raw builder refuses other schemas before activation. Original client/controller algorithms otherwise exact;default/real provenance and all existing refusal controls pass.
- Whole0560/prior ledger retained719/four spans;book/history checks pass. Continue remaining six environment entries at archive reader,then fresh full clean prepush/normal push/actual jobs. D223/PLANNING.5 stay deferred;no whole environment/hosted verdict claimed.
- LIVE G1 6/18/29 suites/G0 human unapproved unchanged;D331 fixed,D156 audits open. Full PNT continues.

## D298/D299 — verified glossary/index repair0484

Done;owner G1-SLICE.6.vocabulary.acceptance. [Whole causes/reproductions/repairs](../history/window23.md#stitchcad-devnotes-part639md)
retain obsolete secondary pointer,definition-only use and token/chapter-pair assumptions;focused4/full41 pass0.

## D300 — corrected execution scope0485

Done;owner .6.5.3.2.1. [Whole reproduction/source proof](../history/window23.md#stitchcad-devnotes-part642md) retains the exact original ledger.

## D301 — parent-aware ease conservation before paired effects

P1 paired prerequisite;owner G1-SLICE.6.5.3.2,in_progress before effects. Ease checkpoint0527/caller0526 scoped. Bounded stop stages/views/exact capture join0529–0535 implemented with focused checks. At derived400 required full gates/push/hosted jobs;then stop component checkpoint/live lineage. Real formula/Profile/geometry,physical/authority/aggregate effects/full proof stay owned.
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

## D308–D331 — prepush repairs

D308–D331 fixed0537–0561;[prior ledger](../history/stitchcad-devnotes-part719.md). D331 conditional v1 setup preserves packed v3 bytes;default/real eight children/provenance/seven physical/seven omissions/three content refusals pass. D223/PLANNING.5 open. Continue six entries after client,then fresh full clean gates/normal push/actual jobs under .6.5.3.2.prepush-400. LIVE G1 6/18.
