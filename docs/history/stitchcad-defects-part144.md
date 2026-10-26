# Sealed archive — whole D239 diagnosis, director ruling and0330 evidence

Immutable historical segment, sealed by leaf `G1-SLICE.6.2.2.c.2.5.2` on `2026-10-26` (UTC).

- **Sealed identity:** 15 lines, 1308 bytes, sha256:755ea7c4b39832eddd113008ef60e096a27a59875991ccf3610aebc72a03b042
- **Coverage:** whole committed diagnosis/ruling interval from 120cc6ee952bf689af676ee54b976e41e001d586; final adoption is recorded separately.
- Whole source interval: docs/tasks/PLANNING.md bytes 84032–85340, 1308 bytes, sha256:755ea7c4b39832eddd113008ef60e096a27a59875991ccf3610aebc72a03b042
- **Retrieval:** bytes below the first rule and blank line; original terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

## D239 — approval data versus immutable manifest identity is unspecified

At22ab074, roadmap9 and release2 include approver/scope/date in the immutable manifest; release1/3
approve a preexisting candidate by its manifest digest. The active decision fixes immutability but
specifies neither prebinding these values nor detached approval storage. Filling unresolved metadata
post-hash changes bytes/identity. Root: field/list closure never defined that lifecycle boundary.
Impact: release input/approval schemas cannot be frozen safely. Owner G1-SLICE.6.2.2.c.2.5.1,
P1 now; .2 adopts director ruling before .3/.4 implementation. Recommend detached immutable approval
records bound to unchanged digest; alternative prebinds exact prospective metadata before final hash.
Historical source clauses484; synthetic witness is diagnostic only, not a product hash or actual approval.
Director2026-10-26 chose detached immutable approval records; .2 coordinates adoption next.
- [x] ROOT CAUSE0330: current clauses and guarded synthetic3 show post-hash fill changes identity; rc=0.
- [x] ADDRESSED0330: director chose detached records; existing decision/book record ruling, adoption owned .2; rc=0.
- [x] NO REGRESSION0330: census9/6/8/0/book/publication76-95 pass; Rust unchanged; full fix awaits adoption, rc=0.

