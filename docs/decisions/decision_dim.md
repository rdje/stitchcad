# Dimension diagnostics retain complete actual and wanted operands

- **Type:** `decision`
- **Date:** `2026-10-03` (UTC)
- **Status:** `active`; engineering-verified, independent approval unclaimed
- **Owner / source:** `G1-SLICE.5b.3c.2b.1`, autonomous engineering directive and D138;
  canonical formula contract §5.2.3, grammar §5–§7 and typed wanted-kind catalogs .3c.1.

answers: "which error wins when a formula has several defects?" · "what does a dimension refusal retain?" · "how can a computed length differ from a tolerance symbol?"

## Decision and rationale

After syntax/input validation, known operations check child expressions left-to-right before
checking their complete signature. If uses condition/then/else; within checks all positional
operands before its symbolic-class rule. An unknown or envelope callee still refuses before any
argument. A child's unresolved name/kind wins over the parent's dimension mismatch: no actual
kind can be fabricated for that child. Both branches are statically checked without value reads.

An expression dimension refusal retains the actual operation, every resolved immediate operand
kind and direct reserved tolerance-name roles, plus all wanted signature alternatives. Ordered
requirements preserve exact kinds, shared arithmetic T, negatable N and the symbolic tolerance
role; variadic flags and results are retained. Class roles derive from direct actual name nodes;
grouping is transparent and computed lengths have no class role. No input state/value is copied.

The reference loads wanted operator/call rows from the canonical tables and uses one dimension
constructor. Its row result aliases express the table's shared kind; product FormulaKindSignature
provides corresponding typed requirements and actual result positions. This reference schema is
scoped diagnostic evidence, not a finalized command serialization or accepted-expression owner.

## Evidence and remaining owners

The canonical clause was documented before implementation. The independent producer checks4023
actual cases/3814 complete refusals and14 multiple-error selections;15 actual compiled faults
prove fields, complete alternatives, role/child order and absence of value reads. The earlier
4032-case matrix/14 faults remains green. Rust metadata APIs remain unchanged.

Header-only arguments remain D139/.5b.3c.3a; an invalid raw annotation has no guessed typed RHS.
D140's geometry-provider kind loss is P0 at .5f.3a, immediately before product expression checking.
Expression checking/statement integration/whole graph/numeric/geometry/release retain separate
owners. Tokens, grammar, canonical syntax identities and independent approval are unchanged.
