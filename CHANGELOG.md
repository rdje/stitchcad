# CHANGELOG.md

Newest first:completed slices and focused validation. Whole earlier journals/navigation remain
retained under stable logical paths through the public reader.

| Sealed index | Retrieval |
| --- | --- |
| [Complete ledger and index through part307](docs/history/stitchcad-devnotes-part954.md) | `bash scripts/history_archive.sh read docs/history/stitchcad-devnotes-part954.md` |

## STITCHCAD-G1-0994 — sc-viewport:exact coordinates,picking and snapping

The viewport math every renderer will share. Views are exact fractions,so 1,000 zoom round trips return to exactly the same view,and picks and snaps compare exact distances. G1-03 stays not met until the renderers are measured.

## STITCHCAD-G1-0993 — the browser run:six steps in real Chrome,equal to native;G1-02 and G1-07 are met

Two Chrome processes load a project,evaluate a recipe,solve a constraint,save to IndexedDB,reopen,render and export,and every result equals the native program's. A fresh profile must fail,and does.
The book has a capability matrix from the recorded run. The drawing is a schematic because drafting is G2. The gauge reads 24 of 66.

## STITCHCAD-G1-0992 — the license census finds no copyleft;G1-06 is met

`bash scripts/check_licenses.sh` reads every package's license from `cargo metadata`,refuses copyleft and anything unreviewed,and proves the solver builds alone for both targets. The gauge reads 22 of 66.

## STITCHCAD-G1-0991 — three runtime profiles build;a session can draw and export;G1-01 is met

`native-full`,`native-headless` and `wasm-viewer` each have one command and a stated list of what they leave out. Sessions gain `render`,a schematic from evaluated values,and `export`.
A save now keeps files and fields it did not write. The gauge reads 21 of 66.

## STITCHCAD-G1-0990 — a headless command line drives the real command bus;G1-04 is met

`stitchcad replay` runs a recorded session:the reference skirt's recipe gives the book's values,an edit changes them,undo and redo restore them,a stale revision is refused,
and the same session prints the same bytes. Reopening never reissues a revision (D285). The worked examples' status is corrected (D389). The gauge reads 20 of 66.

## STITCHCAD-G1-0989 — strict clippy passes on the store;a false record corrected (D393)

0988's leaf recorded a clippy pass that was really a failure,hidden by reading it through tail. The lint is fixed and the record corrected. No behavior changes.

## STITCHCAD-G1-0988 — projects save,migrate and autosave;G1-05 is the first clause met

A project is now a manifest with a schema plus its files. Older schemas migrate all or nothing with backups,newer ones are refused untouched,
and unknown content survives. A process killed while saving and autosaving always reopens whole. The gauge reads 19 of 66.

## STITCHCAD-G1-0987 — the project store engine:a save is all or nothing

`sc-store` saves several files as one commit,so after a crash at any step a project holds exactly its old files or exactly its new ones.
Tests crash every step,crash recovery at every step,and kill a real process. Canonical text gives one byte form for any value. The project layer is next.

## STITCHCAD-G1-0986 — the constraint kernel:typed rules and a deterministic solver

`sc-constraints` answers which values fit a profile's rules,or names the facts and rules in conflict,in sentences,with no dependencies,on native and in the browser build.
4000 seeded problems agree with plain enumeration. The Z3 comparison compiles and has not run;its first run is the next push. No exit clause is met by a library alone.

## STITCHCAD-G1-0985 — six G0 clauses now check what they ask (D392)

Six clauses counted as met if a file existed or held one phrase. Each now checks its substance,reading the roadmap where it lists what is
required,so an emptied chapter or a dropped CI gate fails its clause. G0 stays at 18 of 19. No product code changes.

## STITCHCAD-G1-0984 — commits must name the roadmap clause they advance

The commit hook and hosted CI now refuse a commit with no clause,work on a later gate with no stated dependency,a held leaf,and a book or
tree that disagrees with the clause tables. Fifty commits with no clause met stop until a report to the director says why.

## STITCHCAD-G1-0983 — the gauge counts exit clauses through G6

`bash scripts/gauge.sh` closes each gate's clause rows against the roadmap's text and prints 18 of 66 clauses met,with G1 at 0 of 7.
The availability chapter shows the same counts. A clause counts as met only when its check runs a real product path and passes.

## STITCHCAD-G1-0982 — the 400-commit gate closes;the director's roadmap ruling is recorded

Local gates and all 29 probe suites passed,and 418 commits reached main at224a9805,with every hosted step green on that SHA.
The ruling that work stays on the roadmap through G6,with commercial CAD parked,is recorded word for word. No product code changes.

## STITCHCAD-G1-0981 — the handoff census waits for a passing reader (D391)

The census now blocks only on a process present on every look across about a minute;a terminal's status-bar read no longer fails a
probe run. No process name is exempted,and a real held file still blocks. No product code changes.

## STITCHCAD-G1-0980 — bring six stale checks back to what they watch (D390)

The pre-push probes found them:ci_environment had not run whole since0564. No check is removed or loosened;0974's late chapter read is
corrected by pinning the example count. No product code,book or README text changes.

## STITCHCAD-G1-0979 — state the input-check hold in the semantic recipe chapter (D389)

Three sentences still called held binder work the next implementation;they now state the hold. Prose only,no code fence moved.
D389 logs the worked-examples status line,which no product test has settled either way.

## STITCHCAD-G1-0978 — return feature work to the roadmap's owners (D388)

Director 2026-11-06 adopted the feature characterization and pointed it back at the roadmap,task tree and book.
0977's six parallel leaves are gone;`.8` is next by the roadmap's clause order. The book table now shows the seven G1 exit clauses.

## STITCHCAD-G1-0977 — hold input checks and open the thin executing path (D387)

Director ruling2026-11-06:input checks on hold,all in on feature implementation;more checks come from real need.
16 leaves are blocked on the ruling;nothing is removed. G1-SLICE.thin-path is the new frontier after the due push.

## STITCHCAD-G1-0976 — record the measured distance to the G1 exit (D387)

Availability now states each G1 exit requirement's real state,and a tracked leaf holds the open director decision.
654 commits at6/18 passed the drift census because it counts commits,not distance to the gate. No order changes yet.

## STITCHCAD-G1-0975 — declare every book name and check at each commit (D386)

29 chapters gain a declaration table covering all795 undeclared names;glossary suite43/0 and G0 engineering clauses pass.
A new BOOK-CENSUSES doctrine runs the book censuses at each commit,so this drift cannot wait for a push again.

## STITCHCAD-G1-0974 — re-anchor faults stranded by a refactor (D385)

Nine namespace and ordered-name fault anchors follow the0836 move;every fault still fails a compiled assertion(17+18).
The recipe contract now compiles and runs all14 book examples,not a stale4. Glossary declarations follow.

## STITCHCAD-G1-0973 — retire closed task leaves under the fixed ceiling

67 closed leaves move whole into record1094,freeing885 lines below the unchanged20000-line task ceiling. All1623 leaf
identities,statuses and parsed fields stay live;43 Git intervals match exactly. Pre-push repairs continue.

## STITCHCAD-G1-0972 — restore book publication and the G0-19 clause (D384)

Two chapters gain topic-index rows;my 0971 marker and 0968 out-of-tree link are corrected;G0-19 builds before it
tests. Publication suite50/0 and14 search cases pass. Glossary and mutation-anchor repairs follow.

## STITCHCAD-G1-0971 — name every failing probe suite (D383)

make probes now lists failing suite paths,so a set -e suite that stops silently cannot hide behind one status bit.
The L6b span gets its rust marker and stale copied-book fixtures are cleared;the formula language suite passes16/0.

## STITCHCAD-G1-0970 — restore the sealed changelog index route (D382)

The live changelog links record954 again and the POINTER rule follows only that record. Ledger probes11/0,
17 pointer contracts and5 mutation reds pass;306 parts match by independent grep. Other pre-push repairs follow.

## STITCHCAD-G1-0969 — run the overdue daily artifact cleanup

Guarded cleanup removed182 regenerable paths,374871872B,with zero residue and all2254 skipped paths intact.
No tracked product content changed. Pre-push repairs continue from this clean point.

## STITCHCAD-G1-0968 — watch the RAM trend instead of stopping at a percentage

Director rule2026-11-06:observe the trend and flag only a sustained rise above90%. macos_memory.py drops its
86/88 admission verdicts and reports a flag line;book and records follow. Self-test and mutant control pass.

## STITCHCAD-G1-0967 — own the due pre-push gate

A new leaf owns the full check/gate/wasm/probes run,normal push and hosted-job observation for the
derived400-commit milestone. It claims no semantic acceptance;gate results follow in later commits.

## STITCHCAD-G1-0966 — reconcile producer inventory before push (D381)

The census and effect/file plans cover all370 sources again;a42/a43/n17 own later additions and n16 groups
regain membership. plan_sources.py now enters the producer profile,replacing ambient Cargo/Rustup stores.
Focused census contract and real probe pass;full pre-push gate is next. No runtime locality signoff.

## STITCHCAD-G1-0965 — save requested handoff-ready pause

PNT pauses after a47d761f's verified RAM diagnostic correction. OS-visible handoff passes;
no reference implementation or intensive job started. Resume only on request at.references.record,
with cache-aware RAM admission. Producer reconciliation/full pre-push gates remain pending.

## STITCHCAD-G1-0964 — correct cache-inclusive RAM refusal (D380)

Independent Activity Monitor/Apple category evidence corrects the top-based conclusion. A read-only,
cache-adjusted diagnostic names estimate semantics and explicit peak/pressure/86–88% admission.
Frozen subtraction,old-metric RED,exact thresholds and invalid-input checks pass;existing probe
runner watches the self-test. Incorrect resume blocker removed;Rust/full producer proof unclaimed.

## STITCHCAD-G1-0963 — record authorized run and tighter RAM ceiling

Director authorizes continued work with88% hard host RAM ceiling. The new OS-visible reading
is already above that ceiling;Rust remains deferred until stable headroom is available.
Resume/task owners retain the precise blocker. No memory-intensive job or product change.

## STITCHCAD-G1-0962 — specify bounded placement reference execution

Current-reference planning now names complete query,retention,resolver-visit and ownership-work
bounds,original source parents and first-cause order. One-record and complete-class implementation
remain pending. Focused task-history/document checks pass;no new Rust/runtime proof claimed.
Rust work awaits director-confirmed RAM headroom;no memory-intensive job started.

## STITCHCAD-G1-0961 — save director-requested clean pause

PNT pauses after committed source joins d539e866. OS-visible handoff census passes;brief empty
and Git clean before this checkpoint. Resume only on request at bounded placement current
references;existing bounded point/range primitives are named in the pause leaf. No product change.

## STITCHCAD-G1-0960 — bind complete original Closure placement sources

The source class borrows exact stable role/member/current-work parents and the same finite cursor.
N+1 whole reservation precedes fresh aggregate/source work;late typed refusals retain original
work/index/raw source witnesses without a successful prefix or refunded work. Ten focused cases,
original association4,API3,strict library/test lint,formatting and book build pass. Current references,
aggregate recipe/native/cohort and whole integration verification remain pending.

## STITCHCAD-G1-0959 — compose complete original Closure placement work

The consuming stage reserves2N+1 whole passes before aggregate/member/current-copy work. Every
original instance role retains canonical notion and actual current copy/Piece loans;typed late
resources/missing targets/copies drop private prefixes and retain the same spent cursor. Nine
focused cases,affected Closure7,API3,strict library/test lint,formatting and book build pass.
Placement source/reference and whole native/cohort/integration proof remain pending.

## STITCHCAD-G1-0958 — bind original Closure instance and canonical notion members

Fresh complete content and matching admission precede exact Closure/own-instance/First-or-Second/
canonical notion joins. Original loans and indices survive reorder/multiple instances;typed missing
and quota refusals preserve their first causes. Focused Closure6/Pocket2,API3,arithmetic1,strict
library/affected-target lint,formatting and book build pass. Owned test panic/indexing lint defects
are fixed with checked loans,typed failures and explicit query detection. Copy/source/reference
and complete native/cohort integration remain pending;G1stays6/18 In Progress.

## STITCHCAD-G1-0957 — retain complete closed protocols before physical relations

Eight exact Git intervals retain whole closed original factory/acceptance and recent relation
protocols plus complete committed journals fromfb02c244. Independent Git/hash/base64/public-reader
proof precedes narrative retirement. Every original live task identity/status/heading remains;
fixed limits,old seals,protected stores and product behavior stay unchanged.

[Whole earlier ledger and navigation](docs/history/stitchcad-devnotes-part1093.md) retains exact committed bytes fromfb02c244.
