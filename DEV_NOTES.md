# DEV_NOTES.md

Detailed technical notes — root cause, implementation, validation — per slice. The
engineering-continuity surface (not the public docs; that's `docs/book/`). Newest first.

## _(2026-10-01)_ — sewing joins physical material intervals, not pattern names

- D57's copy identities make two copies of one Piece distinct sewing domains. D35 allows a physical
  copy to join itself only where current positive-length interiors are disjoint. Comparing held edge
  names cannot prove that: a merge can make formerly different names share a current frame. The
  constructor compares resolved fragments and checks every owned portion, including hidden interiors.
- Correspondence, journal traversal and reflection remain separate authored facts. Stops use semantic
  Notches/TurnPoints rather than physical notch representation. Ease is a signed declared source and
  explicit allocation; structural domains are checked, actual walking/arc lengths and resolved values
  are later obligations. A typed failure never silently stretches or reassigns material.
- Eighteen tests cover copies with different neighbours, partial/one-to-many spans, self-seams after
  merge/reversal, interior ownership/repairs, endpoint choices, stop/ease domains and removed targets.
  Disabling self-overlap and interval ownership refusals each produces a red regression. Existing
  notches retain behavior after extracting shared anchor validation; strict checks and WASM pass.
- promotion: promoted by `decision_sewing-spans-address-copies-and-permit-disjoint-self-seams.md`.

## _(2026-10-01)_ — physical-copy identity is authored content, independent of list order

- D57's director ruling makes every physical copy separately addressable. The complete cut plan
  validates explicit ids, source Piece, quantity and authored/reflected populations. It neither
  generates missing copies nor duplicates pattern geometry. Reordering retains identity; replacing
  a copy leaves the removed identity absent so graph validation can expose its missing target.
- Nine tests cover identity/reordering/replay, disjoint namespaces, missing targets, exact counts,
  mirrored populations and authored-only modes. Disabling quantity refusal makes its regression
  red. The plan's private content is checked by a compile-fail doctest; strict checks and WASM pass.
- D59 was already present in the committed docs: a snapshot census at `6abfac3` reports 15 API tokens
  unaccounted for. Chapter-local declarations explain API names without polluting garment vocabulary;
  only cut plan and physical cut copy add new glossary concepts. The regenerated index and census
  pass. Every future ontology update must run that census, alongside feature coverage and rendering.
- D60 appeared only when the moved historical doc-only checklists met the staged-code gate. Their
  `.1`/`.2` ROOT CAUSE bullets gain re-derived evidence from the actual delivery commit; all four
  moved checklists are checked individually before the staged gate passes. This revalidation follows
  the existing `decision_acceptance-evidence-per-leaf.md`, rather than masking old boxes with a new one.
- promotion: promoted by `decision_physical-cut-copies-have-stable-identities.md`.

## _(2026-10-01)_ — semantic anchoring and target-profile resolution have separate obligations

- A valid notch anchor does not supply factory-specific dimensions or encoding. Stable logical
  parameter declarations retain all sample/production fields without copying values or uncertainty
  flags. `DeferredToG4` exposes the missing binding/type/evidence check; G1 has no physical defaults.
- Anchor ownership must use resolved ranges and positions. After merging an owned edge with a foreign
  one, only its surviving portion belongs to the Piece; reversal changes both frames. Seven contract
  tests cover that boundary, split choice, exact merge, deletion and metadata immutability. Disabling
  the membership refusal makes the partial-merge regression red; strict checks and WASM remain green.
- D58 was a contradictory future acceptance, not another policy: canonical release §8 explicitly
  supplies sidecar draft output and forbids defaults. The dependent G4 leaf now matches that authority.
- promotion: promoted by `decision_profile-bindings-stay-symbolic-at-g1.md`, recorded before code.

## _(2026-10-01)_ — separate L/R members differ from one even-total pair request

- The canonical fixture's back pair is two Piece identities, each cut once. `.3c.1` enforced only
  an even-total pair mode, so it could not encode the fixture's L/R member labels. `PairMember`
  adds explicit handedness and a distinct companion identity; the old pair mode remains supported.
- A fixture-shaped test guards both cut-one quantities and reciprocal label metadata. Applying an
  even-total restriction to every pair mode turns that test red; self-companion is also a typed refusal.
  The constructor cannot prove companion existence or mirroring from one definition: `.6` owns the
  collection checks and G2 the geometry. These obligations are explicit rather than presumed.
- The related physical-copy addressing question (D57) is a separate, unspecified graph contract.
  The director's decision is pending; marking it explicit allows independent marks to proceed.
- promotion: promoted by `decision_piece-pair-members-have-explicit-handedness.md`.

## _(2026-10-01)_ — interval coverage and endpoint identity answer different questions

- `G1-SLICE.3c.2a` fixes D55 with a pure exact whole-interval journal fold. Split and offset partition
  ranges by intersection; merge rescales by declared lengths; reverse reflects bounds and direction;
  deleted or trimmed positive-length content becomes a visible repair. No sampling can certify an
  interval: a nanowide gap escapes a hundredths grid and is still detected by the interval fold.
- Coverage, point ambiguity and geometry remain separate. The result retains endpoint point queries
  alongside ordered interval portions. A split-boundary choice is not silently picked just because
  a positive-length interval maps uniquely. G2 still owns continuity and geometric closure.
- Thirteen range tests include a differential comparison against the already-tested point fold,
  exact integer-length merge expectations, terminal repairs and a deliberately disabled delete arm
  observed red. Piece range queries now catch the original counterexample. Existing suites stay green.
- promotion: promoted by `decision_range-resolution-preserves-entire-interval.md`, recorded before code.

## _(2026-10-01)_ — structural pieces and the endpoint/range distinction

- `G1-SLICE.3c.1` implements immutable pieces with private validated content. A public definition is
  editable input; the constructor checks live references, distinct cyclic cut loops, the cut plan,
  complete labels and explicit unresolved-material reasons. Cut quantity means total physical copies;
  mirrored pairs require an even quantity. Label cut information derives from the plan.
- The geometric state has only `DeferredToG2`; cyclic ordering cannot prove endpoint coincidence,
  winding or containment. Endpoint queries likewise do not certify an entire contour. The tracked D55
  counterexample deletes a middle fragment while both original endpoints still resolve. `.3c.2` owns
  the range contract immediately next, before sewing spans can make that mistaken inference.
- Checks: 12 piece-contract tests, existing unit/property suites, a compile-fail privacy check, strict
  clippy, wasm cross-build and book build. The first clippy run refused manual divisibility syntax;
  using the toolchain's integer predicate resolved it before signoff.
- promotion: declined (structural/geometric separation is already a decision record; the new range
  risk is an open defect and scheduled implementation contract in the task tree, not a settled rule).

## _(2026-10-01)_ — the persistent-identity contract: a reference is never rewritten, the journal folds

- `G1-SLICE.3b` landed the persistent-identity contract — `sc_core::ontology::topology`'s `IdentityLedger`:
  an append-only journal of typed edits and a pure fold resolving a held `(EdgeRef, Param)` through split,
  merge, reverse, delete and offset-fragmentation. 62 unit tests + 8 recorded-seed properties green, and
  `sc-core` still cross-builds to wasm.
- The design point worth keeping: **an edit never rewrites a stored reference.** The obvious implementation
  — mutating every consumer of the edited edge in place — is the silent reassignment §1.1 forbids, and a
  missed consumer is silent corruption. Instead the reference is immutable and resolution is a fold of the
  journal, so no consumer can be missed: the answer is computed *from the reference the consumer holds*.
  Repair state (`open_repairs`, `release_readiness`) is **derived** on every query, never stored, so it cannot
  drift from the journal — the discipline every derived-vs-hand-kept count here enforces. Undo (`.6`) then
  becomes journal algebra, not consumer archaeology.
- The exact-arithmetic choice paid off in the property suite: a split↔merge round trip returns the IDENTICAL
  reduced `Rational` (zero drift), and merge's arc-length recomputation is checked against its defining
  proportion cross-multiplied in `i128` — an oracle sharing no code with the fold. A fixed-point parameter
  would drift under the same round trip.
- One contract subtlety the tests pin: at a split point the reference resolves to BOTH fragments and the
  ledger never picks — the consumer states a `SplitSide`. If one side is later deleted, the fold still offers
  the survivor and shows the task on the dead side; whether it is an open repair depends on the stated side.
  "No silent reassignment" holds on every path.
- promotion: promoted by `decision_reference-resolution-journal-fold.md` (which carries `answers:`) — the
  durable boundary (immutable references, fold resolution, derived repair state, offset-consumes-declared-
  intervals until G2 geometry) is recorded there so `.3c`/`.6`/`.7` inherit it rather than re-litigate.

## _(2026-09-30)_ — the identity layer: determinism is a property of the generator, not the id

- `G1-SLICE.3a` landed G1's first new product code: `sc_core::ontology`'s `EntityId` (a hand-rolled
  dependency-free ULID), the injected `IdGenerator`, `EdgeRef`/`PointRef`/`LocalTag`, and the bounded exact
  `Rational` a parameter is stored in — 40 tests (31 unit, 9 recorded-seed properties), and `sc-core` still
  cross-builds to wasm.
- The design point worth keeping: a real ULID embeds a wall-clock timestamp and randomness, yet recipe
  re-evaluation and CLI replay must be byte-identical and canonical content carries no wall-clock. So
  **determinism lives in the generator, not the id** — `EntityId` is just 128 bits, and whoever creates objects
  injects either a `DeterministicIdGenerator` (a counter, for tests and replay) or a clock-plus-entropy
  generator (production, from the command bus at `G1-SLICE.6`). The clock never enters domain code, so the same
  commands always yield the same ids.
- Two house conventions re-confirmed, not reinvented: fallible arithmetic is `checked_*`, not `add`/`sub`
  (clippy's `should_implement_trait`; the precedent `sc-units` set), and a test *helper* that is not a `#[test]`
  fn is not covered by `.clippy.toml`'s `allow-expect-in-tests`, so it carries its own targeted `#[allow]`
  rather than swallowing a failure with `unwrap_or`.
- promotion: declined — the durable design choices are the three decision records `G1-SLICE.3` landed; this is
  one slice's implementation history, and the two conventions are already the house style, not a new rule.

## _(2026-09-30)_ — decompose a too-big leaf and record its design boundaries before writing code

- `G1-SLICE.3` (the whole garment ontology — identity, the persistent-identity contract, and nine
  geometry-bearing object types with their invariants) was one leaf but is three signoff-quality slices:
  `.3a` the identity types, `.3b` the contract that resolves references under split/merge/reverse/delete,
  `.3c` the object types. They are strictly ordered — the contract consumes the types, the objects consume
  both — so a frontier that tried to take them as one would have produced one unreviewable commit.
- The three cross-cutting design questions were settled and recorded as layer-C decisions BEFORE any code,
  because each is the kind of choice a later slice would otherwise re-litigate or silently contradict: the
  `EntityId` is a dependency-free hand-rolled ULID with an **injected** generator (so recipe/CLI replay stays
  byte-deterministic and the wall-clock never enters canonical content); the edge parameter is a **bounded
  exact rational in `sc-core`**, not the ppm `Ratio` and not the formula evaluator's bigint — with a named
  promotion trigger if `sc-geometry` ever needs it in `sc-units`; and G1 enforces only the **structural**
  invariants, handing CCW winding, simplicity and closure to `G2-2D.1` as a visible `DeferredToG2` state
  rather than claiming a 2D proof it cannot make. **Decide and record the boundaries a slice inherits, then
  implement inside them** — the records are why the next session does not reopen them.

# Sealed archive — earlier lessons

| Segment | Coverage | Sealed identity |
| --- | --- | --- |
| [`devnotes-part1.md`](docs/history/stitchcad-devnotes-part1.md) | the `2026-09-29` and `2026-09-04` lessons, plus the bootstrap entry | 66 lines, 5589 bytes, `sha256:d3b94e9a…` |
| [`devnotes-part2.md`](docs/history/stitchcad-devnotes-part2.md) | the two oldest `2026-09-30` lessons (enumeration, and the vocabulary census) | 62 lines, 5915 bytes, `sha256:edcd0808…` |
| [`devnotes-part3.md`](docs/history/stitchcad-devnotes-part3.md) | two `2026-09-30` lessons (two tables, one garment; a fixture internally right) | 50 lines, 4723 bytes, `sha256:fcca661d…` |
| [`devnotes-part4.md`](docs/history/stitchcad-devnotes-part4.md) | two `2026-09-30` lessons (a blocked leaf splits; permission is no criterion) | 42 lines, 3706 bytes, `sha256:c2ac5791…` |
| [`devnotes-part5.md`](docs/history/stitchcad-devnotes-part5.md) | two `2026-09-30` lessons (a rule whose only path is "don't"; a digest is about bytes) | 43 lines, 3977 bytes, `sha256:859ce981…` |
| [`devnotes-part6.md`](docs/history/stitchcad-devnotes-part6.md) | two `2026-09-30` lessons (a spec's tables are its test suite; settle it with the artifact) | 55 lines, 5359 bytes, `sha256:129d50d8…` |
| [`devnotes-part7.md`](docs/history/stitchcad-devnotes-part7.md) | three `2026-09-30` lessons (an arm that removes the rule; a synthetic input is a fixture; a RED arm asserts the refusal) | 57 lines, 5120 bytes, `sha256:13fd6c73…` |
| [`devnotes-part8.md`](docs/history/stitchcad-devnotes-part8.md) | two `2026-09-30` lessons (source layout; i18n population) | 35 lines, 3196 bytes, `sha256:04ab285c…` |
| [`devnotes-part9.md`](docs/history/stitchcad-devnotes-part9.md) | the `2026-09-30` certifying-artifact lesson | 15 lines, 1343 bytes, `sha256:bc7fae65…` |
| [`devnotes-part10.md`](docs/history/stitchcad-devnotes-part10.md) | the `2026-09-30` shipped-work reconciliation lesson | 15 lines, 1380 bytes, `sha256:701d33f2…` |
| [`devnotes-part11.md`](docs/history/stitchcad-devnotes-part11.md) | the `2026-09-30` property-test framework lesson | 15 lines, 1384 bytes, `sha256:ae04eadf…` |

The live window below holds the most recent lessons. When it passes its health target (200 lines /
16 384 bytes) again, the oldest entries are sealed the same way, and the `DESCRIPTOR` rule of
`run_changelog_ledger_probes.sh` proves the digest afterwards.

