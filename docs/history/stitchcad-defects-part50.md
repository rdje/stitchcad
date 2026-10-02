# Sealed archive — D121 reference contribution provenance

Immutable historical segment, sealed by leaf `G1-SLICE.5e.3b` on `2026-10-02` (UTC).

- **Sealed identity:** 8 lines, 740 bytes, `sha256:a12d32af8376aa1a871f4fa622ea7476348e30afffa9d525ef48ba955ce1fbf1`
- **Coverage:** D121; original report unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D121** — reference comparison loses irrational-call provenance required for T1 refusal.
  - Reproduce: assert approx:eps_num=sin(90 deg)==1.0 ACCEPTs true, baseline exit0, contrary
    contract4.2/9's T2-or-looser rule when an irrational result contributes.
  - Root: Val carries only kind/value and assertion applies distance without irrational provenance.
  - Impact: current numeric reference is not a general tolerance/provenance oracle.
  - Owner: G1-SLICE.5e.3, high priority before using reference for product tolerance execution;
    retain/prove through bindings/reads/operators. Does not block static name/kind checks, whose
    controls trap all numerical execution. No current reference approval of T1 provenance claimed.
