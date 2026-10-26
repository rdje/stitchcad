# Annex: guarded artifact cleanup

Cleanup removes regenerable, ignored project outputs using a frozen plan. It preserves tracked
inputs, package stores, current dependency binaries, scaffold backups and audit records. The
[producer profile](../build-and-checks.md#local-producer-profile) keeps stores, scratch and output
on the repository volume; all persisted candidate paths are relative to the current repository.

## Plan, review, then apply

Choose a fresh run name under target/artifact_cleanup_audit. Both modes require exactly the
mode and run directory shown below. A plan freezes the producer hash, Git HEAD, complete tracked
content hash, candidate fingerprints and skipped paths. Each candidate records its relative path,
file/tree kind, file count, byte count and SHA-256 fingerprint.

```bash
python3 -I -B docs/tasks/artifacts/artifact_cleanup/cleanup.py plan target/artifact_cleanup_audit/daily-example
# Inspect plan.json in that run directory before applying it.
python3 -I -B docs/tasks/artifacts/artifact_cleanup/cleanup.py apply target/artifact_cleanup_audit/daily-example
```

The example name is illustrative: use a new name for each actual run. Plan and result JSON files
are created exclusively, with sorted keys, two-space indentation and one terminal newline. An
existing plan is never overwritten. An already applied run refuses. Any change to HEAD, producer,
tracked content, candidates or skipped entries refuses with plan drift; create a fresh plan.

The entry validates its own source, local profile and strict process checker sources before use.
It checks the selected audit directory and plan/result file before metadata delegation, then
validates every Git-listed tracked input before creating stores or audit directories. Apply also
checks plan.json as a read-only input. All Git/process children receive the local environment;
valid prepared stores and unrelated values survive. Read-only tracked inputs may have hard links.

Every audit write has a fresh exact-file plan. Result aliases, hard links, directories or invalid
parents refuse before deletion. Borrowed save(path, value) validates its selected output under
target before its exclusive write. Borrowed apply(directory) validates the same audit authority
as the CLI; its directory must be under target/artifact_cleanup_audit with one run component.

## What can be selected

The census checks doctrine/scratch/tmp, Rust incremental and rendered-book trees, plus ignored
.log, .bin, .tmp, .orig, .rej and .DS_Store strays under target. Protected stores and dependency
binaries are pruned. Retained numerical proof stores target/trig_forward_certificate/ and
target/inverse_threshold_certificate/ are also protected, including their .bin payloads,
checkpoints, frozen sources and executable. Generic cleanup cannot retire those records;
any later retirement needs its own task and evidence. Unsafe entries appear in skipped with a reason; they are never followed or
removed. Nested Git markers, symlinks, foreign devices, special files, nonignored paths and tracked
descendants refuse. A regular deletion file must have exactly one hard link. Directory link counts
retain their normal semantics; a linked read-only input is distinct from a deletion candidate.

Fingerprinting reads bounded chunks and includes each path, kind, file size and content digest.
Apply rechecks the complete frozen census, then each candidate immediately before removal. A
current tracked-content digest must still agree afterward. It verifies all selected paths absent
and records removed tree/stray/file/byte totals, zero residue, unchanged tracked inputs and skips.
An error stops the operation; it supplies no rollback for artifacts already removed.

## Process evidence is required

Before deleting anything, apply runs the project-owned scripts/check_handoff.sh. The canonical
checker requires successful, nonempty, structurally complete ps/lsof evidence and caller ancestry.
An unavailable or malformed census refuses2; a live project job blocks1. Both statuses abort
cleanup before removal. A complete clear census permits cleanup. The inherited neutral checker
is retained as provenance and is not used to authorize deletion.

If OS evidence is unavailable, obtain the necessary OS-visible execution access and rerun; do
not treat an empty census as proof of idleness. The checker uses its default strict mode and does
not infer that CUA is idle. Cleanup's original plan/apply argument interface is unchanged.

After a real run, inspect selected-path residue without activating the producer launcher, which
would recreate scratch. Update the single latest cadence entry in docs/ARTIFACT_CLEANUP.md and
commit its owned task. Rebuild native/WASM/book outputs and run the appropriate controls only
after the immediate residue proof. All roots and examples remain relocatable.

### Incomplete runtime observations

SPINE.23r captured a recurrence: lsof emitted descriptor4 and a name without the requested type.
Sanitized shape/hash evidence and the independent missing-type fixture reproduce the refusal2;
a new actual type-guard fault requires an assertion red. No unreadable handle is exempted.
A fresh full census then passed0. That proves its own snapshot, not the refused one. The earlier
unretained failure remains unexplained; no kernel cause or scheduling race is inferred.

lsof may omit unavailable fields, as its field-output manual explains. On Darwin, Apple's
[process_vnode implementation](https://github.com/apple-oss-distributions/lsof/blob/main/lsof/dialects/darwin/libproc/dfile.c)
prints the `(revoked)` marker after a vnode-information lookup returns ENOENT. Its own comment
warns that this error does not always establish revocation. The marker cannot certify that a
descriptor is harmless. See also the upstream [field-output contract](https://github.com/lsof-org/lsof/blob/master/Lsof.8).

The checker distinguishes missing metadata from malformed framing. For example:

```text
handoff: REFUSED — lsof type unavailable for PID 8048 FD 10 (revoked); handle identity unresolved
```

This remains exit2. PID/FD identify the observation; raw handle names are withheld, apart from
the exact producer marker. Missing process/file fields or duplicate names still report malformed
framing. Caller ancestry and runtime process names do not exempt unresolved metadata.

D217 reproduced this in the Codex daemon, session and code-mode host. Independent read-only
libproc queries also returned ENOENT; one owner was outside caller ancestry. The underlying kernel
cause remains unproved. The installed OS tools and libproc are required read-only host dependencies;
diagnostic outputs stay under the repository's protected audit directory.

Restarting the affected Codex runtime outside the active harness is a recovery experiment, not
a proved root-cause repair. Do not terminate the harness from inside itself. After recovery verify:

```bash
bash scripts/check_handoff.sh --idle-cua
python3 -I -B docs/tasks/artifacts/handoff/contract.py --real
```

The first command needs complete OS-visible evidence and no pending CUA call/result. The second
must observe clear0, a controlled real held-file blocker1, then restored clear0. Only then create
a fresh cleanup plan: HEAD, tracked inputs or candidates may have changed during diagnosis.
Until then cleanup removes nothing and the last successful cadence date stays unchanged.

### D217 investigation and discriminating tests

The tracked diagnostic is docs/tasks/artifacts/handoff/d217_probe.py. Snapshot mode compares
descriptor enumeration, both native vnode metadata queries and lsof without publishing raw paths.
Controls use newly owned files/directories and an unattached terminal; existing descriptors are
never revoked or closed. Calibration deliberately removes each reproducing mechanism and requires
an assertion failure. Pure observation controls and exact committed-record retention run with the
normal handoff probes; native calibration also runs before the real handoff baseline.

| Candidate explanation | Experiment | Observation on 2026-10-25 |
| --- | --- | --- |
| lsof alone invents the failure | Query both native vnode flavors directly | Both return ENOENT for the affected descriptor; neighboring metadata succeeds |
| Ordinary unlink or descriptor close explains it | Hold ordinary files/directories across deletion; compare a closed descriptor | Ordinary metadata stays readable; close returns EBADF |
| Terminal revocation | Revoke only a newly owned unattached terminal | Both native calls return ENOENT; lsof emits the exact untyped marker |
| Removed event-only directory | Remove only a newly owned directory held for notifications | Same native/lsof signature, from a different mechanism |
| Every fresh installed runtime starts broken | Initialize an isolated installed app-server, without a thread or model request | Clean and live-directory controls are readable; deliberately inherited removed-directory descriptor reproduces |

These tests establish mechanisms, not the historical identity or destruction event for the original
descriptor10. Matching signatures cannot choose between those mechanisms. The observed runtime is
Codex0.164.0 on Darwin27.0.0, kernel13432.1.9; that exact kernel source tag was unavailable during
this investigation. Apple's published source explains the API/marker paths, without certifying the
installed kernel's exact failing instruction: [native vnode queries](https://github.com/apple-oss-distributions/xnu/blob/main/bsd/kern/proc_info.c).
The process lists remain stable during the production
capture; no whole-process permission denial was observed. An equal enumeration cannot prove that
an individual descriptor was never reused.

The 2026-10-25 recovery experiment replaced app-server65756 with66024, whose vnode metadata was
readable, but retained updater8048 and its unresolved descriptor. The release-matched
[restart implementation](https://github.com/openai/codex/blob/rust-v0.164.0/codex-rs/app-server-daemon/src/lib.rs)
keeps an enabled updater; restarting the app server alone does not reset its inherited descriptors.
After checking executable, start time, PID record, caller ancestry and children, a single SIGTERM
stopped that separate updater gracefully. Running the installed daemon start command with
close_fds=True replaced it with42768 without replacing active app-server66024. The
[updater termination handler](https://github.com/openai/codex/blob/rust-v0.164.0/codex-rs/app-server-daemon/src/update_loop.rs)
returns on SIGTERM. Both replacement processes supplied readable vnode metadata. The full strict
census and real clear0/held-file1/restored0 passed; cleanup then applied with zero selected residue
and unchanged tracked bytes. Protected startup/recovery-20261025 audit records retain the receipts.

This proved operational recovery at0235. The historical descriptor's identity and invalidation
event remain unproved. The later investigation below establishes and repairs a reproducible
inheritance class; it does not identify the old object. The checker remains strict.
Root owner: cleanup4.hunt in the producer task tree. Never signal a PID from an old receipt without
fresh identity and ancestry checks, and never terminate the active harness from inside itself.

```bash
python3 -I -B docs/tasks/artifacts/handoff/d217_probe.py snapshot --pid 1234
python3 -I -B docs/tasks/artifacts/handoff/d217_probe.py calibrate
python3 -I -B docs/tasks/artifacts/handoff/d217_probe.py runtime --binary /path/to/installed/codex --inherit removed-event-directory
```

The PID and executable path are examples; select the observed process and installed executable.
Runtime mode uses a fresh project-owned state directory and a separate process group. It closes
unselected inherited descriptors, sends only initialization, and verifies its child exits and its
process group disappears. A successful diagnostic is not permission for cleanup deletion.
Isolation follows the documented [Codex state location](https://learn.chatgpt.com/docs/config-file/config-advanced#config-and-state-locations)
and [app-server initialization](https://learn.chatgpt.com/docs/app-server#protocol); no inference is requested.

### Reproduced launch defect and tested local candidate

The 0236 investigation uses the actual installed managed-daemon launcher in private state.
Native device/inode and descriptor flags identify the same controlled directory in both detached
processes before deletion. The close-on-exec flag is unset. Removing the directory then produces
the exact unavailable native/lsof metadata; restarting the app server retains the updater's fault.
Already removed directories and freshly owned revoked terminals also propagate when supplied at
launch. Closing unrelated descriptors or setting their close-on-exec flag prevents this chain.
This establishes a reproducible inheritance defect; the old descriptor10's historical object and
event remain unknown. No lsof or kernel-bug claim is needed to establish this launcher behavior.

The [release-matched detached spawn](https://github.com/openai/codex/blob/rust-v0.164.0/codex-rs/app-server-daemon/src/backend/pid_start.rs)
starts a new session without filtering unrelated inherited descriptors. The tracked patch
docs/tasks/artifacts/handoff/d217_daemon_fix.patch sets their close-on-exec flags in the forked
macOS child, preserving standard streams, the parent's flags and Rust's launch-error channel.
It uses the [documented child execution approach](https://docs.rs/close_fds/0.3.2/close_fds/).
The patch changes two source inputs in revision6aaac694803004a63c6f393d153c96d22521a6b9;
9434 other release files remain exact. The generated dependency lock is retained separately;
existing registry versions/checksums stay fixed, with only the pinned descriptor helper added.

The optimized CLI prevents all three fault cases and preserves both clear controls in the same
five-case installed-daemon experiment. Three compiled regression tests cover inheritance, parent
preservation, standard streams and exec-error reporting. Omitting the actual guard fails its
identity assertion; restoration passes. All88 daemon unit tests pass with the shorter project-local
temporary directory target/t, avoiding the initial fixture socket pathname-limit failures.
Pure observation/patch-selector controls run with the ordinary handoff probes. Native controls
require Darwin and the selected installed or locally built executable; no CI-platform equivalence
is inferred. The new private tools never request a model turn.

```bash
python3 -I -B docs/tasks/artifacts/handoff/d217_daemon.py --binary /path/to/codex
python3 -I -B docs/tasks/artifacts/handoff/d217_daemon.py --binary /path/to/patched/codex --expect-contained
python3 -I -B docs/tasks/artifacts/handoff/d217_upstream.py verify
python3 -I -B docs/tasks/artifacts/handoff/d217_upstream.py calibrate
```

The retained source archive, optimized candidate and build receipts are under
target/artifact_cleanup_audit/d217-upstream-0164; native verdicts are in recovery-20261025.
The complete package preserves its matching companion tools/resources, replaces the CLI and
updates the CLI's voice-resource checksum. Its sidecar records the local patch and file hashes.
Supported native installation and exact original-version rollback pass in a fresh private home;
the tested source selection is a separate local release and never overwrites the vendor release.

**Activation changes the shared Codex daemon and pins production automatic updates.** The
director completed this on2026-10-25 from a standalone Terminal. For a vendor installation,
the script rechecks package hashes, refuses active-harness ancestry and preserves the
native installer's interactive confirmation. All Codex sessions using that daemon are interrupted.

```bash
python3 -I -B docs/tasks/artifacts/handoff/d217_upstream.py activate
```

The installed CLI matches the tested SHA256
3fe68efdbc14e99cbc614b2615514eaddf7520277696d7aee2eeead56dd2cd79.
All41 package file hashes match; daemon94696 and code-mode host97349 run from the local release.
A complete native capture finds51 readable daemon vnodes and zero host vnodes; both metadata
flavors and lsof are clear. The first changing enumeration is retained separately. No updater
PID or update marker remains; the original vendor binary is unchanged. After a separate DiskWatch
scan finishes, the real handoff control passes clear0/held-file1/restored0. D217 closes for this
verified inheritance repair; historical descriptor identity remains unknown. Cleanup is complete.
For exact-version rollback, the original installed CLI's daemon update --from-cli command selects
the original package through the same native installer; that selection also stays pinned. A normal
daemon update command returns to the production update channel. The private rehearsal proves
package switching and byte-exact rollback, without claiming unobserved UI/model/voice sessions.

## Verification scope

Actual captures execute both mode bodies: creation, two exclusive JSON writes, process dispatch
and removal of isolated controlled artifacts. Independent omissions and unsafe physical audit,
input and store fixtures calibrate the plans. Original policy bodies/constants remain exact through
the declared adapters except the two explicitly admitted numerical certificate stores. The original
fifteen cleanup cases and eleven compiled guard-fault assertions retain their refusal families.

The watched certificate-store control uses fresh owned files and the actual read-only census.
Both proof stores refuse selection; ordinary .log and prepared scratch cleanup remain eligible.
Removing exactly the two new protections from the compiled producer makes the same assertion fail,
then the intact producer passes again. File/source hashes remain unchanged; project apply is never
called. Mirrored local stores exist before Git runs, including a same-volume TMPDIR; platform SDK
scratch caches are allowed inside that ordinary tree and startup warnings fail the control.

```bash
python3 -I -B docs/tasks/artifacts/artifact_cleanup/certificate_store_controls.py
```

The existing cleanup entry suite also runs this control and permits exactly the two declared
store additions in its original-policy comparator. Every other old body/constant stays constrained.

D210's real legacy checker returned success on unavailable lsof, and its actual apply body reached
a trapped deletion. The canonical checker refuses that evidence; actual blocked/clear process
fixtures test statuses1/0 separately. D211's multiply-linked candidate was admitted by the old
inspector; the new guard refuses and its omission fails an independent assertion. Linked tracked
inputs remain readable. Real project plan runs preserve every tracked byte and perform no deletion;
controlled process fixtures are not a certificate of an unobserved host's current process state.

```bash
python3 -I -B docs/tasks/artifacts/ci_environment/python_cleanup_entry_contract.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_cleanup_entry_contract.py --real
```

## Guarded cleanup fixtures

The fixture entry is docs/tasks/artifacts/artifact_cleanup/cleanup_probes.py. Loading it runs the
original fifteen cases and eleven in-memory guard faults. Run one real fixture consumer at a time:
its original base is target/scratch/artifact_cleanup_probes, and each selected child must be absent.
A successful run leaves that base empty and preserves the inspector and all six source inputs.

Before the first helper executes, bounded metadata checks require a local regular profile file,
a safe scripts parent and no Git marker there. The fixture entry derives its root from itself,
then checks its own source, inspector, profile, canonical checker/census and README. It plans all
fifteen possible fixture directories, their nested directories and four possible file names before
activating stores or creating the base. Each case rechecks its selected complete plan before
creation; source-text fault compilation and the valid fixture's second write receive fresh checks.
Prepared local stores and unrelated caller variables survive activation and every Git dispatch.

The symlinks, FIFO and empty .git markers are deliberately unsafe test data inside newly owned
fixtures. Teardown checks the selected base and owned root before each marker removal, then checks
the exact tree before safe recursive removal. It never initializes a repository. A nonempty marker
stops teardown before the containing tree is removed. An unowned or aliased root refuses deletion.
Setup and the first valid-case write run inside protected teardown scopes: callback or write failures
remove owned test data when that data remains safe to remove. An unsafe root or failed marker removal
stops rather than broadening authority; it may leave that controlled fixture for explicit recovery.

Borrowed case(name, setup, operation, reason, mutation=None) retains its argument contract. A name
must select one direct child of the original base; escaping, absolute, nested and .git names refuse
before effects. The constructor covers keep, link, pipe and output.log plus the nested directory.
Callers own any additional callback actions. A normal borrowed case can use the same public inspector:

```python
import runpy
fixtures = runpy.run_path("docs/tasks/artifacts/artifact_cleanup/cleanup_probes.py")
fixtures["case"]("borrowed", lambda p: (p / "output.log").write_text("owned"),
    lambda m, p: m["validate_plan"]({"a": 1}, {"a": 2}), "plan drift")
```

D214 calibrated two original mutants: removing the symlink or ignore guard can expose a linked
read-only source to the newer deletion-link guard. Only those in-memory mutants isolate that later
guard. The real inspector retains it, with D211's separate actual admission/refusal/omission proof.
Linked read-only profile and README inputs pass all fifteen cases/eleven faults; hashes, aliases and
link counts remain unchanged. An isolation omission reproduces the original masked fault.

The watched fixture control executes the actual bodies and captures plans, activations, directory/
file/symlink/FIFO/marker/tree calls and prepared Git dispatch. Seventeen physical CLI refusals preserve
complete mirror snapshots. Independent plan/helper/ownership/restoration/isolation faults calibrate
the checks; a nonempty fake marker refuses before tree deletion. Those Git ignore dispatch fixtures
are modeled separately from real project consumers. Neither control path calls project cleanup apply.

```bash
python3 -I -B docs/tasks/artifacts/ci_environment/python_cleanup_fixture_contract.py
python3 -I -B docs/tasks/artifacts/ci_environment/python_cleanup_fixture_contract.py --real
```
