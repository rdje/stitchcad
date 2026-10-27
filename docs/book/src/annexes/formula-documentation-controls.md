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

Importing the inline-context fixture creates no stores, copies or children. Its pure
fixture_plan(activate=False) validates the complete book, outputs and effective child selectors before
activation. Unexpected old entries, links, nonregular files, hardlinked outputs and selected inputs
or parts directories inside the replacement tree refuse before deletion. Plans refresh before each
copy, edit, child and log. Caller argv/selectors/environment survive; relative paths resolve from the
repository root. Selector overrides can change authored fixture verdicts.

The classifier writer is inert on import and validates the complete child plan before activation or
source changes. Each change/child/log receives a fresh plan; both restoration paths validate only the
writable source, so a bad log cannot strand a mutation. Run the writer exclusively:

```bash
python3 -I -B docs/tasks/artifacts/formula_language/inline_context_contract.py
python3 -I -B docs/tasks/artifacts/formula_language/inline_context_mutations.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_inline_context_contract.py
```

## Assertion reference and copied consumer plans

Pure assertion imports/contracts freshly validate declared sources and share the provider's Refusal
identity, with no stores, copies or children. Assertion thresholds, payloads and earlier-error
precedence stay unchanged. Producing calls plan complete sources/copied trees/logs and effective
selectors before activation, then recheck replacement, copied edits, child and output boundaries.
Source/destination aliases and unsafe old entries refuse before effects; prepared stores, caller
argv/environment and relative-selector rules remain intact.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/assertion_contract.py --mutations
python3 -I -B docs/tasks/artifacts/ci_environment/python_assertion_fixture_contract.py
```

Invalid standalone plans exit2 with a named local-environment refusal; pure borrowed plans raise
shared Refusal, and unrelated assertion errors keep their own type. Intercepted children prove
dispatch boundaries; actual reference verdicts and compiled faults supply separate behavior evidence.
Complete original inputs, examples, populations, guard faults and restoration receipts remain whole
in `docs/history/stitchcad-devnotes-part536.md`; retrieve with
`python3 -I -B scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part536.md`.
These plans change no source/reference rule, annotation, vocabulary, grammar or production engine.

## Binding fixture plans

The binding reader/writer produce on load; use binding_fixture.py's inert binding_plan() to inspect
complete validated sources, stores, selectors and destinations without creating or running anything.
Both producing entries validate loader sources first; the writer checks the whole child plan before
activation or shell edits. Plans cover the complete copied book, outputs and mutable shell authority.

Links, nonregular files, foreign repositories/volumes, hardlinked writable files and unexpected old
entries refuse before replacement. Selected files/parts inside the replacement tree refuse even if
other selectors point elsewhere. Read-only sources may share inodes. Refresh plans before copies,
edits, children and logs. Restore using only the writable source plan, so bad logs cannot strand it.
Caller argv/environment and repo-relative selectors remain intact; overrides can change fixture verdicts.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/binding_contract.py
python3 -I -B docs/tasks/artifacts/formula_structure/binding_mutations.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_binding_fixture_contract.py
```

## Binding replay plans

Replay reader/writer entries also produce on load. The separate binding_replay_fixture.py provider
is inert; fixture_plan() validates without activation and retains the actual provider/Refusal identity.
Each replay plans complete book sources, replacements, selectors, formatter context and logs before
effects. Recheck deletion/copy, copied reads/edits, child dispatch and logging. The writer plans its
whole child and logs before activation, then refreshes each edit/child/log and restores only through
the writable source plan. Safety/alias/inode/selector rules above also apply to replay.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/binding_replay_contract.py
python3 -I -B docs/tasks/artifacts/formula_structure/binding_replay_mutations.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_binding_replay_contract.py
```

Run either writer exclusively and consume exact restoration before another reader/build/gate.
Producing standalone/borrowed plan refusals print a named local-environment diagnostic and exit2;
pure borrowed plans raise their actual provider's Refusal. Other assertion errors keep their types.
Intercepted children prove dispatch separately from actual numeric/display verdicts and compiled faults.
Complete original input lists, numerical examples, populations, faults and restoration receipts remain
whole in `docs/history/stitchcad-devnotes-part537.md`; retrieve with
`python3 -I -B scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part537.md`.
No grammar, quantum, rounding, diagnostic, numeric engine or production approval rule changes.

## Origin and provenance consumer plans

Both origin_value_contract.py and provenance_contract.py are inert on import. Borrowed contracts()
validate only the public numerical reference inputs and keep the caller's environment and argv.
They need no copied fixture or prepared stores. Unused sidebar or output faults do not affect a pure
call; read-only source hard links remain valid. The shared origin_provenance_fixture.py provider
checks its loaders before importing them and offers a pure reference_plan() and fixture_plan(labels).

Producing consumer() calls prepare the complete replacement, selectors, logs and child stores before
any copy, deletion, edit or shell dispatch. Origin plans both missing_size and missing_tolerance at
once, so a bad destination in the second case refuses before the first case produces anything.
Provenance also plans its copied mutant.sh even when the current call runs the original shell.
Its compiled faults edit only that copy; the tracked evaluator is never rewritten.

For example, an existing linked consumer.log, a nested Git boundary in a replacement, an unexpected
copied file or an input selector pointing into a replacement raises Refusal before activation.
A FORMULA_PARTS_DIR inside a replacement is refused even when the chosen grammar and examples come
from valid source files. Caller selectors keep the shell's existing meaning; they are checked again
with the actual copied book immediately before dispatch. Prepared local stores and unrelated
variables remain in the child environment. Original argv and result checks remain unchanged.

Replacement checks refresh before deletion/copying, copied inputs before edits, complete sources and
selectors before children, and the complete producing plan before logs or mutant-script writes.
Standalone CLIs report local-environment: REFUSED and return2 for these path/store failures;
borrowed calls raise the adopted provider's Refusal. Numerical assertion failures keep their original
meaning. Actual independently loaded provider classes are checked, including delegated refusals.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/origin_value_contract.py --mutations
python3 -I -B docs/tasks/artifacts/formula_structure/provenance_contract.py --mutations
python3 -I -B docs/tasks/artifacts/ci_environment/python_origin_provenance_contract.py
```

The first two commands retain1466 origin controls and425 contribution controls, thirteen and
26 intended faults, and the actual copied consumers described in the [runtime evidence](formula-runtime-validation.md).
The watched environment control intercepts child dispatch separately from that numerical proof.
It executes the actual bodies, removes guards to require independent assertion failures and exercises
real isolated filesystem refusals, repeated pure calls and provider identity. Its pure positives also
prove that unused linked sidebars/outputs and read-only source hard links do not introduce effects.
Python3.9 and3.14 are verified. These are documentation instruments; production execution, real
geometry, typed API diagnostics and garment signoff remain in their roadmap lanes.
