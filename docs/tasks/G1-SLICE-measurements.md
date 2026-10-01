# G1-SLICE — completed measurement/input contracts and evidence

Bounded semantic sibling of [G1-SLICE](G1-SLICE.md). Completed measurement-family children and
checklists retain their committed text unchanged; the current frontier stays in the parent.
Historical verification/commit tables remain in the parent and [evidence sibling](G1-SLICE-evidence.md).

## Completed child contracts

- ID: `G1-SLICE.4a.1`
  Status: `done`
  Goal: core immutable length-valued declaration with canonical authored state/source references:
  known with required evidence ids, assumed with assumption id, unknown with observation id,
  preference with provenance id, derived with formula id and no independently entered result.
  Acceptance: required state provenance is unrepresentable as absent; empty/duplicate known evidence
  refused; unknown/derived cannot produce a numeric fallback; explicit values retain Length units.
  Pre-code design: common declaration lives in sc-core, so sc-measure can depend on identity/value
  without core depending on the higher measurement crate. Formula inputs later read a core contract;
  .5/.6 validate declaration and dependency registries; G4 validates scoped evidence and artifact
  policy. Declared state is authored content, not independent proof of factual truth or exportability.
  Signed Length/explicit zero stay unchanged; per-measurement procedure/domain constraints are not
  invented here. No generic text-valued parameter API, cached formula output or global export gate.
  Record this boundary before code. Own bounded book vocabulary/examples and live/history/map updates.
  D64 owned now: ontology introduction still says other object types follow after .3c completed;
  fix intro and stale module-inventory tail; verify scope against committed .3c review.
  Impact: book/module status misreports implementation.
  Containment: relocate completed signoff/.13 checklists unchanged; partition completed construction
  child contracts/checklists into a bounded linked sibling if the existing evidence file approaches
  its ceiling. All committed payloads compare unchanged; current checklist remains first in parent.
  Verification: eight contracts + three privacy/state doc-tests; four independent actual red mutations;
  restored strict Rust 298 tests, WASM/book, focused censuses, ledger and staged doctrines green.
  Commit: `STITCHCAD-G1-0024`

- ID: `G1-SLICE.4a.2a`
  Status: `done`
  Goal: core immutable MachineToken, shared by measurement metadata and the future recipe namespace.
  Pre-code design: formalize the specified ASCII lower-snake syntax as [a-z][a-z0-9]* with optional
  underscore-separated nonempty alphanumeric segments. Reject the three grammar keywords let/assert/if;
  reserved built-in parameter names remain legal references, while metadata .2b and recipe .5 reject
  rebinding them. No Unicode normalization, automatic renaming, token→user-label conversion or text value.
  Acceptance: accepted tokens retain exact bytes/order; malformed ASCII/Unicode/separators/keywords
  refuse explicitly; immutable representation/private-field proof; no body/POM or state conversion.
  Own core API/tests, grammar/input-book vocabulary, existing length-input decision promotion and docs.
  Containment: move the completed .4a.1 contract/checklist unchanged to a linked measurement sibling
  before the parent exceeds its health target; retain the current checklist first.
  Verification: six contracts + private-field doc-test; four real regression mutations red;
  restored strict Rust 305 tests, WASM/book, focused censuses/ledger and staged doctrines green.
  Commit: `STITCHCAD-G1-0025`

- ID: `G1-SLICE.4a.2b`
  Status: `done`
  Goal: introduce sc-measure with immutable Measurement, landmark and documented-procedure records;
  validate current record/declaration registries and preserve entered unit plus canonical state/source.
  Pre-code design: metadata references two landmark identities (same identity allowed for a girth
  location), procedure identity and canonical LengthDeclaration. Landmark and procedure record kinds
  must match body/garment; no source standard vocabulary/procedure content is invented. Procedure
  documentation is required nonblank content on the canonical procedure record; metadata carries only
  the procedure id. This proves documented content exists, without inferring physical repeatability or
  source truth (Design/G4). Names must be nonblank;
  Tokens use the shared core type, and reserved formula inputs cannot be rebound. Context duplicates refuse
  before lookup across all three record inventories, never pick the first. Measurement identity
  cannot collide with a supplied context record. Global Design identity/source registries remain .6. Current same-id replacements are inspected; old input stays
  immutable. No subject/domain constraints are fabricated from the standards not read in-repo.
  Own new crate/Cargo.lock, local and CI WASM integration, README standard-command/status scope and book.
  D66 owned here: README reports only G0 work and the workspace header asks to retire the removed
  starter; both misreport delivered foundations.
  Repair status/header while preserving G0 closure-unapproved and no-application facts.
  Acceptance: all metadata fields/canonical queries and typed missing/foreign/ambiguous refusals tested;
  strict Rust/WASM/book/focused and full milestone gates pass. CI verdict remains .2c's obligation.
  Verification: sixteen contracts + three privacy docs; six actual guard mutations red; restored
  strict Rust 325 tests, three-crate WASM/book, full 22 probe suites and staged doctrines green.
  Exceptional push/observed CI belongs to .4a.2c; no remote success is claimed here.
  Commit: `STITCHCAD-G1-0026`

  API before code: MeasurementContext borrows declarations/landmarks/procedures with typed identity
  maps. Landmark and MeasurementProcedure use private validated definitions (nonblank names and
  procedure documentation); kinds are explicit Body/Garment. Measurement holds entered Unit, token,
  kind, two landmark ids, procedure and declaration id. Current context queries borrow canonical
  targets, reject removed/mismatched/ambiguous targets, and preserve authored metadata/state. Target
  queries prove only the named target; validate_current checks the whole measurement. Shared core
  token exposes reserved-input classification for metadata and later recipe binding.

## Completed acceptance checklists

### `G1-SLICE.4a.1` — one canonical length state, no numeric unknown default

- [x] **REPRODUCE / ISSUE** — ontology §2.1/§5 requires authored measurement states/provenance;
  no executable shared declaration existed. D64 left the introduction and module inventory behind
  the committed four-family review. D65's 59/64 history census is owned for its archive trigger.
- [x] **ROOT CAUSE (WHY + WHERE)** — `cargo test -p sc-core --test value_contract
  all_five_states_retain_one_canonical_source_and_their_distinct_required_provenance` → `1 passed`,
  `rc=0`: metadata needs a single lower-layer value source to avoid copied state/dependency cycles.
  Known ids prove only authored inventory, not source truth; observation/evaluation require different
  records. `git show HEAD:docs/book/src/spec/ontology-review.md` confirms completed structural scope.
- [x] **FIX** — immutable core length declaration carries source plus one of five required-provenance
  states. Empty/duplicate known evidence refuses; unknown/derived carry no numeric field or fallback.
  Signed/zero authored input stays exact; Design/recipe/G4 own source, domain and scoped truth checks.
- [x] **ADDRESSED (verified)** — value suite `8 passed`, `rc=0`; three compile-fail examples prove
  immutable fields and absent unknown/derived numeric input. Disabling empty/duplicate evidence guards
  and returning zero for unknown/derived each makes the intended assertion fail, independently,
  `rc=101`; restored full checks pass. Source/state/value replacement preserves the earlier object.
- [x] **NO REGRESSION** — `make check` → `298` tests, strict lint/fmt green; WASM/book warning-free;
  fixture/feature/glossary/tree censuses green; ledger `9 pass / 0 fail`; staged `make gate` →
  `=== all doctrines green ===`, all `rc=0`. Committed-payload oracle proves complete unchanged
  construction contracts and ten checklists; partitioned sibling roots revalidate in the staged gate.
- [x] **LOCKSTEP** — core/value/tests, bounded input chapter/vocabulary, ontology status, decision/map,
  live/resume/index, task graph and histories align. D64 seals to defects-part10; two oldest lessons
  seal unchanged to devnotes-part23. G1 remains 5/18; .4a.2 is next. D65 remains open under SPINE.19.2.

### `G1-SLICE.4a.2a` — machine identifiers preserve exact spelling

- [x] **REPRODUCE / ISSUE** — metadata/formula names share the specified ASCII lower-snake rule;
  constructing independent string validators would permit spelling drift across the common API.
- [x] **ROOT CAUSE (WHY + WHERE)** — `cargo test -p sc-core --test name_contract` → `6 passed`,
  `rc=0`: stable machine spelling needs one lower-layer contract distinct from display labels and
  formula binding authority. Grammar §1 keywords and contract §3.1 reserved inputs have different roles.
- [x] **FIX** — immutable MachineToken retains exact bytes or typed InvalidSyntax/ReservedKeyword;
  formal shared grammar permits lowercase-start alphanumeric segments separated by single underscores.
  No normalization, inferred display name, text scalar or reserved-input binding authority.
- [x] **ADDRESSED (verified)** — six contracts cover spelling, Unicode lookalikes, whitespace,
  separators, digits, keywords, reserved references, exact collection keys and immutable replacement;
  private-field compile-fail passes. Disabling start, empty-segment or keyword guards, or accepting
  uppercase internal letters, each makes its intended regression fail `rc=101`; restored suite green.
- [x] **NO REGRESSION** — `make check` → `305` tests, strict fmt/lint green; WASM/book green;
  focused censuses and ledger `9 pass / 0 fail`; staged `make gate` → `=== all doctrines green ===`,
  `rc=0`. Completed .4a.1 contract/checklist compare byte-identical against committed predecessor.
- [x] **LOCKSTEP** — core API/tests and input chapter/grammar/vocabulary match the promoted decision;
  live/resume/tree/map/history align. .4a.2b introduces metadata/runtime integration, then .2c observes
  CI before parent closure. D66 is owned there; D65 archive transition keeps its actual seal trigger.

### `G1-SLICE.4a.2b` — metadata references canonical documented records

- [x] **REPRODUCE / ISSUE** — ontology §2.1 requires body/POM distinction, entered unit, landmark and
  documented-procedure references. Only shared core values/tokens existed; README/workspace status
  still described earlier bootstrap work (D66). Current metadata belongs to a higher measurement crate.
- [x] **ROOT CAUSE (WHY + WHERE)** — `cargo test -p sc-measure --test measurement_contract` →
  `16 passed`, `rc=0`: metadata needs typed canonical record references, not copied values/state or
  per-entry procedure prose. A body record cannot supply a garment POM. `cargo metadata --no-deps
  --format-version 1` → sc-core/sc-measure/sc-units with path-only dependencies, `rc=0`.
- [x] **FIX** — immutable Measurement/Landmark/MeasurementProcedure retain required fields; canonical
  documentation must be nonblank. Borrowed context rejects repeated identity before lookup; constructor
  validates body/garment targets, identity ownership and reserved binding. Target queries validate their
  own references; validate_current covers all metadata. Source/physical truth retains its later owners.
- [x] **ADDRESSED (verified)** — sixteen contracts and three private-field docs pass. Disabling
  documentation, landmark/procedure kind, reserved binding, context duplication or measurement-identity
  guards each fails the intended regression, independently `rc=101`; restored strict checks pass.
  Same-id source/state/document changes are read as current content; old metadata remains unchanged.
- [x] **NO REGRESSION** — `make check` → `325` tests, strict fmt/lint green; `make wasm` builds all
  three crates; book warning-free; focused censuses green; ledger `9 pass / 0 fail`; `make probes` →
  `22 suite(s)` green; staged `make gate` → `=== all doctrines green ===`, `rc=0`.
  Completed .4a.2a contract/checklist compare byte-identical to the committed predecessor.
- [x] **LOCKSTEP** — new crate/lockfile, shared name classification, local+CI WASM commands, README,
  book/spec/decision/map, live/resume/index and task/history align. D66 closes in defects-part11;
  oldest two lessons seal unchanged in devnotes-part25. G1 stays 5/18; .2c owns observed CI. D67
  promotion freshness is owned by SPINE.22; current decision gains explicit token/metadata questions.

- ID: `G1-SLICE.4a.2c`
  Status: `done`
  Goal: observe exceptional runtime/CI integration push at job/step level; independently sign off
  measurement metadata against ontology, close .4a.2 and hand to table .4a.3.
  Acceptance: CI rust/doctrine jobs completed success, exact revision recorded; all metadata fields
  accounted for without claiming source truth, procedure repeatability or release approval.
  Verification: GitHub jobs 110566989457 (check) / 110566988221 (enforce), all steps
  completed success at bf29b031033ad7ce198db0a02a5c1d205d5594aa; field review and local book/gates green.
  Commit: `STITCHCAD-G1-0027`

### `G1-SLICE.4a.2c` — observed CI and scoped metadata review

- [x] **REPRODUCE / ISSUE** — .2b added a runtime crate and changed CI; local success could not
  satisfy the COMMIT.md exceptional-push/runner obligation. .2c also owns metadata-family review.
- [x] **ROOT CAUSE (WHY + WHERE)** — `gh api repos/rdje/stitchcad/actions/runs/36921077740/jobs`
  and `/36921077711/jobs` → jobs check/enforce completed/success, all steps completed/success,
  `rc=0`; runs match bf29b031033ad7ce198db0a02a5c1d205d5594aa. Explicit job evidence closes the
  runner gap; a run-summary-only inference is not used.
- [x] **FIX** — clean push 3d9f2be..bf29b03 main→main; record exact revision/run/job/step evidence.
  Review maps name/token/unit/kind/landmark/procedure/source/state to canonical immutable APIs and
  current-reference tests. Documentation presence remains distinct from physical/source truth.
- [x] **ADDRESSED (verified)** — rust run 36921077740/job 110566989457: fmt, strict Clippy,
  all tests and three-crate WASM success. Doctrine run 36921077711/job 110566988221: enforcer success.
  Sixteen metadata contracts and three privacy docs already passed locally with six actual red guards.
- [x] **NO REGRESSION** — no code change in this review; book/focused tree and glossary censuses,
  ledger `9 pass / 0 fail` and staged `make gate` → `=== all doctrines green ===`, all `rc=0`.
  Completed .2b contract/checklist compare unchanged to bf29b03. Pre-push strict 325 tests and all
  22 probe suites were green; both runner jobs independently agree.
- [x] **LOCKSTEP** — .4a.2 closes; .4a.3 table remains next and G1 stays 5/18. Book records scoped
  runtime proof; live/resume/index, task evidence and logs match. D65 retention and D67 promotion
  remain owned; no additional seal was needed for this review.

## Named table contract and evidence — preserved from f19982d

- ID: `G1-SLICE.4a.3`
  Status: `done`
  Goal: MeasurementTable stable identity/name and unique measurement id/token inventory with current
  metadata queries; complete measurement-family signoff against ontology .2.1 and book contracts.
  Acceptance: ambiguity/missing targets/current reassignment refused; immutable input; focused and
  milestone gates prove all measurement content without certifying source truth or release readiness.
  Pre-code contract: TableDefinition holds id/name and authored ordered MeasurementBinding entries.
  Each binding pins measurement id, exact token, body/POM kind and canonical declaration id; captures
  may derive from existing immutable metadata. No values/state/source/procedure prose copied into tables.
  Empty draft tables legal (no normative nonempty rule); names nonblank, binding ids/tokens unique.
  TableContext borrows current Measurement inventory plus existing MeasurementContext; rejects duplicate
  metadata ids and cross-kind target identity collisions before lookup. Token uniqueness is table-scoped,
  not imposed over unrelated tables' canonical inventory. Table id cannot reuse a supplied record id.
  Current lookup first selects saved binding by exact token or id, then resolves its measurement id;
  removed ids never select peers by token. Kind/token/declaration reassignment returns typed expected/
  actual evidence; same-id canonical declaration state/source changes remain visible. Display labels,
  entered units and procedure/landmark metadata are current canonical content, with all target checks
  retained. Target query validates selected binding/current metadata; validate_current checks all entries.
  Explicit validated table replacement may rebind and leaves original unchanged. Immutable fields/private
  representation; errors implement Display/Error with structural source. No global Design revision,
  source/evidence truth, formula/profile evaluation, physical repeatability or release proof claimed.
  Own runtime metadata/book/API vocabulary, independent real guard mutations, field-by-field .4a
  structural signoff, focused checks and milestone checks. Previously closed metadata signoff records
  relocate unchanged to the measurements sibling; normal live rollovers if health requires them.
  Verification: 16 contracts + privacy, eight real red guard assertions, restored 342 strict tests;
  three-crate WASM/book/glossary/formula green. Milestone full probes and staged gate recorded
  below. Canonical field review closes .4a structurally; .4b per-POM Ease follows.
  Commit: `STITCHCAD-G1-0028`

### `G1-SLICE.4a.3` — named tables with stable current bindings

- [x] **REPRODUCE / ISSUE** — `rg 'MeasurementTable' crates/sc-measure/src` before implementation
  → only deferred documentation, no executable table. Ontology .2.1 requires a named set and stable
  scalars; standalone metadata alone could not bind a unique table namespace or reject reassignment.
- [x] **ROOT CAUSE (WHY + WHERE)** — code-path review and `cargo test -p sc-measure --test
  measurement_contract` → `16 passed`, rc=0: canonical targets/state already exist, but named-table
  binding ownership was missing. Recipe has one flat namespace; table uniqueness must be scoped to
  its authored entries, resolving current metadata by id rather than selecting a same-token peer.
- [x] **FIX** — immutable named table and ordered id/token/kind/declaration bindings; borrowed current
  context rejects duplicate/cross-kind identities. Queries resolve saved id and compare expected
  token/domain/scalar identity, then validate all selected metadata targets. No scalar/source/state
  cache. Full validation checks every entry; explicit validated rebinding preserves prior objects.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-measure --test table_contract` → `16 passed`,
  rc=0, plus private-field doc. Removal with surviving peer, metadata/scalar reassignment, unknown/
  derived, current source/document/unit edits, duplicates/collisions, shared declarations and reorder
  covered. `bash docs/tasks/artifacts/measurement_table/run_table_mutations.sh` → eight real guard
  reds, each rc=101/actual regression assertion; original source restored byte-identically.
- [x] **NO REGRESSION** — restored `make check` → 342 tests with strict fmt/clippy, rc=0; three-crate
  `make wasm` and warning-free `make book`, rc=0. Glossary → 310 terms/9 parts/0 failures; formula
  → 17 bindings/4 assertions/13 refusals/0 mismatches, rc=0. Full milestone `make probes` →
  `23 suite(s) green`, rc=0; staged `make gate` → `=== all doctrines green ===`, rc=0.
  Final isolated-interpreter mutation run repeats eight assertion reds, restores source exactly;
  restored table contracts again `16 passed`, rc=0.
- [x] **LOCKSTEP** — table/code/API vocabulary, package/README/book and fresh decision questions agree;
  metadata CI review/checklist relocates unchanged to measurement sibling. Changelog-part25 and
  devnotes-part27 seal unchanged predecessor content. .4a closes structurally; .4 remains active
  for Ease/SizeSet/signoff. G1 stays 5/18, four structural families, nine open/58 sealed defects.
  Current-revision source/evidence truth, physical repeatability and release certification deferred.

Return to the [active frontier](G1-SLICE.md#current-frontier).
