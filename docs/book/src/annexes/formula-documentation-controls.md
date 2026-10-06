# Formula documentation controls

The book keeps formula examples and foreign-language code distinct using explicit author context.
Product syntax and its verification remain in the [syntax annex](formula-syntax.md).

## Inline documentation language context

The book vocabulary census cannot infer a code span's language from punctuation: a Rust error-handling
expression can resemble a formula but legitimately use a question mark. D91 supplies explicit author
context for a single inline span in chapters outside the three normative formula parts:

```markdown
<!-- stitchcad-inline: rust -->`(left + right)?` and `waist_girth + ease_waist`
```

The marker must be exact, on the same line and immediately followed by the one backtick span (optional
whitespace only). It excludes that Rust span from formula-vocabulary classification; the adjacent
formula remains checked. Unknown, malformed, duplicate or detached markers refuse L6e. Normative
formula contract/grammar/example parts reject foreign annotations, preserving their formula positions.
Rust fences retain their existing scope. This declares author intent; it does not compile or certify
Rust and does not extend the formula alphabet. An unannotated question mark still refuses L6b.

```bash
bash docs/tasks/artifacts/formula_language/run_formula_language_probes.sh
bash docs/tasks/artifacts/formula_language/run_inline_context_mutations.sh
```

The watched suite includes thirteen independently authored copied-book context verdicts alongside
its fifteen existing agreement/refusal arms. Five actual classifier mutations discriminate removed,
whole-line or normative exemptions, missing refusal counts and unknown-language acceptance; they
require assertion reds and byte-identical restoration. Run mutations alone. Numeric reference and
product syntax behavior are unchanged; evaluator/API/MCP and release proof remain separately owned.

## Copied-book and classifier-writer plans

The context fixture module validates its own source, the local profile and the reference shell on
import. Importing it creates no stores, copies or child processes. fixture_plan(activate=False)
returns the complete validated plan without activation. The plan covers the whole book, all copied
files/directories and the effective shell selectors. Unexpected old entries, links, nonregular inputs,
hardlinked outputs and selected inputs or parts directories inside the replacement tree refuse before
replacement. For example, FORMULA_PARTS_DIR pointing inside target/formula_inline_context/src refuses
before deleting that directory, even when the selected grammar and examples point elsewhere.

contracts() activates prepared local stores, executes all thirteen original cases and rechecks the
replacement before each copy. Copied inputs are validated before editing, then source and selector
plans are refreshed before each shell child. Caller selectors and unrelated environment values are
preserved; relative selectors resolve from the repository root. Overrides can change the expected
verdicts in these authored fixtures. The ordinary command verifies the default complete copied book.

The classifier writer is also inert on import. Its producing plan includes the fixture child's complete
replacement tree, five logs and the single writable shell source. The child plan must pass before
activation or classifier changes. Each source change, child and log receives a fresh plan. Both
restoration paths validate the writable source alone, so an invalid late log cannot prevent restoration.
Run the writer exclusively, with no other reference readers or source-mutating jobs:

```bash
python3 -I -B docs/tasks/artifacts/formula_language/inline_context_contract.py
python3 -I -B docs/tasks/artifacts/formula_language/inline_context_mutations.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_inline_context_contract.py
```

Invalid standalone plans print `local-environment: REFUSED — …` and exit2. Borrowed pure plans raise
the shared provider's Refusal; unrelated assertion failures keep their original type. The watched
last command exercises actual imports, producing calls, pure plans, child-failure restoration, guard
removals and isolated filesystem faults. It also removes the actual nested old-entry guard to prove
that refusal depends on that boundary. Intercepted children prove dispatch; the first two commands
provide the separate thirteen real verdicts and five actual classifier assertion failures.
These producer plans change no annotation, vocabulary, grammar or engine rule.

## Assertion reference and copied consumer plans

The assertion module checks its own file, profile, shell and public signature loader before importing
that loader. It then uses the loader's provider and Refusal class. Import and contracts() validate
sources without creating stores, copying files or invoking a shell. The original262 assertion cases
and eight compiled in-memory faults retain their thresholds, payloads and earlier-error precedence.

consumer() prepares a complete plan for every book source, copied file, directory and false_closure.log.
fixture_plan(activate=False) returns that plan without effects. Effective shell selectors are checked
before replacement; selected files or parts directories inside the replacement refuse. Links,
nonregular inputs, unexpected old entries and hardlinked outputs also refuse before activation.
The replacement is rechecked before deletion, copied inputs before editing, source/selector plans
before the child and the output plan before logging. Prepared stores, unrelated environment values,
relative caller selectors and argv survive. Selector overrides can change the authored consumer verdict.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/assertion_contract.py --mutations
python3 -I -B docs/tasks/artifacts/ci_environment/python_assertion_fixture_contract.py
```

The first command supplies262 real reference cases, one actual copied-book refusal and eight intended
body assertion failures. The second captures actual entry/helper bodies and dispatch, removes guards
and exercises isolated filesystem refusals. Its intercepted child is dispatch evidence; it does not
supply the first command's numerical result. Invalid CLI plans exit2 with the named local-environment
refusal; borrowed pure calls raise the shared provider's Refusal. Assertion failures keep their original
type. No reference source, grammar, threshold, diagnostic token or production engine rule changes.

## Binding fixture plans

The numeric binding reader and mutation writer retain their producing-on-load interfaces. Running
or loading either entry executes its original work. Use the separate binding_fixture.py provider to
inspect a plan without starting that work: importing this provider and calling binding_plan() validate
sources, stores, effective selectors and destinations without creating, copying or running anything.
Its public loaders share the actual local-environment provider and Refusal class.

The producing plan includes every book source, copied directory/file and replay.log inside
target/formula_binding_source. The writer additionally plans all twelve mutation logs and requests
mutable-source authority only for the reference shell. Both entries validate their loader sources
before loading; the writer calls the pure child plan before activation or changing shell bytes.
Links, nonregular files, another repository, hardlinked outputs and unexpected old fixture entries
refuse before replacement. Hardlinked mutable shell sources refuse; read-only sources may share an
inode. Selected files or parts directories inside the replacement tree also refuse, even when
other selected files point elsewhere. For example, selecting FORMULA_PARTS_DIR inside
target/formula_binding_source/spec/formula-language refuses before deleting that directory.

The reader refreshes its plan before replacement, copied input reads/edits, the census child and its
replay log. The writer refreshes before every shell edit, child and log. Its ordinary and final
restoration paths validate only the writable shell, so an unrelated late output failure cannot strand
a mutation. Relative selectors resolve from the repository root; prepared stores, unrelated environment
values and argv remain intact. Overrides can change the authored copied-book verdicts; ordinary commands
use the complete default copied book. No formula grammar, storage width, rounding or token changes.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/binding_contract.py
python3 -I -B docs/tasks/artifacts/formula_structure/binding_mutations.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_binding_fixture_contract.py
```

The reader supplies80 real Fraction/Decimal controls and actual replay of nineteen bindings with zero
mismatches. The writer requires all twelve original compiled faults to fail the binding assertions and
restores the exact shell bytes. Run the writer exclusively, with no overlapping reader, build or gate.
The watched last command exercises actual module bodies, standalone and producing borrowed loads,
pure plans, independent provider identities, guard removals and isolated filesystem faults. It proves
actual mirrored source restoration after a late log refusal. Intercepted children establish dispatch;
they do not replace the first two commands' real numerical and mutation results.

Invalid standalone or producing borrowed plans print `local-environment: REFUSED — …` and exit2.
Borrowed pure provider calls raise that provider's Refusal. Unrelated assertions retain their original
type. This protects reference-verification producers; production evaluation, geometry, API/MCP and
release signoff remain separate roadmap work.

## Binding replay plans

The replay reader and its nine-fault writer retain producing-on-load behavior. Their separate
binding_replay_fixture.py provider is inert on import; fixture_plan() validates the complete plan
without activation. It reuses the public input loader's tree walker and effective shell selectors,
with the actual shared provider and Refusal class. Every loader source is checked before delegation.

Each replay() call prepares all book sources, replacement files/directories and replay.log inside
target/formula_binding_replay. It rechecks the replacement before deleting/copying, copied sources
before editing the examples or Boolean declaration, sources/selectors before the census child and
the complete output plan before its log. The formatter's public context is separately rechecked
before loading. A selected file or FORMULA_PARTS_DIR inside the replacement tree refuses before
replacement. Unexpected old entries, symlinks, foreign repositories/volumes and nonregular files
refuse; writable outputs and the writer's shell source must have a single inode link.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/binding_replay_contract.py
python3 -I -B docs/tasks/artifacts/formula_structure/binding_replay_mutations.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_binding_replay_contract.py
```

The reader supplies nineteen real consumer/formatter/declaration verdicts: seventeen copied-book
census calls plus two invalid Boolean display states. The first call adds fifteen independently
authored valid rows to the seventeen ordinary examples. The following calls check exact refusal
reasons for wrong values, unit/kind mismatches, opaque bindings and removed Boolean bindability.
For example, half a square micrometre binds to1 µm² and displays0.00000001 cm²; reading that stored
binding twice displays0.00000002 cm². A Boolean name retains Boolean kind and controls if().

The writer plans that entire child fixture and all nine logs before activation or a shell change.
Source edits, children and logs receive fresh plans. Ordinary and final restoration validate only
the writable shell, preserving exact restoration when a late unrelated log becomes invalid.
Run the writer alone, without overlapping readers, builds or gates. Prepared stores, unrelated
environment values, relative caller selectors and argv remain intact; selector overrides may change
the authored fixture verdicts. Standalone and producing borrowed refusals print the named
local-environment diagnostic and exit2; borrowed pure plans raise the actual provider's Refusal.

The watched last command executes actual module bodies, guard removals, isolated filesystem faults,
independent provider loads and mirrored late-log restoration. Its intercepted children establish
dispatch; the first two commands supply the separate nineteen numerical/display verdicts and nine
intended compiled assertion failures. No source literals, grammar, display quantum, diagnostic token
or production engine rule changes; evaluator/geometry/API/MCP/release proof remains separately owned.
