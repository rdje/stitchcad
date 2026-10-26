# Sealed archive — implemented numeric call provenance ledger

Immutable historical segment, sealed by leaf `G1-SLICE.5e.3.4` on `2026-10-26` (UTC).

- **Sealed identity:** 4 lines, 319 bytes, sha256:c167f9124ed48c92bd121ceabd6e623a898163d549d544ea16c057f9b5c4d401
- **Coverage:** whole committed source interval from a88c7b7bb05bbb866265128b39c1143967cf35fd.
- Whole source interval: CHANGELOG.md bytes 13754–14073, 319 bytes, sha256:c167f9124ed48c92bd121ceabd6e623a898163d549d544ea16c057f9b5c4d401
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

## STITCHCAD-G1-0270 — implemented numeric calls retain approximation sources

.5e.3.3: sqrt/hypot/abs/min/max/clamp/round_to/arc_length reuse actual numeric kernels and retain all inputs.
Successful rounding calls add their source even for exact results; actual typed refusals propagate. AST/trig execution follows.
