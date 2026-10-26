# Sealed archive — whole0299 paired split intent

Immutable historical segment, sealed by leaf `G1-SLICE.6.2.2.c.1` on `2026-10-26` (UTC).

- **Sealed identity:** 19 lines, 1985 bytes, sha256:34b33d6c809840e9043e213dd7eac844dbdefb5dcae814565dfca85a7341c412
- **Coverage:** whole committed source interval from 198bbb153bb382c1ac393e5d023d6a32e3f16ec3.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 40699–42684, 1985 bytes, sha256:34b33d6c809840e9043e213dd7eac844dbdefb5dcae814565dfca85a7341c412
- **Retrieval:** bytes below the first rule and blank line; original terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.6.2.2.b.2.b`
  Status: `done`
  Goal:0299 D226 paired correspondence intent targets canonical SeamSpan with explicit persistent side-A/side-B cuts.
  Paths: crates/sc-core/src/command/construction.rs;crates/sc-core/src/command/payload.rs;crates/sc-core/src/command.rs;
  crates/sc-core/tests/command_construction_payload_contract.rs; command book/decision/live docs.
  No new result identity, copied ease/stops or guessed side mapping. Execution validates actual canonical
  copy/range/direction/ease/stops and allocates lineage-preserving outputs atomically under later .6/G2/G3.
  Own424 whole0298 task,425 oldest0274 lesson and271 ledger from5e15254; preserve whole Git/hash/public bytes.
  Acceptance0299:
  - [x] ROOT CAUSE0299: rg actual structs -> construction.rs:8, sewing.rs:184, anchor.rs:7, rc=0; span owns copies/ease/stops, anchors retain exact source frames.
  - [x] ADDRESSED0299: cargo test -p sc-core --test command_construction_payload_contract --test command_payload_contract ->4+3 passed/0 failed, rc=0; two named cuts/unequal parameters/swaps/metadata preserved.
  - [x] NO REGRESSION0299: selected strict lib/two-test Clippy rc=0; fmt rc=0; census17 commands/5 classes/5 levels/0 failures and book rc=0.
  - [x] LOCKSTEP0299:424/425/271 whole5e15254 Git bytes/hash/public reader exact rc=0; book/decision/live aligned. No handler/registry/geometric outcome.
  Promotion: existing command decision extended; no new policy. Full integration/platform verification deferred to .6.9/pre-push.
  Commit: STITCHCAD-G1-0299.
  Source census: rg -n 'SplitSeamSpan|split.*(seam|span|correspondence)' ROADMAP.md docs/book/src docs/decisions crates --glob '*.md' --glob '*.rs'.
  The command row names a span split; SeamSpanDefinition has paired a/b/ease/stops, while
  IdentityLedger::split edits one EdgeRef at Param. Director2026-10-26 chose paired sewing split;
  exact cut/reference/ease/identity validation remains the executing operation's obligation.
