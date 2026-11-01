# Sealed engineering continuity — part990

- **Identity:** 27 lines, 2615 bytes, sha256:02269cc8e2c57d35c77d4d89750ca40cbf1fc3bb7eeb2402b4aac99cd2621ae0
- **Coverage:** whole0814 task,lesson and ledger from b0ad71d5c4040b6abedcbb4bafc0c79f1f286f5c.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 50606–51924, 1318 bytes, sha256:15690eca47babb666e893b3d80dfb39349b8912205fc678f2d7e76d5c26bc850
- Whole source interval: DEV_NOTES.md bytes 193–873, 680 bytes, sha256:22f43e50fe742ac860cb4fa00c928f018e05a9b87a00f34615d7ad1d0fedf423
- Whole source interval: CHANGELOG.md bytes 217–834, 617 bytes, sha256:c1fbec597c5231b69f866f58b16fd2d5260961009add93c487cbb6315d3be467
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable;corrections belong in a new record.

---

### G1-SLICE.5f.1.book-search.plan
- ID: `G1-SLICE.5f.1.book-search.plan`
  Status: `done`
  Goal:P2 D372 measure current index/build/search contracts and choose bounded full-text/chapter-preserving containment without warning suppression/cap raise.
  Scope:public mdBook API/output contracts,existing source/search policies and actual corpus;no unmeasured latency or performance claims.
  - [x] ROOT CAUSE: `search_index_census.py` through launcher →118 documents,body5318916B/breadcrumbs5321651B;all72206 body pairs in breadcrumbs,rc=0.
    Native `mdbook build docs/book` →rc=0 with12993569B warning;upstream API fix b7efca70 and published3.1.0 metadata confirmed by urllib,rc=0.
  - [x] ADDRESSED: `python3 -I -B -` task/source review →published unchanged mdBook0.5.4 local build,explicit activation and native verification separately owned,rc=0.
    Read-only census added;book/defect ledger record real field leakage and unresolved warning;no corpus/config/ambient-tool change.
  - [x] NO REGRESSION: launcher/publication `search_controls` →5 actual refusals,original unchanged,rc=0;989 three Git spans/body/public exact;archive1443/76,per-generation21/64 and55/64,rc=0.
  Commit:STITCHCAD-G1-0814. promotion: declined (D372 owns concrete upstream defect and bounded resolution;no new product decision).
## _(2026-11-01 UTC)_ — upstream search field leakage measured

0814 D372:118 records,body5318916B/breadcrumbs5321651B versus10580B stored breadcrumb text;all72206 body pairs leak;build0 warns12993569B.
Official b7efca70 fixes field frequencies;published3.1.0 follows mdBook0.5.4. Read-only census0;existing5 native search refusals0;989 Git/public exact0.
Local same-version build,explicit activation and actual search verification separately owned;no source patch/corpus loss/suppression. Ranking/latency unmeasured.
Promotion declined: existing D372 owner;warning remains open. G1still6/18;native13-family checkpoint accepted0813;archive1443/76,per-generation21/64 and55/64.

## STITCHCAD-G1-0814 — trace the large search index to upstream field leakage

- Added read-only native field census:118 records;body5318916B/breadcrumbs5321651B;
  all72206 body pairs also indexed as breadcrumbs despite10580B of breadcrumb text.
- Verified upstream field-frequency fix and published3.1.0;owned local mdBook0.5.4 build,
  explicit activation and actual search verification. D372 remains open;no warning suppression.
- Native build0 warns12993569B;existing5 search refusals0;989 Git/body/public exact,
  archive1443/76,per-generation21/64 and55/64. G1still6/18;native feature checkpoint unchanged.

