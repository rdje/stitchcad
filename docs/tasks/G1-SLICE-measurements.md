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
