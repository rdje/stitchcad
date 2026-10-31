# Sealed engineering continuity — part864

- **Identity:** 35 lines, 3669 bytes, sha256:54ef0d36d8fb9d97191d67fac6a35d5fd6c88a26fafdf2bffc09ccd98e66debd
- **Coverage:** whole0694 scope, lesson and changelog from 91ebe554bb799be43da2999db439935f88e0c2e8.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 42418–44248, 1830 bytes, sha256:11d3743f90ff2bddadec43c3e8d1e8dce75af529dd688818764a07026e23c5a9
- Whole source interval: DEV_NOTES.md bytes 193–1529, 1336 bytes, sha256:a0aa5e4d49cdbed8af3e2a26eaf9f2a6ac0c9529e8fbba968216ed971c09e5da
- Whole source interval: CHANGELOG.md bytes 207–710, 503 bytes, sha256:b035933c0e4056761c371215c21e399d00df9b00f7ff947b78c7c366c0efffdf
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.5f.1.c.offset.outputs.tags.declaration`
  Status: `done`
  Goal:native explicit-tag declaration of an already named canonical operation;preflight count,
  unique tags and prior journal operation identity before state changes. No IdGenerator call or
  source retirement;net/allowance meanings remain distinct. Caller issuance/global/bus authority
  stays separate. Exact output-book split/compiler guard before growth;retire whole0693/closed
  namespace checklist;focused checks.
  - [x] ISSUE:canonical sparse output tags cannot be inferred from a contiguous slot count;
    derived allowance outputs need declarations that preserve source meaning.
  - [x] ROOT CAUSE (tagged declaration):actual net/allowance/profile/replay and public journal/
    id APIs inspected;native source/registration preservation and old allocator checks pass,rc=0.
  - [x] FIX:provided canonical operation/tag declarations;count/uniqueness/full-journal id checks
    before mutation. No allocator call or source retirement;LocalTag docs/source authority aligned.
  - [x] ADDRESSED (tagged declaration):local_environment.py -- cargo test --offline --locked -p
    sc-core --test tagged_declaration_contract →3 pass,rc=0;sparse tags,actual journal/source/
    registrations,fresh replay,capacity/duplicate/order and retired/noncreating operation nonreuse.
  - [x] NO REGRESSION (tagged declaration):old allocator3/actual input book8/output book3/selected
    clippy/fmt/build/rendered/diff/archive863 four exact spans pass,rc=0. Exact7985B output suffix
    retained;SUMMARY119. D372 warning remains P2;no global issuance/geometry/role continuity proof.
  - [x] LOCKSTEP:book/native/live/frontier sync;promotion: declined (existing identity/source contracts).
  Verification:native6/book11;Commit:STITCHCAD-G1-0694. Next .tags.binding.
## _(2026-10-31 UTC)_ — declare canonical explicit output tags without retiring sources

0694 adds IdentityLedger::declare_tagged_edges for already identified canonical operations. Complete count/capacity,unique tags and full-journal operation nonreuse precede mutation;retired/noncreating operations still reserve ids. No IdGenerator call occurs;host issuance/global namespace/authority stay separate.
Sparse tags/caller order remain actual Declared journal outputs;existing live sources,registrations and resolutions remain unchanged. This preserves net/derived allowance meaning instead of using destructive offset automatically. Fresh-ledger equality proves replayed metadata,not cross-revision logical-role continuity.
Native declaration3/old allocator3/actual input book8/output book3/selected clippy/fmt/build/rendered/diff/history863 four exact Git spans pass. Exact7985B output suffix moved to drafting-output-contracts.md with its own compiler guard;SUMMARY119. Proof:target/history-proof-0694-output-book/proof.json.
LIVE states/G1 6/18/G0 unapproved unchanged;Product note updated. promotion: declined (existing identity/source contracts). Whole0693/closed namespace checklist retained863. D372 P2 persists;actual declaration binding,continuity,G2 maps and bus publication remain owned. Next .tags.binding;PNT continues.

## STITCHCAD-G1-0694 — declare canonical explicit output tags without retiring sources

- Provided canonical operation ids and sparse tags register exact logical outputs without minting ids.
- Count/capacity/duplicate/prior-operation checks preserve complete state on refusal and retain sources.
- Native6/book11/selected lint/fmt/rendered/diff/history863 exact spans pass;output book split exact.
- Global issuance,declaration binding,role continuity,geometry/publication remain owned;D372 P2 open.

