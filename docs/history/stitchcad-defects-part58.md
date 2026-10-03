# Sealed archive — D138 missing dimension payload report

Immutable historical segment, sealed by leaf `G1-SLICE.5b.3c.2b.1` on `2026-10-03` (UTC).

- **Sealed identity:** 7 lines, 684 bytes, `sha256:660a79f2de58e26a77dfb598f803867cd3351eba6ad67852f648ad488e9c0508`
- **Coverage:** D138 report; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D138** — actual dimension refusals carry {}, omitting formula5.2's operation, every operand
  kind and wanted rule. Actual infer/parse on 1 mm + 1.0, - 1, sqrt(1 mm), within(1 mm,1 mm,
  size_count), if(1,1 mm,1 mm) yields formula_dimension with {} (five controls, rc=0). Root cause:
  infer and infer_call provide only prose. Impact: no typed actual/wanted diagnostics for consumers;
  earlier complete reference kind/recognition matrices certify tokens/acceptance, not these fields.
  Owner G1-SLICE.5b.3c.2b.1; fix next before bounded product checker .2b.2, using verified closed
  wanted catalogs and actual kinds, with deterministic multi-error selection documented first.
