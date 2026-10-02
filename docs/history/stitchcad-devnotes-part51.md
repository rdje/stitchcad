# Sealed archive — domain diagnostic context

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3b.3b.3c.1` on `2026-10-02`.

- **Sealed identity:** 19 lines, 1705 bytes, `sha256:6c25796fa5ed961e77e4a780de2ff9d5d448869418c564e2c7465fc334a2ad9c`
- **Coverage:** domain-context lesson; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## _(2026-10-02)_ — domain diagnostics need context without an invented cause

- D90's public DomainExceeded variant lacked operation; forwarded Length construction reused the
  base constructor, losing the caller. D92's generic display claimed every invalid value exceeds an
  upper limit and proves a conversion bug, including a zero positive-range width and negative length.
- Add a static operation field and one private checked-construction path per quantity; forward actual
  caller labels without duplicating guards or changing numeric data. Core resolve/range bridges name
  their public operation. Render neutral outside-domain wording; do not infer a cause from magnitude.
- Actual predecessor public calls produce two assertion failures. Five public tests verify typed and
  rendered direct/forwarded signed refusals, inclusive endpoints and unchanged non-domain behavior;
  three private core tests verify unreachable-invalid totality guards without a geometry certificate.
- Fourteen actual context/rendering mutations compile and fail assertions, restoring every source
  byte. D89's six compiled operator reds still discriminate. Strict native488 including docs,
  release5 and three-crate WASM pass. Division/area domain failures cannot occur for valid Length
  operands; those caller labels are wired without a fabricated public error reproduction.
- Book migration/examples match code. Earlier task evidence and oldest ledgers preserve exact bytes.
  D91 context classifier remains next;
  D83/D84 remain owned. No formula evaluator, command bus, MCP or production-release claim.
- promotion: declined (routine completion of the existing typed-error and truthful-diagnostic contract).
