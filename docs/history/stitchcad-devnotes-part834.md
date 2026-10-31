# Sealed engineering continuity — part834

- **Identity:** 37 lines, 3582 bytes, sha256:3e1bfcddd7a5f35635607187bdf6961d86f8b1c0157a903ba664bb50b5a0cff0
- **Coverage:** whole0671 task scope, lesson and changelog from a81da54cd715245f4cb32811c49876f88710d50c.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 19125–21151, 2026 bytes, sha256:fa8e0270727e30af41b5503464db70e6ea45ffa7865f16fefc995b27ef09386a
- Whole source interval: DEV_NOTES.md bytes 193–1301, 1108 bytes, sha256:8127651b58584052b463ff5dcbe521829dd5b9d42043212ac8da919e763caf61
- Whole source interval: CHANGELOG.md bytes 207–655, 448 bytes, sha256:c8cad137b86f4d687549eb915bad1e35e776a8deabcb5b7b7ccbb53897c6daee
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.5e.4.1`
  Status: `done`
  Goal: reusable fresh scalar recipe replay from the original prepared plan,without prior values,
  read slots or cached bindings. Preserve proof/input identity,explicit limits and tangent policy.
  Scope:actual scalar plan/worker/error API and focused replay tests;retire whole0670 live proof
  to immutable history for task/journal headroom. Paired source replay follows in .2;storage .7,
  geometry .5f.3 and full integration/pre-push verification remain separately owned.
  Acceptance:statement-one execution on every attempt;changed context earns only current reads;
  late failure exposes no accepted prefix and cannot alter an earlier accepted result.
  - [x] ISSUE:whole scalar replay requires rebuilding consumed admission/preparation by hand.
  - [x] ROOT CAUSE (WHY + WHERE):Python Git-source assertion at23ac0e43 confirms evaluate(self)
    consumes plan and no execution/replay module existed,rc=0;actual worker already atomic.
  - [x] FIX:borrow plan,re-admit whole proof,prepare original inputs/policy and evaluate fresh;
    typed Admission/Preparation/Execution errors preserve actual causes without accepted prefix.
  - [x] ADDRESSED:local_environment.py -- cargo test --offline --locked -p sc-core --test
    formula_scalar_recipe_replay_contract →3 pass,rc=0;independent expected stored values/current
    reads,late failure with held result,tangent zero/success and empty recipe. No byte-format claim.
  - [x] NO REGRESSION:formula_scalar_recipe_execution_contract →9 pass including independent
    reference regeneration,rc=0;targeted replay rustdoc1,clippy/fmt/book/history833 three Git spans
    and diff checks pass,rc=0. Initial test indexing lint corrected with first();full CI deferred.
  - [x] LOCKSTEP:book/live/frontier/notes sync;no G1/G0 approval change. Promotion:declined(local
    API uses existing private-result/fresh-runtime doctrine). Paired replay .2 next;full .4 pending.
  Verification:focused native12/rustdoc1;Commit:STITCHCAD-G1-0671.
## _(2026-10-31 UTC)_ — replay scalar recipes from fresh state

0671 adds plan.replay(context):re-admit original whole proof/limits,prepare original inputs/tangent policy,then execute new private runtime. The plan remains borrowable,including one retained by an earlier accepted result;no prior read slot,binding or cache is copied.
Native replay3/existing recipe9 pass,including independent reference regeneration. Fractional binding stores once;changed physical context changes branch/read trace;late assertion refuses without altering held results. Explicit tangent zero/success and empty zero-allowance recipes pass.
Targeted rustdoc1/clippy/fmt/book/diff and history833 three exact Git spans pass. Initial indexing lint in the new fixture corrected with first(). No persistence bytes,paired/geometry/storage replay or full-suite acceptance claimed;those owners remain live.
LIVE states/G1 6/18/G0 unapproved unchanged. Promotion:declined(existing private-result/fresh-runtime doctrine). Whole0670 retained833. Next .5e.4.2 paired source replay;D156 nonblocking audits remain deferred. PNT continues.

## STITCHCAD-G1-0671 — replay scalar recipes from fresh state

- Borrow prepared plan,re-admit original proof/limits/inputs/tangent policy and evaluate fresh.
  Typed stage errors expose no accepted prefix;late failure leaves earlier held results intact.
- Replay3/existing recipe9/rustdoc1/targeted lint/fmt/book/history833/diff pass. Paired,storage,
  geometry/full replay identity remain owned;G1 6/18/G0 unapproved unchanged. PNT continues.

