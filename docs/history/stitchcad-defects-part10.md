# Sealed archive — StitchCAD stale ontology coverage, closed D64

Immutable historical segment, sealed by leaf `G1-SLICE.4a.1` on `2026-10-01`.

- **Sealed identity:** 8 lines, 801 bytes, `sha256:a33e5ff9853a8ab511988882eecd20565af3b4e3f49f105c354f59cfa5864a19`
- **Coverage:** `D64`; stale public and module coverage statements.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D64** — ontology introduction says only Pieces are implemented and other object types follow,
  after all four structural families completed in G1-SLICE.3c; module inventory has the same stale tail.
  - Reproduce: `sed -n '3,6p' docs/book/src/spec/ontology.md` against the committed ontology-review chapter.
  - Impact: the director's book view misreports code coverage despite the accurate implementation index.
  - Owner/schedule: `G1-SLICE.4a.1`, immediate introduction correction and book/scope verification.
  - **Closed:** ontology introduction and core module inventory now report all four completed
    structural families; the committed review remains the scope authority. Warning-free book and
    strict Rust/WASM verification pass; measurement declarations are described separately.
