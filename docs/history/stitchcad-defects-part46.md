# Sealed archive — D126 parameter placement bound

Immutable historical segment, sealed by leaf `G1-SLICE.5b.1c.1` on `2026-10-02` (UTC).

- **Sealed identity:** 8 lines, 726 bytes, `sha256:d95c5eed05f6aee38381f3dd01ed60f9b0056d7f79f14a7c236e49cb0e7228c4`
- **Coverage:** D126; original report unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D126** — formula selector prose treats a10m bounding box as a10m edge-length bound.
  - Reproduce: allowed circular arc radius5m/sweep270deg fits a10m square but has length
    23561945um in the reference; half a10^-6 ratio quantum spans11.7809725um, above T2=10um.
    Independent pi>3 inequality gives distance>11.25um; stdlib calculation agrees, baseline exit0.
  - Root: grammar6.1 converts half a ratio quantum to5um using bounding-box width instead of
    actual curve length. Arc length is not bounded by its box width.
  - Impact: unsupported positional tolerance promise. Owner: G1-SLICE.5b.1c.1 fixes documentation
    and watches independent counterexample now; actual selector error budgets remain .5f.3/G2.
