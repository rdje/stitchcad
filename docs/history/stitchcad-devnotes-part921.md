# Sealed engineering continuity — part921

- **Identity:** 28 lines, 2838 bytes, sha256:0da33cb9081a01b47c47b51e450a007dda2cec0e0d6f09c0e0328ba59c1cdea4
- **Coverage:** whole0746 scope,lesson and changelog from 63f4402721c3d6e73cc0bb7d0f9b7b6782adef9a.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 46918–47925, 1007 bytes, sha256:eba6cb1f9951978cadd9fbb1c3c2b19c38496121d9a6ecb7d71a49d7ac76a1fb
- Whole source interval: DEV_NOTES.md bytes 193–1432, 1239 bytes, sha256:7616c28ee210e9587a7c56c3259b14ad71ea446f64e3199e3ba7ee46e974768d
- Whole source interval: CHANGELOG.md bytes 217–809, 592 bytes, sha256:c2e299d60de2b230774a5564ef686c2c6d76df4c608a6c8f69e3860d033c444c
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.5f.1.c.walk.outputs.manifest.identities.known`
  Status: `done`
  Goal:explicit bounded inspected EntityId exclusions;unlisted is not globally fresh.
  - [x] ISSUE:new identity proposals must avoid definite supplied identity references.
  - [x] ROOT CAUSE (walk exclusions):actual Piece/Notch/profile/source APIs reviewed;
    C/I/N/K then H/R and C+2I+R+14N+9K+3 before ids/allocation;rc=0.
  - [x] FIX:scoped actual-reference set;exhaustive profile accounting,actual parents retained.
  - [x] ADDRESSED (walk exclusions):cargo test --test walk_notch_identity_exclusion_contract
    --test walk_existing_notch_contract:4+4 pass;rc=0;full invocation in receipt.
  - [x] NO REGRESSION (walk exclusions):API3/identity book2,lint/fmt/build/map/render;
    old kernels/tests/fixtures,prior fence/920 spans exact;rc=0.
  - [x] LOCKSTEP:book/live aligned;promotion: declined (existing scoped known-id/nonreuse rule).
  Proof:target/history-proof-0746-walk-id-exclusions/;Commit:STITCHCAD-G1-0746.
## _(2026-10-31 UTC)_ — exclude scoped known walk semantic identity references

0746 checks complete C/I/N/K,then whole H before child lengths,R before creator reads,and checked C+2I+R+14N+9K+3 before id scans/allocation. Set covers supplied copy/Piece/creator/Material refs,original mark creators/six profiles,provided Notch id/owner/creator/six profiles,and operation/graph/selected span.
Exhaustive profile-field patterns couple6-id accounting to canonical records. Repeated fields consume full allowances;set keys may deduplicate. False lookup means scoped unlisted,not global freshness,kind consistency/history/reservation or reuse authority;all actual parents remain borrowed.
Native4+4/API3/identity book2,selected lint/fmt/build/map/render pass0;C2/I2/H0/R3/N1/K2 givesW44/distinct24;hole/fold/Materials H1/R5/W46/distinct26;unrelatedK3 W53/distinct33. Old kernels/tests/fixtures/prior fence and whole0745 spans920 exact0;older31219B notes base retained.
Promotion: declined (existing scoped known-id/nonreuse rule). Explicit host identity/retention plan next;actual current namespace/history/private-reserve/sharing/provider/geometry/all-copy effects/atomic publication/full integration remain owed. G1still6/18/G0 unapproved.

## STITCHCAD-G1-0746 — exclude scoped known walk semantic identity references

- Complete C/I/N/K,H/R and checked C+2I+R+14N+9K+3 bound known-id scans and index allocation.
- Supplied Piece creators/Materials,original request and provided Notch fields stay excluded with actual parents.
- Exhaustive profile accounting;shared values retain whole allowances;unlisted never means globally fresh.
- Native8/API3/identity book2 and focused lint/fmt/build/map/render/history920 checks pass.
- Host identity/retention proposals and actual global/semantic/effect/geometry authority remain owned.

