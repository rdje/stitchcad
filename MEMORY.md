# MEMORY — resume pointer (layer A; overwrite-only)

Read CLAUDE.md/README.md and doctrine pointers. Latest commit: git log --oneline -1.

## Current state

- PNT frontier: G1-SLICE.5c.2.4 — exact abs/min/max/clamp numeric built-ins.
- Next: reuse completed numeric values/exact ordering and existing homogeneous builtin signatures.
- Owner/evidence: docs/tasks/G1-SLICE-recipes.md; deferred audits in G1-SLICE-producers.md.
- Policy: one focused check per feature; broad audit/calibration deferred to integration/pre-push.
- In-flight: none after0250 commit; focused numeric4/0+rational5/0; broad/platform .5c.4/pre-push.
