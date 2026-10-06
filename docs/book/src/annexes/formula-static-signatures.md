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

