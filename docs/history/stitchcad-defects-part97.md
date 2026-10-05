# Sealed archive — D176 task index coverage drift

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.i` on `2026-10-05` (UTC).

- **Sealed identity:** 16 lines, 1629 bytes, `sha256:b0b0a66f706bed5d6aba17f36262f582c525ac6f9935dc17e4dfe6fb66ed9333`
- **Coverage:** D176 original command examples, source/history diagnosis and verified closure.
- **Retrieval:** content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

### Defect D176 — stale current task-coverage command totals (closed)

- Owner: G1-SLICE.5b.4c.h2.b.p.o.i / STITCHCAD-G1-0151.
- Reproduce: run the local-launcher planning census/probes and compare docs/TASK_TREE.md's command examples.
- Actual census:10 lanes/13 trees/13 siblings/zero unowned/orphans/dead links; probes9 pass/0 fail. Original examples below claim11 siblings/seven probes.
- Impact: current-command recipes misstate derived coverage and control population; actual ownership still passes. This is documentation drift, not a missing tree.
- History:19900d013faee51ea014aec5a531ef5962c36066 set11 siblings;513374c22c97600b735d398c00001bde2e9b5829 introduced seven probes. Later sibling and fragment-control additions left those fixed examples stale.
- Repair: retain both commands, replace fixed successful-population comments with current derived coverage/zero-failure requirements. Current results remain in verified live/leaf receipts; immutable past records stay unchanged.
- Verification: target/effect-plan-current-coverage.log and effect-plan-coverage.log terminal rc=0. Current command examples retain both invocations and contain neither obsolete result; independent source/history comparison proves original drift.

Original two command examples at ab3f720, complete bytes:

```bash
bash docs/tasks/artifacts/planning/run_tree_coverage_census.sh   # → 10 lanes / 13 trees / 11 sibling(s) / 0 unowned / 0 orphan(s) / 0 dead link(s)
bash docs/tasks/artifacts/planning/run_tree_coverage_probes.sh   # → probes: 7 pass / 0 fail — watched, because a census no gate runs can go red unnoticed (D44)
```
