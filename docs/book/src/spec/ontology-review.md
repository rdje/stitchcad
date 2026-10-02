# Structural object-family review

`G1-SLICE.3c.4d.2` re-derived the sixteen ontology §4 objects against their required content,
immutable interfaces, focused contracts and the [implementation](ontology-implementation.md),
[construction](ontology-constructions.md) and [closure](ontology-closures.md) examples. Four families are structurally
complete. This closes the object-type work; it does not close gate G1 or certify a production release.

| Family and suites | Tested scope | Later proofs |
| --- | --- | --- |
| Pieces/copies (`piece_contract`, `cut_contract`) | Content/labels, quantity/pairing/fold rules, explicit copies | G2 geometry; Design links |
| Sewing (`sewing_contract`) | Partial/one-to-many spans, stops/ease, disjoint same-copy intervals | G2/G3 walking/ease |
| Marks/allowances (`notch_contract`, `grain_contract`, `allowance_contract`) | Profile bindings, directed references, width/corner intent | G2 geometry/offsets; G4 bindings |
| Garment construction contracts | Distinct kinds, operation origins, current composition and canonical sources | G2/G3 execution; Design/G4 bindings |

At that review, the fourteen listed object/support suites contained 149 contract tests. Additional
identity/range properties and unit tests brought sc-core to 242 regular tests plus 18 doc-tests,
including privacy checks. Test populations are snapshots, derived by `cargo test -p sc-core -- --list`;
`make check` executes them with strict formatting/lint. The WASM smoke build proves cross-compilation,
while real browser/runtime workflow proof remains G1-SLICE.11/.12. Independent physical review and
release evidence remain G6/G7; a green structural suite proves its tested contracts only.

Current consumers must check full interval evidence, unique point choices and every referenced
registry. Immutable objects can expose repairs after journal edits; construction-time approval is
not a current-state certificate. Borrowed Gather/buttonhole/hem/Pocket sources preserve canonical
content without resolving values or authorizing execution. Recipe/Design validates declarations,
operation kinds/dependencies, current copy/Piece/placement links and component contour repairs.
G2 proves physical contours, offsets, angles and walking; G3 executes construction/conservation;
G4 resolves profile values/states. Lining/fly scope refusals remain explicit before execution.
The director's SOTA, signoff and production-grade goal is discharged through these owned proofs,
not inferred from the existence of structural APIs.
