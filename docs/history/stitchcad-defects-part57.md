# Sealed archive — D136/D137 missing call payload reports

Immutable historical segment, sealed by leaf `G1-SLICE.5b.3c.2a` on `2026-10-03` (UTC).

- **Sealed identity:** 19 lines, 1685 bytes, `sha256:87147b3905a52c63355ee0b7af61f53ae7d82f5371fc01ff545e01a819a2d509`
- **Coverage:** D136/D137 reports; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D136** — actual unknown-call reference errors carry empty arguments, although contract5.2
  requires the name and origins searched. Tools-first reproduction via
  static_signature_contract.load_reference followed by infer(parse(source),{}) on
  loop(missing_argument), unlisted_call(1 mm), sin_missing(1 deg) returns formula_unbound_name
  with {} each (three controls, rc=0). The static signature/review probes check the token here,
  not payloads; their previous scoped kind/recognition proof does not certify these arguments.
  Root cause: infer_call raises FErr without arguments before checking operands. Impact: typed
  diagnostic consumers lose the callee and cannot distinguish actual catalog lookup from data
  declaration lookup. Owner G1-SLICE.5b.3c.2a; blocking checker interfaces, fix next. Retain the
  token/grammar, actual query and real searched call-source domains; never invent the nine data
  origins for a call. Document the interface before reference/product repair and compiled controls.

- **D137** — all six actual envelope call refusals carry {}, omitting envelope10's requested
  curve/constraint kind and supported curve set/recipe alternative. Reproduction through actual
  load_reference/infer(parse(alias+'(missing_argument)'),{}) for nurbs/spline/bspline/solve/
  constraint/fixpoint yields the correct named token with {} (six controls, rc=0). Calls precede
  operand lookup as required; caller knows the construct spelling, not any specific geometric
  constraint's parameters. Owner G1-SLICE.5b.3c.2a; fix now alongside D136, retain actual call
  scope/request and normative alternatives without inventing geometry or absent constraint data.
