# Sealed engineering continuity — part993

- **Identity:** 40 lines, 4620 bytes, sha256:96b6c5b0c0b689e6ef4c04a7f815f8ffc18cd6796d88d86813f081d37cfbaa01
- **Coverage:** whole0817 task,lesson,ledger and D377/D372 records from 7c2567ef53d46f68d18136f76076049c32536874.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 51551–52802, 1251 bytes, sha256:6010fca415cd2bbb5166c0010e185091b7806e1331ee454ac1586228c49689c0
- Whole source interval: DEV_NOTES.md bytes 193–957, 764 bytes, sha256:b0377c1520c4c453ca90707aef6412f6db8ccd413652c10b152be6bc9177b788
- Whole source interval: CHANGELOG.md bytes 217–1140, 923 bytes, sha256:e30617ba610900a44c2b28bc73aef2f9fef61d1c200a42d5dbfbed52d4257f47
- Whole source interval: docs/tasks/PLANNING.md bytes 93996–94765, 769 bytes, sha256:2aae9b8dd72008ef193605646a00b187a237540e2bb486f3f90878a2837bbc1a
- Whole source interval: docs/tasks/PLANNING.md bytes 92642–93555, 913 bytes, sha256:cfa5db21430f316b70470a07a444fd9fad1a5e7fad53b9b2495d1238a4debaf6
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable;corrections belong in a new record.

---

### G1-SLICE.5f.1.book-search.activate.publication
- ID: `G1-SLICE.5f.1.book-search.activate.publication`
  Status: `done`
  Goal:route Make/publication/capture through local renderer;fix D377 glossary/index navigation checker drift;update directly affected contracts/setup docs without weakening text/route/warning checks.
  - [x] ROOT CAUSE: `rg mdbook` found ambient Make/publication/capture dispatch;`git log -1 -p -S` traces missing index/glossary headings to0788 cd7466d9/0801 bd33add9,rc=0;D377 owned here.
  - [x] ADDRESSED: `scripts/run_make.sh book` and native capture/migration →warning-free local renderer,1093→118 records/118 chapters,85068 word/72522 term pairs exact,123 inputs fixed,payload7729123B,rc=0.
  - [x] NO REGRESSION: launcher `python_publication_builder_contract.py`/`python_search_capture_contract.py` →19/9 body reds,13 refusals;Make/tool4-2/nav2-8/AST3/search5/native warning-repair/publication118-374-1805-3385 pass,rc=0.
    992 four Git spans/body/public exact;archive1446/79,per-generation21/64 and58/64,rc=0;actual queries remain .verify,not latency or runtime approval.
  Commit:STITCHCAD-G1-0817. promotion: declined (existing public renderer/learning-glossary-index policy;D377 repaired without new scope).
## _(2026-11-01 UTC)_ — local renderer selected in normal publication

0817 Make/publication/capture bind local tool/selector/profile hashes before output;native1093→118 records/118 chapters keep85068 words/72522 terms exact0.
Both indexes warning-free;chapter payload7729123B,123 inputs unchanged. Native malformed generic refuses/preserves old bytes;quoted repair0;query proof remains .verify.
D377 traced headings retired0788/0801;actual glossary/index routes now guarded with2 positives/8 refusals and exact remaining AST0;publication118/374/1805/3385 passes0.
Promotion declined: existing renderer/navigation policy;focused builder19/capture9 reds,tool4/2/Make/search5 pass0;992 Git/public exact0,archive1446/79,per-generation21/64 and58/64;G1still6/18.

## STITCHCAD-G1-0817 — activate the local renderer throughout publication

- Make,publication and capture select guarded local mdBook;binary/selector/profile join
  complete input hashes. Setup/readme/book docs use the explicit local install API.
- Native capture1093→118 records/118 chapters retains85068 chapter-word/72522 term pairs;
  both indexes warning-free;chapter payload7729123B,123 inputs unchanged. Actual query proof next.
- D377 fixed:headings retired0788/0801 now checked through actual ordered glossary/index routes;
  two positive/eight refusal cases and exact remaining checker AST0. SUMMARY bounds unchanged.
- Builder19/capture9 body reds,tool4/2,Make/search5/native warning-preservation-repair0;
  publication118/374/1805/3385 passes0;992 Git/public exact,archive1446/79,per-generation21/64 and58/64.
- G1still6/18;D372 actual query verification remains owned;no latency/runtime approval claimed.

## D377 — publication glossary/index navigation contract drift

Fixed0817;owner G1-SLICE.5f.1.book-search.activate.publication. Repro:builder check(ROOT) refused PUBLICATION_PART:# Glossary,then # Index.
Root:0788 cd7466d9 and0801 bd33add9 retired index/glossary headings to stay within120-line SUMMARY;checker still required them. All chapter routes remained;decision_book-progression requires accessible glossary/index,not those exact headings.
Actual glossary must follow learning/before annexes;topic index must be last/after annexes. Annex specification boundary now uses actual index route. Two positive/eight missing/misplaced route/part controls and exact remaining checker AST pass0;native publication118/374/1805/3385 pass0. Frozen SUMMARY bounds retained.
## D372 — large book-search index warning

P2 active;owner G1-SLICE.5f.1.book-search after native feature completion0813. First payload10015553B/JS10030347B;0814 native build rc0 warns12993569B across118 chapters.
Root measured0814:body5318916B/breadcrumbs5321651B despite breadcrumb text10580B;all72206 body term/document pairs leak into breadcrumbs. Published upstream fix b7efca70/elasticlunr-rs3.1.0 isolates field frequencies.
Impact:warning plus incorrect field membership;ranking/latency unmeasured. Build0815 and activation0817 use guarded published local0.5.4/fixed library;normal build now warning-free. Native1093→118 chapter migration retains85068 word/72522 term pairs,payload7729123B. Actual queries remain .verify.
No source patch,chapter exclusion,warning suppression/cap raise or performance certificate. Existing heading-split-level1/full-text policy retained;native features accepted0813.

