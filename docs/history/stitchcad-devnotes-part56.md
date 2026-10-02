# Sealed archive — observed runner verdict

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3c.1` on `2026-10-02`.

- **Sealed identity:** 12 lines, 1047 bytes, `sha256:cb01f979db690d51973af04a18eb1c61268c1ddee83ddd33f7ec5bf23190db4e`
- **Coverage:** observed CI lesson; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## _(2026-10-02)_ — retained controls need an observed runner verdict

- G1-0052 changes doctrine archive inputs, requiring immediate push despite the400 cadence.
  Verified exact f876913 heads and observed both completed CI jobs, every step success; archive
  prerequisite/enforcer plus Rust fmt/clippy/tests/WASM pass. Post-commit CLI140 controls also pass.
- The newest catalog immutability arm now executes against committed bytes and refuses its actual
  edit. Earlier capture and task/ledger payloads remain exact; no product behavior changed here.
- Initial automatic approval rejected default-main export. Existing public origin/push permission and
  task-owned outgoing payload were checked; the same required push then approved and succeeded.
- Book/task/pointers now carry actual remote job/step evidence. D83 complete review .2 follows;
  no numerical/geometry or production signoff is inferred from this maintenance CI result.
- promotion: declined (routine observed verification of the existing CI exception and archive contract).
