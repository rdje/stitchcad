# Sealed archive — angular conversion rounding and poles

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3b.3a.1` on `2026-10-02`.

- **Sealed identity:** 19 lines, 1611 bytes, `sha256:a6afecc2b66f6fa99a8f1a6fb921079c051c3c7f0c6534cf793220a374a2348c`
- **Coverage:** D85, D86, D87, closed and verified by STITCHCAD-G1-0044; descriptions unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D85** — reference dir truncates a microdegree Decimal instead of nearest-quantum rounding.
  - Reproduce: literal_diagnostic.py p=(0,0), q=(1,6) → angle80537677; rnd(_deg2(6,1)) → 80537678.
  - Root: dir calls norm_angle(_deg2), whose int conversion truncates; atan2 already rounds correctly.
  - Impact: normalized direction can differ by one quantum from the same vector's atan2 result.
  - Owner/schedule: G1-SLICE.5a.3b.3a.1, immediate angle numeric prerequisite.

- **D86** — reference trig/arc_length consumes microdegrees as degrees.
  - Reproduce: actual literal_diagnostic.py arc_length(360 deg,1 um) → 6283185 um instead of rounded6.
  - Root: to_true only rescales ratios; trig/arc_length multiplies internal microdegrees by pi/180.
  - Impact: angular formulas have a million-fold conversion error and periodic trig can mask it.
  - Owner/schedule: G1-SLICE.5a.3b.3a.1, immediate before angle/rational audit; shared direct conversion.

- **D87** — reference tan has no exact singular-angle domain refusal.
  - Diagnostic target: tan(90 deg)/tan(-90 deg)/tan(270 deg) must refuse formula_domain because
    odd quarter-turn tangent is undefined; current generic sin/cos division supplies a numeric result.
    Actual literal_diagnostic.py → tan(90), tan(-90), tan(270) all ratio:0, rc=0 observation.
  - Root: call has no pole guard; conversion D86 currently masks the pole as a whole-turn input.
  - Impact: an invalid direction can enter ratio data rather than a typed refusal.
  - Owner/schedule: G1-SLICE.5a.3b.3a.1, immediate with conversion guard, signed/multi-turn controls.
