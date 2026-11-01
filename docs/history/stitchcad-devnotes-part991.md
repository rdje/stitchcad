# Sealed engineering continuity — part991

- **Identity:** 28 lines, 2948 bytes, sha256:03ad92d0f60e15337e4873e5d5ad2383bafe1687a43928b1794c9dd032ed3d84
- **Coverage:** whole0815 task,lesson and ledger from 43c898b19982a86933e9647575eaa487f19b68c4.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 50787–52307, 1520 bytes, sha256:8c18a718baa1f6e8cc86ef542d0709f6f57b0ed3453c45e0acd3b197297360d5
- Whole source interval: DEV_NOTES.md bytes 193–914, 721 bytes, sha256:267b92e3cdeb7f09ca2134d6744055df3600e719c52475410f4c7efe06b4900b
- Whole source interval: CHANGELOG.md bytes 217–924, 707 bytes, sha256:635ebf776c3a147394d8d94190b06a98cbe718620c93846f2e67df8a68c6a2ea
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable;corrections belong in a new record.

---

### G1-SLICE.5f.1.book-search.build
- ID: `G1-SLICE.5f.1.book-search.build`
  Status: `done`
  Goal:build published mdBook0.5.4 in guarded project-local stores with published elasticlunr-rs3.1.0;record actual resolution and reproduce corrected field isolation through the public renderer.
  Acceptance:no external repository/store writes,source patches,warning suppression or changed corpus;preserve ambient binary and baseline;own installation and focused native field probe.
  - [x] ROOT CAUSE: launcher `search_field_probe.py --ambient --output target/book_search_0815_ambient_fields` →native red1,body frequency2/breadcrumb body-word leakage;both tools0.5.4,expected refusal checked rc=0.
  - [x] ADDRESSED: guarded launcher `cargo install mdbook --version 0.5.4 --root target/cargo-home` →compiled elasticlunr-rs3.1.0,rc=0;local `search_field_probe.py` →frequency1/no leakage,rc=0.
    Native same-source comparison →118 records,7712945B,no warnings;all URLs/stored fields/options/chapter word+term sets exact;120 input hashes match b0ad71d5,rc=0.
  - [x] NO REGRESSION: `python3 -I -B -` probe refusal controls →escape/existing capture/search override refused,bytes unchanged,rc=0;normal ambient build still warns13014554B,rc=0.
    990 three Git spans/body/public exact;archive1444/77,per-generation21/64 and56/64,rc=0;activation/query verification remain separately owned.
  Commit:STITCHCAD-G1-0815. promotion: declined (published upstream defect remedy;existing D372 and full-text/chapter contract).
## _(2026-11-01 UTC)_ — native renderer field fix built locally

0815 guarded published mdBook0.5.4 install compiled elasticlunr-rs3.1.0;host diagnostic red1/local green0 proves frequency2→1 and no breadcrumb leakage.
Same-source118-chapter candidate7712945B builds warning-free;120 hashes match b0ad71d5;stored fields/URLs/options/chapter word+term sets exact0.
Native probe keeps failed captures;escape/existing/override refusals preserve bytes0. Normal ambient build still warns13014554B;activation/query verification next.
Promotion declined: existing D372/full-text contract;no upstream patch,external write or product code change. 990 Git/public exact0;archive1444/77,per-generation21/64 and56/64;G1still6/18.

## STITCHCAD-G1-0815 — build and probe the published renderer fix locally

- Published mdBook0.5.4 compiled with elasticlunr-rs3.1.0 into the local Cargo store.
  Native host red/local green demonstrates corrected frequency and body/breadcrumb isolation.
- Same-source118-chapter candidate7712945B is warning-free;all stored fields,URLs,options,
  chapter words/terms exact;120 input hashes match b0ad71d5. No source patch or external write.
- Probe refuses parent escape,existing capture and search override;bytes unchanged. Normal
  ambient build still warns13014554B;activation and actual query proof remain owned.
- 990 Git/body/public exact;archive1444/77,per-generation21/64 and56/64. G1still6/18.

