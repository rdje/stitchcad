# Sealed archive — StitchCAD G1 units reconciliation

Immutable historical segment, sealed by leaf `G1-SLICE.3c.4c.1b` on `2026-10-01`.

- **Sealed identity:** 19 lines, 1598 bytes, `sha256:a3918baae06f6bf833d51d5c692354903a7680e6424a7f851f1252da1c831b1b`
- **Coverage:** STITCHCAD-G1-0002, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## STITCHCAD-G1-0002 - the first property tests set the framework every later crate inherits (leaf `G1-SLICE.2`)

`G1-SLICE.2` (`sc-units`) was the second leaf `G0-CONTRACT.18` (commit `eb83f01`) pre-empted: that commit
landed `sc-units` in full — 1097 lines of library, 564 lines of property tests — as "the first product code",
not the skeleton its own leaf scoped. Like `.1`, the leaf stayed `pending` while its deliverable shipped. This
slice reconciles it: an audit, no new code.

Every acceptance criterion was re-derived by command and pasted into the leaf's `### G1-SLICE.2` checklist:
`cargo test -p sc-units --test property` → `21 passed` (conversion round-trips,
`the_classes_disagree_so_they_are_load_bearing` for class separation, `counts_are_their_own_dimension` and
`non_finite_floats_are_rejected_at_the_boundary` for typed dimension/non-finite errors); `UnitError` is a typed
enum, never a silent coercion; `make wasm` cross-builds the crate; the five tolerance classes are distinct
`ToleranceClass` variants (T1–T5).

The slice also discharges the Open Question `eb83f01` left open — "property-test framework choice, decided in
`.2`" — by recording `decision_property-tests-dependency-free-recorded-seed.md`: dependency-free hand-rolled
properties with a recorded seed are the default on the `wasm-viewer` critical path, and a framework off that
path is a per-crate recorded decision. The record carries `answers:`, which promotes this slice's `DEV_NOTES`
lesson. `make gate` stays `=== all doctrines green ===`. The frontier advances to `.3`, the `sc-core` ontology.
