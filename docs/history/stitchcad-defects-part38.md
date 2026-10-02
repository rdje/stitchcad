# Sealed archive — D108 current archive copy offset

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3e.2` on `2026-10-02`.

- **Sealed identity:** 11 lines, 927 bytes, `sha256:30ee8df6e8c107deaeff2ac6efc8b329f5fc906d01efbd4881c2f36ce2c3e714`
- **Coverage:** D108; report wording retained, heading normalized to the defect-entry marker.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D108** — current archive payload acquired one extra heading byte (2026-10-02)

- Priority: P2; owner/fix now `G1-SLICE.5a.3e.2` before its commit.
- Reproduce: current G1-0066 ledger run reports REAL/COVERAGE failures; new changelog-part61
  payload starts with ### STITCHCAD-G1-0044 rather than original ##, and its digest is14bb73b6…
  rather than the declared5484241b…. The sealed payload has2472 bytes, original2471.
- Root cause: one-shot archive update used delimiter index+7 for a six-byte newline/rule/blank
  delimiter, retaining the first old payload byte. Tools inspect actual extracted bytes against HEAD.
- Impact: predecessor payload is not byte-identical and coverage/digest declarations are false.
  Uncommitted generated archive only; HEAD retains the authoritative originals. Repair by splitting
  at the exact delimiter, compare full original payload, then rerun ledger/archive retention controls.
