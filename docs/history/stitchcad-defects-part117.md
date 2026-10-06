# Sealed archive — D195 report and verified closure

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.f.n11.3` on `2026-10-06` (UTC).

- **Sealed identity:** 22 lines, 1723 bytes, sha256:7c2d15700c174d4c18f27602365e6fcfca54f0aae58e9cf04835ad1e01ddf362
- **Coverage:** report authored new-overHEAD; 68a6f9da91da7ed95633c9e393d8cfc849d58a51:docs/tasks/PLANNING.md authored bytes 80391–81079; complete original predictions/correction or defect report retained.
- **Retrieval:** content below the first rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer.

---

- **D195** (P2, fix now G1-SLICE.5b.4c.h2.b.p.o.f.n11.3): authored wanted fault predictions incorrectly
  included public builtin body I for cases2/5/17. Repro target/wanted-plan-native-proof.original.py
  verify refuses case2; independent all19 prediction census finds precisely3 extra I predictions/74
  inputs exact. I's nonvariadic assertions call FormulaBuiltin::result_kind, while faults change
  FormulaKindSignature matching; only its min/max assertions consume wanted rows. Impact: signoff
  prediction model falsely attributes3 failures; preserve original, correct3 after tool-backed
  diagnosis, reverify every executed body/current macro/restored both targets before closure.

### Verified closure
- Tool-first full19 census proves precisely3 false I predictions (cases2/5/17);
  every74 frozen input remains exact. I's Within/Abs/If assertions call the
  separate builtin result matcher; only its Min/Max rows match wanted signatures.
- Original helper SHA c76afedb1ed13593d87000d7b20572b1f9a00c02fb07bc3fac890ad275a68985
  and original predictions are preserved; three corrections are explicit.
- Native4265 terminal0:19 actual compiled faults/full5+5 restored. Corrected verify0:
  all executed targets' predicted failed sets/every failed body current macro;
  the other16 predictions remain exact. Cargo stops after the first failed target
  in14 cases; no skipped target is credited with execution.
- Artifact19889 terminal0: both Cargo-selected full5 executables pass0, operator
  585ab408 and builtin4034e067; all74 inputs/63 production Rust/public tests exact.
- D195 closes after measured correction and complete proof. Product/runtime/grammar
  unchanged; D192 call repair and D156 remain owned next.
