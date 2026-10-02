# Sealed archive — checked signed reconstruction

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3b.3b.1b` on `2026-10-02`.

- **Sealed identity:** 17 lines, 1562 bytes, `sha256:d00c73401ab63e79bf0345adacedaae595e16cc200795620cf4db4ca6cc2541a`
- **Coverage:** original signed-rounding lesson, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## _(2026-10-02)_ — signed reconstruction must follow checked magnitude narrowing

- D78's sole introducing round.rs revision is eb83f01 (G0-CONTRACT.18). The public diagnostic
  proves i128 MIN/1 unwinds while other wide sign cases return Overflow and controls pass.
  An unsigned quotient 2^127 was cast to i128 MIN and then negated before checked i64 narrowing.
- Preserve the quotient/remainder rounding rule. Allow the negative i64 endpoint explicitly, then
  checked-convert every other unsigned magnitude to i64 before sign reconstruction. No negation can
  overflow that proven nonnegative range; denominator zero retains its existing diagnostic.
- Four public contracts, 36 arbitrary-precision Fraction oracle rows and five actual guard assertion
  reds verify signs/endpoints/ties/zero; exact restoration, strict 476 tests, release four and WASM pass.
  Debug/release public behavior now agrees; no customer scalar is added to error payloads.
- D79 is separate reference debt: sub-quantum unit literals retain fractions while L1 canonical
  display rounds them. A pair of 0.4 um literals binds 1, but their canonical-zero respelling binds 0.
  Own .5a.3b next before using that oracle for normalization; retain the book instrument's honest scope.
- Completed AST protocol/checklist and oldest ledger/lesson payloads preserve exact predecessor bytes.
  Numeric details stay in the expert units annex; no evaluation/canonical product claim or cap change.
- promotion: declined (routine totality repair; original fixed-point/rounding policy unchanged).
