# Checks during feature delivery

Prioritize complete feature implementation. Check each addition with one affected unit/contract
test; add focused regressions when needed and reuse established tools. Broad regressions, mutations,
full publication probes and repeated platform sweeps wait for integration milestones. Full gates
remain mandatory before authorized pushes; concrete failures/risks may warrant wider checks sooner.

Code quality, task ownership, repository-local storage, book updates, commits and automatic
doctrine hooks remain required. State exactly what was checked and keep unrun verification
owned and scheduled. A focused pass does not imply that the full suite passed.

Maintenance work takes the frontier only when it directly blocks the selected feature or
can destroy or corrupt current work. Other audit and tooling followups remain pending behind
feature implementation. Finish and commit a dirty unit before changing course.

The product-progress report names implemented capabilities and roadmap acceptance. Maintenance
progress is reported separately, including maintenance tracked inside a G1 task.

Director2026-10-25 ruling: `docs/decisions/decision_product-work-takes-the-frontier.md`;
`COMMIT.md` defines the workflow.

Commit, clear the successful brief and check post-conditions in one shell. Consume any running tool's
terminal result before edits/retries/another slice: it may already have committed. Before retrying,
inspect HEAD/status/brief. Failure retains the brief for diagnosis; success clears it immediately.
D227's duplicate receipt established this guard.

Size checks own unique repository-local scratch, removed on trapped exit. D274's native-wc overlap
control protects in-flight measurements from another checker's cleanup. D275 mutates the actual
row's at= token and requires its exact stale-revision refusal. Caps/coverage/refusal rules stay fixed.
