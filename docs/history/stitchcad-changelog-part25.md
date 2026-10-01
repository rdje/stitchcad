# Sealed archive — explicit cut-once L/R piece members

Immutable historical segment, sealed by leaf `G1-SLICE.4a.3` on `2026-10-02`.

- **Sealed identity:** 17 lines, 1371 bytes, `sha256:d57637bea01cba7187ab63d50dbbe5b04e48b46dbe9df95bbcbbaaa2ed5bd353`
- **Coverage:** STITCHCAD-G1-0008, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## STITCHCAD-G1-0008 - separate cut-once L/R members are explicit piece content (leaf `G1-SLICE.3c.1a`)

Fixed D56: the canonical skirt's separate left/right back members, each cut once, were not representable
by the even-total mirrored-pair mode. `Mirroring::PairMember { handedness, companion }` now carries the
member's own L/R label and distinct companion Piece identity; quantity counts this member's copies.
Self-companions are refused. Existing even-total pair requests retain their original meaning.
The book documents both forms; collection-level reciprocity and equal quantities are owned by `.6`,
and geometric mirroring remains G2's obligation.

Validation: 14 piece-contract tests (including the canonical fixture-shaped pair and self-companion
refusal), strict `make check`, `make wasm`, `make book`, reference-fixture derivation, feature census,
doctrine gate and ledger probes green. Applying the even-total rule to all pair modes makes the fixture
regression fail. D56 closes in defects-part3; the decision records the two pairing forms.

D57 is logged and owned by `.3c.2b`: the spec does not decide how a sewing side names physical cut copies.
The director was asked to choose stable copy identities or pattern-level references with later expansion.
No answer is inferred; independent marks/allowances (`.3c.3`) proceed while that decision is pending.
