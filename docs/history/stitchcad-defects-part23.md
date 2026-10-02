# Sealed archive — public length operator domain bypass

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3b.3b.1a` on `2026-10-02`.

- **Sealed identity:** 10 lines, 971 bytes, `sha256:f98863f6c8cdde8db94573b7dd5c6d6b1c9855dbc8104ee1b675b33900edb38e`
- **Coverage:** original D89 observation/ownership, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D89** — public Length addition/subtraction bypass the declared domain invariant.
  - Reproduce: construct both ±MAX_LENGTH_UM through the public constructor, then positive+positive
    or positive-negative produces 2000000000 um without refusal; negative routes also leave ±1 km.
  - Root evidence: length.rs Add/Sub directly construct Self from raw arithmetic; checked_add/sub
    instead call the checked constructor. The private-field invariant is therefore not closed.
  - Impact: downstream Length consumers can receive values their constructors prohibit, undermining
    area/predicate bounds; repeated unchecked operations can eventually overflow or unwind.
  - Owner/schedule: G1-SLICE.5a.3b.3b.1a, fix now before trusting scalar-domain reference normalization.
    Make + and - fallible via the existing checked operations; verify public boundary/ordinary/sign
    routes, actual guard mutations, strict Rust and WASM, and document the Result migration.
