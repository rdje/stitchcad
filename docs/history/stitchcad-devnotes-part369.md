# Sealed archive — complete twelfth retention window book narrative

Immutable historical segment, sealed by leaf `G1-SLICE.5e.2.h13` on `2026-10-26` (UTC).

- **Sealed identity:** 15 lines, 955 bytes, sha256:f222a7b24ef4338e04c58b3d63c52ff0acb8431ec91a0964ecf7dd03143ea2e9
- **Coverage:** whole committed source interval from 87d0987a71d38c587a1da68e1810620d3e5e9b72.
- Whole source interval: docs/book/src/governance.md bytes 39081–40036, 955 bytes, sha256:f222a7b24ef4338e04c58b3d63c52ff0acb8431ec91a0964ecf7dd03143ea2e9
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

### Twelfth retention window

Window12 captures53 complete records from f168eec before more numerical-feature history would
exceed the unchanged64-file working limit. Its174350 original bytes are retained in a46666-byte
content-addressed payload. The fresh repository-volume fixture has no Git directory or captured raw
copies: public source comparison, read and materialization recover every captured file exactly.
All11 prior window descriptors/catalogs/payloads stay unchanged. Installation verifies copied bytes
before removing exactly53 unchanged regular, single-linked originals. Current links use window12
anchors; logical read addresses and offline retrieval remain stable. No schema/cap change or new
application dependency is introduced; whole closed scopes/journal/ledger remain retained records.

```bash
bash scripts/history_archive.sh prove-source window12
bash scripts/history_archive.sh read docs/history/stitchcad-devnotes-part331.md
```
