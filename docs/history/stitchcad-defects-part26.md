# Sealed archive — numeric loader cross-family execution

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3b.3b.2` on `2026-10-02`.

- **Sealed identity:** 10 lines, 1021 bytes, `sha256:6cdfd3bd77f9ef8634edef830e95ff68367610a7edda127f4a4d74063fbd2279`
- **Coverage:** original D94 observation and ownership, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D94** — numeric contract loaders execute unrelated arithmetic assertions during mutation checks.
  - Reproduce: scalar mutation lower-endpoint guard rejects Count0 while importing arithmetic_contract;
    runner refuses the unhandled FErr, before scalar assertions execute. Angle/rational loaders also
    run arithmetic_contract with runpy, so unrelated arithmetic failure can contaminate their reds.
  - Root: reference setup and arithmetic test execution share one top-level script; imports meant
    to obtain table-driven context run every contract. A traceback filename alone cannot prove scope.
  - Impact: cross-family failures can mask a missing target-family assertion or stop useful controls.
  - Owner/schedule: G1-SLICE.5a.3b.3b.2, fix now: extract a side-effect-free shared context loader,
    keep each family’s assertions in its own producer, require their actual reds and full restored
    control counts. No numeric policy change; certify context load produces no assertion/output side effects.
