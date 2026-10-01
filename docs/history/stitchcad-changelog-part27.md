# Sealed archive — physical cut copy identities

Immutable historical segment, sealed by leaf `G1-SLICE.4b.2` on `2026-10-02`.

- **Sealed identity:** 20 lines, 1663 bytes, `sha256:b3824493c1d46d33519a54faac7a72fdef4fa421d7159437bd912f9c93927f79`
- **Coverage:** STITCHCAD-G1-0010, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## STITCHCAD-G1-0010 - physical cut copies have explicit stable identities (leaf `G1-SLICE.3c.2b.1`)

Per the director's D57 ruling, `CutPlan` carries immutable `CutCopy` identities separately from the
pattern Piece. Callers supply copy ids, Piece ids and authored/reflected orientation. Validation checks
unique/disjoint identities, existing Pieces, exact quantities and equal mirrored-pair populations;
Single and separate L/R members retain authored orientation. List order supplies no identity and a
removed copy never transfers its id to a replacement. Geometry transforms remain deferred to G2/V1.

Nine contract tests and the privacy doctest pass; disabling quantity refusal makes its regression red.
Strict `make check`, wasm, warning-free book, feature/glossary/tree censuses, ledger probes and doctrine
gate pass. D59 is fixed: the glossary census reproduced 15 undeclared API terms against `6abfac3`;
a meaningful local API table plus the two new glossary concepts brings the census to zero failures.
The new terms are indexed from the producer. D57 stays open until the sewing graph exercises copy ids.

Completed `.1`, `.2`, `.3a` and `.3b` checklists move to `G1-SLICE-evidence.md` before
this append would take the parent past 1000 lines. Coverage confirms a fourth legitimate sibling. D60 closes after the staged gate exposes the
historical doc-only ROOT CAUSE bullets: `.1`/`.2` retain their prose plus re-derived delivery evidence,
and all four moved checklists are audited separately.
The copy-identity decision records replacement/orientation rules and the required token census.
Next `.3c.2b.2` lands sewing spans and settles D35 explicitly.
