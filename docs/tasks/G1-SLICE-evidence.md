# G1-SLICE — acceptance evidence for completed leaves

The evidence sibling of [`G1-SLICE.md`](G1-SLICE.md), split before the next checklist takes the
parent past 1000 lines (`.doctrine/live_document_size/surfaces.tsv`, `tasks_collection`).
Completed checklists are copied in landing order; D60 adds explicit revalidation evidence to
the historical doc-only `.1`/`.2` ROOT CAUSE bullets. Their original evidence is retained; current-leaf evidence stays in the
parent so its fresh boxes are the first ones the staged acceptance gate reads.

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

### `G1-SLICE.3c.4a.1` — semantic dart intent with visible closure obligations

- [x] **REPRODUCE / ISSUE** — ontology §4.3 and the reference skirt require semantic darts;
  `0c65d75` has no construction object. Intake declaration, geometric apex/legs and the executed
  closing operation must remain distinguishable so G1 cannot silently certify conservation.
- [x] **ROOT CAUSE (WHY + WHERE)** — a valid endpoint or declared intake is insufficient physical
  evidence. `cargo test -p sc-core --test dart_contract
  foreign_middle_of_merged_direction_is_refused_despite_owned_endpoints_after_reversal` → `1 passed`,
  `rc=0`: complete ledger coverage/live owned ends conceal a foreign middle, refused before/after
  reversal. `interior_apex_owned_legs_intake_origin_and_operation_identity_are_immutable_content`
  → `1 passed`, `rc=0`: an internal apex is valid structural intent with visible conservation deferral.
- [x] **FIX** — immutable Dart with intake origin, interior edge-anchored apex, two directed legs,
  explicit directed closing/folding reference and required closing-operation identity. Validate
  nonnegative authored intake, distinct held leg intervals, apex scope and all positive interval
  ownership/coverage/unique endpoints. Preserve raw topology evidence and operation/parameter
  registry obligations; expose G2 geometry and G2/G3 executed-conservation deferrals.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test dart_contract` → `9 passed`, `rc=0`;
  intake/symbolic values, opposite duplicate legs, each field's unknown/foreign scope, internal apex,
  split choice, foreign-middle merge/reversal, directed deletion evidence and immutable replacement
  discriminate. Disable ownership refusal → merged-middle test fails, `rc=101`; restored strict
  `make check` passes. The privacy doctest is green; queries never rewrite the original definition.
- [x] **NO REGRESSION** — `make check` → fmt/strict clippy/all Rust + privacy green, `rc=0`;
  `make wasm` → green; `make book` → warning-free; fixture → `0 mismatch(es)`;
  feature/glossary → `0 failure(s)`; tree census → `0 unowned / 0 orphan(s) / 0 dead link(s)`;
  ledger probes → `9 pass / 0 fail`; staged `make gate` → `=== all doctrines green ===`, all `rc=0`.
  Existing marks/allowance/sewing contracts remain green and numerical goldens do not change.
- [x] **LOCKSTEP** — construction family decomposed into owned semantic children before code;
  structural/geometric decision extended, Rust/subsystem status, bounded construction book companion
  + SUMMARY/ontology index and feature coverage, frontier/evidence/logs, TASK_TREE/MEMORY/LIVE_STATUS,
  CHANGELOG and promoted DEV_NOTES. Three of four object families remain done; next `.3c.4a.2`.

### `G1-SLICE.3c.4a.2a` — distinct tuck/pleat intent with shared structural checks

- [x] **REPRODUCE / ISSUE** — ontology §4.3 requires semantic tucks and pleats, not line art;
  `8753290` has only dart intent. A fold-reference list and a declared intake cannot certify a
  physical fold shape, type-specific closing operation or conservation proof.
- [x] **ROOT CAUSE (WHY + WHERE)** — current interval ownership matters independently of endpoints.
  `cargo test -p sc-core --test fold_contract
  foreign_middle_of_a_merged_fold_line_is_refused_despite_owned_endpoints_after_reversal` → `1 passed`,
  `rc=0`: both Tuck/Pleat refuse the owned-ends/foreign-middle merge before/after reversal.
  Shared structural inputs must not collapse the two semantic kinds into one untyped recipe operation.
- [x] **FIX** — separate immutable wrappers share validated fold content: intake provenance,
  nonempty directed ranges, explicit direction and required closing-operation identity. Reject negative
  explicit intake, empty/duplicate held intervals, unresolved endpoints and unowned current portions.
  Query authored line order + directed raw evidence; retain typed physical/registry obligations.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test fold_contract` → `9 passed`, `rc=0`;
  both types discriminate empty/duplicate opposite lines, explicit/symbolic intake, all reference roles,
  split choices, hidden foreign interiors, directed repairs, partial ranges and immutable replacement.
  Ownership refusal disabled → merged-middle regression fails, `rc=101`; restored strict check passes.
  Two privacy doctests are green; no physical fold-count/shape or executed-conservation claim.
- [x] **NO REGRESSION** — `make check` → fmt/strict clippy/all Rust + privacy green, `rc=0`;
  `make wasm` → green; `make book` → warning-free; fixture → `0 mismatch(es)`;
  feature/glossary → `0 failure(s)`; tree census → `0 unowned / 0 orphan(s) / 0 dead link(s)`;
  ledger probes → `9 pass / 0 fail`; staged `make gate` → `=== all doctrines green ===`, all `rc=0`.
  Earlier dart/mark/sewing contracts stay green; no numerical golden changes.
- [x] **LOCKSTEP** — `.3c.4a.2` decomposed before code; existing structural/geometric decision,
  Rust/subsystem status, bounded construction examples/API vocabulary/ontology index and feature row,
  task evidence/frontier/logs, TASK_TREE/MEMORY/LIVE_STATUS/CHANGELOG and promoted DEV_NOTES.
  Oldest changelog/dev-note entries seal to part18/part14. Next `.3c.4a.2b` gathers.

### `G1-SLICE.3c.4b.1` — served layers retain recipe and material intent with explicit lining scope

- [x] **REPRODUCE / ISSUE** — ontology §4.7 requires distinct served layers; feature-matrix rule 3
  separates modelled content from executable support. The prior commit has no layer descriptors.
- [x] **ROOT CAUSE (WHY + WHERE)** — a structurally valid Lining is still outside the v1 envelope.
  `cargo test -p sc-core --test layer_contract
  modelled_lining_refuses_v1_execution_with_named_diagnostic_piece_and_proving_gate` → `1 passed`,
  `rc=0`: inspection succeeds while execution returns env_lining, served Piece and G7.
  `foreign_middle_of_merged_offset_source_is_refused_despite_owned_ends_after_reversal` → `1 passed`,
  `rc=0`: live endpoints alone cannot establish whole-source ownership.
- [x] **FIX** — separate immutable Facing/Lining/Interfacing wrappers share owned-source and material
  validation. Offset operation/source intent stays authored; recipe owns dimensions and operation
  inputs. Explicit envelope checks distinguish inspection from execution; physical geometry is G2.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test layer_contract` → `9 passed`, `rc=0`:
  all kinds, materials, duplicates, wrong served Piece, split choices, hidden interiors, repair order
  and immutability. Three privacy doctests pass. Disabling lining scope and interval ownership
  separately makes each regression red, `rc=101`; restored checks pass. Moved fold checklist compares
  unchanged with `git show HEAD:docs/tasks/G1-SLICE.md`; existing Piece contracts remain green.
- [x] **NO REGRESSION** — `make check` → fmt/strict clippy/all Rust + privacy green; `make wasm` →
  green; `make book` → warning-free; fixture → `0 mismatch(es)`; feature/glossary → `0 failure(s)`;
  tree census → `0 unowned / 0 orphan(s) / 0 dead link(s)`; ledger → `9 pass / 0 fail`;
  staged `make gate` → `=== all doctrines green ===`, all `rc=0`. Fixture goldens stay unchanged.
- [x] **LOCKSTEP** — pre-code decision, `.6` envelope obligation, module/subsystem status, construction
  examples/API vocabulary, feature reasons, evidence relocation, frontier/logs/index and live memory
  are synchronized. The oldest interval lesson seals unchanged to devnotes-part16 with a verified
  digest. No geometric or release approval is implied. Next `.3c.4b.2` implements Hem.

### `G1-SLICE.3c.4a.2b` — gathers bind physical span sides and borrow canonical ease

- [x] **REPRODUCE / ISSUE** — feature matrix §5 defines gathering through seam ease distribution;
  `0cb97e4` has no Gather. An independent intake/allocation would duplicate the span's source, while
  binding only a span side would silently transfer gathering to a replacement physical copy.
- [x] **ROOT CAUSE (WHY + WHERE)** — span side and physical-copy identity are separate authored facts.
  `cargo test -p sc-core --test gather_contract
  wrong_graph_missing_span_and_changed_side_copy_are_refused_without_retargeting` → `1 passed`,
  `rc=0`: a changed A-side copy sharing the same Piece/range returns CopyBindingChanged.
  `uniform_weighted_and_between_notch_allocation_have_one_canonical_graph_source` → `1 passed`,
  `rc=0`: pointer equality proves each allocation view borrows the graph's existing declaration.
- [x] **FIX** — immutable graph/span/side/copy binding, explicit direction and closing operation;
  borrow attachment and signed intake/allocation from the canonical span. Validate explicit ease
  sign, copy owner and complete owned attachment/direction with unique endpoints. Target queries
  refuse missing/changed bindings; raw evidence and independent current-plan checks remain visible.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test gather_contract` → `11 passed`,
  `rc=0`: both gathered sides, symbolic sources, canonical allocations, changed/missing targets,
  copy reassignment, hidden merged interiors, split choices, repairs and immutable replacement.
  Copy-binding and ownership refusals disabled separately → their regressions fail, `rc=101`;
  restored `make check` passes. Privacy doctest green; three moved mark/dart checklists compare
  unchanged to committed originals. Four public intake structs re-derived from dart/fold/gather sources.
- [x] **NO REGRESSION** — `make check` → fmt/strict clippy/all Rust + privacy green, `rc=0`;
  `make wasm` → green; `make book` → warning-free; fixture → `0 mismatch(es)`;
  feature/glossary → `0 failure(s)`; tree census → `0 unowned / 0 orphan(s) / 0 dead link(s)`;
  ledger → `9 pass / 0 fail`; staged `make gate` → `=== all doctrines green ===`, all `rc=0`.
  Earlier sewing/intake/mark contracts stay green; no fixture golden changes.
- [x] **LOCKSTEP** — pre-code sewing decision extended, module/subsystem status, bounded construction
  examples/API vocabulary/ontology index and feature row, intake parents closed, evidence relocation,
  frontier/logs/index, MEMORY/LIVE_STATUS/CHANGELOG and promoted DEV_NOTES. Oldest lesson seals to
  devnotes-part15. Three of four object families remain done; next `.3c.4b` hem/layer descriptors.

### `G1-SLICE.3c.4b.2` — Hem retains depth/fold intent and validates current Facing targets

- [x] **REPRODUCE / ISSUE** — ontology §4.7 requires Hem depth, fold type, finish edge and turned/faced
  method; the prior commit has served layers but no Hem. Birth approval cannot certify later edits.
- [x] **ROOT CAUSE (WHY + WHERE)** — a Facing can lose interior source material while its original
  endpoints stay live. `cargo test -p sc-core --test hem_contract
  old_facing_is_revalidated_against_current_ledger_instead_of_reusing_birth_approval` → `1 passed`,
  `rc=0`: current queries and Hem birth both return InvalidFacing with the raw interior repair.
  `foreign_middle_of_finish_edge_is_refused_despite_owned_ends_after_reversal` → `1 passed`, `rc=0`:
  full live coverage still fails whole-source ownership, including after reversal.
- [x] **FIX** — immutable finish edge, explicit/symbolic depth, required logical fold binding and
  authored Turned/Faced method. Faced composition borrows the original current Facing after checking
  identity, served Piece and source validation. No duplicated layer data, solved values or defaults.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test hem_contract` → `10 passed`, `rc=0`:
  authored/symbolic depth and fold types, both methods, canonical target borrowing, missing/replaced/
  reassigned targets, stale interiors, foreign finish material, raw directions/repairs and immutability.
  Identity, current-layer and interval-ownership refusals disabled independently → each regression red,
  `rc=101`; restored strict checks and privacy pass. Gather/layer checklist relocation compares
  unchanged to `git show HEAD:docs/tasks/G1-SLICE.md`, with per-checklist staged revalidation.
- [x] **NO REGRESSION** — `make check` → fmt/strict clippy/all Rust + privacy green; `make wasm` →
  green; `make book` → warning-free; fixture → `0 mismatch(es)`; feature/glossary → `0 failure(s)`;
  tree census → `0 unowned / 0 orphan(s) / 0 dead link(s)`; ledger → `9 pass / 0 fail`;
  staged `make gate` → `=== all doctrines green ===`, all `rc=0`. Earlier layer/intake contracts pass.
- [x] **LOCKSTEP** — pre-code decision, Design composition/declaration obligations, module/map status,
  construction examples/API vocabulary, feature reason, `.4b` parent closure, frontier/logs/index and
  live docs align. Oldest entries seal unchanged to changelog-part19/devnotes-part17, with verified
  digests. Three of four
  object families remain done; physical folding remains G2/G3. Next `.3c.4c` closures.

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
