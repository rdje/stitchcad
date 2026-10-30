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
Clippy and formatting. Full coupled acceptance and the fresh complete inverse-certificate read
remain next;aggregate resources,provider/replay,geometry,physical validation and human release
approval retain their separate owners.
