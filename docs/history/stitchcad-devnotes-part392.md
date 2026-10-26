# Sealed archive — whole complete inverse population scope

Immutable historical segment, sealed by leaf `G1-SLICE.5d.1.2.2.2.b.2.c` on `2026-10-26` (UTC).

- **Sealed identity:** 20 lines, 2129 bytes, sha256:2d8e7efac305f521a1e1f58bafc69d6e7bd0913e16e086905c7599f7e3b99baf
- **Coverage:** whole committed source interval from 062346c02e59a276e1ca0a7caf34eeffe2e17b4c.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 29356–31485, 2129 bytes, sha256:2d8e7efac305f521a1e1f58bafc69d6e7bd0913e16e086905c7599f7e3b99baf
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5d.1.2.2.2.b.2.b.2.b`
  Status: `done`
  Goal: complete45M native inverse population from frozen controller sources; full reader, cost/margin/hash evidence, then commit.
  Work unit0286 owns390 whole0285 protection task,391 oldest canonical-length lesson,258 ledger before growth.
  Invoke unchanged inverse_threshold_certificate.py --workers4, resume200k; no critical source edits.
  Verify all450 chunks with default complete reader. Retain exact coverage/strict gaps/source and
  payload root/cost metadata; inverse runtime/branches remain .b.2.c, tangent separate.
  All outputs stay in protected target/inverse_threshold_certificate/0284; do not hand off during job.
  Also own crates/sc-core/tests/fixtures/inverse_rounding_certificate.tsv: compact verified
  population/margin/digest/source witness for later budget checks without repeating the full run.
  Witness adapter: docs/tasks/artifacts/formula_structure/inverse_certificate_witness.py,
  tracked pure stdout metadata producer; it supplements and never replaces full payload reading.
  Acceptance0286:
  - [x] REPRODUCE:200k partial/control evidence existed;45M complete population remained unverified.
  - [x] ROOT CAUSE0286: inverse_threshold_certificate.py --workers4 resumes2 chunks and completes45M/450, rc=0; all targets require actual source-locked coverage.
  - [x] FIX: run frozen producer to completion; retain full data/provenance and compact tracked witness, no native API or precision limit.
  - [x] ADDRESSED0286: python3 -I -B docs/tasks/artifacts/formula_structure/verify_inverse_certificate.py COMPLETE45M/450 exact gaps/hash/coverage/order pass, rc=0; frozen sources/binary exact.
  - [x] NO REGRESSION0286: full reader terminal0/lock absent/child registry empty; inverse_certificate_witness.py reproduces fixture byte-exactly, rc=0. Min>2^104 derives phase margin>2^-540; native budget not admitted. Broad .5d.4/pre-push.
  - [x] LOCKSTEP:390/391/258 exact Git/hash/public bytes, rc=0; book/decision/live match complete proof. G1still5/18; .b.2.c next, tangent/AST/platform open.
  Verification: rc=0; Commit: STITCHCAD-G1-0286.
