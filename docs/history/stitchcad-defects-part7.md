# Sealed archive — StitchCAD defect census, closed D35 and D57

Immutable historical segment, sealed by leaf `G1-SLICE.3c.2b.2` on `2026-10-01`.

- **Sealed identity:** 43 lines, 4172 bytes, `sha256:79373c0f1d7f1792503a781838522071ce793b3872a5d7cb4a1f487766d1ea22`
- **Coverage:** `D35`, `D57`; original questions, director ruling and verified contracts.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D35** — the ontology does not say whether a `SeamSpan` may name the same piece on both sides (a
  *self-span*), and the reference fixture needs an answer for its waistband ends: a band folded lengthwise
  has its two short ends folded right sides together and stitched across, which joins one piece to itself.
  - Reproduce: `grep -n 'the \*\*two sides\*\*' docs/book/src/spec/ontology.md` → §4.2 requires
    "(piece, EdgeRef, parameter range) each" and permits partial and one-to-many spans, and is silent on
    both sides naming one piece; `grep -ci 'self-span\|same piece' docs/book/src/spec/ontology.md` → `0`.
  - Impact: small today — the fixture records the ends as an edge finish and no golden depends on them —
    but it is a type invariant `sc-core` must settle either way. If a self-span is legal, `walk`, `true`
    and the V1 stitcher must handle a span whose two sides share a piece and a frame; if it is illegal, a
    folded band's ends need another representation, and the fixture's assembly order is incomplete without
    one. Whichever way the first implementation happens to compile is the D27 class one layer up: a
    contract settled by accident.
  - Owner: `G1-SLICE.3` (`sc-core` ontology v1 types and invariants) — the invariant must exist before the
    type does, and `G3-GRADING.2`'s `walk`/`true` and `V1-ASSEMBLY.2`'s stitching both consume the answer.
    `G0-CONTRACT.13d` recorded the question in the fixture chapter (§8 and §11) so the fixture does not
    settle it silently, and deliberately chose nothing.
  - **Closed:** `.3c.2b.2` permits same-copy seams with disjoint positive-length interiors and
    shared endpoints, refuses overlap/identical intervals, and checks current fragments after merge.
    The independent self-seam tests pass; disabling the refusal makes the merged-overlap regression
    red (`rc=101`). `cargo test -p sc-core --test sewing_contract` → `18 passed`, `rc=0`.
    Ontology §4.2/§10 and fixture §8/§11 state the rule without inventing folded-band geometry.

- **D57** — a sewing side addresses a pattern Piece and edge range, while a Piece can request
  multiple physical cut copies. The spec has not decided whether copies have stable identities in
  the sewing graph or are expanded later by assembly. Found `2026-10-01` before sewing-schema code.
  - Census: `rg -n -i 'copy id|copy identity|copy reference|cut.copy|physical cop|occurrence|multiplicity|mirrored pair'
    docs/book/src ROADMAP.md crates/sc-core/src --glob '*.md' --glob '*.rs'` enumerates the quantity,
    mirrored-pair and print obligations, but no copy selector or identity contract. Ontology §4.2's
    side tuple is `(piece, EdgeRef, parameter range)`; the fixture distinguishes back members by Piece
    identity. Instantiation §2's output is sized pieces and a graph, without a cut-copy addressing rule.
  - Impact: two physical copies may have different neighbours; choosing which copy a span joins by
    renderer convention would silently widen the canonical semantics and risk incorrect assembly.
  - Owner/schedule: **`G1-SLICE.3c.2b` before implementation**. The director was asked whether to give
    every physical copy a stable identity (recommended) or retain a pattern-level graph with expansion
    at assembly. This is a product-model decision, not an access request. Independent piece/mark work
    proceeded while the answer was pending; no copy-address schema was inferred from elapsed time.
  - **Director ruling (`2026-10-01`):** give every physical cut copy a stable identity so its seams
    can differ. `.3c.2b` implements and verifies that contract next; D57 stays open until delivered.
  - **Closed:** `.3c.2b.1` supplies explicit CutCopy identities; `.3c.2b.2` addresses span sides
    by those identities. Two copies of one pattern use the same source range and distinct neighbours;
    replacing a copy exposes the original missing id instead of transferring its seams.
    `cargo test -p sc-core --test sewing_contract` → `18 passed`, `rc=0`; all copy contracts pass.
    The director's ruling, ontology examples and the two implementation decisions stay aligned.
