# Sealed archive — completed sourced scalar operator and binding scope

Immutable historical segment, sealed by leaf `G1-SLICE.5e.3.3` on `2026-10-26` (UTC).

- **Sealed identity:** 19 lines, 1702 bytes, sha256:7aa3e315242cfc61b16bb801ff669b63bb47b015070fd724765917a9c47c1fa3
- **Coverage:** whole committed source interval from 1e836c155ca107c3339b4720b063af87944e0d77.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 35358–37060, 1702 bytes, sha256:7aa3e315242cfc61b16bb801ff669b63bb47b015070fd724765917a9c47c1fa3
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5e.3.2`
  Status: `done`
  Goal:0269 immutable numeric/Boolean values, operators and bindings retain approximation sources.
  Paths: crates/sc-core/src/recipe/evaluated.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_evaluated_scalar_contract.rs
  Reuse exact numeric unary/binary/comparison and binding/read boundaries; no Boolean coercion.
  Union all evaluated operands, including cancellation/Boolean results, retain once-rounded reads.
  Source attachment is explicit; no AST/name/provider/conditional execution or atomic replay claim.
  Retain352 whole0268 task,353 whole oldest rational lesson,241 ledger; book/live sync.
- [x] REPRODUCE:0269 1542f44 has source sets without scalar operator/binding retention; source read rc=0.
- [x] ROOT CAUSE:0269 exact equality/zero and stored rounding cannot erase executed dependencies;
  payload/source pairs plus operator unions and stored-only reads retain them. Native5/0, rc=0.
- [x] FIX: immutable numeric/Boolean scalar, exact operators, sourced once-rounded binding/read.
- [x] ADDRESSED:0269 python3 -I -B scripts/local_environment.py -- cargo test --offline -p sc-core
  --test formula_evaluated_scalar_contract:5 pass/0 fail, rc=0.
- [x] NO REGRESSION:0269 every operator/cancellation/Boolean/fraction/refusal case passes;
  cargo clippy --offline -p sc-core --test formula_evaluated_scalar_contract -- -D warnings, rc=0.
  Full .5e.2/.3 AST/call/conditional/assertion execution and broad .5g/pre-push remain owned.
- [x] LOCKSTEP:352/353/241 whole Git/hash/public records and book render rc=0; live/frontier align.
  Promotion declined: existing formula4.2/D121; supplied call sources grant no execution proof.
