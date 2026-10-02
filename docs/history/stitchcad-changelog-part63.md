# Sealed archive — STITCHCAD-G1-0046

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3f.1a` on `2026-10-02`.

- **Sealed identity:** 11 lines, 950 bytes, `sha256:939369e8a12595f38650817d12f28f2bfd5767014bae8d936e41342d3e01938b`
- **Coverage:** STITCHCAD-G1-0046; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## STITCHCAD-G1-0046 - public length operators preserve the domain (leaf `G1-SLICE.5a.3b.3b.1a`)

D89 closes: Length + / - return Result using checked arithmetic, so valid operands cannot construct
an invalid length. Use `(left + right)?` / `(left - right)?`; no silent
clamp or saturation. Four public contracts include signed boundaries, inclusive endpoints, ordinary
values/cancellation, Result typing and i128 pair expectations. Six actual production
bypass/operation/saturation mutations compile and fail assertions, with exact source restoration.
Strict native/release/WASM and focused book/recording checks verify the restored candidate.
D90 missing operation context is owned immediately next; D83 scalar/reference i64 and D84 signed-angle
proof remain owned. D91 language context follows D90; prior evidence/history preserves exact bytes.
G1 remains 5/18; defects14 open/76 sealed; next .5a.3b.3b.1b operation context, D70 decision pending.
