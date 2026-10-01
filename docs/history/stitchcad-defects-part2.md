# Sealed archive — StitchCAD defect census, closed D55

Immutable historical segment, sealed by leaf `G1-SLICE.3c.2a` on `2026-10-01`.

- **Sealed identity:** 19 lines, 1834 bytes, `sha256:e8c7c2f0a962c5a35ccd1ca721d94c3e49cad7ebc3046cd47c0edcf9abc18be4`
- **Coverage:** `D55`; discovery record preserved above its closure evidence.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D55** — a point-reference endpoint inventory cannot certify the interior of a boundary or sewing
  range after fragmentation. Found and reproduced by `G1-SLICE.3c.1` (`2026-10-01`); the point ledger
  itself is correct for the registrations it receives, but using its verdict alone for an edge range
  would hide a deleted interior fragment.
  - Reproduce: `cargo test -p sc-core --test piece_contract
    endpoint_inventory_cannot_certify_the_interior_of_an_edge_range` → `1 passed`, `rc=0`. Split an edge
    at one third, split its second fragment at one half, delete the middle fragment, and resolve the
    original start/end: both endpoints still resolve and endpoint registrations report `Releasable`.
    The complete authored edge has lost its middle. The tracked test asserts both observations.
  - Impact: no release generator exists, and pieces visibly defer geometric validity; no invalid release
    is emitted. The risk becomes active when sewing spans or the command bus interpret endpoint repair
    counts as range completeness. That inference would violate ontology §1.1's no-silent-reassignment rule.
  - Owner and schedule: **`G1-SLICE.3c.2`, immediately next**, implements a full-range resolution/repair
    contract before sewing spans use it; `.6` consumes that contract in design validation and G2 still
    proves geometric contour closure. `.3c.1` documents the endpoint query's exact scope now.
  - **Closed:** `G1-SLICE.3c.2a` implements the exact whole-interval fold and piece range queries.
    `cargo test -p sc-core --test range_contract` → `13 passed`, `rc=0`; D55 produces the held-range
    repair naming the middle fragment's delete operation, with both endpoint answers still resolved.
    Removing the delete arm makes the D55 regression test fail (`rc=101`); restored source passes.
