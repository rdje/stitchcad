# Sealed engineering continuity — part895

- **Identity:** 34 lines, 3090 bytes, sha256:9fd772a2c5a78e87ba36bb6b60325b5cd7ded27fc75c96a5d50f74073a57fafc
- **Coverage:** whole0723 scope,lesson and changelog from 3afa28e66cbd84fd0eae6512562642f7d63e0143.
- Whole source interval: docs/tasks/G1-SLICE-mirror.md bytes 2452–4305, 1853 bytes, sha256:72b39b45a02a1748f12b12023b1c8cfe7887bd8d67cfaf616932ed57d01f3ba2
- Whole source interval: DEV_NOTES.md bytes 193–948, 755 bytes, sha256:3f614dc81b6583f45b5f94fb7dd91dd7281f7b597f0c33b3dda1bea11e6fed0c
- Whole source interval: CHANGELOG.md bytes 217–699, 482 bytes, sha256:7fb607b9bcac67485c7838728e70efa342b3f50bb415aa8bc2463ca8f2b8fef6
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.5f.1.c.mirror.outputs.manifest.identity.piece`
  Status: `done`
  Goal:canonical new Piece identity association with actual parents;local intrinsic/source-kind
  aliases refuse. Source companion references remain uses;no global freshness or host-request id.
  - [x] ISSUE:copy output needs a distinct canonical Piece owner without minting or reusing source ids.
  - [x] ROOT CAUSE (mirror Piece identity):actual DesignStructure/reference/host/identity contracts
    inspected;Piece/operation are separate owners,companion references are uses;rc=0.
  - [x] FIX:private borrowed declaration;operation/source/self-companion then original edge/axis/
    Material aliases. Source companion can name planned result;global/issued/current namespace stays owed.
  - [x] ADDRESSED (mirror Piece identity):local_environment.py -- cargo test --offline --locked
    -p sc-core --test mirror_piece_identity_contract --test mirror_piece_edge_roles_contract:4+3 pass;rc=0.
    Actual parents/id pointers,both axis modes,typed aliases,zero id and companion distinction covered.
  - [x] NO REGRESSION (mirror Piece identity):API16/mirror-book6,selected lint/fmt/build/map/render/
    tree/893–894 Git spans pass;rc=0. Duplicate fixture module corrected by existing-module reuse.
    Census10 lanes/13 trees/19 siblings/0 unowned/orphans/dead links;all14 old mirror ids/goals/proofs
    retained,16 current ids unique. All old map links/paths/owner ids retained;map8189B/cap8192 unchanged.
  - [x] LOCKSTEP:book/live/frontier/new task sibling/history align;closed0722 and0064 retained893–894.
    promotion: declined (existing owned-id/reference/host boundary);no allocation/global freshness proof.
  Verification:native7/API16/book6;proof target/history-proof-0723-mirror-identity/.
  Commit:STITCHCAD-G1-0723;next separately typed point-role policy.
## _(2026-10-31 UTC)_ — declare canonical mirror Piece identity

0723 binds canonical Piece id to actual role/metadata parents;intrinsic and typed edge/axis/Material aliases refuse,while source companion remains a use.
Native4+3/API16/mirror-book6,selected lint/fmt/build/map/render/tree pass0;duplicate test fixture inclusion corrected through existing-module reuse.
Whole14-node mirror branch copied exact before scoped updates;all16 current ids unique. Tree10/13/19/0;old map links/paths/owner ids kept within8189B. Whole0722/0064 spans893–894 exact0.
Promotion: declined (existing owned-id/host boundary). Typed point roles next;global nonreuse/reserve,current Design/recipe,copy/semantic/geometry/full CI remain owed. G1still6/18/G0 unapproved.

## STITCHCAD-G1-0723 — declare canonical mirror Piece identity

- Actual canonical id/parent association rejects intrinsic and typed source-owner aliases without allocation.
- Companion references stay uses;current/historical nonreuse and trusted issuance remain host/bus checks.
- Native7/API16/book6 and focused lint/fmt/build/render/map/tree/source-span checks pass.
- Mirror task sibling preserves every old goal/proof;all old map paths/links/owner ids retained within caps.

