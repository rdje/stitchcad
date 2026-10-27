# Sealed engineering continuity — part597

- **Identity:** 13 lines, 1223 bytes, sha256:82264056dbd50b77da7c1e5923f4ed99251c41d9f2ae76378a83d0a5a7b018a0
- **Coverage:** whole D290/D291 current closure ledger from 04a4dbb9ec3c16d348f9d4c9615796a843ea6095.
- Whole source interval: docs/tasks/PLANNING.md bytes 246–1469, 1223 bytes, sha256:82264056dbd50b77da7c1e5923f4ed99251c41d9f2ae76378a83d0a5a7b018a0
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

## D290 — metadata publication recopies the complete unused identity reserve
P2 fixed0442, owner G1-SLICE.6.8.2.2.2.1: shared immutable backing and checked forward prefix.
- [x] ROOT CAUSE: python3 -I -B native reserve/caller census prints remaining.to_vec() even for count0 rc=0.
- [x] ADDRESSED: cargo test -p sc-core --lib command::bus::identity:1 passed rc=0; native bus46 passed rc=0;
  expanded prefix/exhaustion checks and actual cancellation/replay/undo/branch nonreuse preserved.
- [x] NO REGRESSION: selected strict core/native Clippy and fmt finished rc=0; backing-copy census0 rc=0.
D291 P2 fixed0442 same owner: standalone Notch example policy-install count0/1 rc=0; explicit setup added.
D289 fixed0441; [whole native source-copy diagnosis/fix/privacy proof](../history/stitchcad-devnotes-part593.md).
D287/D288 fixed0440; [whole native bypass/fix and package metadata proof](../history/stitchcad-devnotes-part592.md).
D285 runtime fixed0437; persistence/reopen issuance/order remains owned/pending in G1-SLICE.7.
[Whole D285/D286 diagnosis and verified runtime/claim closure](../history/stitchcad-devnotes-part591.md).
D283/D284 fixed; [whole correction/proofs](../history/stitchcad-devnotes-part581.md).

