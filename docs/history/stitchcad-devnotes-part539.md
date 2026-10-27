# Sealed engineering continuity — part539

- **Identity:** 49 lines, 4019 bytes, sha256:344020475af7410a2f1c465c7a8e7ac1451624b6ac74bcaec9312338da984a21
- **Coverage:** whole D262 task/lesson/ledger, corrected report and retirement proof from b3686da9da02332a029542b98dd6811a863367ec.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 48889–50389, 1500 bytes, sha256:aeb6998eba7c9dd90f24e860e651c2da1e32416a8f469c83895bb9cb66782af3
- Whole source interval: DEV_NOTES.md bytes 193–1072, 879 bytes, sha256:6a6f5577df06b4f2dffd10d81f882afeaa479eb53cdc5396581cbb373e2d830b
- Whole source interval: CHANGELOG.md bytes 207–707, 500 bytes, sha256:f4bae9520f12ea1ed251ef2dc7055709382d4098a67e70091625bafb4633b43f
- Whole source interval: docs/tasks/PLANNING.md bytes 88981–89639, 658 bytes, sha256:823a4db5a7ab174f6df5b32b867203b82c1576ae05276d1f30d9637a100ff169
- Whole source interval: docs/tasks/PLANNING.md bytes 90209–90691, 482 bytes, sha256:f6f6315e8cb55189c51ea0c2a23141ab13bfde4e681dc35b18fff6f6855a36fd
- **Retrieval:** bytes below the first rule and blank line; original terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.6.4.5.3.d.2.a`
  Status: `done`
  Goal:0387 D262 source-pinned v2 append and exact journaled installation; all old bytes/caps fixed.
  - [x] REPRODUCE / ISSUE: transition_receipts.py --record docs/history/stitchcad-devnotes-part538.md
    first refused working65>64 rc=1; independent physical census65 rc=0.
  - [x] ROOT CAUSE (WHY + WHERE): Python AST/source check rc=0: old prepare requires v1/16 windows;
    v2 reader already permits immutable append. The working64 cap correctly refused the new raw file.
  - [x] FIX: v2_migration.py append target/history-v2-append-0387 --owner G1-SLICE.6.4.5.3.d.2.a:
    991 logical/33 backing exact,44 raw packed rc=0; existing v2_install.py install same workspace rc=0.
  - [x] ADDRESSED: v2_install_contract.py target/history-v2-append-0387:17 controls/0 failures rc=0;
    real interruptions/recovery/offline materialization/native tar/ledger routes. Original trees retained.
  - [x] NO REGRESSION: history_archive.sh verify/verify-retention/prove-source window21 rc=0;
    992 logical,3992152 decodedB,22 working;44 source files exact. Reader/installer/old formatter AST
    unchanged rc=0; append missing-owner and prepare-on-v2 refuse1 before workspace effects.
  - [x] LOCKSTEP: run_make.sh book and scoped publication79/159/1454/2409 rc=0;
    transition_receipts.py --record docs/history/stitchcad-devnotes-part538.md:6 exact spans rc=0.
    Promotion: bounded append decision; D263 fixture authority audit remains separately owned.
## _(2026-10-27 UTC)_ — bounded v2 history append

0387 fixes D262: the v1-only preparer could not pack committed raw v2 records at the working64 cap.
Append requires an explicit leaf/fresh workspace and exact committed archive snapshot; all prior
windows and legacy bytes stay fixed.44 raw records become21 through the unchanged journaled installer.
991 prior logical/33 backing bytes exact;17 transaction controls/native tar/offline recovery pass0.
Restored538 yields992 logical/3992152 decodedB/22 working;7848B remains under the unchanged4000000 cap.
Book/publication79/159 and6 Git spans pass0. Promotion: bounded-v2-append section in the history decision.
P1 .6.4.5.capacity owns retention before unsafe growth; P3 D263 helper authority remains a later audit.
No full producer/integration/CI verdict is claimed. Product layer-offset views follow while capacity permits.

## STITCHCAD-G1-0387 — append v2 history within working bounds

- Explicit-owner, source-pinned v2 raw capture; unchanged journaled installation and all old bytes/caps.
-44 source records packed;17 transaction controls/native tar/offline recovery, retention/book checks pass0.
- Live992 logical/3992152 decodedB/22 working; capacity decision owned before unsafe growth. D263 audit deferred.
- Whole0386/range/sewing evidence538; scoped publication79/159 passes0. G1 remains6/18; no full CI claim.

## D262 — bounded v2 append preparer (corrected0387)

New draft538 originally made65 resident Markdown files and transition_receipts.py refused1 at64.
Source inspection: old prepare accepts only v1/16 windows; existing v2 reader already supports append.
Owner G1-SLICE.6.4.5.3.d.2.a restores working capacity through committed raw capture and existing
journaled installation.44 source records packed,17 transaction controls pass0; all old bytes/caps fixed.
Live992 records/3992152 decodedB leaves7848B. Separate P1 owner G1-SLICE.6.4.5.capacity must resolve
retention before unsafe growth. Packing grants no aggregate capacity; no full integration claim.

- [x] ROOT CAUSE: transition_receipts.py --record docs/history/stitchcad-devnotes-part538.md:
  6 exact Git spans rc=0; completed0386/range/sewing evidence needed live retirement.
- [x] ADDRESSED: same command rc=0; original bodies/identities remain whole538;44 other records exact21.
- [x] NO REGRESSION: history_archive.sh verify-retention/prove-source window21:rc=0,44 exactfiles;
  scoped publication79/159/1454/2409 and book rc=0. Old logical/backing bytes and caps unchanged.
