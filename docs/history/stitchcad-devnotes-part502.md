# Sealed archive — whole0350 and historical Ease proofs

Immutable historical segment, sealed by leaf `G1-SLICE.6.4.2.d.2.b` on `2026-10-27` (UTC).

- **Sealed identity:** 75 lines, 6228 bytes, sha256:9c0404d2389c7f3f4a7a91ebe1f9a6b2a0bdd5ddbd8c2bf347007c9466f761fe
- **Coverage:** complete committed measurement task/report/lesson/ledger and old Ease book proof section from 89b867e8a73874ad41ce1abb70297fae6bf77ffd.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 47909–49560, 1651 bytes, sha256:164db155eb37fafd97cc3175a99112469e848e168d0ca3498290c1d0da066e1e
- Whole source interval: docs/tasks/PLANNING.md bytes 86637–87237, 600 bytes, sha256:be86489eef3d768e6764bbe9c14f98ce1c8b7bb80530907fdaef76ac731d4282
- Whole source interval: DEV_NOTES.md bytes 193–1324, 1131 bytes, sha256:8c50de1585b0b3f569ed2b35259dfcff8aa2c90816078a0650e4c299b0f95430
- Whole source interval: CHANGELOG.md bytes 207–842, 635 bytes, sha256:468014cfdfe680d1a3ae63efba9c40be2b4b079fe4417aa88ffa6967d8b2d030
- Whole source interval: docs/book/src/spec/ease-inputs.md bytes 8952–11108, 2156 bytes, sha256:f19c5137a2af38645b7849fadf44b04e03971712add54d226a1a07cc9ac86ca0
- **Retrieval:** bytes below the first rule and blank line; original source terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.6.4.2.d.2.a`
  Status: `done`
  Goal: immutable MeasurementInventory owns exact metadata/tables; existing contexts check all current
  targets and cross-family ids. Canonical values stay borrowed; no global Design/source-truth/readiness
  claim. Own focused checks/docs, D254 package description and complete0349 continuity501.
  - [x] REPRODUCE / ISSUE: actual inventory tests expose stale local proofs after metadata/context edits;
    measurement_inventory_contract gives8/0, rc=0, target/measurement-inventory-native-0350.log.
  - [x] ROOT CAUSE (WHY + WHERE): existing contexts borrow records and tables pin bindings. Same-token
    peers cannot repair old bindings; native stale-token/domain/missing-target cases pass8/0, rc=0.
  - [x] FIX: private exact vectors, shared borrows and ordered identity/current-reference checks reuse
    actual validators; native8/0 confirms no value cache, unit reconversion or nested self-reference.
  - [x] ADDRESSED: python3 -I -B scripts/local_environment.py -- cargo test -p sc-measure --test
    measurement_inventory_contract passes8/0, rc=0, including unused metadata and scalar-id collisions.
  - [x] NO REGRESSION: privacy2/strict selected lint/fmt/book/publication77/109/1444/2360 pass0,
    target/measurement-inventory-*-0350.log; D250132 exact baseline rows, delta0, glossary rc=1.
  - [x] LOCKSTEP: D254 cargo metadata/book/API/status/index/pointer aligned, rc=0; continuity501
    compares3 Git spans0. Caps unchanged; G1still6/18. promotion: declined (canonical-source/table rules).
    Commit0350; Ease .2.d.2.b next. Full host/Design/readiness/bus/integration/CI remains later.
## D254 — measurement package description still defers implemented structural SizeSet

Reproduce: sc-measure/Cargo.toml description says "full SizeSet follows"; its lib.rs exports SizeSetInputs,
GarmentSizeSetInputs and MtmSizeSetInputs with implemented structural contracts. Impact: package metadata
understates current structural composition. Owner G1-SLICE.6.4.2.d.2.a, P2 fixed0350: actual owned
metadata/structural inputs reported; numeric execution/full integration stay pending. cargo metadata
--no-deps --offline confirms description, rc=0, target/measurement-inventory-metadata-0350.log.

## _(2026-10-27 UTC)_ — owned current measurement inventory

0350 MeasurementInventory owns exact landmark/procedure/measurement/table vectors in sc-measure.
It checks identities across both canonical families and owned metadata, then reuses every actual
Measurement/Table current validator. Unused metadata still needs targets; stale table bindings never
transfer to same-token peers. Table token scope stays local. Canonical values/states/source remain
borrowed, including same-id edits and original entered units. Contexts are built as borrowed views;
no nested self-reference is stored. Empty drafts accept; shared access grants no mutator.
Native8/privacy2/strict selected lint/fmt/book/publication pass0. D254 package description matches actual
owned metadata/structural SizeSet inputs via cargo metadata, rc=0. D250132 glossary rows remain baseline,
delta0; no full census pass. Whole0349 continuity501 compares3 Git spans. Promotion: existing canonical
source/table rules. Caps unchanged; G1still6/18. Ease/set composition .6.4.2.d.2.b follows; full host/
Design/source truth/readiness/bus/integration/CI remains later.

## STITCHCAD-G1-0350 — own current measurement metadata and tables

- Add immutable MeasurementInventory over exact landmarks/procedures/measurements/tables and borrowed canonical inputs.
- Check cross-family ids and every current target/binding with actual existing validators; preserve table-scoped tokens.
- Native8/privacy2/strict selected lint/fmt/book/publication pass0; D254 package metadata corrected and checked.
- D250132 glossary rows exactly baseline; continuity501 matches3 Git spans; caps unchanged. G1still6/18.
- Ease/set composition .6.4.2.d.2.b next; full host/Design/readiness/bus/integration/CI remains pending.

## Verification boundary

The tracked individual-Ease tests exercise ordered fit classes, canonical borrowing, required side
kinds, distinct amount identity, current identity/binding/metadata changes, signed permission in all
value-bearing states, unknown/derived refusal, same-token namespaces and reordered current inputs.
The individual mutation diagnostic declares all seven numbered logs and restored.log, its mutable
implementation and read-only current test source before input reads or native dispatch. Six pure
cases plus the unchanged source-dependent span and complete alias predicate yield seven faults.
Independent complete dispatch captures, output symlinks, mutable-source hard links, missing-test
refusals and actual guard/classifier omissions are verified separately from native results.
Each real removal must fail at a current guarded test assertion macro's UTF-8 byte location.
All eight frozen build/source inputs and both Rust files remain exact after restoration; the
current artifact selected through public Cargo output passes all thirteen tests. Rust strict checks
and the browser-target build verify the library integration. These are local structural checks;
physical fit and production signoff remain G2/G3/G4/G6 and independent G7 obligations.

Set contracts add per-POM/id/token lookup, three uniqueness rules, shared body/amount sources, mixed
tables, empty drafts, current context collisions, missing/reassigned mappings/tables, current intent/
state edits and selected versus full validation. The set diagnostic plans ten numbered logs and
restored.log, two inputs and one mutable implementation before reads. Its eight pure cases and
six ordered source-dependent statements preserve all ten faults. Current inline assertion macros
use UTF-8 byte columns; independent ASCII/multibyte controls and a real leading-only helper fault
verify that boundary. Complete dispatch/physical refusals are separate from ten real current macro
failures. Exact frozen inputs and both Rust files are restored; the current public Cargo artifact
passes all fourteen tests. These local checks do not claim observed remote CI for this slice.

End of whole0350 and historical Ease proof continuity.
