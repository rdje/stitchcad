# Sealed archive — lazy conditional value scope ledger

Immutable historical segment, sealed by leaf `G1-SLICE.5e.2.2` on `2026-10-26` (UTC).

- **Sealed identity:** 4 lines, 297 bytes, sha256:5926347a6994d25e3339eadd80a6d4a324ecaed8f9064c0526699b8ae411bc3d
- **Coverage:** whole committed source interval from 570d5b52c4576ff935b7ea0fde6eff748bc1d909.
- Whole source interval: CHANGELOG.md bytes 13754–14051, 297 bytes, sha256:5926347a6994d25e3339eadd80a6d4a324ecaed8f9064c0526699b8ae411bc3d
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

## STITCHCAD-G1-0273 — lazy conditional value control

.5e.2.1: check supplied Boolean/equal-numeric branch kinds before callbacks, then run only the selected branch.
Keep condition/taken sources and actual body/output-kind failures; untaken Unknown is unread. Whole AST execution remains open.
