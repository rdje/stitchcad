# Build and check the project

## Local producer profile

`scripts/local_environment.py` prepares the four destinations in the destination table below for
the Make targets listed below. The doctrine driver
(including its hook invocation) and G0 exit review also prepare their children before running checks.
Use `scripts/run_make.sh <target>` to prepare the profile before Make and platform tool dispatch
start. Direct Make recipes also prepare their children, but start after the host dispatcher.
The doctrine driver and review derive their roots from their script locations before invoking Git.
Paths derive from the current root, so relocating the repository preserves the profile.
Unset/empty values and ambient absolute paths outside the repository select local defaults.
Overrides may name directories under `target/`, relative to the root or absolute at runtime.
Every existing component must be a directory on the repository volume, without symlinks or Git
boundaries. Parent escapes, Git metadata and source-tree stores refuse before any store is created
or child executes. This profile neither migrates nor deletes a shared cache.

```bash
scripts/run_make.sh toolchain
scripts/run_make.sh check
TMPDIR=target/alternate-scratch scripts/run_make.sh probes
RUSTUP_TOOLCHAIN=1.99.0 scripts/run_make.sh toolchain
RUSTUP_TOOLCHAIN=1.99.0 CARGO_TARGET_DIR=target/alternate-build scripts/run_make.sh check
python3 -I -B scripts/local_environment.py --inspect
python3 -I -B scripts/local_environment.py -- cargo test -p sc-units
```

The first setup command installs the channel selected by `rust-toolchain.toml`; a named override
selects that channel instead. Setup requests the minimal profile, rustfmt, Clippy and the WASM target
with `--no-self-update`. The existing host launcher is a read-only platform dependency. Implicit
installation is disabled, so a missing local toolchain requires the explicit setup command.
Cargo owns its normal fingerprints; `make wasm` no longer deletes fingerprint directories before
validation. The wrapper executes the supplied argument vector directly and preserves the child's
exit status. `--inspect` prints relative destinations without creating them; `--verify` freshly checks effective
variables and existing directories without creating paths. No marker can bypass verification.

| Variable | Local profile behavior |
| --- | --- |
| RUSTUP_AUTO_INSTALL | Forced to 0; implicit toolchain installation disabled |
| RUSTUP_TOOLCHAIN | A named override is preserved; path overrides refuse |
| MAKE_TMPDIR | Coupled to the verified TMPDIR destination for Make temporary files |

Standing probes cover56 runtime cases,13 compiled helper faults, two actual Make faults, one
actual launcher fault, real
exec arguments/status and18 review-child environments. Make captures use compiler stubs; device
mismatch is simulated without off-volume writes. Actual stable setup/native/WASM checks run with
caller exports absent. D156's direct standalone native/temp producer audit under .h2.b remains
required before closure; CI's separate prepare/verify evidence above remains valid.

Daily artifact cleanup uses a frozen ignored-file plan, checks active jobs, preserves tracked
inputs and package/dependency stores, and verifies every selected path absent before regeneration.
The immediate read-only residue census must not activate this launcher: activation recreates
`target/scratch`. Use nonactivating verification, then rebuild native/WASM/book outputs and run
focused safety checks. The latest run is recorded in `docs/ARTIFACT_CLEANUP.md`.

The generated Knowledge Map lists task and decision links with concise labels. Their complete
repository-relative filenames remain in each link destination; freshness checks verify regeneration.

## Destinations

| Variable | Repository-relative default |
| --- | --- |
| CARGO_HOME | target/cargo-home |
| RUSTUP_HOME | target/cargo-home/rustup-ci |
| CARGO_TARGET_DIR | target |
| TMPDIR | target/scratch |

## Commands

Each target below is accepted by scripts/run_make.sh. Rustup, Python and mdBook are required
read-only host executables; toolchains/packages and project outputs use the guarded local stores.

| Target | Behavior |
| --- | --- |
| `check` | Formatting check, strict Clippy and all native tests |
| `fmt` | Format all Rust sources |
| `clippy` | All targets/features; warnings are errors |
| `test` | All native workspace tests, including documentation tests |
| `wasm` | Build sc-units, sc-core and sc-measure for wasm32-unknown-unknown |
| `book` | Build the mdBook under docs/book/book/ |
| `probes` | Run every diagnostic probe suite; report any failure |
| `gate` | Run the doctrine registry, including project checks |
| `toolchain` | Install the selected channel/components locally without self-update |
| `hooks` | Activate this repository's Git hooks |
| `bootstrap` | Run the repository bootstrap |
| `update-scaffold` | Run the scaffold updater with the supplied URL |
| `push-due` | Report whether 400 unpushed commits are reached |
| `help` | Print the target list |

Pushes follow the director2026-10-04 ruling:400 unpushed commits, with an earlier push only
by explicit request. CI/doctrine/hook changes have no automatic exception. The read-only
checker exits0 below cadence,1 when due,2 if Git metadata/count cannot be verified. It never pushes;
Make prints the verdict while deliberately ignoring that diagnostic exit status. Use
`PUSH_DUE_BASE=<revision>` for a historical comparison. Run full checks/gate/probes before a push,
then inspect the exact commit's CI jobs and every step; local checks supply no runner verdict.
Cadence controls run in the existing environment probe suite:19 verdicts and four actual checker-body
faults. Boundary/failure counts are simulated; two integrations use real Git counts, including a
historical comparison containing formerly exceptional paths. No source or Git metadata is mutated.

A refusal exits2 without executing a child; a running child retains its exit status.
For example, CARGO_HOME=crates/cache refuses as a source-tree store, while an ambient external
Cargo home selects target/cargo-home. TMPDIR=target/../scratch refuses before creating stores.
A named toolchain must already be installed through the explicit setup command; implicit downloads
stay disabled. Existing shared caches are retained, and no project workflow depends on their contents.

## Observed runner scope

At exact pushed8f87a10, the [doctrine job](https://github.com/rdje/stitchcad/actions/runs/37185684578/job/111386981964)
completed all eight steps successfully, including the new driver/profile. The [Rust job](https://github.com/rdje/stitchcad/actions/runs/37185684564/job/111386981852)
completed all eleven: actual stable1.99.0,703 native tests/59 groups, three WASM libraries and four
prepared/verified checkout-volume stores. Rust CI uses its separate explicit CI helper; local
profile/default/fault evidence above remains separately scoped. G1-SLICE.5b.4c.h2.v records the
observation; standalone producer audit .h2.b is still required. Product approval is unclaimed.

## Standalone audit boundary

The maintained producer inventory includes Python scripts, shell bodies and their embedded Python,
Make, workflows, Git hooks, public crate manifests and book configuration. Its tracked entry ledger
assigns each path to the Python, shell/destination or final integration audit. It retains every Python
call and complete non-Python source for inspection, including dynamic delegation and fixture code.
An inventory match establishes coverage of these sources; it does not establish runtime locality.
The census refuses missing or duplicate entries, malformed Python and literal heredoc boundaries,
source symlinks and nested Git boundaries. Its existing diagnostic suite watches those refusals.

A pre-repair capture of the actual size-membership mutation body stops at its first directory write
with all five store exports absent, before any source mutation or compiler invocation. This confirms
the original standalone gap owned by D156 .h2.b.p. Use the guarded Make launcher for current workflows while
the remaining direct entries and output destinations are repaired and verified. D156 stays open.

## Direct Python producers

The size-membership mutation entry now prepares the shared profile before its first owned output
or native child. It validates every declared output directory and source file before creating any
stores or changing the environment. Outputs must remain under target/; sources must already be
regular files within the current workspace. Parent escapes, symlinks, Git boundaries and foreign
volumes refuse. Invalid path types or control bytes also refuse. An invalid entry prints the
local-environment refusal and exits2 before invoking a child. Named toolchains and unrelated
environment values are preserved; paths still derive from the current repository root.

Size membership additionally declares all seven numbered mutation logs and restored.log before
reading inputs. Its implementation is an explicit mutable source; its current contract test is a
guarded read-only source. Panic evidence must identify an assertion macro's current source, line
and byte column in the failed-test body. Passed assertion names and labelled expect messages cannot
supply that evidence. Complete intercepted captures check eight children/eight files; four physical
copied-entry controls refuse the first/last log alias, a mutable-source hard link and a missing test
source before any input read, write, activation or child. Native proof remains separate: all seven
real faults match current macro sites, source bytes restore exactly, and the public current Cargo
artifact executes all12 tests successfully.

Size-chart observations now use the same complete plan: eight numbered logs/restored.log, the
mutable implementation and read-only current test input. Complete captures check all nine
children/files; four copied-entry alias/missing-input refusals and eight actual body faults pass.
Separate native replay verifies all eight current macro failures, exact inputs and the public
restored artifact's sixteen tests.

Measurement tables also declare nine logs/two inputs/one mutable source before reads. The existing
source-dependent mutation span is derived afterward. Complete nine-child/file captures and four
physical refusals pass; eight real current macro failures restore exact inputs and the public
sixteen-test artifact.

MTM declares thirteen logs/two inputs/one mutable source before reads. Complete thirteen-child/file
captures and four physical refusals pass; twelve real current macro failures restore exact inputs
and the public fifteen-test artifact.

Collections declare fifteen logs/two inputs/one mutable source, preserving all fourteen cases and
ordered replacement anchors. Complete fifteen-child/file captures and four physical refusals pass;
independent second-anchor omission and inline UTF-8 byte-column controls catch actual body faults.
Fourteen real current macro failures restore exact inputs and the public eighteen-test artifact.

Individual Ease declares eight logs/two inputs/one mutable source before reads. Its six pure
cases and five source-dependent statements still yield the original seven faults. Complete
eight-child/file capture and four physical refusals pass; seven real current macro failures
restore exact inputs and the public thirteen-test artifact.

Ease sets declare eleven logs/two inputs/one mutable source before reads, preserving eight pure
cases and six dependent statements. Complete dispatch/four physical refusals and independent
inline UTF-8 byte-location controls pass. Ten real current macro failures restore exact inputs
and the public fourteen-test artifact.

Signed rounding declares six logs/three inputs/one mutable source. Its imported Fraction fixture
is guarded with the current test; full dispatch/five physical refusals and actual fixture/totality
site omissions pass. The test's totality helper now uses an explicit assertion. Five real current
macro failures preserve all original cases and restore the public four-test artifact.

Unsigned rounding declares twelve logs/three inputs/one mutable source. Independent complete
captures preserve the nine debug calls, additional release call before source restoration and both
restored builds. File/profile/helper/fixture omissions and five physical refusals discriminate.
Nine real debug macro failures and one release failure preserve the original CALL-derived cases;
ten selected inputs restore exactly and both public five-test artifacts pass. Its totality helper
also uses an explicit assertion; production arithmetic remains unchanged.

Length operators declare seven logs/two inputs/one mutable source before reads. Six original cases
select two crossing contracts; complete dispatch/four physical refusals discriminate missing guards.
Six real failures reach current assertion macros and restore nine frozen inputs/three Rust files
exactly. The current public executable passes all four contracts, including the independent pair
oracle and Result typing. No Rust code or tests changed.

Domain diagnostics declare sixteen logs/six unique inputs/five mutable sources before reads. The
read-only public unit test and both private core test modules supply current macro sites; the core
modules also belong to the mutable production set. Complete fourteen-fault/two-restoration dispatch,
eight physical refusals and per-source/site/consumer omission controls pass. Fourteen real failures
match their expected failed-test sets; seventeen inputs restore exactly and current Cargo-selected
executables pass all five public unit tests and all three focused private guards. Rust is unchanged;
the private guards certify totality, separately from geometry.

The lexer entry plans ten diagnostic logs and four inputs under the
[local producer profile](#direct-python-producers) before reading or writing.
Both production files are explicitly mutable. The current public test and its compile-time imported
worked examples are read-only inputs. Failed-body panics must name an assertion macro in that current
test, including the scan helper; a passed test name or assertion-labelled extraction panic supplies
no evidence. Independent complete captures check all ten children, exact source binding and eleven
restoration writes. Six physical refusals and actual guard/helper omissions watch the plans; these
captures certify dispatch separately from native behavior. Nine real mutations preserve the original
cases, twelve frozen inputs and five Rust files. After exact restoration, the current Cargo-selected
executable passes all thirteen public lexer contracts. No grammar or Rust change is needed.

Semantic metadata plans three inputs: implementation, current public test and its compile-time
canonical table page. Ordinary mode declares eleven logs/one mutable source; classifier-only mode
declares neither files nor mutable sources and invokes no native child. It still prepares the local
stores/output directory and checks ten original anchors/current macro locations. Complete captures
of both modes, eight physical refusals and actual source/mode/restoration omission controls pass.
A copied classifier-only run accepts a read-only source with three hard links and ignores unused
log aliases, with no source/output writes. Ten real ordinary faults match their authored failed-test
sets; thirteen frozen inputs/six Rust files restore exactly and the current Cargo-selected executable
passes all five public metadata contracts. No Rust or metadata API change is needed.

Recipe mutations plan seven inputs: both implementations, current public test, three imported
TSVs and the worked-example book page. Ordinary mode declares sixteen logs; coupled mode declares
six, with both production sources explicitly mutable. Classifier-only mode declares zero files or
mutable sources and invokes no native child. Three complete captures preserve the original15+5
faults, exact source binding/child selection/restoration and prepared stores. Twenty-five physical
refusals and actual input/file/mutable/mode/environment/restoration omissions calibrate these plans.
Twenty real faults reach current assertion macros, including small-stack workers; twenty-two frozen
inputs/twelve Rust files/four compile-time inputs and the current full eleven-test artifact restore
exactly. Custom macro messages pass; a labelled join panic alone supplies no assertion evidence.

Statement mutations plan five inputs: implementation, current public test, its two imported TSVs
and the worked-example book page. Ordinary mode declares sixteen logs and one mutable source;
classifier-only mode declares zero files or mutable sources and invokes no native child. Both
complete captures preserve the original fifteen cases, full ten-test selection, read order, child
stores and restoration. Twelve physical refusals and actual input/file/mutable/mode/environment/
restoration omissions calibrate those plans. A copied classifier-only entry accepts read-only
linked input and ignores unused log aliases without source/output writes. Fifteen real faults
reach current macros in every failed-test body, including anonymous worker assertions. Twenty-one
frozen inputs/twelve unchanged Rust files/three compile-time fixtures remain exact; the current
Cargo-selected executable passes all ten public statement contracts after restoration.

Literal conversion plans four inputs: implementation/current public test/two compile-time TSVs.
Ordinary mode declares fourteen diagnostic files/one mutable source; classifier-only mode declares
zero files/mutable sources/native effects. Two complete captures, ten physical refusals and actual
input/file/mutable/mode/stores/restoration omissions preserve thirteen cases, selected exact tests
and the full six-test restoration. Thread panic locations must match current assertion macros; a
nonthread location mention refuses, with an independent legacy-regex body fault. Copied readonly
classifier mode accepts linked input/unused aliases without source/output writes. Thirteen real
faults reach current macros; twenty-two frozen inputs/thirteen unchanged Rust files/two compile-time
fixtures remain exact and the current full six-test executable passes after restoration.

Normalized expression plans five inputs: implementation/current public test/two imported TSVs/
worked examples. Ordinary mode declares eighteen diagnostic files/one mutable source; classifier-only
declares zero files/mutable sources/native effects. Two complete captures, twelve physical refusals
and eighteen mode omissions preserve seventeen cases, exact selected tests and full eight-test
restoration. Current thread macro locations qualify; nonthread mentions refuse, calibrated by an
independent legacy-regex fault. Copied readonly classifier mode accepts linked input/unused aliases
without source/output writes. Seventeen real faults reach current macros; twenty-three frozen inputs/
thirteen unchanged Rust files/three compile-time inputs and the current full eight-test executable
restore exactly. The four repaired D178 classifiers are summarized below.

Normalized recipe plans seven inputs: three implementations/current public test/three imported
TSVs. Ordinary mode declares eighteen diagnostic files/three mutable sources; classifier-only
declares zero files/mutable sources/native effects. Two complete captures, sixteen physical refusals
and twenty-four mode omissions preserve seventeen full-target faults and every source binding.
Current thread macro locations qualify; nonthread mentions refuse, calibrated by an independent
legacy-regex fault. Copied readonly classifier mode accepts linked input/unused aliases without
source/output writes. Seventeen real faults reach current macros in every failed body, including
the small-stack worker; independently authored failed-test sets stay exact. Twenty-seven frozen
inputs/seventeen unchanged Rust files/three compile-time TSVs and the current full eight-test
executable restore exactly.

Canonical recipe plans eight inputs: implementation/current test/four imported TSVs/book grammar/
byte decision. Ordinary mode declares twenty-two diagnostic files/one mutable source; classifier-only
declares zero files/mutable sources/native effects. Two complete captures/eighteen physical refusals
and twenty-four mode omissions preserve twenty-one original cases/full ten-test arguments/restoration.
Current thread macro locations qualify; nonthread mentions refuse with an independent legacy-regex
fault. Copied readonly mode accepts linked input/unused aliases without source/output writes. All21
real faults reach current macros in every failed body, including small-stack workers; independently
authored failed-test sets stay exact. Thirty-one frozen inputs/eighteen unchanged Rust files/six
compile-time inputs and the current full ten-test executable restore exactly. D178 closes after all
four original classifiers and their native cases are independently repaired and replayed; remaining
native groups inspect the boundary before signoff, with no broader affected-population claim.

Checked statement plans four current inputs, twenty-three diagnostic files and three mutable
implementation sources; classifier-only plans zero files/mutables/native effects. Two complete
captures/ten physical refusals/twenty mode faults preserve all twenty-two source bindings and
count-limited replacements, including both canonical-owner sites. Current thread macro evidence
refuses nonthread notes, with a legacy-regex fault. All three read-only sources admit three links
each and unused log aliases without native/source/output effects. Exclusive twenty-two real
faults match authored failed sets; every failed body reaches a current macro. Four D183 public
preconditions guard acceptance/refusal before extraction. Seventy-two inputs, sixty-three
unchanged production Rust files, twenty-three local logs and the current full seven-test artifact
remain exact. Full formatting/strict Clippy/fifty-nine native groups/seven-hundred-three tests pass.
D179 checked statement fixed; checked recipe remains immediate P1 .n08.4.

Checked expression plans two current inputs, seventeen diagnostic files and one mutable source.
Classifier-only declares zero files/mutables/native effects. Two complete captures/six physical
refusals/thirteen mode faults preserve all sixteen ordered cases, including both callee-priority
replacements; omitted second-step bytes refuse before dispatch. Thread-only current macro evidence
and an independent legacy-regex fault reject nonthread notes. Copied readonly input/unused aliases
cause no native or file-write effects. Sixteen exclusive real faults match authored failed-test
sets; every failed body reaches a current macro. D182's two measured dependency/conditional
extraction paths now assert acceptance first. Seventy-two inputs/sixty-three unchanged production
Rust files/seventeen local logs/current full seven-test executable remain exact. Full formatting,
strict Clippy and fifty-nine native groups/seven-hundred-three tests pass. D181 grammar availability
now names implemented statement/whole proofs; formal grammar stays. D179 checked expression fixed;
checked statement is also repaired; checked recipe remains immediate P1 and runs next.

Canonical expression plans seven inputs: implementation/current test/three imported TSVs/literal
annex/worked examples. Ordinary mode declares twenty diagnostic files; coupled mode declares four;
both declare one mutable source. Classifier-only declares zero files/mutable sources/native effects.
Three complete captures/twenty-five physical refusals and thirty-five mode faults preserve original
nineteen ordinary/three coupled cases, all five spacing replacements and the coupled_ selector.
Current thread macros qualify; nonthread mentions refuse with an independent legacy-regex fault.
Copied readonly mode accepts linked input/unused aliases without source/output writes. Original19
and3 real faults match authored failed-test sets; every failed body has current macros, including
small-stack workers. Consume each restored full9 executable before the next mode; ordinary evidence
is copied before coupled mode reuses diagnostic filenames. Twenty-six frozen inputs/fourteen unchanged
Rust files/five compile-time inputs remain exact through both runs. A final availability correction
changes only imported annex prose, retaining its six normative rows and the other twenty-five inputs;
the final current full nine-test executable is rebuilt and verified against that exact documentation.
D179 canonical/expression/statement fixed; checked recipe remains immediate P1 .n08.4.
D180 active decision/annex availability drift is fixed; identity bytes/grammar remain unchanged.

D177's fifteen entries/sixteen predicates are now repaired and independently replayed. Current
macro locations and calibrated failed-body controls replace message substrings and labelled
extraction/join panics. This closes that diagnostic defect; the remaining D156 producer plans
continue under their owned native/fixture/delegation/shell groups.

The shared Python API separates inspection from activation. `plan_producer` validates stores,
declared source files, output directories and explicit `output_files` without creating paths or
changing the environment. It returns the output directory names relative to the supplied root.
`prepare_producer` rechecks that plan, creates the stores/directories and returns the prepared
environment; `enter_producer` adds the standalone refusal diagnostic and exit2 behavior.
For example, from the repository root:

```python
from pathlib import Path
import runpy

profile = runpy.run_path('scripts/local_environment.py')
outputs = profile['plan_producer'](
    Path.cwd(), directories=('target/check',),
    output_files=('target/check/report.log',), sources=('Cargo.toml',))
assert outputs == ('target/check',)

# Declare any source that the producer may rewrite, including restoration.
profile['plan_producer'](
    Path.cwd(), sources=('Cargo.toml',), mutable_sources=('Cargo.toml',))
```

Missing output files are allowed. Existing destinations must be regular files on the repository
volume; symlinks, including dangling links, refuse. Their parent components also undergo the
directory, volume and Git-boundary checks. A directory declaration alone does not inspect filenames
inside it: each planned file must be declared before producing effects. The remaining producer
filename audit is a priority task within open D156; source/directory guard evidence does not certify
those undeclared destinations.

Existing output files must have exactly one hard link. Source-writing producers also declare
`mutable_sources`, a subset of `sources`; those files must have exactly one hard link before
activation. A missing declaration in `sources` refuses as `mutable producer source is not declared`.
Physical aliases refuse as `hard-linked producer output` or `hard-linked mutable producer source`.
Both activation helpers forward the declaration. Read-only sources retain their existing validation
and may have hard links; inspection still creates no paths or environment changes. The example above
validates Cargo.toml for a possible rewrite without performing one.

Physical fixture controls use three names for one inode and verify all bytes unchanged after
refusal. Native mutation entries still need to adopt complete filenames and explicit mutable sources
in their owned repair groups. Declared source/directory guards alone do not certify source-writing
ownership or undeclared outputs; D156 remains open.

The individual-Ease and Ease-set mutation entries use the same declared-path and profile guard.
Their native runs with caller overrides absent observe seven and ten real assertion failures,
then restore exact source and rebuild each current artifact. Mutation diagnostics run sequentially.

Measurement-table, garment-observation, chart-collection and MTM mutation entries also use
the declared-path guard before output or child dispatch. Each restores its exact source and
runs its complete focused test target again to rebuild the current artifact before returning.

Rounding, length-operator and domain-context mutation entries prepare the same profile and declared
paths. The mixed domain entry validates six unique inputs and five mutable sources before writing.
Unsigned rounding rebuilds both debug and release profiles after restoration; domain diagnostics rebuild both
unit and core targets. Run native mutation diagnostics exclusively.

Lexer and semantic-metadata mutation entries also guard declared paths before source reads or
output. The lexer validates four inputs, including its test and imported examples, and both mutable sources.
Semantic children inherit the prepared stores,
including valid local overrides; they preserve the selected channel and unrelated environment.
Both ordinary entries rebuild their restored focused targets. Semantic classifier-only mode plans
no files or mutable sources; its complete mode-specific proof is described above.

Recipe mode plans its complete files and seven inputs before reads; ordinary/coupled modes declare
both mutable sources and preserve prepared child stores. Classifier-only mode checks all anchors
and current macro evidence with no files/mutable sources/native child. Statement mode likewise
plans all five inputs and sixteen ordinary logs/one mutable source; classifier-only declares zero
files/mutables/native effects. Native modes rebuild the restored full focused target before returning.

Literal and normalized-expression entries guard implementation/test sources and output before
reads/writes, retain child stores and rebuild their full restored targets. Failed-body panic paths,
lines and columns must match current test assertion sites; custom messages pass and labelled expect
panics refuse. Three refusal preconditions now assert explicitly. Thirty compiled mutation cases
are actual macro-site reds; standing controls calibrate broad/prefix/site faults with independent
fixtures. Classifier-only mode checks anchors/noise without native fault dispatch.

Normalized-recipe and canonical-recipe entries likewise validate implementation/test paths before
reads/writes, preserve prepared child stores and rebuild their full focused target after restoration.
The normalized entry declares all three mutated sources and its read-only test source before output.
Both match failed-body panic locations to current assertion macro sites; custom messages pass and
assertion-labelled expect noise refuses. Their38 compiled faults are actual macro-site reds; six
source files remain exact and full eight/ten-test targets pass after restoration. Classifier-only
mode checks anchors/noise without native dispatch. Standing controls exercise actual site faults.

Canonical-expression and checked-expression entries also guard source/output before reads/writes,
inherit prepared child stores and rebuild complete focused targets after exact source restoration.
Canonical coupled selection remains intact; checked-expression classifier-only mode stays available.
Both require failed-body panic locations at assertion macro sites in their guarded current test
source. Custom assertion messages remain valid; assertion-labelled expect messages refuse.

Checked-statement and whole-recipe entries use the same guard and source-location classifier.
The statement entry plans all three mutated sources and its focused test source before activation.
Both preserve prepared child stores/classifier-only modes and rebuild complete restored targets.

Initial-namespace and ordered-name entries also guard implementation/test sources and output,
retain prepared child stores/classifier-only modes, and rebuild full targets after exact restoration.
The ordered-name entry plans both mutated sources first. Both use current test assertion locations.

Declaration and exact-name entries use these same guards, current test assertion sites and prepared
child stores. Declaration restoration rebuilds both the complete native target and its five doctests.
The captures also exercise its negative-construction dispatch; simulated output checks dispatch only.

Built-in and operator-signature entries now use these guards/current test assertion sites, retain
prepared child stores and rebuild complete restored targets. Their35 compiled faults are macro-site
reds, with four Rust sources exact and both full five-test targets restored. Classifier-only modes
check anchors/noise without native dispatch; custom messages pass and labelled expects refuse.

Wanted-signature and call-lookup entries use the same implementation/test/output guards and current
assertion-site proof. Wanted declares both test sources and preserves its joint target order in fault
and restoration calls; actual child controls refuse either target's omission. Their37 compiled faults
are macro-site reds, five Rust sources remain exact, and full wanted five-plus-five/call five-test
targets pass after restoration. Prepared stores and classifier-only behavior are retained.

Expression and reduction entries also guard current tests and require failed-body macro locations.
All fifteen compiled faults pass that proof; four current Rust sources restore exactly and the
full fifteen/six-test targets pass. Four expression refusal preconditions now assert explicitly.

The coupled recipe-input entry guards four implementations and its current test source, preserves
its original test filters, proves seven current macro-site reds and rebuilds the full five-test target.
Its five Rust sources restore byte-identically; labelled expect messages cannot certify a fault.

Recipe/statement owner compiler entries guard their source/output plans before reads or writes;
the recipe entry also plans this book's example source. Both preserve prepared Cargo/Rustc stores
and consume public current-artifact metadata. Ten real documentation guards require only the
expected E0451/E0515 codes; four recipe chapter examples compile and execute against that artifact.

The coupled static entry validates20 helper/reference/Rust/fixture/chapter inputs and its output
before helper loading or fixture reads. Both modes preserve shared populations, prepared stores
and public compiler/driver arguments. Driver panics must match a current assertion macro's source,
line and byte column; custom messages pass, assertion-labelled expect noise refuses. Exact Rust
source restoration and the complete restored driver rebuild run even after compiler/link refusal.
Captured compiler output proves dispatch separately from native faults; no numerical or physical
authority is added. Publication controls also reject D175's retired whole-static-graph claim, with
an independent later-section fixture and actual compiled catalog-omission assertion failure.

The first-phase bridge plans its own two source paths, output directory and nine file destinations
before loading the coupled module. That module retains its existing20-input import guard and
activation behavior. The producing call rechecks the complete22-source union and all nine outputs
before building, and every native child retains the prepared stores. Import-time bridge inspection
creates no additional outputs. Separate complete captures exercise both modes, all four child calls
and eight text outputs, early output/file-alias and late delegated-source refusals, and nine actual
bridge guard/environment faults. These captures simulate compiler/driver results to check dispatch.
The real consumer separately passes26,723 shared cases,263 public/reference phase cases and ten
reference faults; exact input bytes and the current public compiled artifact are verified afterward.

Standing shared controls cover31 runtime cases,24 output-file controls,26 physical mutable-path
controls and529 actual body faults, with35 calibrated failed-body classifiers,
including thirty-seven real
entry captures at their first write. The membership native run with caller exports absent observes seven compiled
assertion failures, then restores exact source bytes and rebuilds the current artifact with all
12 membership tests passing. All thirty-seven entry captures also refuse a late missing source before
input reads, output writes or child dispatch. Those thirty-seven captures establish their declared
source/directory guards; remaining file destinations still require the priority audit. Python
fixture/output and delegated entries remain under the
bounded .h2.b.p audit. Its finite output/delegation plan assigns each current Python candidate to a
bounded repair group in docs/tasks/G1-SLICE-producers.md. The watched TSV must cover each current
path/kind and terminal group owner exactly; missing, duplicate, malformed or unowned rows refuse.
Actual guard-omission faults and a split-owner positive check watch this planning boundary.
Each group reviews complete sources, transitive public entries and borrowed fixture/root contracts
before repairs. Pure library calls stay pure; historical captures retain fixed source revisions and
fresh-fixture contracts. This plan establishes ownership; runtime locality remains pending.
The additional `python_file_entries.tsv` covers every maintained Python path, including native,
profile and reference entries. Native file repairs have bounded canonical groups in the producer
task tree; the watcher rejects duplicate group identities, oversized groups, membership/owner drift,
and disagreement with the existing output/delegation owners. Split groups retain the parent record
and move membership to terminal children. Independent data refusals and actual guard-body faults
watch these rules. The first native repair is size membership, followed by the other native groups
and the existing fixture/delegation audits. Each repair must resolve its actual filenames, dynamic
label population and delegated public-call outputs before guarding the complete producing entry.
Assigning a source to an owner does not establish destination resolution or runtime signoff.
The original pre-repair capture is retained in the census record; current
captures check the repaired entry. This scoped repair does not close D156.

Twenty-six native-child capture cases execute actual entries with custom local stores,
intercepting every source/output write and initial/restoration child calls. The declaration entry
also captures negative-construction and native/doc restoration calls. The captures check actual
arguments, working directory, stores, channel and unrelated environment before dispatch. Reintroducing
the prior per-child store reset earns a body assertion failure, as does discarding coupled selection.
These captures compile no native fault;
the exclusive native mutation runs establish that separate proof.
Two complete compiler-entry captures additionally check every Cargo/Rustc/example call and all
generated fixture/log bytes under custom stores. Authored public metadata and compiler results
simulate dispatch only; native owner runs provide the separate diagnostic and example proof.
Three complete static-entry captures check baseline/mutation modes and an early compiler refusal,
with every write and child intercepted. Actual omission faults refuse a book input, mode selection
or restored compiler dispatch. Simulated driver output proves dispatch; native coupled runs supply
the separate shared-case and actual fault proof.

## Formula documentation currency

The publication probes watch current scope in nine formula annexes against public expression,
ordered-statement and whole-recipe check methods. Known retired availability claims refuse;
each introduction links the complete kind-proof boundary. Copied-body omissions must fail the
same stale-fixture assertion as the intact checker. A changed link label remains valid.
The statement contract count is checked against its public test markers, with both declaration
and source-marker faults watched. Actual native tests retain the execution proof.
These finite controls complement chapter/API/link checks; they do not certify all book prose.
Static proofs still grant no numerical execution, physical correctness or product approval.
