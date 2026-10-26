# Sealed archive — whole canonical stored numeric-read lesson

Immutable historical segment, sealed by leaf `G1-SLICE.6.2.1` on `2026-10-26` (UTC).

- **Sealed identity:** 14 lines, 1024 bytes, sha256:99912089d0f7a86f81b3f8095ce9cddc5e6235448bf2de8905b6907efaf2999f
- **Coverage:** whole committed source interval from 666229849f9afd057cbe54d201f294b398ee1bc9.
- Whole source interval: DEV_NOTES.md bytes 22790–23814, 1024 bytes, sha256:99912089d0f7a86f81b3f8095ce9cddc5e6235448bf2de8905b6907efaf2999f
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

## _(2026-10-26 UTC)_ — exact canonical and stored numeric reads

0267 .5e.1.6 converts canonical scalar payloads or once-rounded bindings into exact numeric values.
Actual kinds/internal integer units survive, including signed raw turns and i64 MIN; no second
rounding, ppm scaling, modulo or pre-binding fraction appears. Boolean false/true both refuse
numeric conversion. The caller retains its original source/state wrapper and binding provenance;
these source-free conversions invent no identity, statement context or approval.

Five focused native tests and selected strict Clippy pass0. The previous test output was lost at
compaction; OS-visible census found no remaining check, then the focused rerun established this
verdict. The stale LIVE_STATUS “sums next” note was reproduced by reading its row and corrected
to actual full-evaluation scope in this leaf. Whole0266 task/ledger retain348/239 with exact Git/
hash/public reads; book/live align. Promotion declined: existing numeric and binding contracts.

