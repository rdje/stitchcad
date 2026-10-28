# Sealed engineering continuity — part632

- **Identity:** 84 lines, 7619 bytes, sha256:996cac8f3832b59786acc12ccab8480baff3b43ffdbedb8dc33a52590fd5e587
- **Coverage:** whole0477 native acceptance scope, lesson, changelog and0475–0477 decisions from d5298be8304b891c697e4d1f386f43a5a21c9bad.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 53293–54857, 1564 bytes, sha256:97b63d57956284dcdee79d2c0cf8494fa38cef5f9e7abed495b80508a614f804
- Whole source interval: DEV_NOTES.md bytes 193–1361, 1168 bytes, sha256:142e7171c2bfe13b88781c0a3cdfa6a21cca79e8a3a6bd485d170a071e62d7c8
- Whole source interval: CHANGELOG.md bytes 207–944, 737 bytes, sha256:2bdcc19bb061ff23b992742ccf7441d3f50f4c5b1e1357948cd4ac2543e25f00
- Whole source interval: docs/decisions/decision_command-layer-contract-and-undo-granularity.md bytes 9136–13286, 4150 bytes, sha256:00dfd262a2e982201b29dac8535076eda679c45cbb5e4fe44f01f12760798464
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.9.3`
  Status: `done`
  Goal: combined current implemented metadata-bus/public schemas/authority/resource/privacy acceptance.
  - [x] REPRODUCE / ISSUE: completed feature had focused checkpoints;native current-target census/log
    producer derives19 core/2 measure targets and98 actual bus cases,all source artifacts local rc=0.
  - [x] ROOT CAUSE: python3 -I -B bounded log census →core121/measure196,command15/design34 docs rc=0;
    no production changes in this leaf;source/API classes and actual runtime subset remain distinct.
  - [x] FIX: execute current combined contracts after feature implementation;record exact populations
    and update user book while preserving pending full publication,other handlers and product boundaries.
  - [x] ADDRESSED: cargo test core17 command_* targets plus design_resource/design_metadata →121 passed;
    measure design_inputs/resource_contract →196 passed,total317 rc=0;all98 bus cases in163 pass0.
  - [x] NO REGRESSION: cargo test -p sc-core --doc command:: →15 passed;--doc design:: →34 passed rc=0;
    book86 chapters/161 API rows/1549 source/2615 rendered links rc=0;no workspace-wide CI claim.
  - [x] LOCKSTEP: MEMORY/DEV/CHANGELOG/decision/index and LIVE combined-native note updated;README
    correct unchanged;whole0476/D297 retained631(4 Git spans),retention1085/2436702residentB rc=0.
  Verification: selected combined317/doctests49/book/retention rc=0;full publication probes next.
  Commit: STITCHCAD-G1-0477;parent .6.9 remains in_progress until .6.9.publication closes.

## _(2026-10-28 UTC)_ — combined current native metadata-bus acceptance

0477 .6.9.3 accepts current native metadata bus/public inputs:317 cases across21 targets;core121(17 command targets66+resource47+metadata8),measure196(Design-input163+source33). Completed logs derive the actual bus subset98;all pass0.
Command15/Design34 namespace doctests pass49, including privacy/lifetime/sealing controls. Registry/error/resource/transaction features participate in the full current targets. Production sources unchanged;publication fixture probes still pending.
Book command-workflows#combined-native-acceptance records populations/scope;86chapters/161APIrows/1549source/2615renderedlinks pass0. Broader handlers,app/adapters/persistence/browser/physical/release/full-workspace CI remain actual owners.
Promotion:indexed decision. Whole0476/D297 in631(4 exact Git spans);retention1085/2436702B passes0. LIVE combined native note,G1still6/18;README correct unchanged. Next .6.9.publication full existing probes;push cadence unchanged.
[Whole0476 native-workflow/D297 lesson](docs/history/stitchcad-devnotes-part631.md) retains precise metadata/permission/runtime evidence.
## STITCHCAD-G1-0477 — accept the combined current native metadata bus

- All317 cases pass across21 selected targets:core121 and measure196, including all163 Design-input cases and98 actual bus cases;49 command/design doctests also pass.
- Current workflow, discarded-error, resource, transaction, authority and public-input features participate in the combined targets; production sources unchanged.
- Book records exact populations and scoped evidence; broader handlers, adapters, persistence/browser, full workspace CI and physical/release proof remain separately owned.
- Whole0476/D297 retained631; next full existing publication fixture probes, including named-result compatibility. No early push or whole-project certificate.

## Discarded private preparation context0475

answers: "how does a failed group identify discarded preceding commands without exposing candidates?" · "does the command census prove execution behavior?"

D295 reproduced natively:the mixed prefix/alias/failing-Notch error named only the failing AddNotch,
although command-layer §9/§11 require preceding discarded commands. Mutation/RetryConflict/Difference
now carry fixed Copy CommandGroupDiscardedStep data:index, command and key from actual successful
first-occurrence steps. Preserve original positions/order and omit aliases/failing/unexecuted members.
The producer copies no report/model and calls no authority/source provider; original error sources stay
unchanged. Diagnostic construction occurs only on these error paths, within admitted member counts.
Exhaustive external error patterns must add discarded or ..; existing wildcard patterns remain valid.

Native diagnostic1 fails before/passes after;transaction3/group-preview6/group-commit6/lint/fmt pass0.
Preview/commit witness positions0/2/3 and original keys, empty prefix, native Notch source and key-conflict
cause, then corrected same-key/two-ID/one-slot publication equal to a real reference. Lifecycle and
accounting sources stay unchanged; final Difference uses the same actual-step producer.
D296 corrected the G0-era census header:no executable bytes changed;17/5/5/0 native census passes.
Specification closure does not establish execution; native workflow evidence/full bus acceptance follow.

## Builtin workflow discovery and exact permission sets0476

answers: "where are actual native workflows declared?" · "why report a permission set instead of the strictest level?"

CommandWorkflowId/CommandWorkflowStep declare3 immutable native sequences once. Stable exact lookup
has no whitespace/case alias or default. Unique command/class metadata derives from catalog identities;
complete distinct rights derive from each purpose. Extract the existing group-purpose match unchanged
and share it with actual authorization; all other grant-check bytes/order and lifecycle code stay exact.
Discovery contains no model/session/policy/grant and is never runtime availability or authorization.
Non-exhaustive public enums permit future additions; actual new stages/workflows need native proof.

Native acceptance interprets every declared stage over real complete sources/models:inspection1,
edit/preview/replay5 and Undo/Redo7. It checks original/current reads, actual metadata differences,
identity issuance, capacity replay and restored snapshot pointers. A separate permission matrix proves
Inspect cannot commit and Commit cannot read/preview. Native2/metadata2/schema4/catalog2/group-auth5/
strict lint/fmt pass0; generated stdout table is watched by the core book-projection contract.
D297 clarified the misleading ordered/strictest wording. Named permissions stay distinct; native
preview requires Inspect alone, Propose is separate metadata, and partial grants never gain a union
through discovery. UI/API/MCP parity remains empty until actual adapter tests/digests exist. The new
book part and2 public API rows explain this boundary; broader .6.9.3/publication acceptance follows.

## Combined current native acceptance0477

answers: "what current native command populations passed together after implementation?"

All317 selected cases pass across21 targets:core command-contract66 in17 targets plus model-resource47
and metadata8;higher Design-input163 plus source-resource33. Completed output derives98 actual bus
cases within that163. Command15/Design34 doctests pass49, including privacy/lifetime/sealing controls.
Current workflow, discarded-error, resource, transaction and authority paths all participate. Production
source is unchanged in this acceptance leaf; source diagnostics/types and actual metadata handlers
are distinguished from still-unimplemented other runtime handlers. The book records native commands,
counts and limits. Full publication fixture probes follow before this metadata lane closes; workspace
CI/persistence/browser/adapter parity and physical/release evidence remain separate owners.
