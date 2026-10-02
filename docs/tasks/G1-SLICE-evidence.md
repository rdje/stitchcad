# G1-SLICE — acceptance evidence for completed leaves

The evidence sibling of [`G1-SLICE.md`](G1-SLICE.md), split before the next checklist takes the
parent past 1000 lines (`.doctrine/live_document_size/surfaces.tsv`, `tasks_collection`).
Completed checklists are copied in landing order; D60 adds explicit revalidation evidence to
the historical doc-only `.1`/`.2` ROOT CAUSE bullets. Their original evidence is retained; current-leaf evidence stays in the
parent so its fresh boxes are the first ones the staged acceptance gate reads.

The third-window retention prerequisite also has its scoped active nodes here; fresh acceptance
boxes remain first in the primary tree, with detailed capture/remote receipts in this sibling.

[Completed garment-construction checklists](G1-SLICE-constructions.md) have their own bounded sibling.

## Completed acceptance checklists

### `G1-SLICE.1` (reconciled) — the workspace shape shipped under `G0-CONTRACT.18`, so this leaf closes on re-derived evidence, not new code

`G0-CONTRACT.18` (commit `eb83f01`) landed the G0 CI shape and, in its own words, retired the bedrock
starter crate "closing defect D10 ahead of `G1-SLICE.1`". It therefore delivered every artifact this leaf
names, but the leaf was left `pending` — a tree↔code drift: the leaf's status disagreed with the shipped
workspace. This slice audits that delivery against each acceptance criterion and records the closure. No
Rust changes; the gate that judges it is the doctrine enforcer plus the re-derived commands below.

- [x] **REPRODUCE / ISSUE** — the leaf said `pending` while its deliverables were committed:
  `git show HEAD:docs/tasks/G1-SLICE.md | awk '/ID: .G1-SLICE\.1./{f=1} f&&/Status:/{print;exit}'` →
  ``  Status: `pending` ``, `rc=0`, against `git log --oneline -- crates/sc-units crates/sc-core` →
  `eb83f01 STITCHCAD-G0-0018 (leaf G0-CONTRACT.18): the first product code - sc-units`, `rc=0`. A frontier
  that points at a leaf whose work already shipped misdirects the next session into redoing it.
- [x] **ROOT CAUSE (WHY + WHERE)** — `G0-CONTRACT.18`'s acceptance let it answer the starter-crate question
  itself ("retired here or explicitly handed to `G1-SLICE.1`"), and its commit message records "The bedrock
  starter crate is retired (git rm crates/app), closing defect D10 ahead of G1-SLICE.1". The G0 leaf closed;
  the G1 leaf it pre-empted was never reconciled. The remedy is the audit, not new code: the deliverables
  exist and are green.
  Revalidated during `.3c.2b.1` relocation (D60): `git show --format=fuller --no-patch eb83f01`
  → `STITCHCAD-G0-0018 (leaf G0-CONTRACT.18): the first product code - sc-units`, explicitly
  "closing defect D10 ahead of G1-SLICE.1", `rc=0`, confirming the pre-emption at its source.
- [x] **ADDRESSED (verified)** — each acceptance criterion re-derived by command. (1) `cargo metadata
  --no-deps --format-version 1` lists exactly the roadmap crates that exist so far, `sc-core, sc-units`
  (§4.3 — the rest appear when their stage starts), `rc=0`. (2) `git ls-tree HEAD crates/` shows only those
  two trees, so `crates/app` is gone, and `grep -rn 'Hello\|template\|starter' crates/` returns no match —
  no crate prints the template message. (3) The G0 CI shape is green locally and in the gate: `make check`
  → `test result: ok. 21 passed` (property) plus `1 passed` (doc-test), `make wasm` → `wasm-viewer
  smoketest: sc-units + sc-core build for wasm32-unknown-unknown`, and `run_g0_exit_review.sh` reports
  `G0-17 MET` (CI) and `G0-18 MET` (the wasm build). (4) `KNOWLEDGE_MAP.md` names both subsystems (lines 12
  and 17, each with entry point, conformance and owner).
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `rc=0`; `make check` green; `make
  wasm` green. This slice stages no Rust, so the acceptance gate (`check_task_acceptance.sh`, which fires
  only on staged code) does not govern it; these boxes are the tree's own per-leaf record.
- [x] **FIX** — marked `.1` `done`, recorded its verification and commit, advanced the frontier, and added
  this subsection so the closure carries re-derivable evidence.
- [x] **LOCKSTEP** — `docs/TASK_TREE.md` (frontier cell), `MEMORY.md` (next action), `LIVE_STATUS.md` (G1 row
  → In Progress), `CHANGELOG.md`, `DEV_NOTES.md` and the regenerated Knowledge Map, all in this commit.
  promotion: declined (instance of the D34 hand-kept-state class `PLANNING.5` owns; this slice fixes the instance, a new record would duplicate that ownership).

### `G1-SLICE.2` (reconciled) — `sc-units` shipped under `G0-CONTRACT.18`, and the property-test choice it made is now a recorded decision

`G0-CONTRACT.18` (commit `eb83f01`) landed `sc-units` in full — 1097 lines of library and 564 lines of
property tests — as "the first product code", well past the skeleton its own leaf scoped. That is this leaf's
deliverable, so `.2` closes on the same audit `.1` did: every acceptance criterion re-derived by command, no
new code. The one thing `eb83f01` left unrecorded was the property-test-framework choice this tree's Open
Questions deferred to `.2`; closing the leaf records it.

- [x] **REPRODUCE / ISSUE** — the leaf said `pending` while `sc-units` was committed and green:
  `git show HEAD:docs/tasks/G1-SLICE.md | awk '/ID: .G1-SLICE\.2./{f=1} f&&/Status:/{print;exit}'` →
  ``  Status: `pending` ``, `rc=0`, against `wc -l crates/sc-units/src/*.rs crates/sc-units/tests/*.rs` →
  `1723` lines delivered by `eb83f01`. The Open Question "Property-test framework choice … decided in `.2`"
  was never discharged, so a later crate would re-litigate `proptest` vs `quickcheck`.
- [x] **ROOT CAUSE (WHY + WHERE)** — the same pre-emption as `.1`: `G0-CONTRACT.18` answered the G0 CI clause
  by building `sc-units` out fully rather than as a skeleton, and closed only itself. `.2`'s work was done but
  its leaf, and the decision the Open Question routed to it, were not recorded.
  Revalidated during `.3c.2b.1` relocation (D60): `git show --format=fuller --no-patch eb83f01`
  → the committed delivery lists 1097 library lines, 564 property-test lines and 30 passing tests,
  `rc=0`; `git log --oneline -- crates/sc-units crates/sc-core` still traces `sc-units` to that
  commit. This confirms the delivery, not a new implementation or a rerun of its old test counts.
- [x] **ADDRESSED (verified)** — each acceptance criterion re-derived. (1) Property tests for conversion
  round-trips and tolerance-class separation: `cargo test -p sc-units --test property` → `test result: ok. 21
  passed; 0 failed`, including `conversion_round_trips_for_integral_units` (line 62),
  `the_classes_disagree_so_they_are_load_bearing` (448) and `counts_are_their_own_dimension` (500). (2) A
  dimension error is a typed error, never a silent coercion: `UnitError` (`crates/sc-units/src/error.rs`)
  carries `DomainExceeded`/`DivisionByZero`/`NonFinite`/`Overflow`/`EmptyDerivation`, and
  `non_finite_floats_are_rejected_at_the_boundary` (291) proves the boundary rejects NaN/inf. (3) Compiles for
  `wasm32-unknown-unknown`: `make wasm` → `wasm-viewer smoketest: sc-units + sc-core build for
  wasm32-unknown-unknown`. The five tolerance classes are distinct `ToleranceClass` variants (T1–T5,
  `tolerance.rs`). The framework choice is recorded as
  `docs/decisions/decision_property-tests-dependency-free-recorded-seed.md` (with `answers:`, listed in the
  INDEX), resolving the Open Question.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `rc=0`; `make check` green; `make wasm`
  green. The new decision record is a `docs/decisions` file; the KNOWLEDGE-MAP doctrine confirms the
  regenerated map is in sync. No Rust staged, so the acceptance gate does not fire.
- [x] **FIX** — marked `.2` `done`, recorded its verification and commit, resolved the Open Question into the
  Decisions section and a layer-C record, advanced the frontier to `.3`, and added this subsection.
- [x] **LOCKSTEP** — `docs/decisions/decision_property-tests-dependency-free-recorded-seed.md` (new) and its
  INDEX row; `docs/TASK_TREE.md` (frontier cell), `MEMORY.md` (next action), `LIVE_STATUS.md` (G1 row → 2 of
  16), `CHANGELOG.md`, `DEV_NOTES.md` (the lesson, promoted by the new record's `answers:`) and the
  regenerated Knowledge Map, all in this commit.

### `G1-SLICE.3a` — the identity layer: a ULID with an injected generator, stable references, an exact parameter

G1's first new product code. Ontology §1 requires every entity to carry a ULID stable across saves and
exports, every geometric sub-entity to be addressed by identity rather than index, and a parameterized
position to be an exact rational in `[0, 1]`. This slice implements that identity layer in `sc-core::ontology`
against the two design decisions `.3` recorded — dependency-free, and wasm-safe.

- [x] **REPRODUCE / ISSUE** — the requirement, located: `grep -n 'a ULID, assigned once at creation'
  docs/book/src/spec/ontology.md` → line 24, `rc=0`, and the glossary's `parameterized reference` → "a
  position along an edge as a rational in [0, 1]". Before this slice `sc-core` was a skeleton
  (`git show HEAD:crates/sc-core/src/lib.rs | grep -c 'Status: skeleton'` → `1`, `rc=0`); the identity layer
  the whole ontology rests on did not exist in code.
- [x] **ROOT CAUSE (WHY + WHERE)** — the design is the two recorded decisions, not an ad-hoc choice.
  `EntityId` is a hand-rolled dependency-free ULID because `sc-core` builds for `wasm32-unknown-unknown`
  (`make wasm` → green) and a ULID crate would pull a clock and entropy into the wasm graph, so generation is
  behind an injected `IdGenerator` (`decision_entity-identity-ulid-injected-generator.md`); the parameter is a
  bounded exact `Rational` in `sc-core`, not `sc-units`' ppm `Ratio`, which would round and drift on every
  split/merge (`decision_edge-parameter-bounded-exact-rational.md`). Both records carry `answers:` and are in
  the INDEX: `grep -c 'answers:' docs/decisions/decision_entity-identity-ulid-injected-generator.md` → `1`,
  `rc=0`.
- [x] **ADDRESSED (verified)** — every acceptance criterion is a passing test. `cargo test -p sc-core` →
  `test result: ok. 31 passed` (unit) and `9 passed` (property), `rc=0`: an `EntityId` round-trips its
  26-character Crockford form and its text order equals its byte order
  (`an_entity_id_round_trips_and_sorts_lexicographically`); a deterministic generator reproduces a sequence
  byte-for-byte (`a_deterministic_generator_reproduces_its_sequence`); the parameter is exact under the four
  operators (`arithmetic_is_exact_and_never_rounds`, `subtraction_and_division_are_exact_inverses`), reduced to
  a canonical form (`reduction_is_canonical`), totally ordered (`ordering_is_exact_and_total`), bounded by
  `in_unit_interval` (`the_unit_interval_predicate_is_exact_at_its_ends`), and reports overflow and
  division-by-zero as typed diagnostics (`a_result_past_i64_is_an_overflow_not_a_wrap`,
  `division_by_zero_is_a_diagnostic`). `make wasm` → `sc-units + sc-core build for wasm32-unknown-unknown`.
- [x] **NO REGRESSION** — `make check` → fmt clean, clippy `-D warnings` clean, `cargo test --all` green
  (sc-units' 21 properties and doc-test unaffected); `make gate` → `=== all doctrines green ===`, `rc=0`. The
  change is additive: `sc-core` gained an `ontology` module and a property-test target, no existing behaviour
  changed, and `sc-units` was not touched.
- [x] **FIX** — implemented `sc_core::ontology`: `id` (`EntityId`, `IdError`, `IdGenerator`,
  `DeterministicIdGenerator`), `rational` (`Rational`), `reference` (`LocalTag`, `EdgeRef`, `PointRef`, `Param`,
  `ParamError`), re-exported from `ontology/mod.rs`; updated `lib.rs`'s status and module table; 31 unit tests
  inline and 9 dependency-free recorded-seed properties in `tests/identity_property.rs`.
- [x] **LOCKSTEP** — `knowledge-map/subsystems.md` (sc-core is no longer a skeleton) and the regenerated
  `KNOWLEDGE_MAP.md`; `docs/TASK_TREE.md` (frontier cell), `MEMORY.md` (next action → `.3b`), `LIVE_STATUS.md`
  (G1 → 3 of 18), `CHANGELOG.md`, `DEV_NOTES.md`, and this tree's frontier, logs and changelog.
  promotion: declined (the durable design choices are the three decision records `.3` landed; this slice's note is one leaf's implementation history, and the two conventions it records are already the house style `sc-units` set, not a new general rule).

### `G1-SLICE.3b` — the persistent-identity contract: a reference is never rewritten, the journal folds

G1's second new product code. Ontology §1.1 requires that when an edit changes the topology a reference
points into, the reference either resolves onto the new topology or becomes a visible repair task — never a
silent reassignment. This slice implements that contract in `sc_core::ontology::topology` against the design
decision recorded before the code, dependency-free and wasm-safe.

- [x] **REPRODUCE / ISSUE** — the requirement, located and unimplemented: `grep -n 'The persistent-identity
  contract' docs/book/src/spec/ontology.md` → line 39, `rc=0` (§1.1's five-row edit table and the "no silent
  reassignment" rule), against `git show HEAD:crates/sc-core/src/ontology/mod.rs | grep -c topology` → `0`,
  `rc=1`. `.3a` landed the identity *types* (`EdgeRef`/`Param`); nothing resolved a reference through an edit
  and nothing turned an orphan into a repair task, so the contract the whole design's reference integrity
  rests on was spec-only.
- [x] **ROOT CAUSE (WHY + WHERE)** — the design is a recorded decision, not an ad-hoc choice:
  `decision_reference-resolution-journal-fold.md` (`grep -c '^answers:' …` → `1`, `rc=0`) — a stored reference
  is never rewritten; resolution is a pure fold of an append-only edit journal, so no consumer can be missed,
  and repair state is *derived* (`open_repairs`/`release_readiness`), never stored, so it cannot drift. It
  inherits `.3`'s two prior boundaries: the injected-generator ULID
  (`decision_entity-identity-ulid-injected-generator.md`) and the bounded exact rational
  (`decision_edge-parameter-bounded-exact-rational.md`), which is why split/merge recompute with zero drift.
- [x] **ADDRESSED (verified)** — every acceptance criterion is a passing test. `cargo test -p sc-core` →
  `62 passed` (unit) + `8 passed` (contract property) + `9 passed` (identity property), `rc=0`. **Split**:
  the trichotomy resolves into the containing fragment, and a reference at the split point returns a
  `SplitPoint` with both sides for the consumer to state (`SplitSide`) —
  `a_reference_at_the_split_point_resolves_to_both_and_the_consumer_states_which_it_wants`, property
  `split_resolves_by_trichotomy_and_the_split_point_offers_both_sides`. **Merge by arc length**:
  `merge_recomputes_the_parameter_by_arc_length` (1 cm + 3 cm maps `t=1/2` to `1/8` and `5/8` exactly), and
  the property checks the defining proportion cross-multiplied in `i128`, an oracle sharing no code with the
  fold. **Reverse**: `1 − t` + `Direction::Reversed`, an involution. **Delete → RepairTask**:
  `delete_orphans_a_reference_into_a_visible_repair_task` — the task names the reference, the orphaning edit
  (`Deleted { operation, edge }`) and the candidates (empty for a deletion); the property proves exactly the
  references resolving onto the victim orphan and nothing else moves. **Savable/inspectable/not-releasable**:
  `a_design_with_unresolved_references_is_inspectable_but_not_releasable` — every query still answers and
  `release_readiness()` → `Blocked { unresolved: 1 }`. `make wasm` → `sc-units + sc-core build for
  wasm32-unknown-unknown`.
- [x] **NO REGRESSION** — `make check` → fmt clean, clippy `-D warnings` clean, `cargo test --all` green
  (sc-units' 21 properties + doc-test and `.3a`'s 9 identity properties unaffected); `make gate` → `=== all
  doctrines green ===`, `rc=0`; `make probes` → `22 suite(s) green`; `make wasm` green. The change is
  additive: `sc-core` gained an `ontology::topology` module and a second property-test target, no existing
  behaviour changed, and `sc-units` and `.3a`'s `id`/`rational`/`reference` were untouched. The two
  rolling-ledger rollovers this append owed (D54) are verified by `run_changelog_ledger_probes.sh` →
  `9 pass / 0 fail`.
- [x] **FIX** — implemented `sc_core::ontology::topology`: `IdentityLedger` (an append-only journal +
  live-edge index + registrations), `TopologyEdit` (declare/split/merge/reverse/delete/offset),
  `Resolution`/`ResolvedRef`/`Direction`/`SplitSide`, `RepairTask`/`OrphaningEdit`/`OpenRepair`,
  `ReleaseReadiness`, `OffsetInterval`/`OffsetFragment`, `LedgerError`, `MAX_EDGES_PER_OPERATION`;
  re-exported from `ontology/mod.rs`; updated `lib.rs`'s status + module table; 31 inline unit tests and 8
  dependency-free recorded-seed properties in `tests/identity_contract_property.rs`. Also repaired two
  hand-kept-doc defects the sync gates cannot see (they check derivation, not source form): a run-on bullet
  in `knowledge-map/subsystems.md` (two entries shared one line) and an accidental duplicated sentence in
  `docs/TASK_TREE.md`'s execution-order prose (a D34-class drift, logged in the leaf rather than a new id
  since `PLANNING.5` owns deriving that index).
- [x] **LOCKSTEP** — `decision_reference-resolution-journal-fold.md` (new) + its INDEX row;
  `knowledge-map/subsystems.md` + regenerated `KNOWLEDGE_MAP.md` (trimmed under its 8192 ceiling, D53);
  `docs/TASK_TREE.md` (frontier cell), `MEMORY.md` (next action → `.3c`), `LIVE_STATUS.md` (G1 → 4 of 18,
  census → 9 open), `CHANGELOG.md` (entry + the part13 rollover), `DEV_NOTES.md` (lesson + the part7
  rollover), `PLANNING.md` (D53, D54), and this tree's frontier, three logs and changelog.
  promotion: promoted by `decision_reference-resolution-journal-fold.md` (carries `answers:`) — the durable
  design boundary is recorded there so `.3c`/`.6`/`.7` inherit it rather than re-litigate.

### `G1-SLICE.3c.1` — immutable structural pieces with visibly deferred geometry

- [x] **REPRODUCE / ISSUE** — ontology §4.1 requires piece content and invariants, but
  `git grep -n 'pub struct Piece' a6d465f -- crates` → no matches, `rc=1`. The existing identity types
  and journal do not supply a piece constructor. `make book` also exposed a formatting defect in
  release §6: literal scope placeholders were parsed as unclosed `<receiver>` / `<version>` HTML tags.
- [x] **ROOT CAUSE (WHY + WHERE)** — the missing constructor is the next ontology layer, not a
  geometry-kernel defect: `cargo test -p sc-core --test piece_contract` → `12 passed`, `rc=0`, with
  invalid-loop/cut-plan/label/live-reference cases pinpointing the constructor boundary in `piece.rs`.
  The structural/geometric decision remains authoritative. An intentional mutation replacing the
  empty-loop predicate with `false` makes `empty_boundary_and_each_empty_hole_name_the_exact_loop`
  fail at the expected empty-boundary assertion (`rc=101`); the source was restored before signoff.
- [x] **FIX** — `ontology::piece` adds immutable `Piece`, editable `PieceDefinition`, directed cyclic
  edge lists, explicit material assignment, complete label view, typed `PieceError` and the only
  geometric state `DeferredToG2`. Labels derive cut information from the plan. Public access is
  shared-only; a compile-fail doctest exercises private-content protection. Literal release scope
  placeholders are code-formatted, so they render visibly. `.3c`'s four children own the remaining scope.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test piece_contract` → `12 passed`, `rc=0`:
  all required content retained; invalid boundary/hole/live-edge/cut-plan/fold/material/label cases
  name their exact typed refusal; cloned-input mutation leaves the validated object unchanged;
  reverse/delete endpoint queries expose direction and repair without rewriting authored content.
  `make wasm` → cross-build green, `rc=0`. `make book` → no warnings, `rc=0`; rendered release HTML
  contains `<code>&lt;receiver&gt;</code> <code>&lt;version&gt;</code>` instead of hidden raw tags.
  D55's passing counterexample explicitly proves the endpoint inventory is not range completeness;
  its repair is scheduled immediately at `.3c.2`, before sewing spans consume the journal.
- [x] **NO REGRESSION** — `make check` → fmt and clippy `-D warnings` clean, existing 62 unit +
  8 contract-property + 9 identity-property tests, 12 piece-contract tests and the privacy doctest
  green; `sc-units` unchanged and its unit/property/doc tests green, `rc=0`. `make gate` →
  `=== all doctrines green ===`, `rc=0`; release and feature-matrix censuses → `0 failure(s)`, `rc=0`;
  `run_changelog_ledger_probes.sh` → `9 pass / 0 fail`, `rc=0`. The new ontology §10 initially made
  feature coverage red (one uncited clause); citing it in the label row restored the census without
  weakening its coverage rule.
- [x] **LOCKSTEP** — ontology §10 and its implementation-status note, feature-matrix label row,
  release-scope rendering, crate docs/description, subsystem map input and generated Knowledge Map;
  `.3c` decomposition and evidence, defect D55 with its scheduled owner, task index, resume pointer,
  LIVE_STATUS, CHANGELOG and DEV_NOTES in this commit. Reviewed read-only source policy bodies match
  the local neutral bodies; cleanup is not due (latest run `2026-09-30` 19:00 UTC, startup
  `2026-10-01` 12:47 UTC).
  promotion: declined (the structural/geometric boundary is already a decision; D55 is a tracked
  open contract to be resolved, rather than a settled general rule).

### `G1-SLICE.3c.2a` — whole-range resolution, fixing D55 before sewing spans

- [x] **REPRODUCE / ISSUE** — D55's tracked point-only counterexample at `0d7a4a5` leaves both held
  endpoints resolved after deleting the middle of three fragments. Its ledger verdict is still
  `Releasable`, correctly scoped to point registrations. `cargo test -p sc-core --test piece_contract
  endpoint_inventory_cannot_certify_the_interior_of_an_edge_range` → `1 passed`, `rc=0`: sampling
  cannot prove an interval's integrity, and the new piece assertion now demands a visible range repair.
- [x] **ROOT CAUSE (WHY + WHERE)** — the point fold answers one parameter's fate, while a sewing span
  holds an interval; the error is treating those as the same quantity. `cargo test -p sc-core --test
  range_contract arbitrarily_narrow_trimmed_gaps_cannot_escape_the_interval_fold` → `1 passed`, `rc=0`:
  every hundredths-grid point resolves while the exact uncovered interval is returned as a repair.
  The solution is interval partition through the journal, recorded before code in
  `decision_range-resolution-preserves-entire-interval.md`, not adding more sampling points.
- [x] **FIX** — `ontology::range`: `EdgeRange`, `RangeResolution`, `ResolvedRange`, ordered
  `RangePortion`s, typed `RangeRepairTask`/`RangeIssue`, and the exact whole-interval fold on the
  existing public journal. Split/offset use interval intersection (offset gaps retained); merge uses
  declared arc lengths; reversal preserves authored traversal; deletion and arithmetic refusal retain
  visible tasks. `Piece::range_resolutions` exposes the full-edge evidence. Point API unchanged.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test range_contract` → `13 passed`, `rc=0`:
  D55 returns a task naming the original held range, deleted middle fragment and delete operation,
  despite resolved endpoints; partial ranges, merge/reverse, offset order/gaps, missing sources and
  overflow are checked by exact payload. Generated scripts compare the interval fold with the
  independent existing point fold away from ambiguous boundaries. Disabling the range-delete arm
  makes `d55_deleted_interior_blocks_range_coverage_despite_two_resolved_endpoints` fail, `rc=101`;
  restored source passes. D55 closes in immutable `stitchcad-defects-part2.md`.
- [x] **NO REGRESSION** — `make check` → fmt, strict clippy and all unit/property/contract/doc suites
  green, `rc=0` (62 core unit, 8 identity-contract, 9 identity-property, 12 piece-contract and 13
  range-contract tests; sc-units unchanged). `make wasm` → wasm cross-build green; `make book` →
  warning-free build; feature census → `0 failure(s)`; ledger probes → `9 pass / 0 fail`; `make gate`
  → `=== all doctrines green ===`, all `rc=0`. The point registration verdict is not broadened:
  full coverage, endpoint ambiguity and G2 geometric validity remain separately visible.
- [x] **LOCKSTEP** — pre-code decision + INDEX, ontology §10, feature-matrix identity row, crate
  module docs, task decomposition/evidence/frontier, task index, resume pointer, LIVE_STATUS,
  CHANGELOG and DEV_NOTES; regenerated Knowledge Map. Rollover seals the oldest changelog entry
  into part14 and two dev-note lessons into part8, with exact digests watched by ledger probes.
  The closed-defect pointer names new part2; counts derive to 9 open / 45 sealed. The map's
  interchange orientation entry is shortened without losing its entry path or owner; D53 remains owned.

### `G1-SLICE.3c.1a` — the canonical fixture's separate cut-once L/R members (D56)

- [x] **REPRODUCE / ISSUE** — `reference-skirt.md` §6 declares separate `skirt_back_right` and
  `skirt_back_left` Piece identities, quantity one each and reciprocal pairing. At `0a64a17`,
  `piece.rs`'s mirroring enum has only `Single` and even-total `MirroredPairs`; it cannot carry a
  cut-once member's hand/companion. Applying an even-total restriction to every pair mode makes
  `the_reference_skirts_cut_once_pair_members_print_their_own_handedness` fail (`rc=101`).
- [x] **ROOT CAUSE (WHY + WHERE)** — the first implementation conflated two pairing forms: one
  definition requesting both hands, and two separately identified members. The glossary's mirrored
  pair and the fixture's explicit Piece rows are the independent authorities. `cargo test -p sc-core
  --test piece_contract the_reference_skirts_cut_once_pair_members_print_their_own_handedness` →
  `1 passed`, `rc=0`: preserving cut-one quantities requires explicit member metadata, not name parsing.
- [x] **FIX** — `Mirroring::PairMember { handedness, companion }` and `Handedness::Left/Right`;
  this mode counts only this member's copies. `PieceError::SelfCompanion` rejects a member naming
  itself. The existing even-total mode is unchanged. Complete printed labels derive both forms
  from their cut plan. `.6` owns collection-level reciprocity/opposite-hand/equal-quantity checks;
  G2 owns geometric mirroring. D57's copy-address question stays separate and unanswered.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test piece_contract` → `14 passed`,
  `rc=0`: the fixture-shaped separate cut-one L/R members retain reciprocal ids and exact label
  metadata; self-companion is the expected typed refusal, and existing even-total and odd-total
  cases still discriminate. Reinstating the overbroad even-total condition makes the new fixture
  regression red (`rc=101`); restored source passes. D56 is sealed closed in defects-part3.
- [x] **NO REGRESSION** — `make check` → fmt, clippy `-D warnings`, all existing suites and new
  pair-member tests green, `rc=0`; `make wasm` → cross-build green; `make book` → warning-free;
  fixture derivation → `20 derived rows / 4 closure checks / 5 pieces / 0 mismatch(es)`;
  feature census → `0 failure(s)`; ledger probes → `9 pass / 0 fail`; `make gate` →
  `=== all doctrines green ===`, all `rc=0`. No fixture cut quantity, pairing or geometry was edited.
- [x] **LOCKSTEP** — pairing decision + INDEX, ontology §10 and label examples, Rust type docs,
  task evidence/frontier, task index, resume pointer, LIVE_STATUS, CHANGELOG and promoted DEV_NOTES
  lesson; closed D56 pointer and sealed descriptor, open D57 with the exact census and pending
  director question. The generated map retains orientation paths/owners after its units entry is
  shortened. Next `.3c.3` is independent work while the sewing-copy decision is pending.

### `G1-SLICE.3c.3a` — semantic notches; physical profile bindings stay unresolved

- [x] **REPRODUCE / ISSUE** — ontology §4.5 requires semantic edge/parameter anchoring with
  profile-owned style, sample/production dimensions and encoding. At `486607b` no `Notch` exists.
  D58's dependent G4 acceptance says default-plus-sidecar despite its no-default Goal and release §8.
- [x] **ROOT CAUSE (WHY + WHERE)** — G1 needs semantic identity before G4's target-profile schema
  exists; copying physical values would invent facts and duplicate parameter state. Ownership must
  compare current ranges and current positions, not only live edge ids. `cargo test -p sc-core
  --test notch_contract ownership_uses_surviving_partial_ranges_and_resolved_parameters` → `1 passed`,
  `rc=0`: the independently expected owned `[0,3/10]` portion refuses the merged foreign remainder,
  including after reversal. `run_release_contract_census.sh` → `8 matrix rows / 0 failure(s)`, `rc=0`
  identifies the canonical matrix D58's Acceptance must follow.
- [x] **FIX** — immutable `Notch`, editable `NotchDefinition`, exact `EdgeAnchor`, symbolic
  `ProfileParameterRef` and complete sample/production/style/encoding bindings. Construction requires
  a live unique owned point; edits return the original ledger's choices/repairs without mutations.
  `ProfileBindingValidation::DeferredToG4` exposes the missing physical validation. G4 acceptance
  now names badge preview / sidecar draft / block production and no defaults; G4 still owns execution.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test notch_contract` → `7 passed`, `rc=0`:
  boundary/hole/construction anchors and endpoints accepted, absent/foreign anchors refused, exact
  reverse/merge recomputation, split-side choice, deletion operation/held-reference evidence and
  preserved symbolic fields tested. Disabling ownership refusal makes the partial-merge test fail
  (`rc=101`); restored source passes. The privacy doctest also passes. D58 closes in defects-part4.
- [x] **NO REGRESSION** — `make check` → fmt, strict clippy, all existing/new suites green, `rc=0`;
  `make wasm` → green; `make book` → warning-free; feature/release censuses → `0 failure(s)`;
  ledger probes → `9 pass / 0 fail`; `make gate` → `=== all doctrines green ===`, all `rc=0`.
  The style vocabularies implement no geometry and do not widen the staged release envelope.
- [x] **LOCKSTEP** — binding decision + INDEX, Rust module docs, ontology §10 + feature matrix,
  G4 dependent acceptance, task status/frontier/evidence/logs, index, MEMORY, LIVE_STATUS,
  CHANGELOG and promoted DEV_NOTES; D58 closure + descriptor and live pointer. Oldest ledger
  entries seal in changelog-part15/devnotes-part9; map orientation is tightened and regenerated.
  D57 was answered before commit: stable identities per physical cut copy. Next `.3c.2b` implements
  that contract and sewing spans; D57 closes only after verification.

### `G1-SLICE.3c.2b.1` — explicit physical-copy identities, quantities and orientations

- [x] **REPRODUCE / ISSUE** — D57's director ruling (`2026-10-01`) requires stable physical-copy
  identities so copies of a cut-two Piece can have different neighbours. At `6abfac3` only Piece
  quantity exists. The glossary census against that committed snapshot also reports `15 failure(s)`
  (`rc=1`, D59), showing existing API terms lacked vocabulary ownership before this slice.
- [x] **ROOT CAUSE (WHY + WHERE)** — pattern identity/quantity cannot select one physical copy;
  ordinal-derived ids would change under reordering. `cargo test -p sc-core --test cut_contract
  copies_retain_explicit_identity_when_reordered_and_geometry_stays_deferred` → `1 passed`, `rc=0`:
  the explicit-id oracle retains both copy definitions across reversed list order. The committed
  snapshot's `GLOSSARY_ROOT` census → `15 failure(s)`, `rc=1`, pins D59 to undeclared API tokens
  in ontology prose, not an index or glossary-term duplication.
- [x] **FIX** — immutable `CutPlan`/`CutCopy`, editable explicit ids/source Piece/orientation.
  Validate unique/disjoint identities, known Pieces, exact quantity and equal authored/reflected
  populations for MirroredPairs; authored-only modes refuse reflection. No id is minted or reassigned.
  Geometry stays DeferredToG2; companion and global design checks remain `.6`. Genuine new concepts
  enter the glossary; the chapter-local API table declares implementation names without exemptions.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test cut_contract` → `9 passed`, `rc=0`:
  identity/reorder/replay, duplicates/collisions, unknown/unmentioned Pieces, exact quantities,
  mirroring, explicit replacement and empty plan contracts pass. Disabling quantity refusal turns
  its regression red (`rc=101`), restored source passes. Glossary census → `310 terms / 9 parts /
  158 tokens / 0 failure(s)`, `rc=0`; D59 closes in defects-part5. D60 closes in defects-part6 after historical-root revalidation;
  staged `make gate` → all doctrines green, `rc=0`. D57 awaits graph integration.
- [x] **NO REGRESSION** — `make check` → strict clippy, all suites and three core privacy doctests
  green; `make wasm` → green; `make book` → warning-free; feature/glossary censuses → `0 failure(s)`;
  tree coverage → `10 lanes / 13 trees / 4 sibling(s) / 0 unowned / 0 orphan(s) / 0 dead link(s)`;
  ledger probes → `9 pass / 0 fail`; `make gate` → `=== all doctrines green ===`, all `rc=0`.
  The original moved payload was byte-identical before D60 added revalidation evidence to the
  historical doc-only `.1`/`.2` ROOT CAUSE bullets; all four checklists are validated separately.
- [x] **LOCKSTEP** — copy-identity decision + INDEX, source/module docs, ontology §4.2/§10,
  feature matrix, two glossary entries + derived A–Z index, G1 task evidence sibling/status/logs,
  task index, MEMORY, LIVE_STATUS, CHANGELOG, promoted DEV_NOTES, closed D59/D60 descriptors/live pointers;
  devnotes-part10 seals the oldest lesson. Next `.3c.2b.2` implements copy-addressed sewing spans.

### `G1-SLICE.3c.2b.2` — copy-addressed sewing spans; D35/D57 verified closed

- [x] **REPRODUCE / ISSUE** — D57 requires physical-copy addressing so cut-two copies can have
  different neighbours; D35 needs an explicit same-copy sewing rule before the type exists.
  At `9ba32e0`, copy plans exist but no graph does. The canonical folded band's short-end finish
  is the relevant same-copy case; G1 must not invent its unconstructed numeric G2 ranges.
- [x] **ROOT CAUSE (WHY + WHERE)** — Piece identity alone conflates physical material domains;
  different held edge names alone cannot certify self-seam disjointness. `cargo test -p sc-core
  --test sewing_contract self_seam_disjointness_is_checked_after_merge_instead_of_by_edge_name`
  → `1 passed`, `rc=0`: two held edges resolve to disjoint current intervals, while a new merged
  overlapping range is refused before/after reversal. The separate hidden-interior ownership
  regression expects refusal of an unowned middle third despite owned endpoints.
- [x] **FIX** — immutable SewingGraph/SeamSpan with copy ids, exact positive ranges, explicit
  correspondence, signed ease source/distribution and side-specific semantic stop ids. Validate
  the complete plan, scope identities, interval ownership/coverage, endpoint uniqueness, disjoint
  self-seams, stops and weighted/notch-anchored domains. Shared born-valid anchor validation supplies
  TurnPoint without changing the NotchError API alias. Queries preserve held content and missing ids.
  Geometry/realized ease and formula/profile value resolution remain explicit later obligations.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test sewing_contract` → `18 passed`,
  `rc=0`: copy-specific neighbours, partial/one-to-many spans, disjoint self-seams and overlap
  after merge/reversal, complete ownership, interior repairs and endpoint choices, stop/ease
  domains and missing replacement targets discriminate. Self-overlap and ownership refusals each
  disabled → their independent regression red (`rc=101`); restored source passes. D35/D57 close
  in defects-part7; graph privacy doctest passes, and all seven existing notch contracts pass.
- [x] **NO REGRESSION** — `make check` → fmt/strict clippy/all Rust suites green, `rc=0`;
  `make wasm` → green; `make book` → warning-free; fixture → `20 derived rows / 4 closure checks /
  5 pieces / 0 mismatch(es)`; feature/glossary censuses → `0 failure(s)`; ledger probes →
  `9 pass / 0 fail`; staged `make gate` → `=== all doctrines green ===`, all `rc=0`.
  The prior copy milestone's full `make probes` → `22 suite(s) green`, `rc=0`; no golden numbers change.
- [x] **LOCKSTEP** — pre-code sewing decision + INDEX, Rust module docs, ontology §4.2/§10 +
  local API vocabulary, feature matrix, fixture's now-settled self-seam record, task status/parents/
  evidence/frontier/logs, index, MEMORY, LIVE_STATUS, CHANGELOG and promoted DEV_NOTES. D35/D57
  closure descriptor + live pointer; changelog-part16/devnotes-part11 seal oldest entries atomically.
  The map keeps paths/owners after shortening its glossary orientation. Next `.3c.3b` is grainlines.

### `G1-SLICE.3c.3b` — directed grainlines and independent print references

- [x] **REPRODUCE / ISSUE** — ontology §4.6 requires direction, explicit angular intent and dual
  print references; `ea631c6` has no grainline object. Directed arrow semantics cannot collapse to
  endpoint-only identity or an undirected axis. D61 misstates the glossary declaration scope.
- [x] **ROOT CAUSE (WHY + WHERE)** — ordered interval traversal and authored direction are distinct
  from point fate and journal direction. `cargo test -p sc-core --test grain_contract
  reversed_traversal_orders_split_fragments_and_interior_repairs_from_its_own_start` → `1 passed`,
  `rc=0`: reverse traversal visits the second split fragment first and retains a lost middle repair
  in traversal position. `sed -n '388,426p' docs/tasks/artifacts/glossary/run_glossary_census.sh`
  → a global DECLARED_TOKENS set supplies C1, locating D61's diagnostic-only discrepancy, `rc=0`.
- [x] **FIX** — immutable Grainline with directed arrow, explicit alignment/angle source and optional
  independent stripe/plaid ranges; all fields require complete owned intervals and unique endpoints.
  Directional views reverse fragment order and compose reversal without rewriting raw range evidence.
  Reuse the tested ownership fold; expose G2 geometry and G4 profile-value obligations explicitly.
  Correct D61's diagnostic scope without changing predicates or exemptions.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test grain_contract` → `9 passed`, `rc=0`;
  opposite arrows, reversed split/repair order, all field scopes, ambiguous endpoints, merged foreign
  remainder after reversal, explicit/symbolic angles, optional metadata and immutability discriminate.
  Disable reversed next_back → order regression fails, `rc=101`; restored `make check` passes.
  Glossary probes → `10 pass / 0 fail`, `rc=0`; D61 closes in defects-part8. The partition oracle
  compares the old executable body to the linked chapter → `231 lines / 18018 bytes unchanged`, `rc=0`.
- [x] **NO REGRESSION** — `make check` → fmt/strict clippy/all Rust suites + privacy green, `rc=0`;
  `make wasm` → green; `make book` → warning-free; feature/glossary censuses → `0 failure(s)`;
  ledger probes → `9 pass / 0 fail`; `make gate` → `=== all doctrines green ===`, all `rc=0`.
  Existing eighteen sewing and seven notch contracts remain green after ownership sharing.
- [x] **LOCKSTEP** — existing structural/geometric decision extended before code; module/subsystem
  status, ontology bounded §10 index + linked implementation examples/SUMMARY, feature matrix,
  G1 frontier/evidence/logs, TASK_TREE, MEMORY, LIVE_STATUS, CHANGELOG and promoted DEV_NOTES.
  D61 seal/pointer; changelog-part17/devnotes-part12 preserve oldest live entries. Next `.3c.3c`.

### `G1-SLICE.3c.3c` — per-edge allowance intent and symbolic target policy

- [x] **REPRODUCE / ISSUE** — ontology §4.4 requires derived per-edge width, corner and target
  inclusion; `11eac44` has no allowance descriptor. Neither structural ownership nor a symbolic
  profile declaration proves an offset or resolves an inclusion policy.
- [x] **ROOT CAUSE (WHY + WHERE)** — owned endpoints cannot certify a whole merged edge.
  `cargo test -p sc-core --test allowance_contract
  a_merged_edge_with_owned_endpoints_and_foreign_middle_is_not_an_owned_allowance_edge` → `1 passed`,
  `rc=0`: a current edge has full ledger coverage and owned ends but includes a foreign middle;
  construction refuses that interval before and after reversal. Logical inclusion cannot select
  a value without a target profile, as the existing profile-binding decision specifies.
- [x] **FIX** — immutable per-edge descriptor holds width origin, explicit corner and mandatory
  profile inclusion declaration. Validate nonnegative authored width, complete interval coverage,
  unique endpoints and Piece scope through the shared ownership fold. Query raw evidence without
  rewriting held content. Offset/error-bound and target binding validation remain typed deferrals.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test allowance_contract` → `8 passed`,
  `rc=0`: explicit zero/negative width, symbolic sources, every corner, shared origins, immutable
  replacement and split/merge/reverse/delete evidence discriminate. Ownership refusal disabled →
  foreign-middle regression fails, `rc=101`; restored `make check` green. The privacy doctest passes.
  Earlier object checklists compare unchanged with the committed originals before relocation.
- [x] **NO REGRESSION** — `make check` → strict clippy/fmt/all Rust + privacy green, `rc=0`;
  `make wasm` → green; `make book` → warning-free; fixture → `0 mismatch(es)`;
  feature/glossary → `0 failure(s)`; ledger → `9 pass / 0 fail`; staged `make gate` →
  `=== all doctrines green ===`, all `rc=0`. Earlier sewing/notch/grain suites remain green.
- [x] **LOCKSTEP** — pre-code profile decision extended, shared geometric deferral docs generalized,
  Rust module/subsystem status, ontology §10 companion/examples/vocabulary and feature coverage,
  marks/allowances parent closed, frontier/evidence/logs/index, MEMORY/LIVE_STATUS/CHANGELOG and
  promoted DEV_NOTES; oldest lesson sealed to devnotes-part13. Next `.3c.4` constructions.






## Historical verification log

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-29` | tree seeded | `scripts/check_doctrines.sh` | `=== all doctrines green ===`, `rc=0` |
| `2026-09-30` | `.1` | `cargo metadata --no-deps`; `make check`; `make wasm`; `make gate`; `run_g0_exit_review.sh` | crates `sc-core, sc-units`; `21 passed` property + `1` doc-test; wasm build green; `=== all doctrines green ===`; `G0-17`/`G0-18` `MET` — every `.1` criterion re-derived, `rc=0` |
| `2026-09-30` | `.2` | `cargo test -p sc-units --test property`; `make wasm`; `make gate` | `21 passed` (round-trips, class separation, typed dimension/non-finite errors); wasm cross-build green; `=== all doctrines green ===` — every `.2` criterion re-derived, `rc=0` |
| `2026-09-30` | `.3a` | `cargo test -p sc-core`; `make check`; `make wasm`; `make gate` | `31 passed` unit + `9 passed` property; fmt/clippy `-D warnings` clean; `sc-core` cross-builds to wasm; `=== all doctrines green ===` — every `.3a` criterion re-derived, `rc=0` |
| `2026-10-01` | `.3b` | `cargo test -p sc-core`; `make check`; `make wasm`; `make gate`; `make probes`; `run_changelog_ledger_probes.sh` | `62 passed` unit + `8 passed` contract property + `9 passed` identity property; fmt/clippy `-D warnings` clean; `sc-core` cross-builds to wasm; `=== all doctrines green ===`; `22 suite(s) green`; ledger probes `9 pass / 0 fail` — every `.3b` criterion re-derived, `rc=0` |
| `2026-10-01` | `.3c.1` | `make check`; piece contract; `make wasm`; `make book`; `make gate`; release/feature censuses; ledger probes | `12 passed`; privacy doctest green; wasm green; book warning-free; all doctrines green; `0 failure(s)`; `9 pass / 0 fail`, `rc=0` |
| `2026-10-01` | `.3c.2a` | range contract; `make check`; wasm; book; gate; feature census; ledger probes | `13 passed`; all Rust suites green; wasm green; book warning-free; doctrines green; `0 failure(s)`; `9 pass / 0 fail`, `rc=0` |
| `2026-10-01` | `.3c.1a` | piece contract; `make check`; wasm; book; fixture/feature censuses; ledger probes; gate | `14 passed`; all Rust suites green; wasm/book green; `0 mismatch(es)`; `0 failure(s)`; `9 pass / 0 fail`; doctrines green, `rc=0` |

| `2026-10-01` | `.3c.3a` | notch contract; `make check`; wasm; book; feature/release censuses; ledger probes; gate | `7 passed`; privacy check green; Rust/WASM/book green; `0 failure(s)`; `9 pass / 0 fail`; doctrines green, `rc=0` |

| `2026-10-01` | `.3c.2b.1` | cut contract; check; wasm; book; feature/glossary/tree censuses; ledger; gate | `9 passed`; privacy green; all Rust/WASM/book green; censuses green; ledger `9 pass / 0 fail`; doctrines green, `rc=0` |

| `2026-10-01` | `.3c.2b.2` | sewing contracts; check; wasm; book; fixture/feature/glossary; ledger; gate | `18 passed`; graph privacy + notch suites green; Rust/WASM/book green; censuses/ledger/doctrines green, `rc=0`; prior copy milestone full probes `22 suite(s) green` |

| `2026-10-01` | `.3c.3b` | grain contracts; check; wasm; book; feature/glossary/probes; ledger; gate | `9 passed`; reversal mutation red; all restored checks green, `rc=0`; D61 fixed |

| `2026-10-01` | `.3c.3c` | allowance contracts; check; wasm; book; fixture/feature/glossary; ledger; gate | `8 passed`; ownership mutation red; restored Rust/WASM/book/censuses/gates green, `rc=0` |

| `2026-10-01` | `.3c.4a.1` | dart contracts; check; wasm; book; fixture/feature/glossary/tree; ledger; gate | `9 passed`; ownership mutation red; restored checks/censuses/gates green, `rc=0` |

| `2026-10-01` | `.3c.4a.2a` | fold contracts; check; wasm; book; fixture/feature/glossary/tree; ledger; gate | `9 passed`; ownership mutation red; restored checks/censuses/gates green, `rc=0` |

| `2026-10-01` | `.3c.4a.2b` | gather contracts; check; wasm; book; fixture/feature/glossary/tree; ledger; gate | `11 passed`; copy/ownership mutations red; restored focused checks/gates green, `rc=0` |

| `2026-10-01` | `.3c.4b.1` | layer contracts; check; wasm; book; fixture/feature/glossary/tree; ledger; gate | `9 passed`; scope/ownership mutations red; restored focused checks/gates green, `rc=0` |

| `2026-10-01` | `.3c.4b.2` | Hem contracts; check; wasm; book; fixture/feature/glossary/tree; ledger; gate | `10 passed`; identity/current-layer/ownership mutations red; restored checks/gates green, `rc=0` |


## Historical commit log

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| tree seed | `STITCHCAD-PLANNING-0002 (leaf PLANNING.2)` | created by the seeding leaf |
| `.1` | `STITCHCAD-G1-0001 (leaf G1-SLICE.1)` | reconciled: delivered by `G0-CONTRACT.18` (`eb83f01`) ahead of this leaf |
| `.2` | `STITCHCAD-G1-0002 (leaf G1-SLICE.2)` | reconciled: `sc-units` delivered by `G0-CONTRACT.18`; property-test decision recorded |
| `.3` | `STITCHCAD-G1-0003 (leaf G1-SLICE.3)` | decomposed into `.3a`/`.3b`/`.3c`; the three design boundaries recorded as layer-C decisions |
| `.3a` | `STITCHCAD-G1-0004 (leaf G1-SLICE.3a)` | the identity layer: `EntityId`/ULID + injected `IdGenerator`, `EdgeRef`/`PointRef`/`LocalTag`, the exact `Rational`/`Param`; 40 tests, wasm green |
| `.3b` | `STITCHCAD-G1-0005 (leaf G1-SLICE.3b)` | the persistent-identity contract: `IdentityLedger`'s append-only edit journal, fold resolution under split/merge/reverse/delete/offset, derived `RepairTask`s + release rule; 62 unit + 8 property tests, wasm green; recorded in `decision_reference-resolution-journal-fold.md` |
| `.3c.1` | `STITCHCAD-G1-0006 (leaf G1-SLICE.3c.1)` | immutable structural pieces with deferred geometry; D55 owned by the next child |
| `.3c.2a` | `STITCHCAD-G1-0007 (leaf G1-SLICE.3c.2a)` | exact whole-range journal fold and visible range repairs; D55 fixed |
| `.3c.1a` | `STITCHCAD-G1-0008 (leaf G1-SLICE.3c.1a)` | D56 fixed: separate cut-once L/R members, explicit companion metadata |
| `.3c.3a` | `STITCHCAD-G1-0009 (leaf G1-SLICE.3c.3a)` | semantic notches and symbolic bindings; D58 fixed |
| `.3c.2b.1` | `STITCHCAD-G1-0010 (leaf G1-SLICE.3c.2b.1)` | explicit physical-copy plan; D59/D60 fixed |
| `.3c.2b.2` | `STITCHCAD-G1-0011 (leaf G1-SLICE.3c.2b.2)` | copy-addressed sewing; D35/D57 fixed; sewing parents closed |
| `.3c.3b` | `STITCHCAD-G1-0012 (leaf G1-SLICE.3c.3b)` | directed grainlines; independent print references; D61 fixed |
| `.3c.3c` | `STITCHCAD-G1-0013 (leaf G1-SLICE.3c.3c)` | per-edge allowance intent; marks/allowances parent closed |
| `.3c.4a.1` | `STITCHCAD-G1-0014 (leaf G1-SLICE.3c.4a.1)` | semantic dart intent; explicit conservation deferral |
| `.3c.4a.2a` | `STITCHCAD-G1-0015 (leaf G1-SLICE.3c.4a.2a)` | distinct tuck/pleat intent; shared validation |
| `.3c.4a.2b` | `STITCHCAD-G1-0016 (leaf G1-SLICE.3c.4a.2b)` | gather physical-side binding; canonical ease; intake parents closed |
| `.3c.4b.1` | `STITCHCAD-G1-0017 (leaf G1-SLICE.3c.4b.1)` | served-layer intent; lining execution scope refusal |
| `.3c.4b.2` | `STITCHCAD-G1-0018 (leaf G1-SLICE.3c.4b.2)` | Hem depth/fold intent and current Facing composition; hem/layer parent closed |
| `.3c.4c.1a` | `STITCHCAD-G1-0019 (leaf G1-SLICE.3c.4c.1a)` | physical notion placements; current copy/anchor/direction validation |
| `.3c.4c.1b` … `.16` | `pending` | remaining constructions and G1 execution leaves |


## Historical task changelog

## Changelog

- `2026-09-29`: Tree created by `PLANNING.2` with 16 leaves; owns defect D10 (starter crate).
- `2026-09-30`: `.1` reconciled and closed — its workspace shape (starter crate retired, `sc-units` +
  `sc-core` created, G0 CI shape) was delivered by `G0-CONTRACT.18` (`eb83f01`) "ahead of `G1-SLICE.1`";
  every acceptance criterion re-derived by command and recorded in the `### G1-SLICE.1` checklist. D10 was
  already closed by that commit; the frontier advances to `.2`.
- `2026-09-30`: `.2` reconciled and closed — `sc-units` (fixed-point µm/µ°, the five tolerance classes T1–T5,
  exact-ratio conversions, typed `UnitError` for domain/overflow/division-by-zero/non-finite) was delivered in
  full by `G0-CONTRACT.18` (`eb83f01`); its 21 property tests, typed dimension errors and wasm build were
  re-derived against every acceptance criterion. The property-test-framework Open Question is resolved into
  the Decisions section and recorded as `decision_property-tests-dependency-free-recorded-seed.md`. The
  frontier advances to `.3` (the `sc-core` ontology), G1's first new product code.
- `2026-09-30`: `.3` decomposed into `.3a` (identity types), `.3b` (the persistent-identity contract) and
  `.3c` (the geometry-bearing object types) — one leaf was three signoff-quality slices in strictly ordered
  dependency. Its three cross-cutting design boundaries were recorded before any code, so the implementation
  slices build against a fixed design: a dependency-free ULID `EntityId` with an injected generator, a
  bounded exact rational edge parameter in `sc-core` (not `sc-units`, not the formula bigint), and the
  structural-at-G1 / geometric-at-G2 invariant boundary. The tree is 18 leaves; the frontier advances to
  `.3a`.
- `2026-09-30`: `.3a` landed — G1's first new product code. `sc_core::ontology`'s identity layer:
  `EntityId` (a dependency-free hand-rolled ULID, Crockford base32, lexicographically sortable) and the
  injected `IdGenerator` with a `DeterministicIdGenerator` for replay; `EdgeRef`/`PointRef`/`LocalTag` (stable
  topological references, never indices); the bounded exact `Rational` and its `[0, 1]` `Param`. 31 unit tests
  and 9 dependency-free recorded-seed properties green, `sc-core` still cross-builds to wasm, `make gate`
  green. Built against `decision_entity-identity-ulid-injected-generator.md` and
  `decision_edge-parameter-bounded-exact-rational.md`. The frontier advances to `.3b` (the persistent-identity
  contract).
- `2026-10-01`: `.3b` landed — G1's second new product code. `sc_core::ontology::topology`: the
  `IdentityLedger` (an append-only journal of typed `TopologyEdit`s + a live-edge index + registrations) and
  the pure fold that resolves a held `(EdgeRef, Param)` through split, merge, reverse, delete and
  offset-fragmentation. A stored reference is never rewritten; repair state (`RepairTask`, `open_repairs`,
  `release_readiness`) is derived, never stored, so it cannot drift from the journal. Split offers both
  fragments at the split point for the consumer to state a `SplitSide`; merge recomputes by caller-declared
  arc length in exact `Rational`; reverse maps `t` to `1 − t` and tells directed consumers via `Direction`;
  delete and offset orphan references into visible `RepairTask`s naming the reference, the orphaning edit and
  the candidates — no silent reassignment. 62 unit + 8 recorded-seed contract properties green (merge checked
  against a cross-multiplied `i128` oracle, offset against a hundredths grid, the live set against the journal
  replayed, replay byte-identical), `sc-core` still cross-builds to wasm, `make gate` + `make probes` (22
  suites) green. Built against `decision_reference-resolution-journal-fold.md`, recorded before the code. The
  append also discharged two containment obligations it owed (D54: the CHANGELOG and DEV_NOTES rollovers the
  prior slice crossed without sealing) and surfaced D53 (the KNOWLEDGE_MAP ceiling pressure `.3a` flagged).
  The frontier advances to `.3c` (the geometry-bearing object types).

- `2026-10-01`: `.3c` decomposed into four children; `.3c.1` landed `Piece`, its immutable validated
  content and complete derived label view. Structural refusals are tested, geometry is always
  `DeferredToG2`, endpoints expose repairs without rewriting references. A tracked middle-fragment
  deletion counterexample creates D55, owned immediately by `.3c.2` before sewing spans are built.
  The book's two literal scope placeholders were repaired after the renderer diagnosed hidden HTML;
  feature coverage now cites the implemented piece contract. All focused gates pass; next `.3c.2`.

- `2026-10-01`: `.3c.2` split into range resolution (`.2a`) and sewing graph (`.2b`). `.2a` fixes
  D55: the exact interval fold retains deleted/trimmed portions and arithmetic failures as repairs,
  separately from point endpoint ambiguity and G2 geometry. Thirteen contract tests and the existing
  suites pass, with the delete arm observed red when disabled. The ledgers roll over atomically;
  D55 is sealed closed in defects-part2. Next `.3c.2b` implements sewing spans and settles D35.

- `2026-10-01`: `.3c.1a` fixes D56, found by comparing the new model with the canonical fixture:
  separate cut-once L/R members now carry explicit hand/companion metadata; self-pairing is refused.
  Existing even-total pair requests stay supported. Fourteen piece tests and all focused checks pass;
  D56 is sealed closed. D57's physical-copy addressing is asked of the director and owned by `.3c.2b`;
  the frontier takes independent marks/allowances (`.3c.3`) while that decision is pending.

- `2026-10-01`: `.3c.3` decomposed into notches, grainlines and allowances; `.3c.3a` lands
  immutable semantic Notch anchors and symbolic target-profile bindings with no physical defaults.
  Seven contract tests + privacy doctest pass; ownership mutation goes red. D58's G4 acceptance
  aligns with the normative release matrix and seals closed. Ledgers roll over atomically;
  the director answered D57 before commit: stable physical-copy identities; next `.3c.2b`.

- `2026-10-01`: `.3c.2b` splits into copy plan and spans. `.3c.2b.1` implements the director's
  stable-copy ruling with explicit identities and complete quantity/orientation validation. Nine
  contract tests pass and quantity mutation goes red. D59's committed-baseline vocabulary failures
  are repaired; glossary census is a required focused ontology check. Completed `.1`/`.2`/`.3a`/`.3b`
  evidence blocks move to a linked sibling (SHA-256
  `ab447c14f2db092114863e1ccbb4f555441dbb204abe18de80cbe19fd8698517`). Next `.3c.2b.2`.

  D60's staged relocation refusal is fixed by preserving the original `.1`/`.2` text and adding
  re-derived delivery evidence inside their ROOT CAUSE bullets; all four moved checklists pass
  the per-bullet audit and staged gate. Original payload checksum above describes the pre-supplement
  move, not the final augmented sibling.

- `2026-10-01`: `.3c.2b.2` lands immutable copy-addressed sewing intent, disjoint same-copy seams,
  semantic stops and explicit ease distributions. Eighteen tests and graph privacy pass; both
  self-overlap and interval-ownership mutations go red. D35/D57 seal verified closed; `.3c.2b`
  and `.3c.2` close. Prior copy milestone full probes: 22 suites green. Books/censuses remain aligned,
  with no invented folded-end geometry or changed arithmetic golden. Next `.3c.3b` grainlines.

- `2026-10-01`: `.3c.3b` lands directed grain and independent stripe/plaid references with
  explicit angle intent and deferred geometric/value validation. D61 diagnostic fixed; ontology §10
  executable examples move unchanged to a linked chapter. Next `.3c.3c` allowances.

- `2026-10-01`: `.3c.3c` lands per-edge allowance descriptors and closes marks/allowances.
  Earlier object checklists move unchanged to the existing evidence sibling before 1000 lines.
  Moved payload: 213 lines / 19504 bytes, SHA256
  `d07e9e3c21252f6d903cb5fcedfde703897a34074e7efe7fce70efe5862a1dff`. Next `.3c.4` constructions.

- `2026-10-01`: `.3c.4` decomposes into intake/finish/closure/pocket children; `.3c.4a.1` lands
  semantic dart intent with an internal apex and explicit intake/operation provenance. Conservation
  remains G2/G3. A bounded construction companion carries examples. Next `.3c.4a.2`.

- `2026-10-01`: `.3c.4a.2a` lands distinct immutable tuck/pleat intent with shared fold validation
  and directed repair evidence. Parent `.2` splits to keep sewing-linked gathers separate. Physical
  shape/conservation remain G2/G3; live-window rollover owned and verified. Next `.3c.4a.2b`.

- `2026-10-01`: `.3c.4a.2b` containment moves three recent completed mark/dart checklists
  unchanged to the existing sibling: 90 lines / 8135 bytes, SHA256
  `7ca4e34bf67fc0cd9b5d708d51ec00443f88225b8aaedb603fa235670a87a1b0`. Current gather evidence stays first.

- `2026-10-01`: `.3c.4a.2b` lands immutable gather bindings and borrowed canonical span ease,
  preserving original physical material targets and raw repair evidence. Intake parents close with
  all four kinds implemented structurally; actual closing remains G2/G3. Next `.3c.4b`.

- `2026-10-01`: `.3c.4b.1` containment moves completed fold evidence unchanged to the sibling: 28
  lines / 2558 bytes, SHA256 `a5723a6070b5da7125c5d7bf6d455123703ba8f55a65a96a521697235e0c69ad`.

- `2026-10-01`: `.3c.4b.1` lands three served-layer types and explicit lining scope refusal; recipe
  owns offset dimensions, shared material guard preserves Piece behavior. Next `.3c.4b.2` Hem.

- `2026-10-01`: `.3c.4b.2` moves completed layer/gather checklists unchanged to the evidence sibling;
  independent committed-payload comparison passes, with staged revalidation before commit.

- `2026-10-01`: `.3c.4b.2` lands Hem intent with explicit depth/fold origins and stable current Facing
  validation. Hem/layer parent closes structurally; physical folding remains G2/G3. Next `.3c.4c`.

- `2026-10-01`: `.3c.4c.1a` moves completed Hem evidence unchanged to the existing sibling: 28
  lines / 2590 bytes; SHA256 `6ab6459295f45c7c816826b47810e301f07949dbba070c5ca60a1cc4b3f960f1`.

- `2026-10-01`: `.3c.4c.1a` verification-log containment preserves the completed table unchanged:
  32 lines / 4337 bytes; SHA256 `ba3645f60a7be24edcc500079448238f951041de50cd45af09da22223825aac4`.

- `2026-10-01`: `.3c.4c.1a` lands stable physical notion placements and shared current anchor checks;
  raw repairs remain visible. Closure kinds/counts/sizes follow `.1b`/`.2`, with geometry deferred.

- `2026-10-01`: `.3c.4c.1b` preserves completed placement evidence and commit-log history unchanged
  in the evidence sibling, with independent committed-content comparison and direct retrieval.

- `2026-10-01`: `.3c.4c.1b` lands centred zipper/hook-bar intent with stable physical instances,
  canonical current placements and named env_fly refusal. Next `.3c.4c.2` button/buttonhole derivation.




### `G1-SLICE.13` (acceptance rewritten by `G0-CONTRACT.11`) — a consumer leaf names its instrument, not a protocol in prose

This tree had no completed leaf, so it carried no acceptance boxes — and a staged `docs/tasks/*.md` file
with no ticked box is refused by `scripts/check_task_acceptance.sh` whenever the same commit stages code.
These boxes are the evidence for the change `G0-CONTRACT.11` made to this tree, added in the commit that
made it, which is the remedy `G0-CONTRACT.4c` used for `G3-GRADING.md`.

- [x] **REPRODUCE / ISSUE** — `.13`'s acceptance pointed at a protocol that did not exist anywhere in the
  repository: `git show HEAD:docs/tasks/G1-SLICE.md | grep -c 'the measurements the protocol named are
  recorded'` → `1`, `rc=0`, while `git ls-tree HEAD docs/decisions/ | grep -c adr-0002` → `0`, `rc=1`. A
  leaf whose acceptance names an absent document cannot be verified, and the spike it owns is the one the
  roadmap warns must not be argued after the fact.
- [x] **ROOT CAUSE (WHY + WHERE)** — the protocol is `G0-CONTRACT.11`'s deliverable
  (`grep -n 'ADR-0002 — UI stack' ROADMAP.md` → `328`, `rc=0`), and that leaf had not been taken, so the
  consumer was written against an intention. The fix is not a better sentence in this tree: it is the
  protocol existing as an instrument whose output this leaf's acceptance can cite.
- [x] **ADDRESSED (verified)** — `.13`'s acceptance now names the instrument by path, the corpus by its
  declared numbers (16 pieces, 400 boundary vertices each, 200 pick probes, 200 snap probes, 1 000 fidelity
  round trips), and the evidence obligations a verdict is scoped to (reference hardware, OS versions, corpus
  script identity), and requires a recorded decision naming the rows for any human overruling the printed
  verdict. The instrument exists and reports the honest state:
  `bash docs/tasks/artifacts/canvas_spike/run_spike_verdict.sh` → `spike verdict: 0 rows / 0 profiles /
  PENDING — ADR-0002's canvas half stays `proposed` until G1-SLICE.13 records measurements / 0 refusal(s)`,
  `exit=0`; `TMPDIR=$PWD/target/scratch bash docs/tasks/artifacts/canvas_spike/run_spike_verdict_probes.sh`
  → `probes: 13 pass / 0 fail`.
- [x] **NO REGRESSION** — `scripts/check_doctrines.sh` → `=== all doctrines green ===` once this section
  existed (before it, the same command refused this file by name: `TASK-ACCEPTANCE: docs/tasks/G1-SLICE.md
  has no 'ROOT CAUSE' box in its acceptance checklist`, `exit=1`, which is the gate working rather than a
  defect in it); `make probes` → `18 suite(s) green`; `bash
  docs/tasks/artifacts/planning/run_tree_coverage_census.sh` → `census: 10 lanes / 13 trees / 3 sibling(s)
  / 0 unowned / 0 orphan(s) / 0 dead link(s)`, `exit=0`. No Rust file changed in this slice, so `make
  check` is not its gate; the two new instruments are bash and both were run.
- [x] **FIX** — rewrote `.13`'s acceptance to consume the protocol as an artifact, and added this
  subsection so the tree file carries fresh evidence for the change it stages.
- [x] **LOCKSTEP** — `G0-CONTRACT.md`'s leaf `.11`, its frontier, decisions, three logs and checklist;
  `MEMORY.md`, `LIVE_STATUS.md`, `CHANGELOG.md`, `DEV_NOTES.md`, `docs/TASK_TREE.md`, `TOOLBOX.md`,
  `docs/decisions/INDEX.md` and the regenerated Knowledge Map, all in this commit.

## Third history window before reference namespace records

- ID: `G1-SLICE.5b.1b.0`
  Status: `done`
  Goal: discharge the measured64-file working-history bound before .1b's next normal seal.
  Pre-code protocol: capture all current raw sealed records from exact af98fff into window3;
  do not alter prior windows/manifests/payloads or any decoded bytes, limits, reader or checker.
  Reuse the published manifest/catalog/payload contract; frozen-source capture tool generates
  an isolated same-volume fixture and compares all original full files/read/materialization.
  Install copies, verify/source-prove/use the installed reader before deleting exactly the captured
  raw copies; redirect maintained Markdown pointers to member catalog anchors. Prove residue0,
  previous windows exact and complete logical membership unchanged. No repository boundary crossed.
  Own oldest live ledger rollover/current resume/book upkeep and completed-task evidence relocation
  within existing siblings if needed; no new sibling or cap increase. Product .1b remains next.
  Acceptance: all exact source/member evidence, independent newest refusal/previous-window controls,
  full local Rust/WASM/book/probes/gates terminal, locked live/book/task pointers. Commit before
  exceptional push required by COMMIT.md; remote/newest-committed proof is owned by .0v.
  Handoff diagnostic owned here: recheck process-census visibility if restricted ps fails;
  use a controlled local open-file process, observe real versus restricted verdicts, stop it
  before recording completion; route any checker defect to a scheduled SPINE leaf after clean CI.
  Verification: local capture/source/residue/full checks pass; detailed receipts below.
  Commit: `STITCHCAD-G1-0074`; observed remote/newest-committed proof remains .0v.

- ID: `G1-SLICE.5b.1b.0v`
  Status: `done`
  Goal: observe both actual CI jobs/steps at the exact window3 pushed head and newest committed
  catalog immutability refusal; record authoritative completed/success results before .1b work.
  Acceptance: exact SHA/run/job/step receipts, no aggregate-only polling inference; fix failing jobs
  if any, committed durable receipt, clean/no-jobs resume to namespace/preflight reference review.
  Verification: exact-head two jobs/all steps completed-success; exclusive archive28/CLI200 green.
  Commit: `STITCHCAD-G1-0075`; P0 SPINE.23 precedes namespace review.

### Third-window local receipts — `2026-10-02`

- Frozen af98fff capture:62 exact full files/1635 lines/117560B;37055 compressedB/174080tarB,
  SHA256 cf4b959af6083df735534c6798dc866232cff6e489b48cbf6c6adfe03a3be017. Fresh isolated
  same-volume capture and installed-input fixtures prove all189 historical logical records exactly;
  no missing/extra identities. Installed reader source proof precedes exact62-file retirement; residue0.
- Prior two catalogs/manifests/payloads independently compare exact to Git HEAD; published reader,
  checker, schema and all bounds unchanged. Two new complete oldest ledger seals retain951/1432B
  original payloads exactly. Current history191 logical/5 workingMarkdown/9403 decodedlines/
  714065 decodedB/302579 residentB; canonical defect census12open/102unique sealed/overlap0.
- `make check` terminal exit0:591 tests/48groups, strictfmt/clippy green. `make wasm` and `make book`
  separately repeated with terminal exit0 after wrapper exit visibility was clarified; three libraries
  cross-compile, warning-free rendered HTML. No Rust source/test changed.
- `make probes` terminal exit0:26 suites; publication53chapters/25scoped APIs/1102source/1705rendered
  links,9 controls; ledger9 and pointer13 controls green. A manual archive CLI runner overlapped the
  shared-fixture full suite; its receipts are discarded. Exclusive archive rerun terminal exit0:
  28 probes and199 CLIcontrols/191 reads/3windows; newest committed window3 arm awaits .0v.
- Tree census10lanes/13trees/10siblings/0unowned-orphan-deadlinks; diff check0. Gate receipt follows.
- D114: controlled live PID46805 held a local file; restricted checker printed OK0 while ps denied.
  OS-visible checker detected it and returned1. Stopped controlled session terminal1/KeyboardInterrupt;
  real ps confirms PID absent. D115: four idle CUA kernel/worker services have repo cwd only, no repo
  file handles, but root metadata triggers blocking. No CUA action/result is pending. Never stop shared
  infrastructure. Both defects are owned P0 SPINE.23 after current clean archive/CI; restricted green
  is invalid evidence meanwhile. No project verification process remains in flight at local commit.
- Final staged `make gate` →13 doctrine checks/all doctrines green, terminal exit0.

### Third-window observed CI — `G1-SLICE.5b.1b.0v`, `2026-10-02`

- `git push origin main` terminal exit0: f876913..b595a37, ahead0. No dirty tracked input at push.
- Exact head b595a37bda29a8f019e06bdb09c79ef395decb4d. Abbreviated head filter returns0 runs;
  corrected full SHA yields2. rust run37060421597/job111015421472 check; doctrines
  run37060421575/job111015421405 enforce. Both completed/success, every reported step success;
  APIs terminal0. Rust run aggregate initially in_progress; authoritative job completed/success.
- Exclusive post-commit archive suite terminal exit0:28 pass/0fail;200 CLIcontrols/191 logical
  reads/3windows. Newest committed catalog edit refuses rc1, as do payload/member/membership/collision
  controls. No source/schema/caps changed. Full local native/WASM/book/probes/gate earned in .0.
- Oldest live G1-0057 ledger sealed exact predecessor bytes; newest receipt is ordinary docs.
  P0 SPINE.23 now takes clean frontier; D114/D115 remain12open/102sealed until verified repair.
- promotion: declined (canonical observed-job and actual-refusal policy applied).
- Receipt leaf focused publication9, ledger9/pointer13 and staged make gate all terminal0; no needed job remains.

## Namespace and whole static preflight review

- ID: `G1-SLICE.5b.1b.1`
  Status: `done`
  Goal: D116/D117/D118/D120 reference namespace and single-statement static header repair.
  Pre-code contract: formula contract2/3/3.1/4.1/5.1/5.2 and grammar1/1.1/5/7. Expected nine
  origins/eight reserved names authored independently; read actual declared populations both ways.
  Build namespace from declaration pairs before a dictionary can hide duplicates. Validate machine
  names/kinds/origins with no value/state/geometry reads; reject reserved rebinding and collisions.
  Separate syntax_statement, static_statement and runtime statement, preserving syntax-only
  fixture/canonical bytes and runtime tuple shapes. Prior syntax controls use the actual syntax phase,
  not runtime inference traps that can now be preempted by valid early namespace refusals.
  All six let kinds, eight reserved bindings, prior recipe/input collisions and both arithmetic
  assertion operands checked before evaluation; header tolerance roles use precisely five names.
  Non-class tolerance annotation violates grammar TOLERANCE and raises formula_parse; unavailable
  valid tolerance remains a runtime refusal. Preserve original assertion spacing and syntax scope.
  Fixture-only computed_fixture origin becomes canonical parameter in existing numeric controls.
  Independent metadata-only declarations, role/context/spelling/collision/header matrices and actual
  compiled in-memory guard assertion faults, source unchanged during controls. Runtime T1 provenance
  D121 belongs .5e.3; atomic whole-recipe no-execution D119 belongs next .1b.2.
  Acceptance: every scoped reference defect fixed, focused existing signature/syntax/binding/book
  controls green; current book/live/task records synchronized, exact old records retained, commit.
  Verification:1139 metadata/static cases/13 actual body assertion reds; full reference/language16,
  Rust30/3groups/publication9 pass0. Commit: `STITCHCAD-G1-0076`; .1b.2 next.

- ID: `G1-SLICE.5b.1b.2`
  Status: `done`
  Goal: D119 whole ordered static preflight before any reference value computation.
  Pre-code protocol: read contract3/4.1/4.3/5.1/9, grammar1/5/7, actual namespace/static_statement,
  product ordered/statement public parsing contracts and book L2/L4 replay. Partition original
  ASCII source at top-level let/assert tokens, preserving spacing; newlines remain whitespace.
  Consume initial declaration pairs before dictionaries; build only local kind/origin metadata,
  publishing each successful let for subsequent statements. Reject bare expressions in recipes.
  Return a complete tuple only after every header/operand/branch and structural limit succeeds;
  no caller namespace update, numeric/state/availability/geometry read or runtime callback.
  Existing parser literal-input conversion remains permitted. Independently author ordered source,
  last-error, reserved/context/collision, 4096/4097, node/depth and no-partial-result controls;
  compile actual guard faults in memory and require named body assertion reds. Book's17 lets and
  four assertions must preflight together before L2 starts; copied-book late static error plus
  earlier division by zero proves consumer ordering. Preserve runtime-only refusal examples.
  Watch through existing structural probes, document scope in existing annex/index, focused
  signature/namespace/language/publication/ledger/gate checks and per-leaf commit.
  Acceptance: all statements/branches/headers and declaration order checked, no execution/value
  reads, no accepted partial plan after late static refusal; initial-origin collision and last
  statement/structural limits independently tested. Integrate book replay after full preflight,
  preserving valid values and runtime refusal examples' separate execution stage. All namespace/
  signature controls remain watched. Product graph/typed argument payloads remain .5b.2–.4.
  Related D123: old L8 adds13 independent refusal cases to the21-statement worked recipe;
  reproduce a declared25-statement ceiling before repairing actual per-recipe measurement here.
  Verification:196 whole-source cases/14 actual guard assertion reds; consumer3 and per-recipe
  measurement control green, runtime/value/geometry trapped; full reference/language16/publication9
  pass0. D119/D123 fixed, original reports retained. Commit: `STITCHCAD-G1-0077`.

[Exact completed records](G1-SLICE-canonical.md#completed-namespace-and-preflight-receipts--preserved-from13f8c75) are retained in the canonical sibling.

## Runtime assertion repair

- ID: `G1-SLICE.5e.3a`
  Status: `done`
  Goal: independent D125 reference false-assertion diagnostic repair while D124 is pending.
  Acceptance: valid assertion tuples preserved; false raises formula_assertion with name/values/
  class; five classes and numeric kinds, inclusive threshold, earlier static/runtime errors,
  actual consumer refusal and actual guard faults verified. D121 provenance remains separate.
  Containment prerequisite: move exact completed receipts/checklists into existing canonical sibling
  before primary/evidence exceed1000 lines; update links, retain hashes; no new path or cap increase.
  Pre-code protocol: contract5.1/5.2/9, grammar1.1 assert and existing class/context/value stages.
  FErr gains an optional owned arguments dictionary; false assertion carries label, both exact
  values/kinds and symbolic tolerance class/value. No fabricated ordinal/canonical bytes or claim
  of typed production payloads. Preserve true five-element tuple and <= threshold; earlier static,
  numeric and missing-context errors retain their tokens. Consumer L4 must report named failure
  instead of testing a False tuple; numerical provenance D121 and context routing D122 stay owned.
  Independently author all five arithmetic kinds/classes, below/at/above threshold/signed values,
  caller immutability/static-valid false example and actual copied-book consumer diagnostic.
  Compile guard/payload faults in memory; require actual body assertion reds, unchanged producer.
  Focus reference/language/publication/ledger/gate; synchronize current docs and commit .3a.
  Root-cause check of initial fixture failure: grammar5.1 and independent signature matrix both
  declare Count*Ratio -> Ratio, while Count/Ratio -> Count. The actual loader agrees; no contract
  drift. Use doubled integer bindings divided by2.0 to produce exact halves in all five kinds.
  Verification:262 cases/eight actual body assertion reds/copied-book named refusal/full reference
  terminal0; publication/ledger/gate receipts below before commit. Commit: `STITCHCAD-G1-0079`.

## Complete static review: exclusion diagnostic boundary

- ID: `G1-SLICE.5b.1c.1`
  Status: `done`
  Goal: map static obligations and reproduce undefined excluded-construct recognition before
  implementing product .5b.2–.4. Parent .1c owns complete review; this child owns D124 diagnosis,
  reproducible baseline producer, concrete decision proposal and synchronized book/live records.
  Pre-code evidence: actual static_statement with numerical/storage/geometry callbacks trapped:
  loop(width), repeat(2,width), while(width>0 um) raise formula_unbound_name; fn/macro definition
  shapes raise formula_parse; width^3 raises formula_unsupported; spline/solve preserve envelope
  tokens. Declared scalar name loop is accepted. Contract6 requires unsupported loops/functions,
  while grammar1.1 reserves only let/assert/if and grammar6 requires unknown calls unbound.
  Protocol: compare closed populations/clauses and existing4032/1139/196 matrices in both directions;
  independently retain these actual baseline source/token results and envelope precedence faults.
  All21 worked static headers and13 independently authored refusal sources/outcomes are checked
  against actual book rows, including static acceptance of runtime-only failures. D125 actual false
  assertion returns a Boolean verdict, equal control returns True; schedule runtime repair .5e.3.
  D126: public arc radius5m/sweep270deg fits10m square; actual reference length23561945um,
  half-ratio-quantum distance11.7809725um; pi>3 proves>11.25um>T2. Repair grammar6.1's false
  bbox-to-edge-length implication; add independent arithmetic counterexample, no geometry claim.
  Do not pretend example spellings define v1 excluded syntax. Record recommended diagnostic policy
  and alternative with compatibility effects; obtain director clarification before changing it.
  Acceptance: pending boundary explicitly owned, producer/fault controls green, all remaining
  static proof owners mapped, book/task/live records honest, commit completed diagnostic child.
  Verification:67 actual static cases/four compiled guard assertion reds;21/13 book populations
  exact, execution/value/geometry trapped. D126 independent exact chord bound and current reference
  arithmetic control pass0. Full reference/language16/publication9 pass0; D124/D125 remain open.
  Commit: `STITCHCAD-G1-0078`; .1c.2 awaits director ruling, parent not complete.

- ID: `G1-SLICE.5b.1c.2`
  Status: `pending`
  Goal: settle D124 excluded-syntax diagnostic ruling, implement its exact recognizable forms and
  exclusions/precedence controls, then close complete .1c static obligation map.
  Acceptance: chapter/grammar/reference diagnostic rules agree, no unintended identifier reservation;
  all worked/refusal static outcomes and remaining .5b.2–.4/.5c–.5g owners verified and explicit.
  Director diagnostic choice required; independent static review continues at .1c.1 meanwhile.
  Verification: `pending`; Commit: `pending`.

### Static review receipts — `G1-SLICE.5b.1c.1`, `2026-10-02` (UTC)

- Actual static_review_contract.py --mutations terminal0:67 cases,21 worked headers and13 refusal
  sources/outcomes independently authored;6 envelope calls retain precedence over unknown operands/
  kinds, including either conditional branch. Four actual compiled body assertion reds and unchanged
  source. Metadata-only fixture declarations/reserved views, all execution/storage/geometry trapped.
- Three refusal candidates statically pass and retain runtime owners; a false numeric assertion also
  passes static checking. Separate actual runtime D125 baseline returnsFalse for1cm==2cm, True for
  equal-value control, exit0; .5e.3 owns named diagnostic repair with D121. No runtime approval.
- D124 scope census: read full canonical contract2/3/5/6/9 and grammar1/5/6/7; three keywords
  let/assert/if, unknown-call rule and excluded rows. Actual loop/repeat/while calls unbound, fn/macro
  definition shapes parse, scalar loop accepted, ^3 unsupported. No recognized excluded source forms
  declared there. This gap concerns normative source recognition, not a claim about all repositories.
  ADR-0003 concrete proposal preserves existing grammar; alternative requires exact forms/name effects.
  Director clarification requested; parent .1c and production exclusion dispatch remain unapproved.
- D126 root/public contracts: units1.1 permits10m square; units4 permits270deg/radius5m arc.
  Actual reference length23561945um, independent chord11.780972451um, exact3<pi<22/7 and
  sine lower inequality prove chord>10um. Correct false bbox-to-length implication in grammar6.1;
  counterexample watched by structural runner. Pure arithmetic proof, no product geometry execution.
  ROUTING EVIDENCE: curve/box/T2 facts come from separate units contracts; independent exact
  primitive-arc inequality reproduces without the formula evaluator. Actual selector integration/
  combined error budget stays .5f.3/G2, with acceptance explicitly added to .5f.3; no fabricated case.
- Current language16/publication9 terminal0;53chapters/25scoped APIs/1112source/1721rendered links,
  warning-free book. Full reference runner terminal0 includes4032/1139/196 prior matrices and review67;
  focused final producer also earns D126 arithmetic controls. Rust source/test bytes unchanged.
- Complete static obligation table and public-input implementation sequence .5b.2–.4 recorded in
  existing annex. Storage/bus owners checked against actual .7/.6 nodes, not guessed .9/.5f paths.
  Original D126 report retained in part46; D124 recognition and D125 runtime errors remain owned.
- Final ledger9/pointer13 and staged gate13 doctrine checks terminal0. Initial gate caught334B G1
  live-row width; first shortening targeted the wrong row, corrected actual row without raising320B
  ceiling. Retention202logical/16workingMarkdown/9731decodedlines/737344decodedB/325858residentB;
  independent14open/111unique sealed/overlap0. Tree10lanes/13trees/10siblings/0unowned-orphan-deadlinks.
  Old G1-0061 ledger1091B/CI lesson625B independently match Git predecessor slices. No Rust diff;
  map100lines/8181B, diff check0. D124 choice remains pending; no implementation change while awaiting it.

### Assertion repair receipts — `G1-SLICE.5e.3a`, `2026-10-02` (UTC)

- `python3 -I -B docs/tasks/artifacts/formula_structure/assertion_contract.py --mutations`
  terminal0:262 independently authored cases/five arithmetic kinds/five classes/inclusive boundary,
  exact owned failure arguments and caller unchanged. Actual copied-book consumer refuses1 with
  formula_assertion/waistband_width_closure/length80000vs40000/eps_num1; producer verdict0.
- Eight actual in-memory compiled false-guard/boundary/token/label/order/kind/class/value faults
  require body assertion reds; source unchanged. Full structural runner terminal0 retains prior
  signature4032/namespace1139/recipe196/review67 and numerical/canonical/binding families.
- Initial test syntax/setup errors were discarded before evidence; Count*Ratio failure diagnosed
  through actual loaded table, grammar5.1 and independent signature matrix. All specify Ratio;
  correct Count/Ratio control preserves Count. No new contract or hidden table repair.
- Language16 and publication9 terminal0; runtime assertion annex/index/contract/tool route agree.
  D125 original report, oldest G1-0062 ledger and two oldest lessons independently match Git HEAD
  before sealing. Completed52/63line task blocks retain exact predecessor bytes in existing sibling;
  Knowledge Map path set unchanged, no cap raised. Rust/serializer source bytes unchanged.

- Publication53chapters/25 API rows/1114source/1725rendered links; nine refusal controls, terminal0.
  Ledger9/pointer13 and tree census10lanes/13trees/10siblings/0unowned-orphan-deadlinks pass0.
  Independent materialized defect census13open/112unique sealed/overlap0/duplicates0, rc=0.
  Staged doctrine registry13 checks terminal0; hook repeats this final staged record.

## Missing-value routing

- ID: `G1-SLICE.5e.1a`
  Status: `pending`
  Goal: independently repair D122 reference missing-value diagnostic routing across nine origins
  and reserved size/tolerance contexts before product .5e.1 adapter proof.
  Acceptance: canonical origin-specific tokens/arguments, populated values unchanged, malformed
  metadata refused, actual fault controls and book/runtime replay remain honest; D124 untouched.
  Verification: `pending`; Commit: `pending`.
