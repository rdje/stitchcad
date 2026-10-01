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

