# Sealed archive — StitchCAD dev notes, reconciling shipped work

Immutable historical segment, sealed by leaf `G1-SLICE.3c.2b.1` on `2026-10-01`.

- **Sealed identity:** 15 lines, 1380 bytes, `sha256:701d33f26a35ae26bfd6d5e0f9d3be15655d5dadfdd58c1c0d8498042171dd69`
- **Coverage:** the oldest live `2026-09-30` lesson (reconciling the shipped workspace), copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## _(2026-09-30)_ — a leaf marked `pending` whose work shipped under a sibling is a frontier that lies

- `G0-CONTRACT.18` retired the starter crate and created `sc-units` + `sc-core` "closing defect D10 ahead of
  `G1-SLICE.1`" — then closed only itself. The G1 leaf kept saying `pending` while its deliverables were
  committed and green, so the layer-B frontier pointed the next session at finished work. The drift is not a
  bad sentence in the tree; it is a missing reconciliation step: when a leaf pre-empts a sibling, the sibling's
  status is part of that commit's lockstep, or it must be audited promptly after.
- The closure is an audit, not new code: each acceptance criterion re-derived by command (`cargo metadata`,
  `git ls-tree`, `make check`/`wasm`/`gate`, the G0 exit review, the Knowledge Map), the verdicts pasted into
  the leaf's checklist so a reader who wrote none of it can reproduce the whole closure. **The certification of
  finished work gets the same treatment as the work** — a derived verdict, not a confident one (the rule
  `G0-CONTRACT.15`'s gate review runs on).
- promotion: declined — the lesson is an instance of the D34 hand-kept-state class (a tree cell no derivation
  watches) already owned by `PLANNING.5`; a new decision record would duplicate that ownership. The instance is
  fixed here, the class stays with its derivation.
