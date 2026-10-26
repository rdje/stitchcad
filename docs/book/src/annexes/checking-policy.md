# Checks during feature delivery

The current priority is complete feature implementation. Each feature addition receives one
small check of the behavior it changes. Select an existing affected unit or contract test;
add a focused regression when the change needs one. Reuse established test tools.

Broad regression suites, mutation campaigns, full publication probes and repeated platform
sweeps are deferred to later integration milestones. Full verification remains required before
an authorized push. A concrete failure or integration risk can justify a wider check sooner.

Code quality, task ownership, repository-local storage, book updates, commits and automatic
doctrine hooks remain required. State exactly what was checked and keep unrun verification
owned and scheduled. A focused pass does not imply that the full suite passed.

Maintenance work takes the frontier only when it directly blocks the selected feature or
can destroy or corrupt current work. Other audit and tooling followups remain pending behind
feature implementation. Finish and commit a dirty unit before changing course.

The product-progress report names implemented capabilities and roadmap acceptance. Maintenance
progress is reported separately, including maintenance tracked inside a G1 task.

This policy adopts the director's 2026-10-25 ruling. The canonical decision is
`docs/decisions/decision_product-work-takes-the-frontier.md`; `COMMIT.md` defines the workflow.

Run commit, successful brief clearing and post-condition checks in one shell invocation. When a tool
reports a running session, consume its terminal result before changing files, retrying the commit or
selecting another slice. An unread result may already have committed successfully. Before a retry,
inspect HEAD, Git status and the brief. A failed commit retains its brief for diagnosis; a successful
one clears it immediately. D227's duplicate receipt led to this explicit operational guard.
