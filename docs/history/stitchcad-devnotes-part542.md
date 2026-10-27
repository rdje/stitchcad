# Sealed engineering continuity — part542

- **Identity:** 63 lines, 5599 bytes, sha256:be63d3c195cb80b3ac7372d4885e63f3056e40c49d381b7853c4c283838840bf
- **Coverage:** whole0392 cutover task, lesson, ledger and D263–D267 report from b951604787effce59a3e3cac0781e4bd264453a7.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 48733–50358, 1625 bytes, sha256:ec2b018f34715d8d7987301a1da36069b535a5330cc650d708c5464f0ef9f469
- Whole source interval: DEV_NOTES.md bytes 304–1607, 1303 bytes, sha256:ad6bef91fd72c0e820d9a6e6005acf17050cff63a3c5f756f684362954812ef3
- Whole source interval: CHANGELOG.md bytes 207–818, 611 bytes, sha256:dece308ef3264abe60f84cc32601b3d6702cf91eb0a5e78af5e658557b243dbd
- Whole source interval: docs/tasks/PLANNING.md bytes 89293–91353, 2060 bytes, sha256:5152eb851663d419930bab069d1617e503040a12982e114684f44e0ef30f6f5f
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.4.5.capacity.apply.cutover`
  Status: `done`
  Goal:0392 strict v3 decoder, source/immutable guards and source-pinned journaled cutover.
  - [x] REPRODUCE / ISSUE: generation_budget.py:3999898B/102B headroom rc=0; next full retirement blocked.
    D266 normal owner81 refused1 after effects; D267 -O missing owner admitted0 with effects; Root unchanged.
  - [x] ROOT CAUSE (WHY + WHERE): source/Git inspection rc=0; global budget coupled to single generation;
    D263 helper/source plans, D264 installer identity, D265 audit binding, D266/267 runtime guards absent.
  - [x] FIX: generation_budget.py --self-test:25 budget+4 runtime controls rc=0; all27 assertions replaced.
    Old published v2 formatter output byte comparison rc=0; source hash/old decoder AST comparisons0.
  - [x] ADDRESSED: v3_reader_contract.py target/history-generations-0392f/candidate:46 physical controls0;
    v2_install_contract.py target/history-generations-0392g:26/0 rc=0, real interruptions and2 body omissions.
    v2_install.py install same workspace rc=0; all original files/backup trees byte-exact; master sealed1.
  - [x] NO REGRESSION: v2_reader_contract.py:22/0 rc=0; original v1/v2 decoders AST unchanged0;
    v3 profile-free API/Git-free CLI, native tar/tree hashes and full materialization exact. No Rust changes.
  - [x] LOCKSTEP: book/publication/size and history_archive.sh verify-retention rc=0;
    transition_receipts.py --record docs/history/stitchcad-devnotes-part541.md:8 exact Git spans rc=0.
    Promotion: decision_history-generations.md. Authoring/current-client/resource acceptance remains .accept.
## _(2026-10-27 UTC)_ — source-pinned v3 generation cutover

0392 installs two bounded generations, retaining every994 prior record/33 opaque backing and all
physical bytes, including the old v2 master as generation1. Whole physical tree hashes protect seals;
ordinary reads remain profile-free/Git-free. Source HEAD/code/audit/journal and both tree images are pinned.
D263 plans/ledger schema, D264 self-installer pin and D265 audit binding/bounded journal corrected.
D266 owner81 late effects and D267 -O missing-owner admission were reproduced without Root changes;
27 producer assertions became unconditional checks. Budget25/runtime4, reader46, transaction26 and v2
reader22 pass0; actual self-hash/journal omissions admit faulty fixtures, proving the controls discriminate.
A shared mutable fault map had re-signed a negative case; independent descriptor comparison identified
fixture aliasing, then immutable case snapshots fixed it. Old decoder AST/formatter bytes remain exact.
Original trees stay target/history-generations-0392g/installation/backup; later541/HEAD drift rightly
refuses old recovery.541 retains8 exact Git spans. Promotion: history-generations decision. Authoring,
current-client/permanent smoke/full-budget resource acceptance follows; full platform/CI proof is deferred.

## STITCHCAD-G1-0392 — install byte-exact bounded history generations

- Strict v3 index/tree seals; every994 prior record/33 opaque backing and all source bytes retained.
- Reader46/transaction26/v2 reader22 pass0; real interruptions, native oracles, Git-free/profile-free reads.
- D263–D267 plans/hash/journal/early-owner/optimization guards repaired; actual omission/admission controls.
- First new continuity541 safely stored; whole0391/proposal/0389/D263/retirement retained in8 exact spans.
- Authoring, current clients and full-budget resources remain immediate work; G1still6/18, full CI deferred.

## D263–D267 — cutover source/guard repairs0392

D263: installer fixture omitted helper/CHANGELOG/producer/sidecar and complete child sources. Corrected
lists, frozen-input preservation and ledger-route equality pass in v2_install_contract.py:26/0 rc=0.
Deep whole-producer authority calibration remains its existing .f.fixture-verification owner after .6.9.
D264: preparation never pinned installer code. Git7ad3853 inspection confirms missing installer_sha256;
zero current installer hash now refuses before transaction creation. Actual compiled self-hash omission
installs the faulty owned fixture, calibrating the refusal. Corrected; same26-control producer rc=0.
D265: the journal omitted the full audit and read unbounded bytes. Exact audit hash now binds both
identity lists; whitespace audit drift and8193B journal refuse before recovery changes. A compiled
binding omission accepts the changed audit and rolls back, proving discrimination. Corrected;26/0 rc=0.
D266: owner validation admitted81 characters while manifests bound80, creating a workspace before
refusal. D267: Python -O removed27 assert guards; missing-owner generations returned0 with effects.
Reproduced target-only; original Root bytes unchanged. All producer guards now unconditional, owner
bound80 checked before profile/effects. generation_budget.py --self-test:25 budget+4 runtime controls0;
normal/optimized missing/long owners refuse1 with no workspace. Registered controls; corrected.
All owned by G1-SLICE.6.4.5.capacity.apply.cutover. No complete integration/platform/CI claim.

- [x] ROOT CAUSE: transition_receipts.py --record docs/history/stitchcad-devnotes-part541.md:
  8 exact Git spans rc=0; completed proof and original D263 report occupied live capacity.
- [x] ADDRESSED: same command rc=0; original task/lesson/ledger/report/retirement bytes retained541.
- [x] NO REGRESSION: history_archive.sh verify-retention rc=0; reader46/transaction26/v2 reader22 pass0;
  scoped book/publication and live-size checks rc=0. Old physical files and both backup trees exact.
