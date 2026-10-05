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

D177 records the same panic-evidence weakness in fourteen remaining entries. Their existing
.f.n02–06 repairs own guarded current test locations and independent controls before native signoff.

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
paths. The mixed domain entry validates all five source files before writing. Unsigned rounding
rebuilds both debug and release profiles after restoration; domain diagnostics rebuild both
unit and core targets. Run native mutation diagnostics exclusively.

Lexer and semantic-metadata mutation entries also guard declared paths before source reads or
output. The lexer validates both source files. Semantic children inherit the prepared stores,
including valid local overrides; they preserve the selected channel and unrelated environment.
Both entries rebuild their restored focused targets. The semantic classifier-only mode remains available.

Recipe and statement mutation entries likewise validate all declared sources/output before reads
or writes, retain prepared child stores and rebuild their restored full focused target. Recipe mode
also retains its coupled test selection; classifier-only mode invokes no native fault.

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
controls and179 actual body faults,
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
