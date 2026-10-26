# Sealed archive — whole0291 command catalog scope

Immutable historical segment, sealed by leaf `G1-SLICE.6.2.1` on `2026-10-26` (UTC).

- **Sealed identity:** 16 lines, 1738 bytes, sha256:1fdb89028e82be9b92dd832ef8f83b95a3e2b673005cb5a6200986bb2839481b
- **Coverage:** whole committed source interval from 666229849f9afd057cbe54d201f294b398ee1bc9.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 38245–39983, 1738 bytes, sha256:1fdb89028e82be9b92dd832ef8f83b95a3e2b673005cb5a6200986bb2839481b
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.6.1`
  Status: `done`
  Goal:0291 typed declared command catalog/class/authority/reversibility/undo metadata, checked against book.
  Paths: crates/sc-core/src/command.rs;crates/sc-core/src/lib.rs;crates/sc-core/Cargo.toml;crates/sc-core/tests/command_catalog_contract.rs
  Own402 whole0290 task,403 oldest0266 lesson,263 whole0290 ledger,404 whole closed population subtree before growth.
  Also own capture_window14.py/.doctrine/history_archive/window14.json/windows.json/docs/history/
  and archive-reader book: fixed64-file reader refused after403; pack only committed b49128e sources.
  Metadata only; no handler/authorization/transaction availability claim. Focused native/lint/book.
  Acceptance0291:
  - [x] REPRODUCE: normative command table existed without native typed discovery metadata.
  - [x] ROOT CAUSE0291: local_environment.py -- cargo test -p sc-core --test command_catalog_contract:2/0, rc=0; actual book17-row comparison closes catalog binding.
  - [x] FIX: immutable IDs/class/permission/reversibility/undo metadata and exact lookup; no handlers or grants implied.
  - [x] ADDRESSED0291: same native2/0 plus command-layer census17/5/5/0 failures, rc=0; strict names and release distinction pass.
  - [x] NO REGRESSION0291: selected lib/test strict Clippy/fmt pass, rc=0; capture_window14.py source/materialize/public50 exact, prior13 unchanged, rc=0. No archive cap/reader changes; broad integration/pre-push.
  - [x] LOCKSTEP:402/403/404/263 whole Git/hash/public exact, rc=0; book build rc=0. Fixed64 limit triggered at403, resolved by packing committed50. G1still5/18; README/live align.
  Promotion declined: implementation of existing command contract; no new policy. Commit: STITCHCAD-G1-0291.
