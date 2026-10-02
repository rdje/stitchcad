# Sealed archive — angle refusal reason masking

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3b.3a.2` on `2026-10-02`.

- **Sealed identity:** 7 lines, 694 bytes, `sha256:36d8a475871a27da4a56cd8f66d13a8af20ed86438cc854436fe652cf1890fef`
- **Coverage:** original D88 observation/ownership, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D88** — angle-domain contracts accept an unrelated refusal, masking a removed tangent pole guard.
  - Reproduce: after the D83 rational guard, run_angle_mutations.sh mutation5 removes the tan pole
    guard but angle_contract.py returns rc=0: the huge numeric pole result instead raises a rational
    formula_domain, and the test checks only the token. The mutation runner correctly fails overall.
  - Impact: a wrong root cause can masquerade as the required mathematical-domain refusal.
  - Owner/schedule: G1-SLICE.5a.3b.3a.2, fix now by checking pole operation/reason in each tan refusal;
    retain a distinct atan2-zero reason, rerun seven actual angle reds and exact restoration.
