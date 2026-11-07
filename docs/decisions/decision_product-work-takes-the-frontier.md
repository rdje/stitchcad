# Product work takes the frontier; spine work only when it blocks or a defect is live

- **Type:** `decision`
- **Date:** `2026-09-29` (absolute)
- **Status:** `active`
- **Owner / source:** director's question during the first working session, recorded by leaf
  `PLANNING.4`; established by a commit census, not by impression (defect D24 in
  `docs/tasks/PLANNING.md`)

answers: "what do I work on next?" · "why did the session produce no product work?" · "when is spine or governance work allowed?" · "how do I notice the frontier has drifted off the product?" · "is fixing doctrine defects a legitimate use of a slice?" · "how much verification follows each feature?"

## The fact / decision

1. **The frontier belongs to the product.** The next slice is a specification chapter, a crate, a
   conformance fixture — whatever the active gate's tree names next.
2. **Spine, governance and hygiene work is taken only when one of these holds:**
   - it **blocks** the product slice about to be taken (example: the acceptance gate could not
     attribute evidence to the right leaf, so every later code commit would have been judged on
     somebody else's evidence);
   - a defect is **live** — it can destroy or corrupt work now (example: the scaffold updater could
     silently overwrite the task-tree index; a build artifact dirtied the tree after every documented
     command);
   - the director **asks** for it.
   Curiosity, tidiness and "one more defect found while looking" are not triggers.
3. **A found defect is always logged and owned** (directive §15 — that does not weaken), but logging is
   not scheduling. Priority is decided by the test above, and a non-blocking defect waits its turn
   behind product work.
4. **The symptom to watch for is a run of commits none of which touches the product.** The census is
   one command:

   ```bash
   git log --oneline | grep -cE 'leaf (SPINE|PLANNING|BOOTSTRAP)'   # governance slices
   git log --oneline | grep -cE 'leaf (G[0-7]|V[12])'               # product slices
   ```

   When the first number grows and the second does not, stop and re-sequence — do not finish the
   governance lane first, because governance lanes do not end.

## Why

Measured on this repository's first session: of the first 17 commits, **15** were spine or planning
slices and **0** were product slices; `ls docs/book/src/spec/` held `index.md` alone and
`git ls-files 'crates/*'` held the bedrock starter crate. Meanwhile five of the ten roadmap lanes had
no task-tree at all — the tracking was incomplete *and* the product was untouched.

The work was not worthless: it closed a data-loss path in the scaffold updater, made the acceptance
gate attribute evidence to the leaf that earned it, adopted three policies, bounded the live documents
and sealed an archive. Every one of those was a real defect with real evidence. That is precisely why
the pattern is dangerous — **governance work is self-rewarding, infinitely discoverable, and always
defensible in isolation.** Each slice passed its own gate and closed its own defect, and the
repository's reason to exist still had not started.

Nothing in the spine asks whether the frontier is still on the product: the gates judge the slice in
front of them, the task-tree index reports what is being worked on without ranking it, and a defect
census grows monotonically. So the check has to be a decision a future session reads, and a census it
can run.

## How to apply

- At session start, after reading the resume pointer, run the two census commands above. If product
  slices are not growing, the frontier moves to the active gate's tree before anything else.
- When a spine defect appears mid-product-work: log it in the defect census with an owner, then ask
  only "does it block the slice I am about to take, or can it destroy work now?" If neither, schedule
  it behind the product leaves and continue.
- Keep governance slices **small and terminal**. A governance slice that reveals three more governance
  slices is a lane, not a slice; name it, defer it, and return to the product.
- This record does not lower the bar on the work that *is* taken: a spine slice still needs its
  evidence, its probe and its lockstep. It changes the order, not the quality.

Related: [[decision_acceptance-evidence-per-leaf]] · [[decision_adopted-external-policy-references]] ·
`docs/tasks/PLANNING.md` (defect D24) · `MEMORY.md` (the pointer that carries the frontier).

## Director ruling — 2026-10-25: feature delivery and focused checks

The director explicitly asks to reduce verification overhead so implementation reaches 100% feature
coverage. After a feature addition, run one small check of its affected behavior. Broad regression,
mutation campaigns, full publication probes and repeated platform/CI sweeps wait for a later
integration milestone and the required pre-push gate. Escalate checking when a concrete failure or
integration risk warrants it; do not make exhaustive calibration the default cost of each slice.

This changes verification scheduling, not implementation quality. Keep contracts, readable code,
task ownership, book synchronization, commits and the automatic doctrine hooks. Record deferred
verification with an owner and milestone. A focused result never claims that unrun suites passed.
The 400-commit push cadence and full pre-push gate remain unchanged.

Finish the current dirty unit before switching. Then take the next implementable product feature.
Remaining producer-audit and tooling work stays owned and pending, but does not block features
unless it directly blocks the selected feature or can destroy/corrupt current work. D156 and D223
remain open; deferral does not constitute repair or runtime signoff.

The old commit-prefix census above cannot distinguish tooling inside a G1 task from product behavior.
Report feature progress by implemented capabilities and roadmap acceptance, with maintenance
progress stated separately. A G1-labelled maintenance commit does not advance feature completeness.

Authority: director message in the 2026-10-25 session; adoption owned by
`G1-SLICE.5b.4c.h2.b.p.o.a25.2`, with deferred fixture calibration owned by .f.fixture-verification.

## Director ruling — 2026-11-06: input checks on hold

The director's words:

> we can't guarantee 100% input check. We have to do as much check as we can, but we can't spend all of
> time checking input. More checks on input shall be added based on needs. We can go back and add more
> input checks after receiving feedback from downstream consumers or users or the when STITCHCAD is
> feature complete. Until then please put an hold on input checks and go all in on product work,
> feature implementation.

How this applies until feature completeness or real feedback reopens it:

- New admission, quota, source-binding or refusal layers are not taken as their own slices. The
  `G1-SLICE.5f.2` dependency, check and verify leaves are `blocked` on this ruling; the work already
  done stays in the tree, compiled and tested, and nothing is removed.
- A new input check is added when a feature cannot work correctly without it, or when feedback from a
  downstream consumer or user names it. It is part of that feature's slice, not a campaign.
- A feature refuses what it does not support yet with a plain error. That is feature behavior, and it
  needs only what that feature's one focused check requires.
- A feature is the roadmap's next requirement, built by its existing task-tree owner to the contract
  the book gives, with the book's status and examples changing in the same commit. It opens no parallel
  plan. The order toward the G1 exit is in `docs/tasks/G1-SLICE-runtime-proof.md`.

The refusal and new-check bullets above are the engineer's characterization. The director adopted them
and pointed feature work back at its sources:

> Let's use your characterisation of what a feature work entails. But don't we have a roadmap and their
> task-tree counterpart? and the mdbook ?

Unchanged: the doctrine hooks, task tree, book synchronization, one focused check per feature, and the
400-commit full gate. Existing checks keep passing; this holds new input-check work, not the gates.

## Director ruling — 2026-11-06: stay on the roadmap

The director's words, verbatim:

> Once we settle on the roadmap yoy'll then need to always make sure that, that roadmap, the codebase and
> the mdbook always stay in sync, in lockstep!!

> Look, you have the original roadmap. Not sure why you drift away and worked on 650 commits for almost
> nothing, right ? What can I say. Do whatever you can to come back on track.
>
> You have to handle this yourself with a sota, signoff and production-grade mindset "- Measuring the
> printed pattern on paper for G2.
> - Getting evaluation seats on commercial CAD. G0 warns these take months, and I'll check where that
> stands after the probes."
>
> Before we reach G7, we need to prove G0-6 are worth the review. We need to be really close to a finished
> product.
>
> From now on, please stay on track. If you need more enforcement, build them but we definitely need to
> stick to the plan, the roadmap.
>
> Can you do this ?

> I can't have access to "Getting evaluation seats on commercial CAD".

> So we will have to implement STITCAD without that! that's just another constraint

> Regarding "What this costs: without a seat, nothing can show that commercial CAD systems open our files,
> so StitchCAD can't claim that. Factories use those systems, so this will be the biggest gap left at G7,
> and I won't play it down.", you drifted from the roadmap for 650 commits, so right now commercial CAD is
> not out biggest concern. We are not their yet. First let's implement all the things described in out
> roadmap first, we will worry about a commercial CAD when the project will be feature complete.

> Please stay focus until we complete G6. G0 -> G1 -> G2 -> G3 -> G4 -> G5 -> G6. Not sure things will be
> executed in this order though.

How this applies:

- The plan is `ROADMAP.md`, its task-tree counterpart and the book, at v0.4. No parallel plan exists.
- Gates close in order. Every commit advances a named clause of the earliest open gate that needs it. Work
  on a later gate starts early only when an earlier gate's clause depends on it, and the commit says so.
  G7 and the V-tracks wait.
- No commercial CAD seat is available. This is a fixed constraint. Commercial CAD is parked until StitchCAD
  is feature complete; nothing is planned around it. G0's evaluation-seat clause stays open under the
  2026-09-30 ruling, and G6's import-filter clause stays not met.
- G2's printed-square clause stays not met until a person measures one. The exporter and its numeric and
  independent-renderer checks come first, and one scan or photo is requested at the end of G2.
- Enforcement exists to hold this. Whether a commit really advances the clause it names stays the author's
  judgment, so the measure is exit clauses met, not commits or leaves.

The bullets are the engineer's reading, given to the director on 2026-11-06.


## Director ruling — 2026-11-07: the engineer decides procurement, fetches and pushes

> Please decide yourself for (2)/(3)/(4), they better be sota, signoff and production-grade. Not sure what I have to do for (1) and (5)

> Try to do (1) and (5) yourself. Is printing really necessary and is (5) really needed

(1) printed square, (2) Aldrich's book for `G3-GRADING.16`, (3) four refused fetches, (4) a push before 400, (5) a
real-plotter fixture and a sewing expert. Decided: (2) nothing bought; blocks from FreeSewing (MIT, read 2026-11-07),
a community system, after G6. (3) nothing fetched here; Z3 and ezdxf (`G2-2D.4.third-party-reader`) on the hosted
runner; the PDF library evaluation there before `G5-SHELLS.12`; no eframe, since `G5-SHELLS.1` is the native shell.
(4) push now after the full gate; also at each gate exit or when a hosted-only proof is owed; 400 stays the bound.
(1) and (5) need a person or machine this project lacks: `not met`, director authority, not holding the frontier.
