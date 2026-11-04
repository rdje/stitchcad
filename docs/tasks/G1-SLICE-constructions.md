# G1-SLICE — construction contracts and ordered integration

The bounded construction sibling of [G1-SLICE](G1-SLICE.md). Completed child goals link to
sealed originals;the final ordered-integration branch owns its active work. MEMORY.md names the current leaf.
Historical verification/commit/changelog tables stay in [the evidence sibling](G1-SLICE-evidence.md).

## Completed construction child contracts

Whole closed goals/acceptance/proofs remain byte-exact in [part959](../history/window29.md#stitchcad-devnotes-part959md).

- ID: `G1-SLICE.3c.4a`
  Status: `done`
  Goal:completed;whole original contract/proof in part959 above.
  Children: `.3c.4a.1` (dart), `.3c.4a.2` (tuck/pleat/gather).

- ID: `G1-SLICE.3c.4a.1`
  Status: `done`
  Goal:completed;whole original contract/proof in part959 above.

- ID: `G1-SLICE.3c.4a.2`
  Status: `done`
  Goal:completed;whole original contract/proof in part959 above.
  Children: `.3c.4a.2a` (tucks/pleats), `.3c.4a.2b` (gathers linked to a sewing span).

- ID: `G1-SLICE.3c.4a.2a`
  Status: `done`
  Goal:completed;whole original contract/proof in part959 above.

- ID: `G1-SLICE.3c.4a.2b`
  Status: `done`
  Goal:completed;whole original contract/proof in part959 above.

- ID: `G1-SLICE.3c.4b`
  Status: `done`
  Goal:completed;whole original contract/proof in part959 above.
  Children: `.3c.4b.1` (served layers), `.3c.4b.2` (hem including faced-hem links).

- ID: `G1-SLICE.3c.4b.1`
  Status: `done`
  Goal:completed;whole original contract/proof in part959 above.

- ID: `G1-SLICE.3c.4b.2`
  Status: `done`
  Goal:completed;whole original contract/proof in part959 above.

- ID: `G1-SLICE.3c.4c`
  Status: `done`
  Goal:completed;whole original contract/proof in part959 above.
  Children: `.3c.4c.1a` (physical notion placements), `.3c.4c.1b` (centred zipper/hook-bar/fly),

- ID: `G1-SLICE.3c.4c.1a`
  Status: `done`
  Goal:completed;whole original contract/proof in part959 above.

- ID: `G1-SLICE.3c.4c.1b`
  Status: `done`
  Goal:completed;whole original contract/proof in part959 above.

- ID: `G1-SLICE.3c.4c.2`
  Status: `done`
  Goal:completed;whole original contract/proof in part959 above.

- ID: `G1-SLICE.3c.4d`
  Status: `done`
  Goal:completed;whole original contract/proof in part959 above.
  Children: `.3c.4d.1` (Pocket metadata/composition), `.3c.4d.2` (construction/object-family signoff).

- ID: `G1-SLICE.3c.4d.1`
  Status: `done`
  Goal:completed;whole original contract/proof in part959 above.

- ID: `G1-SLICE.3c.4d.2`
  Status: `done`
  Goal:completed;whole original contract/proof in part959 above.

## Completed construction acceptance checklists

### `G1-SLICE.3c.4a.1` — semantic dart intent with visible closure obligations

[Whole original dart acceptance](../history/stitchcad-devnotes-part1003.md) remains byte-exact.

### `G1-SLICE.3c.4a.2a` — distinct tuck/pleat intent with shared structural checks

[Whole original fold acceptance](../history/stitchcad-devnotes-part1046.md) remains byte-exact.

### `G1-SLICE.3c.4b.1` — completed served-layer protocol

Whole0017 [goals/acceptance/proofs](../history/window28.md#stitchcad-devnotes-part914md) retained exactly.

### `G1-SLICE.3c.4a.2b` — gathers bind physical span sides and borrow canonical ease

[Whole original Gather acceptance](../history/stitchcad-devnotes-part1047.md) remains byte-exact.

### `G1-SLICE.3c.4b.2` — Hem retains depth/fold intent and validates current Facing targets

[Whole original Hem acceptance](../history/stitchcad-devnotes-part1048.md) remains byte-exact.

### `G1-SLICE.3c.4c.1a` — physical notion placements preserve stable copy bindings

[Whole original notion acceptance](../history/stitchcad-devnotes-part1049.md) remains byte-exact.

### `G1-SLICE.3c.4c.1b` — zipper/hook-bar instances borrow current placements and refuse fly scope

- [x] **REPRODUCE / ISSUE** — ontology §4.7 requires closure kinds, placement, count and sizes;
  fixture §9 names a zipper and hook/bar. The prior commit only supplies notion placements.
- [x] **ROOT CAUSE (WHY + WHERE)** — current placements can lose interior material after birth.
  `cargo test -p sc-core --test closure_contract
  current_placement_repairs_are_revalidated_at_birth_all_target_checks_and_borrowed_queries` →
  `1 passed`, `rc=0`: birth/all-target/borrowed queries return the same current interval evidence.
  `fly_scope_refusal_precedes_geometry_and_names_requested_closure_gap_and_gate` → `1 passed`,
  `rc=0`: env_fly carries Closure/trousers gap/G7 even before any geometry context exists.
- [x] **FIX** — immutable stable instances and canonical placement ids; Count derives from nonempty
  instances. Distinct zipper/hook-bar size origins stay symbolic; no defaults. Current targets and
  unambiguous registry ids validate; fly scope refuses before geometry. Nested evidence is boxed.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test closure_contract` → `10 passed`, `rc=0`:
  kinds/sizes/counts/reordering, identity reuse, ambiguous/missing contexts, current repairs/removals,
  scope and immutability. Count boundary unit + privacy pass. Scope, reuse, length and current-target
  mutations each fail red, `rc=101`; restored strict checks pass. Placement checklist/commit table
  compare unchanged with `git show HEAD:docs/tasks/G1-SLICE.md`, with staged revalidation.
- [x] **NO REGRESSION** — `make check` → fmt/strict clippy/all Rust + privacy green; `make wasm` →
  green; `make book` → warning-free; fixture → `0 mismatch(es)`; feature/glossary → `0 failure(s)`;
  tree census → `0 unowned / 0 orphan(s) / 0 dead link(s)`; ledger → `9 pass / 0 fail`;
  staged `make gate` → `=== all doctrines green ===`, all `rc=0`. Earlier placement contracts pass.
- [x] **LOCKSTEP** — pre-code copy/instance decision, Design obligations, modules/map, closure examples/
  API vocabulary and feature reasons, bounded evidence/commit retrieval, frontier/index and live docs
  align. Oldest entries seal unchanged to changelog-part20/devnotes-part19. Hardware remains G2/G3; buttons
  follow `.3c.4c.2`. G1 remains 4/18 top-level leaves and 3/4 object families.

### `G1-SLICE.3c.4c.2` — buttonhole length has one canonical button/operation source

[Whole original Buttonhole proof](../history/stitchcad-devnotes-part1027.md) remains byte-exact.

### `G1-SLICE.3c.4d.1` — Pocket retains physical composition and owned placement intent

[Whole original Pocket acceptance](../history/stitchcad-devnotes-part1041.md) remains byte-exact.

### `G1-SLICE.3c.4d.2` — structural object families reviewed against the production-grade bar

- [x] **REPRODUCE / ISSUE** — all individual §4 objects are delivered; parents still await a complete
  review. Full probes expose D63 (feature `11 pass / 1 fail`), and crate-scoped test counts expose D62.
- [x] **ROOT CAUSE (WHY + WHERE)** — `cargo test -p sc-core -- --list` → `260` tests including `18`
  docs, `rc=0`; fourteen object/support suites contain `149` contracts. Source/API review accounts
  for sixteen §4 objects in four families. `cargo test -p sc-units` → `5 + 21 + 1`, `rc=0`;
  `git show eb83f01:crates/sc-core/src/lib.rs` → three original smoke tests, explaining workspace 30.
  D63's literal old explanation matches no current tuck/pleat row; fixture unchanged, census green.
- [x] **FIX** — close structural object parents after review; retain all geometric, recipe/profile,
  current-registry and release obligations. Correct crate-scoped live counts (D62). D63 targets the
  feature's gate cell and independently proves one malformed gate; absent/duplicate targets refuse
  setup. Director's API/MCP control requirement belongs existing .6/.9/G5 parity/evaluation contracts.
- [x] **ADDRESSED (verified)** — `run_feature_matrix_probes.sh` → `14 pass / 0 fail`, `rc=0`:
  malformed gate refused by unchanged census; missing/duplicate target setup refused. No-op mutation
  of gate writer → `13 pass / 1 fail`, `rc=1`, naming fixture setup failure; restored suite passes.
  `make check` executes `287` tests including `19` docs, `rc=0`; all 149 object/support contracts pass.
  Completed Pocket/button checklists compare unchanged to HEAD and staged gates revalidate them.
- [x] **NO REGRESSION** — strict `make check`, `make wasm`, warning-free `make book`, full `make probes`
  → `22 suite(s) green`; fixture → `0 mismatch(es)`; feature/glossary → `0 failure(s)`; tree →
  `0 unowned / 0 orphan(s) / 0 dead link(s)`; ledger → `9 pass / 0 fail`; staged `make gate` →
  `=== all doctrines green ===`, all `rc=0`. No census predicate, diagnostic or envelope was weakened.
- [x] **LOCKSTEP** — object status/module/map, bounded book review and current-query limits,
  command/MCP requirement, parent closures, fresh frontier/index/live records align. D62/D63 close
  in the sealed defect ledger; 8 open / 54 sealed. G1 → 5/18 leaves, all four structural families;
  next .4 measurements/ease/sizes. G0 stays unapproved and production declaration remains G7.
  Review examples have a bounded ontology-review chapter; oldest allowance lesson seals unchanged
  to devnotes-part22. Completed historical checklists retain independent HEAD comparison evidence.
  promotion: declined (review re-verifies existing canonical decisions; D63 is a local fixture repair).

## Completed recipe byte records — preserved from d7a421e

## Complete recipe byte contract protocol

- ID: `G1-SLICE.5a.3f.1a`
  Status: `done`
  Goal: repair D109 exact-byte specification before complete recipe identity implementation.
  Clean predecessor eee15a8; no jobs/user edits. Read grammar4, contract, worked examples, existing
  D84/D95/D103 expression rules, statement/ordered APIs and governance6.1 delegated authority.
  Decide engineering bytes under the director's standing delegation; engineer authors/applies same
  decision and cannot approve its evidence. No new human/release/gate closure claimed.
  Retain existing (bind name kind expression); select flat (assert name tolerance left right),
  explicit (recipe statement...) envelope, empty (recipe), one space/no final newline. Names,
  declared kind/symbolic tolerance and authored operand/statement order remain identity-bearing.
  No sorting, duplicate merging, annotation inference, tolerance resolution or algebraic rewrite.
  Typed identity domains remain distinct; bytes do not claim project schema, hash namespace or
  canonical-input reader. Later store .7 must frame its typed fields/digest domains explicitly.
  Independently author statement and empty/whole ordered recipe byte fixtures; actual recursive
  book-reference parsing supplies every operand without infer/evaluate. Author-chunk recipe
  coverage remains distinct from an independent whole-recipe parser. Check all six bindable kinds/
  five tolerances plus aliases/raw angles/signs/calls/branches/order/empty/whitespace examples.
  Actual renderer opcode/name/annotation/operand/envelope/order/newline/empty faults must fail
  authored exact-byte assertions; exclusive finally restore exact tracked producer source.
  Standing structural suite watches producer/anchors/classifier; no product serializer proof claim.
  Book grammar + statement annex/index and a one-record technical decision expose exact bytes,
  implementation status, independent reviewer/unapproved evidence and principal's reversal path.
  Preserve closed subtree/closure/oldest ledger payloads exactly; shorten orientation input only
  as needed for new record within unchanged map ceiling. Scoped syntax/book/recording gates.
  Verification: actual reference16 statements/six kinds/five tolerances; nine whole sources/
  twelve ordered chunks/all four displayed examples; nine interpreter authored-byte assertion reds/
  exact restore. Focused syntax20/structural/language16/publication9 pass0. Product Rust source/test
  bytes stay exact eee15a8; original subtree/closure/oldest ledgers/report payloads preserved.
  Book51 chapters/21 APIs/1060 source/1640 rendered links; technical evidence unapproved.
  Commit: `STITCHCAD-G1-0068`.

### Complete recipe contract recording receipts

Ledger9/13 pointer controls, archive28/184 CLI controls/175 logical reads and retention175 records/
50 working Markdown/8712 decoded lines/673893 decoded bytes/330215 resident bytes pass0. Fresh
reconstruction yields10open/99unique sealed/zero overlap0. Tree10lanes/13trees/nine siblings/zero gaps;
glossary310/nine/158; feature105/29; uncertainty133/16; fixture20/four/five/zero mismatch all0.
Original D109 report and oldest G1-0046 changelog/G1-0066 lesson payloads compare exact eee15a8.
All scoped jobs observed terminal; final staged doctrine gate is required before commit.
Two actual reference witnesses serialize bind(width, length, 25 mm) identically to its binding
statement and recipe(bind(n, count, 1)) identically to its one-statement recipe. Typed field/digest
framing is therefore a concrete .7 acceptance obligation, not a new keyword or untyped hash claim.
Producer and watched structural rerun observed terminal rc=0 after all nine actual renderer reds;
source restoration remains exact. Product implementations are unchanged.
Staged make gate → === all doctrines green ===, rc=0; every scoped verification job observed terminal.

## Complete recipe byte closure — preserved during G1-0069

### `G1-SLICE.5a.3f.1a` — exact complete recipe byte contract

- [x] **REPRODUCE / ISSUE** — D109 exact assertion/empty/envelope bytes absent from grammar4;
  original report preserved byte-identically in sealed part40. Existing binding bytes stay fixed.
- [x] **ROOT CAUSE (WHY + WHERE)** — `git diff -- docs/book/src/spec/formula-language/grammar.md`
  shows predecessor's explicit missing contract versus exact new4.1 templates/examples, rc=0.
- [x] **FIX** — retained bind, flat assert payload, ordered/empty recipe wrapper/single spaces/no
  final newline; decision discloses same-party author/applier, review/reversal and future typed .7 owner.
- [x] **ADDRESSED (verified)** — `recipe_byte_contract.py` actual reference syntax matches16 authored
  statements/nine whole sources/twelve chunks/all four published examples; `recipe_byte_mutations.py`
  nine actual authored-byte assertion reds/exact restoration, rc=0. No compiled product claim.
- [x] **NO REGRESSION** — focused `cargo test` recipe11/statement9:20 passed, rc=0; structural/
  language16/publication9 all rc=0. Every Rust source/test byte equals eee15a8; no behavior changed.
- [x] **LOCKSTEP** — normative grammar/annex/index and decision/ADR/current live records align;
  old subtree/closure/three sealed payloads exact; D109 technical gap closed, .1b normalization next.

## Verification Log

Pre-change protocol precedes instruments/book edits. Actual recursive reference syntax independently
renders16 authored statement rows (six kinds/five tolerances), nine complete sources/twelve chunks and
all four actual book/decision examples. Inference/evaluation trapped; no whole-reference-recipe parser
or compiled product serializer claim. Two actual ordinary-call byte collisions confirm the typed
identity boundary assigned to .7. Nine actual interpreter authored-byte assertion reds/exact
source restore and normal rerun observed terminal0; classifier anchors/noise controls watched.
Focused syntax20 (recipe11/statement9), structural suite/language16 and publication9 pass0:
51 chapters/21 APIs/1060 source/1640 rendered links. Rust source/test bytes compare exact eee15a8.
Ledger9 arms/13 pointers, archive28 arms/184 CLI controls/175 logical reads and retention175 records/
50 working Markdown/8712 decoded lines/673893 decoded bytes/330215 resident bytes pass0.
Fresh reconstructed defects10open/99unique sealed/zero overlap; tree10 lanes/13trees/nine siblings/
zero gaps, glossary310/nine/158, feature105/29, uncertainty133/16, fixture20/four/five pass0.
Predecessor complete syntax subtree/closure and three sealed payloads compare byte-identically0.
Map orientation shortened within unchanged ceiling; no unique maintained facts or ceilings removed.
Staged make gate → === all doctrines green ===, rc=0; independent evidence approval unapproved.

## Commit Log

| Leaf | Commit subject | Verification |
| --- | --- | --- |
| `.5a.3f.1a` | `STITCHCAD-G1-0068 (leaf G1-SLICE.5a.3f.1a): specify exact statement and ordered recipe identity bytes` | 16 statements/nine sources/nine actual reds |


Retention: complete predecessor payload preserved by .5b.2a.

## Completed product metadata receipts — preserved from cfd0748

## Product semantic vocabulary receipts — .5b.2a,2026-10-03 (UTC)

[Whole closed receipt](../history/stitchcad-devnotes-part1037.md) retains its exact original evidence.

## Completed product metadata checklist — preserved from cfd0748

### G1-SLICE.5b.2a — closed product semantic metadata

[Whole closed checklist](../history/stitchcad-devnotes-part1037.md) retains its exact original evidence.

## Completed product metadata protocol — preserved from cfd0748

- ID: `G1-SLICE.5b.2a`
  Status: `done`
  Goal:0083 closed metadata protocol;[whole scope/proofs](../history/stitchcad-devnotes-part1032.md).

Retention: complete predecessor payload preserved by .5b.2b.

## Completed declaration protocol — preserved from db19b90

- ID: `G1-SLICE.5b.2b`
  Status: `done`
  Goal:0084 sourced declaration protocol;[whole scope/proofs](../history/stitchcad-devnotes-part1033.md).

## Completed declaration receipts — preserved from db19b90

## Sourced declaration receipts — .5b.2b,2026-10-03 (UTC)

- Closed named immutable source views: input metadata and canonical declaration IDs kept distinct;
  existing length records borrowed by pointer; geometry creator/tag preserved, no resolution.
  Recipe annotations/name/whole-name spans/ordinal come from actual normalized let positions only.
  No state/value query in kind/origin inspection; generic metadata does not certify target records.
- Seven public contracts and five actual negative construction/private/lifetime doctests verify
  five canonical length domains, three general scalar domains/six kinds, all five LengthStates,
  point/edge refs/eight reserved names, zero/absent/assertion recipe positions and boundary4096.
-19 actual compiled source/kind/origin/identity/context/privacy body assertion reds plus one
  actual widened library API that compiles forbidden measurement input and fails its negative
  construction contract; producer restored exactly, script rc=0. Normal strict608/50groups pass0.
- Owned draft faults: assert(false) placeholders refused by strict lint, replaced by actual variant
  assertions; custom messages then hid standard assertion signatures. Classifier correctly refused
  those reds; default body messages restored, classifier unchanged. D130 five-domain generic input
  was overbroad; distinct scalar type prevents measurement/Ease non-length metadata before commit.
  Borrow-discard fault retargeted to actual cloned-record cache after the new type boundary made
  its old replacement ill-typed; pointer identity catches the compiled clone, not compiler failure.
- Reference structural/language16/publication9 terminal0:55 chapters/33 APIs/1137 source links/
  1765 rendered links, warning-free. Reference evidence remains separate from product acceptance.
- Exact cfd0748 task26lines/2390B/29b1b16a receipts,21lines/1781B/288041bd protocol and20lines/
  1601B/04dd4b10 checklist retained in constructions. Oldest ledger11lines/974B/aad9b836 and
  lesson11lines/932B/2d6e66b8 match HEAD; D130 eight-line/772B report5f6b8b11 retained in part53.
- Archive221 logical/35 resident records and fresh reconstruction10open/119sealed/disjoint pass0;
  tree10lanes/13trees/10siblings/zero gaps. Ledger/WASM/staged-gate final receipts follow.
- WASM all three libraries compile, rc=0; native608/50groups includes all five construction/
  privacy/lifetime negative contracts. Source/state/availability methods were not used for kind.
  Final tree/archive/defect predecessor checks pass0; ledger9/13 pointer controls terminal0.
  Staged doctrine gate follows; all current verification jobs observed terminal.
- Staged make gate:13 checks/all green, rc=0; final diff check0. Metadata/source predicates and
  closed API type boundary remain restored; pre-commit hook repeats the staged gate.

## Completed declaration checklist — preserved from db19b90

### G1-SLICE.5b.2b — immutable sourced declarations

- [x] **REPRODUCE / ISSUE** — initial seven public draft contracts accept generic non-length
  measurement/Ease metadata (D130), contradicting canonical length-target contracts; rc=0.
- [x] **ROOT CAUSE (WHY + WHERE)** — generic origin/kind matrix overgeneralizes reference metadata
  into canonical adapters. Current formula_declaration_contract →7 tests/0 failed, rc=0; five
  negative construction/lifetime/private doctests pass in make check. Widening the actual scalar
  API makes the forbidden measurement call compile and its negative contract fail, rc=101.
- [x] **FIX** — distinct five length/three general scalar source domains, borrowed canonical
  length records and typed refs; recipe metadata comes only from actual normalized let positions.
- [x] **ADDRESSED (verified)** — declaration_mutations.py →19 actual compiled body assertion
  reds plus one actual widened negative construction contract red; exact source restored, rc=0.
  Seven normal public contracts preserve all identities/origins/kinds/refs/ordinals/spans/privacy.
- [x] **NO REGRESSION** — strict make check →608 passes/50 groups, rc=0; full reference/structural,
  language16/publication9 pass, rc=0. WASM and final recording-gate receipts follow in recipes.
- [x] **LOCKSTEP** — declaration annex/source limits/examples/33 API rows, README and live/task/
  resume records align;55 chapters/1137 source/1765 rendered links verified, rc=0. D130 fixed
  before commit; exact prior task/ledger/lesson/report payloads retained, no ceilings raised.
  promotion: declined (canonical single-source ownership, typed metadata and original-context rules).

## Completed reserved diagnostic proposal protocol — preserved from 862c0d9

- ID: `G1-SLICE.5b.2c.1a`
  Status: `done`
  Goal: reproduce the reserved-name diagnostic mismatch before choosing product error arguments.
  Work unit: `STITCHCAD-G1-0085`; clean predecessor db19b90, no pending jobs or user edits.
  Protocol: compare contract3.1/5.2, static annex and actual reference namespace/static_statement
  entry points. Record real name/origin/message/arguments; no invented statement index. Propose a
  source-aware formula_rebinding schema, retain the closed tokens and current grammar. Obtain the
  director's decision before changing canonical semantics or implementing the blocked namespace.
  Own focused reproducibility tool/negative controls, book/live/task alignment and exact completed
  task/oldest ledger/lesson retention within existing ceilings. Product Rust remains unchanged.
  Acceptance: durable reproduction, explicit proposed required arguments and the precise decision
  blocker; complete documentation/reproduction leaf committed before handing off the ruling.
  Verification:121 actual cases/three compiled body assertion reds; source unchanged; focused
  structural/language/publication/ledger/archive checks pass0. Commit: `STITCHCAD-G1-0085`.

## Completed reserved diagnostic proposal receipts — preserved from 862c0d9

## Reserved diagnostic proposal receipts — .5b.2c.1a,2026-10-03 (UTC)

- reserved_diagnostic_review.py --mutations →120 reserved refusals/one ordinary rebinding/three
  actual compiled token/fabricated-index assertion reds, rc=0. Prefix source unchanged; no product
  diagnostic or repaired-argument acceptance. Canonical row/actual empty dictionaries reproduce D131.
- Full structural runner including new watched producer passes0; language16/publication9/ledger9
  and13 actual pointer controls pass0. Warning-free book55 chapters/33 APIs/1138 source/1767
  rendered links. git diff HEAD -- crates Cargo.toml Cargo.lock →empty, rc=0; no Rust change.
- Exact db19b90 declaration protocol29lines/2647B/7ea9f42a, receipts31lines/2836B/c1fc8b5c,
  checklist19lines/1700B/91e75ce2 retained in constructions; original routes remain live.
  Oldest ledger12lines/1035B/451e0450 and lesson13lines/1139B/c79e3803 sealed byte-exact to HEAD.
- Archive verify-retention/materialize →223 logical/37 resident records, rc=0; fresh canonical
  defect census11open/119unique sealed/no overlap, rc=0. Tree10lanes/13trees/10siblings/zero gaps.
  No existing sealed payload, registry cap, token, grammar or product implementation changed.
- D131 owned at .2c.1b for director's concrete schema ruling/repair, then .2c.2 namespace code.
  Proposal/reproduction .1a complete; neither unresolved defect nor product namespace marked done.
  Final staged doctrine receipt follows; all focused jobs observed terminal0.
- Staged make gate →13 checks/all green, rc=0. Final publication after heading placement still
  55/33/1138/1767 and9 pass/0 fail, rc=0; diff check0. Pre-commit repeats the staged doctrine gate.

## Completed reserved diagnostic proposal checklist — preserved from 862c0d9

### G1-SLICE.5b.2c.1a — reserved diagnostic conflict reproduction/proposal

- [x] **REPRODUCE / ISSUE** — reserved_diagnostic_review.py --mutations →120 reserved refusals/
  one ordinary rebinding/three actual assertion reds, rc=0. Canonical row requires two indices;
  actual eps_num initial/let refusals expose formula_rebinding and empty arguments.
- [x] **ROOT CAUSE (WHY + WHERE)** — contract3.1 reserved prohibition and static reference guards
  use rebinding, but contract5.2 raised-when/required-arguments row only describes repeated lets.
  Actual reproduction120/one and three compiled token/fabricated-index body reds, rc=0. Reserved
  sources have no prior statement; initial declarations have no attempt statement to index.
- [x] **ADDRESSED (verified)** — reproduction/proposal leaf only: actual121 cases/three assertion
  reds and unchanged producer bytes, rc=0. D131 remains open, owned and scheduled at .1b after
  director ruling; no schema change, invented context, namespace implementation or defect closure.
- [x] **NO REGRESSION** — full structural runner including121/three new controls, language16,
  publication9 and ledger9/13 pointer controls pass, rc=0. Book55 chapters/33 APIs/1138 source/
  1767 rendered links; archive223 logical/37 resident records and Rust diff unchanged, rc=0.
  Final staged doctrine receipt follows in recipes before commit.
- [x] **LOCKSTEP** — book marks argument-proof boundary and pending proposal; task/ADR/live/resume
  route .1b next. Prior task/ledger/lesson payloads retained exactly, no ceiling raised.
  promotion: declined (pending director decision; existing truthful-context doctrine).

## Completed static signature checklist — preserved from 862c0d9

### `G1-SLICE.5b.1a` — complete finite static signature evidence

- [x] **REPRODUCE / ISSUE** — direct actual reference baseline →min/max(length) refuse
  formula_dimension; within(..., size_index/size_count/is_base_size) accepts boolean, rc=0.
  New static_signature_contract.py first run →actual min(v_length) body assertion red, rc=1.
- [x] **ROOT CAUSE (WHY + WHERE)** — infer_call admits all reserved names while _matches skips
  tolerance kind; variadic _matches imposes max(2, listed arity). Actual prefix/call-path review
  and direct baseline rc=0 pinpoint those guards against grammar5/6 and ADR five-class rule.
  static_signature_contract.py --mutations →12 actual predicate faults fail body assertions, rc=0.
- [x] **FIX** — within guard admits exactly five tolerance names; min/max use the documented
  one-kind minimum. Independent hardcoded dimensional tables compare actual loaded populations
  in both directions; signature matrix traps environmental reads and execution callbacks.
- [x] **ADDRESSED (verified)** — static_signature_contract.py --mutations →4032 actual
  parse/infer cases,22 closed names,12 compiled actual predicate/body assertion reds, rc=0.
  Each faulty guard fails the assertion; producer on disk unchanged. D112/D113 close; no new
  grammar/identity/storage or product type/evaluation implementation claim.
- [x] **NO REGRESSION** — run_formula_structure_probes.sh →all existing structural/input/
  canonical/numeric/replay controls plus4032/12 green; language probes →16 pass/0 fail, rc=0.
  Publication →53 chapters/25 APIs/1102 source/1705 rendered links,9 pass/0 fail; bash -n
  on both shell producers →rc=0. Focused cargo test -p sc-core expression/canonical/recipe
  contracts →34 pass/3 groups; staged make gate →all doctrines green, rc=0.
- [x] **LOCKSTEP** — expert annex/grammar/type contract/index/status, complete .1a–.1c ownership,
  defect closure/history/live/resume agree; Rust sources/tests unchanged. G1 stays5/18;
  canonical defect marker census →10 open/102 unique sealed/0 intersection, rc=0. First scratch
  census incorrectly counted task headings; canonical markers corrected the query and draft
  defect markers before final sealing. Original report bodies/oldest ledger payloads retained.
  promotion: declined (already canonical independent-evidence doctrine; no new durable policy).

Final staged gate:all doctrines green, rc=0. Ledger:nine arms/thirteen pointer controls pass, rc=0.
Archive CLI:198 controls/189 full logical reads, rc=0; retention189 logical records/64 working
Markdown/9092 decoded lines/700909 decodedB/357231 residentB, rc=0. Prior main milestone checklist
1474B and recipe completed prefix47500B retained exactly; oldest live ledger payloads exact HEAD.
The first gate selected the primary tree's historical checklist lacking a recognized result shape;
added current owning .1a evidence before it, leaving historical bytes unchanged. Matrix guard review
replaced a kind-only dict with Mapping so get(value) cannot silently default; actual twelfth fault
proves the read trap. Draft count replacement was corrected for D112/D113 and1102 link identities
before final checks. No checker, cap, grammar or product Rust changed; no needed job remains live.

## Completed binding-source decision protocol — preserved from 1972f57

- ID: `G1-SLICE.5b.2c.1b`
  Status: `done`
  Goal: apply the delegated D131 diagnostic decision to contract/reference and independent argument
  controls before product namespace implementation. Acceptance: no fabricated indices, all eight
  reserved names and every originating declaration domain covered; ordinary rebinding retains
  both actual recipe ordinals. Director2026-10-03 delegates the choice and application to the
  engineer; same-party author/applier disclosed, independent approval remains unclaimed.
  Work unit: `STITCHCAD-G1-0086`; predecessor862c0d9 clean, no owned jobs or user edits.
  Pre-code protocol: contract3.1/4.1/5.2, actual namespace/static_statement/preflight and all
  existing static controls read. Keep formula_rebinding for reserved names; add a discriminated
  reserved refusal carrying name/reserved kind-origin-context and attempted metadata locator.
  Initial reference declarations carry their actual ordered-pair position, not a recipe index.
  Detached recipe statements carry actual local spans but no invented ordinal. Whole preflight
  supplies global attempted ordinal/spans and the real prior binding's ordinal/spans for repeated
  lets. Reference metadata locators certify no canonical record/value identity; product sources
  remain the immutable typed declarations and later registry validation. No state/value reads.
  Independently authored all8 names×9 metadata origins and8×6 annotations, every real recipe
  position/name span, actual repeated indices/assertion gaps; forbidden availability/execution
  reads trapped. Actual argument/token/context faults must fail body assertions with source exact.
  Repair canonical row/book/ADR, disclose reopening/reversal and approval scope; no grammar change.
  Own exact completed .1a blocks/oldest ledgers/lesson/report retention within current bounds.
  Verification:3624 exact argument cases/19 actual compiled assertion reds; full reference,
  language16/publication9/ledger9/13 controls pass0. Commit: `STITCHCAD-G1-0086`.

## Completed binding-source decision receipts — preserved from 1972f57

## Delegated diagnostic source receipts — .5b.2c.1b,2026-10-03 (UTC)

- reserved_diagnostic_review.py --mutations →3624 exact argument cases/19 actual compiled body
  assertion reds, rc=0; actual producer unchanged. State/value/availability/execution read traps
  cover direct namespace/detached/runtime-static/whole preflight. No canonical record IDs invented.
- Invalid draft cm2/true fixtures refused by actual grammar; corrected to square/comparison forms
  before acceptance. Two original guard fault anchors remain; whole execution fault anchor follows
  the new private static composition, preserving actual assertion classification and scope.
- Full structural/signature4032/namespace1139/whole196/review100 and all prior runtime controls
  pass0; language16/publication9/ledger9/13 pointer controls terminal0. Book warning-free55
  chapters/33 API rows/1140 source/1770 rendered links. Rust diff to HEAD empty, rc=0.
- Exact .1a protocol14lines/1195B/3e442bf4, receipts19lines/1698B/2ebc780c and checklist19lines/
  1667B/6b1d9137 retained in constructions. Older signature37lines/3281B/eb496c6e and whole
  preflight20lines/1705B/98f57f67 retained with old anchor routes. No historical bytes rewritten.
- Oldest ledger12lines/1076B/61590cec, lesson14lines/1110B/f1ad9ba4 and original D131 report
  14lines/1255B/c94b9f9c sealed exactly to862c0d9 payloads; existing caps/windows unchanged.
- Same author/applier under explicit delegation disclosed in ADR-0003; reopened by director or
  independent reviewer on a concrete counterexample, approval unclaimed. Product namespace .2c.2
  remains next; canonical source registry/localization/order/type integration retain their owners.
  Final archive/census/staged doctrine receipts follow; no scoped verification job remains live.
- Final archive226 logical/40 resident records and fresh defect10open/120unique sealed/no overlap,
  tree10lanes/13trees/10siblings/zero gaps, rc=0. All sealed originals compare exactly to HEAD;
  shell syntax/diff checks0. Staged doctrine receipt follows before commit.
- Initial staged gate refuses ADR-0003 at17738B over16384B, rc=2. Split the complete D131
  proposal/decision into decision_rebinding.md, retain ADR forwarder and index; shorten map
  orientation inputs without dropping entry paths/owners. No cap or gate weakened; rerun follows.
- Final staged gate →13 checks/all green, rc=0; bounded ADR14095B/new record4533B/map8163B.
  New-record terminal blank removed after diff check; final staged diff check repeats before commit.

### G1-SLICE.5f.1.c.join.outputs.contours.tracking

- [x] ROOT CAUSE:Git interval census:17893B completed child-goal block;15 closed nodes,rc=0.
  Whole original959 source intervals/body/hash/public reader exact;no old requirement text discarded.
- [x] ADDRESSED:same Git comparison:every ID/status/child relation and all later acceptance bytes
  identical,rc=0. Only closed goal details become pointers;current native join implementation unchanged.
- [x] NO REGRESSION:`run_tree_coverage_census.sh`:10 lanes/13 trees/20 siblings/0 issues,rc=0;
  `history_archive.py --root . verify`:1413 logical/46 working,rc=0;source5/output7 doctests and book0.
  Full feature/integration verification remains G1-SLICE.5f.4/.5g/pre-push,not run for this retention.

## Join physical-copy correspondence

- ID: `G1-SLICE.5f.1.c.join.outputs.copies.correspondence`
  Status: `done`
  Goal:after bounded source/output/identity contexts,declare complete original-copy actions and
  explicit A/B-to-result correspondence;no silent pairing,discarded source quantities or new ids.
- ID: `G1-SLICE.5f.1.c.join.outputs.copies.correspondence.results`
  Status: `done`
  Goal:0795 explicit result-copy requests;[whole scope/proofs](../history/window29.md#stitchcad-devnotes-part971md).
- ID: `G1-SLICE.5f.1.c.join.outputs.copies.correspondence.actions`
  Status: `done`
  Goal:0796 complete original-copy actions;[whole scope/proofs](../history/window29.md#stitchcad-devnotes-part972md).
### G1-SLICE.5f.1.c.join.outputs.copies.correspondence.pairs
- ID: `G1-SLICE.5f.1.c.join.outputs.copies.correspondence.pairs`
  Status: `done`
  Goal:0797 original paired-copy correspondence;[whole scope/proofs](../history/stitchcad-devnotes-part1017.md).

### G1-SLICE.5f.1.c.join.outputs.copies.inventory.tracking — retention evidence

[Whole0798 retention proof](../history/stitchcad-devnotes-part1025.md) remains byte-exact.

## Complete join candidate inventory

- ID: `G1-SLICE.5f.1.c.join.outputs.copies`
  Status: `done`
  Goal:complete original-copy consumption/preservation/result correspondence and explicit multiplicity.
- ID: `G1-SLICE.5f.1.c.join.outputs.copies.population`
  Status: `done`
  Goal:0776 explicit cut population;[whole scope/proofs](../history/window29.md#stitchcad-devnotes-part952md).
[Complete native copy-correspondence branch](G1-SLICE-constructions.md#join-physical-copy-correspondence)
retains all original goals,statuses and focused acceptance evidence.

- ID: `G1-SLICE.5f.1.c.join.outputs.copies.inventory`
  Status: `done`
  Goal:reconcile actual result/source metadata and candidate full inventory/CutPlan with declared
  copy actions/population/identity,including remaining source copies and all unrelated members.
- ID: `G1-SLICE.5f.1.c.join.outputs.copies.inventory.plan`
  Status: `done`
  Goal:0799 actual canonical candidate plan;[whole scope/proofs](../history/window29.md#stitchcad-devnotes-part975md).
- ID: `G1-SLICE.5f.1.c.join.outputs.copies.inventory.fields`
  Status: `done`
  Goal:0800 whole inventory field resources;[whole scope/proofs](../history/window29.md#stitchcad-devnotes-part976md).
- ID: `G1-SLICE.5f.1.c.join.outputs.copies.inventory.originals`
  Status: `done`
  Goal:preserve remaining original fields except derived quantity and explicit guarded companion repairs;
  canonical reciprocal mates cannot silently keep retired source identities.
- ID: `G1-SLICE.5f.1.c.join.outputs.copies.inventory.originals.tracking`
  Status: `done`
  Goal:0801 bounded inventory reference;[whole scope/proofs](../history/window29.md#stitchcad-devnotes-part977md).
- ID: `G1-SLICE.5f.1.c.join.outputs.copies.inventory.originals.companions`
  Status: `done`
  Goal:0802 explicit retained companion requirements;[whole scope/proofs](../history/window29.md#stitchcad-devnotes-part978md).
- ID: `G1-SLICE.5f.1.c.join.outputs.copies.inventory.originals.fields`
  Status: `done`
  Goal:0803 exact original preservation;[whole scope/proofs](../history/window29.md#stitchcad-devnotes-part979md).
- ID: `G1-SLICE.5f.1.c.join.outputs.copies.inventory.result`
  Status: `done`
  Goal:0804 actual joined-result fields;[whole scope/proofs](../history/stitchcad-devnotes-part999.md).
- ID: `G1-SLICE.5f.1.c.join.outputs.copies.inventory.tracking`
  Status: `done`
  Goal:0798 bounded inventory continuity;[whole scope/proofs](../history/window29.md#stitchcad-devnotes-part974md).
- ID: `G1-SLICE.5f.1.c.join.outputs.copies.tracking`
  Status: `done`
  Goal:0775 bounded continuity;[whole scope/proofs](../history/window29.md#stitchcad-devnotes-part951md).

[Whole0805 retention evidence](../history/stitchcad-devnotes-part999.md) remains byte-exact.

## Native declaration search checkpoint

- ID: `G1-SLICE.5f.1.book-search`
  Status: `done`
  Goal:P2 D372 after typed v1 feature completion:resolve new large-index build warning;
  payload10015553B/JS10030347B. Preserve full-text/chapter search;no latency claim,cap raise or warning suppression.
- ID: `G1-SLICE.5f.1.book-search.plan`
  Status: `done`
  Goal:0814 upstream field leakage census and bounded remedy;[whole scope/proofs](../history/window29.md#stitchcad-devnotes-part990md).
- ID: `G1-SLICE.5f.1.book-search.build`
  Status: `done`
  Goal:0815 published local renderer/native field correction;[whole scope/proofs](../history/window29.md#stitchcad-devnotes-part991md).
- ID: `G1-SLICE.5f.1.book-search.activate`
  Status: `done`
  Goal:select the verified local renderer explicitly in normal build/publication/capture paths;guard tool/source paths and update installation/search docs and directly affected contracts.
  Acceptance:all full-text/chapter coverage retained;unexpected renderer warnings still refuse;no implicit dependency on ambient executable resolution.
- ID: `G1-SLICE.5f.1.book-search.activate.tool`
  Status: `done`
  Goal:0816 guarded local tool setup and D376 repair;[whole scope/proofs](../history/window29.md#stitchcad-devnotes-part992md).
- ID: `G1-SLICE.5f.1.book-search.activate.publication`
  Status: `done`
  Goal:0817 local renderer/publication activation and D377 repair;[whole scope/proofs](../history/window29.md#stitchcad-devnotes-part993md).
- ID: `G1-SLICE.5f.1.book-search.verify`
  Status: `done`
  Goal:0818 actual native search/query checkpoint and D372 closure;[whole scope/proofs](../history/window29.md#stitchcad-devnotes-part994md).

## Ordered drafting integration

- ID: `G1-SLICE.5f.2.inputs`
  Status: `done`
  Goal:0821 borrowed13-family raw input view;[whole scope/proofs](../history/window29.md#stitchcad-devnotes-part997md).
- ID: `G1-SLICE.5f.2.context.contract`
  Status: `done`
  Goal:0822 actual owner/frame/current snapshot contract;[whole scope/proofs](../history/window29.md#stitchcad-devnotes-part998md).
- ID: `G1-SLICE.5f.2.context`
  Status: `done`
  Goal:implement explicit immutable source port and bounded actual-root binding from .context.contract;retain actual frame/current typed output provenance and original causes,no fabricated geometry or planned-output liveness.
- ID: `G1-SLICE.5f.2.context.source`
  Status: `done`
  Goal:0823 immutable drafting source port;[whole scope/proofs](../history/stitchcad-devnotes-part999.md).
- ID: `G1-SLICE.5f.2.context.archive`
  Status: `done`
  Goal:0824 sealed window29 packing;[whole scope/proofs](../history/stitchcad-devnotes-part1000.md).
- ID: `G1-SLICE.5f.2.context.bind`
  Status: `done`
  Goal:0825 bounded actual context and D378 fix;[whole scope/proofs](../history/stitchcad-devnotes-part1001.md).
- ID: `G1-SLICE.5f.2.context.geometry`
  Status: `done`
  Goal:resolve exact typed existing geometry request under bound context;check returned reference/frame/snapshot against actual sources,retain original witness and cause,no value query or planned-output liveness.
- ID: `G1-SLICE.5f.2.context.geometry.sources`
  Status: `done`
  Goal:0826 original source records and D379 repair;[whole scope/proofs](../history/stitchcad-devnotes-part1002.md).
- ID: `G1-SLICE.5f.2.context.geometry.check`
  Status: `done`
  Goal:0827 exact existing-output bindings;[whole scope/proofs](../history/stitchcad-devnotes-part1003.md).
- ID: `G1-SLICE.5f.2.program`
  Status: `done`
  Goal:whole ordered scalar/drafting owner and complete logical limits;retain actual scalar recipe ordinals separately from genuine combined-program positions and original operation/document owners.
- ID: `G1-SLICE.5f.2.program.plan`
  Status: `done`
  Goal:0828 sourced six-stage programme plan;[whole scope/proofs](../history/stitchcad-devnotes-part1004.md).
- ID: `G1-SLICE.5f.2.program.recipe`
  Status: `done`
  Goal:0829 immutable complete factory recipe shape;[whole scope/proofs](../history/stitchcad-devnotes-part1005.md).
- ID: `G1-SLICE.5f.2.program.fields`
  Status: `done`
  Goal:0830 exhaustive native held-field charges;[whole scope/proofs](../history/stitchcad-devnotes-part1006.md).
- ID: `G1-SLICE.5f.2.program.steps`
  Status: `done`
  Goal:0831 actual mixed source owner;[whole scope/proofs](../history/stitchcad-devnotes-part1007.md).
- ID: `G1-SLICE.5f.2.program.admit`
  Status: `done`
  Goal:0832 complete actual mixed content/population/order admission;[whole scope/proofs](../history/stitchcad-devnotes-part1008.md).
- ID: `G1-SLICE.5f.2.program.normalize`
  Status: `done`
  Goal:0833 actual complete child normalization;[whole scope/proofs](../history/stitchcad-devnotes-part1009.md).
- ID: `G1-SLICE.5f.2.program.views`
  Status: `done`
  Goal:0834 original mixed step arena loans;[whole scope/proofs](../history/stitchcad-devnotes-part1010.md).
- ID: `G1-SLICE.5f.2.namespace`
  Status: `done`
  Goal:actual initial/always-visible and earlier-only names;check collisions and future geometry before values,using existing complete kind proofs without treating a metadata cursor as program acceptance.
- ID: `G1-SLICE.5f.2.namespace.plan`
  Status: `done`
  Goal:0835 sourced namespace stages;[whole scope/proofs](../history/stitchcad-devnotes-part1011.md).
- ID: `G1-SLICE.5f.2.namespace.scope`
  Status: `done`
  Goal:0836 borrowed visible-name/checker adapters;[whole scope/proofs and closed race](../history/stitchcad-devnotes-part1012.md).
- ID: `G1-SLICE.5f.2.namespace.inputs`
  Status: `done`
  Goal:0837 raw actual name/output sources;[whole scope/proofs](../history/stitchcad-devnotes-part1013.md).
- ID: `G1-SLICE.5f.2.namespace.admit`
  Status: `done`
  Goal:fresh complete program/name/source/catalogue held-content and prospective name-work bounds before namespace callbacks;preserve original owner/role locations and first refusal,then exact actual program associations,no accepted prefix.
- ID: `G1-SLICE.5f.2.namespace.admit.fields`
  Status: `done`
  Goal:0838 complete additional held fields;[whole scope/proofs](../history/stitchcad-devnotes-part1014.md).
- ID: `G1-SLICE.5f.2.namespace.admit.program`
  Status: `done`
  Goal:0839 complete raw/retained program fields;[whole scope/proofs](../history/stitchcad-devnotes-part1015.md).
- ID: `G1-SLICE.5f.2.namespace.admit.bounds`
  Status: `done`
  Goal:fresh cumulative full content and explicit conservative prospective name-work bounds before namespace factories;actual complete original fields,first typed refusal and checked arithmetic,no caller totals or accepted prefix.
- ID: `G1-SLICE.5f.2.namespace.admit.bounds.content`
  Status: `done`
  Goal:0840 complete cumulative program/name content;[whole scope/proofs](../history/stitchcad-devnotes-part1016.md).
- ID: `G1-SLICE.5f.2.namespace.admit.bounds.workspace`
  Status: `done`
  Goal:0841 actual complete name-work bounds;[whole scope/proofs](../history/stitchcad-devnotes-part1017.md).
- ID: `G1-SLICE.5f.2.namespace.admit.association`
  Status: `done`
  Goal:after complete bounds,bind exact original program positions/fixed schemas/typed role metadata and actual initial sources;reject mismatches/future initial geometry before values,retain full owners and original first cause in private admission.
- ID: `G1-SLICE.5f.2.namespace.admit.association.positions`
  Status: `done`
  Goal:0842 genuine output-name positions;[whole scope/proofs](../history/stitchcad-devnotes-part1018.md).
- ID: `G1-SLICE.5f.2.namespace.admit.association.roles`
  Status: `done`
  Goal:bind variable-role candidates through actual published family proof parents/manifests;Offset uses intentional shared Edge aliases with its own lineage proof obligations,no operation-id-only source truth.
- ID: `G1-SLICE.5f.2.namespace.admit.association.roles.offset`
  Status: `done`
  Goal:0843 Offset/shared Edge aliases and Issue0843 closure;[whole scope/proofs](../history/stitchcad-devnotes-part1019.md).
- ID: `G1-SLICE.5f.2.namespace.admit.association.roles.bind`
  Status: `done`
  Goal:actual published variable-role proof parents under original Root/context,exact schema/catalogue membership and first typed causes before namespace visibility,no fabricated bound family or matching-id-only source grant.
- ID: `G1-SLICE.5f.2.namespace.admit.association.roles.bind.sources`
  Status: `done`
  Goal:0844 authentic planned fields/source facet;[whole scope/proofs](../history/stitchcad-devnotes-part1020.md).
- ID: `G1-SLICE.5f.2.namespace.admit.association.roles.bind.check`
  Status: `done`
  Goal:0845 complete actual role sources;[whole scope/proofs](../history/stitchcad-devnotes-part1021.md).
- ID: `G1-SLICE.5f.2.namespace.admit.association.initial`
  Status: `done`
  Goal:bind actual extra initial sources to the original Root canonical/source/context records,refuse planned creators as initial geometry before values;retain exact original sources and causes,no metadata-id-only truth.
- ID: `G1-SLICE.5f.2.namespace.admit.association.initial.bounds`
  Status: `done`
  Goal:0846 complete initial-binding record bounds;[whole scope/proofs](../history/stitchcad-devnotes-part1022.md).
- ID: `G1-SLICE.5f.2.namespace.admit.association.initial.sources`
  Status: `done`
  Goal:0847 actual initial metadata source loans;[whole scope/proofs](../history/stitchcad-devnotes-part1023.md).
- ID: `G1-SLICE.5f.2.namespace.admit.association.initial.check`
  Status: `done`
  Goal:0848 complete initial associations;[whole scope/proofs](../history/stitchcad-devnotes-part1024.md).
- ID: `G1-SLICE.5f.2.namespace.order`
  Status: `done`
  Goal:actual Root and explicit always-visible names plus only earlier scalar/native declarations in mixed order;check reserved/cross-origin/repeated headers and self/future names without values or metadata-prefix acceptance.
- ID: `G1-SLICE.5f.2.namespace.order.initial`
  Status: `done`
  Goal:0849 actual private initial namespace;[whole scope/proofs](../history/stitchcad-devnotes-part1025.md).
- ID: `G1-SLICE.5f.2.namespace.order.mixed`
  Status: `done`
  Goal:earlier-only genuine mixed positions and scalar ordinals;check original headers/operands before exposing each actual output,no self/future name or whole acceptance from metadata alone.
- ID: `G1-SLICE.5f.2.namespace.order.mixed.bounds`
  Status: `done`
  Goal:0850 complete actual order-checker work;[whole scope/proofs](../history/stitchcad-devnotes-part1026.md).
- ID: `G1-SLICE.5f.2.namespace.order.mixed.check`
  Status: `done`
  Goal:0851 complete mixed name/kind order;[whole scope/proofs](../history/stitchcad-devnotes-part1027.md).
- ID: `G1-SLICE.5f.2.dependencies`
  Status: `in_progress`
  Goal:all original formula/geometry/semantic dependencies,source frame/current registry/output provenance;canonical Pocket kind/domain/state and Buttonhole button-size dependency,no separately authored hole length or approximation.
- ID: `G1-SLICE.5f.2.dependencies.plan`
  Status: `done`
  Goal:0852 sourced complete dependency stages;[whole scope/proofs](../history/stitchcad-devnotes-part1028.md).
- ID: `G1-SLICE.5f.2.dependencies.sources`
  Status: `done`
  Goal:0853 actual mixed Root source proof bridge;[whole scope/proofs](../history/stitchcad-devnotes-part1029.md).
- ID: `G1-SLICE.5f.2.dependencies.canonical`
  Status: `done`
  Goal:all original mixed canonical/Derived dependencies and actual source graph,kind/state/domain and cycle obligations before values,no id-only targets.
- ID: `G1-SLICE.5f.2.dependencies.canonical.owners`
  Status: `done`
  Goal:0854 exact dependency proof owners;[whole scope/proofs](../history/stitchcad-devnotes-part1030.md).
- ID: `G1-SLICE.5f.2.dependencies.canonical.occurrences`
  Status: `done`
  Goal:0855 original mixed dependency stream;[whole scope/proofs](../history/stitchcad-devnotes-part1031.md).
- ID: `G1-SLICE.5f.2.dependencies.canonical.bind`
  Status: `done`
  Goal:0856 bounded complete canonical bindings;[whole scope/proofs](../history/stitchcad-devnotes-part1032.md).
- ID: `G1-SLICE.5f.2.dependencies.geometry`
  Status: `done`
  Goal:all actual native/selector geometry references under original frame/snapshot and strict earlier creators;unlabelled planned roles need authentic manifests,never live-future lookup or synthetic names.
- ID: `G1-SLICE.5f.2.dependencies.geometry.formulas`
  Status: `done`
  Goal:0857 original formula geometry uses;[whole scope/proofs](../history/stitchcad-devnotes-part1033.md).
- ID: `G1-SLICE.5f.2.dependencies.geometry.native`
  Status: `done`
  Goal:complete direct geometry fields/selectors across all13 native families,then genuine mixed consumers;semantic-target geometry still requires actual Root-bound sources,no empty/id-only substitutes.
- ID: `G1-SLICE.5f.2.dependencies.geometry.native.fields`
  Status: `done`
  Goal:0858 direct original native geometry fields;[whole scope/proofs](../history/stitchcad-devnotes-part1034.md).
- ID: `G1-SLICE.5f.2.dependencies.geometry.native.mixed`
  Status: `done`
  Goal:0861 original native fields under genuine mixed consumers;[whole scope/proofs](../history/stitchcad-devnotes-part1035.md).
- ID: `G1-SLICE.5f.2.dependencies.geometry.requests`
  Status: `done`
  Goal:genuine earlier creating-step/reference request and authentic planned-family port,including unnamed outputs;classify all creators,refuse ambiguous/self/future before live lookup;no global name manufacture.
- ID: `G1-SLICE.5f.2.dependencies.geometry.requests.classify`
  Status: `done`
  Goal:0862 actual creator classification and genuine earlier-step requests;[whole scope/proofs](../history/stitchcad-devnotes-part1036.md).
- ID: `G1-SLICE.5f.2.dependencies.geometry.requests.port`
  Status: `done`
  Goal:0863 authentic unnamed planned-source facet;[whole scope/proofs](../history/stitchcad-devnotes-part1037.md).
- ID: `G1-SLICE.5f.2.dependencies.geometry.bounds`
  Status: `done`
  Goal:0864 complete fresh geometry work admission;[whole scope/proofs](../history/stitchcad-devnotes-part1038.md).
- ID: `G1-SLICE.5f.2.dependencies.geometry.bind`
  Status: `done`
  Goal:0867 complete static geometry suppliers;[whole scope/proofs](../history/stitchcad-devnotes-part1040.md).
- ID: `G1-SLICE.5f.2.dependencies.geometry.bind.book-capacity`
  Status: `done`
  Goal:0865 full book exemption;[whole scope/proofs](../history/stitchcad-devnotes-part1039.md).
- ID: `G1-SLICE.5f.2.dependencies.geometry.bind.book-capacity.task-guards`
  Status: `done`
  Goal:0866 independent task/history guards;[whole scope/proofs](../history/stitchcad-devnotes-part1039.md).
- ID: `G1-SLICE.5f.2.dependencies.semantic`
  Status: `in_progress`
  Goal:actual Root-bound garment/native semantic sources,including Pocket opening kind/domain/state/component recipe and Buttonhole canonical button/derivation;no empty substitute or authored hole length.
- ID: `G1-SLICE.5f.2.dependencies.semantic.sources`
  Status: `done`
  Goal:0868 actual garment source and fresh content admission;[whole scope/proofs](../history/stitchcad-devnotes-part1041.md).
- ID: `G1-SLICE.5f.2.dependencies.semantic.logical.sources`
  Status: `in_progress`
  Goal:actual readonly logical/Profile/operation source records for Pocket openings and canonical Buttonhole button/derivation,including original kind/domain/state and component recipe sources;no id-only or empty replacement. Exhaustively charge new held records before bounded core queries.
  Children: `.contract` (actual source/model boundary), `.model` (immutable logical domain/state),
  `.operations` (actual typed semantic operation dependencies), `.ports` (original source facets).
- ID: `G1-SLICE.5f.2.dependencies.semantic.logical.sources.contract`
  Status: `done`
  Goal:0869 actual logical model/recipe source contract;[whole scope/proofs](../history/stitchcad-devnotes-part1042.md).
- ID: `G1-SLICE.5f.2.dependencies.semantic.logical.sources.model`
  Status: `done`
  Goal:immutable canonical logical declarations with actual authored domain/state/provenance,
  explicit complete bounds and exhaustive field accounting;Unknown/Derived have no fallback.
- ID: `G1-SLICE.5f.2.dependencies.semantic.logical.sources.model.declarations`
  Status: `done`
  Goal:0870 immutable finite logical declarations;[whole scope/proofs](../history/stitchcad-devnotes-part1043.md).
- ID: `G1-SLICE.5f.2.dependencies.semantic.logical.sources.model.inventory`
  Status: `done`
  Goal:actual logical inventory membership and disjoint identities,then atomic original Root/resource integration;numeric-only scopes stay explicitly empty,never defaulted logical state.
  Children: `.component` (standalone original owner), `.attach` (actual Root plus complete producers).
- ID: `G1-SLICE.5f.2.dependencies.semantic.logical.sources.model.resources`
  Status: `done`
  Goal:0872 exhaustive original logical content;[whole scope/proofs](../history/stitchcad-devnotes-part1045.md).
- ID: `G1-SLICE.5f.2.dependencies.semantic.logical.sources.model.inventory.component`
  Status: `done`
  Goal:0871 original logical membership/kind loans;[whole scope/proofs](../history/stitchcad-devnotes-part1044.md).
- ID: `G1-SLICE.5f.2.dependencies.semantic.logical.sources.model.inventory.attach`
  Status: `done`
  Goal:0873 actual canonical logical attachment;[whole scope/proofs](../history/stitchcad-devnotes-part1046.md).
- ID: `G1-SLICE.5f.2.dependencies.semantic.logical.sources.operations`
  Status: `in_progress`
  Goal:actual typed semantic derivation/component-recipe sources under original ownership;
  canonical button declaration is the dependency,no independent hole length or invented native step.
  Children: `.buttonhole` (immutable bounded operation), `.logical` (logical derivation sources),
  `.components` (genuine creating/import origins), `.inventory` (original ordered owner), `.attach` (Root/program/accounting integration).
- ID: `G1-SLICE.5f.2.dependencies.semantic.logical.sources.operations.buttonhole`
  Status: `done`
  Goal:0874 standalone bounded Buttonhole recipe;[whole scope/proofs](../history/stitchcad-devnotes-part1047.md).
- ID: `G1-SLICE.5f.2.dependencies.semantic.logical.sources.operations.logical`
  Status: `done`
  Goal:0875 standalone original typed logical derivations;[whole scope/proofs](../history/stitchcad-devnotes-part1048.md).
- ID: `G1-SLICE.5f.2.dependencies.semantic.logical.sources.operations.components`
  Status: `in_progress`
  Goal:authentic component-creation recipe and explicit imported primitive origins;no fabricated native step or reconstructed creation history.
  Children: `.carriers` (original native Piece identity/recipe or imported source loans),
  `.bind` (bounded authentic Root/program/snapshot/source association after source ports exist).
- ID: `G1-SLICE.5f.2.dependencies.semantic.logical.sources.operations.components.carriers`
  Status: `done`
  Goal:0876 raw actual component source loans;[whole scope/proofs](../history/stitchcad-devnotes-part1049.md).
- ID: `G1-SLICE.5f.2.dependencies.semantic.logical.sources.operations.components.bind`
  Status: `pending`
  Goal:after source ports/accounting,bind complete original component origins to actual Root Piece/copy,recipe owner/native creator and frame/snapshot under explicit bounds;typed original causes,no historical reconstruction or prefix acceptance.
- ID: `G1-SLICE.5f.2.dependencies.semantic.logical.sources.operations.inventory`
  Status: `done`
  Goal:complete immutable original semantic operation order/membership with explicit bounds and first duplicate causes;typed original operation positions remain distinct from scalar/native positions.
  - [x] REPRODUCE / ISSUE:0876 held standalone semantic declarations and raw component origins,but no immutable original semantic sequence/member owner. Actual Root/source association requires this owner before any mixed ordinal can be admitted.
  - [x] ROOT CAUSE: `cargo test -p sc-core --test semantic_recipe_contract`:3 pass,rc=0;original semantic operation IDs/kinds and whole owned sequence need a separate owner. Referenced rule/output/input IDs are not operation declarations or mixed-program positions.
  - [x] FIX:SemanticRecipe moves exact original typed operation vector;complete N/N*N arithmetic and count/comparison quotas precede original duplicate scans without workspace allocation. Private member/kind loans retain actual recipe/operation/semantic index;no source/value or rule execution.
  - [x] ADDRESSED: same focused command3 pass,rc=0;original buffers/order,exact3/9 and short quotas,cross-family/identical duplicates,missing/kind causes,equal-data foreign owner/member pointers and empty0/0. Exhaustive actual producer gives6/24/0 and prior8/27/1;four exact first-field refusals and prior latch remain original.
  - [x] NO REGRESSION: `cargo clippy -p sc-core --lib --test semantic_recipe_contract -- -D warnings`:pass,rc=0;fmt0,`cargo test -p sc-core --doc recipe::semantic_recipe`:3 pass0 and recipe::semantic_examples9 pass0. Guarded book build0;coupled .5f.2.verify deferred.
  - [x] LOCKSTEP:semantic reference documents source sequence/kind/member/limits/complete resource fields with compiled examples;implementation plan reflects standalone owner only. README reviewed unchanged,LIVE note updated,G1still6/18;MEM/index/DEV/ledger select atomic actual Root integration,next mixed program.
  - [x] CONTINUITY:1049 retains whole0876,original notion acceptance and0321;transition_receipts.py reports5 exact Git spans0 and public reader byte-exact0. No retained-window or ceiling changes.
  - [x] PROMOTION: promotion: declined (existing original ownership/complete bounds/exhaustive actual accounting;book owns semantic owner API). Root/mixed/source/rule/graph/currentness and execution remain pending;no fabricated native/scalar position. Commit:STITCHCAD-G1-0877.
- ID: `G1-SLICE.5f.2.dependencies.semantic.logical.sources.operations.attach`
  Status: `pending`
  Goal:atomically attach actual ordered semantic recipe to Root/program identity,resource and order contracts;numeric-only scopes stay explicitly empty and no invented native position appears.
  Children: `.root` (whole actual Root identity/resource/data paths), `.program` (actual mixed order/admission).
- ID: `G1-SLICE.5f.2.dependencies.semantic.logical.sources.operations.attach.root`
  Status: `pending`
  Goal:actual semantic recipe field/getter and all-family Design identity census;exhaustive actual/prospective accounting and candidate reconstruction preserve the complete original cohort atomically. Legacy numeric-only fixtures supply explicit empty semantic scope;source/rule binding remains separate.
- ID: `G1-SLICE.5f.2.dependencies.semantic.logical.sources.operations.attach.program`
  Status: `pending`
  Goal:actual original Root semantic ordinals in complete mixed admission,normalization/order/dependency metadata and bounds;scalar/native positions remain authentic and semantic operations gain no fabricated geometry output/name.
- ID: `G1-SLICE.5f.2.dependencies.semantic.logical.sources.ports`
  Status: `pending`
  Goal:readonly authentic logical/Profile/operation facets with original record/Root/snapshot
  loans and typed causes;all held source fields participate in fresh accounting before queries.
  Includes original rule/policy source loans and typed signatures;operation-held IDs alone grant no complete dependency acceptance.
- ID: `G1-SLICE.5f.2.check`
  Status: `pending`
  Goal:whole owner/static-order/context/argument proof;complete logical bounds before child work,actual first causes and source fields,no accepted prefix or manufactured geometry/currentness/evidence.
- ID: `G1-SLICE.5f.2.pause-20261102`
  Status: `done`
  Goal:0859 director-requested clean pause after product9f2fca2c;record resume intent without starting native.mixed or changing code/book.
  - [x] CHECKPOINT: 0859 OS-visible `bash scripts/check_handoff.sh --idle-cua`:handoff OK/no project jobs,rc=0;9f2fca2c clean,brief0,295 ahead. Native8/strict/fmt/Rustdoc3/book proofs remain those of0858.
  - [x] RESUME: 0859 PNT paused by director;resume only on request at .dependencies.geometry.native.mixed. MEM/index/changelog updated;README/LIVE/book/DEV reviewed unchanged,no new technical lesson or product claim.
- ID: `G1-SLICE.5f.2.verify`
  Status: `pending`
  Goal:after the agreed integration set,verify coupled source/order/frame/currentness/argument/semantic/resource and allocator refusals,original causes and public proof lifetimes;actual geometry/selector execution stays .5f.3/.4.geometry/G2/G3.
