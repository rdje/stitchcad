# Sealed archive — self-contained bounded history

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3b.3b.2` on `2026-10-02`.

- **Sealed identity:** 9 lines, 809 bytes, `sha256:dfe49803a1b543ab34d1c418d0c0fa83cd2e5a76e40a697e1fa44659d1e9d8c5`
- **Coverage:** STITCHCAD-SPINE-0019b, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## STITCHCAD-SPINE-0019b - self-contained bounded history windows (leaf `SPINE.19.2`)

D65: retained all 64 historical files byte for byte in a content-addressed window; complete manifests
and catalog preserve logical paths. Reader list/read/materialize/verify works without historical Git
objects; exact capture reconstruction is independently proved. Decoded and resident storage retain
original aggregate bounds, with finite controls/payload/decompression and immutable committed windows.
Ledger probes consume logical records, all nine arms pass. D68 fixes the coverage mutation's unrelated
false pass. New archive refusals, binary sizing, strict Rust/WASM/book/full probes and staged gates pass;
exceptional push/observed CI follow in .19.2v. Older live records seal unchanged; product remains G1 .4a.3.
