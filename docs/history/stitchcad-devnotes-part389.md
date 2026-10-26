# Sealed archive — whole inverse controller and defect discovery scope

Immutable historical segment, sealed by leaf `G1-SLICE.5d.1.2.2.2.b.2.b.2.h` on `2026-10-26` (UTC).

- **Sealed identity:** 17 lines, 1873 bytes, sha256:b1da2eb766031b99c18ea56906cd971cb6a9903d335861f30e6b1e7aeb13d3bb
- **Coverage:** whole committed source interval from a1f7e2a5a006724792aefc978968b77383f3475d.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 28977–30850, 1873 bytes, sha256:b1da2eb766031b99c18ea56906cd971cb6a9903d335861f30e6b1e7aeb13d3bb
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5d.1.2.2.2.b.2.b.2.a`
  Status: `done`
  Goal:0284 complete guarded resumable inverse controller/reader; focused partial/resume/refusal before full run.
  Own386 whole0283 task,387 oldest midpoint lesson,256 ledger before growth.
  Also own388 whole closed .5b.4c.h2.b.p.n.m1 node/receipts for acceptance/defect tracking space.
  Paths: docs/tasks/artifacts/formula_structure/inverse_threshold_certificate.py;docs/tasks/artifacts/formula_structure/verify_inverse_certificate.py
  Native stdout to bounded local chunks, source/binary hashes, exclusive lock, child cleanup,
  explicit partial manifest and independent exact row/hash/coverage reader; no full45M claim.
  Commit verified controller before .b full job. Decision/book/live sync; output run0284 under target.
  Acceptance0284:
  - [x] REPRODUCE: native chunks lacked durable full-population controller/independent reader.
  - [x] ROOT CAUSE0284: inverse_threshold_certificate.py --chunks1 --workers1 publishes100k/PARTIAL, rc=0; default full reader refuses partial. Coverage requires450 exact ranges and frozen producer provenance.
  - [x] FIX: guarded parts/atomic manifests/source capture/verified binary copy/exclusive lock/owned-child reaping; exact Python reader.
  - [x] ADDRESSED0284: same controller resumes1 chunk to200k/PARTIAL, rc=0; verify_inverse_certificate.py --allow-partial checks both chunks/hash/coverage, rc=0.
  - [x] NO REGRESSION0284: reader corruption3/coverage2/source-drift1/live-read1 and actual owned-child reaping2 controls pass, rc=0. Lock absent/registry empty; no45M/native API claim; broad proof .5d.4/pre-push.
  - [x] LOCKSTEP:386–388/256 exact Git/hash/public bytes, rc=0; decision/book/live align. Cleanup certificate deletion candidate reproduced read-only; next P1 .h owns fix before full run. G1still5/18.
  Verification: rc=0; Commit: STITCHCAD-G1-0284.
