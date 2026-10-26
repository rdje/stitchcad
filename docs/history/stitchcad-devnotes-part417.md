# Sealed archive — whole0295 construction and ruling scope

Immutable historical segment, sealed by leaf `G1-SLICE.6.2.2.b.2.a.receipt` on `2026-10-26` (UTC).

- **Sealed identity:** 20 lines, 2018 bytes, sha256:3675ea84c83e9e2b9c7d1f2cdb0709515be23b5871f244e5859cd44843fb37a4
- **Coverage:** whole committed source interval from 1184214f401174508e07596585a5cbe20ffb5d2c.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 40003–42021, 2018 bytes, sha256:3675ea84c83e9e2b9c7d1f2cdb0709515be23b5871f244e5859cd44843fb37a4
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.6.2.2.b.2.a`
  Status: `done`
  Goal:0295 CloseDart canonical target and AddNotch creation fields, with bus-supplied new identity.
  Paths: crates/sc-core/src/command/payload.rs;crates/sc-core/src/command/construction.rs;crates/sc-core/src/command.rs
  Tests: crates/sc-core/tests/command_construction_payload_contract.rs.
  Reuse EdgeAnchor/NotchProfileBindings and Dart's referenced closure; no copied intake/profile defaults.
  Own414 whole0294 task,415 oldest0270 lesson,267 whole0294 ledger,416 whole completed execution-prerequisite summaries.
  D226 paired-cut split schema remains separate .b; this slice completes dart/notch. Focused native/lint/book.
  Director2026-10-26 confirms D226 paired correspondence with explicit cuts on both sides;
  D70 requires at least one authored axis per complete SizeSet. Record rulings; finish this slice,
  then resume .4c.2 before split payload work. No axes code in the current slice.
  Own whole prior D70 defect record137 and current book/decision ruling sync before edits.
  Acceptance0295:
  - [x] REPRODUCE: dart/notch intents were absent; D70/D226 needed actual director rulings.
  - [x] ROOT CAUSE0295: cargo test -p sc-core --test command_construction_payload_contract --test command_payload_contract:2+3/0, rc=0; canonical target/creation fields bind real command variants.
  - [x] FIX: canonical dart target and exact notch anchor/bindings; bus supplies new id. Record both confirmed rulings without axes/split implementation.
  - [x] ADDRESSED0295: same tests preserve target, all bindings, explicit fresh identities and prior payloads, rc=0.
  - [x] NO REGRESSION0295: selected lib/two-test strict Clippy/fmt and command census17/5/5/0 pass, rc=0; broad integration/pre-push.
  - [x] LOCKSTEP:414/415/416/267/137 whole Git/hash/public exact; book/ruling field/section check rc=0. D70/D226 resolved, G1still5/18; .4c.2 next.
  Promotion declined: existing canonical/command contracts plus explicit director rulings. Commit: STITCHCAD-G1-0295.
