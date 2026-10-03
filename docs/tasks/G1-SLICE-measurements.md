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

## Individual Ease contract and evidence — preserved from 0088969

- ID: `G1-SLICE.4b.1`
  Status: `done`
  Goal: immutable individual Ease mappings over saved body/POM bindings and one canonical signed
  LengthDeclaration; FitIntent ordered Close < Semi < Loose, explicit compression provenance.
  Pre-code protocol: retain measurement id/token/kind/scalar expectations, resolve current metadata
  by id, require Body then Garment and all current landmark/procedure targets. Reject mapping-id
  collisions and amount-id aliasing either measurement scalar. Reuse the borrowed unambiguous
  inventory; matching tokens across distinct measurement namespaces remain legal here.
  The canonical amount owns numeric state/source; mapping provenance owns fit/correspondence intent.
  Compression is Forbidden or Declared with a provenance id, never inferred from fit class. Accept
  unknown/derived drafts without numeric fallback; enforce permission on every present/current or
  externally evaluated signed amount. No body-plus-ease computation, fitted thresholds, source truth,
  v1-envelope expansion or physical/release certificate. Explicit replacements preserve old mappings.
  Acceptance: current reassignment/removal/invalid metadata refuse; negative known/assumed/preference
  without declaration refuses; declared negatives/zero/positives preserved; unresolved states remain
  queryable and numeric queries refuse. Typed errors retain mapping/side/target and underlying cause.
  Verification: 13 contracts + privacy, seven real guard reds, strict 356 tests/WASM/book; current
  binding/source/uncertainty and compression permission verified locally.
  Commit: `STITCHCAD-G1-0029`

### `G1-SLICE.4b.1` — individual canonical Ease intent

- [x] **REPRODUCE / ISSUE** — `rg 'Ease' crates/sc-measure/src` at f19982d → deferred documentation
  only; ontology .2.2 requires a body-to-POM mapping, signed amount, fit, state and provenance.
- [x] **ROOT CAUSE (WHY + WHERE)** — `cargo test -p sc-measure --test table_contract` predecessor
  → 16 passed, rc=0: current canonical measurements exist, but cannot alone express correspondence,
  fit or permission. New ease module borrows existing context; no numeric/state cache is needed.
- [x] **FIX** — immutable mapping, saved four-field side bindings, distinct canonical amount,
  ordered FitIntent and explicit compression/provenance. Current queries refuse reassignment, missing
  metadata and undeclared negative values; unknown/derived remain inspectable without numeric values.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-measure --test ease_contract` → 13 passed, rc=0;
  private-field doc passes. `bash docs/tasks/artifacts/ease/run_ease_mutations.sh` → seven real guard
  assertion reds, rc=101 each; source restored byte-identically. Permission tested in all three numeric
  states and externally supplied derived results; current edits/reorder/namespace/metadata covered.
- [x] **NO REGRESSION** — `make check` → 356 tests, strict fmt/clippy, rc=0; three-crate `make wasm`
  and warning-free `make book`, rc=0. Glossary → 310 terms/9 parts/158 tokens/0 failures, rc=0.
  Final mutation run restores exactly; tree census → 10 lanes/13 trees/6 siblings/0 gaps, rc=0;
  ledger probes → 9 pass/0 fail, rc=0; staged `make gate` → all doctrines green, rc=0.
- [x] **LOCKSTEP** — book fields/examples/API, canonical decision, live pointers and logs agree;
  completed table contract/checklist relocates unchanged against f19982d. Routine rolling records
  retained unchanged. Individual intent only; set/table membership .4b.2 and family review .4b.3 remain.
  G1 stays 5/18, four completed ontology families; physical fit/source/evaluation/release proof owned.

Return to the [active frontier](G1-SLICE.md#current-frontier).

## Per-POM set contract and evidence — preserved from b4e0bc7

- ID: `G1-SLICE.4b.2`
  Status: `done`
  Goal: ordered immutable Ease set with unique mapping identity/token/POM and per-POM lookup;
  explicit table-membership/current binding contract and current inventory validation.
  Pre-code protocol: EaseSetDefinition has stable id, body/garment table ids (one mixed table legal),
  and authored EaseBinding entries: set-scoped token, Ease id, saved body/POM four-field bindings and
  amount id. Namespaces are unique by token, mapping id and POM id; shared body/amount sources legal.
  EaseSetContext borrows canonical tables/Ease records and existing MeasurementTableContext; reject
  duplicate and cross-kind record identities before lookup. Set identity cannot alias supplied records.
  Queries select saved id/POM/token, resolve current Ease by id, compare body/POM/amount expectations,
  require both measurements in their named current tables and validate current Ease. Current fit,
  provenance/compression and canonical amount/state/source edits visible; retarget requires explicit
  replacement. Empty drafts legal with existing table references; unselected bad entries do not block
  selected lookup, full validation checks all mappings. No fabricated coverage, arithmetic or release
  proof. Own package-status defect D69 repair, existing D34 stale census correction and normal seals.
  Acceptance: unique per-POM mappings, shared body sources legal, current ambiguity/missing members
  refused, lookup order independent, no default mapping or numeric fallback.
  Verification: 14 contracts + privacy, ten real guard assertion reds; strict 371 tests/WASM/book;
  current canonical bindings, selected membership and namespace checks pass locally.
  Commit: `STITCHCAD-G1-0030`

### `G1-SLICE.4b.2` — unique current per-POM sets

- [x] **REPRODUCE / ISSUE** — `rg 'EaseSet' crates/sc-measure/src` at 0088969 → no executable set;
  ontology .2.2/.3.1 and instantiation-paths .2 require a per-POM mapping namespace and table ownership.
- [x] **ROOT CAUSE (WHY + WHERE)** — `cargo test -p sc-measure --test ease_contract` predecessor
  → 13 passed, rc=0: individual current mappings exist, but do not bind selected tables or unique POMs.
  New set module borrows canonical mappings/tables and existing current metadata context.
- [x] **FIX** — immutable ordered mapping-id/token/POM-unique namespace; saved target bindings,
  explicit body/garment table ids, no copied fit/value/state/source. Current queries resolve saved
  mapping id, refuse target reassignment, require both table memberships and validate current Ease.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-measure --test ease_set_contract` → 14 passed,
  rc=0, plus privacy doc. `bash docs/tasks/artifacts/ease/run_ease_set_mutations.sh` → ten real guard
  assertion reds, rc=101 each, source restored byte-identically. Duplicate POM uses two valid canonical
  mappings to the same POM; removing only its uniqueness guard accepts that genuinely ambiguous set.
- [x] **NO REGRESSION** — `make check` → 371 tests with strict fmt/clippy, rc=0; three-crate WASM
  and warning-free book, rc=0. Glossary → 310 terms/9 parts/158 tokens/0 failures, rc=0.
  Tree census → 10 lanes/13 trees/6 siblings/0 gaps, rc=0; ledger → 9 pass/0 fail, rc=0;
  staged `make gate` → all doctrines green, rc=0.
- [x] **LOCKSTEP** — code/book/API/canonical decision and live pointers agree; individual contract/
  checklist retained unchanged against 0088969. Package metadata verifies D69 repaired; corrected
  existing D34's stale sibling example, with its mechanical ownership retained. Normal history seals
  preserve exact predecessor records. Set intent implemented; .4b.3 structural review next, then SizeSet.

Return to the [active frontier](G1-SLICE.md#current-frontier).

## Structural Ease review — preserved from b6bd985

### `G1-SLICE.4b.3` — structural Ease family review

Review scope: ontology .2.2, .3.1, instantiation-paths .2 and ease-inputs implementation guide.
`rg` over EaseDefinition/EaseSetDefinition and both contract files maps every required field to its
canonical representation. The book's structural review table names current checks and deferred proofs.
No product code changes; strict 371-test/WASM evidence at b4e0bc7 is for unchanged current code.
Fresh `cargo test -p sc-measure` → 65 tests/docs, rc=0; `make probes` → 23 suites green, rc=0;
warning-free `make book`, rc=0. All .2.2 structural fields covered; .4b closes, .4c/.4d remain.
promotion: declined (routine milestone review; canonical decisions unchanged, results in book/task).
Completed set contract/checklist is retained byte-identically against b4e0bc7 in the measurement sibling.

Return to the [active frontier](G1-SLICE.md#current-frontier).


## Size membership contract and evidence — preserved from 0b77235

- ID: `G1-SLICE.4c.1`
  Status: `done`
  Goal: immutable membership foundation: SizeSetReference(id, Count revision), SizeSystem, validated
  human SizeLabel, stable SizeMember ids, authored ordered members and exactly one base member.
  Pre-code protocol: SizeMembershipDefinition has reference/system/member list/base id. It contains
  no chart, breaks, quantities, axes or physical defaults; .4c.2/.3/.4 supply those distinct contracts.
  Reject empty members, repeated member/set ids, exact duplicate labels and absent base; nonblank
  labels preserve exact Unicode/spacing/case and carry no machine-token or numerical semantics.
  Id/label queries are order-independent; inventory order is never sorted. Revision successor retains
  identity and checks Count overflow, without pretending to enforce command-registry currentness.
  Acceptance: custom single member/base supported, author order preserved, blank labels/duplicate ids/
  labels/missing base refused, same labels in different sets carry distinct identity; immutable/private
  content, structured diagnostics and overflow refusal. Canonical charts/axes/evidence remain deferred.
  Verification: 12 contracts + three privacy/quantity docs, seven real assertion reds; restored
  strict 386 tests/book/glossary; WASM and staged gate below. D70 ruling remains pending for axes.
  Commit: `STITCHCAD-G1-0032`

### `G1-SLICE.4c.1` — size membership without inferred order or measurements

- [x] **REPRODUCE / ISSUE** — `git grep -n -E 'SizeMembership|SizeSetReference' b6bd985 --
  crates/sc-measure/src` → 0 matches, expected rc=1: no executable membership; size-sets .2–.4/.11 requires identity/revision, human labels,
  authored order and exactly one existing base, with no quantity or label-derived measurements.
- [x] **ROOT CAUSE (WHY + WHERE)** — `rg -n 'axes|single axis|two representations'
  docs/book/src/spec/size-sets.md` → conflicting field row/one-axis rule, rc=0; sc-units ratio.rs
  inspection → Count(u32), checked successor required. Membership separates identity/label/position.
  D70 source census pins contradictory optional/multidimensional-only versus single-axis requirements;
  axes representation is not needed by this independently owned membership foundation.
- [x] **FIX** — private nonblank exact SizeLabel and immutable SizeMembership; stable member ids,
  explicit system, pinned reference/revision and base identity. Refuse empty/duplicate members/labels,
  missing base/lookups and overflow. Never sort, normalize, parse a measurement or invent quantities.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-measure --test size_membership_contract` →
  12 passed, rc=0; three privacy/quantity docs pass. `bash docs/tasks/artifacts/size_membership/
  run_size_membership_mutations.sh` → seven real assertion reds, rc=101 each; source restored exactly.
  Authored sort mutation and wrapped revision fallback fail real regressions, not compilation.
- [x] **NO REGRESSION** — `make check` → 386 tests, strict fmt/clippy, rc=0; three-crate `make wasm`
  and warning-free book, rc=0. Glossary → 310 terms/9 parts/158 tokens/0 failures, rc=0;
  tree census → 10 lanes/13 trees/6 siblings/0 gaps, rc=0; ledger → 9 pass/0 fail, rc=0.
  Final staged `make gate` → all doctrines green, rc=0. Uncertainty census → 133 markers/16 files/
  0 unowned/0 failures, rc=0. No implicit axis, chart measurement or default quantity.
- [x] **LOCKSTEP** — API/book/partial SizeSet status, canonical ownership record, live pointers and
  logs agree. Completed Ease review retains exact predecessor text; rolling records seal unchanged.
  D70 logged/owned for .4c.2 with director question; .4c.3/.4/.5/.4d preserve all remaining scope.
  Membership is not a complete SizeSet, MTM-ready chart, current-registry certificate or release proof.


## Garment chart observation contract and evidence — preserved from 71aaf19

- ID: `G1-SLICE.4c.3a`
  Status: `done`
  Goal: immutable garment-chart observation for one member and design POM, with current member/set
  revision, logical POM and measured-input bindings, named table memberships and mapping provenance.
  Pre-code protocol: SizeChartObservationDefinition holds id, pinned membership reference/member id,
  design/chart table ids, saved POM/measurement four-field bindings and correspondence provenance id.
  Context borrows membership/current tables/MeasurementTableContext, rejecting duplicate/cross-kind
  identities (including member/set identities). Both measurements must be Garment; Body/MTM inputs
  belong to .3c and are never silently interchanged. Current queries pin set id/revision, resolve member
  id, table ids and measured ids, reject token/kind/scalar retargeting and validate required metadata.
  Value/state/source/entered unit borrowed from the canonical measurement, no numeric/state cache.
  Same input may also be a Design-table POM (base-size authored data); no duplicated scalar imposed.
  This is an authored chart-to-POM correspondence, not a regenerated geometry measurement or proof
  of physical quantity equivalence. G3/G4 validate that correspondence/source/evidence; later generated
  results remain distinct measurements. No axis, default, inferred label, completeness or path-ready claim.
  Acceptance: missing/revised member, tables, bindings and metadata refuse with scoped errors; current
  scalar/metadata edits visible; unknown/derived numeric queries refuse; selected targets borrowed;
  immutable replacement leaves originals unchanged. Own book/API/live docs, evidence retention/seals.
  Verification: 16 contracts/privacy, eight actual assertion reds, restored strict 403 tests/WASM/book;
  glossary/uncertainty/tree/ledger and staged doctrine gate green. Completed membership preserved exactly.
  Commit: `STITCHCAD-G1-0033`

### `G1-SLICE.4c.3a` — current authored garment chart observations

- [x] **REPRODUCE / ISSUE** — `git grep -n -E 'SizeChartObservation|SizeChartContext'
  0b77235 -- crates/sc-measure/src` → 0 matches, expected rc=1; size-sets §5 needs canonical
  per-member/POM chart inputs distinct from later regenerated geometry measurements.
- [x] **ROOT CAUSE (WHY + WHERE)** — `rg -n 'chart POM|regenerated POM|state|chart'
  docs/book/src/spec/size-sets.md` → authored chart/source requirements at §5, rc=0; existing tables
  validate metadata but carry no set/member/revision correspondence. Design inputs name a logical
  quantity; they are not already measured regeneration results. Base input sharing needs no copy.
- [x] **FIX** — immutable observation pins member/set reference, two named table identities and
  saved garment bindings. Borrow current metadata/state/source; refuse aliases, missing/revised
  membership, absent tables, invalid targets and retargeting. Mapping provenance is not physical proof.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-measure --test size_chart_contract` → 16 passed,
  rc=0; privacy doctest passes. `bash docs/tasks/artifacts/size_chart/run_size_chart_mutations.sh`
  → eight real assertion reds, each rc=101, exact source restore, rc=0. Same-label/member and
  same-content/table substitutions, wrong domains and explicitly rebound tables cannot transfer intent.
- [x] **NO REGRESSION** — `make check` → strict fmt/clippy and 403 tests, rc=0; `make wasm` →
  three crates cross-build, rc=0; `make book` → no warnings, rc=0. Glossary → 310 terms/9 parts/
  158 tokens/0 failures; uncertainty → 133 markers/16 files/0 unowned/0 failures, both rc=0.
  Tree census → 10 lanes/13 trees/6 siblings/0 gaps; ledger → 9 pass/0 fail; staged `make gate`
  → all doctrines green, rc=0. New product code has local integration proof, not observed remote CI.
- [x] **LOCKSTEP** — book/API/README/package status, canonical ownership, live pointers and logs
  updated together. Full .4c.1 contract/checklist retains exact HEAD bytes in linked sibling;
  oldest changelog/lesson seals preserve predecessor bytes. Chart collection, MTM, breaks/composite,
  profile resolution and later physical/evidence/release validation remain owned; D70 not defaulted.


## Structural Ease review contract — preserved from 71aaf19

- ID: `G1-SLICE.4b.3`
  Status: `done`
  Goal: re-derive ontology .2.2 field coverage, reference/currentness and compression boundaries;
  review book and milestone checks before closing .4b structurally.
  Review protocol: map each ontology .2.2 field to current immutable API and test; review per-POM
  set/current tables, no numeric fallback and negative permission including evaluated-result boundary.
  Run current sc-measure contracts/docs, full 23-suite milestone probes and warning-free book; staged
  doctrines. Existing 371-test strict/WASM evidence is for unchanged b4e0bc7 code, no remote CI claim.
  Own bounded review docs, preserve completed set contract/checklist unchanged, update pointers/logs
  and normal byte-identical rolling seals. Close .4b structurally only after all owned checks pass.

  Verification: 65 current sc-measure tests/docs, full 23-suite milestone, warning-free book,
  current code unchanged from b4e0bc7 strict 371/WASM proof; staged gate below.
  Commit: `STITCHCAD-G1-0031`

Return to the [active frontier](G1-SLICE.md#current-frontier).

## Garment chart collection evidence — preserved from e299771

- ID: `G1-SLICE.4c.3b`
  Status: `done`
  Goal: canonical per-member/POM chart collection with unique observations and declared coverage,
  current correspondence/table validation, explicit completeness errors and ordered-member lookup.
  Pre-code protocol: SizeChartDefinition pins id, membership reference, Design table, ordered target
  POM bindings and ordered SizeChartBinding entries. Bindings capture observation/reference/member/
  table/measurement targets, never values/state/provenance copies. Context borrows canonical observations
  and SizeChartContext; refuse duplicate/cross-kind ids. Current queries compare saved targets and
  borrow the selected current observation, source/state and provenance; no peer or label substitution.
  Construction/current validation permits incomplete drafts but checks every authored reference.
  Completeness explicitly requires a nonempty target inventory equal to every current garment POM
  in the Design table, plus exactly one observation for every member/POM cell. Validate the full named
  Design table for this claim; omitted POMs cannot certify a deliberately narrowed chart as complete.
  Refuse duplicate observation/POM/token/cell, undeclared POMs, wrong domains, stale refs or Design-table
  mismatches. Selected member row returns observations in declared POM order; members retain membership
  order. Selected queries do not certify unrelated rows. Unknown/derived values do not break structural
  coverage but numeric queries refuse. Shared canonical measurement inputs need distinct member-pinned
  observations/provenance; sharing one canonical observation across chart collections is explicit.
  No interpolation, measurements inferred from labels, axis model, path readiness or physical proof.
  Acceptance: draft/full coverage distinction, current targets/provenance, exact missing member/POM
  errors, borrowed values, immutable replacement, shared inputs and identity/domain guards. Own book/
  API/live docs, exact completed evidence relocation and normal history seals; real production mutations.
  Verification: 18 contracts/privacy, 14 actual assertion reds; exact restore, strict 422 tests/WASM/
  book, glossary/uncertainty/tree/feature/ledger and staged doctrines green. Draft/full boundaries explicit.
  Commit: `STITCHCAD-G1-0034`

### `G1-SLICE.4c.3b` — exact current Design/member/POM chart coverage

- [x] **REPRODUCE / ISSUE** — `git grep -n -E 'pub struct SizeChartDefinition|pub struct
  SizeChartCollectionContext' 71aaf19 -- crates/sc-measure/src` → 0 matches, expected rc=1;
  individual observations alone cannot establish size-sets §5's per-member Design-POM chart coverage.
- [x] **ROOT CAUSE (WHY + WHERE)** — `rg -n 'Per member|same POMs|chart POM'
  docs/book/src/spec/size-sets.md` → §5 current quantity/measurement correspondence requirements,
  rc=0; authored membership and observation targets supply identities, not collection completeness.
  Completeness must use the current Design table, not a self-declared reduced target subset.
- [x] **FIX** — immutable chart and saved canonical observation targets; unique identities/POMs/
  tokens/cells, exact membership/table/binding checks, borrowed current source/state/provenance.
  Draft validation remains distinct from nonempty exact Design-POM/member coverage. Ordered row
  queries refuse missing cells, and shared measurement inputs require explicit member observations.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-measure --test size_chart_collection_contract`
  → 18 passed, rc=0; privacy doctest passes. `bash docs/tasks/artifacts/size_chart_collection/run_size_chart_collection_mutations.sh` → fourteen actual assertion reds, rc=101 each, exact
  source restore, rc=0. Reduced targets, valid duplicate cells, stale bindings and zero fallback fail.
- [x] **NO REGRESSION** — `make check` → strict fmt/clippy and 422 tests, rc=0; three-crate `make
  wasm`, warning-free `make book`, rc=0. Glossary → 310 terms/9 parts/158 tokens/0 failures;
  uncertainty → 133 markers/16 files/0 unowned/0 failures; tree → 10 lanes/13 trees/6 siblings/
  0 gaps; feature → 105 rows/29 diagnostics/0 failures; ledger → 9 pass/0 fail; staged `make gate`
  → all doctrines green, rc=0. Local native/WASM integration does not assert new remote-CI verdicts.
- [x] **LOCKSTEP** — book/API/package/README status, live pointers and task/log records agree.
  Completed .3a contract/checklist retains exact HEAD bytes in the linked measurement sibling;
  rolling windows remain below health targets, so no seal is due. MTM/body, axes, breaks/composite,
  resolution and later physical/source/release proofs retain their owners; D70 is not defaulted.

## Completed G1 task journal — preserved from e299771

## Commit Log

[Completed commits through physical placements](G1-SLICE-evidence.md#historical-commit-log) are
preserved unchanged in the evidence sibling; fresh current-slice entries remain here.

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.3c.4c.1b` | `STITCHCAD-G1-0020 (leaf G1-SLICE.3c.4c.1b)` | zipper/hook-bar instances, derived Count, current targets, env_fly |

| `.3c.4c.2` | `STITCHCAD-G1-0021 (leaf G1-SLICE.3c.4c.2)` | canonical buttonhole source; Closure parent closed |

| `.3c.4d.1` | `STITCHCAD-G1-0022 (leaf G1-SLICE.3c.4d.1)` | physical Pocket composition, placement, explicit opening deferral |

| `.3c.4d.2` | `STITCHCAD-G1-0023 (leaf G1-SLICE.3c.4d.2)` | four structural families closed; D62/D63; API/MCP requirement retained |

| `.4a.1` | `STITCHCAD-G1-0024 (leaf G1-SLICE.4a.1)` | canonical length/state/provenance; D64; bounded construction sibling |

| `.4a.2a` | `STITCHCAD-G1-0025 (leaf G1-SLICE.4a.2a)` | shared exact machine-token grammar; measurement semantic sibling |

| `.4a.2b` | `STITCHCAD-G1-0026 (leaf G1-SLICE.4a.2b)` | canonical metadata/records; native/WASM; CI observation pending .2c |

| `.4a.2c` | `STITCHCAD-G1-0027 (leaf G1-SLICE.4a.2c)` | observed bf29b03 runtime/doctrine jobs; metadata parent closes |

## Changelog

[Completed task changelog through zipper/hook-bar](G1-SLICE-evidence.md#historical-task-changelog)
is preserved unchanged in the evidence sibling; new changes are recorded here.

- `2026-10-01`: `.3c.4c.2` preserves the completed closure checklist and task changelog unchanged;
  independent committed-payload comparison passes. Active contracts retain their evidence pointers.

- `2026-10-01`: `.3c.4c.2` lands button/hole pairs with one canonical size/operation source; Closure
  parent closes structurally. Physical hole derivation remains G3; next `.3c.4d` pockets/signoff.

- `2026-10-01`: `.3c.4d.1` implements Pocket physical composition/placement intent and visible opening
  deferral; next `.3c.4d.2` re-derives structural family signoff against the director's quality bar.

- `2026-10-01`: `.3c.4d.2` closes all four structural object families and fixes D62/D63. All sixteen
  §4 objects have immutable content/reference contracts and book evidence; later physical/release
  proofs retain their owners. Director's comprehensive external-agent MCP/API control requirement
  reinforces existing .6/.9/G5 parity/discovery/recovery evaluation. Next .4 measurements/ease/sizes.

- `2026-10-01`: `.4a.1` preserves canonical length inputs with explicit authored-state availability;
  D64 closes, D65 is scheduled at the archive trigger. Next .4a.2 metadata consumes core declarations.

- `2026-10-01`: .4a.2 splits into token, metadata/runtime and observed-CI signoff children before
  code. .2a delivers shared token grammar; .4a.1 moves unchanged to the measurement sibling.

- `2026-10-01`: .4a.2b adds sc-measure standalone metadata/current records and three-crate WASM
  integration; D66 closes. .2c observes the required exceptional CI push before parent signoff.

- `2026-10-01`: .4a.2c observes Rust/doctrine CI jobs and every step successful at bf29b03;
  metadata parent closes structurally. .4a.3 table follows; physical/source/release proof stays deferred.

- `2026-10-02`: .4a.3 supplies named-table current bindings; sixteen contracts/privacy and eight
  real guards validate id/token/kind/scalar pinning and borrowed canonical state. Parent .4a closes
  structurally; G1 stays 5/18. Ease .4b follows; full .4 needs Ease/SizeSet/.4d signoff.

| `.4a.3` | `STITCHCAD-G1-0028 (leaf G1-SLICE.4a.3): named measurement tables retain stable current bindings` | 16 contracts + privacy; eight real guard reds; 342 tests; WASM/book; full 23 suites; .4a structurally closed |

- `2026-10-02`: .4b.1 implements individual canonical Ease intent; .4b.2 set/membership follows.
  Strict lint found large reassignment error payloads; boxed binding snapshots retain exact structured
  evidence. Mutation diagnostic rejected an unwrap panic until an explicit error assertion preceded it.
  All seven final real guard mutations fail assertions, restore exact source; 356 strict tests pass.

| `.4b.1` | `STITCHCAD-G1-0029 (leaf G1-SLICE.4b.1): individual Ease mappings retain current signed intent` | 13 contracts + privacy, seven guard reds, 356 tests/WASM/book; set/membership next |

- `2026-10-02`: .4b.2 supplies unique per-POM sets with current canonical mappings and selected
  table membership. Same-id fit/provenance/state edits remain visible; retargeting requires explicit
  set replacement. Fourteen contracts/privacy, ten real guard reds, strict 371 tests pass. D69 fixed.

| `.4b.2` | `STITCHCAD-G1-0030 (leaf G1-SLICE.4b.2): per-POM Ease sets validate current table membership` | 14 contracts/privacy, ten guard reds, 371 strict tests/WASM/book; .4b.3 review next |

- `2026-10-02`: .4b.3 maps every ontology .2.2 field to current API/contracts and distinct deferred
  proofs. Current measure tests/docs 65, full 23-suite milestone and book green; strict 371/WASM code
  at b4e0bc7 unchanged. Staged make gate → all doctrines green, rc=0; final ledger → 9 pass/0 fail,
  rc=0. .4b closes structurally; next .4c SizeSet, then combined .4d review.

| `.4b.3` | `STITCHCAD-G1-0031 (leaf G1-SLICE.4b.3): Ease structural review passes the milestone gate` | field/currentness review, 65 current tests, full 23 suites/book; .4b structural closure |

- `2026-10-02`: .4c splits before code into membership, axes, charts/breaks, resolved intent and
  review. .4c.1 verifies label/order/base/reference separation; D70 axes conflict reproduced and owned,
  director cardinality question pending. Independent membership neither selects nor defaults axes.

| `.4c.1` | `STITCHCAD-G1-0032 (leaf G1-SLICE.4c.1): size membership preserves authored order and base identity` | 12 contracts/three docs, seven real reds, strict 386 tests; axes D70 ruling pending |

- .4c.1 verification caught the draft D70 entry at an inline quoted heading marker: live census was
  nine instead of ten. A newline-anchored heading correction retains all prior PLANNING text exactly;
  re-derived census is 10 open/59 sealed, disjoint identities. No old planning record changed.

- .4c.1 staged containment refused a 329-byte product-status row; the measured row is shortened to
  260 bytes under the unchanged 320-byte ceiling. Final staged doctrines pass, rc=0.

- `2026-10-02`: .4c.3 decomposed before code into observations, collection, MTM, breaks/composite
  and review. .3a implements current authored garment correspondence; .3b coverage can proceed while
  axes D70 awaits ruling. Sixteen contracts/privacy, eight real guard reds and strict 403 tests pass;
  native/WASM/book/current-reference boundaries remain explicit. Completed membership is preserved
  byte-identically against HEAD in the measurement sibling; no prior evidence is rewritten.

| `.4c.3a` | `STITCHCAD-G1-0033 (leaf G1-SLICE.4c.3a): garment chart observations retain current member and POM references` | 16 contracts/privacy, eight real reds, 403 strict tests/WASM/book; collection coverage next |

- `2026-10-02`: .4c.3b implements explicit Design/member/POM chart coverage over current canonical
  observations. Incomplete drafts remain inspectable; reduced targets cannot hide omitted Design POMs.
  Full-table and cell coverage do not certify numeric, physical or release readiness. Eighteen contracts/
  privacy, fourteen actual guard reds and strict 422 tests pass. Initial strict lint refused an unchecked
  u128-to-i64 fixture cast; checked conversion fixes it and the full gate is rerun. .3a evidence retains
  exact predecessor bytes in the linked sibling; next .3c MTM/body, with axes D70 still pending.
- promotion: declined (routine current-reference/coverage implementation; book/task own the contract).

| `.4c.3b` | `STITCHCAD-G1-0034 (leaf G1-SLICE.4c.3b): garment charts verify exact current Design and member coverage` | 18 contracts/privacy, fourteen reds, 422 strict tests/WASM/book; MTM/body next |

## MTM contract and evidence — preserved from 285e238

- ID: `G1-SLICE.4c.3c`
  Status: `done`
  Goal: typed MTM body-input chart correspondence for custom single-member ranges, current body/Ease
  mappings and provenance; preserve body versus garment observation distinction and path-1 semantics.
  Pre-code protocol: immutable MtmChartDefinition pins id, membership reference/member, expected
  EaseSetDefinition (reference target snapshot: set/table ids and ordered mapping bindings only),
  and chart correspondence provenance. MtmChartContext borrows membership/current Ease sets and
  EaseSetContext; reject duplicate/cross-kind identities including set/member ids. Require Custom
  membership of exactly one, with the authored member equal to its existing sole/base member.
  Resolve the saved canonical set id and compare every expected set/table/ordered mapping target;
  current fit/compression/provenance/source/state remain borrowed. Selected POM queries use current
  EaseSet membership/target checks; body and signed amount declarations/values remain distinct from
  garment metadata, with no body-as-POM result or automatic addition/evaluation. Unknown/derived
  source drafts stay inspectable and numeric queries preserve their required observation/evaluation.
  Constructor/current validation checks all authored mappings but permits empty/incomplete drafts.
  Completeness requires nonempty mappings and coverage of every current Design-table garment POM,
  with the full Design table validated; no narrowed mapping subset may certify itself complete.
  Direct grade-rule input refuses explicitly for this authored MTM chart, independent of numeric
  readiness. No breaks field/default or axis representation; composite/path execution belongs .3d/G3.
  Acceptance: member/system/reference guards, missing/stale/current mapping and table failures,
  canonical borrowing/state/provenance edits, body/Ease numeric refusal, explicit immutable replacement,
  full POM coverage and grade refusal. Own book/API/live docs, exact evidence relocation and rolling
  seals where health milestones require them; real production mutations and focused integration checks.
  Book publication: apply the director's incremental teaching requirement to this chapter now;
  put low-level API/reference/currentness/verification detail in a linked expert annex.
  Verification: 15 contracts/two privacy-role docs, twelve real reds; 439 strict tests/WASM/book;
  glossary/uncertainty/feature/tree/ledger and staged doctrines.
  Commit: `STITCHCAD-G1-0035` (this recording commit).

### `G1-SLICE.4c.3c` — canonical MTM body/Ease inputs

- [x] **REPRODUCE / ISSUE** — `git grep -n 'pub struct MtmChart' e299771 -- crates/sc-measure/src`
  → 0 matches, expected rc=1; size-sets §11 needs custom-member body/Ease input correspondence.
- [x] **ROOT CAUSE (WHY + WHERE)** — `rg -n 'Made-to-measure|Body|Ease' docs/book/src/spec/size-sets.md`
  → MTM regeneration requires explicit body-to-POM mappings, rc=0; garment observations alone
  cannot supply a body input or certify its evaluated garment result.
- [x] **FIX** — private immutable MTM charts pin exact custom sole-member and Ease-set reference
  targets; borrow current mappings, metadata, declarations, source/state/provenance. Unknown/derived
  numeric inputs refuse; completeness checks every current Design garment POM; grading always refuses.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-measure --test mtm_chart_contract` → 15 passed,
  rc=0; two privacy/role compile-fail docs pass. `bash docs/tasks/artifacts/mtm_chart/run_mtm_chart_mutations.sh`
  → twelve actual assertion reds (rc=101 each), exact production-source restoration, runner rc=0.
  Body and Ease zero fallbacks and enabled grading are independently rejected.
- [x] **NO REGRESSION** — `make check` → strict fmt/clippy and 439 tests, rc=0; `make wasm` and
  warning-free `make book`, rc=0. Glossary/tree/feature/uncertainty censuses, ledger and staged
  doctrines green. Source and rendered main/annex links and table cells checked; no new remote-CI claim.
- [x] **LOCKSTEP** — roadmap size-sets/regeneration contracts, package/API, book/main/annex and live
  pointers agree. Completed .3b contract/checklist retains exact e299771 bytes in the linked sibling;
  oldest changelog/lesson payloads seal unchanged. Director's publication requirement and landing
  status defect D71 are owned by .4d.1 next; axes D70 remains unanswered, with no representation default.

## Publication contract and evidence — preserved from 9ef9602

- ID: `G1-SLICE.4d.1`
  Status: `done`
  Goal: apply the director's 2026-10-02 book requirement: roadmap, code and book in lockstep;
  incremental learning path for students/newcomers, direct expert navigation, glossary and index,
  detailed contracts/verification in annexes. Preserve canonical anchors and normative requirements.
  Reproduce D71: introduction.md says "project is in gate G0" while LIVE_STATUS/sc-measure report G1
  runtime libraries. Impact: readers cannot distinguish built capabilities from future contracts.
  Priority: next safe leaf after MTM commit; correct landing/status and review the current chapter
  inventory against roadmap/code/task owners. No app, MCP server, geometry or release claim before proof.
  Acceptance: bounded newcomer progression, expert annex/reference links, complete topic index plus
  existing verified glossary; source/rendered-link checks, status/requirement map and owned remaining
  chapter migrations when too large for one safe leaf. D71 fixed and verified before closing.
  Pre-edit publication protocol: new learning chapters progressively introduce recipe, measurements,
  pieces, sizes and agents using declared examples; plain availability page distinguishes existing
  libraries from future application/geometry/MCP/release. Existing detailed contracts enter the Annexes
  navigation section at their unchanged source URLs/anchors; glossary remains independently reachable.
  Topic index covers the entire registered chapter population, with expert links for current APIs.
  Add a scoped roadmap/code/book status map and standard-library publication checker with actual
  copied-fixture refusal probes for orphaned/missing chapters, missing index coverage, invalid source/
  rendered links and status/API mismatch. Fixtures and generated HTML remain on this repo volume.
  Adopt directive in a decision and roadmap principle/disposition; compact map input at unchanged cap.
  Correct D71 landing and measurement-input status; preserve earlier checklists/journals unchanged,
  D72 milestone blocker: archive resident-limit RED uses the live archive plus 22 fixed records;
  486538 + 3520000 decoded bytes exceeds 4000000, so decoded-limit refusal preempts its intended
  resident predicate. Fix this small fixture defect here before closing publication verification:
  use a fixed minimal valid archive, paired passing/overflow resident cases, no reader/bound changes.
  Keep ROADMAP within its original baseline by moving its exact D32 explanation into the existing
  canonical decision and leaving a concise disposition link; no registry/ceiling changes.
  Roll live records by exact predecessor bytes if required. Focused checks plus full probe milestone.
  Verification: 47 chapters/14 scoped public APIs, all source/rendered links; nine publication probes,
  28 archive arms and full 24 suites green; glossary/ledger/censuses and staged doctrines.
  Commit: `STITCHCAD-G1-0036` (this recording commit).

### `G1-SLICE.4d.1` — progressive publication and scoped lockstep

- [x] **REPRODUCE / ISSUE** — `git show 285e238:docs/book/src/introduction.md` reports G0-only
  status despite G1 libraries; measurement-inputs calls implemented Ease future work. Full milestone
  initially fails archive resident fixture with decoded-limit error and roadmap baseline growth.
- [x] **ROOT CAUSE (WHY + WHERE)** — first `make probes` → archive `AssertionError` naming
  decoded versus resident predicate, rc=2; `wc -lc ROADMAP.md` → 964/54067, rc=0. D71 is stale
  landing/status prose, not missing code. D72 is
  history_archive_probes.py: 486538 + 22 × 160000 = 4006538 exceeds decoded 4000000 before resident
  validation. The reader is correct; a production-sized baseline made the intended RED dependent on
  unrelated growth. Roadmap 964/54067 exceeds unchanged 951/53153 baseline; trimming is the remedy.
- [x] **FIX** — five progressive learning chapters, truthful availability, independent glossary/
  topic index and detailed annex navigation with preserved URLs/anchors. Fourteen API/status rows
  link roadmap/code/book/task owners; publication refusal probes own regression coverage. Fixed
  independent resident fixture plus green/control arm; D32 disposition retained exactly in its
  canonical decision while roadmap becomes concise, 951 lines/53129 B. No cap or reader changed.
- [x] **ADDRESSED (verified)** — `bash docs/tasks/artifacts/book_publication/run_book_publication_probes.sh`
  → 47 chapters/14 rows/983 source links/1489 rendered links; eight named refusal mutations and
  one real-tree green, nine pass/0 fail, rc=0. Archive → 28 pass/0 fail, rc=0. Landing G1 and Ease
  scope corrected; D71/D72 close with exact historical descriptions preserved in defects part14.
- [x] **NO REGRESSION** — `TMPDIR="$PWD/target/scratch" make probes` → 24 suites green, rc=0;
  glossary → 310 terms/9 parts/158 tokens/0 failures; tree/feature/uncertainty/ledger green, rc=0.
  Warning-free book and source/rendered table cells pass; staged `make gate` → all doctrines green.
  Rust/Cargo unchanged from strict 439-test native/WASM 285e238; no new runtime or remote-CI claim.
  Browser screenshot review is unavailable under the environment's local-URL policy; rendered
  artifact checks verify content/navigation instead, without asserting a visual browser inspection.
- [x] **LOCKSTEP** — director directive adopted in roadmap §2/disposition and indexed retrievable
  decision. Public source map and availability distinguish structural libraries from future geometry/
  API/MCP/apps/approval. Original D32 disposition and completed MTM contract/checklist retain exact
  HEAD bytes. Rolling seals preserve payloads; no baseline/ceiling raised. .4d.2 combined review stays
  pending; .5a syntax can progress independently of D70's required unanswered axes ruling.

Publication continuation is owned by the current parent frontier.

## Completed origin and context records — preserved from dab0ee4

## Missing-value routing

- ID: `G1-SLICE.5e.1a`
  Status: `done`
  Goal: independently repair D122 reference missing-value diagnostic routing across nine origins
  and reserved size/tolerance contexts before product .5e.1 adapter proof.
  Acceptance: canonical origin-specific tokens/arguments, populated values unchanged, malformed
  metadata refused, actual fault controls and book/runtime replay remain honest; D124 untouched.
  Pre-code contract: formula3/3.1/5.1/5.2, grammar7/7.1, namespace metadata-only interfaces and
  existing geometry/lazy/replay boundaries reviewed. Actual baseline size_index -> tolerance-unbound,
  missing geometry/tolerance -> unknown, and supplied eps_fmt25 still refused because always=False.
  Related D127: missing kind/origin/state and unhashable origin leak KeyError/TypeError; own/fix here.
  Scope: origin-specific missing-value routing, explicit reserved context values independent of
  always-available metadata, named malformed declaration/absent-fact-state refusals. Preserve populated
  values, kind/value units, static metadata isolation, optional lazy geometry and earlier error order.
  Reference diagnostic arguments include name/origin, actual state for unknown facts, searched
  origin vocabulary for unbound names and explicitly supplied context label for tolerances. No
  invented artifact policy, statement ordinal/canonical bytes or production typed diagnostic claim.
  Independently author closed nine-origin/eight-kind/five-state missing population, four populated
  states, reserved no-context/provided contexts, taken-only reads, lazy geometry and malformed inputs.
  Actual in-memory predicate/payload faults must fail body assertions; full reference/language/book/
  ledger/gate and exact task/ledger/report containment before commit. D124/D121 stay separate.
  Verification:1466 actual cases/thirteen actual body reds/two actual copied-book consumer refusals,
  final full reference/language/book checks rerun for D128 before commit. Commit: `STITCHCAD-G1-0080`.
  Containment exact completed block: 27lines/2348B/SHA256 4ae12e55e188117d0bb18e9bb54ca18f363b66e08106616433d4359ad4eb529c; existing sibling, no cap/path change.
  Containment exact completed block: 35lines/3250B/SHA256 10fcfb9c08fe35156ff789ded8c71d4952169e91d4ee68217a9ddf71ff76e061; existing sibling, no cap/path change.
  Initial full runner correctly refused obsolete namespace fault anchors after the declaration
  guard moved into try. Retarget spelling/origin/value-read faults at the same actual predicates
  with valid indentation; retain all1139 expected cases and thirteen body assertion reds.
  Documentation prerequisite: static annex338lines/23868B is near its400line/24576B health target.
  Move runtime assertion details into a dedicated runtime annex before adding origin/context detail;
  update SUMMARY/index/contract/cross-links, preserve current scope and verify source/rendered links.

### Origin/context receipts — `G1-SLICE.5e.1a`, `2026-10-02` (UTC)

- Actual origin_value_contract.py --mutations terminal0:1466 cases/thirteen actual body assertion reds,
  actual loaded table/definitions; two copied books preflight21 then report distinct missing
  size_index/eps_fmt runtime tokens, consumer1/producer0. Named metadata refusals replace host errors.
- Full structural reference runner terminal0 retains signature4032/namespace1139/thirteen faults,
  whole recipe196/review67/assertion262 and all numerical/canonical/binding families. Source stays
  unchanged during in-memory faults. Initial full runner's stale anchor refusal was corrected;
  an anchor or syntax error never counted as a body red. No Rust source/test/serializer diff.
- Language16 terminal0 after book split. Origin/context arguments remain scoped: no artifact policy,
  typed production diagnostic, guessed statement ordinal or object/entity geometry certificate.
- Completed node27lines/2348B/SHA4ae12e55e188117d0bb18e9bb54ca18f363b66e08106616433d4359ad4eb529c
  and receipts35lines/3250B/SHA10fcfb9c08fe35156ff789ded8c71d4952169e91d4ee68217a9ddf71ff76e061
  match Gitac7f0bf byte-exact in canonical sibling; no new task path/map growth or cap change.

- Publication9 terminal0:54chapters/25APIrows/1122source/1740rendered links, no build warnings.
  Ledger9/pointer13/tree10lanes/13trees/10siblings/0unowned-orphan-deadlinks terminal0.
  Materialized independent defects12open/115unique sealed/overlap0/duplicate0; old D122 report,
  G1-0063 ledger and recognition lesson independently match Git predecessor payloads, rc=0.

- D128 tools-first final state audit: actual measurement/length/value17 returns17 with unknown
  and invalid states, baseline0. Owned here before commit: explicit invalid state and unknown+
  populated value refuse parse before lazy work; unknown lazy geometry refuses by its origin before
  constructing/caching data. Independently expand nine-origin/eight-kind populated-state controls,
  valid no-state computed fixtures and trapped unknown geometry, then rerun reference/book checks.

- Final D128-expanded source: full reference runner1466 cases/thirteen actual origin/state faults
  terminal0; current language16/publication9/ledger9 and embedded pointer13 terminal0. Publication
  remains54chapters/25APIrows/1122source/1740rendered links. Final independent defects12open/
  115unique sealed/overlap0/duplicates0, rc=0. Unknown payload/state/lazy guards fail actual body
  assertions when removed; valid/no-state fixtures and static metadata-only controls stay green.

- Final staged doctrine registry13 checks terminal0; hook repeats final committed records.


## Completed assertion checklist — preserved from dab0ee4

### G1-SLICE.5e.3a — D125 reference assertion repair

- [x] **TOOLS-FIRST / ROOT CAUSE** — actual statement returnedFalse instead of formula_assertion;
  equal control True, actual contract5/9 requires named failure; tracked producer262 controls, rc=0.
- [x] **ADDRESSED** — false raises owned name/values/kinds/class arguments, true tuple/boundary
  preserved; actual copied-book consumer raises formula_assertion and refuses1, producer rc=0.
- [x] **NO REGRESSION** — full structural reference suite and eight actual guard/payload assertion
  reds pass, rc=0; original serializer/Rust bytes unchanged. D121/D122/D124 remain separate owners.
- [x] **RETENTION** — completed52line/4156B and63line/5643B blocks retained byte-exact in canonical
  sibling. SHA c701597a4f5cbd2435cd336f62253fc0ff3d32c951fb9077969f7d89b4e27634 and
  6e7c34b99aabe774e372162cf60219bd28d61a8715f0d1185a5ec59ca013da29 match Git13f8c75, rc=0.
- [x] **LOCKSTEP** — language16/publication9/ledger9/pointer13/tree census green, rc=0;
  book/live/task records agree, G1 stays5/18,13open/112sealed; D125 original report preserved.
  Staged doctrine registry13 checks pass, rc=0; hook repeats final records. No product runtime claim.
  promotion: declined (existing independent-diagnostic/dimensional-algebra/retention principles).

[Exact completed records](G1-SLICE-canonical.md#completed-static-checklists--preserved-from13f8c75) are retained in the canonical sibling.

Retention: complete relocated dab0ee4 task payloads verified by G1-SLICE.5b.1c.2.

## Completed recognition and provenance checklists — preserved from d7a421e

### G1-SLICE.5b.1c.2 — approved current-grammar recognition

- [x] **REPRODUCE / ISSUE** — predecessor contract6 promised formula_unsupported for forms whose
  actual static reference prints unbound-name/parse; static_review_contract.py baseline67, rc=0.
- [x] **ROOT CAUSE (WHY + WHERE)** — contract6 diagnostic cells conflict with grammar1.1/6's
  closed keywords/unknown-call rule. static_review_contract.py --mutations →100cases/seven
  actual source guard reds/three loaded-doc reds, rc=0; metadata/execution callbacks trapped.
- [x] **FIX** — director ruling preserves current grammar: unknown calls unbound-name, malformed
  syntax parse, recognized non-square powers unsupported; ordinary scalar names remain valid.
- [x] **ADDRESSED (verified)** — static_review_contract.py --mutations →100cases/seven body reds/
  three documentation reds, rc=0. Both exclusion cells/three keywords, five ordinary scalar names/
  headers, unknown-call/parse/envelope precedence independently verified; actual source unchanged.
- [x] **NO REGRESSION** — full structural reference and language16/publication9 pass, rc=0.
  Signatures4032/namespace1139/recipe196 and numerical families stay watched; Rust bytes unchanged.
  Final ledger/retention/staged gate receipts recorded in the evidence sibling before commit.
- [x] **LOCKSTEP** — contract/grammar/static annex/ADR/owned review/live/frontier agree. Exact
  completed-task payloads and oldest ledger/lesson/D124 report retained in existing bounded parts.
  G1 remains5/18; complete product namespace/type/graph/numerical/geometry proof stays pending.
  promotion: declined (application of the director's recorded ruling and existing static boundaries).


### G1-SLICE.5e.3b — D121 executed contribution provenance

- [x] **REPRODUCE / ISSUE** — actual predecessor reference accepts sin90 at T1 in both
  assertion and within; scoped loader reproduction prints ACCEPT True/boolean1, rc=0.
- [x] **ROOT CAUSE (WHY + WHERE)** — actual Val only carries kind/value; stored/read/operator/
  lazy/book paths discard derivation. provenance_contract.py --mutations exercises425 cases and
  26 actual compiled guard/source/cache/consumer body assertion reds, rc=0, locating those seams.
- [x] **FIX** — immutable sources through executed dependencies, coordinate-specific cache evidence,
  actual book binding publication and two named-class guards. Director selects formula_domain.
- [x] **ADDRESSED (verified)** — provenance_contract.py --mutations →425 cases/26 body reds,
  rc=0; actual copied-book equal bound operands preflight21 then refuse named eps_num/hypot.
  Exact/untaken/coordinate-independent T1 stays admissible; no product execution claim.
- [x] **NO REGRESSION** — full reference runner and language16 pass, rc=0; earlier numeric/static/
  origin/assertion controls remain watched. Publication/ledger/tree/staged-gate receipts recorded
  in the evidence sibling before commit; Rust/serializer bytes unchanged.
- [x] **LOCKSTEP** — ADR/ruling, diagnostic/runtime/static book, task/live/resume/tool routes align;
  oldest ledger/lesson/D121 report and completed task payloads retained exactly in bounded parts.
  promotion: declined (existing provenance, lazy execution and independent-evidence principles).


Retention: complete predecessor payload preserved by .5b.2a.

## Completed whole-preflight checklist — preserved from 862c0d9

### `G1-SLICE.5b.1b.2` — whole reference static preflight

- [x] **REPRODUCE / ISSUE** — D119 lacks whole preflight; D123 copied25 ceiling accepts21 in
  preflight then old aggregate L8 refuses34, baseline rc1. Original reports retained in part45.
- [x] **ROOT CAUSE (WHY + WHERE)** — book L2 evaluates as it discovers later errors; L8 adds
  unrelated refusal rows. static_recipe_contract.py --mutations →196 cases/14 actual assertion
  reds, rc=0; copied25-ceiling baseline reports34 and exits1 despite accepted21, rc=1.
- [x] **FIX** — original-source top-level boundaries, local metadata-only ordered namespace,
  complete plan after all statements pass; whole worked recipe preflight before L2, actual size21.
- [x] **ADDRESSED (verified)** —196 cases/14 actual guard body assertion reds; consumer3 and
  copied per-recipe ceiling control pass; static_recipe_contract.py --mutations →rc=0,
  execution/value/geometry trapped, no accepted prefix. Actual source unchanged during controls.
- [x] **NO REGRESSION** — full reference suite including4032 signature/1139 namespace matrices
  run_formula_structure_probes.sh →rc=0; run_formula_language_probes.sh →16 pass/0 fail, rc=0;
  run_book_publication_probes.sh →9 pass/0 fail, rc=0. No Rust source/test bytes changed.
- [x] **LOCKSTEP** — progressive worked chapter/annex/index/live/task/ledger records agree,
  G1 stays5/18, defects12open/110sealed; .1b done, .1c next. Exact old records retained.
  promotion: declined (existing whole-refusal, declaration-order and independent-evidence principles).

[Exact completed namespace checklist](G1-SLICE-evidence.md#completed-namespace-checklist--preserved-from-b2c4d6e) retained.
