# Sealed archive — completed CI and cleanup protocols

Immutable historical segment, sealed by leaf `G1-SLICE.5f.3a.t1` on `2026-10-03` (UTC).

- **Sealed identity:** 118 lines, 8944 bytes, `sha256:532de5aed204cd230b48fffc7751129a58b9a6d734148ba13333ae8ef5dd72cf`
- **Coverage:** G1-0098 through G1-0101 protocols and receipts; original payload unchanged from dc8d7888bc46476d25b6f9ef1c0a96ca89de5e20.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## D142 pre-code protocol

  Work unit `STITCHCAD-G1-0098`; aaaf0fd clean, 0-byte untracked brief/no pending jobs.
  Relevant native/WASM roadmap/CI and exact namespace iterator/source/errors/public contracts,
  declaration annex/governance reviewed. Exact runner1.99 before/fix proof with root-derived
  target/cargo-home/rustup-ci, target/cargo-home and target/scratch; host toolchains read-only.
  No lint waiver, toolchain pin or grammar/API change; remote proof stays .h1.v.

## D142 acceptance checklist

### G1-SLICE.5b.3c.2b.h1.r1

- [x] **REPRODUCE / ISSUE** — exact1.99 root-local cargo clippy --all-targets --all-features --
  -D warnings→101 at namespace.rs:221 double_must_use; identical actual runner diagnostic. Local
  1.95/1.98 before-controls green; toolchain1.99 install/caches/TMP under protected target/, rc=0.
- [x] **ROOT CAUSE (WHY + WHERE)** — actual1.99 compiler identifies a bare function must_use on
  an Iterator result already carrying that obligation; official [Clippy double_must_use documentation](https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#double_must_use)
  agrees. Native source/scope diff proves one redundant attribute; exact compiler before probe101.
- [x] **FIX** — remove only that attribute. No lint allow, severity/channel change, new test or
  signature/body/kind/source/order/namespace behavior change. Existing public controls are the oracle.
- [x] **ADDRESSED (verified)** — exact1.99 make check:fmt and strict Clippy green,663 tests/
  56result groups passed, rc=0. Host rustc still1.95; isolated rustc1.99 matches the runner revision.
- [x] **NO REGRESSION** — exact1.99 make check→663 passed/56groups includes namespace/
  name-read/ordered scopes; cargo build --target wasm32-unknown-unknown -p sc-units -p sc-core
  -p sc-measure→3 libraries built; publication→10 pass/0fail, ledger→9/pointer13 pass, each rc=0.
  Final staged gate/hook checks follow; remote repair proof remains .h1.v.
- [x] **LOCKSTEP / RETENTION** — product annotation-only repair/current CI scope documented in
  governance/live/task/resume; D142 stays owned/open until actual repaired runner. Prior whole
  ledger/lesson retained exact and immutable windows unchanged. G1 stays5/18,14open/128sealed.
  Promotion: declined (existing exact-head CI and locality policy).

## D143 pre-code protocol

  Work unit `STITCHCAD-G1-0099`; d24f1a9 clean/brief0/no jobs.
  Published Cargo-home/Rustup env and toolchain installation contracts, existing workflows, locality
  doctrine/cleanup protected stores/governance reviewed. No shared cache deletion or pin.
  Root-derived Cargo/Rustup/target/scratch paths prepared before public CLI installation; observed CI
  verifies effective environment/root/device and actual compiler; strict jobs unchanged.
  Focused locality controls plus full exact1.99 check/gate/probes before exception push/.v.

## D143 acceptance checklist

### G1-SLICE.5b.3c.2b.h1.r2

- [x] **REPRODUCE / ISSUE** — exact prior CI job/log111242008842 shows a runner-home Cargo store;
  existing workflow supplied no override. Recorded API/log retrieval returns0; D143 remains open.
- [x] **ROOT CAUSE (WHY + WHERE)** — published Cargo/Rustup overrides and actual workflow/log
  agree: setup inherited shared defaults. scripts/ci_environment.py prepare/verify on actual root
  reports four local paths/same checkout volume, rc=0; independent literal expected catalog and
  runtime refusal/fault producer verifies19 controls/six compiled assertion reds, rc=0.
- [x] **FIX** — export root-derived paths before stable/profile/components/target installation via
  public Rustup --no-self-update, then verify effective environment, directory type/symlink/Git/
  device guards before fmt/Clippy/tests/WASM. Only GitHub's existing GITHUB_ENV protocol file
  outside checkout is written, under declared RUNNER_TEMP; no shared cache or launcher changed.
- [x] **ADDRESSED (verified)** — run_ci_environment_probes.sh→19 runtime controls/six actual
  compiled body assertion reds/workflow order/0fail, rc=0. Simulated device metadata is explicit;
  all actual fixture/root writes remain local. On-disk producer exact after all faults.
- [x] **NO REGRESSION** — exact1.99 make check→663/56 green0; make wasm→3 built0; full
  make probes→28 suites green0, publication10/ledger9+13 pass0; make gate→all green0; hook follows;
  Each invocation rc=0; repaired remote evidence belongs to .h1.v.
- [x] **LOCKSTEP / RETENTION** — complete prior86line/6837B protocols fromd24f1a9 retained in
  engineering-continuity record98, SHA741f466e90014631d80c559909345327a4ef6598e30ea80c95548e958046e902;
  same original headings now route from this live sibling. Whole oldest ledger/lesson retained;
  no archived window/reader/schema/cap change. Book/live/map/toolbox/task/resume reflect scoped CI
  changes; G1 stays5/18,14open/128sealed until .v observes repaired jobs/steps/effective paths.
  Promotion: declined (existing locality, exact retention and observed-CI policy).

## Repaired CI observation — .h1.v

### G1-SLICE.5b.3c.2b.h1.v

Work unit STITCHCAD-G1-0100; d5dd11f clean, brief0, full local gates/probes green before push.
Clean exceptional push→0; exact repaired-head runs queried by full SHA/event push, API→0.
Observe actual job/step verdicts, locality verification output and compiler/build paths; refuse
aggregate inference. D142/D143 close only after observed success. Cleanup due after this unit.

- [x] **ROOT CAUSE (WHY + WHERE)** — gh API exact-head jobs shows both completed/success;
  eight doctrine/eleven Rust steps completed/success, API0. Rust log→compiler1.99, strict lint,
  663passed/56groups/WASM3 and four effective root-derived stores, log retrieval/extraction rc=0.
- [x] **ADDRESSED (verified)** — actual remote verification confirms directory/device and
  environment identity after installation before all builds, rc=0; old redundant lint is absent.
  Doctrine run37157386833/job111303492309, Rust37157386892/job111303492427 at full
  d5dd11f3a5483b4d28f40feb58c86a8b3c148724. Each actual step succeeded; no aggregate inference.
- [x] **NO REGRESSION** — publication→10 pass/0fail; ledger→9 pass/pointer13 verdicts0fail;
  archive verify-retention→261 records/16working Markdown/11401lines/845172decodedB/375163residentB;
  all invocations rc=0; make gate→all doctrines green, rc=0. Exact source reports retained; hook follows.
- [x] **LOCKSTEP / RETENTION** — D142/D143 original reports and previous complete ledger/lesson
  sealed fromd5dd11f; book/live/task/resume agree with exact runner scope. G1 stays5/18;
  census12open/130sealed to be independently rederived. Promotion declined (existing CI policy).

Census correction: row-only recipe129 omits exact heading-style D131 in immutable record54.
Reader/body report identities→130 distinct sealed/12 open; D18 absent by design. Source report
and repair identity were independently inspected; D38 owns both-form/uniqueness automated proof.
PLANNING's live recipe corrected; no archived payload or unresolved diagnostic conflict changed.

## Required artifact cleanup — .h1.c

### G1-SLICE.5b.3c.2b.h1.c

Work unit STITCHCAD-G1-0101; source5174e32 clean/brief0, no unfinished local/remote jobs.
Due since2026-10-03 19:18UTC; user daily cleanup mandate, docs/ARTIFACT_CLEANUP.md and actual
cleanup.py plan/apply/protected-store/process/residue contracts reviewed. Same-volume ignored
regenerable output only; frozen producer/HEAD/tracked/candidate fingerprints before deletion.
Inspect all candidates/skips, never cross Git/volume/link boundaries; independent immediate residue
and tracked equality, then regenerate native/WASM/book/probes/gate. Package/toolchain stores retained.

- [x] **ROOT CAUSE (WHY + WHERE)** — actual latest cleanup was2026-10-02 19:18UTC; clock after
  2026-10-03 19:18UTC. Frozen cleanup.py plan→997 candidates/0skipped, rc=0; actual ignored
  path/content/Git/link/device/store checks agree with independent candidate catalog inspection0.
- [x] **ADDRESSED (verified)** — cleanup.py apply→6trees/991strays/15647files/1819917894B,
  residue0/tracked_change false/0skipped, rc=0. OS-visible census→handoff OK. Immediate independent
  filesystem absence and tracked-content hash equals frozen plan,997 absent, rc=0, before rebuild.
- [x] **NO REGRESSION** — exact1.99 make check→663passed/56groups; make wasm→3built;
  make probes→28suites green, including publication10/ledger9+13; make gate→all green; each rc=0.
  Release/debug deps direct .bin/.log census→0files; built dependency outputs remain protected.
- [x] **LOCKSTEP / RETENTION** — latest cleanup overwritten, existing guarded tool reused; original
  complete oldest ledger/lesson retained from5174e32. Book/live/task/resume agree; no package store,
  archived window, reader schema or cap changed. Promotion declined (existing cleanup policy).
