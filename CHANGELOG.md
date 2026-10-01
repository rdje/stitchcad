# CHANGELOG.md

Newest first: one section per completed slice, in commit order. Older slices live in sealed, immutable
segments under `docs/history/`, each named below with its identity and retrieval path.

# Sealed archive — earlier slices

| Segment | Coverage | Sealed identity |
| --- | --- | --- |
| [`part1.md`](docs/history/stitchcad-changelog-part1.md) | slices 1–15, `STITCHCAD-PLANNING-0001` … `STITCHCAD-SPINE-0004b` | 365 lines, 30452 bytes, `sha256:f4aec75a…` |
| [`part2.md`](docs/history/stitchcad-changelog-part2.md) | slices 16–20, `STITCHCAD-SPINE-0014` … `STITCHCAD-G0-0002` | 152 lines, 12811 bytes, `sha256:5783ac36…` |
| [`part3.md`](docs/history/stitchcad-changelog-part3.md) | slices 21–24, `STITCHCAD-G0-0013` … `STITCHCAD-G0-0018` | 147 lines, 12289 bytes, `sha256:14ad5278…` |
| [`part4.md`](docs/history/stitchcad-changelog-part4.md) | slices 25–29, `STITCHCAD-G0-0004` … `STITCHCAD-SPINE-0017` | 177 lines, 15440 bytes, `sha256:a8cc1de6…` |
| [`part5.md`](docs/history/stitchcad-changelog-part5.md) | slices 30–31, `STITCHCAD-G0-0005` … `STITCHCAD-G0-0013c` | 82 lines, 7505 bytes, `sha256:18548ff7…` |
| [`part6.md`](docs/history/stitchcad-changelog-part6.md) | slices 32–33, `STITCHCAD-G0-0007` … `STITCHCAD-G0-0006` | 93 lines, 8284 bytes, `sha256:3148dd0f…` |
| [`part7.md`](docs/history/stitchcad-changelog-part7.md) | slices 34–35, `STITCHCAD-G0-0013d` … `STITCHCAD-G0-0008` | 105 lines, 9793 bytes, `sha256:ff62d418…` |
| [`part8.md`](docs/history/stitchcad-changelog-part8.md) | slices 41–42, `STITCHCAD-G0-0014` … `STITCHCAD-G0-0004b` | 106 lines, 9766 bytes, `sha256:2c7ee35a…` |
| [`part9.md`](docs/history/stitchcad-changelog-part9.md) | the two oldest live entries, `STITCHCAD-G0-0004c` and `STITCHCAD-SPINE-0004d` — no slice range, because the earlier ranges have no producer (D51) | 90 lines, 8386 bytes, `sha256:8e4081d4…` |
| [`part10.md`](docs/history/stitchcad-changelog-part10.md) | two spine slices on the table convention, `STITCHCAD-SPINE-0020` and `STITCHCAD-SPINE-0015` | 73 lines, 6652 bytes, `sha256:062ccfa3…` |
| [`part11.md`](docs/history/stitchcad-changelog-part11.md) | the delegation-and-uncertainty slice, `STITCHCAD-G0-0014c` | 46 lines, 4507 bytes, `sha256:de34382e…` |
| [`part12.md`](docs/history/stitchcad-changelog-part12.md) | the dialects and formula-language slices, `STITCHCAD-G0-0010` and `STITCHCAD-G0-0009` | 84 lines, 7767 bytes, `sha256:9c61ba7c…` |
| [`part13.md`](docs/history/stitchcad-changelog-part13.md) | the release-contract and canvas-spike-rule slices, `STITCHCAD-G0-0012` and `STITCHCAD-G0-0011` | 75 lines, 7188 bytes, `sha256:1e52b5c9…` |
| [`part14.md`](docs/history/stitchcad-changelog-part14.md) | the i18n slice, `STITCHCAD-G0-0016` | 38 lines, 3631 bytes, `sha256:2c895780…` |
| [`part15.md`](docs/history/stitchcad-changelog-part15.md) | the recurring cleanup slice, `STITCHCAD-SPINE-0021` | 23 lines, 2123 bytes, `sha256:f93154e3…` |
| [`part16.md`](docs/history/stitchcad-changelog-part16.md) | the closed-defect sealing slice, `STITCHCAD-SPINE-0019a` | 27 lines, 2450 bytes, `sha256:211f9bec…` |
| [`part17.md`](docs/history/stitchcad-changelog-part17.md) | the command-layer contract, `STITCHCAD-G0-0017` | 38 lines, 3626 bytes, `sha256:819f240a…` |

**Correction (D30).** part1's own descriptor says its coverage runs "through `STITCHCAD-SPINE-0004c`".
It does not: part1's newest entry is `STITCHCAD-SPINE-0004b`, and `SPINE-0004c` is sealed in part2.
Sealed segments are immutable, so the correction is recorded here and in part2's descriptor rather than
by editing part1.

The bedrock scaffold's own changelog — the provenance of this repository's discipline spine — is sealed
in [`docs/history/bedrock-scaffold-changelog.md`](docs/history/bedrock-scaffold-changelog.md).

The live window below holds the most recent slices. When it passes its health target (400 lines /
32 768 bytes) again, the oldest entries are sealed the same way, and
`bash docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh` proves the order, the uniqueness and
the digests afterwards.

## STITCHCAD-G1-0012 - directed grainlines retain independent print references (leaf `G1-SLICE.3c.3b`)

Immutable Grainline content holds a directed arrow, explicit parallel/angle intent and independent
optional stripe/plaid ranges. All born references require complete owned intervals and unique endpoints.
Directed queries preserve raw evidence, reverse fragment order for reverse traversal and compose journal
direction; repairs stay visible in traversal order. Explicit 45°/180° and symbolic angle declarations
supply intent without geometry proof or default values. G2/G4 retain physical/value validation.

Nine grain contracts and privacy doctest pass. Disabling reverse fragment order makes the independent
regression red; restored strict Rust, wasm, warning-free book and feature/glossary censuses pass.
D61 closes with unchanged census predicates and ten green/red glossary probes. Existing sewing/notch
suites pass after sharing whole-interval ownership. Ledger probes and staged doctrines pass.
Ontology §10 becomes a bounded index; its 231-line executable body moves unchanged to the linked
implementation chapter, which gains grain examples. Oldest live changelog/dev-note entries seal.
Next `.3c.3c` implements allowance descriptors; G1 remains 4/18 top-level leaves, 2/4 object families.

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

## STITCHCAD-G1-0009 - semantic notch anchors with symbolic profile bindings (leaf `G1-SLICE.3c.3a`)

`Notch` is immutable semantic content: a born-live, uniquely resolved point on a surviving interval
of its named Piece, plus logical target-profile declarations for style, sample/production depth and
width, and encoding. Construction refuses absent/foreign anchors, including a merged edge's foreign
remainder. Edits expose split choices, exact recomputation and repair tasks without rewriting anchors.
Bindings always report `DeferredToG4`; no physical values, profile pin or defaults are supplied.

Seven contract tests and the privacy doctest pass. Disabling ownership refusal makes the partial-merge
regression fail. Strict `make check`, wasm, warning-free book, feature/release censuses, ledger probes
and doctrine gate pass. The book documents examples and the unimplemented physical-export boundary.
D58 closes: G4's contradictory default-plus-sidecar acceptance now matches release §8's no-default
matrix; G4 still owns enforcement. The symbolic-binding decision is recorded and promoted.
The oldest changelog and dev-note entries roll into sealed segments in the same commit.
The director answered D57 before commit: each physical cut copy has a stable identity so seams can
differ. Next `.3c.2b` implements that contract; D57 closes after verified delivery.

## STITCHCAD-G1-0008 - separate cut-once L/R members are explicit piece content (leaf `G1-SLICE.3c.1a`)

Fixed D56: the canonical skirt's separate left/right back members, each cut once, were not representable
by the even-total mirrored-pair mode. `Mirroring::PairMember { handedness, companion }` now carries the
member's own L/R label and distinct companion Piece identity; quantity counts this member's copies.
Self-companions are refused. Existing even-total pair requests retain their original meaning.
The book documents both forms; collection-level reciprocity and equal quantities are owned by `.6`,
and geometric mirroring remains G2's obligation.

Validation: 14 piece-contract tests (including the canonical fixture-shaped pair and self-companion
refusal), strict `make check`, `make wasm`, `make book`, reference-fixture derivation, feature census,
doctrine gate and ledger probes green. Applying the even-total rule to all pair modes makes the fixture
regression fail. D56 closes in defects-part3; the decision records the two pairing forms.

D57 is logged and owned by `.3c.2b`: the spec does not decide how a sewing side names physical cut copies.
The director was asked to choose stable copy identities or pattern-level references with later expansion.
No answer is inferred; independent marks/allowances (`.3c.3`) proceed while that decision is pending.

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

## STITCHCAD-G1-0005 - the persistent-identity contract: a reference is never rewritten, the journal folds (leaf `G1-SLICE.3b`)

G1's second new product code. `sc_core::ontology` now carries the persistent-identity contract the whole
design's reference integrity rests on, implemented against the design decision this slice recorded,
dependency-free and wasm-safe:

- `topology` — the `IdentityLedger`: an append-only journal of typed `TopologyEdit`s (declare, split, merge,
  reverse, delete, offset-fragment). A stored reference is **never rewritten by an edit** — the `(EdgeRef,
  Param)` a consumer holds stays byte-identical, and resolution is a pure fold of the journal, so a replay
  reproduces every fragment identity. Split resolves a reference into the fragment holding its parameter and
  offers BOTH sides at the split point for the consumer to state (`SplitSide`); merge recomputes by
  caller-declared arc length (exact `Rational`, never a rounding); reverse maps `t` to `1 − t` and tells
  directed consumers through `Direction`; delete and offset-fragmentation orphan references into visible
  `RepairTask`s naming the reference, the orphaning edit and the candidate resolutions. No silent
  reassignment: repair state is *derived* (`open_repairs`, `release_readiness`), never stored, so it cannot
  drift from the journal. A design with unresolved references stays inspectable — every query answers
  identically — but is `Blocked` from release, which is §1.1's rule.

Validation: `cargo test -p sc-core` → 62 unit + 8 contract-property + 9 identity-property, all green (the
properties prove split's trichotomy and both-sides split point, merge's arc-length recomputation against a
cross-multiplied oracle, reverse an involution, split↔merge round-trips to the identical reduced rational
with zero drift, a delete orphaning exactly the references resolving onto the victim and nothing else,
offset against a hundredths-grid oracle, the live-edge set equal to the journal replayed, and byte-identical
replay of one script); `make check` clean at clippy `-D warnings`; `make wasm` cross-builds `sc-core`;
`make gate` → `=== all doctrines green ===`. The design is recorded in
`decision_reference-resolution-journal-fold.md`.

Two containment obligations this append discharged atomically, both surfaced as defects:

- **CHANGELOG rolled over** — the prior append (`STITCHCAD-G1-0004`) crossed the live window's 32 768-byte
  health target (31 237 → 33 338) without sealing, which the protocol requires of the crossing commit. The
  rollover milestone is a convention the size checker only *warns* at (rc=0 past a health target; only the
  ceiling fails), so the miss passed every gate — defect **D54**. Sealed `STITCHCAD-G0-0012` +
  `STITCHCAD-G0-0011` into `stitchcad-changelog-part13.md` (75 lines / 7188 bytes, digest reproduced by
  `run_changelog_ledger_probes.sh`).
- **KNOWLEDGE_MAP hit its 8192-byte ceiling** — the pressure `STITCHCAD-G1-0004` flagged ("99% … for the
  next record/tree addition"): this slice's decision record tipped it to 8216. Tightened two subsystem
  entries (the containment doctrine's only local lever) back under → 8187. The structural cause — the
  generated decision-record and task-tree sections grow a line per slice while the sole trim lever is the
  bounded subsystem list — is defect **D53** for the containment owner. The same edit repaired a run-on
  bullet in `knowledge-map/subsystems.md` (two entries shared one line, invisible to the sync gate, which
  checks derivation not source form).

- lockstep: `G1-SLICE.md` (`.3b` done, a fresh evidence-backed acceptance subsection, frontier → `.3c`,
  verification + commit logs, changelog), `lib.rs` status + module table, `ontology/mod.rs` re-exports,
  `knowledge-map/subsystems.md` + regenerated `KNOWLEDGE_MAP.md`, `docs/TASK_TREE.md`, `MEMORY.md`,
  `LIVE_STATUS.md` (G1 → 4 of 18, census → 9 open), `PLANNING.md` (D53, D54), `DEV_NOTES.md` (the lesson +
  its own rollover), `CHANGELOG.md` (this entry + the part13 rollover).

## STITCHCAD-G1-0004 - the identity layer: determinism is a property of the generator, not the id (leaf `G1-SLICE.3a`)

G1's first new product code. `sc_core::ontology` now carries the identity layer the whole ontology rests on,
implemented against the two design decisions `G1-SLICE.3` recorded:

- `id` — `EntityId`, a hand-rolled dependency-free ULID (128 bits: a 48-bit timestamp and 80 bits of
  randomness, 26-character Crockford base32, lexicographically sortable), and the injected `IdGenerator` trait
  with a `DeterministicIdGenerator`. The design point: a real ULID embeds a wall-clock and randomness, yet
  recipe re-evaluation and CLI replay must be byte-identical and canonical content carries no wall-clock — so
  determinism lives in the *generator*, not the id. Domain code never reads a clock; the composition root
  injects a deterministic generator for replay and a clock-plus-entropy one for production (`G1-SLICE.6`).
- `rational` — `Rational`, a bounded exact rational (`i64` numerator/denominator, `i128` intermediates, reduced
  canonical form). The four operators never round, so a parameter survives unbounded splits and merges with no
  drift; a result past `i64` is a typed `UnitError::Overflow`, never a wrap. Not `sc-units`' ppm `Ratio`, not
  the formula evaluator's bigint.
- `reference` — `EdgeRef`/`PointRef` (the creating operation's `EntityId` plus a persistent `LocalTag`, never
  an array index) and `Param`, a `Rational` constrained to `[0, 1]` at construction.

Validation: `cargo test -p sc-core` → 31 unit + 9 dependency-free recorded-seed properties, all green;
`make check` clean at clippy `-D warnings`; `make wasm` cross-builds `sc-core`; `make gate` → `=== all doctrines
green ===`. Two house conventions re-confirmed: fallible arithmetic is `checked_*` (clippy's
`should_implement_trait`, the precedent `sc-units` set), and a non-`#[test]` helper carries its own targeted
`#[allow(clippy::expect_used)]` because `.clippy.toml`'s `allow-expect-in-tests` does not reach it. The
frontier advances to `.3b` (the persistent-identity contract).

## STITCHCAD-G1-0003 - the ontology leaf is three slices, and its design boundaries are recorded first (leaf `G1-SLICE.3`)

`G1-SLICE.3` named the whole garment ontology as one leaf — identity, the persistent-identity contract, and
nine geometry-bearing object types with their invariants. That is three signoff-quality slices, not one, and
they are strictly ordered (the contract consumes the identity types; the objects consume both). This slice
decomposes `.3` into `.3a` (identity types), `.3b` (the persistent-identity contract) and `.3c` (the object
types), and records the three cross-cutting design boundaries BEFORE any code, so each implementation slice
builds against a fixed design rather than re-deciding it.

The three decisions, each a layer-C record with `answers:` so a later slice asking the question finds it:
`decision_entity-identity-ulid-injected-generator.md` — an `EntityId` is a dependency-free hand-rolled ULID
produced through an injected `IdGenerator`, because `sc-core` builds for wasm and recipe/CLI determinism
forbids a free-function clock; `decision_edge-parameter-bounded-exact-rational.md` — the edge parameter `t` is
a bounded exact rational in `sc-core`'s ontology, not the ppm `Ratio` and not the formula evaluator's
arbitrary-precision rational, with a named promotion trigger to `sc-units` if `sc-geometry` (G2) needs it;
`decision_ontology-invariants-structural-g1-geometric-g2.md` — G1 enforces the structural invariants and hands
CCW winding, simplicity, closure and intake conservation to `G2-2D.1` as a visible `DeferredToG2` state,
never a 2D-correctness claim.

No Rust changes; `make gate` stays `=== all doctrines green ===` and the regenerated Knowledge Map carries the
three new records. The tree is 18 leaves; the frontier advances to `.3a`, G1's first new product code.

## STITCHCAD-G1-0002 - the first property tests set the framework every later crate inherits (leaf `G1-SLICE.2`)

`G1-SLICE.2` (`sc-units`) was the second leaf `G0-CONTRACT.18` (commit `eb83f01`) pre-empted: that commit
landed `sc-units` in full — 1097 lines of library, 564 lines of property tests — as "the first product code",
not the skeleton its own leaf scoped. Like `.1`, the leaf stayed `pending` while its deliverable shipped. This
slice reconciles it: an audit, no new code.

Every acceptance criterion was re-derived by command and pasted into the leaf's `### G1-SLICE.2` checklist:
`cargo test -p sc-units --test property` → `21 passed` (conversion round-trips,
`the_classes_disagree_so_they_are_load_bearing` for class separation, `counts_are_their_own_dimension` and
`non_finite_floats_are_rejected_at_the_boundary` for typed dimension/non-finite errors); `UnitError` is a typed
enum, never a silent coercion; `make wasm` cross-builds the crate; the five tolerance classes are distinct
`ToleranceClass` variants (T1–T5).

The slice also discharges the Open Question `eb83f01` left open — "property-test framework choice, decided in
`.2`" — by recording `decision_property-tests-dependency-free-recorded-seed.md`: dependency-free hand-rolled
properties with a recorded seed are the default on the `wasm-viewer` critical path, and a framework off that
path is a per-crate recorded decision. The record carries `answers:`, which promotes this slice's `DEV_NOTES`
lesson. `make gate` stays `=== all doctrines green ===`. The frontier advances to `.3`, the `sc-core` ontology.

## STITCHCAD-G1-0001 - the frontier pointed at a leaf whose work had already shipped (leaf `G1-SLICE.1`)

The G1 frontier named `G1-SLICE.1` (the workspace crate layout) as the next slice to take, but its every
deliverable had already shipped: `G0-CONTRACT.18` (commit `eb83f01`) retired the bedrock starter crate
"closing defect D10 ahead of `G1-SLICE.1`", created `sc-units` and `sc-core` with the workspace lints
inherited, and landed the G0 CI shape (fmt / clippy / unit+property / a real `wasm32-unknown-unknown` build).
The leaf was left `pending`, so the tree's status disagreed with the workspace — a resuming session pointed at
`.1` would have re-done finished work.

This slice is the reconciliation: it audits `eb83f01` against each of `.1`'s acceptance criteria and records
the closure rather than writing new code. Every criterion was re-derived by command — `cargo metadata
--no-deps` lists exactly `sc-core, sc-units` (the roadmap crates that exist so far; §4.3 grows the rest at
their gates); `git ls-tree HEAD crates/` shows `crates/app` gone and no crate prints the template message;
`make check` is green (21 property tests + doc-test), `make wasm` cross-builds both crates, and
`run_g0_exit_review.sh` reports `G0-17 MET` (CI) and `G0-18 MET` (the wasm build); `KNOWLEDGE_MAP.md` names
both subsystems. `make gate` stays `=== all doctrines green ===`.

The lesson is recorded in `DEV_NOTES.md` and promotion declined there: a leaf's status drifting when a sibling
leaf delivers its work early is an instance of the D34 hand-kept-state class `PLANNING.5` owns, so this slice
fixes the instance and leaves the class to its derivation. The frontier advances to `.2` (`sc-units`), whose
code likewise shipped under `eb83f01` and is reconciled next.

## STITCHCAD-G0-0015 - the gate review is a command, and it reports one clause this repository cannot close (leaf `G0-CONTRACT.15`)

Gate G0 had nineteen obligations, twenty leaves marked done, and no verdict for any of them: the gate's state
existed only as an impression. The leaf's acceptance forbids marking a clause met on prose alone - and a review
WRITTEN as prose is exactly that - so the review parses the roadmap and runs the checks.

- **the review** - `run_g0_exit_review.sh` reads §11's `**Exit:**` bullet, splits it into fragments (plus the
  `Fixture:` bullet), requires every fragment to be dispositioned by a row of `g0_exit_clauses.tsv` and every
  row's key to appear in a fragment, then RUNS each row's check: `G0 EXIT: 18 met / 1 not met / 19 clauses`,
  exit=0, in two seconds. Eleven of the checks are censuses; two are `cargo test -p sc-units` and `make wasm`.
- **the one open clause** - G0-12, evaluation-seat procurement, is `not met` and accepted open by the
  director's ruling of `2026-09-30`, with the cost named rather than hidden: no target system reads our
  artifacts back, so the interchange claims stay `cited-from-roadmap` and G6's receiver validation falls to
  the roadmap's partner-run fallback or does not happen. G0-13 (governance) is met as drafted with its
  qualification printed: the model and both review paths exist, the project owner is the director acting, the
  domain seat is vacant.
- **the closure is unapproved, and that is recorded** - governance §6.1 rule 2 withholds approval of a
  decision's evidence from its author, and the reviewing party authored sixteen of the nineteen deliverables.
  The mitigation §6.1 prescribes is in place: every verdict is a command's exit status, so independence is
  available to whoever reads next instead of being held by anyone now. The roadmap's status line is unchanged,
  which is the honest outcome - line 9 says DRAFT until the exit criteria are met, and one is not.
- **the ruling, recorded when it was made** - `decision_director-ruling-2026-09-30-no-seats-proceed-unapproved.md`
  carries the director's words, the boundary table of what proceeds on engineering evidence alone versus what
  needs a seat and when, and the amendment that the residual dependency is **a measurement, not a
  credential**: knowledge is substitutable by sourced reading (D27 was settled that way), judgement under
  disagreement is substitutable if a synthesized default stays `assumed` and cited, and physical truth is not
  substitutable at all - but it needs a machine, a printer and a ruler, not a hire. `G2-2D.15` was created to
  own that protocol, because a ruling that names a cost without an owner is a wish.
- **the probe suite** - `run_g0_exit_review_probes.sh` -> `10 pass / 0 fail`, including ROADMAP-GROWS (a clause
  added to a COPY of §11 is refused by name), CHECK-FAILS (a failing instrument reads as an unmet clause and
  `GATE FAILS`, not as a broken review), NO-BLOCKER and CONTROL. Two parser bugs were fixed on the way, both
  the session's recurring class: the bullet matcher looked for `**Exit:**` after emphasis had been stripped,
  and one roadmap clause spans two `;`-separated fragments, so a row needed a key list.
- **the tree's own ledger sealed again** - adding the review pushed `G0-CONTRACT.md` to 99 136 bytes against a
  98 304 ceiling, a breach rather than a warning, so four changelog entries went to
  `g0-contract-changelog-part2` (48 lines / 4651 bytes, digest reproduced) and the tree fell to 94 923.
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `22 suite(s) green`; the review re-runs
  every clause's own census green; containment `OK`
