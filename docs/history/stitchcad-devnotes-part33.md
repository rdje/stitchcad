# Sealed archive — archive capacity and retrieval

Immutable historical segment, sealed by leaf `G1-SLICE.4c.3c` on `2026-10-02`.

- **Sealed identity:** 18 lines, 1591 bytes, `sha256:aad7494ad08ead70ab156e9ec9900f322dcb602a620091ffc31f51d03e6b96fb`
- **Coverage:** archive capacity/retrieval lesson, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## _(2026-10-01)_ — archive capacity and retrieval must be verified together

- D65's 64-file limit blocks ordinary rollover despite a small decoded archive. One immutable
  content-addressed window retains all 64 original full files; bounded manifests/catalog preserve
  logical addresses. Copy/verify/use/source reconstruction precede exact working-copy retirement.
- Python standard-library maintenance tool uses bounded decompression and safe in-memory record
  reads, no tar extraction/network/old-Git dependency. Fresh same-volume target materialization
  refuses symlinks, nested repositories and overwrite. Hook/CI checks committed window immutability.
- Original per-part/aggregate bounds still govern decoded records; compressed resident history and
  finite control/payload collections are counted independently. Compression cannot hide growth.
- Existing ledger probes consume materialized logical records. D68: original coverage mutation
  appended a declaration grep -m1 ignored and passed on unexempted D30. Replace the actual first
  declaration in part2 and require exactly that refusal with D30's exemption retained.
- Calibrated archive refusals, exact source reproof, strict Rust/WASM/book/full probes and staged
  gates validate the transition. Post-commit probes 27/0 include committed-window mutation.
  .19.2v observes ebed2c5 Rust/doctrine jobs and all steps successful, including Python prerequisite
  and native/WASM. No source/physical/release truth is inferred from a digest.
- promotion: promoted by `decision_history-windows-retain-self-contained-bytes.md`.
