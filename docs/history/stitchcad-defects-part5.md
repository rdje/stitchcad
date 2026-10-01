# Sealed archive — StitchCAD defect census, closed D59

Immutable historical segment, sealed by leaf `G1-SLICE.3c.2b.1` on `2026-10-01`.

- **Sealed identity:** 16 lines, 1457 bytes, `sha256:bdf6db85cbf1c00d77a25d1c6abf16fb426bf05a889b2bdb46127478ce92c749`
- **Coverage:** `D59`; committed-baseline reproduction and closure evidence.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D59** — ontology implementation prose uses 15 API tokens without glossary ownership or
  chapter-local table declarations at committed `6abfac3`. Found by the physical-copy slice's
  glossary census; current new tokens also require declarations.
  - Reproduce: archive `6abfac3`'s `docs/book/src`, `ROADMAP.md` and `docs/tasks` to project-local
    scratch, then run `GLOSSARY_ROOT=<scratch> run_glossary_census.sh` → `15 failure(s)`, `rc=1`.
    These are Piece/range/notch API names, not new garment concepts; earlier focused checks did
    not run this token-coverage census after extending implementation prose.
  - Impact: the normative vocabulary boundary is broken despite the rendered examples being visible;
    full CI/probes would reject the document set.
  - Owner/schedule: **`G1-SLICE.3c.2b.1`, now**, add meaningful chapter-local API declarations,
    keep genuine new copy/plan concepts in the glossary, and require the glossary census for future
    ontology implementation updates. Do not widen the census's exemptions to silence the failure.
  - **Closed:** `.3c.2b.1` declares local Piece/range/notch/copy API vocabulary and adds genuine
    cut-plan/physical-copy concepts to the glossary; its A–Z index is regenerated. The same census
    now reports `310 terms / 9 parts / 158 tokens / 0 failure(s)`, `rc=0`, without adding exemptions.
    Future ontology implementation slices require this token census as a focused check.
