# Sealed archive — D98 push-verdict attribution

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3b.3b.3c.2` on `2026-10-02`.

- **Sealed identity:** 9 lines, 830 bytes, `sha256:6112b26651eb38b989a421af7a794239a9dfeddfcbd88dec6ea129940518cafa`
- **Coverage:** D98; original description retained unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D98** — preserved G1-0053 evidence labels the checker’s due exit as make’s exit.
  - Reproduce: the retained .1v checklist says make push-due reports due with rc=1. Makefile
    deliberately prefixes the checker recipe with minus and states it never fails the build;
    the direct checker returns1 for due while its Make wrapper returns0.
  - Root/impact: evidence collapsed producer and reporting-wrapper statuses. It can mislead an
    automated consumer, though the recorded successful push and exact CI verdicts are unaffected.
  - Owner/schedule: G1-SLICE.5a.3b.3b.3c.2 corrects this small claim now; preserve historical bytes,
    verify direct versus Make verdicts through the published older-comparison-base interface and
    clarify COMMIT.md without changing push authority, cadence or actual build behavior.
