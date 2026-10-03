# Call diagnostics retain actual lookup sources and requested constructs

- **Type:** `decision`
- **Date:** `2026-10-03` (UTC)
- **Status:** `active`; engineering-verified, independent approval unclaimed
- **Owner / source:** `G1-SLICE.5b.3c.2a`, session's autonomous engineering directive;
  D136/D137 reproductions and formula contract §5.2/5.3, envelope §10, units §4.

answers: "which origins are searched for a formula call?" · "what kind can an envelope call refusal truthfully report?" · "does a scalar declaration make its name callable?"

## Decision

Call lookup searches the envelope aliases, then the closed built-in catalog. Its typed sources are
Envelope and BuiltinCatalog, distinct from the nine data-name origins. An unknown callee retains its
exact borrowed name and both sources in search order. A scalar or reserved data name is not callable.

An envelope refusal stops at Envelope and retains the exact alias as its requested construct in
formula_call scope. It reports the declared supported curve set or ordered recipe alternative;
it cannot infer geometric constraint parameters, an entity, or a recipe ordinal from a callee.
Tokens, function population, source grammar and keywords remain unchanged.

## Evidence and application

D136/D137 exposed correct refusal tokens with empty arguments. Their repair uses one actual
reference callee check at static/runtime entry points, before argument access. Canonical schema
was documented before implementation in `docs/book/src/spec/formula-language.md` §5.2.2 and
`docs/book/src/spec/feature-matrix.md` §10. Product FormulaBuiltin::resolve_call accepts a validated
MachineToken and returns existing metadata or a privately constructed, query-borrowing refusal.

The public accessors expose exact name, scope, reason, searched sources and typed alternatives.
Debug omits authored payload; Display is token-only. Name-only lookup claims no span, canonical
expression, numerical value or geometry. Enclosing checkers attach only context they actually own.
Syntax/input validation precedes static callee dispatch; argument name/kind errors cannot mask it.

Five public contracts/two negative lifetime/privacy examples and18 actual compiled source faults,
plus166 independent reference cases/12 compiled faults/three loaded normative-set faults, verify
the scoped repair. Source bytes restore exactly. Wanted dimension payloads and accepted expression,
whole-recipe, numerical and geometry proofs remain separately owned at .2b/.3/.4/.5c–.5g/G2.

A future change to the closed call population or curve/recipe alternatives must update the canonical
schema, typed catalogs, independent controls and book together. New call sources must describe an
actual search, rather than copying data origins into a domain that never searched them.
