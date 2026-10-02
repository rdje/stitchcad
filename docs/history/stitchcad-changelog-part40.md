# Sealed archive — recurring cleanup

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3b.3a.1` on `2026-10-02`.

- **Sealed identity:** 9 lines, 799 bytes, `sha256:156e91988e99ddf23d1b80488802ee21ab75f6e61dbe6876d577557e1150917e`
- **Coverage:** STITCHCAD-SPINE-0021a, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## STITCHCAD-SPINE-0021a - recurring artifact cleanup preserves the product frontier (leaf `SPINE.21a`)

Before the next product slice crosses the 24-hour mark, removed six ignored scratch/incremental/book
roots and 255 safe stray artifacts. Target went 663084 → 328736 KB; book removal adds 4832 KB,
for 339180 KB reclaimed. Independent residue census found zero remaining artifacts and no tracked
deletion; release/deps bin/log scans were zero. Shared stores, other repositories and built dependency
outputs remain untouched. Strict Rust's 287 tests, WASM, book, all 22 probe suites and staged gates
pass after regeneration. Prior cleanup checklist moves unchanged, and oldest changelog entry seals
to part23. Latest-run record and bounded book upkeep align; G1 .4 remains the product frontier.
