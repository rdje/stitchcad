# Sealed engineering continuity — part553

- **Identity:** 43 lines, 2921 bytes, sha256:ee1c7a845c9aed635669e3ef1bd0ef63a8f45e154da0b53dd98139a3c7cacb1f
- **Coverage:** whole completed units/measurement/SizeSet parent scopes from 49a62725963d78e56eec885667e55ffd7241b2b0.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 3346–4126, 780 bytes, sha256:eac9d82623724d45e92a9f21aac3f31bffea6b7da5827f7954518f27cbbc5cfb
- Whole source interval: docs/tasks/G1-SLICE.md bytes 6565–7324, 759 bytes, sha256:94cd72f2af8996fc770814b928635a3fdb8f2975407f3b354353f58715bf9a11
- Whole source interval: docs/tasks/G1-SLICE.md bytes 9252–9950, 698 bytes, sha256:30ee803a512cc362195e5d062cdc313d4d607cf14226bcbf0bd1042ad95d856e
- Whole source interval: docs/tasks/G1-SLICE.md bytes 15763–16447, 684 bytes, sha256:f9fa4c8bfca4ca144f0983649e4b1d1d3aceb81fe03d37be9b516f4363ad67b9
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.2`
  Status: `done`
  Goal: `sc-units` — fixed-point micrometre quantities (i64), the five tolerance classes as
  distinct types, unit conversions with explicit rounding, rejection of non-finite and
  dimensionally invalid expressions (implements the `G0-CONTRACT.2` spec).
  Acceptance: property tests for conversion round-trips and tolerance-class separation; a
  dimension error is a typed error, never a silent coercion; compiles for `wasm32-unknown-unknown`.
  Verification: recorded below — `sc-units` was delivered in full by `G0-CONTRACT.18` (commit
  `eb83f01`); every acceptance criterion re-derived by command in the `### G1-SLICE.2` checklist,
  and the property-test-framework choice recorded as a layer-C decision.
  Commit: `STITCHCAD-G1-0002`

- ID: `G1-SLICE.4`
  Status: `done`
  Goal: `sc-measure` — MeasurementTable (body vs garment POM, landmarks, source, procedure),
  Ease as a first-class body→garment mapping with fit intent, and SizeSet (implements
  `G0-CONTRACT.4`/`.6`).
  Acceptance: a POM without a landmark or procedure is rejected; ease is queryable per POM;
  size labels vs order vs base size are distinct fields with tested semantics.
  Children: .4a (measurement declarations/tables), .4b (per-POM ease), .4c (SizeSet), .4d (signoff).
  Verification: child contracts plus0321 measurement16/table16/Ease14,0320 model25 and unchanged composite31; publication74/87 pass0.
  Commit: STITCHCAD-G1-0321 (structural library); runtime/application/registry proof remains G1/G3/G4 work.

- ID: `G1-SLICE.4c`
  Status: `done`
  Children: .4c.1 (membership), .4c.2 (axes), .4c.3 (chart/breaks), .4c.4 (resolution), .4c.5 (review).
  Goal: SizeSet identity/revision/system, authored ordered labels/base, chart/state/provenance, adjacent
  breaks and axes; immutable profile resolution/transformation intent per canonical SizeSet decision.
  Acceptance: label/order/base distinct; path-2 missing-break refusal, MTM of one, no quantities/defaults;
  profile transformations never mutate Design references; execution/equivalence/approval later owned.
  Split into safe slices before implementation; preserve every size-sets chapter requirement.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.4c.4`
  Status: `done`
  Goal: immutable resolved SizeSet/profile transformation intent, pinned Design reference/profile revision,
  typed label mapping/break scaling/base substitution/chart replacement with state/evidence provenance.
  Acceptance: new resolved object retains original reference and records transformation; no in-place
  mutation. Execution, precedence/hard restrictions, equivalence and release approval retain later owners.
  Verification:0311–0317 focused state/binding/aggregate contracts,0318 unchanged-code/book/anchor review pass0.
  Commit: STITCHCAD-G1-0311–0318 (includes due cleanup/D232 repair); actual G4 execution remains separate.

