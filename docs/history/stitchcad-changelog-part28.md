# Sealed archive — copy-addressed sewing spans

Immutable historical segment, sealed by leaf `G1-SLICE.4c.1` on `2026-10-02`.

- **Sealed identity:** 20 lines, 1689 bytes, `sha256:6900e83eb66fdfd712b02a7a3d3397c19955536dbf75724567f389749391fed4`
- **Coverage:** STITCHCAD-G1-0011, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## STITCHCAD-G1-0011 - sewing spans address copies and permit disjoint self-seams (leaf `G1-SLICE.3c.2b.2`)

Immutable `SewingGraph`/`SeamSpan` content now names physical CutCopy identities and exact partial
ranges, with explicit endpoint correspondence, signed ease source/distribution and semantic stops.
Construction validates the complete cut plan, owned whole intervals and unique endpoints; stops
resolve to Notches or born-valid TurnPoints on their side. Symbolic amounts supply no defaults.
Geometry, walking/realized ease and profile/recipe value resolution remain explicit later obligations.

D35 closes: disjoint same-copy ranges may sew together, touching endpoints are legal, and overlapping
material intervals are refused after current-frame resolution. D57 closes: two copies of one Piece
have different neighbours, and removed targets remain missing rather than transferring their seams.
The fixture records the rule while retaining its existing edge-finish procedure until G2 constructs
actual folded-end ranges. Neither the five physical cuts nor its arithmetic goldens change.

Eighteen sewing contract tests + the graph privacy doctest pass; disabling either self-overlap or
whole-interval ownership refusal makes its regression red. Strict `make check`, wasm, warning-free
book, fixture/feature/glossary censuses, ledger probes and staged doctrines pass. The copy milestone's
full `make probes` passed all 22 suites. Shared anchor validation retains notch behavior and the
NotchError alias; original notch tests pass. Closed D35/D57 seal together in defects-part7.
Oldest CHANGELOG/DEV_NOTES entries roll over atomically. Next `.3c.3b` implements directed grainlines.
