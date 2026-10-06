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
