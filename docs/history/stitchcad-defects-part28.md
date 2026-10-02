# Sealed archive — D96 changelog target recognition

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3b.3b.3c.1` on `2026-10-02`.

- **Sealed identity:** 12 lines, 1117 bytes, `sha256:bc034b1a3f06978ae9de52e1497835fb0e17e41e932bd1026bff29940d49f508`
- **Coverage:** D96; original description retained unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D96** — changelog POINTER recognizes filename text rather than the retained record’s target.
  - Reproduce: window2’s valid catalog links keep short labels changelog-part24.md/part25.md;
    the actual ledger diagnostic emits one POINTER FAIL for15 unpointed records (including24/25),
    rc=0. Full ledger
    probes report7 pass/2 fail (REAL and COVERAGE-RED); full make probes exits2. Other rules pass.
  - Root: grep extracts stitchcad-changelog-partN.md anywhere in the live text. Packed headings
    legitimately use partNmd; short labels omit the prefix. No source record or catalog is missing.
    The converse can count a correct filename label whose target is wrong, so label editing alone
    would not repair the contract. Pointer evidence must come from actual maintained link targets.
  - Impact: valid retained navigation falsely refuses; filename-label presence can mask bad targets.
  - Owner/schedule: G1-SLICE.5a.3b.3b.3c.1 fixes this small blocking archive consumer now; independent
    raw/catalog/short-label/label-only/wrong-target controls must discriminate, preserve D30 exemption.
