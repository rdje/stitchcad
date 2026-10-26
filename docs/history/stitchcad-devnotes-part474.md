# Sealed archive — whole thirteenth-window governance receipt

Immutable historical segment, sealed by leaf `G1-SLICE.4d.2` on `2026-10-26` (UTC).

- **Sealed identity:** 17 lines, 983 bytes, sha256:b68f5f6d8a4def89ccf50ea1e3ec8030a510ed6b52d4053794197b7a624150f2
- **Coverage:** whole committed source interval from 140422e8fc036994f5c9d04c8076ab2f25e0515c.
- Whole source interval: docs/book/src/governance.md bytes 35614–36597, 983 bytes, sha256:b68f5f6d8a4def89ccf50ea1e3ec8030a510ed6b52d4053794197b7a624150f2
- **Retrieval:** bytes below the first rule and blank line; original terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

### Thirteenth retention window

Window 13 captures 52 complete records from `87d0987` at the unchanged limit of 64 working files.
Its 107786 original bytes are retained in a 31312-byte payload. The fresh local fixture has no
Git directory or raw captured copies; public source/read/materialization recover every file exactly.
All 12 earlier manifests/catalogs/payloads remain byte-identical. Verified installation retires only
52 unchanged regular single-link originals. Live links use window13 anchors; logical read addresses
and offline retrieval remain stable. No schema/cap or application dependency changes.

The complete window 12 narrative remains available through the [archive reader](annexes/archive-reader.md).

```bash
bash scripts/history_archive.sh prove-source window13
bash scripts/history_archive.sh read docs/history/stitchcad-devnotes-part365.md
bash scripts/history_archive.sh read docs/history/stitchcad-devnotes-part369.md  # complete window 12 narrative
```

