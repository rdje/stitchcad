# Sealed archive — StitchCAD stale landing/workspace status, closed D66

Immutable historical segment, sealed by leaf `G1-SLICE.4a.2b` on `2026-10-01`.

- **Sealed identity:** 10 lines, 919 bytes, `sha256:4d46b154df281275e208c579faf42b7b6f031f460bfb31a0b0cdbdcd786b51f6`
- **Coverage:** `D66`; landing/workspace bootstrap status corrections.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D66** — README status reports only G0 contract work after executable G1 foundations landed;
  the workspace header still asks for removal of the already retired starter crate.
  - Reproduce: README's status paragraph/workspace header against committed G1 .3c family review,
    .4a.1 core input and the reconciled G1.1 starter-crate removal.
  - Impact: the landing page understates implementation, although LIVE_STATUS and the book are accurate.
  - Owner/schedule: `G1-SLICE.4a.2b`, next metadata/runtime integration slice; repair status without
    implying G0 human approval or an existing user application.
  - **Closed:** README reports executable foundations and accurate unapproved/no-application scope;
    the workspace header describes the current crate convention. The new make wasm quick-start command
    builds all three domain crates. Strict Rust, book, full probes and staged gates pass locally.
