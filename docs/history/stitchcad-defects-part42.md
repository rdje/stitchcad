# Sealed archive — D112 and D113 reference signature defects

Immutable historical segment, sealed by leaf `G1-SLICE.5b.1a` on `2026-10-02`.

- **Sealed identity:** 22 lines, 1734 bytes, `sha256:11dc6f70f4f0d83b03b2aaba2c172218c580c2328a4b1ebbf1d94bf176fdfb86`
- **Coverage:** D112 and D113; report bodies retained; entry markers normalized before sealing.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D112** — within admits reserved size names as tolerance operands

- Status: `open`; priority: blocking the static-signature oracle; owner: G1-SLICE.5b.1a.
- Reproduction: actual Evaluator parse/infer with the published within signature and reserved
  kinds accepts within(1 cm, 1 cm, size_index), size_count and is_base_size as boolean.
  The 2026-10-02 direct baseline prints ACCEPT boolean for all three, rc=0.
- Root: infer_call tests membership in all eight reserved names, while _matches deliberately
  skips the tolerance operand's kind. ADR-0003 and grammar notation require one of five tolerance names.
- Impact: reference signature evidence can certify a size ordinal/Boolean as a tolerance class.
- Scheduled fix: this active leaf narrows the accepted role and tests every reserved candidate,
  numeric/derived operands, all arithmetic kinds and actual removed-guard assertion failures.

- **D113** — min and max reject the one-argument variadic base case

- Status: `open`; priority: blocking the static-signature oracle; owner: G1-SLICE.5b.1a.
- Reproduction: actual Evaluator parse/infer with grammar min/max rows rejects min(1 cm) and
  max(1 cm) with formula_dimension; direct baseline on 2026-10-02 rc=0 reports both refusals.
- Root: _matches imposes max(2, len(want_list)); grammar5 defines variadic from the number
  of kinds listed, and min/max each list one T. Parser independently requires at least one argument.
- Impact: documented variadic arity and reference disagree before any product validator exists.
- Scheduled fix: this leaf uses the documented one-kind minimum, clarifies the book example,
  checks all eight kinds/arity boundaries and faults the actual predicate. No new language extension.
