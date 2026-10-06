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
