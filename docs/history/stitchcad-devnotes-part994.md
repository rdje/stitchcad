# Sealed engineering continuity — part994

- **Identity:** 32 lines, 3271 bytes, sha256:d119b4ce33e11776a97a607372a823897eaf15dfc9582390ecb60892604309b0
- **Coverage:** whole0818 task,lesson,ledger and D372 closure from 0923a472e350a99dfad083516d0bd6711c64346b.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 51794–52964, 1170 bytes, sha256:3c4679a93465d17bd244746c6158a8bfbe3769b58e87c38025e9e28e52a06993
- Whole source interval: DEV_NOTES.md bytes 193–971, 778 bytes, sha256:6303c4ceae7a95b71519611e262607cc5b2b6257c65315577ef5ddb41d966fc3
- Whole source interval: CHANGELOG.md bytes 217–1076, 859 bytes, sha256:c14bf53496b1a0e9a015310a7520d642a1f93c82e22b1875381961a0abbb8a12
- Whole source interval: docs/tasks/PLANNING.md bytes 92642–93106, 464 bytes, sha256:1f91ccd0f96a27c6f6bfcb7b37acf82238aac102486222789a600b31ed3c89e8
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable;corrections belong in a new record.

---

### G1-SLICE.5f.1.book-search.verify
- ID: `G1-SLICE.5f.1.book-search.verify`
  Status: `done`
  Goal:after implementation verify actual full-text/chapter search and warning-free build against original corpus;preserve existing public behavior.
  - [x] ROOT CAUSE: `rg search_data` scoped publication reader validates data without JS;`node --version` →v26.8.1,rc=0;actual query/landing proof is the remaining declared D372 boundary.
  - [x] ADDRESSED: launcher `search_query_contract.py` →14 native cases/118 landings/4 data refusals,current/captured chapters pass,rc=0;poisoned Node options/cache/coverage removed and section profile refused,rc=0.
    Native capture/migration →1094→118 documents,85167 chapter-word/72600 term pairs exact;both warning-free,chapter payload7736437B,rc=0;no UI/ranking/latency approval.
  - [x] NO REGRESSION: `check(ROOT)` →118/374/1806/3387,source census370,runner syntax and five search-data refusals pass,rc=0;993 five Git spans/public/archive retained,1447/80,per-generation21/64 and59/64,rc=0.
  Commit:STITCHCAD-G1-0818. promotion: declined (existing full-text/chapter and public-runtime verification policy;D372 closed).
## _(2026-11-02 UTC)_ — actual native book search checkpoint

0818 D372 closes:generated elasticlunr engine14 queries/118 landings/4 data refusals pass0,current/captured chapter indexes;case/prefix/Bézier/empty/no-match covered.
Node26.8.1 is a documented read-only host dependency;stdin only,cache/options sanitized,poisoned flags create no cache/coverage paths0;section profile refuses0.
Final native1094→118 migration85167 words/72600 terms exact0;both builds warning-free,chapter7736437B;publication118/374/1806/3387/source census370 pass0.
Promotion declined: existing public-runtime/search policy;native13-family parent closes with D372,actual ordered/frame integration .5f.2 next. 993 five Git/public exact0,archive1447/80,per-generation21/64 and59/64;G1still6/18.

## STITCHCAD-G1-0818 — verify real book search queries and close D372

- Generated engine/index passes14 real queries across118 chapter landings plus four
  result-refusal controls. Case,prefix,accented term,body/title and empty/no-match cases pass.
- Documented read-only Node22.8+ prerequisite;stdin assets,no packages/cache/browser profile;
  actual poisoned options/cache/coverage remain absent. Section indexes refuse chapter profile.
- Native full-text migration1094→118 retains85167 word/72600 term pairs;both builds warning-free,
  chapter payload7736437B. Publication118/374/1806/3387/source census370 pass0;runner watches queries.
- 993 five Git/public spans exact;archive1447/80,per-generation21/64 and59/64. D372/native declaration
  parent closed;actual ordered/frame/dependency integration next. G1still6/18;no UI/latency/ranking approval.

## D372 — large book-search index warning

Fixed0818;owner G1-SLICE.5f.1.book-search.verify;[whole cause/prior checkpoints](../history/stitchcad-devnotes-part993.md).
Published local0.5.4/fixed3.1.0 keeps full text/chapter coverage and warning refusals;native14 queries/118 landings/4 result faults plus exact1094→118 migration85167 words/72600 terms pass0. Browser UI/ranking quality/latency remain unclaimed;no cap raise,excluded chapter or upstream patch.

