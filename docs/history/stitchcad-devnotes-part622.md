# Sealed engineering continuity — part622

- **Identity:** 38 lines, 3570 bytes, sha256:7d400a1f577485e5dd15c8c69090f82d8bca9cb9cbcf2e0f9b19677eab2c7fd4
- **Coverage:** whole0468 retained-policy configuration scope/lesson/ledger from 4719e8098c773843e06d23d471879bd09c895a4b.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 52188–54268, 2080 bytes, sha256:1ded07285160618aecc9b431054714e2d9883d05505cfb748613c60e5b1ede43
- Whole source interval: DEV_NOTES.md bytes 193–1034, 841 bytes, sha256:762e05d28a60694c2e9b2efe37313f12d3f879a351e2a7793f89cf2e3cac7aac
- Whole source interval: CHANGELOG.md bytes 207–856, 649 bytes, sha256:5c5c36d6a5dc5f8e95d4abbf7729beafee153a0cd1e3e8e425afa0b2e9038f6d
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.8.2.2.2.2.4.3.3.1`
  Status: `done`
  Goal: install explicit immutable retained-content policy, without traversal/callbacks/defaults; require
  all three policies for costly metadata entry points and preserve policy-free authorized Piece reads.
  - [x] REPRODUCE / ISSUE: python3 -I -B native setup:explicit absent constructor slots, both admission
    helpers require retained presence; original model/operation setup precedence preserved rc=0.
  - [x] ROOT CAUSE (WHY + WHERE): transition_receipts.py --record docs/history/stitchcad-devnotes-part621.md:
    3 exact Git spans rc=0; whole0467 retained. Retained setup must not traverse or imply admission.
  - [x] FIX: cargo clippy -p sc-core --lib -- -D warnings and affected sc-measure strict Clippy:both
    rc=0; once-only private setter, noDefault fallback/callback/walk; all3 policies gate availability.
  - [x] ADDRESSED: cargo test -p sc-measure --test design_inputs_contract command_bus::retained_policy:::
    2 passed rc=0, plain/reserve/edit-history constructors and single/group/commit/history refusal;
    same/larger replacement rejected, cheap authorized reads unaffected, source0/0/0+authority0.
  - [x] NO REGRESSION: model setup1/operation setup1/both package fmt pass rc=0. cargo test -p sc-core
    --doc command::bus::CommandBus:11 privacy cases passed rc=0. Spaced line-suffix filter expanded171;
    owned rustdoc/children terminated (101/SIGTERM), corrected space-free type filter11/160 filtered.
    Native group/history/preparation/identity/resource files unchanged; effect runtime byte-exact,
    policy-count rustdoc only. No current/prospective retained-content check is claimed.
  - [x] LOCKSTEP: mdbook build/native publication84 chapters/159 scoped API rows/1519 source links/
    2558 rendered links rc=0; book setup examples/pending boundary and indexed decision promoted.
    history_archive.sh verify-retention:1075 logical/42 working Markdown/60115 decoded lines/
    4548123 decoded bytes/2378935 resident bytes rc=0. Current .3.3.2/prospective .3.4/coupled/full pending.
## _(2026-10-28 UTC)_ — explicit immutable retained-policy configuration

0468 .6.8.2.2.2.2.4.3.3.1 adds a private absent-by-default retained policy and once-only setter.
All costly entry points require operation/model/retained setup; cheap authorized Piece reads need none.
Native setup2 plus model/operation setup cases and strict lint/fmt pass:three constructors, same/larger
replacement refusals, source0/0/0 and authority0 for configuration/missing-policy errors. Test helpers
supply explicit retained maxima; without_model_limits deliberately omits only the model policy.
Current .3.3.2/prospective .3.4 admission remain pending; no retained certificate. Whole0467 retained621; indexed decision promoted.
Doctest filter with spaces unexpectedly selected171; stopped owned tree (101), space-free type filter selects11 privacy checks.
## STITCHCAD-G1-0468 — require explicit immutable retained-policy configuration

- Add an absent-by-default private retained policy and once-only positive-limits setter, without traversal, validation or callbacks.
- Costly metadata entry points require all three explicit policies; authorized Piece inspection remains independent of policy setup.
- Native retained setup2 and existing model/operation setup cases plus strict lint/fmt pass; original error precedence and zero source/authority callbacks verified.
- Whole0467 retained621; book distinguishes configuration from pending current/prospective retained admission and broader acceptance.

