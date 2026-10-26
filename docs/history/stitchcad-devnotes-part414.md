# Sealed archive — whole0294 canonical payload scope

Immutable historical segment, sealed by leaf `G1-SLICE.6.2.2.b.2.a` on `2026-10-26` (UTC).

- **Sealed identity:** 17 lines, 1729 bytes, sha256:43d22eaabbb4728d830ebfde0eb88e17773bb90ed4993e98d0fc02cf59bf020f
- **Coverage:** whole committed source interval from e95fa7af29b543fb96d28e3bd691058011f2cbc5.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 39627–41356, 1729 bytes, sha256:43d22eaabbb4728d830ebfde0eb88e17773bb90ed4993e98d0fc02cf59bf020f
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.6.2.2.b.1`
  Status: `done`
  Goal:0294 ChangeMeasurement/AssignMaterial canonical payloads bound to mutation context without caller command-ID override.
  Paths: crates/sc-core/src/command.rs;crates/sc-core/src/command/payload.rs;crates/sc-core/src/lib.rs;crates/sc-core/Cargo.toml
  Tests: crates/sc-core/tests/command_payload_contract.rs; reuse actual LengthDeclaration/MaterialAssignment contracts.
  Preserve target/declaration/source/state, ordered nonempty unique Piece selection, shared material reason rule.
  Own411 whole0293 task,412 oldest0269 lesson,266 whole0293 ledger before live growth.
  Also own413 whole five completed sourced-adapter summaries before task acceptance growth.
  Focused payload/native/selected lint/fmt/book; actual registry/auth/execution remains .6.3–.6.6.
  Acceptance0294:
  - [x] REPRODUCE: mutation context lacked canonical measurement/material payload binding.
  - [x] ROOT CAUSE0294: cargo test -p sc-core --test command_payload_contract:3/0, rc=0; payloads derive real command IDs and retain original canonical data.
  - [x] FIX: two typed intents, ordered distinct selection and shared reason predicate; canonical source replacements remain allowed, no draft immutability restriction.
  - [x] ADDRESSED0294: same native3/0 all five states/context/first duplicate/material rules pass, rc=0.
  - [x] NO REGRESSION0294: selected lib/test strict Clippy/fmt, command census17/5/5/0 failures pass, rc=0; broad integration/pre-push.
  - [x] LOCKSTEP:411/412/413/266 whole Git/hash/public exact and book build rc=0; G1still5/18.0293 brief cleared late before code, verified0/untracked.
  Promotion declined: existing canonical/command contracts. Commit: STITCHCAD-G1-0294.
