# Sealed archive — reference machine input

Immutable historical segment, sealed by leaf `G1-SLICE.5a.2b.1` on `2026-10-02`.

- **Sealed identity:** 11 lines, 1083 bytes, `sha256:76f40fb4b8d204c84874258694de481bc8b43eab78e5ddd1c13c411d406be0c8`
- **Coverage:** D76, closed and verified by STITCHCAD-G1-0039; original description and dated widening unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D76** — reference formula input parser accepts malformed machine identifiers and missing unit gap.
  - Reproduce: load actual reference definitions through formula_structure.py; parse Upper, _a and
    a__b returns name ASTs. With canonical cm unit context, parse 1cm returns a length literal,
    although grammar §1 requires lower-snake names and §2 requires exactly one space before a unit.
  - Impact: the reference grammar oracle cannot independently verify these product parser refusals.
  - Owner: `G1-SLICE.5a.2b.1`, priority next safe input-parity prerequisite before expression trees;
    fix spelling/keyword positions/unit separation and establish paired controls/actual guard reds.
    Product MachineToken/FormulaLexer already refuse malformed names; this is reference-input debt.
  - Recovery widening `2026-10-02`: direct actual-reference calls also accept bare if/let,
    assert(1), abs(), and let if/let declaration names. Empty args contradict grammar §1;
    scope .5a.2b.1 includes these input-shape refusals with unchanged valid conditional behavior.
