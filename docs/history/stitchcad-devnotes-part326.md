# Sealed archive — completed scalar binding conversion scope and proof

Immutable historical segment, sealed by leaf `G1-SLICE.5d.1.1` on `2026-10-25` (UTC).

- **Sealed identity:** 22 lines, 1889 bytes, sha256:c31145abcb86e1ecfbdc61df109a47e3dce0c0419909ee1015d963f9dff5682c
- **Coverage:** whole committed source interval from 597e45234c8fe47e2621aba483c5642d61e37a7b.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 25753–27642, 1889 bytes, sha256:c31145abcb86e1ecfbdc61df109a47e3dce0c0419909ee1015d963f9dff5682c
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5c.3.2`
  Status: `done`
  Goal: one signed i64 binding conversion for numeric values plus typed Boolean/name context.
  Own0253: immutable stored numeric/Boolean values, borrowed actual binding-name errors and focused
  native target; full i64 endpoints, unsigned-magnitude half-away and honest rounded-value witnesses.
  Paths: crates/sc-core/src/recipe/bound.rs;crates/sc-core/src/recipe/mod.rs;crates/sc-core/src/recipe/rational.rs;crates/sc-core/src/lib.rs;crates/sc-core/Cargo.toml;crates/sc-core/tests/formula_binding_value_contract.rs
  Conversion returns stored integer for later reads; no namespace mutation/rebinding/provider claim.
  Defect owned/P2/fix now: crate lib.rs/Cargo metadata say whole checking is future; lib claims WASM
  currency; reproduce by reading its header against completed .5b proofs/new native-only features.
  Impact: stale API docs misstate available behavior/evidence. Update lib/recipe module summaries.
  One affected binding native target; broad/platform .5c.4/pre-push. Retain complete scopes/ledger
  324/325/225 under fixed caps; book/API map/live sync and commit before further features.


- [x] REPRODUCE / ISSUE:a9d86c4 lacks bound storage; crate overview drift observed, rc=0.
- [x] ROOT CAUSE: signed storage follows unsigned magnitude rounding; cargo test --offline -p
  sc-core --test formula_binding_value_contract:3 passed/0 failed, rc=0.
- [x] FIX: private integer/Boolean storage, actual name/kind/bounds/full rounded witness; overview repaired.
- [x] ADDRESSED: same native target proves endpoints/tie neighbors/full-width errors,3/0, rc=0.
- [x] NO REGRESSION: same target retains numeric/input/raw-turn/later-read integration,3/0, rc=0.
- [x] LOCKSTEP:324/325/225 source/public bytes exact, rc=0; book/API/live synced. Broad/platform .5c.4.
  promotion: declined (established binding/numeric/focused-check policy).
