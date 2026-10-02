# G1-SLICE — completed formula syntax contracts and evidence

Bounded semantic sibling of [G1-SLICE](G1-SLICE.md). Completed syntax contracts/checklists retain
exact committed text; the current frontier and verification/commit journal remain in the parent.

## Lexical contract and evidence — preserved from 60c7305

- ID: `G1-SLICE.5a.1`
  Status: `done`
  Goal: borrowed ASCII lexical stream for the normative machine-form formula syntax; explicit
  lexical kind, original text and byte span, with typed first-error refusal and fused termination.
  Pre-code protocol: sc_core::recipe owns the syntax front-end. Immutable FormulaLexeme and
  FormulaSourceSpan retain exact source text/positions; FormulaLexer scans borrowed input without
  cloning names/numbers or interpreting values. Classify three keywords using one shared private
  name-module classifier; preserve all existing MachineToken public constructor/error behavior.
  ASCII preflight reports the first full offending Unicode scalar before producing tokens. General
  ASCII whitespace is ignored between tokens but span gaps remain; the later parser enforces the
  literal's exact single-space unit separator. Identifier spelling uses the same allocation-free
  lower-snake predicate as MachineToken. Number tokens retain digits/optional nonempty fraction,
  with no numeric conversion/rounding/implicit unit. Punctuation/operators include longest paired
  comparisons; malformed identifiers, decimal fraction, lone ! and unsupported characters refuse.
  After first error/end, iterator remains ended. Debug of lexer reveals scope/position, not source.
  Lexical success certifies only tokens: adjacent atoms, comments assembled from slash operators,
  unsupported calls/exponents/units, type/name errors and recipe bounds retain parser/checker owners.
  Own native/WASM tests, borrowing/privacy docs, every current worked example's machine-token scan,
  actual guard mutations, book/public-status map/index, canonical ADR clause-link fix D73, bounded
  live/evidence seals and commit. No evaluation, canonical formula identity or app/MCP claimed.
  Verification: 13 lexical contracts/two privacy-lifetime docs; nine actual assertion reds/exact restore;
  454 strict native tests, WASM/book; publication nine, formula 15 and ledger nine probes green.
  Commit: `STITCHCAD-G1-0037` (this recording commit).

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
  Goal: repair D75 reference-oracle traversal of all call arguments and conditional-depth refusal.
  Reproduce: repo-local copied-book diagnostic replaces only dart_count's expression, keeping value 1;
  abs(1 + 128 zero terms) has 258 semantic nodes and abs-wrapped 17-level if has depth 17. Both
  `FORMULA_BOOK=target/formula-structural-diagnostic/<case>/src run_formula_language_census.sh`
  return rc=0, measured seven nodes/zero depth, zero mismatches. Expression bounds falsely certify.
  Pre-edit protocol: explicit child traversal includes call argument lists and every static if branch.
  Count expression nodes/depth iteratively without recursive walker stack; grouping/exponent payloads
  retain original semantic-node definition. Refuse max_if_depth with formula_domain before inference/
  evaluation, measured depth and unchanged bound. Existing per-expression node refusal stays strict.
  Add paired controls just below/at bounds and actual over-bound forms, including ifs inside ordinary
  calls and untaken branches. Count/depth probes independently construct fixture sizes and verify exact
  typed domain signatures, not failure on unrelated values/vocabulary/margins. Actual guard mutations
  distinguish both child walkers and early depth refusal, restoring original bytes. Register durable
  root-cause diagnostic/toolbox; update live/book/roadmap-owned limits and scope, retain lexer behavior.
  D77 milestone prerequisite: sibling-owner census rejects a valid .md#fragment link as orphan;
  recognize optional fragments without admitting different target names; add paired green/red probes.
  D76 malformed reference names/unit gap is owned next by .2b.1 before product parser implementation.
  This fixes the reference instrument, not the product evaluator or a declaration of G1 completion.
  Preserve completed lexical contract/checklist exactly if main task health needs partition; roll live
  ledgers at health without changing caps. Focused checks, full reference suite and commit before .2b.
  Verification: 16 structural controls/refusals plus two copied-book refusals; four actual guard reds
  and exact restoration; tree nine, reference language 15, full 25 suites green.
  Commit: `STITCHCAD-G1-0038` (this recording commit).

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
  Goal: fix D76 reference machine identifier/keyword-position and literal unit-gap parity before
  using it as an independent product parser oracle; explicit pre-code protocol and mutation controls.
  Pre-code protocol: repair the actual reference tokenizer/parser without changing language or Rust
  behavior. Whole-source ASCII preflight precedes tokens; validate captured lower-snake words without
  trimming/normalization. Keep let/assert/if recognized only in their grammar positions: no bare-name,
  ordinary-call or declaration-name use. Share a private reference identifier check across those paths.
  Retain original source positions through filtering; a numeric token immediately followed by a known
  unit requires the exact gap " ", including zero-gap/tabs/multiple-space refusals. General ASCII
  whitespace matches the lexer; units remain the canonical table's closed set. Do not transform an
  invalid gap into a valid literal during assert splitting. Existing supported if form remains intact.
  Diagnostic widening found zero-argument calls accepted as ASTs although args requires an expression;
  refuse before returning a call. Add that observation to D76 and cover nonempty/trailing-comma controls.
  Tests load actual reference definitions with canonical unit/binding contexts; positive names/reserved
  inputs, three keyword roles/declarations, all seven unit gaps, ASCII/whitespace, malformed numbers,
  empty calls and current worked examples. Cross-check unchanged Rust token/name contracts. Add paired
  copied-book controls and actual guard mutations, with exact restoration and no overlapping gates.
  Extend the existing structural suite with input controls so registered suite count remains 25.
  Reference checks are not product parsing, complete name/type/evaluation proof or command diagnostics.
  Keep remaining .2b.2 AST, .5a.3 canonical literal/recipe and later checker owners explicit; sync expert
  annex/live records; preserve completed .2a contract/checklist at health; rollover exact old ledgers.
  Verification: 130 input controls/three copied-book refusals; nine actual guard reds/exact restore;
  existing structural 16+2, language 15 and unchanged Rust name/lexer 20 green.
  Commit: `STITCHCAD-G1-0039` (this recording commit).

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
  Goal: immutable borrowed expression syntax with normative precedence/if and precise refusals.
  Pre-code protocol: read full formula contract/grammar/examples, ADR-0003, roadmap 4.1/4.2/10/G1,
  existing lexer/name contracts and book availability/annex. Syntax success grants no value, current
  name, call vocabulary, dimension, binding, statement, canonical identity or evaluation certificate.
  FormulaExpression privately owns a flat semantic-node arena borrowing number/name spellings; only
  read-only root/child views and argument iteration are exposed. No forgeable/cross-tree node indices.
  Literal units use the seven closed tokens and original exact one-space gap; retain decimal text
  without conversion/rounding. Grouping extends spans but creates no node; exponent 2 is square payload.
  Iterative operator/value/delimiter stacks enforce comparison/add/mul/unary/power precedence, left
  associativity, square tighter than unary minus, exactly one comparison per grammatical expr, and
  square-only power. Parenthesized comparisons remain independently syntactic; type checking is later.
  Ordinary calls require at least one argument; if requires exactly three. Keywords cannot be names.
  Unknown well-spelled call names remain syntax until the later closed vocabulary/name checker.
  Stream existing lexer, preserving ASCII preflight/errors. Reject adjacent atoms, comments, assignments,
  missing operands/delimiters, empty/trailing arguments, invalid gaps and chained comparison precisely.
  Reserve each semantic node on encounter and refuse measured 257 > 256; conditional frames refuse
  measured 17 > 16, including calls and every branch. Flat arena/drop and explicit stacks avoid input
  recursion; parentheses do not gain an invented language cap. Workspace is linear in source size,
  semantic arena bounded; command-layer input-byte/work budgets remain roadmap 10 owned future work.
  Parse errors carry source span/refused rule; limit errors carry typed limit/bound/measured size with
  formula_domain, unsupported exponent formula_unsupported, syntax/lexical errors formula_parse.
  Debug/error output omits customer source. Statement/canonical context is unavailable at this low-level
  syntax API and remains later wrapper responsibility; never fabricate a canonical expression.
  Native contracts independently enumerate precedence/spans/roles/unit forms/refusals; all current
  book expressions scanned with syntax-only verdicts. Exact node/depth boundaries, hidden branches,
  long prefix/ordinary-call/grouping inputs and small-thread stack tests cover arbitrary source shapes.
  Privacy/lifetime doctests and actual production guard mutations require assertion reds/exact restore.
  Compare independent reference shape controls with current product trees; strict Rust/WASM plus
  scoped book/reference/ledger/archive/censuses. No new dependencies; no need for web/library lookup.
  Update progressive availability and expert annex/API map with exact scope; glossary/index routes
  retained. Seal oldest history/lessons at health, preserve predecessor evidence, commit before .3.
  Verification: product 15 contracts/three privacy-lifetime docs; eleven actual assertion reds;
  reference twelve shape/count/depth fixtures; restored strict 472 tests/WASM green.
  Commit: `STITCHCAD-G1-0040` (this recording commit).

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
  Goal: investigate/repair D78 total public rounding at extreme i128 magnitudes before conversion.
  Pre-code protocol: round.rs public i128 numerator/denominator has no narrowing precondition;
  units chapter 1/2/9, numerical ADR and library docs require UnitError rather than panic/wrap.
  Suspected path: n=i128::MIN, d=1 yields unsigned q=2^127; casting q to i128 gives MIN and
  negating it overflows before the existing i64 try_from can refuse. Build a repo-local actual
  public-interface diagnostic first, catch unwinds and compare positive/negative extremes/control.
  Confirmed actual public probe: MIN/1 unwinds, MAX/1 and MIN/-1 return typed Overflow; controls
  succeed, rc=0. Scoped history: eb83f01 G0-CONTRACT.18 introduced the only round.rs revision.
  Preserve half-away quotient/remainder behavior; checked magnitude-to-i64/sign
  reconstruction must allow exactly |i64::MIN| for negative results and refuse every wider magnitude.
  No float, saturation, changed rounding/tie rule or narrower caller precondition. Cover i128 MIN/MAX,
  both signs, denominator MIN/zero, i64 endpoints and just-outside values; real guard mutations must
  discriminate magnitude bounds/sign/negative endpoint/ties and restore original source bytes.
  Strict Rust/WASM, scoped book/reference/ledger/archive/censuses and staged doctrines; report root
  and measured public behavior. No canonical literal/evaluation claim. D79 reference literal identity
  is already independently reproduced and scheduled next by .3b; do not use it as canonical oracle yet.
  Preserve completed AST protocol/checklist and oldest live ledgers at health, sync numerical annex
  and live resume/status; commit before reference repair/product normalization.
  Verification: four public contracts/36 Fraction rows; five actual assertion reds/exact restoration;
  strict 476 tests, release four and three-library WASM green.
  Commit: `STITCHCAD-G1-0041` (this recording commit).

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
  Goal: exact once-rounded count/ratio/unit reference literal nodes, D79; repair D80/D81 doc drift.
  Pre-code protocol: grammar 2/2.1/4 canonical kind:integer; units 2 direct conversion/half-away.
  Diagnostic first: actual parser/evaluator at named sub-quantum/tie/precision/unit cases, including
  bare ratio, compare independently authored quantum oracle and canonical respellings. Only literal
  input is quantized here; operator evaluation remains separate .3b.2 and may not be used to sign off
  exact-arithmetic identity. Counts stay counts, bare decimals/pct stay ratios, all seven units retain
  exact chapter factors; no new float, implicit kind conversion, angle modulo or literal folding.
  Finalize paired cases before source edit; L1 compares the actual integer node rather than rounding
  its display to conceal a fraction. Keep curated reference scope, not arbitrary-input product safety.
  Own tracked diagnostic/contracts/fixtures and real reference guard mutations with exact restoration;
  run input/structural/language/publication checks and doctrine gate. Preserve predecessor evidence
  byte-identically. Numerical canonical/binding domains and stored angles remain .3b.3 prerequisites.
  D80: units heading 2.2 occurs twice after G1-0041; public rounding becomes 2.3, display stays 2.2.
  D81: LIVE_STATUS G1 still names completed .5a.2b.1; update to actual current frontier in this slice.
  Verification: 60 rows/360 controls, six actual reds/exact restore; reference/language/book green.
  Commit: `STITCHCAD-G1-0042` (this recording commit).

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
  Goal: repair reproduced D82 exact reference arithmetic against formula contract 4.2 before oracle use.
  Pre-code protocol: complete formula contract/grammar/ADR exact + - * /, square and rational selector;
  canonical integer inputs (.3b.1) precede expression arithmetic. Current actual diagnostic returns
  2 for 1 um / 2 + 1 um / 2, and zero for ratio 0.000001 squared. evaluate square/product/quotient
  and param_at explicitly call rnd before binding. Remove only these premature rounding steps and
  return exact reduced Fraction in result-kind internal units. Keep +/-, comparisons, lazy branches,
  dimension signatures, direct ratio scaling, zero division, explicit round_to quantization and
  irrational nearest-quantum results. Binding rounding remains census L2; no product evaluator/API.
  Own explicit exact result fixtures, independent Fraction parameter controls/reassociation/binding
  tie proof, read-only selector model controls, lazy/nonzero/irrational/round_to controls and actual
  arithmetic guard mutation/restoration. Do not invent arbitrary curve geometry for reference edges.
  Rational/domain/binding enforcement D83 stays next .3b.3. Clarify explicit round_to vs implicit
  arithmetic rounding in the book without adding/removing any declared language operation.
  Run restored structural/input/literal/language/publication/recording checks and stage doctrine gate;
  preserve prior exact protocol/evidence and oldest live ledgers at health before growth.
  Verification: 24 rows/100 independent Fraction cases/162 controls, nine actual reds/exact restore; focused checks green.
  Commit: `STITCHCAD-G1-0043` (this recording commit).

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
  Goal: repair D85 direction truncation, D86 microdegree/radian scaling and D87 exact tan pole refusal.
  Pre-code protocol: units 1.2/formula 4.2 and grammar 6/6.1 require microdegree inputs, nearest internal
  quantum for irrational results, normalized dir, and typed refusal outside a mathematical domain.
  Actual diagnostic: arc_length(360 deg,1 um) → 6283185, not rounded6; dir((0,0),(1,6)) → 80537677,
  nearest80537678. Trig source treats to_true(angle) as degrees though it returns microdegrees; dir
  normalizes int(Decimal) without rnd. tan odd-quarter turns are undefined, independent of rounding.
  Shared exact microdegree-to-radian helper divides by 180*1000000 before applying pi; no change to
  arithmetic internal units or modulo sweep. Round dir before normalization; refuse exact rational
  tan poles using microdegree modulo180deg, with formula_domain quantity/operation context.
  Own independently expected quarter/half/full/signed/multi-turn and fractional angle controls,
  Decimal precision/model scope, dir/atan2 agreement, preserved arithmetic/literal contracts and actual
  guard mutations/exact restoration. Clarify tan domain in grammar, details only in expert annex.
  D84 director ruling preserves formula sign/turns; .3c owns verification. D83 width/domains stay pending.
  Verify focused reference/language/book/recording/doctrines; commit before rational-bound repair.
  Verification: 42 rows/72 controls, independent math42, seven actual reds/exact restore; focused gates green.
  Commit: `STITCHCAD-G1-0044` (this recording commit).

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
  Goal: D83 exact rational limit refusal at actual literal/value boundaries, before product oracle use.
  Pre-code protocol: contract 4.2/4.3/5.2 max_rational_bits=128 on reduced exact numerator/denominator;
  see currently only measures and L8 only reddens book census after evaluation. Introduce measured
  typed formula_domain on the first value wider than128, with unchanged bound and named operation.
  Test 127/128/129 bit numerators/denominators, reduced fractions, converted input before quantum
  rounding, positive/negative values, zero and trailing-zero spellings. Preserve .3b.1 input rounding
  after exact conversion; a long sub-quantum literal must not hide an oversized converted fraction.
  Actual evaluation checks result-kind internal Fractions at every completed numeric node, including
  literals/names/unary/+/-/square/products/quotients/selectors; only taken branches compute values.
  Do not bound unreduced raw products or intermediate representation temporaries: the declared value
  is the reduced rational. Literal normalization checks converted exact value before its input round;
  static node/depth/statement bounds are distinct from runtime rational width on computed values.
  Own actual reference diagnostic/contracts/guard mutations/restoration, including angle contract
  observations under the recorded D84 ruling without changing angle semantics; fix the earlier 100-zero
  literal control to demand width refusal and keep a separate reducible long-zero positive control.
  D88 is additionally owned here: the new width guard masks a removed tan pole guard because the
  angle contract checks only formula_domain. Require the mathematical pole reason, distinct from
  atan2 zero-vector refusal, and rerun existing mutation suites with exact restoration.
  No scalar-domain or complete angle implementation is claimed; .3b/.3c own those prerequisites next.
  Verify focused reference/language/book/ledger/archive/censuses and doctrine; preserve completed text.
  Verification: 61 Fraction controls/twelve actual reds; existing six/nine/seven reds and exact restoration.
  Commit: `STITCHCAD-G1-0045` (this recording commit).

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
  Goal: close D89: every public Length + and - preserves the constructor’s ±MAX_LENGTH_UM invariant.
  Pre-code protocol: units1.1/9 and private Length fields require checked, non-saturating results.
  Trait Output becomes Result<Length,UnitError>; delegate to existing checked_add/checked_sub instead
  of constructing Self. UnitError shape is unchanged here; .1b owns missing operation context D90.
  Public tests first reproduce actual old-operator values, then verify inclusive endpoints, signed
  crossings, zero/cancellation/ordinary values, refusal identity and no unwinding. Curated independent
  i128 pair oracle decides expected sum/difference, never a second Length implementation. Real source
  mutations must compile and fail assertions, restore bytes; no probe/build overlap during mutation.
  Update crate docs with ? migration and the book’s exact public contract/proof boundary. Update
  live/task records, preserve completed reference history/ledgers unchanged; strict Rust/release/WASM,
  focused publication/censuses/recording gates before commit. No formula evaluator/MCP signoff.
  Verification: four public contracts/six compiled reds/exact restore; strict native480/release4/WASM green.
  Commit: `STITCHCAD-G1-0046` (this recording commit).

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
  Goal: D90 typed/display domain failures retain actual operation, including forwarding constructors
  and checked arithmetic; update every in-repo construction/match site without changing numeric limits.
  Pre-code protocol: units1.1/9 and UnitError’s own contract require operation plus kind/value/limit.
  D92 is also owned here: generic display says zero range width exceeds1 and invents unit conversion
  as its cause. Render neutral outside-domain wording with no unsupported cause; own signed/zero
  message controls and actual rendering mutation, preserving all numeric fields and other variants.
  Add static operation field to DomainExceeded and render it. Length/Area private checked construction
  accepts caller context; direct public constructors name themselves; conversion/arithmetic callers
  retain their operation. Ratio::scale forwards its context. Other error variants retain existing labels.
  Core range/topology bridges name resolve_range/resolve; their invalid-rational arms are totality guards,
  unreachable through validated journals, so private tests check those arms without claiming geometry.
  Public tests require direct/forwarded signed context, add/sub/mul, Ratio scale and unchanged refusals;
  inclusive endpoints and D89 Result semantics remain.
  First prove pre-fix displayed operation absence with actual public calls; add compiled actual field/
  forwarding/display mutations and restore exact sources. Update existing full-variant test matches,
  public API migration and numeric annex; preserve prior evidence/ledgers. Strict native/release/WASM,
  focused book/reference/censuses/recording gates before commit. D91 context, D83/D84 remain later.
  Verification: public5/private3/fourteen compiled reds; restored strict488/release5/WASM and focused gates.
  Commit: `STITCHCAD-G1-0047` (this recording commit).

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
  Goal: D91 formula-vocabulary census honors explicit foreign-code context; preserve actual formula
  operator refusals and add Rust-positive/formula-negative controls before normalization oracle use.
  Pre-code protocol: read L6a/b/d populations and line-local code-span extraction, existing fifteen
  end-to-end probes and expert annex. Formula characters stay closed; ? is never globally permitted.
  Outside the three normative formula parts only, allow an exact immediate inline annotation
  <!-- stitchcad-inline: rust --> before one backtick span. Skip that single declared foreign span,
  never adjacent unannotated spans, a whole line/chapter or formula positions in the normative parts.
  Refuse unknown/malformed/detached annotations and any foreign annotation inside formula parts.
  New scratch-book controls must reproduce predecessor Rust-positive failure, verify Rust ? accepted,
  unannotated/formula ? and adjacent bad operator still refused, malformed context and normative
  exemption refused. Add real census mutations removing/overbroadening exemption; prove actual
  contract assertion reds/restoration. Keep reference evaluator/parser/numeric contracts unchanged.
  D93 annex mutation-count drift is also owned here; derive twelve actual cases and re-run their reds.
  Repair the stale D34 task index cell to match this verified frontier; D34’s derived-pointer task
  remains owned by PLANNING.5. Synchronize book authoring annex and live records; preserve preceding
  task protocol/checklist/journal and oldest ledgers byte-exact. Run focused language/structural/
  publication/ledger/archive/censuses and staged gate before commit; no new Rust product claim.
  Verification: thirteen independent copied-book verdicts/five actual reds/exact restore; focused gates.
  Commit: `STITCHCAD-G1-0048` (this recording commit).

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
  Goal: D83 reference signed length/area and nonnegative count domains at completed value boundaries;
  align exact Fraction tests with production invariants after .1a/.1b, no extra arithmetic rounding.
  Pre-code protocol: units1.1 declares signed length±10^9/area±10^18; formula2 declares Count
  never negative, formula4.2 retains exact arithmetic and grammar5 forbids Count unary negation.
  Read length/area bounds from the actual units table and confirm the nonnegative Count contract;
  no duplicate hardcoded numeric limit or scalar use of the geometry-only piece-box10m bound.
  Add typed scalar check to every completed numeric node, after existing rational-width refusal;
  canonical literals check their rounded input value during parsing, including untaken branches.
  Exact expression fractions are checked before binding rounding. Count fractions remain exact and
  nonnegative until binding; ratio/angle have no extra scalar bound here (i64 storage is .3).
  Test independent Fraction endpoints/signs/just-outside values, input half-quantum boundaries,
  literal/name/operator/call/selector/tolerance paths, lazy branches and unchanged opaque refs.
  First reproduce actual predecessor length acceptance; require specific operation/kind/bounds/
  measured value and token for refusals, not just any domain error. Mutate actual guards and restore
  bytes with no overlapping build/probe. Update width fixtures whose huge lengths are now invalid:
  retain 128-bit refusal controls using unrestricted ratio/count values or tiny valid fractions,
  never exempt domain checks to keep old fixtures green. Re-run twelve width/six literal/nine
  arithmetic/seven angular/five inline mutations and focused structural/language/book checks.
  D94 is also owned here: extract table-only context loading from arithmetic test execution so
  unrelated failures cannot contaminate scalar/rational/angular mutation evidence; prove quiet load.
  Update normative boundary explanation/expert annex and live records; preserve prior task/ledger
  payloads exactly. D83 remains open for i64 .3; no production evaluator or geometry/MCP claim.
  Verification: scalar57/eleven actual reds; rational61/twelve reds; restored focused checks.
  Commit: `STITCHCAD-G1-0049` (this recording commit).

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
  Goal: actual reference let binding rounds once and refuses stored numeric integers outside i64.
  Pre-code protocol: formula2/4.2 and units1/1.2 declare signed64 stored length/area/ratio/angle/count.
  Completed exact nodes retain128-bit rational width and scalar domains; no i64 cap on temporaries.
  Binding rounds half away once, checks inclusive -2^63..2^63-1 plus existing scalar domain, and
  returns the stored value. Count stays nonnegative. Boolean remains a boolean; no opaque geometry
  numeric reclassification. L2 consumes that actual bound integer without its own second round.
  Independent Fraction/Decimal controls must reproduce predecessor let retaining fractions or storing
  oversize values; verify both endpoints, half-quantum edges, negative Count refusal, sign/turn
  preservation, exact unbound temporaries/cancellation, atomic refusal and actual replay behavior.
  Mutate real binding/round/limit/census-consumption guards, require assertion reds and exact restore;
  re-run scalar/width/literal/arithmetic/angular/inline controls without exempting boundaries.
  Adopt the received D95 ruling in a decision/book/roadmap; full canonical proof remains .3b.
  Compact curated Knowledge Map orientation before adding its derived decision row; no cap changes.
  Preserve prior
  task/ledger payloads and synchronize normative binding explanation/expert annex/live pointers.
  Focused reference/language/publication/ledger/archive/censuses and staged gate before commit.
  No Rust evaluator, caller environment mutation, full DAG/geometry/MCP or release claim.
  Verification: binding80/twelve actual reds; focused checks and gate in parent.
  Commit: `STITCHCAD-G1-0050`.

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
