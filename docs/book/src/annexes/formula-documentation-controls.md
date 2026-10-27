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
