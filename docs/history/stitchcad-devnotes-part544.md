# Sealed engineering continuity — part544

- **Identity:** 52 lines, 4402 bytes, sha256:bf349af830c78da8cf5b1d7fa3ce87605ac426e04217cee542f4b6811682bdaf
- **Coverage:** whole0394 client task, lesson, ledger and D269 report from 5a77d104531e43b860bc5a5af09bc100ac5ad34b.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 49223–50793, 1570 bytes, sha256:8323f5527a6477da042a226a6ee7c12106ea774618e7e4658d67a1a3811b8802
- Whole source interval: DEV_NOTES.md bytes 193–1481, 1288 bytes, sha256:3f75c4080abffa0b5bd179aea273ea551484ed0d0f94a17138958879384285e6
- Whole source interval: CHANGELOG.md bytes 207–709, 502 bytes, sha256:96908310896c60268c0f4e9120ffe7e82097d0b95de2e5a9c22acc131e5b4f24
- Whole source interval: docs/tasks/PLANNING.md bytes 89674–90716, 1042 bytes, sha256:f22c3b2047fc697ac701922b04d366a01b9f2520c3985ee2b9e35abba4632279
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.4.5.capacity.apply.accept.clients`
  Status: `done`
  Goal:0394 preserve original refusal predicates on v3; permanent committed-source transaction smoke.
  - [x] REPRODUCE / ISSUE: legacy_refusals.py/window_contract.py fail KeyError windows rc=1;
    native original immutable-catalog assertion detects unrelated missing decoder; D269 logged/owned.
  - [x] ROOT CAUSE (WHY + WHERE): source/AST diagnostic rc=0: v2-only discovery, unused v3 dependency
    in v1 fixtures, new typed guard nodes and source-hardlink masking output-existence predicate.
  - [x] FIX: legacy_refusals.py:28 pass/0 fail rc=0 with original producer/source/population fixed;
    window_contract.py:1011 controls/0 fail,5 windows/996 reads rc=0; typed immutability preserved.
  - [x] ADDRESSED: v3_transaction_smoke.py:27 real interruption/recovery/tar/ledger controls rc=0;
    --identity-only:real byte-identical install/recovery rc=0; live HEAD/archive/ledger inputs unchanged.
  - [x] NO REGRESSION: python_archive_reader_contract.py:996 exact files,15 physical refusals,
    8 actual omission reds rc=0; python_archive_fixture_contract.py:(411,203,9),11 dirs/1077 files rc=0.
    Original predicate AST comparison retains only exact declared adapters, each required once.
  - [x] LOCKSTEP: history_archive.sh verify-retention:997 records/4018435 decodedB rc=0;
    transition_receipts.py --record543:4 exact Git spans0; book/publication/size rc=0. No Rust changes.
    Promotion: history-generations decision. Full-budget resources and feature acceptance remain .resources.
## _(2026-10-27 UTC)_ — generation client predicates and permanent transaction smoke

0394 fixes D269: v2-only window/cohort discovery and an unused v3 dependency blocked original v1
fixtures. Active v3 plans include its decoder; v1 copies do not require it. Original28 native legacy
assertions/source/population remain unchanged. Window1011 controls/996 reads pass0; title/prose
fixtures re-sign only owned metadata so inventory stays valid and committed immutability still refuses.
The reader companion compares original predicates after removing only exact new typed validation
adapters, each required once. Output-hardlink now links a separate fixture file, preserving the
original destination-exists assertion; source-hardlink is a separate refusal. Reader996 exact files,
15 physical refusals/8 compiled omission reds; fixture(411,203,9),11 directories/1077 files pass0.
Permanent smoke freezes current GitHEAD archive/ledger and runs27 real interruption/recovery/tar/ledger
controls under target; live inputs remain exact. Focused --identity-only proves identical-byte real
install/recovery states for no-raw fallback. Four-span543/book/publication/size pass0. Promotion:
history-generations decision. Resource/full-feature acceptance next; deep authority/platform/CI deferred.

## STITCHCAD-G1-0394 — preserve archive client predicates and permanent transaction smoke

- D269 v3 discovery/dependency/AST fixture assumptions corrected; original legacy28/window1011 pass0.
- Reader companion996 exact files/15 physical/8 actual omission reds; fixture(411,203,9) pass0.
- Permanent committed-HEAD transaction27 plus identity install/recovery pass0; all live inputs preserved.
- Whole0393 evidence543 has4 exact Git spans0. Full-budget resource and feature acceptance remain open.

## D269 — generation client schema/dependency assumptions

Owner G1-SLICE.6.4.5.capacity.apply.accept.clients; immediate. Legacy/window native entries fail
KeyError windows on v3; original immutable-catalog assertion then detects an unrelated missing
unused generation decoder in v1 copied fixtures. Discover flat windows without changing predicates;
require decoder only for v3. Both companions and permanent transaction smoke corrected0394.

- [x] ROOT CAUSE: native legacy/window KeyError rc=1; AST diff rc=0 locates added typed guard
  nodes. Source-hardlink fixture masks original output-existence predicate; fixtures now independent.
- [x] ADDRESSED: legacy28/window1011/transaction27 and --identity-only rc=0; original assertions
  and typed immutability intact, exact original/live bytes preserved; new source-hardlink control added.
- [x] NO REGRESSION: reader companion996 exact files/15 physical/8 omission reds rc=0; fixture
  companion(411,203,9) rc=0; retention997/4-span543/book/publication/size rc=0. No deep/full CI claim.
