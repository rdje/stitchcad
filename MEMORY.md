# MEMORY — resume pointer (layer A; overwrite-only)

Read CLAUDE.md/README.md and doctrine pointers. Latest commit: git log --oneline -1.

## Current state

- Active G1-SLICE; frontier .5b.4c.h2.b.p.o.f.cleanup4 (blocked by D217).
- Next: recover the Codex runtime and obtain a complete OS-visible handoff census, then
  create a fresh cleanup plan and finish cleanup4 before publication builder .a23.3.2.
- Diagnosis complete: cleanup4.fix identifies unavailable PID/FD; no unsafe exemption.
- Host restart outside active harness required; rerun handoff and handoff contract.py --real.
- Owner/evidence: docs/tasks/G1-SLICE-producers.md; D217 in docs/tasks/PLANNING.md;
  recovery commands/primary sources: docs/book/src/annexes/artifact-cleanup.md.
- In-flight uncommitted: none; all issued jobs consumed.
- Handoff remains unverified while the runtime supplies incomplete lsof records.
