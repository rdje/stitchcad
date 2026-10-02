# Sealed archive — D83 numeric boundaries and D97 planning drift

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3b.3b.3c.2` on `2026-10-02`.

- **Sealed identity:** 23 lines, 2062 bytes, `sha256:9a59f1f2b79ac2be610ba232dbe9c1439b57a06df047190111b2b31ee4f2e3aa`
- **Coverage:** D83 and D97; original descriptions retained unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D83** — reference numeric domains are measured without complete typed refusal enforcement.
  - Original reproduce: literal_diagnostic.py accepted count 2^128 (129 numerator bits) and length
    1000000001 um, with no formula_domain; source see only updates max_bits, L8 is a census verdict.
    L2 stores rnd(val.v), so 360 deg remains unnormalized at a stored angle binding. The director's
    D84 ruling confirms this raw sweep behavior is desired; it is not a missing modulo defect.
  - Impact: reference success is insufficient evidence for numeric/binding domains.
  - Width repair: .5a.3b.3a.2 now refuses reduced values above128 bits at input/result boundaries;
    61 controls/twelve actual reds verify this portion. Scalar .3b.2 now verifies signed length/area
    and nonnegative count with57 controls/eleven actual reds. Binding .3a verifies80 controls/twelve
    actual reds. Canonical .3b verifies146 controls/twelve actual reds and closes D95; complete
    review .3c remains before D83 closure.
  - Owner/schedule: G1-SLICE.5a.3b.3b, after width enforcement and before product normalization.
    D84 records the director’s direction/sweep distinction; rational/scalar repairs implement no
    formula modulo. Remaining signed-angle verification stays owned by .3c.

- **D97** — live scalar parent retains a superseded canonical-i64 goal after D95.
  - Reproduce: rg canonical/binding in docs/tasks/G1-SLICE.md matches .5a.3b.3b Goal:
    canonical/binding i64 bounds. Formula2/grammar4 and decision_literals instead allow exact
    canonical nodes up to128-bit rational magnitude and require i64 only for bound numeric values.
  - Root/impact: received D95 ruling updated the child/contract but left the pre-ruling parent Goal;
    a resumed agent could incorrectly narrow canonical nodes and reject signed i64 MIN’s wide child.
  - Owner/schedule: G1-SLICE.5a.3b.3b.3c.2 corrects this planning contradiction now during D83 review;
    retain past evidence, verify current parent/contract agreement and existing canonical146 controls.
