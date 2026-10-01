# Sealed archive — whole-range reference resolution and structural pieces

Immutable historical segment, sealed by leaf `SPINE.19.2` on `2026-10-01`.

- **Sealed identity:** 39 lines, 3139 bytes, `sha256:30ff1380d2f9f354103d678771e30f24477a56147dfaca8257d095e0e882a011`
- **Coverage:** STITCHCAD-G1-0007, STITCHCAD-G1-0006, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## STITCHCAD-G1-0007 - whole-interval reference resolution keeps lost interiors visible (leaf `G1-SLICE.3c.2a`)

`IdentityLedger::resolve_range(EdgeRange)` now folds the complete positive-length interval through
split/merge/reverse/delete/offset. `RangeResolution` preserves ordered directed live fragments and
`RangeRepairTask`s for deleted or trimmed portions, missing source edges and exact-arithmetic refusal.
Endpoint point queries stay separate, so a full-coverage interval can still carry a boundary choice.
No query rewrites the stored reference or claims geometric validity or approval. Piece full-edge
queries consume this contract; the point-only registration verdict keeps its original scope.

D55 is fixed: deleting a middle fragment produces a visible range repair while both endpoints resolve.
The regression test goes red if the range-delete arm is removed. Thirteen range tests cover exact
partial bounds, traversal order, narrow gaps, arithmetic refusal and a recorded-seed differential
comparison with the existing point resolver. `make check`, `make wasm`, `make book`, doctrine gate,
feature-matrix census and ledger probes pass. The pre-code decision records the coverage/point/geometry
boundary; next `.3c.2b` implements sewing spans and resolves D35.

The same append rolls CHANGELOG's oldest entry into `part14` and DEV_NOTES' oldest two lessons into
`devnotes-part8`; closed D55 moves to immutable `defects-part2`. Their content identities are re-derived
by the ledger probes. The new Knowledge Map record fits after its interchange orientation entry is
tightened; D53's durable generator remedy remains separately owned.

## STITCHCAD-G1-0006 - immutable structural pieces, with geometry visibly deferred (leaf `G1-SLICE.3c.1`)

`sc_core::ontology::piece` now builds immutable `Piece` objects from editable `PieceDefinition` input.
Directed cyclic cut loops must be nonempty, distinct and live in the identity ledger; construction lines
must also exist. Cut quantities, mirrored pairs, fold-edge declarations, material explanations and complete
print text are checked with typed `PieceError` diagnostics. Quantity, pair and fold print fields derive
from the cut plan. Every piece reports `GeometricValidation::DeferredToG2`; no winding, simplicity,
containment or geometric-closure claim is made.

The object-type leaf `.3c` now has four independently committed children. Piece endpoint queries expose
repair tasks after edits without rewriting authored content. A tracked counterexample proves endpoints
cannot certify an entire fragmented edge (D55); the next child `.3c.2` owns full-range resolution before
sewing spans use it. Ontology §10 documents the implemented API and its limits.

Validation: `make check` (fmt, strict clippy, unit/property suites and private-content compile-fail test),
`cargo test -p sc-core --test piece_contract` (12 contract tests), `make wasm`, `make book`, doctrine gate
and changelog-ledger probes, all green. Startup compared the neutral README, claim-verification and
containment policy bodies with their read-only sources: no differences. Cleanup remained within 24 hours.
