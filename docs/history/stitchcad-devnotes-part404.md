# Sealed archive — whole completed certificate population subtree

Immutable historical segment, sealed by leaf `G1-SLICE.6.1` on `2026-10-26` (UTC).

- **Sealed identity:** 48 lines, 3320 bytes, sha256:b149a6bd9870db3e72012713a2e580693b34e1951f2b4159c6e8e50e22523f7f
- **Coverage:** whole committed source interval from b49128ed528b66872baa2596bc3e61139db7c21a.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 27122–30442, 3320 bytes, sha256:b149a6bd9870db3e72012713a2e580693b34e1951f2b4159c6e8e50e22523f7f
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5d.1.2.2.2.b`
  Status: `done`
  Goal: if .a succeeds, complete and independently verify the full applicable threshold population.
  No cohort/throughput extrapolation becomes all-input proof; retain coverage, margins and costs.
- ID: `G1-SLICE.5d.1.2.2.2.b.1`
  Status: `done`
  Goal:0278 complete1M forward bands/100 chunks and independent exact gap reader pass0.
  Paths: docs/tasks/artifacts/formula_structure/forward_threshold_certificate.py;docs/tasks/artifacts/formula_structure/verify_forward_certificate.py
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part374.md).
- ID: `G1-SLICE.5d.1.2.2.2.b.2`
  Status: `done`
  Goal: inverse first-octant generator and full45M threshold proof; split before implementation.
- ID: `G1-SLICE.5d.1.2.2.2.b.2.a`
  Status: `done`
  Goal:0282 inverse38/algebraic4/Fraction-atan2/negative2 feasibility controls pass0.
  Paths: docs/tasks/artifacts/formula_structure/inverse_threshold_feasibility.py;docs/decisions/decision_inverse-thresholds.md
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part383.md).
- ID: `G1-SLICE.5d.1.2.2.2.b.2.b`
  Status: `done`
  Goal: after successful cohort, full45M certificate with bounded local chunks/source lock and independent exact reader; consider native producer for cost.
- ID: `G1-SLICE.5d.1.2.2.2.b.2.b.1`
  Status: `done`
  Goal:0283 native10035/Python-band67/invalid-input5/lint0; bounded chunk only.
  Paths: crates/sc-core/examples/inverse_threshold_chunk.rs;docs/tasks/artifacts/formula_structure/native_inverse_probe.py
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part386.md).
- ID: `G1-SLICE.5d.1.2.2.2.b.2.b.2`
  Status: `done`
  Goal: after .1, source-locked complete45M controller/checkpoints/exact reader and full coverage evidence.
- ID: `G1-SLICE.5d.1.2.2.2.b.2.b.2.a`
  Status: `done`
  Goal:0284 guarded controller/reader200k partial/resume/control checks pass0; full45M remains open.
  Paths: docs/tasks/artifacts/formula_structure/inverse_threshold_certificate.py;docs/tasks/artifacts/formula_structure/verify_inverse_certificate.py
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part389.md).
- ID: `G1-SLICE.5d.1.2.2.2.b.2.b.2.b`
  Status: `done`
  Goal:0286 complete45M/450/full reader and compact witness verified; native budget still separate.
  Paths: docs/tasks/artifacts/formula_structure/inverse_certificate_witness.py;crates/sc-core/tests/fixtures/inverse_rounding_certificate.tsv
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part392.md).
- ID: `G1-SLICE.5d.1.2.2.2.b.2.b.2.h`
  Status: `done`
  Goal:0285 retained forward/inverse stores protected; actual census/omission/restoration and policy controls pass0.
  Paths: docs/tasks/artifacts/artifact_cleanup/cleanup.py;docs/tasks/artifacts/artifact_cleanup/certificate_store_controls.py
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part390.md).
- ID: `G1-SLICE.5d.1.2.2.2.b.2.c`
  Status: `done`
  Goal:0287 bounded576-bit/five-half/64-term atan/atan2 plan admitted; native next.
  Paths: docs/tasks/artifacts/formula_structure/atan_runtime_plan.py;docs/decisions/decision_atan-runtime.md
  Acceptance: [whole scope/six proof boxes](../history/stitchcad-devnotes-part394.md).
