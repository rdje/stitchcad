# Sealed archive — STITCHCAD-G1-0058

Immutable historical segment, sealed by leaf `SPINE.23` on `2026-10-02`.

- **Sealed identity:** 12 lines, 970 bytes, `sha256:ef59b0a1089b21be5060b517c184d99370f37cfc944a7a2af5e3c04f82aef830`
- **Coverage:** STITCHCAD-G1-0058; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## STITCHCAD-G1-0058 - full-width unsigned rounding (leaf `G1-SLICE.5a.3c.1`)

A new sc-units unsigned128 API shares the signed half-away magnitude rule, preserving wide positive
literal children before later signed binding. Subtraction replaces doubled remainder so every
nonzero u128 ratio rounds without narrowing, overflow or wrap; zero names the public operation.
Signed i64 endpoints, signs and overflow remain unchanged. No formula conversion/execution yet.

Five public contracts/138 independent Decimal rows, nine actual debug mutation reds and one release
red pass with exact source restoration. Existing signed contracts/36 Fraction rows/five reds remain
required. Strict Rust494 (units46), release public tests, three WASM crates and focused book/reference
checks pass. Indexed expert annex and unit API/example align with task/live records; prior evidence
and oldest ledger payloads remain exact. G1 stays5/18; next .5a.3c.2 exact typed literal conversion.
