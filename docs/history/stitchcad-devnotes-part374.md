# Sealed archive — whole full-forward threshold certificate scope

Immutable historical segment, sealed by leaf `G1-SLICE.5d.1.2.2.2.c` on `2026-10-26` (UTC).

- **Sealed identity:** 16 lines, 1707 bytes, sha256:0db3f45055a1542f5dbb7c7e77b0f3cad812afcf74b366624beaf944f3ebfb0d
- **Coverage:** whole committed source interval from 6037472180a6ea95568746ff217e98a93ed6130a.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 27374–29081, 1707 bytes, sha256:0db3f45055a1542f5dbb7c7e77b0f3cad812afcf74b366624beaf944f3ebfb0d
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5d.1.2.2.2.b.1`
  Status: `done`
  Goal:0278 full1M forward-band certificate, resumable chunks and independent exact gap/coverage reader.
  Own372 whole0277 task,373 oldest root-plan lesson,250 ledger before producer growth.
  Paths: docs/tasks/artifacts/formula_structure/forward_threshold_certificate.py;docs/tasks/artifacts/formula_structure/verify_forward_certificate.py
  Reuse pure explicit-pi helpers; source/profile/hash-locked checkpoints under target only.
  Stream fixed-width exact endpoints; complete index coverage, strict margins and refusal controls.
  No native precision admission/inverse/tangent claim. Decision/book/live sync, focused producer check.
  Acceptance0278:
  - [x] REPRODUCE:0277 certified38 only; no full forward population claim then.
  - [x] ROOT CAUSE0278: forward_threshold_certificate.py --chunks1 checkpoints10k/PARTIAL, rc=0; complete-reader explicitly refuses it. Cohort coverage cannot establish1M.
  - [x] FIX: source-locked100 chunks/exclusive run/atomic checkpoints; exact208-byte rows, independent integer gap reader.
  - [x] ADDRESSED0278: python3 -I -B docs/tasks/artifacts/formula_structure/forward_threshold_certificate.py resumes1 chunk then COMPLETE100/1M, rc=0. verify_forward_certificate.py checks all1M/hash/strict gaps/coverage, rc=0.
  - [x] NO REGRESSION0278: producer distinct known/Fraction/small-Q/negative controls and3 malformed-row refusals pass, rc=0; no runtime changed. Broad/native/platform obligations .c/.5d.4/pre-push.
  - [x] LOCKSTEP:372/373/250 exact Git/hash/public reads, rc=0; decision/book/live match full result; G1still5/18. No native/inverse/tangent admission.
  Verification: rc=0; Commit: STITCHCAD-G1-0278.
