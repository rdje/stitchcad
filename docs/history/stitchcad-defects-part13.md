# Sealed archive — Ease package-discovery drift

Immutable historical segment, sealed by leaf `G1-SLICE.4b.2` on `2026-10-02`.

- **Sealed identity:** 7 lines, 672 bytes, `sha256:6ed0c7dd00fef3bf29830c78f6c857d7451b989af334ef621674490c0df6d37b`
- **Coverage:** D69, repaired in this slice.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D69** — package description still claims Ease follows although individual Ease is executable.
  - Reproduce at 0088969: cargo metadata --offline --no-deps exposes sc-measure description saying
    "ease and size sets follow"; crates/sc-measure/src/lib.rs exports Ease implemented by .4b.1.
  - Impact: API/package discovery gives external agents a stale capability description.
  - Owner/schedule: G1-SLICE.4b.2, fix package description with set implementation and verify metadata.
  - Fixed by G1-SLICE.4b.2: package metadata now names per-POM Ease sets; cargo metadata --offline
    --no-deps verifies current description, rc=0. Code/book/package discovery agree.
