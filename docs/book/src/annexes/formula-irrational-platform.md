# Observed native and browser numerical execution

The numerical APIs now have actual execution evidence on one native/browser pair. The same31
public-consumer checks run as ordinary Rust code on the native host and as WebAssembly inside
installed headless Chrome. Each check keeps its original assertions. This evidence covers the
available numeric/sourced APIs;the complete wasm-viewer application workflow and browser/device
matrix retain their G1/G5 owners.

## Observed runtime pair

| Component | Observed identity |
| --- | --- |
| Native compiler target | aarch64-apple-darwin |
| Browser compiler target | wasm32-unknown-unknown |
| Rust | 1.99.0,b940084d7eb6a299eb4bfeb8e34901bc051e7ac4,LLVM23.1.1 |
| Host | Darwin27.0.0,arm64 |
| Actual browser | Google Chrome156.0.8078.74 |
| Build mode | Release,offline and locked dependencies |

Compiler and Chrome version commands agree before and after the run. The receipt also pins
the Chrome launcher hash. Its raw browser user-agent string is retained separately;host
identity comes from the OS/compiler records. This observation does not establish support for
another compiler,browser version or operating system.

## Original checks and fixtures

Nine existing integration-test files supply37 ordinary test functions. Six functions regenerate
fixtures through host Python subprocesses;those remain native checks and are explicitly excluded
from the browser adapter. All31 remaining functions execute on both runtimes. Closed manifest
checks refuse an unclassified test or a changed population before compiler/browser dispatch.

| Mathematical fixture | Original rows | Covered boundaries |
| --- | --- | --- |
| sqrt/hypot | 85 | Exact midpoint roots,full rationals,domain and later storage refusals |
| arc_length | 23 | Signed turns,independent pi midpoints,exact widths and scalar domains |
| sin/cos | 44,with88 function results | Full turns,midpoint neighbors,sourced storage and T1 |
| atan/atan2 | 81 | Principal signs,whole ratios,quadrants and negative180 degrees |
| tan | 139 |125 values,10 poles,4 exact widths and signed storage boundaries |
| Tan resource outcomes | 33 | Preparation/execution stage,next demand,prior counters and precedence |

The six frozen fixtures contain405 rows and26610 bytes. Additional ordinary checks cover actual
kind priority,zero vectors,exact signed endpoints,full-width operands,false/cancelled results,
error chains,refinement and both source/policy retention. All original550 project input hashes
remain unchanged throughout the capture. The original numerical implementation is not copied
into a substitute kernel or amended for the browser.

The adapter removes only test-discovery attributes from isolated consumer copies and adds
small call wrappers in those same modules. Every assertion body remains intact. The adapter
links the actual sc-core through its public APIs,with dependencies checked against the existing
root lock. Its source,manifest,lock,consumer copies and all fixtures remain in the retained run.
Both the original project and all copied consumer/fixture bytes are checked after restoration.

## Actual browser execution and a failing control

Ordinary Rust main calls every selected function,checks31 completions and returns the computed
ExitCode104. The native executable returns104. The browser compiles and instantiates the
captured WebAssembly bytes,then calls that same main and observes104. The module has no host
imports. This bridge preserves unsafe_code forbid and needs no unsafe export or new binding
dependency.

An embedded local HTML page runs synchronously inside actual Chrome. It has no server or
external page dependency. A restrictive content policy permits the inline script and Wasm
execution. The runner reads the serialized result element after execution,as supported by the
[Chrome headless DOM command](https://developer.chrome.com/docs/automation-and-testing/headless-cli).
[Rust's target notes](https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html)
describe the target's host/standard-library boundary;the selected checks require no host Python.

The failing control changes one copied independent expectation:the first tangent row at
negative720 degrees expects1 ppm instead of its original0. Both native and Wasm builds still
compile successfully. Native execution then returns101 with the named fixture assertion;
actual Chrome reaches execution and raises WebAssembly.RuntimeError. A compile,import or
JavaScript protocol failure cannot count as that rejection. This is a runner/expectation control;
the separate per-function chapters retain their actual kernel-body mutation evidence.

The copied fixture is restored after every outcome,including exceptions. Four actual failure-
writer ownership cases and an in-memory guard omission also pass. A refused preflight or reused
process identity cannot amend an earlier retained capture. Run identities include UTC time and
process identity,and the mutable latest receipt pointer is published atomically.

## Data locality and retained original evidence

D344 exposed an exploratory ABI probe that supplied a local Chrome profile/cache but inherited
the host's off-volume temporary directory. That exploratory launch supplied no browser numerical
verdict. The reusable runner now prepares the local producer environment and explicitly supplies
repository-local TMPDIR,TMP and TEMP to its children. Chrome profile,cache and crash paths also
belong to the fresh retained run. The short temporary namespace is repository-derived.

The completed run observes no new ambient Chrome-family temporary entries. After an OS-visible
clear process census,the owned temporary tree moves into the retained run with equal entry/
size/hash census and zero old-path residue. This run's final temporary tree is empty. The first
probe's empty,time-correlated host directory has ambiguous ownership and is left untouched under
the shared-cache policy. No outside cache is deleted. Complete transitive compiler/browser
direct runtime-locality review is completed below;complete transitive proof remains D156.

The capture store target/irrational_platform_certificate/ was explicitly protected before
creation. Generic cleanup refuses it directly and prunes it from the census. The focused cleanup
control checks all three numerical stores and five archive/search families:protected8,ordinary
outputs2,four actual guard omissions rejected and restored source/data exact. The legacy policy
comparator admits only the three declared stores and existing family adapters. This protection
change performs no daily cleanup and does not alter the last completed cleanup date.

The original run is retained at:

```text
target/irrational_platform_certificate/0640-20261030T173036575781Z-2055/report.json
sha256:46a3dd7aecc9e0d5a95d4b55edf13f7ab29600d2ac881aa10c33a1479f7adf19
```

Its directory retains both native/Wasm binaries and hashes,original inputs and restored copies,
compiler/browser records,build/run logs,actual DOM captures,isolated profiles and temporary
evidence. Earlier failed discovery/process-admission attempts remain separate incomplete records.
Only the complete receipt above carries this numerical verdict.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/irrational_platform.py --plan-only
python3 -I -B docs/tasks/artifacts/formula_structure/irrational_platform.py
```

The actual capture requires a clear project process census;finish other project jobs first.
Installed OS/browser launchers are read-only external tool dependencies. Project sources,
generated workspaces,dependency stores,profiles,captures and scratch stay on the repository
volume with repository-relative persisted paths. The final adapter passes strict affected
Clippy and formatting. The fresh complete certificate reads below now pass. D345 capture
preservation and full probes pass below. The [scoped numerical review](formula-irrational-review.md#scoped-acceptance-and-remaining-owners)
is complete;aggregate resources,provider/replay,geometry,physical and human release owners remain.


## Fresh complete certificate acceptance

After the numeric,mixed-call and observed-platform implementation slices,0641 reruns the full
inverse and forward readers against their original protected stores. Both terminate successfully:
45,000,000 inverse rows/450 chunks and1,000,000 forward rows/100 contiguous chunks. Their exact
minimum margins match the [original proofs](formula-irrational-review.md). Every encoded strict
gap and digest passes;source-locked producers retain the mathematical/index construction
boundary. Readers check encoded gaps;they do not reconstruct outward interval construction.

The compact inverse witness regenerates byte-for-byte:1299 bytes,SHA-256
6b60d154f4160b0599f0ad13731b943e3e8dbd05f29496e33a186dea2d06840b.
The atan P576/five-half-angle/64-term plan passes independent14 atan/16 atan2/two zero-vector/
four root/two weak-budget controls. Its analytic Newton bound remains13;observed maximum is11.
The sin/cos P512/64-term plan passes14 Fraction/50 signed-octant/two weak-profile controls.
Forward construction passes known3/Fraction4/small-Q32/negative2 controls without rewriting the
historical feasibility report. The qualitative midpoint checker passes8 theorem values/9 axes,
161 exact Chebyshev identities/three known roots and the scale1 counterexample. Its huge
conservative precision ceiling is qualitative evidence,not a practical resource promise.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/verify_inverse_certificate.py
python3 -I -B docs/tasks/artifacts/formula_structure/verify_forward_certificate.py
python3 -I -B docs/tasks/artifacts/formula_structure/inverse_certificate_witness.py
python3 -I -B docs/tasks/artifacts/formula_structure/atan_runtime_plan.py
python3 -I -B docs/tasks/artifacts/formula_structure/sincos_runtime_plan.py
python3 -I -B docs/tasks/artifacts/formula_structure/trig_midpoint_certificate.py
```

Original logs,manifests,witness and nine fault-report preservation baselines have19 verified
copies beneath this protected receipt:

```text
target/irrational_platform_certificate/coupled-0641-20261030T180826454783Z-33211/receipt.json
sha256:0670624492d132854ba8f876726a9d7734ed27bde6dffe262dfbc4c66f240596
```

The complete local probe gate now passes below. D345 exposed fixed native fault-capture paths that a
later replay could overwrite. The actual old writer replaced a sentinel only in an owned
controlled copy;all nine original reports remain unchanged. The preservation repair below
passes before full probes. Scoped numerical acceptance is recorded in the coupled review;
aggregate resources,provider/replay,geometry,application and production approval remain owned.


## Fresh native fault-capture identities

D345 is fixed in the shared root_body_faults.py controller used by all six native entry points.
Each invocation keeps the wrapper's historical run path only as a provenance label. The actual
capture receives a UTC-microsecond/process suffix beneath the already protected numerical store:

```text
target/irrational_platform_certificate/native-body-faults/<family>/<label>-<UTC>-<PID>/
```

The complete output plan is validated before effects. An exclusive directory creation reserves
the new run;an existing directory refuses before any snapshot write. Receipt publication checks
the reserved device/inode identity and opens the report exclusively. It refuses an existing
report or replaced ownership. No replay writes into the earlier historical directory. Generated
snapshots,logs,controls and reports remain on the repository volume and under cleanup protection.

Actual sink controls admit one fresh receipt,refuse directory reuse,report replacement and a
foreign ownership identity,and preserve the guarded original bytes. Two changes to the actual
helper bodies deliberately remove exclusive directory creation or exclusive report creation;
the same controls reject both. These are real filesystem effects inside isolated control sinks.
Plan-only mode keeps its read-only/no-compiler contract. All nine wrapper-family plans pass.

A focused actual root replay passes the original85-row baseline and all five successfully
compiled body faults. Their first independent failure ordinals remain4,9,17,24,4. The run keeps
547 original input hashes,545 restored snapshot files and557 planned outputs;both owned process-
group timeout controls pass. All nine historical reports retain their original SHA256 values;
all4882 files across those historical snapshots match their original receipt hashes.

```text
target/irrational_platform_certificate/native-body-faults/root_body_faults/0632-exact-root-20261030T181144027386Z-78668/report.json
sha256:c9403edbcbf21eb79cb574c34823a965bfa6f24d1340d714b0bd25467d7f3065
```

The existing root/arc/forward/inverse/tangent/resource commands and formula-structure suite use
this shared behavior automatically. Incomplete runs stay separate;they cannot replace a complete
receipt. The0642 focused repair preceded a complete48-fault replay. The later0654 full gate is
recorded below;complete transitive runtime/locality review retains its separate owner.


## Coupled verification checkpoint

The0646 full verification attempt preserves separate results for each scope:

| Check | Observed result |
| --- | --- |
| Full Rust gate | Formatting,strict workspace Clippy,1635 native and372 documentation tests pass |
| Wasm foundations | sc-units,sc-core and sc-measure compile for wasm32-unknown-unknown |
| Doctrine gate | All13 checks pass |
| Fresh numerical body faults | Nine baselines pass;all48 successfully compiled faults are rejected |
| Fresh observed runtime pair | All31 selected native/Chrome checks pass;compiled bad expectation fails both |
| Full probe command | All29 suites attempted;overall Make exit2 because verification-tool checks fail |

The fresh runtime capture uses Rust1.99.0/b940084d7 and Chrome156.0.8078.74 on Darwin27 arm64.
Its405 original fixture rows and550 source hashes remain exact. All6614 files in the legacy
native roots,including the recovered logs,and all nine earlier native reports remain unchanged.

D349 was a copied-fixture defect:HTML copies omitted the generated search index,so an intended
missing-source-link case hit SEARCH_INDEX first. The repaired fixture plans/copies that index
and verifies each valid copied baseline before mutation. D351's historical AST/count control
now admits the exact approved search additions;all earlier behavior remains checked. D350 is a
stale independent selector expectation:the real recipe producer correctly checks24 inputs,the
old control expected23 and omitted the newly linked formula-recipe-execution.md chapter. Its
repaired population retains exact set equality;an actual producer fault omitting only that
chapter is rejected. The publication refusal families and all source/alias guards remain exact.

The full transcript,individual numerical captures and successful gate logs are linked by:

```text
target/irrational_platform_certificate/coupled-0646-baseline-20261030T202107306619Z-42055/receipt.json
sha256:f31003fadb278e69b2d9544e124118ccc0538d9204be1d686cc822b655886b4b
```

That0646 checkpoint left scoped acceptance open;the complete milestone and final review below
record its resolution. Hosted CI,application and production-release approval remain separate.

After those focused repairs,the0649 environment run passes the producer,recipe and binding/replay
controls but stops at a later origin control. D352 retained the old1466 count and pre0581 AST,
whereas approved input-requirement cases now produce1471. Runtime tracing distinguishes the
earlier1466 result calls from exactly five added external-input cases;provenance remains425.
This environment attempt exits1 with all snapshotted source hashes unchanged. The subsequent
full probe command does not start. The D352 focused repair now checks the exact approved0581
origin contract/fault definitions and1471 calls;unaffected origin/provenance bodies remain checked.
Two actual preservation changes refuse. Complete environment/full-probe verification remains open.

```text
target/irrational_platform_certificate/coupled-0649-integration-20261030T204352575454Z-95261/environment.json
sha256:941a68045fc0c853290fc6e7ad506e1c0f92587e4abfa502ca5ce0f95a85fb99
```

The0651 retry passes the repaired origin control and later numeric,retention and cleanup-entry
checks. It stops at D353:the cleanup-fixture companion pins revision0315 while the inspector
exactly matches reviewed0645. The intervening delta adds protected numerical/legacy roots and
retained-proof family guards. Its exact baseline needs an explicit review update. D354 also
reproduces in the optional continuity mode,which reads raw records now stored in logical
archives and assumes an obsolete task/frontier population. The focused repair now pins the
exact reviewed0645 inspector and uses public logical reads,three exact0224 records,immutable
prior bytes/order and the current linked canonical frontier. Twelve independent refusals and
four compiled guard omissions pass;the default control watches continuity as well.
This attempt exits1 with1220 snapshotted sources unchanged;full probes again do not start.

```text
target/irrational_platform_certificate/coupled-0651-integration-20261030T210702634267Z-10812/environment.json
sha256:3986118ee5f11d8d9f9916d2279456d9292cc45f73748bcc0e0973593bd73c54
```

The0653 preflight then passes all twelve previously blocked environment commands in canonical
order:ledger fixture/mutations,publication fixture/builder,map/handoff fixture,archive client/
reader/full fixture,static loader,payload library,native loader and push cadence. All1222
snapshotted source hashes remain unchanged. The retained full Rust gate still has an exact
548-input Rust/build basis:1635 native and372 documentation tests,formatting and strict Clippy.
A fresh complete29-suite make probes command remains the next verification step.

```text
target/irrational_platform_certificate/coupled-0653-tail-20261030T213709318829Z-87999/receipt.json
sha256:b116a422e29b0d8668cda662a85a93b91b7fb6193864d318e89c9d15e7ef1b4e
```

## Complete local probe milestone

The0654 actual scripts/run_make.sh probes command finishes with all29 suites green and exit0.
All1223 snapshotted project inputs are unchanged. This complete run includes the repaired
environment controls,publication50 and fresh numerical replays. Nine native baselines pass;
all48 successfully compiled arithmetic/resource faults are rejected at their independent
fixture assertions. All4905 copied snapshot files are restored byte-for-byte.

The fresh observed runtime pair passes the same31 public-consumer checks on native Rust and
actual Chrome. The compiled bad expectation returns101 natively and traps during browser
execution. All550 original sources,405 fixture rows and four retained binaries are exact;
workspace restoration and zero new ambient Chrome temporary entries pass. Runtime identity
remains Rust1.99.0/b940084d7 and Chrome156.0.8078.74 on Darwin27 arm64.

Independent preservation checks retain all6614 original legacy files and nine earlier native
reports unchanged. Both original certificate manifests and all19 retained0641 verification
copies are exact. The548-input Rust/build basis still matches the retained full gate with1635
native and372 documentation tests,formatting and strict Clippy. Wasm foundation and doctrine
evidence retain their earlier scope;the commit-integrity gates also check each documentation update.

```text
target/irrational_platform_certificate/coupled-0654-probes-20261030T214824664186Z-24178/receipt.json
sha256:037ded5a5d07de1fe46373acd4b95b1dbd4a869dc3fc188b0675f17f83430376
target/irrational_platform_certificate/coupled-0654-probes-20261030T214824664186Z-24178/preservation.json
sha256:1abf1475eb721a16c7a11205a931308366223bf5eb8ef294d97a71bf7f23f71a
```

The full verification result supports the [completed scoped numerical review](formula-irrational-review.md#scoped-acceptance-and-remaining-owners).
Application/device coverage,complete allocator/provider/replay/geometry integration,physical
validation and human production-release approval keep their separate owners. Hosted CI remains
the previously observed SHA-scoped result until the authorized push cadence is reached.

## Read-only numerical snapshot inventory

The shared root/arc/forward/inverse controller inventories this repository's tracked crate files
and root workspace manifests before planning its exact copied workspace. Its Git child discards
unrelated Git selectors,disables optional index locks and reads no system/global configuration.
The caller's local build stores,toolchain selection and other environment values stay intact.
An unrelated GIT_DIR therefore cannot redirect this inventory to another repository.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/root_body_faults.py --plan-only
python3 -I -B docs/tasks/artifacts/formula_structure/arc_body_faults.py --plan-only
python3 -I -B docs/tasks/artifacts/formula_structure/forward_body_faults.py --plan-only
python3 -I -B docs/tasks/artifacts/formula_structure/inverse_body_faults.py --plan-only
```

These commands validate source and destination plans and print classifier controls without
creating directories,writing snapshots or dispatching a compiler. The focused watcher runs the
actual four entries with unrelated Git selectors and rejects six actual inventory-body faults.
The separate effect and fresh native results below complete this four-entry boundary. Remaining
n16/platform,fixture-wrapper and broader integration scopes retain their owners;no old receipt is overwritten.

## Controller effects and failure boundaries

The shared root/arc/forward/inverse controller validates the complete source,snapshot,log and
receipt plan before activation. The selected kernel and fixture must belong to that copied
workspace;an unrelated supplied file cannot become an unplanned mutation target. Baseline and
fault labels must be distinct. Mutation and restoration explicitly declare the copied body as
both an input and a mutable output. Original production sources remain read-only.

Each Cargo dispatch uses the copied manifest,offline/locked dependencies,the original named test
and prepared local stores. Published Cargo artifact paths undergo source validation before hashing.
Child processes own separate sessions;timeout or interruption stops the owned group before copied
body restoration. Compilation failure and a surviving fault cannot count as numerical rejection.
A late log refusal,linked mutable body or foreign artifact path cannot publish acceptance.

The focused runtime watcher exercises all four entry configurations with only their diagnostic
output labels changed. Its thirty simulated variants test dispatch and copied-body effects;
retained reports explicitly say SIMULATED and supply no compiled numerical evidence. Seven
failure paths,twelve preflight refusals and eight actual Python-body faults calibrate the checks.
Device and Git-marker refusals use controlled observations;source/output aliases and types use
owned fixtures. Actual parent/child timeout fixtures verify that killing a parent alone permits a
late child write,while group cleanup prevents it. Original inputs and complete copied workspaces
are checked after success and failure;the watcher also cleans its intentionally broken restoration.

These controls close the scoped effect review. Separate fresh compiled-native acceptance now
passes below;complete n16/platform and broader D156 integration remain owned.
The fixture wrapper's own complete producer audit is separately scheduled at a41;its scoped
controller checks do not close that transitive boundary.

## Fresh native controller acceptance

The0659 run executes all four original public commands sequentially after the inventory and
effect repairs. Actual Rust1.99.0 (b940084d7),LLVM23.1.1,on aarch64-apple-darwin passes four
baselines and rejects all26 successfully compiled kernel faults:root5,arc5,forward7,inverse9.
Every rejection retains an actual watched-test panic and a complete independent fixture row.
Compiler failure never counts as a numerical fault rejection.

All551 recorded input files remain byte-exact. All four545-file copied workspaces restore
completely,2180 snapshot files in total;the6614 legacy evidence files remain unchanged during
the run. The548 Rust/build inputs still match the retained full Rust gate's source basis.
Actual timeout-group and exclusive-capture controls also pass. An independent reader checks
report/log identities,compiled outcomes,original rows and complete restored file populations.

```text
target/irrational_platform_certificate/native-0659-20261031T003645962930Z-64800/receipt.json
sha256:0fee096d24b88963e434182710341cc65fd33d86c2e59118ba017d65311dfeb5
```

This closes n16.1's supported four-entry inventory/effect/native boundary on the observed host.
Tangent/resource/browser acceptance passes below;the fixture wrapper remains a41,and complete D156
integration retains its owner. No new full-suite,application/device,allocator,physical or human
release verdict follows from this native acceptance. Historical captures remain immutable.

## Platform inventory authority

The separate platform runner's plan-only entry also isolates Git selectors and system/global
configuration before reading this repository's index. It preserves the prepared local store
values and unrelated caller variables without activating stores. An unrelated GIT_DIR cannot
redirect its source inventory. NUL-separated Git output retains the existing filename boundary.

The read-only watcher runs the actual entry,retains all nine consumers,31 selected checks,six
host-only exclusions and405 fixture rows,and rejects seven actual inventory-body faults. It
permits no directory creation,file write,compiler or browser dispatch. The original four native
plan-only entries and their six body controls remain green. Ownership has separate controls below;
the scoped effect controls below pass;fresh execution remains the following n16.2 leaf.

## Exclusive platform capture ownership

The platform runner resets stale ownership before preflight. After validating its complete plan,
it reuses only the common store parent;run and short temporary directories require exclusive
creation. A competing run never becomes owned by this invocation. A competing temporary root
remains untouched;the new owned run may retain its own incomplete-failure record.

Reports,failure records and pending latest pointers use exclusive file creation. A prior pending
file cannot be overwritten;the existing validated latest pointer still changes by atomic rename.
The temporary root's original device/inode identity must also survive until sealing.
Actual collision fixtures preserve competing bytes and perform no compiler/browser execution.
The read-only watcher independently rejects exclusive-directory,exclusive-file and stale-owner
reset omissions. These ownership controls are separate from the full-entry effect controls below
and the fresh native/browser acceptance below.

## Preflight at the platform child boundary

The compiler-command helper verifies the supplied effective local stores and log destination
before starting a child. The browser helper verifies its stores,WASM input,profile/cache/crash
and temporary directories,and HTML/DOM/log destinations before writing the generated page or
starting Chrome. Linked log destinations and external supplied stores refuse at this boundary.
Both helpers keep explicit child environments and separate process sessions;timeout or interruption
stops the owned group before returning control to restoration.

The focused dispatch watcher checks two simulated successes,eight physical log-alias/store
refusals,four simulated timeout/interruption cleanup paths and four actual Python-body omissions.
These controls prove helper ordering and dispatch behavior. Tangent configuration coverage and
full-entry controls pass below;fresh compiled native/browser acceptance remains a separate leaf. Earlier numerical captures retain their original scope and identity.

## Tangent controller configuration coverage

The controller effect watcher now observes both tangent wrappers through their actual public
entry dispatch. It captures all three arithmetic and both resource configurations with the original
source,fixture,test and fault sets. Only the owned diagnostic output labels change.
The focused run checks27 simulated variants,two failure paths and three actual wrapper-body
omissions,with complete original/copy restoration and actual owned timeout/group/capture controls.
Simulated compiler outcomes remain explicitly marked;fresh compiled acceptance is still separate.
The full-entry platform effect controls and fresh native/browser acceptance follow below.

## Full platform entry effect controls

The actual entry keeps MAKE_TMPDIR aligned with its selected owned browser temporary directory,
so helper verification sees one consistent store profile. The copied bad-expectation fixture is
explicitly admitted as a mutable input before reading,mutation and restoration. Original project
inputs remain read-only and must retain their exact digests.

The full-entry watcher uses modeled compiler/browser children and labels its receipts accordingly.
It checks complete plans before activation,caller stores,child arguments/tool identities,exact
copy restoration,temporary-root identity and sealing,and atomic latest publication. Six failure
paths cover compilation failure,a surviving bad expectation,interruption,version drift,temporary
root replacement and a linked late log destination. Six actual Python-body omissions calibrate
its checks. No failed path can publish acceptance;the linked-log case refuses before activation.

This completes the scoped tangent/platform effects review. Fresh actual tangent/resource kernel
faults and fresh native/Chrome platform execution pass below. Complete transitive compiler/
browser locality and the full D156 integration scope retain their separate owners.

## Fresh tangent and resource native acceptance

The0665 capture runs both original tangent wrappers after the effect repairs. All five baselines
pass,and all22 successfully compiled faults are rejected against their original independent rows:
three folding,three interval,three execution,six preparation-resource and seven execution-resource
faults. Compilation failure never counts as a numerical rejection.

The independent read-only reader checks exact source and report membership,Cargo success,
watched-test panics,complete original rows and all copied file names/bytes. All550 original inputs,
6614 legacy evidence files and2725 copied snapshot files remain exact. Eight deliberately defective
reports are refused. Actual owned process-group and exclusive-capture controls also pass.

Fresh tests from the restored original sources pass all three tangent and two resource checks.
Both resulting native binaries are retained byte-exact. The observed compiler remains Rust1.99.0,
b940084d7,LLVM23.1.1,on aarch64-apple-darwin. This native acceptance supplies no fresh Chrome,
full-suite,application,physical or release verdict.

```text
target/irrational_platform_certificate/acceptance-0665-21696/native-receipt.json
sha256:4dd2880262f84d41564940b62ba73b0d9d8fe6e9246f0ce1322a4389f3be8297
```

The reader is reproducible with python3 -I -B
followed by docs/tasks/artifacts/formula_structure/numerical_acceptance_reader.py and the owned
acceptance directory above. Add --self-test to exercise the defective-report controls without
changing any capture. The fresh actual platform native/Chrome acceptance follows below.

## Fresh actual platform acceptance after effect repairs

The0666 actual capture again completes all31 original checks natively and in installed Chrome
WebAssembly. Both wrong-expectation builds compile;native execution then returns101 and actual
Chrome traps during execution with WebAssembly.RuntimeError. Baseline native execution returns104
and Chrome reports the complete104 marker with no imports. A compile/protocol error cannot count.

The independent --platform reader verifies both retained DOM results,four retained native/Wasm
binary digests,all original consumer assertion bodies,the complete copied consumer/fixture set,
version logs,temporary sealing and the OS-visible handoff result. All six fixtures restore;both
the native and platform550-input manifests remain exact,alongside the6614 legacy evidence files
and2725 restored native snapshot files. Eight deliberately defective platform reports are refused,
in addition to the eight native report controls. The earlier native receipt stays byte-exact.

Observed versions remain Rust1.99.0,b940084d7,LLVM23.1.1,and Chrome156.0.8078.74,on Darwin27.0.0
arm64. No new ambient Chrome temporary entries appear in the capture's bounded census.

```text
target/irrational_platform_certificate/0640-20261031T013514679084Z-88934/report.json
sha256:36bf632ede7579160c2cd88838949ef371525e7ec882f9eca52599bc1f1cad6f

target/irrational_platform_certificate/acceptance-0665-21696/receipt.json
sha256:6ec359292fac03fffccdd994ea171c838d0375d8865250f7ea0f3a9aa8a306b2
```

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/numerical_acceptance_reader.py --platform --self-test target/irrational_platform_certificate/acceptance-0665-21696
```

Without --platform,the reader validates native evidence only and explicitly excludes platform
acceptance from its scope. These results close n16's supported direct entry/source/artifact/
profile/temp/child boundary on this host. The fixture wrapper's own producer audit remains a41;
complete transitive compiler/browser locality,full D156 integration and broader application,
physical and human release acceptance retain their separate owners. No fresh full-suite verdict
follows from these focused commands.

## Fixture input admission before setup

The numerical effect fixture now validates its core inputs and planned work root before its
read-only Git inventory. Metadata receives prepared local store values without creating them,
with Git selectors and system/global configuration isolated. Complete snapshot inputs are then
admitted and frozen before any store activation or fixture-owned directory creation. Caller
store values and unrelated variables survive. A controlled late Cargo.lock refusal causes no
owned setup or caller-environment change.

The startup watcher exercises real valid activation,that late refusal,and three actual Python-body
omissions. The existing five-configuration tangent effect consumer remains green. This closes
only the fixture's startup child;the remaining fixture-own file/lifecycle review remains a41.effects.

An optional macOS diagnostic pins the selected Git executable and denies persistent file writes,
with the required OS null-device sink exception. It reads the observed542 index rows and proves
that an actual file-creation control is denied. Inside this harness,Git can warn about Darwin
confstr fallback when a planned temporary directory does not yet exist. The OS-visible enforced
run succeeds without creating that directory;it does not independently reproduce the warning
branch or certify every Git implementation. No compiler or browser is run by these controls.

```bash
python3 -I -B docs/tasks/artifacts/ci_environment/numerical_fixture_startup_contract.py
# Optional macOS diagnostic;requires OS-visible sandbox/process access:
python3 -I -B docs/tasks/artifacts/ci_environment/numerical_fixture_startup_contract.py --macos-readonly-git
```

## Exclusive fixture work roots

Each numerical effect fixture invocation now derives a UTC/PID work name,validates its common
parent and work paths,freezes all inputs,then creates the common parent and reserves the work
root exclusively before activating caller stores. An existing root refuses without changing prior
artifact bytes,creating caller stores or changing the caller environment. Two ordinary bootstraps
receive distinct fresh roots,so a recycled process identity cannot silently adopt an old workspace.

The controls reproduce the old overwrite only inside an isolated controlled fixture. Three
startup and three ownership omissions are rejected. The complete original four-family and
five-configuration tangent replay retains all30+27 simulated variants,original failure/refusal
controls,actual owned timeout/group checks and complete source/copy restoration. These outcomes
remain explicitly simulated compiler evidence;actual numerical/Chrome proof inputs are unchanged.

One historical modeled receipt sample records547 original controller inputs while omitting the
fixture emitter's own identity. The separate provenance repair below closes that omission before
fixture signoff. The historical receipt remains untouched;this gap does not alter the separately
observed actual native/Chrome verdicts.

## Modeled receipt emitter identity

New modeled receipts additionally retain fixture_instrument with the repository-relative fixture
path,frozen source SHA256,Python version and implementation name. The producer verifies that its
source file still matches the frozen basis before publishing the exclusive receipt. A changed
basis cannot publish acceptance. These fields identify the model's emitter;the existing original
controller-input map continues to describe its own inputs.

The focused control checks actual publication,four wrong identity fields,a controlled in-memory
basis change and two actual Python-body omissions. Source files stay unchanged. Older receipts
remain immutable and retain their historical scope. Every modeled receipt still explicitly says
SIMULATED and supplies no compiled numerical or browser verdict.

```bash
python3 -I -B docs/tasks/artifacts/ci_environment/numerical_fixture_provenance_contract.py
```

The remaining a41 signoff owns complete fixture-own alias/lifecycle acceptance and the current
full modeled replay. Actual numerical/Chrome acceptance remains the separate observed proof above.
