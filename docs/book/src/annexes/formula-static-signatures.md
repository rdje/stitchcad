# Reference formula signature evidence

This reference review complements [the complete static review](formula-static-validation.md).

## Complete signature matrix

The tracked static_signature_contract.py under docs/tasks/artifacts/formula_structure loads the
actual reference definitions and table-loading code from run_formula_language_census.sh. Expected
populations are authored independently: all eight kinds, five arithmetic kinds, four negatable
kinds, eight reserved names, positional products/quotients, and all 22 closed built-in/selector names.
Expected signatures are compared with the published populations in both directions; no extra or
missing reference row can disappear behind an example that happens not to call it.

The 4032 actual parse/infer cases cover:

- Every unary/square kind and all 64 ordered kind pairs for each binary operator.
- Every fixed function/selector signature at its arity over all positional kind combinations,
  zero/short/long arity refusals, and min/max at one, two and three arguments.
- Both variadic functions at the 255-argument syntax boundary and first excess, for all five
  arithmetic kinds. A wider call is a structural refusal, independently of its signature.
- All 512 condition/branch kind combinations, unresolved names in either branch and conditional arities.
- All eight reserved candidates against all 64 within operand pairs, plus numeric, derived and
  ordinary-name tolerance refusals and wrong arities.
- All six envelope call names, preserving their named precedence before inspecting an unbound
  argument, plus an undeclared ordinary call.

Environment entries expose only kind. Access to value, state, origin or geometry payload fails;
numerical evaluation, binding/storage and geometry callbacks also fail if invoked. Parsing still
uses the existing literal-input conversion where a control contains a literal; this is not a claim
that input normalization does no numerical work.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/static_signature_contract.py --mutations
bash docs/tasks/artifacts/formula_structure/run_formula_structure_probes.sh
```

Fourteen in-memory variants of the actual reference predicates must fail the matrix assertions:
quotient product advice, missing product advice, variadic base arity, tolerance role, count negation,
square result, addition consistency, directed quotient, variadic homogeneity, generic arithmetic
membership, Boolean condition, matching branches, envelope precedence and an environmental value
read through get. The producer on disk remains unchanged. These are actual failed body assertions,
not merely nonzero script exits.

### Reference source locations

The table-loaded reference is extracted from a Python heredoc inside the shell census. Its loader
preserves the original header line offset when compiling the normal reference, so tracebacks name
the real source line and show the matching source text. In-memory fault variants use an explicitly
virtual filename instead of claiming that their changed code is the unchanged source file.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/reference_locator_contract.py --mutations
```

D144 and D173 controls independently parse each prefix's top-level functions and direct class
methods/decorators, then compare original code positions and geometry-dimension traceback text.
Both loaders retain the85-line shell header; in-memory variants use virtual filenames. Losing
the offset or impersonating the original causes body assertion failures in each loader (four faults).
Sources, grammar, diagnostic arguments and results stay unchanged; structural probes watch both
catalogs. D174 removes stale fixed totals: definitions and locations derive from current source.


## Read-only reference plans

The signature, namespace and static-review helpers inspect their own bootstrap inputs before
loading another helper. Each borrowed reference call freshly validates the complete source list:
the shell reference, public helper chain, local profile and five normative book files (formula
contract, grammar, examples, reference skirt and units/tolerances). These calls create no stores,
files or child processes, and preserve prepared store values, unrelated environment values and
caller argv. The existing permissive `--mutations` recognition is unchanged.

Standalone invalid inputs print `local-environment: REFUSED — …` and exit2. A borrowed call that
reaches its reference plan raises the shared provider's `Refusal`; unrelated parsing/assertion
failures retain their own type. Missing files, symlinks, directory inputs and another Git boundary
refuse before reference reads. Invalid selected stores or toolchain names also refuse before effects.
Pure calls use only their declared inputs: they do not validate or copy the whole book tree.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/static_namespace_contract.py --mutations
python3 -I -B docs/tasks/artifacts/formula_structure/static_review_contract.py --mutations
python3 -I -B docs/tasks/artifacts/ci_environment/python_static_loader_contract.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_static_loader_contract.py --native
```

The watched control executes all four original bodies: imports, ordinary and mutation CLI modes,
borrowed contracts, both reference loaders and exception-path argv restoration. Per-call source
plans are checked independently before reads and delegation; omission of an actual guard must fail.
Real isolated filesystem faults and disabled actual planner bodies distinguish refusal from admission.
The control's `--native` mode activates guarded local stores, runs the published first-phase and
coupled consumers, and checks every frozen input after each child. Logs, source identities and the
Cargo-current library identity are saved under `target/loader-*`; the probe runner watches this mode.
Every fixture and capture store stays under repository-derived `target/` on the repository volume.
Python3.9+ and its standard library are read-only runtime dependencies.

The static metadata contracts retain their execution traps. The review helper's D126 parameter
quantum control deliberately evaluates the reference arc-length counterexample and independent
rational bounds; its numerical proof remains separate from product geometry signoff. The native
bridge and copied-book instruments check nested read-only plans as well as their producing plans.

The dimension- and ambiguity-payload libraries use the same pure boundary: bootstrap plans
cover four and five sources, and fresh borrowed plans cover nine and ten. They reuse the
provider's Refusal class. Their original diagnostic metadata, operand/source order, kind/value
traps, case populations and CLI argument recognition remain unchanged. For example:

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/dimension_payload_contract.py --mutations
python3 -I -B docs/tasks/artifacts/formula_structure/ambiguity_payload_contract.py --mutations
python3 -I -B docs/tasks/artifacts/ci_environment/python_payload_library_contract.py
```

The watched payload control checks repeated borrowed calls, exception-path argv restoration,
actual guard omissions, helper/book filesystem faults and unchanged prepared environments.
The native control above also runs both real payload mutation CLIs before the native consumers
and verifies frozen inputs after each. Diagnostic inference remains metadata-only.

The geometry-argument and call-lookup libraries also validate four bootstrap and nine reference
inputs. Geometry's original static refusals and successful numerical cache/provenance replay both
remain active; these in-memory checks create no files or stores. Call lookup retains callee-before-
argument refusal and its unchanged catalog. The standalone mutation flags retain their original rules.

The direct curve_contract() reader validates only its own script, the profile and the units/curve
chapter. It can inspect that table while an unrelated reference chapter is missing or an unused
log alias exists. A complete reference contract still refuses its missing required chapter.
The shared payload control checks these boundaries, repeated calls, all actual guard omissions and
physical helper/book refusals; the native control runs both original mutation CLIs before consumers.

`reserved_diagnostic_review.py` applies the same four-input bootstrap and fresh nine-input borrowed
plan before checking its canonical diagnostic row or compiling the reference. Its original 3,624
D131 source-bearing cases and nineteen faults retain formula_rebinding, actual available locations
and metadata-only inputs. It preserves permissive mutation argv and shares the provider's Refusal.
The payload control covers this fifth library; native mode runs its real mutation CLI too.
The [D131 decision](formula-static-validation.md#reserved-name-diagnostic-sources), grammar and
independent-approval boundary are unchanged by these file/source plans.

The location library validates six bootstrap inputs before either helper is loaded, then eleven
reference inputs before its original AST, traceback or delegated reference checks. Standalone
mutations also validate the selected helper source before reading or compiling it in memory.
Both loader parameters remain available; calls preserve caller argv and prepared stores. The
existing real filename/line/text checks and explicitly virtual fault filenames remain active.
For example, contracts() checks the signature prefix, review_contracts() checks the whole-review
prefix, and mutations(REVIEW_SOURCE, review=True) exercises the second actual loader. These calls
create no files or child processes. The shared static-loader control covers both borrowed paths,
repeated calls, selected-source guard omissions and physical helper/book refusals; native mode
runs the real CLI and both prefixes' original offset/filename faults before the engine consumers.
