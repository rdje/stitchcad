# Sealed archive — whole original pending retention-capacity declaration

Immutable historical segment, sealed by leaf `G1-SLICE.4c.5` on `2026-10-26` (UTC).

- **Sealed identity:** 5 lines, 394 bytes, sha256:6de27d3e6613a30de99eab56173860e4760a441d8851afb37ab9724039e621bd
- **Coverage:** whole committed source interval from 44ec7d086a2716d5a56f28dea3ef4e21102a182e.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 33935–34329, 394 bytes, sha256:6de27d3e6613a30de99eab56173860e4760a441d8851afb37ab9724039e621bd
- **Retrieval:** bytes below the first rule and blank line; original terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.4c.5.h16.capacity`
  Status: `pending`
  Goal: P1 design/verify bounded retention continuation before the next packing after16; current schema refuses17.
  Preserve immutable records/manifests, logical addresses and all independent caps; do not silently raise bounds.
  Trigger: before future required seal exceeds64 with16 windows registered; product slices proceed meanwhile.
