# Sealed archive — completed exact scalar numeric-read scope

Immutable historical segment, sealed by leaf `G1-SLICE.5e.3.1` on `2026-10-26` (UTC).

- **Sealed identity:** 19 lines, 1720 bytes, sha256:46adcf306ad78e4892f0aaec75af3177d65114dd0c62ed7f8f056d70f927e7d0
- **Coverage:** whole committed source interval from b01a2e447f435d8c4e37f51277cc755860274dc7.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 33585–35305, 1720 bytes, sha256:46adcf306ad78e4892f0aaec75af3177d65114dd0c62ed7f8f056d70f927e7d0
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5e.1.6`
  Status: `done`
  Goal:0267 canonical/stored scalar-to-numeric expression bridges, without extra rounding/scaling.
  Paths: crates/sc-core/src/recipe/numeric.rs;crates/sc-core/tests/formula_value_read_bridge_contract.rs
  Own typed TryFrom conversions, raw turns/exact internal units, signed endpoints and explicit
  Boolean nonnumeric refusal. Stored reads use the once-rounded integer, never the old fraction.
  Focused native/selected lint; retain348/239 whole0266 and book/live sync. Source/provenance stays
  with existing read wrappers and later .5e.3. D70 question pending; no axis change.
- [x] REPRODUCE:0267 eed45a4 lacks typed canonical/stored numeric conversions; source read rc=0.
- [x] ROOT CAUSE:0267 input units and stored integer are exact; neither is an old expression
  fraction or Boolean coercion. Focused native kind/fraction/endpoints witnesses pass5/0, rc=0.
- [x] FIX: typed TryFroms retain units/kinds/raw turns and refuse Boolean; source wrapper stays caller-owned.
- [x] ADDRESSED:0267 python3 -I -B scripts/local_environment.py -- cargo test --offline -p sc-core
  --test formula_value_read_bridge_contract:5 pass/0 fail, rc=0; lost prior output recovered by rerun.
- [x] NO REGRESSION:0267 all numeric kind/limit, once-rounded fraction and false/true controls pass;
  cargo clippy --offline -p sc-core --test formula_value_read_bridge_contract -- -D warnings, rc=0.
  Broad .5g/pre-push and source propagation .5e.3 retain their owners. Stale status note fixed here.
- [x] LOCKSTEP:348/239 whole Git/hash/public reads and warning-free book render rc=0; live align.
  Promotion declined: existing numeric/binding contracts; no new source or execution proof claimed.
