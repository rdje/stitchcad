# Sealed archive — whole D185 report and verified closure

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.f.n09.1` on `2026-10-05` (UTC).

- **Sealed identity:** 16 lines, 1362 bytes, `sha256:058f3165cd7c5781e294a656ab41083daf35a72c9d2e504ffefc1969312fcd7b`
- **Coverage:** whole report/closure; docs/tasks/PLANNING.md interval 78846–79494, SHA-256 c9a6fb6cc1539c93e807e6ba1a28917d3e9262027b917209706ea590df89035c.
  Report authored over baseline 335b64d91fc1c8d54c67f7599bb25fca7a71a5f7 and absent from that commit; closure follows.
- **Retrieval:** content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D185** — namespace fault classifier accepts a nonthread location note as current macro
  evidence. Reproduce: original namespace_mutations.py through shared classifier_controls
  with strict=True/thread_only=True raises `D159 assertion evidence accepted noise`; diagnostic
  harness exits0 after verifying that exact failure. Unanchored regex accepts compiler-note text
  inside a failures block. This weakens fault signoff without changing engine behavior. Own
  G1-SLICE.5b.4c.h2.b.p.o.f.n09.1, P1 now; require thread-bound location, actual omission
  control, all17 real fault bodies/current artifact and exact restoration before closure.

D185 closes at .n09.1: namespace classifier requires a current thread macro; the actual
legacy-regex body fault independently refuses nonthread location mentions. Two complete
captures/six physical refusals/thirteen mode omissions preserve original17 cases and full10
argv/stores/exact restoration. Shared560 body reds/35 classifiers/38 captures/210 refusals0.
All17 real faults match corrected source-derived sets; every failed body reaches a current
macro. Prediction5 gained privacyJ because its eps_phys refusal uses optional metadata;
other16 sets exact before correction. Frozen72 inputs/63 production Rust/public tests/18
local logs/current full10 executable429f1099 exact0. No Rust/engine/grammar change.
