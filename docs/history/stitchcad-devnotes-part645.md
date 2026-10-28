# Sealed engineering continuity — part645

- **Identity:** 30 lines, 3678 bytes, sha256:922bc48f2eb457afc17ee429436e476b0cb667c26ad831dbc269e3d88e17a9e6
- **Coverage:** whole0490 task scope, lesson and changelog from 55ec1a18774ff7932f34bd3ff5a5c84eecfd2d34.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 40174–42038, 1864 bytes, sha256:360dfaecd3add366d05066937136e7d13d64b1b724e321317f3035c2f311a106
- Whole source interval: DEV_NOTES.md bytes 193–1311, 1118 bytes, sha256:2aa5a508b36243323494cec5022b4ee24390e3ab49384d44c573a03768b713e3
- Whole source interval: CHANGELOG.md bytes 207–903, 696 bytes, sha256:755efec130e5703f8ee763bda79d8121f2caeb0cf8a5305f3cb5d7f0d85be2ca
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.3.2.retention`
  Status: `done`
  Goal:0490 D302 repeated D284 aggregate/active count confusion corrected;no archive packing needed.
  - [x] REPRODUCE / ISSUE: python3 -I -B docs/tasks/artifacts/history_archive/generation_pressure.py
    →before644:generation1=21,generation2=43,total64;after644:21+44=65,20 active working slots,rc=0.
  - [x] ROOT CAUSE: same diagnostic →independent physical/catalog census agrees with each validated group,rc=0.
    Earlier aggregate verify-retention count was compared with one group's64 ceiling despite public64/group and192 aggregate contract;D284 recurred in0489 prose.
  - [x] FIX: tracked read-only pressure/census producer with scope controls;correct current pointer/index/ledger/book.
    Preserve exact erroneous0489 in sealed644;no packing,rollover,deletion,cap increase or archive implementation change.
  - [x] ADDRESSED: python3 -I -B docs/tasks/artifacts/history_archive/generation_pressure.py --self-test
    →3 named controls:aggregate64/active43,aggregate85/active64,next active44;rc=0. Real producer →65 aggregate/44 active,rc=0.
  - [x] NO REGRESSION: bash scripts/history_archive.sh verify-retention →1098 logical/65 workingMD/2502007 residentB,rc=0;
    transition_receipts.py --record docs/history/stitchcad-devnotes-part644.md →3 exact Git spans,rc=0.
    mdbook build/publication check →88 chapters/168 APIs/1559 source/2656 rendered links,rc=0;glossary census →0 failures,rc=0.
    Independent pressure rerun SHA census →all archive/control files byte-identical,rc=0;full archive/CI campaigns remain pre-push/integration owners.
  - [x] LOCKSTEP: MEMORY/DEV/CHANGELOG/indexed generation decision/book/task index/defect ledger updated;
    LIVE reviewed with D302 correction noted,G1 6/18 unchanged. README unchanged;next .3.2.3 canonical original source/provenance.
## _(2026-10-28 UTC)_ — active versus aggregate archive pressure

0490 D302 repeats D284:0489 prose compared64 aggregate working files with a per-generation64 cap. Public contract says192 aggregate;actual before644=21 sealed+43 active. No storage action was needed or taken.
Tracked read-only generation_pressure.py validates each group and independently counts physical raw/catalog files.
Scope controls3 distinguish aggregate64/active43,aggregate85/active64,next active44;all pass0. After exact whole0489 record644,real readout21+44=65,20 active slots;retention1098/2502007 residentB passes0. Original seals/backups/caps untouched;current pointers corrected. Working slots do not waive other resource bounds.
Promotion:indexed generation decision. Book/publication/glossary focused checks recorded in owning leaf. LIVE D302 correction noted,G1 6/18 unchanged;README unchanged. Product resumes canonical original ease source/provenance .3.2.3,D301 P1 still open.
[Whole0489 uniform-source lesson](docs/history/stitchcad-devnotes-part644.md) preserves the exact native allocation proof and erroneous pressure claim.
## STITCHCAD-G1-0490 — distinguish active archive pressure from aggregate counts

- Fix D302 recurrence of D284:64 aggregate working files did not require packing;before644 active generation used43/64. Add read-only validated group pressure and independent physical/catalog census with3 scope controls.
- Preserve whole0489 exact in644;afterward65 aggregate/44 active,20 working slots. Retention1098/2502007 residentB passes0;no packing,deletion,seal rewrite or cap change. Correct current pointers and resume D301 canonical provenance.
- Focused book/publication/glossary checks pass;LIVE D302 correction noted,G1 progress6/18 unchanged. Full archive/CI integration remains separately owned.

