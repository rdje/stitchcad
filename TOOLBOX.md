# TOOLBOX.md — the tools-first diagnostic doctrine

⛔ **TOOLS-FIRST.** For ANY unknown — a failure, a crash, a hang, a surprising result, a
"why isn't this working" — reach for a diagnostic tool FIRST. Never eyeball the code and
guess a root cause.

## The rule

- A code change cannot land without **tool-backed WHY + WHERE** and a **measured
  before→after** recorded in its task-tree leaf (see the acceptance checklist in
  `DOCTRINE_ENFORCEMENT.md`).
- If no existing tool shows WHY+WHERE, **build one** — a probe, a tracer, a counter, a
  minimal reproduction harness. The diagnostic tool is a first-class deliverable, kept in
  the repo, not a throwaway.
- **ANTI-SPIN TRIPWIRE:** if you have analyzed for ~2 turns without producing NEW tool
  output that pinpoints WHY+WHERE, STOP — run a tool, build one, or escalate. Never loop
  on analysis.

## The 3-step UNKNOWN protocol (adapt the specific tools to your domain)

1. **WIDEN** — dump the full picture: enumerate all cases/states, the broadest inventory,
   so the failing one is visible in context.
2. **NARROW** — probe the specific failing case for its exact verdict + position/state.
3. **PINPOINT** — a scoped trace that names the exact function/rule/line that fails and why.

The point is to convert "it's broken somewhere" into "line X of function Y rejects input Z
because predicate P is false" before writing a single line of fix.

## This project's toolbox

<!-- Fill this in as your project grows. List each diagnostic tool, what question it
answers (WHY / WHERE / how-much), and how to invoke it (binary, flag, env var). The next
agent should be able to reach for the right tool without reading the source. -->

| Tool | Answers | How to invoke |
| --- | --- | --- |
| handoff census | are process/handle observations available, and does project work remain? | `bash scripts/check_handoff.sh` (OS-visible); `--idle-cua` attests no pending CUA call/result; `--all` lists advisories |
| handoff controls | do unavailable evidence, real handles and narrow idle metadata still discriminate? | `bash docs/tasks/artifacts/handoff/run_handoff_probes.sh` →43 fixtures/13 actual guard assertion reds |
| doctrine enforcer | is the repository committable — do all 13 registered doctrines hold right now? | `scripts/check_doctrines.sh` (same as `make gate`; prints `=== all doctrines green ===`) |
| per-check self-test | does a single doctrine check still discriminate (both arms fire)? | `scripts/check_<name>.sh --self-test` (e.g. `scripts/check_live_doc_currency.sh --self-test`) |
| multi-leaf shadowing probe | which leaf's evidence does `TASK-ACCEPTANCE` actually judge, and does a placeholder block an honest leaf? (defect D15) | `bash docs/tasks/artifacts/task_acceptance/run_multileaf_shadowing_probe.sh` → `probes: N pass / M fail` |
| scaffold updater | which spine files would a sync overwrite, and which are guarded project content? (defect D17) | `scripts/update_scaffold.sh <bedrock-url-or-path> --dry-run` (live run needs a clean tree; `--force-project-sections` to override a guard) |
| scaffold-sync probe suite | does the updater still protect the task-tree index, refuse a dirty tree and write nothing on a dry run? | `bash docs/tasks/artifacts/scaffold_sync/run_update_scaffold_probes.sh` → `probes: N pass / M fail` |
| signature-portability probe | which evidence strings does the acceptance gate actually recognise, and does the answer depend on the regex engine? (the tracked producer behind the published `12 of 36` / `2 of 36` measurement) | `bash docs/tasks/artifacts/evidence_signatures/run_signature_portability_probe.sh` → per-line table, the two counts, `probes: N pass / M fail` |
| live-doc size check | is every tracked live document classified, owned and inside its ceiling — and does every route end at a classified destination? | `scripts/check_live_doc_size.sh` (runs inside `make gate`); refusal classes: `--self-test` |
| tree-coverage census | is the whole roadmap still captured — every §11 lane owned by a tree that names it, every tree registered with a lane, every `docs/tasks/` file either a tree or a linked sibling, every index link resolving? | `bash docs/tasks/artifacts/planning/run_tree_coverage_census.sh` → `census: N lanes / M trees / K sibling(s) / 0 unowned / 0 orphan(s) / 0 dead link(s)` |
| tree-coverage probe suite | does that census still NOTICE an unlinked evidence sibling, a stray file, a lane with no tree, a tree with no lane, a dead index link? (defect D44: a census no gate ran went red for two commits) | `bash docs/tasks/artifacts/planning/run_tree_coverage_probes.sh` → `probes: N pass / M fail` |
| table-pipe gate | does any staged `.md` table row carry a raw pipe inside a code span — the shape a renderer splits, dropping the rightmost cell silently? (project doctrine `TABLE-CODE-PIPE`, defect D47) | `scripts/check_table_code_pipes.sh` (runs inside `make gate`); arms: `--self-test`; whole tracked tree: `--all` |
| uncertainty census | what does this project NOT know yet — every `assumed` / `unknown` / `unverified-with-owner` / `read-external` / `(proposed)` / `vacant` marker in the book, with the authority that resolves it, and a refusal when a blocking marker's status section names nobody? | `bash docs/tasks/artifacts/uncertainty/run_uncertainty_census.sh` → `uncertainty census: N markers / M files / K unowned / 0 failure(s)` |
| uncertainty probe suite | does that census still NOTICE an unowned assumption, a chapter added later, and does it still leave a definition alone? | `bash docs/tasks/artifacts/uncertainty/run_uncertainty_probes.sh` → `probes: N pass / M fail` |
| table-render oracle | does a renderer really split a table cell on a raw `\|` inside a code span, and does escaping fix it? (defect D22, settled by a rendered page rather than by reading a spec) | `bash docs/tasks/artifacts/table_render/run_table_render_probes.sh` → prints the rendered rows, then `probes: N pass / M fail`; needs `mdbook`, and refuses with exit=2 without it |
| cell-budget census | what should a table-shaped collection's widest-line target BE — derived from each table shape's per-column cell budget instead of from prose or from today's widest line? (SPINE.4.4) | `bash docs/tasks/artifacts/live_doc_size/run_cell_budget_census.sh` → per-shape column budgets, then `cell budget: N shapes / M data rows measured / recommended maxline health K B`; `--self-test` proves the arithmetic on a synthetic table; `CELL_BUDGET_GLOB=<glob>` measures another population |
| live-doc size probe suite | does the containment check still pass on the real tree, and does it NOTICE a surface that lost its registry row? | `bash docs/tasks/artifacts/live_doc_size/run_live_doc_size_probes.sh` → `probes: N pass / M fail` |
| containment data plane | which surfaces and routes are governed, with what targets and ceilings? | `.doctrine/live_document_size/surfaces.tsv` and `routes.tsv` (read the header comments first) |
| fresh-evidence check | does THIS commit add its own ticked, evidence-backed acceptance boxes for the code it stages? (project doctrine `FRESH-ACCEPTANCE-EVIDENCE`) | `scripts/check_fresh_acceptance_evidence.sh` (runs inside `make gate`); arms: `--self-test` |
| fresh-evidence probe suite | does the fresh-evidence doctrine refuse stale evidence and still accept a co-staged documentation tree? | `bash docs/tasks/artifacts/fresh_evidence/run_fresh_evidence_probes.sh` → `probes: N pass / M fail` |
| task-acceptance probe suite | does the inherited acceptance gate still hold its shipped properties? | `bash docs/tasks/artifacts/task_acceptance/run_task_acceptance_probes.sh` |
| Rust gate | do fmt, clippy (deny warnings) and the tests pass? | `make check` |
| WASM smoketest | do the foundation crates really cross-compile for the browser profile (not a host `cargo check`)? | `make wasm` |
| push-due | is an exceptional push owed — did an unpushed commit touch CI, a doctrine check, the `.doctrine/` seams or the hooks? | `make push-due` (exit 1 = a push is due, and it lists the files) |
| glossary census | is the vocabulary sound — one meaning per term, one owner per machine token, every canonical reference resolving to a real clause/leaf, every token the spec set uses declared somewhere? | `bash docs/tasks/artifacts/glossary/run_glossary_census.sh` → `glossary census: N terms / M parts / K tokens / 0 failure(s)`; `--emit-index` regenerates the A–Z index the chapter must carry |
| feature-matrix census | is the envelope complete — every ontology object clause cited by a row, every roadmap non-goal rejected, every envelope garment supported, every refusal naming a declared diagnostic, every gate cell a real gate, every link resolving? | `bash docs/tasks/artifacts/feature_matrix/run_feature_matrix_census.sh` → `feature-matrix census: N rows / M diagnostics / 0 failure(s)`, plus the `A1` list of rows no gate has accepted and the `A3` list of cells whose gate is a *proposal* the roadmap does not carry yet |
| feature-matrix probe suite | does that census still NOTICE a dropped non-goal, an uncited ontology clause, an undeclared diagnostic, a prose gate? | `bash docs/tasks/artifacts/feature_matrix/run_feature_matrix_probes.sh` → `probes: N pass / M fail` |
| standards census | is any external standard cited anywhere in the book without a registered role, a status from the closed vocabulary and a named owner? | `bash docs/tasks/artifacts/standards/run_standards_census.sh` → `standards census: N registered / M designations used / 0 failure(s)`, plus a per-designation list of where it is used |
| standards probe suite | does that census still NOTICE a smuggled citation, an invented status, an ownerless claim, a bare `read-in-repo`? | `bash docs/tasks/artifacts/standards/run_standards_probes.sh` → `probes: N pass / M fail` |
| changelog-ledger probes | is the changelog a ledger — live window in commit order, nothing both live and sealed, every sealed segment's sha256 and line count true, coverage and pointer claims closed? The digest rule runs over **every** logical history segment (raw and packed), so a non-changelog rollover (the dev-notes archive) is watched too; coverage and pointer stay changelog-scoped, which is defect D40 and `SPINE.19`'s | `bash docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh` → `probes: N pass / M fail`; the pending entry is derived from `git diff HEAD`, and `LEDGER_PENDING=<id>` declares it explicitly if the tree cannot |
| fixture derivation | does the reference skirt still agree with itself — every §4 formula evaluating to its published number, all four closure checks closing, every piece accounted for by a span or a declared non-sewn attachment, §12's count matching §6's list, and §4's band width describing the same construction as §6's band pieces? (defects D27, D33) | `bash docs/tasks/artifacts/reference_fixture/run_fixture_derivation.sh` → `fixture derivation: N derived rows / M closure checks / K pieces / 0 mismatch(es)`; `FIXTURE_CHAPTER=<path>` points it at another copy |
| fixture derivation probe suite | does that instrument still NOTICE a faced two-piece band, an unaccounted piece, a wrong piece count, an edited formula, an undeclared token, a falsified closure, a changed constant? | `bash docs/tasks/artifacts/reference_fixture/run_fixture_probes.sh` → `probes: N pass / M fail` |
| glossary probe suite | does the census still NOTICE a duplicate token, an invented clause, a lost ⚠, an undeclared token, a drifted index? | `bash docs/tasks/artifacts/glossary/run_glossary_probes.sh` → `probes: N pass / M fail` |
| formula-language census | does the formula language still agree with itself — literals canonicalizing through the unit table, examples evaluating to their published values, names matching the fixture chapter, assertions holding at their class, refusals raising the token they name, vocabulary declared and implemented, links resolving, parts listed, limits above what the book measures? | `bash docs/tasks/artifacts/formula_language/run_formula_language_census.sh` → `formula-language census: N bindings / M assertions / K refusals / 0 mismatch(es)`; `FORMULA_BOOK=<dir>` points it at another copy |
| formula-language probe suite | does that census still NOTICE a wrong canonical form, a value its expression does not compute, a fixture chapter it disagrees with, a falsified assertion, a misnamed diagnostic, an undeclared function, an undeclared operator, a function the evaluator and the tables disagree about, a dead clause, an unlisted part, an unusable limit — and stay green on an edit that breaks no rule? | `bash docs/tasks/artifacts/formula_language/run_formula_language_probes.sh` → `probes: N pass / M fail` |
| interchange census | is the interchange contract closed — every layer the roadmap's ADR-0004 names dispositioned by the chapter (and no layer invented), one meaning per layer, every axis a registry column and every column an axis, no target named that §2 does not register, the entity policy inside its closed vocabulary with POLYLINE written and SPLINE refused, every diagnostic declared, every link and clause resolving? | `bash docs/tasks/artifacts/interchange/run_interchange_census.sh` → `interchange census: N layers / M targets / K entities / 0 failure(s)`; `INTERCHANGE_BOOK=<dir>` and `INTERCHANGE_ROADMAP=<path>` point it at another copy |
| interchange probe suite | does that census still NOTICE a dropped layer, a convention the roadmap grows, an invented AAMA name, a doubly claimed layer, an axis and a column that disagree, an unregistered target, a widened entity policy, a per-release entity set, an undeclared diagnostic, a dead clause — and stay green on an edit that breaks no rule? | `bash docs/tasks/artifacts/interchange/run_interchange_probes.sh` → `probes: N pass / M fail` |
| canvas-spike verdict | which canvas topology wins ADR-0002, and by which rule — the declared gates, margin and tiebreak order applied to the measurements `G1-SLICE.13` records, per runtime profile, with R5 escalation when nothing survives? | `bash docs/tasks/artifacts/canvas_spike/run_spike_verdict.sh` → the rule trace, then `spike verdict: N row(s) / M profile(s) / <outcome> / K refusal(s)`; `PENDING` while `results.tsv` is empty, exit 1 on a data set that cannot produce a verdict, exit 2 on a missing one; `SPIKE_DIR=<dir>` for another plane |
| canvas-spike probe suite | does that verdict still NOTICE a correctness gate outranking speed, a tie broken by memory, a native-only winner, a profile with no survivor, an unmeasured gate, an undeclared topology, a missing row — and does tightening a threshold in the TSV change the outcome, proving the gates are read and not hardcoded? | `bash docs/tasks/artifacts/canvas_spike/run_spike_verdict_probes.sh` → `probes: N pass / M fail` |
| release-contract census | is the release chapter still the roadmap's §9 and §8.2 — every manifest field the roadmap names realised (and none invented in its name), the six acceptance states in the roadmap's order, the matrix covering every artifact class and every ontology state, the disposition vocabulary closed and used, every diagnostic declared, every link resolving? | `bash docs/tasks/artifacts/release_contract/run_release_contract_census.sh` → `release-contract census: N manifest fields / M states / K matrix rows / 0 failure(s)`; `RELEASE_BOOK=<dir>` points it at another copy |
| release-contract probe suite | does that census still NOTICE a dropped manifest field, a field wearing the roadmap's authority, a reordered or renamed ladder rung, a missing artifact class, an unrecorded tuning, an undispositioned state, an undeclared or unused disposition, an undeclared diagnostic, a dead clause — and stay green on an edit that breaks no rule? | `bash docs/tasks/artifacts/release_contract/run_release_contract_probes.sh` → `probes: N pass / M fail` |
| i18n census | is the message inventory complete — every diagnostic token any chapter declares, and every `UnitError` variant in the crate, covered by exactly one family row with a count the census re-derives; every family tiered from §9's set with the safety families not tiered ordinary; every lint exemption justified; every `i18n_*` token declared; every link resolving? | `bash docs/tasks/artifacts/i18n/run_i18n_census.sh` → `i18n census: N families / M message ids / 0 failure(s)`; `I18N_BOOK=<dir>` and `I18N_UNITS_ERROR=<path>` point it at another copy |
| i18n probe suite | does that census still NOTICE a dropped family, a count that drifted from its source, an invented family, a variant added to the CRATE, a mistiered safety family, an unjustified exemption, an undeclared diagnostic, a dead clause — and stay green on an edit that breaks no rule? | `bash docs/tasks/artifacts/i18n/run_i18n_probes.sh` → `probes: N pass / M fail` |
| command-layer census | is the command contract still the roadmap's §4.4 and §7.8 — every command the roadmap names carried, every row's class / authority / reversibility inside the vocabularies §1 and §7 declare, the five authority levels equal to the five §7.8 names in both directions, `approve` still human-only, every parity column grounded, every diagnostic declared, every link resolving? | `bash docs/tasks/artifacts/command_layer/run_command_layer_census.sh` → `command-layer census: N commands / M classes / K levels / 0 failure(s)`; `COMMAND_BOOK=<dir>` points it at another copy |
| command-layer probe suite | does that census still NOTICE a dropped roadmap command, an undeclared class or authority or reversibility, a sixth authority level, a missing one, a softened human-only rule, an ungrounded parity column, an undeclared cell value or diagnostic, a dead clause — and stay green on an edit that breaks no rule? | `bash docs/tasks/artifacts/command_layer/run_command_layer_probes.sh` → `probes: N pass / M fail` |
| G0 exit review | is gate G0 met — every fragment of roadmap §11's exit list dispositioned, each clause's cited check RUN, and a human-act clause named with its blocker? | `bash docs/tasks/artifacts/g0_exit/run_g0_exit_review.sh` → the clause-by-clause review, then `G0 EXIT: N met / M not met / K clauses — <verdict>`; the data plane is `g0_exit_clauses.tsv`; `G0_EXIT_SKIP_CHECKS=1` closes the clauses without running the checks |
| G0 exit-review probe suite | does that review still NOTICE a dropped clause, a row keyed on words the roadmap does not contain, a clause the roadmap grows, a missing deliverable, a failing check, an unowned human act, a malformed row — and stay green on an unrelated edit? | `bash docs/tasks/artifacts/g0_exit/run_g0_exit_review_probes.sh` → `probes: N pass / M fail` |
| book publication probes | are chapter/index coverage, progressive routes, source/rendered links and scoped API/status owners aligned? | `bash docs/tasks/artifacts/book_publication/run_book_publication_probes.sh` → chapter/API/link counts, then named refusal probes; semantic/runtime proof stays in canonical tests |
| all probe suites | does every diagnostic probe in the repository still discriminate? (scratch pinned to this volume) | `make probes` → per-suite `probes: N pass / M fail`, then `N suite(s) green` — the count is the command's, never a number kept by hand |
| Knowledge Map | is the derived orientation map in sync with its sources? | `knowledge-map/scripts/check_knowledge_map.sh`; regenerate with `knowledge-map/scripts/gen_knowledge_map.sh > "$(knowledge-map/scripts/gen_knowledge_map.sh --print-map-path)"` |

⚠ Run probe suites through `make probes`, which pins `TMPDIR` to `target/scratch` on the repository
volume: the inherited suites call `mktemp -d`, which otherwise lands on the system volume (defect
D16, leaf `SPINE.10`). The shared `scripts/check_task_acceptance.sh` still takes one trap-cleaned
scratch directory from `mktemp -d` at commit time; that residual is recorded in the leaf, not
patched into shared code.

⚠ **A RED arm must remove the property, not one instance of it.** Measured twice in one session: an arm
that renamed a diagnostic token renamed its declaration and its use together, and an arm that de-cited one
row left two other rows citing the same clause — both passed at `exit=0` where a refusal was owed, so both
reported the census sound while testing nothing. Before trusting a RED arm, ask what else in the tree still
satisfies the rule; if anything does, the arm is a report. Its sibling trap: a "portability" edit to a
regex (`{2,4}` → `###+`) silently stopped matching `##` headings, so a coverage rule required three clauses
fewer and still printed green — only a GREEN arm over the real tree plus a RED arm over one citation caught
it.

⚠ Probes and checks must **pin the instrument they measure with**. This machine's `PATH` puts GNU
coreutils ahead of BSD userland, so `stat -f %m` (BSD mtime) means "filesystem status of a file named
`%m`" and fails; `awk` is BSD awk 20200816, which lacks the GNU regex extensions `\b` and `{n}` that
`grep -E` here supports. A probe that measures with whatever is first in `PATH` measures the `PATH`.

History retrieval/pressure: `bash scripts/history_archive.sh verify` checks full-file identities and
resident/decoded bounds; `list`, `read docs/history/<basename>` and `materialize target/<fresh-dir>`
recover exact original records without Git history. `prove-source window1` separately compares the
capture against its named Git snapshot when available. Calibrated refusals:
`bash scripts/check_archive_retention.sh --self-test` (Python 3.9+ standard library; no packages).

Formula lexer guard proof: `bash docs/tasks/artifacts/formula_lex/run_formula_lex_mutations.sh`
mutates actual production sources, expects nine assertion reds and restores exact bytes. Run alone;
no overlapping build/gate/commit. The library contract tests cover borrowed spans and lexical scope.

Formula structural controls: `bash docs/tasks/artifacts/formula_structure/run_formula_structure_probes.sh`
loads the actual reference definitions and checks independently built node/depth boundary fixtures plus
copied-book refusals. `run_formula_structure_mutations.sh` in that directory disables four actual
reference guards and restores exact bytes; run alone. This is reference evidence, not a product parser.

Reference input parity: the structural suite also runs 130 spelling/keyword/unit-gap/argument controls
and three copied-book refusals. `bash docs/tasks/artifacts/formula_structure/run_formula_input_mutations.sh`
disables nine actual input guards, requires assertion reds and restores exact source bytes. Run alone;
no overlapping reference/probe/build/gate/commit. This verifies curated reference input, not product parsing.

Product expression syntax: `cargo test -p sc-core --test formula_expression_contract` covers precedence,
spans, units, bounds, 20736 short inputs and small-stack grouping. The structural suite checks twelve
explicit shared shape/count/depth fixtures with the actual reference. Production guard/order proof:
`bash docs/tasks/artifacts/formula_structure/run_formula_expression_mutations.sh` requires eleven
assertion reds and exact restoration. Run alone; no overlapping build/reference/probe/gate/commit.

Public rounding endpoints: `cargo test -p sc-units --test round_contract` (also `--release`)
checks i128 inputs and signed i64/zero/tie refusals. `round_reference.py` in the formula_structure
artifact directory independently verifies 36 exact Fraction rows; that directory's structural suite
runs it. `bash docs/tasks/artifacts/formula_structure/run_round_mutations.sh` requires five real
production assertion reds/exact restoration; run alone. No complete literal/evaluation proof implied.

Whole normalized arenas: `cargo test -p sc-core --test formula_normalized_contract` exercises eight
public contracts/24 independent reference shape rows/nested100 literal inputs/25 book expressions,
source/privacy/atomic refusal/argument/limit/small-stack behavior. Structural probes watch
`normalized_expression_reference.py`; `run_normalized_expression_mutations.sh` requires17 actual
compiled assertion reds/exact restoration, exclusively. No type/name/binding/evaluation certificate.

Individual product literal conversion: five public contracts/100 independent Fraction rows in
`literal_normalization_reference.py` cover exact unit/reduction/width/quantum/scalar boundaries.
`run_literal_normalization_mutations.sh` compiles13 actual assertion reds and restores exact source;
run exclusively. Structural probes watch the fixture verifier. Native/release/WASM/source/privacy
proof is scoped to one literal, not a normalized arena or recipe execution.

Unsigned round primitive: `cargo test -p sc-units --test unsigned_round_contract` exercises five
public full-u128/tie/zero/context/signed-bridge contracts and138 Decimal fixture rows. The structural
suite watches `unsigned_round_reference.py`; --emit reproduces its authored boundary population.
`run_unsigned_round_mutations.sh` requires nine compiled debug assertion reds and one release wrap
red, with byte-identical restoration; run exclusively. Existing signed four/36/five remain required.

Public round diagnostic: `bash docs/tasks/artifacts/formula_structure/run_round_diagnostic.sh`
links the actual current Cargo artifact and prints caught unwinds/typed results for six fixed inputs.
It is a diagnostic producer, not a passing verdict; round_contract judges the values.

Reference literal identity: `literal_contract.py` in formula_structure checks 60 explicit rows/361
controls with independent Decimal rounding, kind-preserving respellings and sums/signs; the structural
suite runs it. `bash docs/tasks/artifacts/formula_structure/run_literal_mutations.sh` requires six
actual guard assertion reds/exact restoration; run alone. `literal_diagnostic.py` in that directory
prints actual literal/arithmetic/domain observations; diagnostic rc=0 is not a correctness verdict.
D83/D84 scoped reference reviews are complete; production normalization/evaluation remain separate.

Reference exact arithmetic: `arithmetic_contract.py` in formula_structure checks 24 explicit rows,
100 independent Fraction parameter cases and 162 precision/dimension/binding/selector controls;
structural suite runs it. `bash docs/tasks/artifacts/formula_structure/run_arithmetic_mutations.sh`
requires nine actual assertion reds/exact restoration; run alone. D83 numeric/D84 angle reference
reviews are complete; selector scope is the reference length-only model, not curve accuracy.

Reference angular guards: `angle_contract.py` in formula_structure checks 42 angular rows/72 controls;
`angle_math_oracle.py` independently checks those defined curated rows with standard-library math.
Structural suite runs both. `bash docs/tasks/artifacts/formula_structure/run_angle_mutations.sh`
requires seven actual conversion/direction/pole reds/exact restoration; run alone. Decimal60 proof
scope is curated, not arbitrary transcendental correctness. D83/D84 reference reviews are complete.

Signed-angle reference: `signed_angle_contract.py` in formula_structure supplies90 independent
principal/binding/equality/sweep/copied-book controls. Structural suite watches it;
`run_signed_angle_mutations.sh` requires fifteen compiled actual assertion reds and exact restoration,
run exclusively. Curated branches/endpoints do not certify arbitrary transcendental inputs.

Reference rational refusal: `rational_contract.py` in formula_structure checks 61 independent Fraction
boundaries, reduced internal results, converted input before rounding and taken-only computation.
Structural suite runs it. `bash docs/tasks/artifacts/formula_structure/run_rational_mutations.sh`
requires twelve actual assertion reds/exact restoration; run alone. Bound/measurement/operation must
appear in formula_domain. Angle pole tests require their reason, so another domain failure cannot mask
removed guards. Scalar/i64/signed-angle reference controls pass; no product evaluator.

Public length operators: `cargo test -p sc-units --test length_operator_contract` checks four contracts
with a nine-by-nine i128 oracle, inclusive endpoints, signed crossings and explicit Result typing.
`bash docs/tasks/artifacts/formula_structure/run_length_operator_mutations.sh` requires six compiled
production bypass/operation/saturation assertion reds and exact source restoration; run alone.
These primitive checks certify no formula evaluation or release.

Domain context: `cargo test -p sc-units --test domain_context_contract` verifies five public typed/
rendered direct/forwarded refusal contracts. `cargo test -p sc-core --lib domain_context_contracts`
verifies three private totality guards; invalid arms are unreachable through validated journals.
`bash docs/tasks/artifacts/formula_structure/run_domain_context_mutations.sh` requires fourteen
compiled context/rendering assertion reds and exact multi-source restoration; run alone. Operation
labels and neutral wording preserve numeric payloads; no external geometry or MCP proof is claimed.

Inline documentation context: `bash docs/tasks/artifacts/formula_language/run_inline_context_contract.sh`
checks thirteen independent copied-book verdicts; the existing language probe suite watches them.
`bash docs/tasks/artifacts/formula_language/run_inline_context_mutations.sh` requires five actual
classifier assertion reds and exact restoration; run alone. Explicit Rust context excludes one
span outside normative formula parts only; malformed context refuses, adjacent formulas stay checked.

Scalar reference: `python3 -I -B docs/tasks/artifacts/formula_structure/scalar_contract.py` checks
57 independent Fraction/domain controls; the structural suite watches them. Actual declarations
supply signed length/area and nonnegative Count bounds; quiet numeric setup executes no unrelated
contracts. `bash docs/tasks/artifacts/formula_structure/run_scalar_mutations.sh` requires eleven
actual assertion reds and multi-source byte restoration; run alone. Separate binding/angle families
complete the scoped D83/D84 review; scalar checks alone supply no production numeric evaluation.

Numeric binding reference: `python3 -I -B docs/tasks/artifacts/formula_structure/binding_contract.py`
checks80 independent Fraction/Decimal controls; structural probes watch them. Numeric let rounds
once into declared signed storage; exact temporaries retain wider width. Count/scalar domains still
apply. `bash docs/tasks/artifacts/formula_structure/run_binding_mutations.sh` requires twelve
compiled actual assertion reds and byte restoration; run alone. Copied-book replay verifies the
census consumes returned bound integers. Production evaluation and full canonical proof remain owned.

Binding replay reference: `binding_replay_contract.py` in formula_structure exercises the published
census with all six kinds, signed Area rounding/replay, Boolean state/reads and format/declaration
refusals:19 independent verdicts. Structural suite watches it. `run_binding_replay_mutations.sh`
requires nine compiled actual assertion reds and byte-identical restoration; run exclusively.

Canonical literal reference: `canonical_literal_contract.py` under `docs/tasks/artifacts/formula_structure/`
checks146 independent node/Decimal controls; the structural suite watches them. Width128 means
absolute reduced magnitude, not signed i128; unary identity and later binding storage remain distinct.
`bash docs/tasks/artifacts/formula_structure/run_canonical_literal_mutations.sh` requires twelve
compiled actual assertion reds/exact restore; run alone. D95 closes; D83 review and production proof remain.

Retained-window CLI controls: `python3 -I -B docs/tasks/artifacts/history_archive/window_contract.py`
checks every listed/read/materialized logical record and newest-window digest/member/catalog
refusals plus cross-window collision. The archive probe runner watches it; no source Git is needed
for retrieval. Newest committed catalog edits are refused after the recording commit. Capture tool
`capture_window2.py` in that directory prepares/proves its fixed372033f snapshot in target/ only.

Ledger target controls: `python3 -I -B docs/tasks/artifacts/changelog/ledger_pointer_contract.py`
checks13 independently authored actual POINTER verdicts; the ledger runner watches them.
`ledger_pointer_mutations.py` in that directory requires four actual assertion reds and exact source
restoration. Run mutations exclusively: they temporarily edit the checker they test.

Artifact cleanup: python3 -I -B docs/tasks/artifacts/artifact_cleanup/cleanup.py plan
 target/artifact_cleanup_audit/<run>; apply the same run with apply. Safety/exclusions:
docs/ARTIFACT_CLEANUP.md. The standing make probes runner watches its refusal controls.

Static signature oracle: `python3 -I -B docs/tasks/artifacts/formula_structure/static_signature_contract.py --mutations`
checks closed kind/function matrices with value access trapped; the structural runner watches it.

Static namespace/header oracle: `python3 -I -B docs/tasks/artifacts/formula_structure/static_namespace_contract.py --mutations`
checks1139 metadata-only cases/thirteen actual guard reds; the existing structural runner watches it.
Whole static recipe/consumer oracle: `python3 -I -B docs/tasks/artifacts/formula_structure/static_recipe_contract.py --mutations`
checks196 cases, replay/measurement controls and actual guard reds; watched by the structural runner.
Static review: `python3 -I -B docs/tasks/artifacts/formula_structure/static_review_contract.py --mutations`
checks21/13 book rows, envelope precedence and observed D124 forms; no runtime/exclusion approval.

Assertion diagnostics: `python3 -I -B docs/tasks/artifacts/formula_structure/assertion_contract.py --mutations`; watched by structural suite.
