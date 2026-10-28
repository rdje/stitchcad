# Sealed engineering continuity — part634

- **Identity:** 24 lines, 3085 bytes, sha256:a8c38fd98a7e7f55fb16a15002dcae33a0ea779c3082a0d1928187eb43d2a597
- **Coverage:** whole0479 task scope, lesson and changelog from 59189cd00819344c815cb1a425673404b82fbd10.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 34317–35779, 1462 bytes, sha256:b5c91cfb4514dddb6cb44db13f38922c356922e3bf71e5a323adf3abaa4df7d7
- Whole source interval: DEV_NOTES.md bytes 193–1260, 1067 bytes, sha256:3c7896e7336f4feeae24ed6ee79a4aebf3a482923debc9b76db82c8604933167
- Whole source interval: CHANGELOG.md bytes 207–763, 556 bytes, sha256:39d5ad3b6c4e9c28b6c3eb3895ae0aeb0c31b0c494729765bd581b8812fd2bc2
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.vocabulary.1`
  Status: `done`
  Goal: declare command/authority/evaluation/artifact/release API and variant meanings in five chapters.
  - [x] REPRODUCE / ISSUE: bash docs/tasks/artifacts/glossary/run_glossary_census.sh →140 C1 token/use rows rc=1;all other rules clear.
  - [x] ROOT CAUSE: same census/current Rust declaration review →43 rows in five command chapters lack table declarations rc=0;types/variants exist and remain unchanged.
  - [x] FIX: add chapter-local meanings quoted from public core contracts;no API/runtime,global exemption or census changes.
  - [x] ADDRESSED: same census →97 C1 rows rc=1 expected;Python exact set difference →43 resolved/0 new rows rc=0;all five chapter populations clear.
  - [x] NO REGRESSION: Python copied declaration-removal controls for CommandRequest/CommandAuthoritySnapshot/ArtifactPayload/SizeMemberTarget/BlankAlgorithm →5 named refusals rc=1 expected,driver rc=0;mdbook/publication →86/161/1549/2620 rc=0.
  - [x] LOCKSTEP: MEMORY/DEV/CHANGELOG/decision/index/D250 updated;LIVE reviewed unchanged G1 6/18,D250 owned;README unchanged. transition_receipts.py --record part633 →3 exact Git spans rc=0;history_archive.sh verify-retention →1087 records/2450002 residentB rc=0.
  Verification: focused declaration delta/copied controls/book/retention pass;remaining97 and full glossary acceptance still owned.
  Commit: STITCHCAD-G1-0479;next .vocabulary.2 size-set vocabulary.
## _(2026-10-28 UTC)_ — command vocabulary declarations

0479 .6.vocabulary.1 remeasures D250:140 token/use rows across13 chapters,not unique defects. Scope review quotes existing Rust API names/variants;five command-family chapters lacked table declarations.
Declare exact core input,scope,authority,evaluation,artifact and release meanings locally. No API/runtime change or census exemption. Exact set delta resolves43 rows with0 new;97 remain in size-set/formula/local chapters.
Five actual copied declaration removals refuse their named token;driver passes0 over existing97 baseline. mdbook/publication86 chapters/161 API rows/1549 source/2620 rendered links pass0. Full glossary acceptance stays pending.
Promotion:indexed machine-token decision;whole0478 scope/lesson/changelog633 verified3 exact Git spans. Retention1087/2450002 residentB pass0. LIVE reviewed unchanged:G1 6/18,D250 owned;README unchanged. Next .vocabulary.2.
[Whole0478 publication acceptance](docs/history/stitchcad-devnotes-part633.md) retains full45 controls and exact feature boundary.
## STITCHCAD-G1-0479 — declare actual command API vocabulary

- Declare existing command,scope,authority,evaluation,artifact and release API/variant meanings in five owning chapters;no API or runtime change.
- Current glossary census140→97 token/use rows:43 resolved,0 new. Five copied actual declaration-removal controls refuse by name;full glossary acceptance remains pending.
- Focused book build/publication86 chapters/161 API rows/1549 source/2620 rendered links pass. Whole0478 scope/lesson/changelog retained633;continue size-set declarations.

