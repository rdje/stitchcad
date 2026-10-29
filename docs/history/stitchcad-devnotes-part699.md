# Sealed engineering continuity — part699

- **Identity:** 48 lines, 5246 bytes, sha256:2dd8223fb76f5779fd4f07ddf1885e23a96b287b314d490940c334f1333e4725
- **Coverage:** whole0540 task scope, lesson and changelog from c7d67b210997574024cab4d77d910b26bde0c8cb.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 25354–28574, 3220 bytes, sha256:30c5a1c1595e65755a1d468fc033e03b593fd9a503967f79129334a47e0bdc65
- Whole source interval: DEV_NOTES.md bytes 193–1413, 1220 bytes, sha256:97ccaa0f0c4b746e0c2eea9627627762cc79b67367c7a0a6a4dfc7a886b2548d
- Whole source interval: CHANGELOG.md bytes 207–1013, 806 bytes, sha256:fce4269178ced9f365ef8681d23b4858c25433527ee1be60f7b3742b496e2401
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.3.2.prepush-400.archive-profiles`
  Status: `done`
  Goal:0540 D310 P1 descriptor repair;exact retained/decoded identity checks with legacy refusal
  and independent coverage controls. No sealed record edited or newline normalized.
  - [x] REPRODUCE / ISSUE: full0538 probes at95925f7b →REAL/COVERAGE-RED failed,rc2.
    Old descriptor uses shell substitution/readded LF and rejects raw blank EOF/no-LF/encoded
    source records despite valid canonical inventory. Diagnostic scope/owners retained698.
  - [x] ROOT CAUSE (WHY + WHERE): 0540 canonical inventory/header census →569 legacy,
    580 whole-raw,2 encoded at1151 records,rc=0. Identity hashes describe retained/decoded payload,
    not shell-normalized text. Historical interval metadata can separately describe normalized or
    folded retrieval;descriptor checking does not assert every interval equals its Git bytes.
    D282 raw-byte policy and current records require preserving exact EOF. Native shasum frozen
    alpha/LF/blank/no-LF hashes match independent fixtures,rc=0. No filename-specific exception.
  - [x] FIX: byte-reader checks unique declared identity/hash/lines/optional legacy bytes;
    structured whole-source profiles require byte count,retain raw EOF or decode exact base64
    fence/alphabet/width/size. Legacy exactly-one-LF refusal retained. Coverage/Git source proofs
    stay independent. Shell consumes diagnostic streams,prints actual FAIL lines and names checker
    failure instead of dropping it. New helpers registered for D315's final inventory repair.
  - [x] ADDRESSED (verified): 0540 local_environment.py -- bash
    docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh →11 pass/0 fail,rc=0,
    REAL1462 verdicts/0fail,COVERAGE-RED sole mutated refusal;legacy blank/digest/nonledger/order/
    pointer controls retained. Pointer contract16/0;descriptor contract4 frozen profiles/18
    refusals/independent coverage pass,rc=0. First partial run was interrupted by editing its
    running shell input;consumed rc2 and reran final frozen source to completion before further
    edits. Lesson:consume terminal before changing any running producer input;no partial success.
  - [x] NO REGRESSION: bash -n ledger script →rc=0;mdbook build/positive publication
    →93/248/1590/2771,rc=0;glossary310/9/158/0 failures,rc=0;map →pass,rc=0. Receipt698 →3 exact
    Git spans,rc=0;retention →1152 logical/57 working/64935 decodedL/5051627 decodedB/2628964
    residentB,rc=0. D311–D315 remain owned;full check/gate/probes retry after those repairs,then
    normal push and actual hosted jobs. No full repository or hosted verdict inferred here.
  - [x] LOCKSTEP: whole0539 task/lesson/changelog retained698 before curation. Archive-reader
    book explains three profiles/examples and descriptor-versus-source/coverage boundary. MEM/
    DEV/CHANGE/LIVE/index/ledger align;LIVE marks D310 fixed,29 suites/G1 6/18 unchanged,G0 human
    unapproved. README objective/commands unchanged. Commit before next .catalog-mutation D311.
    promotion: declined (application of indexed byte-preserving archives,independent proof scope and consume-before-edit contract;no new product policy).
## _(2026-10-29 UTC)_ — preserve exact archive payload descriptor profiles

0540 D310:old shell newline normalization conflicted with retained raw/encoded records. Byte reader checks identity/hash/lines/bytes;whole-source raw EOF survives,explicit base64 decodes strictly,legacy blank EOF still refuses. Coverage and historical Git proof remain separate;sealed records unchanged.
Focused ledger11/0,pointer16/0,descriptor4 frozen profiles/18 corruption refusals pass0. Diagnostics consume streams/report actual failures. First partial run edited while its shell was active ended2;final frozen rerun completed0. Consume producer terminals before changing inputs.
Book93/248/1590/2771,glossary0/map/698 receipt3/retention1152 pass0. Whole0539 retained before curation. LIVE29 suites/G1 6/18 unchanged,D310 fixed,D311–D315 P1 owned. Full retry/push/actual hosted jobs wait for remaining repairs;G0 human unapproved.
[Whole0539 diagnostic scope](docs/history/stitchcad-devnotes-part698.md). Promotion declined:indexed archive byte preservation/proof scope/terminal discipline. Historical normalized/folded interval metadata is not a descriptor identity or an automatic exact-Git proof. Next D311 authority-row mutation.
## STITCHCAD-G1-0540 — preserve exact archive descriptor profiles

- Fix D310 with byte-preserving raw and explicit decoded base64 identities;check hashes/line and byte counts while retaining legacy blank-EOF refusal. Keep coverage and historical source proofs independent. Never edit sealed records or normalize their bytes.
- Focused ledger11/0,pointer16/0,descriptor4 frozen profiles/18 refusals pass;report actual failure lines without early pipe closure. Interrupted partial run discarded;final frozen producer completed successfully before dependent edits.
- Whole0539 retained698/three exact Git spans;book93/248/1590/2771,glossary0/map/retention1152 pass. LIVE29 suites/G1 6/18 unchanged,D310 fixed,D311–D315 owned. Full gates/push/hosted verdict await remaining repairs;G0 human unapproved.


