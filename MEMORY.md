# MEMORY — resume pointer (layer A; overwrite-only)

Read CLAUDE.md/README.md and doctrine pointers. Latest commit: git log --oneline -1.

## Current state

- Active G1-SLICE; frontier .5b.4c.h2.b.p.o.f.cleanup4.hunt (D217 investigation).
- Next: external daemon restart with unrelated descriptors closed, then immediately capture
  replacement descriptors before cleanup; run handoff/contract.py --real. See cleanup annex.
- Continue causal investigation until repair is verified; then finish cleanup4 and stop.
- Owner/evidence: docs/tasks/G1-SLICE-producers.md; D217 in docs/tasks/PLANNING.md;
  recovery commands/primary sources: docs/book/src/annexes/artifact-cleanup.md.
- In-flight uncommitted: none; all issued jobs consumed.
- No cleanup apply until the runtime supplies complete process/handle evidence.
