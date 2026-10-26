# Sealed archive — whole0293 mutation-context scope

Immutable historical segment, sealed by leaf `G1-SLICE.6.2.2.b.1` on `2026-10-26` (UTC).

- **Sealed identity:** 16 lines, 1488 bytes, sha256:7f064bdda98945c2f10dc1c4868e51f6bd6106ce3f8fdf160f50131a47dec247
- **Coverage:** whole committed source interval from 4e986498404e8464c5361eeebcf37a3281f0cc06.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 39163–40651, 1488 bytes, sha256:7f064bdda98945c2f10dc1c4868e51f6bd6106ce3f8fdf160f50131a47dec247
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.6.2.2.a`
  Status: `done`
  Goal:0293 typed mutation context with actor/principal claims, exact unbounded revision, retry/group identities.
  Paths: crates/sc-core/src/command.rs;crates/sc-core/src/command/context.rs;crates/sc-core/tests/command_context_contract.rs
  Additional paths: crates/sc-core/src/lib.rs;crates/sc-core/Cargo.toml.
  Claims confer no authority/currentness/registry proof; no payload/handler or wire-format claim.
  Own409 whole0292 task,410 oldest0268 lesson,265 whole0292 ledger before growth.
  Focused full-width counter/context checks, selected lint/fmt/book; .6.3/.6.5/.6.6 validate execution.
  Acceptance0293:
  - [x] REPRODUCE: catalog existed without typed actor/revision/retry/group mutation metadata.
  - [x] ROOT CAUSE0293: cargo test -p sc-core --test command_context_contract:2/0, rc=0; exact request metadata was the missing boundary.
  - [x] FIX: immutable typed context, distinct agent/principal, mandatory revision/key, private exact counter successors; no trust or execution implied.
  - [x] ADDRESSED0293: same native2/0 exact identities/groups and beyond-u128 counters pass, rc=0.
  - [x] NO REGRESSION0293: selected lib/test strict Clippy/fmt and command census17/5/5/0 failures pass, rc=0; broad integration/pre-push.
  - [x] LOCKSTEP:409/410/265 exact Git/hash/public bytes and book build rc=0; G1still5/18, actor claims remain untrusted.
  Promotion declined: existing command metadata contract. Commit: STITCHCAD-G1-0293.
