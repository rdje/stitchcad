# Sealed engineering continuity — part854

- **Identity:** 7 lines, 598 bytes, sha256:47c40790afe5c0dda8bee16a778e8406c2b6ee78323d253927f5f329a4787a70
- **Coverage:** whole prior D307 through D369 live ledger tail from e9a11dbbfbb3fa557f63e4fe0711d517089aaafb.
- Whole source interval: docs/tasks/PLANNING.md bytes 94094–94692, 598 bytes, sha256:47c40790afe5c0dda8bee16a778e8406c2b6ee78323d253927f5f329a4787a70
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

D307–D368:[whole prior ledger](../history/stitchcad-devnotes-part848.md). D334/D223/D156 remain owned.

## D369 — G1 schema completion incorrectly waits for G2 geometry

P1 fixed0685;owner G1-SLICE.5f.1.c.stage-order. Repro:0680 offset.outputs required actual G2 fragments before G1 schema closure;G2 entry requires G1 closed.
Root:0680 conflated output contract with geometry execution. Impact:cycle can strand product delivery.
Fix:native contract acceptance separate from actual G2/G3 integration (.5f.4.geometry). Focused15/fault controls/book pass;G1 spike/G2 entry and runtime unchanged.
