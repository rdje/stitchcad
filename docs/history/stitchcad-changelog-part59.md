# Sealed archive — STITCHCAD-G1-0041

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3d.3` on `2026-10-02`.

- **Sealed identity:** 13 lines, 1098 bytes, `sha256:38e6cdf2def2379a5569d852f1d38b63b36b2fa8df7baa53c97d990c72ec0c11`
- **Coverage:** STITCHCAD-G1-0041; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## STITCHCAD-G1-0041 - total extreme-magnitude rounding (leaf `G1-SLICE.5a.3a`)

D78 closes: public i128 MIN/1 previously panicked before its checked i64 conversion. Checked unsigned
quotient narrowing and an explicit negative i64 endpoint preserve half-away rounding and return typed
Overflow for wider magnitudes. No clamp, wrap, new precondition or changed UnitError is introduced.

Four public contracts/36 independent Fraction rows and five actual guard assertion reds pass with exact
restoration. The public diagnostic now returns typed errors for all three wide magnitude cases;
controls and both i64 endpoints pass. Restored strict 476 tests, release four contracts and three-library
WASM pass; book/reference/ledger/archive/censuses/staged doctrines verify the recording commit.
Completed AST evidence and oldest live payloads preserve b681a49 bytes. D79 reference literal identity
was independently found/logged and is scheduled next before product canonicalization; its open scope
is stated in the expert annex. G1 stays 5/18; defects 11 open/67 sealed; D70 axes ruling remains pending.
