# Sealed archive — whole archive13 summary and retention

Immutable historical segment, sealed by leaf `G1-SLICE.6.2.1` on `2026-10-26` (UTC).

- **Sealed identity:** 9 lines, 775 bytes, sha256:ec5d7a9a7757de9f98d7042bd7e5c5cb3b39ab53c3df806829ef7333ae2e9fcd
- **Coverage:** whole committed source interval from 666229849f9afd057cbe54d201f294b398ee1bc9.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 5849–6624, 775 bytes, sha256:ec5d7a9a7757de9f98d7042bd7e5c5cb3b39ab53c3df806829ef7333ae2e9fcd
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5e.2.h13`
  Status: `done`
  Goal:0276 window13 packs52 whole records; prior12/source/read/materialization exact.
  Paths: docs/history/;.doctrine/history_archive/;docs/tasks/artifacts/history_archive/capture_window13.py
  Acceptance: [whole scope/six proof boxes](../history/window14.md#stitchcad-devnotes-part370md).
  Retention0277, owned .5d.1.2.2.2.a:
  - [x] ROOT CAUSE0277: git show3d97348:docs/tasks/G1-SLICE-execution.md interval5838:8312 is whole2474B, Python hash comparison rc=0.
  - [x] ADDRESSED0277: retained370 body/source/hash exact, Python comparison rc=0.
  - [x] NO REGRESSION0277: scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part370.md returns whole file byte-exact, Python comparison rc=0; no reader change.
