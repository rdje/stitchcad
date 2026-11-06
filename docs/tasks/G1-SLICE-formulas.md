# G1-SLICE — completed formula syntax contracts and evidence

## Journal-link transition — G1-SLICE.6.5.2.b.sources

- [x] ROOT CAUSE: native journal census7 historical nodes have done canonical copies; generated map8240B exceeds8192B, rc=0.
- [x] ADDRESSED: transition_receipts.py --record docs/history/stitchcad-devnotes-part572.md:1 exact Git span rc=0; all21 current links retain original fragments.
- [x] NO REGRESSION: task_source_contract.py11 tests OK rc=0; coverage10 lanes/13 trees/17 siblings/0 orphans and map8185B rc=0.

Bounded semantic sibling of [G1-SLICE](G1-SLICE.md). Completed syntax contracts/checklists retain
exact committed text; the current frontier and verification/commit journal remain in the parent.

## Lexical contract and evidence — preserved from 60c7305

- ID: `G1-SLICE.5a.1`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

### `G1-SLICE.5a.1` — borrowed lexical syntax with explicit parser boundaries

- [x] **REPRODUCE / ISSUE** — complete formula contract/grammar/examples and ADR read before
  lexical protocol/subleaves. No recipe module existed at predecessor 9ef9602. D73 ADR names absent
  examples/exclusions clauses. First gap contract finds D74: 12 pass/1 fail, rc=101, byte 11 refused.
- [x] **ROOT CAUSE (WHY + WHERE)** — `cargo test -p sc-core --test formula_lex_contract` →
  UnsupportedCharacter at bytes 11..12, 12 passed/1 failed, rc=101; lexical whitespace helper omits
  vertical tab. `rg -n '§10|§11' docs/decisions/decision_adr-0003-construction-recipe-and-formula-language.md`
  found two obsolete clause pointers, rc=0; main headings are §1–§9 and actual exclusions §6.
  Missing lexical front-end is a planned feature; it must borrow source and carry no evaluation authority.
- [x] **FIX** — immutable token/kind/span and typed source-private first-error/fused scanner; shared
  spelling/three-keyword classifiers retain MachineToken behavior. Full ASCII preflight precedes tokens;
  number spelling and whitespace gaps retain later parser authority. Added vertical tab, corrected ADR
  clause links, contract/privacy tests, isolated actual source mutations and indexed syntax annex.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test formula_lex_contract` → 13 passed,
  0 failed, rc=0; all 17 bindings/four assertions/13 refusal forms consumed with explicit lexical scope.
  `bash docs/tasks/artifacts/formula_lex/run_formula_lex_mutations.sh` → nine actual assertion reds,
  rc=101 each, runner rc=0; both sources restored byte-identically. Two privacy/lifetime doctests pass.
  D73 targets exist and D74 gap regression remains green; exact logged descriptions seal to part15.
- [x] **NO REGRESSION** — `CARGO_HOME="$PWD/target/cargo-home" TMPDIR="$PWD/target/scratch" make check`
  → strict lint and 454 tests including docs green, rc=0; `make wasm` → three-library cross-build, rc=0.
  Formula-language probes 15 pass/0 fail, ledger nine pass/0 fail, publication nine pass/0 fail, rc=0.
  Feature/tree/glossary/uncertainty/fixture censuses green; source/rendered publication links verified.
  Staged `make gate` → all doctrines green, rc=0. Focused checks are appropriate for this syntax
  subleaf; full milestone belongs .5a.4. No remote-CI claim.
- [x] **LOCKSTEP** — roadmap language unchanged; .5a.2/.3/.4 own parser/canonical/bounds completion.
  Learning/availability/module/package/README/status map/index and detailed API annex state actual
  lexical scope. Forty-eight chapters/15 API rows, 991 source/1506 rendered links checked. Exact
  predecessor publication contract/checklist and oldest ledger payloads preserved; no cap raised.
  D70 required axes ruling stays pending. G1 remains 5/18 top-level leaves with .4/.5 structurally partial.

## Structural reference contract and evidence — preserved from 9d26ddc

- ID: `G1-SLICE.5a.2a`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

### `G1-SLICE.5a.2a` — complete reference structural bounds and owned evidence links

- [x] **REPRODUCE / ISSUE** — real copied-book dart_count forms preserve value 1 but exceed
  256 nodes/16 if levels; old census returns rc=0, measured seven nodes/zero depth/zero mismatches.
  Full milestone initially fails the new evidence sibling's anchored link as ORPHAN.
- [x] **ROOT CAUSE (WHY + WHERE)** — copied-book diagnostic under target/formula-structural-diagnostic
  → both call_nodes/call_if_depth rc=0, although independently constructed shapes have 258 nodes/
  17 levels. count_nodes/if_depth only visited tuple children, skipping call arguments stored in lists;
  if depth was only checked in aggregate L8. `make probes` → tree 5 pass/2 fail, overall rc=2;
  `run_tree_coverage_census.sh` owner regex demands .md immediately followed by closing parenthesis,
  excluding the actual .md#fragment link. D75/D77 are instrument defects, not missing product evaluation.
- [x] **FIX** — explicit semantic children include all call arguments/if branches; iterative node/depth
  walkers count nodes without list/group/exponent payload inflation. Early measured depth refusal retains
  formula_domain and unchanged bounds. Reference controls load actual definitions, not a copied parser.
  Task owner matching permits an optional fragment with exact filename; paired target controls retain
  orphan refusals. Completed lexical contract/checklist and historical journal preserve predecessor bytes.
- [x] **ADDRESSED (verified)** — `run_formula_structure_probes.sh` → 16 pass/0 fail plus copied-book
  two pass/0 fail, rc=0: 256 nodes/16 levels accepted, 257/258 nodes and 17 levels refused with exact
  measured domain signatures. Four actual reference guard mutations yield AssertionError/rc=1 and
  byte-identical source restoration, runner rc=0. Tree coverage probes → nine pass/0 fail, rc=0;
  fragment control passes while different target and unlinked siblings refuse. D75/D77 descriptions seal.
- [x] **NO REGRESSION** — `CARGO_HOME="$PWD/target/cargo-home" TMPDIR="$PWD/target/scratch" make probes`
  → 25 suite(s) green, rc=0; reference language 15, publication nine, ledger nine, archive 28 pass;
  glossary 310 terms/9 parts/158 tokens, tree 10 lanes/13 trees/8 siblings/zero gaps. Warning-free book:
  48 chapters/15 APIs, 992 source/1508 rendered links. `git diff --name-only -- crates` is empty, rc=0;
  native/WASM behavior stays the strict 454-test baseline at 60c7305. Staged `make gate` → all
  doctrines green, rc=0, after adding journal preservation boxes and the exact decline token.
  No new runtime/remote-CI claim.
- [x] **LOCKSTEP** — node/depth proof scope and reproduction commands are in the expert syntax annex;
  learner progression, glossary/index and normative caps remain unchanged. No product expression tree,
  evaluation or arbitrary-input reference-parser safety is claimed. D76 names/unit-gap debt is owned
  next by .2b.1; D70 required axes ruling stays unanswered. G1 remains 5/18 top-level. Live defect census
  re-derived as 11 open/65 sealed; journal/oldest live payloads retain exact predecessor bytes.

## Reference input contract and evidence — preserved from e797874

- ID: `G1-SLICE.5a.2b.1`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

### `G1-SLICE.5a.2b.1` — reference machine-input parity

- [x] **REPRODUCE / ISSUE** — actual reference accepts Upper/_a/a__b, 1cm, bare/declared keywords
  and abs(); all violate canonical spelling, gap, keyword role or nonempty-argument grammar.
- [x] **ROOT CAUSE (WHY + WHERE)** — load_reference/direct parse/statement diagnostic → invalid forms
  return ASTs/bindings, rc=0. tokenize permits broad words and only checks a present sp token; parser
  roles omit keyword checks and p_args explicitly returns an empty list. This is D76 instrument debt.
- [x] **FIX** — whole-source ASCII, private captured spelling/role checks, original-source unit gap,
  nonempty arguments and all ASCII whitespace. Tests load actual reference and canonical unit table;
  paired direct/copied-book controls and isolated guard mutations restore exact bytes.
- [x] **ADDRESSED (verified)** — run_formula_structure_probes.sh → structural 16+2 and input 130+3
  pass/0 fail, rc=0. run_formula_input_mutations.sh → nine assertion reds/rc=1 each and exact
  restoration, runner rc=0. Invalid unit gaps inside assert remain refused; D76 description seals intact.
- [x] **NO REGRESSION** — run_formula_language_probes.sh → 15 pass/0 fail, rc=0; cargo test -p sc-core
  --test name_contract --test formula_lex_contract → 20 passed/0 failed, rc=0. Rust files unchanged;
  strict native/WASM baseline remains 60c7305. Publication nine and ledger nine probes pass/0 fail;
  history_archive.sh verify/verify-retention → 95 records/32 working MD/238983 resident bytes, rc=0.
  Tree 10 lanes/13 trees/8 siblings/zero gaps; glossary 310 terms/9 parts/158 tokens; feature, uncertainty
  and fixture censuses green, rc=0. Book warning-free: 48 chapters/15 APIs, 992 source/1509 rendered links.
  Staged make gate → all doctrines green, rc=0; no new runtime or remote-CI claim.
- [x] **LOCKSTEP** — expert annex/toolbox describe actual input proof scope and remaining product
  AST/canonical/checker owners. Progressive book, glossary/index, normative roadmap and limits unchanged.
  Exact completed .2a contract/checklist and oldest history payloads preserve predecessor bytes.
  Live defect census 10 open/66 sealed; G1 stays 5/18, D70 pending, next .5a.2b.2.

## Production expression contract and evidence — preserved from b681a49

- ID: `G1-SLICE.5a.2b.2`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

### `G1-SLICE.5a.2b.2` — immutable production expression syntax

- [x] **REPRODUCE / ISSUE** — prior lexer certifies tokens but no expression grammar/AST. Full
  language/grammar/examples, affected code/tests/ADR/roadmap/book read before explicit pre-code protocol.
- [x] **ROOT CAUSE (WHY + WHERE)** — git ls-tree -r --name-only e797874 -- crates/sc-core/src/recipe → lexer.rs/mod.rs only,
  rc=0 at e797874; expression parser absent, a planned feature. Grammar grouping creates no node,
  so post-recursive semantic bounds cannot prove input stack safety. Existing reference is a book
  instrument, not production arbitrary-input parsing. Typed syntax must precede canonical/evaluation work.
- [x] **FIX** — private flat borrowed arena and read-only child views; explicit parser stacks, exact
  unit gaps, precedence, three-part if/nonempty calls, measured node/depth refusal and source-private
  errors. No forgeable indices or dependency added. Existing lexical contracts remain authoritative.
- [x] **ADDRESSED (verified)** — make check → formula_expression_contract 15
  passed/0 failed, rc=0, including 20736 short inputs, 50000-level grouping on 64 KiB thread stack,
  exact 256/16 bounds and measured 257/17 refusals. run_formula_expression_mutations.sh → eleven
  assertion reds/rc=101 each, exact restoration, runner rc=0. Shared reference fixture command →
  twelve shape/count/depth controls pass, rc=0. Three privacy/source/arena lifetime doctests pass.
- [x] **NO REGRESSION** — CARGO_HOME=target/cargo-home TMPDIR=target/scratch make check → strict
  lint and 472 tests including docs green, rc=0; make wasm → three-library cross-build green, rc=0.
  Initial borrow-check and test-helper lint errors corrected before restored signoff. Structural/input
  reference controls 16+2/130+3/twelve fixtures and language 15 probes pass, rc=0. Publication nine
  probes, warning-free book: 48 chapters/16 APIs, 993 source/1512 rendered links, rc=0. Ledger nine
  probes; archive verify/verify-retention → 97 records/245151 resident bytes, rc=0. Tree 10 lanes/13
  trees/eight siblings/zero gaps; glossary 310 terms/nine parts/158 tokens; feature/uncertainty/fixture
  censuses green, rc=0. Staged make gate → all doctrines green, rc=0; no new remote-CI claim.
- [x] **LOCKSTEP** — README/package/module, learner/availability, API map and expert syntax annex
  state actual syntax scope. Glossary/index routes retained; numeric/canonical/statement/name/type/
  evaluation work stays owned and unclaimed. Completed .2b.1 and oldest ledger payloads preserve
  exact predecessor bytes; no cap changed. G1 remains 5/18, D70 pending; next .5a.3.

Further syntax work remains owned by the parent frontier.

## Rounding contract and evidence — preserved from 543dfa6

- ID: `G1-SLICE.5a.3a`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

### `G1-SLICE.5a.3a` — total extreme-magnitude public rounding

- [x] **REPRODUCE / ISSUE** — actual public-interface diagnostic catches an unwind for MIN/1;
  MAX/1 and MIN/-1 return typed Overflow, i64 MIN/1 and ±1/2 controls succeed. D78 is owned before fix.
- [x] **ROOT CAUSE (WHY + WHERE)** — rustc repo-local probe linked to actual debug sc-units, then
  target/formula-round-diagnostic/probe → MIN/1 Err(Any), controls as above, rc=0; tracked
  round_diagnostic.rs/run_round_diagnostic.sh preserve that public producer. round.rs casts
  unsigned q=2^127 to i128 MIN and negates it before checked i64 narrowing. git log --follow round.rs
  → sole introduction eb83f01 G0-CONTRACT.18, rc=0. Public i128 signature promises typed failure.
- [x] **FIX** — explicit negative i64 endpoint and checked unsigned-to-i64 conversion before sign;
  no wider quotient is cast/negated, no caller restriction or rounding/error-family change.
- [x] **ADDRESSED (verified)** — cargo test -p sc-units --test round_contract → four passed/0 failed,
  rc=0; independent round_reference.py → 36 exact Fraction rows pass, rc=0. run_round_mutations.sh
  → five actual assertion reds/rc=101 each and exact source restoration, runner rc=0. Rebuilt public
  probe → all wide cases typed Overflow, controls unchanged, no unwind, rc=0. D78 description seals.
- [x] **NO REGRESSION** — make check → strict lint/476 tests including docs green, rc=0; make wasm
  → three-library cross-build green, rc=0; cargo test -p sc-units --release --test round_contract
  → four passed/0 failed, rc=0. Structural/input/reference/Fraction controls and language 15 probes pass, rc=0; publication nine
  probes and warning-free book: 48 chapters/16 APIs, 993 source/1514 rendered links, rc=0. Ledger
  nine probes and archive verify/verify-retention → 100 records/250460 resident bytes, rc=0. Tree
  10 lanes/13 trees/eight siblings/zero gaps; glossary 310 terms/nine parts/158 tokens; feature/
  uncertainty/fixture censuses green, rc=0. Exact AST/ledger preservation and 11/67 defect census
  verified; durable public diagnostic agrees with typed results. Staged `make gate` → all doctrines green, rc=0.
  No new remote-CI, full canonical identity or product evaluation claim.
- [x] **LOCKSTEP** — expert numerical annex explains endpoint/error behavior and scoped proof;
  formula annex records D79's independently observed identity gap with immediate .3b ownership.
  Normative rounding/roadmap, learner routes, glossary/index remain unchanged. Completed AST
  contract/checklist and oldest live payloads preserve b681a49 bytes; no cap changes. Live snapshot
  11 open/67 sealed, G1 5/18, sc-units 31 tests including its doc; next .3b, D70 pending.

Current work remains in the parent frontier.

## Literal identity contract and evidence — preserved from 6f26ca3

- ID: `G1-SLICE.5a.3b.1`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

### `G1-SLICE.5a.3b.1` — reference literal quantum and canonical identity

- [x] **REPRODUCE / ISSUE** — literal_diagnostic.py actual parser/evaluator → two 0.00004 cm or
  two bare 0.0000004 literals accumulate 4/5 quantum and bind 1, unlike canonical-zero respellings;
  diagnostic rc=0 is observation only. New literal_contract.py fails on 0.49 um, assertion rc=1.
- [x] **ROOT CAUSE (WHY + WHERE)** — p_atom retains fractional unit/bare-decimal values while L1
  rounds display; canonical kind:integer identity loses hidden precision. git log --follow →
  introduction 3704b8a G0-CONTRACT.9, input/structural repairs unchanged numeric paths, rc=0.
  D80 rg headings → two 2.2 clauses; D81 LIVE_STATUS differs from committed task/MEMORY frontier.
- [x] **FIX** — once-round converted literal nodes with exact factors/ratio scaling, retain kinds;
  L1 compares actual node, no display rounding. Units public endpoint section 2.3 keeps display 2.2;
  live G1 route names the actual next .3b.2. No modulo, arithmetic folding or production API change.
- [x] **ADDRESSED (verified)** — literal_contract.py → 60 explicit rows/360 controls/0 fail, rc=0;
  independent Decimal quantum oracle agrees with all rows. run_literal_mutations.sh → six actual
  assertion reds/rc=1 and byte-identical restoration, runner rc=0. Restored structural suite runs
  those controls plus structural 16+2/input 130+3/expression twelve/Fraction 36, rc=0. D79/D80/D81
  descriptions seal unchanged; wider actual D82/D83 reproductions are owned next, not certified.
- [x] **NO REGRESSION** — language 15/publication nine probes pass, rc=0; warning-free book →
  48 chapters/16 API rows, 994 source/1515 rendered links, rc=0. Ledger nine pass, rc=0;
  archive verify/retention → 103 records/256571 resident bytes, rc=0. Tree 10 lanes/13 trees/eight
  siblings/zero gaps, glossary 310/nine/158, feature 105/29, uncertainty 133/16/zero unowned,
  fixture 20/four/five/zero mismatch; recording census producers pass, rc=0. Exact predecessor
  preservation and 12/70 defect census independently verified. Staged `make gate` → all doctrines green, rc=0.
  No Rust changed: last strict 476/release four/WASM proof belongs to .3a, not a new native claim.
- [x] **LOCKSTEP** — contract states existing canonical-input boundary, annex details conversions/
  proof gaps; source/index/glossary/learner routes preserved. Completed .3a/journals and oldest live
  payloads retain predecessor text. Live 12 open/70 sealed, G1 5/18; next .3b.2, D70 pending.

Current work remains in the parent.

## Exact arithmetic contract and evidence — preserved from f70edf7

- ID: `G1-SLICE.5a.3b.2`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

### `G1-SLICE.5a.3b.2` — exact reference arithmetic

- [x] **REPRODUCE / ISSUE** — actual literal_diagnostic.py at 6f26ca3 → division sum 2 instead of 1;
  tiny ratio square zero instead of 1/1000000, rc=0 observations. Corrected arithmetic_contract.py
  against committed predecessor → exact-result assertion failure on 1 um / 2, rc=1; candidate restored.
- [x] **ROOT CAUSE (WHY + WHERE)** — git show 3704b8a reference source → square/product/quotient/
  param_at explicitly rnd before binding; actual diagnostic/exact Fraction controls disagree with
  formula 4.2/ADR-0003, rc=0 observation. Earlier input/walker/literal repairs retain those paths.
- [x] **FIX** — preserve result-kind exact Fraction through square/product/quotient/selector; keep
  canonical integer input, explicit round_to/irrational rounding and census L2 binding quantization.
  No dimension/branch/zero-divisor/scale/geometry contract change; D83 remains owned next.
- [x] **ADDRESSED (verified)** — arithmetic_contract.py → 24 rows/100 independent Fraction cases/
  162 controls/0 fail, rc=0. run_arithmetic_mutations.sh → nine actual assertion reds/rc=1 each,
  exact source restoration, runner rc=0. Predecessor/candidate round trip confirms real regression.
- [x] **NO REGRESSION** — structural suite → structural 16+2/input 130+3/expression twelve/Fraction
  36/literal 60+360/arithmetic 24+100+162 pass, rc=0; language 15/publication nine green, rc=0.
  Warning-free book: 48 chapters/16 APIs, 995 source/1517 rendered links, rc=0. Ledger nine/archive verify/retention pass, rc=0;
  106 records/261782 resident bytes. Tree 10/13/eight/zero gaps, glossary 310/nine/158, feature 105/29,
  uncertainty 133/16/zero unowned, fixture 20/four/five/zero mismatch, rc=0. Predecessor payload/
  checklist and 11/71 defect census verified. Staged `make gate` → all doctrines green, rc=0.
  No Rust changed; strict native/release/WASM evidence remains .3a, no new runtime/remote-CI claim.
- [x] **LOCKSTEP** — exact/operator vs explicit quantization boundaries described in contract and
  annex; learner/glossary/index routes retained. Prior literal evidence/oldest ledgers preserve
  committed bytes. Live 11 open/71 sealed, G1 5/18, next .3b.3; D70 pending.

Current work remains in the parent.

## Angular contract and evidence — preserved from 1c95ea4

- ID: `G1-SLICE.5a.3b.3a.1`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

### `G1-SLICE.5a.3b.3a.1` — reference angular conversion and domain guards

- [x] **REPRODUCE / ISSUE** — literal_diagnostic.py → full-turn radius1 arc6283185, sin90=0,
  cos90=1000000, dir80537677 vs nearest80537678, tan90/-90/270=0, rc=0 observations. New angle
  contract → sin30 result assertion failure, rc=1. D84 storage conflict is logged/asked, not inferred.
- [x] **ROOT CAUSE (WHY + WHERE)** — git show 3704b8a → two pi/180 paths over internal microdegrees,
  to_true only rescales ratio; dir passes Decimal to norm_angle's int truncation, no tangent pole guard;
  source signatures verified, rc=0. Direct post-fix sweep vs normalized-binding model →6 vs0 proves D84.
- [x] **FIX** — shared direct microdegree/radian conversion preserves sign/turn/fraction; round dir
  before normalization; exact rational odd-quarter pole guard uses stable formula_domain. No binding
  modulo choice, new product API, geometry solver or changed scalar/rational cap.
- [x] **ADDRESSED (verified)** — angle_contract.py →42 rows/72 controls/0 fail, rc=0; independent
  angle_math_oracle.py →42 defined curated rows agree, rc=0. run_angle_mutations.sh →seven actual
  assertion reds/rc=1 each, exact restoration, runner rc=0. Diagnostic now returns6/1000000/0/
  80537678 and typed poles, rc=0. D85/D86/D87 seal unchanged; D84 verification/D83 guards remain owned.
- [x] **NO REGRESSION** — restored structural/input/literal/arithmetic/angle controls green, rc=0;
  language15/publication9 pass, rc=0; warning-free book48 chapters/16 APIs,998 source/1521 rendered
  links. Ledger9/archive verify/retention pass, rc=0;109 records/267226 resident
  bytes. Tree10/13/eight/zero gaps, glossary310/nine/158, feature105/29, uncertainty133/16/zero
  unowned, fixture20/four/five/zero mismatch pass, rc=0. Exact predecessor preservation and12/74
  defect census verified; independent math42 agrees. Staged make gate after bounded ruling records: all doctrines green, rc=0. No Rust changed; no new native/WASM/remote-CI verdict implied.
- [x] **LOCKSTEP** — tan domain/annex/director angle ruling and live pointers match; learner,
  glossary/index routes unchanged. Exact predecessor arithmetic/decisions/journals/oldest payloads
  retained without cap growth. Live12 open/74 sealed, G1 5/18; next.3a.2, D84 verification/D70 ruling pending.

## Rational contract and evidence — preserved from b8ed62d

- ID: `G1-SLICE.5a.3b.3a.2`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

### `G1-SLICE.5a.3b.3a.2` — reduced rational refusal and discriminating domain reasons

- [x] **REPRODUCE / ISSUE** — new rational_contract.py against predecessor accepts 129-bit literal,
  raising its oversized-value assertion, rc=1. Diagnostic previously accepted count2^128. D88 actual
  angle mutation5 removed pole guard yet contract returned0; mutation runner refused, rc=1.
- [x] **ROOT CAUSE (WHY + WHERE)** — actual see only measured; L8 reddened the final census; p_atom
  rounded without exact-width check. Old product/square/quotient see observed true-unit temporaries,
  not result-kind values. git show 1c95ea4 source-signature assertion → measurement-only see/three
  true-unit observations/no exact-input guard verified, rc=0. D88 checked only formula_domain;
  run_angle_mutations.sh exposed mutation5 contract rc=0 instead of assertion red, runner rc=1.
- [x] **FIX** — measured typed refusal in see, converted literal guards before rnd, one numeric result
  wrapper around recursive evaluation. Check reduced internal values, not unscaled temporaries;
  retain lazy branch computation. Angle refusals require exact pole/zero-vector reason. Caps unchanged.
- [x] **ADDRESSED (verified)** — rational_contract.py →61 independent Fraction controls/0 fail, rc=0;
  run_rational_mutations.sh →twelve actual assertion reds/rc1 each, exact restoration, runner0. Existing
  literal/arithmetic/angle runners →six/nine/seven actual reds/exact restore, rc=0; D88 seal unchanged.
- [x] **NO REGRESSION** — restored structural suite and language15 green, rc=0; literal60/361,
  arithmetic24/100/162, angle42/72/math42 and rational61 agree. Publication9/ledger9/archive verify/
  retention green, rc=0:48 chapters/16 APIs/998 source/1522 rendered links;112 records/272155 resident
  bytes. Tree10/13/eight/zero gaps, glossary310/nine/158, feature105/29, uncertainty133/16/zero unowned,
  fixture20/four/five/zero mismatch green, rc=0. Exact preservation/live12/sealed75/no overlap verified.
  Staged make gate → all doctrines green, rc=0; no Rust changed or new native/WASM/remote-CI claim.
- [x] **LOCKSTEP** — reference input/result rational bounds and partial scalar/signed-angle status
  match book/live/task records. Glossary/index/learner routes retained; predecessor angular contract/
  checklist/journal and oldest history preserve exact bytes. G1 5/18; live12/75; next scalar .3b.

## Length operator contract and evidence — preserved from f432d68

- ID: `G1-SLICE.5a.3b.3b.1a`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

### `G1-SLICE.5a.3b.3b.1a` — fallible public length operators

- [x] **REPRODUCE / ISSUE** — cargo test --test length_operator_contract against predecessor →three
  public assertion failures, rc=101: valid operands return1000000001 and±2000000000 instead of typed
  domain refusal. Output names failed addition/subtraction/independent pair contracts, no compile error.
- [x] **ROOT CAUSE (WHY + WHERE)** — length.rs Add/Sub directly construct Self from raw arithmetic,
  while checked_add/sub call the bounded constructor. git show b8ed62d source assertions → unchecked
  operator construction/bounded checked methods verified, rc=0. Private field did not close the
  invariant. D90 error shape omits operation; D91 valid-Rust L6b false refusal →13 pass/2 fail, rc=1.
  Both have scheduled owners; explicit Rust fences unblock publication without claiming classifier repair.
- [x] **FIX** — public traits return Result<Length,UnitError> through checked_add/sub; migrate crate
  doctest to ?, document breaking result handling. No clamp, panic or inferred caller precondition;
  no UnitError shape change here. D90 is scheduled immediately next; scalar reference remains later.
- [x] **ADDRESSED (verified)** — public length_operator_contract →four tests pass, rc=0; nine-by-nine
  i128 oracle agrees on both operations/inclusive/signed/ordinary routes; explicit Result output.
  run_length_operator_mutations.sh →six compiled actual assertion reds/rc101 each, runner rc=0,
  production source restored byte-identically. Release four pass, rc=0; predecessor D89 seals unchanged.
- [x] **NO REGRESSION** — restored make check →fmt/clippy strict/native480 incl docs pass, rc=0;
  make wasm →sc-units/core/measure build browser target, rc=0. Restored structural and language15,
  publication9/ledger9/archive verify/retention green, rc=0:48 chapters/16 APIs/998 source/1523 rendered
  links;115 records/277423 resident bytes. Tree10/13/eight/zero gaps, glossary310/nine/158, feature105/29,
  uncertainty133/16/zero unowned, fixture20/four/five/zero mismatch pass, rc=0. Exact predecessor
  preservation/live14/sealed76/no overlap verified. Staged make gate: all doctrines green, rc=0; no remote-CI
  or full release claim.
- [x] **LOCKSTEP** — public Result migration and pending D90 context are explicit in expert units/
  status; current/next tasks and live pointers match. Predecessor rational protocol/checklist/journal
  and oldest ledgers retain exact bytes. G1 remains5/18; sc-units35, live14/76; next .3b.1b.

## Domain-context contract and evidence — preserved from d91df0a

- ID: `G1-SLICE.5a.3b.3b.1b`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

### `G1-SLICE.5a.3b.3b.1b` — actual-operation and truthful domain errors

- [x] **REPRODUCE / ISSUE** — predecessor public domain_context_contract before source repair:
  two compiled assertions fail, rc=101 (missing operation, invented zero-width cause).
- [x] **ROOT CAUSE (WHY + WHERE)** — error.rs DomainExceeded omitted operation; Length callers reused
  the base constructor and Display asserted upper-bound excess/conversion cause for every kind.
  Predecessor f432d68 source-signature assertions verify absent field/direct forwarding/false cause,
  rc=0. Core zero-width guard makes the unsupported cause independently reproducible.
- [x] **FIX** — static operation field; shared private checked Length/Area construction forwards
  actual caller labels; Ratio/core bridges retain their operation. Neutral outside-domain display
  preserves kind/signed value/limit; all other variant labels and numeric limits remain unchanged.
- [x] **ADDRESSED (verified)** — public domain_context_contract →5 pass/0 fail; private core guards
  →3 pass/0 fail, rc=0. run_domain_context_mutations.sh →fourteen actual compiled assertion reds,
  rc101 each, exact multi-source restoration/runner0; predecessor D89 operator runner→six reds/0.
- [x] **NO REGRESSION** — make check →strict fmt/lint/native488 including docs, rc=0; release public5
  and make wasm→all three libraries, rc=0. Focused reference/language/publication/ledger/archive/
  census checks green, rc=0; staged make gate→all doctrines green, rc=0. Private invalid core arms are
  totality guards, not a geometry certificate; valid Length divide/area cannot leave their domain.
- [x] **LOCKSTEP** — book operation-field migration, signed/zero truthful messages and honest proof
  boundaries match Rust. Learner/glossary/index/annex routes retained. Exact prior task evidence and
  oldest live payloads preserved. G1 stays5/18, sc-units40; live13/sealed78, next D91 .3b.1c.

## Inline-context contract and evidence — preserved from 0a6e9b0

- ID: `G1-SLICE.5a.3b.3b.1c`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

### `G1-SLICE.5a.3b.3b.1c` — inline code language context

- [x] **REPRODUCE / ISSUE** — predecessor inline_context_contract.py→AssertionError on valid
  annotated Rust, rc=1; actual census names L6b question mark, not a syntax/loading failure.
- [x] **ROOT CAUSE (WHY + WHERE)** — census code_spans discarded language context; outside-part
  L6b operator heuristic classified Rust as formula. Inspect actual HEAD d91df0a signatures→no
  annotation/context argument and operator-driven population, rc=0. D93 AST case count→12 versus
  annex Ten, rc=0; D34 index cell still D79 while committed MEMORY points to D91 (rg, rc=0).
- [x] **FIX** — exact immediate single Rust annotation outside normative parts; adjacent spans remain
  in the operator population. Malformed/unknown/detached/normative annotations refuse L6e and count
  in final mismatches; ? is unchanged. Correct D93 annex to twelve and D34 current index cell.
- [x] **ADDRESSED (verified)** — inline_context_contract.py→13 independent book verdicts/0 fail,
  rc=0. run_inline_context_mutations.sh→five actual assertion reds, rc1 each, exact restoration/
  runner0. run_rational_mutations.sh→twelve actual reds and exact restoration, rc=0; annex corrected.
- [x] **NO REGRESSION** — run_formula_language_probes.sh→16 pass/0 fail, rc=0; structural/numeric
  reference, publication9/ledger9/archive/censuses green, rc=0 (details below). Staged make gate→all doctrines green, rc=0.
  No Rust changed; no new native/WASM/CI claim. Marker declares author intent, not Rust validity.
- [x] **LOCKSTEP** — inline migration examples and expert context annex match executable census;
  progressive learner/glossary/index routes retained. Exact previous task protocol/checklist/journal
  and oldest payloads preserved. G1 stays5/18; live12/sealed80; next D83 .3b.2. D34 derivation stays owned.

## Scalar-domain contract — completed in G1-0049

- ID: `G1-SLICE.5a.3b.3b.2`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

## Scalar-domain acceptance — preserved from 86b81a9

### `G1-SLICE.5a.3b.3b.2` — exact scalar domains and isolated numeric setup

- [x] **REPRODUCE / ISSUE** — scalar_contract.py against predecessor→AssertionError:
  1000000001 um accepted, rc=1. D94 lower-endpoint mutation runner refuses unhandled Count0 FErr
  inside imported arithmetic_contract before scalar assertions run, rc=1; no scalar red claimed.
- [x] **ROOT CAUSE (WHY + WHERE)** — predecessor 0a6e9b0 result wrapper invokes width-only see;
  p_atom rounds without scalar check. Source-signature assertions verify absent scalar guard and
  numeric-family runpy execution of arithmetic_contract, rc=0. D94 trace confirms unrelated test path.
- [x] **FIX** — read normative signed length/area and nonnegative Count declarations; canonical
  inputs round once then check; exact completed numeric nodes check after width before binding.
  Actual operation/kind/bounds/fraction retained. Table-only shared setup replaces contract imports.
  Width fixtures use unrestricted kinds/tiny fractions; no runtime exemption or extra rounding.
- [x] **ADDRESSED (verified)** — scalar_contract.py→57 independent Fraction/domain controls/0 fail,
  rc=0; run_scalar_mutations.sh→eleven actual assertion reds, rc1 each, exact two-source restoration/
  runner0. Quiet-context assertion and actual output mutation distinguish D94. Rational61/twelve
  actual reds; literal6/arith9/angle7/inline5 actual reds and byte restoration, rc=0.
- [x] **NO REGRESSION** — run_formula_structure_probes.sh→structural16/end-to-end2, input130/3,
  expression fixtures12, literal60/361, arithmetic24/100/162, angle42/72/math42, rational61/scalar57
  green, rc=0. Focused language/book/ledger/archive/censuses green, rc=0; staged make gate→all
  doctrines green, rc=0 (details below).
  No Rust changed or new native/WASM/remote CI claim; selectors certify only the curated edge model.
- [x] **LOCKSTEP** — formula normative boundaries and expert annex match scalar/refusal behavior;
  Count fractions and geometry-only box scope explicit. Learner/glossary/index routes retained;
  exact predecessor task/oldest payloads preserved. G1 stays5/18; live12/sealed81; next i64 .3.


## Numeric binding protocol — completed in G1-0050

- ID: `G1-SLICE.5a.3b.3b.3a`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

## Numeric binding acceptance — preserved from 9b3b9b3

### `G1-SLICE.5a.3b.3b.3a` — once-rounded numeric binding storage

- [x] **REPRODUCE / ISSUE** — binding_contract.py against predecessor accepts Count MAX+1 at let:
  AssertionError oversize bound integer accepted, rc=1 (target/binding-pre-fix.log). Half-length
  bindings also retained fractions because statement returned evaluate unchanged; L2 rounded later.
- [x] **ROOT CAUSE (WHY + WHERE)** — predecessor let returns exact val without storage validation;
  normative §2/4.2 declares bound integers. run_binding_mutations.sh bypass→assertion red, rc=1;
  source restored byte-identically, runner0.
- [x] **FIX** — read declared numeric binding widths, round once in actual statement, check inclusive
  signed storage and scalar domain with binding/kind/bounds/rounded integer. Boolean stays Boolean.
  L2 consumes the returned integer without another round; unbound exact temporaries remain wider.
  D95 ruling is adopted without sign folding; full canonical proof remains separately owned .3b.
- [x] **ADDRESSED (verified)** — binding_contract.py→80 independent Fraction/Decimal controls/0 fail,
  rc=0; run_binding_mutations.sh→twelve compiled actual assertion reds, rc1 each, exact restoration/
  runner0. Copied specification width32 and missing declaration controls prove source consumption;
  two copied-book bindings verify actual replay/census. Valid/refused bindings do not publish to caller.
- [x] **NO REGRESSION** — structural16+2/input130+3/expression12/literal60/361/arithmetic24/100/162/
  angle42/72/math42/rational61/scalar57/binding80 all green, rc=0. Literal6/arith9/angle7/rational12/
  scalar11/inline5 actual reds and exact restoration, rc=0. Language16/publication9 pass, rc=0.
  Ledger/archive/censuses and staged make gate green, rc=0 (details below).
  No Rust changed or new native/WASM/remote CI claim; curated reference is not production evaluation.
- [x] **LOCKSTEP** — D95 decision, formula binding distinction, grammar/unary identity, expert annex,
  roadmap and live task pointers agree. Prior task/oldest ledger payloads preserve committed bytes;
  map regeneration uses compact curated input, unchanged ceilings/generator. G1 stays5/18; next .3b.


Current canonical verification remains in [G1-SLICE](G1-SLICE.md).

## Completed reference subtree — preserved during G1-0058

- ID: `G1-SLICE.5a.3b`
  Status: `done`
  Children: .3b.1 literal quantum/identity, .3b.2 exact arithmetic audit/repair, .3b.3 numeric domain
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

- ID: `G1-SLICE.5a.3b.1`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

- ID: `G1-SLICE.5a.3b.2`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

- ID: `G1-SLICE.5a.3b.3`
  Status: `done`
  Children: .3b.3a exact rational bit limit; .3b.3b scalar domains/storage bounds; .3b.3c signed-angle contract verification.
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

- ID: `G1-SLICE.5a.3b.3a`
  Status: `done`
  Children: .3a.1 D85/D86/D87 angle conversion/rounding/pole guards; .3a.2 D83 rational bit limit.
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

- ID: `G1-SLICE.5a.3b.3a.1`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

- ID: `G1-SLICE.5a.3b.3a.2`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

- ID: `G1-SLICE.5a.3b.3b`
  Status: `done`
  Children: .3b.1a D89 public operators; .3b.1b D90 operation context; .3b.1c D91 code-language context;
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

- ID: `G1-SLICE.5a.3b.3b.1a`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

- ID: `G1-SLICE.5a.3b.3b.1b`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

- ID: `G1-SLICE.5a.3b.3b.1c`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

- ID: `G1-SLICE.5a.3b.3b.2`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

- ID: `G1-SLICE.5a.3b.3b.3`
  Status: `done`
  Children: .3a numeric binding storage, .3b D95 signed literal ruling/canonical input, .3c review.
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

- ID: `G1-SLICE.5a.3b.3b.3a`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

- ID: `G1-SLICE.5a.3b.3b.3b`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

- ID: `G1-SLICE.5a.3b.3b.3c`
  Status: `done`
  Children: .1 triggered archive capacity, .1v observed CI, .2 complete numeric boundary review.
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

- ID: `G1-SLICE.5a.3b.3b.3c.1`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

- ID: `G1-SLICE.5a.3b.3b.3c.1v`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

- ID: `G1-SLICE.5a.3b.3b.3c.2`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

- ID: `G1-SLICE.5a.3b.3c`
  Status: `done`
  Children: .1 six-kind book replay prerequisite; .2 signed inverse-trig/binding/equality repair;
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

- ID: `G1-SLICE.5a.3b.3c.1`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

- ID: `G1-SLICE.5a.3b.3c.2`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

- ID: `G1-SLICE.5a.3b.3c.3`
  Status: `done`
  Goal:completed;[whole scope/proofs](../history/stitchcad-devnotes-part1094.md).

## Normalized arena proof — preserved from b41418d

### `G1-SLICE.5a.3c.3` — immutable normalized expression arena

- [x] **REPRODUCE / ISSUE** — .2 converts one literal view, while the recipe needs all literal inputs
  retained in an immutable whole graph before serialization. Actual syntax has private flat nodes/
  edges; new independent reference fixtures establish normalized shape/count/depth before code.
- [x] **ROOT CAUSE (WHY + WHERE)** — expression.rs keeps raw literals across all structural roles.
  Normalization must copy every node/edge, not only root/taken branch. Actual reference→24 authored
  normalized shape/count/depth rows pass, rc=0; product public tests→eight pass, rc=0.
- [x] **FIX** — separate privately built flat normalized arena; explicit normalize_literals borrows
  original source, keeps syntax reusable, converts every literal and returns unchanged located errors.
  Public root/kind/argument views retain arena lifetime/ordered edges; no operator/name/type evaluation.
- [x] **ADDRESSED (verified)** — eight public contracts include24 shapes/100 nested literal rows/
  25 book expressions/all input positions/refusals/source/iterator/privacy/small-stack controls, rc=0.
  Seventeen actual compiled root/name/operator/edge/order/coverage/depth/span/unit/refusal/iterator/
  Debug faults→assertion reds, rc=101 each, runner rc=0; exact production source restored.
- [x] **NO REGRESSION** — make check→513 passed, including three privacy-lifetime compile-fail docs
  and one runnable doc, rc=0; release eight pass, rc=0. WASM three crates compile, rc=0 (compile only).
  Reference/structural families, language16/publication9 controls pass, rc=0; no recipe execution claim.
- [x] **LOCKSTEP** — README/availability/language/grammar/API map/index/expert annex/decisions/live/
  task pointers agree; prior evidence/oldest ledger bytes retained exact. G1 still5/18, defects10open/
  89sealed; .5a.3c.4 owns coupled review, serializer/binding/evaluation remain separate.

## Verification Log

normalized_expression_reference.py watches24 independent authored rows through the actual book
reference/context; public formula_normalized_contract.rs consumes them. All100 independent literal
rows also normalize/refuse nested in product calls. Actual compiled mutation runner requires assertion
reds and exact restoration. Logs: target/g1-0060-{native,release-confirmed,wasm,structure,language-final,
publication-final,reds-final}.log, all final producers rc=0. Publication50chapters/18scopedAPIs/
1020source/1573rendered links. Ledger9 arms/13 pointer controls, archive28 arms/160 CLI controls and retention151
logical/26 working Markdown/8082 decoded lines/627932 decoded bytes/284254 resident bytes pass,
rc=0. Tree10lanes/13trees/eight siblings/zero gaps; glossary310/nine/158/zero drift; feature105/29,
uncertainty133/16/zero unowned and fixture20/four/five/zero mismatches pass, rc=0.
Initial fixture setup used empty unit context; load_context supplies the actual published unit table.
Initial binary-operator fault anchor matched builder and view; runner refused before mutation, then
anchored the builder specifically. Final all17 faults compile and fail public assertions.
README status only; objective/layout/commands unchanged. Prior protocol/history payloads retained exact.
Staged make gate→=== all doctrines green ===, rc=0; no project-owned background job remains.

## Commit Log

| Leaf | Commit subject | Verification |
| --- | --- | --- |
| `.5a.3c.3` | `STITCHCAD-G1-0060 (leaf G1-SLICE.5a.3c.3): retain normalized literal expression arenas` | independent24/nested100/seventeen actual reds/native513 |

## Changelog

- `2026-10-02`: whole immutable normalized syntax arena complete; next coupled normalization review.
- promotion: declined (routine literal normalization/immutable arena under received D95/D84 contracts).

## Coupled normalization pre-code review — completed in G1-0061

- ID: `G1-SLICE.5a.3c.4`
  Status: `in_progress`
  Goal: complete scoped production normalization review against formula2/4.2/4.3, grammar2/2.1/4,
  numerical input contract and D95/D84; close .5a.3c only after every obligation has concrete proof.
  Pre-code review protocol: retain prior byte-exact proof/source identities; map spelling/kind/unit,
  converted reduced128-bit width, shared half-away rounding, length-after-round, source/unary/raw
  turns, every child/branch/span/count/depth, atomic refusal/privacy/lifetime/flat clone/drop to public
  controls and actual fault evidence. Re-derive current producers; no evaluator/entity/serializer claim.
  Add independent Fraction reduction-boundary fixtures for all seven unit multipliers plus bare
  decimal ratios: powers2 at scales54..63 and powers5 at125..136, checking both sides of denominator
  width after exact cancellation. Verify actual published factors and valuations <=6; derive134/39/
  173 refusal/workspace witnesses. New public contract exercises each fixture through individual
  conversion and a nested whole arena; preserve exact kinds/errors/spans. Four compiled actual early-
  scale/raw-mantissa/mantissa-cancellation/unit-cancellation faults must fail this new assertion oracle,
  restore exact source; watch fixture verifier in structural suite. Production implementations unchanged.
  Run strict native/release/WASM and scoped reference/language/publication/ledger/archive/censuses/gate;
  publish obligation map in existing expert annex, update live/task parents and retain old evidence
  exactly within fixed document budgets. Commit before .5a.3d canonical serialization.
  D101 immediate/P1: publication caught stale-variable index overwrite; root identified in edit,
  restore committed index plus intended link and verify all chapter/source/render routes before closure.
  D102 immediate/P2: D101 log matched ## Decisions inside a quoted historical command;
  record before repair, restore exact D46 bytes and verify distinct heading/defect placement.
  Verification: `pending`
  Commit: `pending`


## Literal normalization parent — preserved during G1-0061

- ID: `G1-SLICE.5a.3c`
  Status: `in_progress`
  Goal: exact bounded literal normalization with typed kind/integer/source and shared rounding;
  D95 literal magnitude/reduced rational width128 is distinct from bound signed64. Preserve kind
  and unary syntax, input quantum/scalar boundaries; normalized Angle cannot hold raw sweeps.
  Children: .1 shared unsigned128 magnitude rounding; .2 exact typed literal normalization;
  .3 immutable normalized expression arena; .4 coupled normalization review. No evaluation.
  Literal conversion must reduce exact converted rational before its128-bit width check; scalar
  length bound follows input rounding. Arbitrarily large raw decimals must not narrow before valid
  cancellation, and pathological input must not require unbounded workspace. The completed .2 protocol uses bounded exact decimal reduction with no added dependency; whole
  arena normalization and its coupled review remain .3/.4.
  Verification: `pending`
  Commit: `pending`

[Current canonical-expression frontier](G1-SLICE.md#current-frontier).

## Coupled normalization proof — preserved from 168bf0f

### `G1-SLICE.5a.3c.4` — coupled production normalization review

- [x] **REPRODUCE / ISSUE** — .1/.2/.3 have isolated proofs; verify their coupled input obligations
  before identity serialization. Fraction controls expose valid reduced decimals beyond raw width.
- [x] **ROOT CAUSE (WHY + WHERE)** — literal.rs reduces decimal/unit factors before width/rounding;
  normalized.rs visits all structural roles with private flat storage. Actual producer→176 rows,
  103 valid/73 width refusals, valuations6/digits39/scale134/workspace173 witnesses pass, rc=0.
  Source SHA identities equal exact 97f124c/15d1520/b41418d implementations, rc=0. D101 stale local
  edit variable and D102 unanchored heading match were diagnosed from actual file/tool failures.
- [x] **FIX** — add independent conversion-frontier fixtures/producer/watched suite and one public
  contract exercising individual and nested whole conversion; publish full scoped obligation map.
  Restore index from HEAD plus intended link, and exact historical D46 bytes before sealing defects.
- [x] **ADDRESSED (verified)** — six literal/eight arena contracts include176 frontier rows and
  previous100 literal/24 shape/25 worked-expression controls, rc=0. Four actual scale/mantissa/unit
  cancellation faults compile and fail public assertions, rc=101 each; runner/restoration rc=0.
  D101 publication→50 chapters/49 indexed routes/1026 source/1580 rendered links, rc=0; D102 exact
  D46 comparison/one actual Decisions heading/two distinct entries before sealing pass, rc=0.
- [x] **NO REGRESSION** — strict make check→514 passed, rc=0; release six+eight contracts pass,
  rc=0. WASM three crates compile, rc=0 (compile only). Structural/reference families/language16/
  publication9 pass, rc=0. Exact production implementations unchanged; evaluation remains future.
- [x] **LOCKSTEP** — scoped normalization .5a.3c closes; book/grammar/decisions/live/task pointers
  agree. README objective/layout/commands unchanged. G1 remains5/18; serializer .5a.3d is next.
  Prior task/ledger payloads retained exact; D101/D102 fixed, remaining defects still owned.

## Verification Log

Logs target/g1-0061-{coupled,reds,native,release,wasm,structure,language,publication-final}.log are
observed terminal rc=0. New Fraction producer derives exact converted denominator widths and verifies
published multipliers; accepted scale133 angle versus refused134 challenges early guards. Public
checks compare typed kind/magnitude/refusal/span through both APIs. All four mutations require actual
compiled assertion failure and exact restore. Initial publication failure came from my stale edit
variable; restored index plus intended link passes. Record insertion then matched a quoted heading;
D46 restores byte-identical to HEAD, D101/D102 original logged payloads seal with verified disposition.
Ledger nine arms/13 pointer controls, archive28 arms/164 CLI controls and retention155 logical/30
working Markdown/8181 decoded lines/634890 decoded bytes/291212 resident bytes pass, rc=0.
Tree10 lanes/13 trees/eight siblings/zero gaps, glossary310/nine/158/zero drift, feature105/29,
uncertainty133/16/zero unowned and fixture20/four/five/zero mismatches pass, rc=0. Defects10open/
91sealed derive from live entries and reconstructed archives; prior payloads retained unchanged.
No new public behavior, numerical rule, dependency, binding or execution added. Complete normalization
obligation map is in the expert annex; canonical serialization and remaining recipe stages stay open.
Staged make gate→=== all doctrines green ===, rc=0; no unresolved verification remains.

## Commit Log

| Leaf | Commit subject | Verification |
| --- | --- | --- |
| `.5a.3c.4` | `STITCHCAD-G1-0061 (leaf G1-SLICE.5a.3c.4): complete coupled normalization review` | independent176/four actual reds/native514/release14 |

## Changelog

- `2026-10-02`: coupled production input normalization review complete; canonical serializer next.
- promotion: declined (routine scoped normalization review and repair of two local documentation edits).

## Canonical byte-contract pre-change protocol — completed in G1-0062

- ID: `G1-SLICE.5a.3d.1`
  Status: `in_progress`
  Goal: resolve the specification gap before production identity bytes become persistent contract.
  Pre-change protocol: inspect complete grammar4/operators/keywords and actual reference tuple nodes;
  derive coverage over all seven expression roles/ten binary operators. D103 owns missing exact unary/
  square bytes, P1 before serializer. Record concrete symbolic proposals; demonstrate named neg/square
  tags collide with distinct ordinary-call syntax while symbolic opcodes cannot be ordinary names.
  Add a tracked reference-backed contract audit, watch it in structural suite, publish proposal in
  existing expert annex/grammar/decision index without claiming implementation or changing semantics.
  Run scoped structural/language/publication/ledger/archive/tree/feature/glossary/gate controls; retain
  exact completed proofs and update resume/live/task pointers. No production byte serializer yet.
  Verification: `pending`
  Commit: `pending`
