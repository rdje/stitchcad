# Sealed archive — D101 index repair and D102 record placement

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3c.4` on `2026-10-02`.

- **Sealed identity:** 14 lines, 1264 bytes, `sha256:34739bcc0c165652fe729de5aa2dd12ba4ad6830ef3805f56f86fc0dbaed5aed`
- **Coverage:** D101/D102; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D101** — a documentation edit replaced the topic index with unrelated decision text.
  - Reproduce: G1-SLICE.5a.3c.4 publication INDEX_COVERAGE reports all49 required chapters missing;
    topic-index.md begins with decision_angles content. The inline edit assigned p to the index but
    its conditional expression tested/used stale s from the previous decision edit on its else path.
  - Impact: the rendered book loses its usable chapter/topic index; publication gate blocks release.
  - Owner/schedule: G1-SLICE.5a.3c.4, immediate/P1. Restore exact committed index before applying
    the intended additional link from an explicitly read index value; verify publication/link coverage.

- **D102** — a defect-log edit inserted D101 inside a quoted historical command.
  - Reproduce: D46's sed command contains the inserted D101 block; the inline insertion selected
    the first ## Decisions substring, which belongs to that command rather than a section heading.
  - Impact: the preserved D46 record is corrupted and D101 lacks its own proper entry position.
  - Owner/schedule: G1-SLICE.5a.3c.4, immediate/P2. Restore byte-exact D46 from HEAD; relocate
    D101 by exact payload removal and a line-anchored heading, verify original record and ledger.
