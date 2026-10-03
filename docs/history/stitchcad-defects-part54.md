# Sealed archive — D131 reserved diagnostic argument conflict

Immutable historical segment, sealed by leaf `G1-SLICE.5b.2c.1b` on `2026-10-03` (UTC).

- **Sealed identity:** 14 lines, 1255 bytes, `sha256:c94b9f9c08234d64b10c7bf4be098bf8979dde5cc4c21cb3c568b1adebaa3ac1`
- **Coverage:** D131 original report; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

### Defect D131 — reserved-name refusal cannot meet the rebinding argument row

- Discovered2026-10-03 (UTC), before initial namespace implementation. Owner: G1-SLICE.5b.2c.1a
  reproduction/proposal, then .1b diagnostic repair and .2 product namespace; priority blocking .2c.
- Reproduce: python3 -I -B docs/tasks/artifacts/formula_structure/reserved_diagnostic_review.py
  --mutations →120 reserved refusals/one ordinary rebinding/three actual assertion reds, rc=0.
  namespace eps_num/measurement and let eps_num:length=1 both refuse formula_rebinding with {} args.
- Contract3.1 forbids reserved rebinding; static annex uses formula_rebinding. Contract5.2 requires
  two recipe statement indices for that token. Reserved metadata has no prior recipe statement;
  an initial authored input has no attempt statement. Zero indices would violate context truth.
- Impact: product namespace cannot satisfy that literal argument contract. Recommended reserved
  variant retains token, name, reserved metadata and attempted source/origin, actual let ordinal/
  spans only when present; ordinary repeated lets retain both real indices. ADR-0003 records the
  concrete proposal. Director ruling required for the canonical conflict; repair scheduled at .1b.
