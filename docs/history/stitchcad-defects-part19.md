# Sealed archive — literal identity and publication drift

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3b.1` on `2026-10-02`.

- **Sealed identity:** 19 lines, 1573 bytes, `sha256:6c99bb36f87c90d0c8b64c3525dee6cbf59ae251d1256d1a73defc7d0ad3c001`
- **Coverage:** D79, D80, D81, closed and verified by STITCHCAD-G1-0042; descriptions unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D79** — reference unit literal evaluation disagrees with its canonical integer identity.
  - Reproduce: actual reference through formula_input.py load_context; parse/infer/evaluate
    0.00004 cm + 0.00004 cm → length 4/5, rounded binding 1. Respelling the two individual
    canonical length:0 literals as 0 um + 0 um → length 0, rounded binding 0.
  - Root: L1 rounds literal canonical displays, while p_atom retains exact fractional unit values.
  - Impact: canonically identical formulas produce different reference results; this oracle cannot
    sign off product literal normalization or identity-preserving recipe respelling.
  - Owner/schedule: G1-SLICE.5a.3b, next after primitive rounding; fix/verify before .3c/.3d product
    canonicalization. Check stored-angle/literal domain parity with the authoritative contracts.

- **D80** — units annex repeats section number 2.2.
  - Reproduce: rg '^### 2\.2 ' docs/book/src/spec/units-and-tolerances.md → two headings.
  - Impact: numeric clause references are ambiguous after the rounding annex added by G1-0041.
  - Owner/schedule: G1-SLICE.5a.3b.1, immediate small lockstep repair; keep display 2.2/public round 2.3.

- **D81** — LIVE_STATUS G1 next pointer still names completed reference input .5a.2b.1.
  - Reproduce: compare G1 row to MEMORY/task frontier .5a.3b and git log G1-0039 through G1-0041.
  - Impact: published implementation summary directs a fresh reader to work already complete.
  - Owner/schedule: G1-SLICE.5a.3b.1, immediate small lockstep repair with measured active frontier.
