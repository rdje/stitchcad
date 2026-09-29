# Product work takes the frontier; spine work only when it blocks or a defect is live

- **Type:** `decision`
- **Date:** `2026-09-29` (absolute)
- **Status:** `active`
- **Owner / source:** director's question during the first working session, recorded by leaf
  `PLANNING.4`; established by a commit census, not by impression (defect D24 in
  `docs/tasks/PLANNING.md`)

answers: "what do I work on next?" · "why did the session produce no product work?" · "when is spine or governance work allowed?" · "how do I notice the frontier has drifted off the product?" · "is fixing doctrine defects a legitimate use of a slice?"

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
