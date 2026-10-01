# Sealed archive — StitchCAD defect census, closed D60

Immutable historical segment, sealed by leaf `G1-SLICE.3c.2b.1` on `2026-10-01`.

- **Sealed identity:** 13 lines, 1268 bytes, `sha256:3bc74c836d1639197e7561a7e148e6c11abab0aeaad783d6ed1214d21475aca1`
- **Coverage:** `D60`; relocation refusal and evidence revalidation.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D60** — moving historical doc-only `.1`/`.2` checklists into G1's evidence sibling during a
  code commit exposes ROOT CAUSE boxes with no tool-output signature. The original doc-only leaves
  did not trigger the staged-code gate; the relocated evidence must meet it now.
  - Reproduce: stage the new sibling with the copy code and run `make gate` → TASK-ACCEPTANCE
    refuses the sibling's first ROOT CAUSE box, `rc=2`. Its `.2` ROOT CAUSE likewise has only prose.
  - Impact: containment's required split cannot commit until both historical bullets carry evidence;
    adding a newer first checklist would mask the problem instead of validating the relocation.
  - Owner/schedule: **`G1-SLICE.3c.2b.1`, now**, read `git show eb83f01`, supplement both historical
    ROOT CAUSE bullets with exact re-derived delivery evidence, and verify all four moved checklists.
  - **Closed:** `.3c.2b.1` retains the original prose and adds re-derived `git show eb83f01`
    invocation/result/rc inside both historical ROOT CAUSE bullets. An explicit per-checklist audit
    confirms all three gated bullets in `.1`/`.2`/`.3a`/`.3b` have evidence. The staged `make gate`
    now returns `=== all doctrines green ===`, `rc=0`, without changing the gate or its signatures.
