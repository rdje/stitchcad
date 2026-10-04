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
| `push-due` | Report the exceptional-push/cadence verdict |
| `help` | Print the target list |

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
regular files within the current workspace. Parent escapes, links, Git boundaries and foreign
volumes refuse. Invalid path types or control bytes also refuse. An invalid entry prints the
local-environment refusal and exits2 before invoking a child. Named toolchains and unrelated
environment values are preserved; paths still derive from the current repository root.

The individual-Ease and Ease-set mutation entries use the same declared-path and profile guard.
Their native runs with caller overrides absent observe seven and ten real assertion failures,
then restore exact source and rebuild each current artifact. Mutation diagnostics run sequentially.

Measurement-table, garment-observation, chart-collection and MTM mutation entries also use
the declared-path guard before output or child dispatch. Each restores its exact source and
runs its complete focused test target again to rebuild the current artifact before returning.

Standing controls cover17 runtime cases and thirteen actual body faults, including seven real
entry captures at their first write. The membership native run with caller exports absent observes seven compiled
assertion failures, then restores exact source bytes and rebuilds the current artifact with all
12 membership tests passing. All seven entry captures also refuse a late missing source before
output writes or child dispatch. Other Python native, fixture and delegated entries remain under the
bounded .h2.b.p audit. The original pre-repair capture is retained in the census record; current
captures check the repaired entry. This scoped repair does not close D156.
