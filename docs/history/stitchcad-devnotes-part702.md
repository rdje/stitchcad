# Sealed engineering continuity — part702

- **Identity:** 41 lines, 4461 bytes, sha256:3711e1deacdb064ab3b8efb2ee82dc792282362c46949f036edf8b279935f133
- **Coverage:** whole0543 task scope, lesson and changelog from b08f0cab72dad292bc1398969ca3747550187617.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 25845–28469, 2624 bytes, sha256:d693225d107ffa8977299399d6963b4d9c9e0010d3ad919b4cecdc14ce522c74
- Whole source interval: DEV_NOTES.md bytes 193–1335, 1142 bytes, sha256:ccd5ef4203835f277d0fe7eb00df6996bfda92fbf00255a157a59411bceefda7
- Whole source interval: CHANGELOG.md bytes 207–902, 695 bytes, sha256:634da28ecbe47ad6c8fd5e12d8b49eea0e19483f3d8e4c1c12b7a2372fcec480
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.3.2.prepush-400.matrix-transport`
  Status: `done`
  Goal:0543 D316 P1 align normative MCP envelope with already approved roadmap v0.4;
  watch the stale restriction without approving any runtime transport or changing policy.
  - [x] REPRODUCE / ISSUE: feature-matrix7 claimed MCP stdio-only while roadmap7.8 and active
    decision_mcp-transports permit four profiles eligible from G1 after admission. Before-change
    command census →17/5/5/0,rc=0,target/d316-before-census.log;the contradictory row was accepted.
  - [x] ROOT CAUSE (WHY + WHERE): source check_transport_profiles reads roadmap,command chapter
    and glossary;its normative-input list omitted envelope. Direct source/decision comparison
    confirms stale row,rc=0. This is a bounded K7 input gap,not a claim about every repository
    checker. Original policy already requires threat model/tests and explicit network/relay opt-in.
  - [x] FIX: envelope row now says stdio-first,each profile needs its threat model/admission tests
    from G1. K7 reads parsed MCP envelope row,requires one five-cell row and refuses retired spaced
    or hyphenated stdio-only restriction. Actual copied-row faults remove/duplicate/malform/restore
    old policy;removed guard bodies must admit the same faults. No policy or runtime implementation.
  - [x] ADDRESSED (verified): local_environment.py -- bash
    docs/tasks/artifacts/command_layer/run_command_layer_probes.sh →15 pass/0 fail,rc=0;
    new envelope5 refusals/actual checker-body assertion reds,prose control0. Existing transport8
    refusal/body reds,glossary refusal,future/historical positives pass;source exact. REAL17/5/5/0.
  - [x] NO REGRESSION: bash -n both command scripts →rc=0;feature matrix census107/29/0,
    rc=0;mdbook build/positive publication93/248/1590/2771,rc=0. Receipt701 →3 exact Git spans,
    rc=0;retention1155 logical/60 working/65098 decodedL/5067711 decodedB/2645048 residentB,
    rc=0;git diff --check →rc=0. Full gates/push/actual hosted jobs follow remaining D313–D315.
  - [x] LOCKSTEP: whole0542 task/lesson/changelog retained701. MEM/DEV/CHANGE/LIVE/index/ledger/
    book align;D316 fixed,29 suites/G1 6/18 unchanged. Last scoped G0 review18/1 remains human
    unapproved;runtime profiles unapproved. README objective/commands unchanged. Next D313 cleanup
    guard isolation;commit before moving. This applies existing transport decision,not a new choice.
    promotion: declined (application of indexed approved transport policy and independently falsified normative projection checks;canonical decision already answers this scope).
## _(2026-10-29 UTC)_ — align and watch the approved MCP envelope

0543 D316:matrix stdio-only row contradicted active four-profile roadmap/decision while K7 read only roadmap,command chapter and glossary. Correct the projection to stdio-first with per-profile threat model/admission from G1. No runtime approval or new policy.
K7 requires one five-cell MCP envelope row and refuses retired restriction. Command15/0 includes5 copied-row refusals/actual body assertion reds and prose positive;existing8 transport body controls,glossary/future/history positives pass0. Before-change census17/5/5/0 accepted stale row.
Book93/248/1590/2771,matrix107/29/0,701 receipt3/retention1155 pass0. Whole0542 retained before curation. LIVE29 suites/G1 6/18 unchanged,D316 fixed,D313–D315 owned. Last G0 review18/1/human unapproved;full gates/push/actual hosted jobs await repairs.
[Whole0542 matrix repair](docs/history/stitchcad-devnotes-part701.md). Promotion declined:existing approved transport decision/independent projection checks. Scope is the actual K7 input gap,not a whole-tree absence claim. Next controlled protected-store mutation D313.
## STITCHCAD-G1-0543 — align and watch the approved MCP envelope

- Fix D316 stale stdio-only row using existing approved stdio-first/per-profile admission policy. K7 requires one valid envelope row and refuses the retired restriction;no runtime transport approval or roadmap change.
- Command15/0 includes5 envelope refusals/actual checker-body assertion reds and prose positive;existing transport controls pass. Matrix107/29/0,book93/248/1590/2771 and history checks pass;whole0542 retained701/three exact Git spans.
- LIVE29 suites/G1 6/18 unchanged,D316 fixed,D313–D315 owned. Last G0 review18/1 remains human unapproved. Full gates/push/actual hosted jobs wait for remaining repairs.


