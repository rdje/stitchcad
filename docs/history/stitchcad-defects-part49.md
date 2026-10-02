# Sealed archive — D128 populated uncertainty state

Immutable historical segment, sealed by leaf `G1-SLICE.5e.1a` on `2026-10-02` (UTC).

- **Sealed identity:** 7 lines, 651 bytes, `sha256:850c0abaca2fd536b8544d2272ced4b531fbd77a98ba9f36dce30e52cce80feb`
- **Coverage:** D128; original report unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D128** — populated reference records bypass uncertainty-state consistency.
  - Reproduce: actual statement input_value with measurement/length/value17 and state unknown
    returns length17; the same record with state invalid also returns length17. Baseline exit0.
  - Root: state validation only runs after finding no value; a populated value bypasses it.
  - Impact: an unknown fact can expose a supplied payload contrary to ontology5 and formula2/3.
  - Owner: G1-SLICE.5e.1a now; validate explicit states before reads, refuse unknown-plus-value,
    prevent lazy resolution of unknown geometry and preserve valid/no-state computed fixtures.
