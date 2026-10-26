# Sealed archive — whole G1-SLICE.5d.3.2.1 summary and retention

Immutable historical segment, sealed by leaf `G1-SLICE.5d.1.2.2.2.d.1` on `2026-10-26` (UTC).

- **Sealed identity:** 9 lines, 754 bytes, sha256:198b29080a57e8f61d303ade69193418dc8ddabab6fac7b7938e4aeaeabc15b6
- **Coverage:** whole committed source interval from cbbfb7c1167d49a00ed2f7120e41910deb1edf99.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 33606–34360, 754 bytes, sha256:198b29080a57e8f61d303ade69193418dc8ddabab6fac7b7938e4aeaeabc15b6
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5d.3.2.1`
  Status: `done`
  Goal:0280 native sin/cos3/0 with44 independent angles; shared pi1/0, arc2/0/lint0.
  Paths: crates/sc-core/src/recipe/trig.rs;crates/sc-core/src/recipe/pi.rs;crates/sc-core/tests/formula_sincos_value_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part379.md).
  Retention0281, owned .5e.3.6:
  - [x] ROOT CAUSE0281: git show89e0e87 complete native node source interval retained379; Python byte/hash comparison rc=0.
  - [x] ADDRESSED0281: whole node/source/hash exact in379, Python comparison rc=0.
  - [x] NO REGRESSION0281: scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part379.md whole file exact, Python comparison rc=0; no reader change.
