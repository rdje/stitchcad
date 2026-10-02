# Sealed archive — measurement table bindings

Immutable historical segment, sealed by leaf `G1-SLICE.4d.1` on `2026-10-02`.

- **Sealed identity:** 18 lines, 1699 bytes, `sha256:3dc0b61983a73e4ffa9cdd9e7fcbabceb95d8d0dcd23bedc2272add9c279ec21`
- **Coverage:** measurement table binding lesson, copied unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## _(2026-10-02)_ — measurement tables pin bindings and borrow current scalar records

- A table owns stable id/name and authored order of measurement/token/kind/declaration bindings.
  It holds no copied values, source/state or procedure text. Context borrows unambiguous metadata
  and existing canonical targets; tokens are unique per table, while unrelated tables may share
  spelling. Empty named drafts and shared declarations are legal; names never establish identity.
- Saved bindings resolve by measurement id before expected token/domain/declaration checks. Removed
  measurements never transfer a token to peers. Same-id declaration state/source or procedure text
  revisions stay canonical/current; scalar identity reassignment requires explicit validated rebinding.
  Targeted queries check the selected metadata's required references; whole-table validation checks all.
- Sixteen contracts and a privacy doc pass; eight real production guard mutations fail assertions,
  not compilation, and restore source byte-identically. The initial mutation diagnostic rejected an
  unwrap_err panic as lacking an assertion marker; direct err comparison makes the contract explicit.
  Strict Rust executes 342 tests; WASM/book/censuses and milestone probes/gates verify the restore.
- Named tables complete .4a's structural field/reference contract; Ease/SizeSet/family signoff remain
  .4b/.4c/.4d, G1 stays 5/18. Caller Design revision, source/evidence truth, formula evaluation and
  physical procedure repeatability remain separate proofs. Completed metadata review relocates unchanged.
- promotion: promoted by `decision_length-declarations-retain-state-and-provenance.md`'s table section.
