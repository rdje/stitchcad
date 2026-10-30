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
runtime-locality review remains owned by n16.

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
