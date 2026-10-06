# Sealed archive — D196 report and focused verified closure

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.f.n11.3` on `2026-10-06` (UTC).

- **Sealed identity:** 22 lines, 1674 bytes, sha256:a81ef47caa63271904b7c6d131f1a89c07d3e009d68ab7fa08d02c91cd7a0ae8
- **Coverage:** whole new-over68a6f9da91da7ed95633c9e393d8cfc849d58a51 authored docs/tasks/PLANNING.md bytes 80495–81115; preparation/report plus explicitly appended focused observations.
- **Retrieval:** content below the first rule and following blank line; exactly one terminal newline.
- **Write policy:** none; later verdicts belong in the live pointer.

---

- **D196** (P1, fix now G1-SLICE.5b.4c.h2.b.p.o.f.n11.3): ledger NO-DUP scans every logical history
  Markdown, so whole task receipts in DEV228/230 with live work-unit headings falsely collide with
  CHANGELOG entries. Repro wanted-plan-ledger.log REAL NO-DUP FAIL0188/0189; full run7pass2fail.
  Function sealed census line89 uses *.md while COVERAGE/POINTER select changelog-part identities.
  Impact: correct immutable task retention blocks signoff; constrain only NO-DUP's entry population,
  add same-ID nonledger green/legacy-wide red, preserve true duplicate/all-history descriptor
  controls and archive bytes.

### Verified focused closure
- The exact actual ledger function with its former broad sealed census falsely
  reports NO-DUP for the live work-unit's independent dev-notes/defect receipts.
- Corrected census selects changelog-part identities, matching existing COVERAGE/
  POINTER roles. Independent same-ID nonledger fixture passes all7 rule verdicts;
  a genuine changelog duplicate gives exactly the NO-DUP refusal. Focused helper0.
- Tracked runner adds NONLEDGER green and NONLEDGER-LEGACY-RED; existing genuine
  DUP-RED and all-history DESCRIPTOR checks remain. Shell syntax/diff0.
- No admitted archive is rewritten; complete Source task bytes/headers remain
  retrievable. The full11-arm ledger and13 pointer contracts are confirmatory,
  pending in the active leaf, which must consume them before signoff/next activation.
- D196 focused repair is verified; this receipt makes no unobserved full-suite claim.
  Product grammar/engine/push cadence unchanged. Existing P3 pipeline issue keeps
  .b.s ownership and is not included in this repair.
