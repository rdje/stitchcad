# Sealed archive — D114 and D115 handoff census

Immutable historical segment, sealed by leaf `SPINE.23` on `2026-10-02`.

- **Sealed identity:** 28 lines, 2567 bytes, `sha256:263e4087f2da67367352bfb89f7efdb60ac0223e979b8d699874008ce6d468d2`
- **Coverage:** D114 and D115; original reports retained unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D114** — handoff census reports success when restricted process visibility fails.
  - Reproduce: a controlled Python process PID46805 held target/window3-handoff-visibility.txt
    open and remained live; restricted check_no_background_jobs.sh printed handoff: OK, exit0.
    Direct /bin/ps raises PermissionError (Operation not permitted); CTRL-C afterwards terminated
    that same still-running process with KeyboardInterrupt, exit1. Tool sessions prove its lifetime.
  - Root: script suppresses primary ps/lsof stderr and does not refuse their nonzero statuses;
    empty SNAP falls through to the clean verdict. An ancestry lookup can fail separately, but
    the whole-process census must not turn unavailable evidence into absence.
  - Impact: false safe-to-clear status can strand a writer or lose verification continuity.
  - Owner/schedule: SPINE.23, P0; take immediately after current G1-SLICE.5b.1b.0/.0v commits/CI
    leave a clean tree, before namespace code. Add real denial/empty/success/live controls and
    fix the appropriate project-local seam; do not change any other Git repository.
  - Interim: all handoff censuses in this run use authorized OS visibility plus terminal tool-handle
    receipts. The controlled fixture process is stopped; restricted green alone is never evidence.

- **D115** — OS-visible handoff census treats idle CUA runtime metadata as project work.
  - Reproduce: the OS-visible D114 control snapshot blocks four persistent Codex CUA kernel/worker
    services as well as the actual controlled process. After stopping the controlled process,
    lsof -a -p13406,13407,14498,14499 reports only repo cwd entries and zero repo file handles.
    No CUA operation/tool result is in flight. Controlled PID46805 is absent in real ps, exit1.
  - Root: command lines carry the inherited workspace in harness configuration/working-dir metadata;
    the command-text arm calls that project work despite the documented inherited-cwd discriminator.
    Existing own-harness exclusions do not recognize these runtime invocation forms.
  - Impact: an authorized census can prevent legitimate clean handoffs indefinitely, while restricted
    execution masks the problem with D114's false green. Do not kill shared tool infrastructure.
  - Owner/schedule: SPINE.23, P0 alongside D114 after current archive/CI becomes Git-clean.
    Preserve detection of real project handles and active jobs; narrowly characterize idle runtime
    forms, test actual-handle controls and caller ancestry without blanket process-name exclusions.
