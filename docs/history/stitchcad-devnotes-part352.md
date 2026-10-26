# Sealed archive — completed approximation-source metadata scope

Immutable historical segment, sealed by leaf `G1-SLICE.5e.3.2` on `2026-10-26` (UTC).

- **Sealed identity:** 19 lines, 1750 bytes, sha256:2b2bcd58f484ce3bcdd6ac1df1c78713ec0e9117bbc34fec858ca4df690b8206
- **Coverage:** whole committed source interval from 1542f44de74419460292ce5fe04eb55e475d66ad.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 35007–36757, 1750 bytes, sha256:2b2bcd58f484ce3bcdd6ac1df1c78713ec0e9117bbc34fec858ca4df690b8206
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5e.3.1`
  Status: `done`
  Goal:0268 immutable executed approximation-source sets and symbolic T1 admission guard.
  Paths: crates/sc-core/src/recipe/provenance.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/tests/formula_approximation_sources_contract.rs
  Keep all11 normative rounding/irrational call sources, even exact-valued calls; exact operations
  add none. Deterministic deduplicated union, all five symbolic classes; no value/provider execution.
  Retain349/350 whole0267/closed .o.h scope,351 oldest complete journal group,240 whole0267 ledger.
  Book/live sync; full evaluator/propagation and diagnostic comparison context remain .5e.2/.3.
- [x] REPRODUCE:0268 b01a2e4 has value/read APIs without product call-source sets; source read rc=0.
- [x] ROOT CAUSE:0268 provenance follows executed calls/dependencies, not accidental integer
  equality or numeric epsilon. All22 built-ins/5 classes in focused native4/0, rc=0.
- [x] FIX: sealed immutable source union/lexical inspection and symbolic T1 refusal with actual sources.
- [x] ADDRESSED:0268 python3 -I -B scripts/local_environment.py -- cargo test --offline -p sc-core
  --test formula_approximation_sources_contract:4 pass/0 fail, rc=0; fixture E0599/101 repaired.
- [x] NO REGRESSION:0268 unions/old copies/class matrix and explicit selected-source examples pass;
  cargo clippy --offline -p sc-core --test formula_approximation_sources_contract -- -D warnings, rc=0.
  Full .5e.2/.3 execution/propagation and broad .5g/pre-push remain owned, not claimed here.
- [x] LOCKSTEP:349–351/240 whole Git/hash/public reads and book render rc=0; live/frontier align.
  Promotion declined: existing formula4.2/D121; no invented source, provider or comparison context.
