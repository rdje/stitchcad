# Sealed engineering continuity — part582

- **Identity:** 37 lines, 3152 bytes, sha256:d469a1424ac9ce18b8c3e0afc10fc36956a783226c3598238f01d02b960acfa9
- **Coverage:** whole0430 group-authority task/lesson/ledger from 51359e53a09d3bbaf771f45361a11bf94039020e.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 42972–44965, 1993 bytes, sha256:b02626dd2903384959441ff4ab8e77d7635c1977a382612b287927e1f77b6366
- Whole source interval: DEV_NOTES.md bytes 193–851, 658 bytes, sha256:819d9ad92b6478035b5a5b7702e21157997ce5514be59ab6d74f0d884c9e7ba7
- Whole source interval: CHANGELOG.md bytes 207–708, 501 bytes, sha256:7803677fcb57dc946bda680595708012922ab527d0cf09bebc655cdcfb56032a
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.4.2.2.1`
  Status: `done`
  Goal:0430 fresh whole-group single-complete-grant decision, original source/member refusal context;
  descriptive only, no actual model lookup or group execution.
  - [x] REPRODUCE / ISSUE: cargo test -p sc-core --test command_authorization_contract:13 passed rc=0;
    group case separately authorizes both members then proves their two partial grants cannot cover group.
  - [x] ROOT CAUSE (WHY + WHERE): authorization13 rc=0; per-request selection cannot establish one grant
    across all members. Shared original actor/status/permission/scope predicates preserve exact semantics.
  - [x] FIX: cargo clippy -p sc-core --test command_authorization_contract -- -D warnings:finished rc=0;
    one trusted observation, full identity/duplicate-grant checks, source-ordered per-grant member checks.
  - [x] ADDRESSED: cargo test -p sc-core --test command_authorization_contract:13 passed rc=0;
    original8/group5, later whole grant over earlier partial, no unions, inspect/commit distinction/freshness.
  - [x] NO REGRESSION: original8 authorization cases pass within13 rc=0; cargo test -p sc-core --doc
    command::group_authorization:1 passed rc=0; strict selected lint/fmt rc=0. No model/allocator effects.
  - [x] LOCKSTEP: whole0428/0429 and D283/D284 scope/proofs retained581; five exact Git intervals
    verified rc=0; retention1035/global65 with actual active44 fits unchanged caps rc=0. Book/pointer/decision aligned.
  Native publication80 chapters/159 scoped API rows/1464 source links/2444 rendered links rc=0;
  maintained-reference81 files/17587 lines/1227649 bytes rc=0; independent rg raw44/diff check rc=0.
  Cargo commands use scripts/local_environment.py. Original core identity failures remain in standard
  source chain; NoMatchingGrant retains each grant's actual first failing index/command/refusal. Decisions
  borrow the original group, confer no handler availability/capability, and must be refreshed before effects.
## _(2026-10-27 UTC)_ — one complete grant for an entire group

0430 .6.5.4.2.2.1 authenticates one fresh trusted snapshot and selects one grant covering every
original mutation. Shared actor/status/permission/scope checks preserve single-command order.
Original per-grant first failing index/command/refusal survives; partial grants cannot union coverage.
Returned decision borrows original group and is no execution capability. Native authorization13
includes original8/group5; lifetime1/strict selected lint/fmt pass. Promotion: command-layer decision.
D284 remains fixed: archive counts are aggregate; actual per-generation budgets govern retirement.

## STITCHCAD-G1-0430 — authorize whole groups with one complete grant

- Fresh trusted snapshot and one grant must cover all original mutation members; partial grants cannot union.
- Preserve original source identity/duplicate-grant failures and per-grant first failing member/refusal.
- Native authorization13 (original8/group5), lifetime1/lint/fmt; decisions remain descriptive, no execution.
- Current archive record581 fits actual active-group bounds; D284 aggregate-scope correction retained.

