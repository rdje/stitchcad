# Sealed archive — whole G1-SLICE.5d.3.3.1 summary and retention

Immutable historical segment, sealed by leaf `G1-SLICE.5d.1.2.2.2.d.1` on `2026-10-26` (UTC).

- **Sealed identity:** 9 lines, 700 bytes, sha256:36160352851770fe154e1fab8aa19af25bcc5ceec906725b88074c44e8e76df2
- **Coverage:** whole committed source interval from cbbfb7c1167d49a00ed2f7120e41910deb1edf99.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 34613–35313, 700 bytes, sha256:36160352851770fe154e1fab8aa19af25bcc5ceec906725b88074c44e8e76df2
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5d.3.3.1`
  Status: `done`
  Goal:0288 native inverse3/0 with independent27/54 fixtures/live oracle/lint0.
  Paths: crates/sc-core/src/recipe/inverse.rs;crates/sc-core/tests/formula_inverse_value_contract.rs
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part396.md).
  Retention0289 owned .5e.3.7:
  - [x] ROOT CAUSE0289: git show3219d3f whole0288 node retained396; Python source/hash comparison rc=0.
  - [x] ADDRESSED0289: whole node/source/hash exact in396, Python comparison rc=0.
  - [x] NO REGRESSION0289: scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part396.md exact whole file, Python comparison rc=0; no reader change.
