# Sealed archive — D109 exact statement and recipe identity bytes

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3f.1a` on `2026-10-02`.

- **Sealed identity:** 9 lines, 881 bytes, `sha256:68d3627b208a41b33dfea5b604e50eb215d5950fde51db9874f4990fa21dd0d3`
- **Coverage:** D109; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D109** — canonical grammar does not specify assertion or whole-recipe identity bytes.
  - Reproduce: grammar4 has binding examples and the complete expression mapping, but no assertion
    opcode/payload order or empty/whole ordered recipe separator/envelope/terminal-newline contract.
    Formula introduction says no further decision is needed despite .3f.1 owning this prerequisite.
  - Impact: choosing serializer bytes now would invent a persistent identity contract; two compliant
    implementations could emit different recipe identity. No product statement serializer exists yet.
  - Owner/schedule: G1-SLICE.5a.3f.1, P2 next, before implementation; finalize concrete bind/assert/
    empty/ordered examples, independent exact-byte controls and necessary authority ruling.
    Current .3e.3 corrects the overbroad completeness claim and keeps syntax closure scoped.
