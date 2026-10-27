# Sealed engineering continuity — part568

- **Identity:** 60 lines, 4759 bytes, sha256:9ab6cbbd4ef64e1f93e697e7caa441fc4c665f704151cb99dd7fb38b73729bf3
- **Coverage:** whole maintained-book subtree,0418 lesson, ledger and D271–D280 retirement from 41400cd5f4a74a7f3866a06b38e26c027b2b1b2c.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 46546–48719, 2173 bytes, sha256:20a43abf2561841c044e870e2cf19c998aedc40d83be84477e8d5049db111390
- Whole source interval: DEV_NOTES.md bytes 193–1266, 1073 bytes, sha256:825147c73067a91c5ef40b5fb9bcb7fc992aeb594d6aa673a6cb7e6aa70b56bf
- Whole source interval: CHANGELOG.md bytes 207–680, 473 bytes, sha256:f8647fb851b98f9fde271038b5d44c077647e301bba5ed8523ef76f1281e24e3
- Whole source interval: docs/tasks/PLANNING.md bytes 90009–91049, 1040 bytes, sha256:501deee855f1b9d8972ceaa550b21ac3947f82916168c3fdbcc22bef1901ed22
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.book`
  Status: `done`
  Goal: unique product mdBook maintained-reference contract, source policy lifecycle; preserve complete
  direct navigation and current per-part/index bounds. Exact baseline/content digest and fresh owned
  signed deltas for every aggregate change, refuse stale/reused/banked authority; no arbitrary cap rise.
  Completed0416–0418:bounded reader/native refusals, guarded authoring/atomic adoption and scoped
  acceptance before bus product work. Full neutral package remains unadopted.
- ID: `G1-SLICE.6.5.book.1`
  Status: `done`
  Goal:0416 strict reader23/optimized2/native controls; [whole proof](../history/stitchcad-devnotes-part566.md).
- ID: `G1-SLICE.6.5.book.2`
  Status: `done`
  Goal:0417 installed guarded authority/D278; [whole proof](../history/stitchcad-devnotes-part567.md).
- ID: `G1-SLICE.6.5.book.3`
  Status: `done`
  Goal:0418 scoped adoption acceptance, D279 CRLF width and D280 exact Git storage-byte correction.
  - [x] REPRODUCE / ISSUE: newline_contract.py --baseline:6 native subcase failures rc=1, expected;
    awk441 vs440; Git changes CRLF object ids. Actual03460b0 committed-mode verification rc=0.
  - [x] ROOT CAUSE (WHY + WHERE): native awk/Git probe rc=0:width includes CR; Git cleans raw bytes
    after hashing. No root attributes existed. Ordinary target fixtures, same Git metadata read-only.
  - [x] FIX: newline_contract.py:3 tests OK rc=0; one CR excluded from width, raw wc unchanged;
    effective text/filter/ident/encoding attributes must be unset; writer freezes root policy.
  - [x] ADDRESSED: maintained_reference_contract.py:23 tests OK rc=0; authoring contract8 OK rc=0;
    native conversion settings preserve exact bytes; malformed/effective transformation refuses.
  - [x] NO REGRESSION: live-doc-size probes7/0 (numeric17/overlap2), AST/bash/book/publication/retention
    and git diff --check rc=0; source-neutral policy body unchanged. Full product proof remains owned.
  - [x] LOCKSTEP: transition_receipts.py --record docs/history/stitchcad-devnotes-part567.md:4 exact
    Git spans rc=0; complete0417 proof retained; book/decision/live records sync. G1 remains6/18.
## _(2026-10-27 UTC)_ — exact byte storage and scoped book acceptance

D279/D280: newline_contract.py --baseline reproduces6 native subcase failures rc=1:terminal CR is
counted as content and Git autocrlf rewrites bytes after authority hashing. Corrected3 controls pass0:
strip one CR for width only; retain raw wc/digests; explicitly unset Git text/filter/ident/encoding
for governed Markdown/control and require effective native attributes. Freeze root policy during
recording; no source normalization. Native hash-object never writes objects; fixtures are ordinary
project-local directories using this same repo's Git read-only. Hash-object resolves --path against
cwd, so baseline reproduction selects both fixture cwd/worktree; check-attr alone is insufficient.
Reader23/authoring8 pass0; actual03460b0 committed-mode verification passes0. Scoped acceptance keeps
runtime/semantic/physical/release proof separately owned. Whole0417 retirement567:4 exact spans0.
Promotion: containment decision. Probes7/0 (numeric17/overlap2), AST/bash/book/publication/retention0.

## STITCHCAD-G1-0418 — close scoped book adoption and correct newline storage

- Actual03460b0 committed-mode verification passes; reader23/authoring8/native newline3 pass0.
- Fix D279 CRLF content width and D280 Git clean-byte drift; effective storage attributes enforced.
- Preserve raw source bytes, bounded semantic parts/index and exact fresh change authority.
- Whole0417 evidence/retirement567 retains4 exact Git spans; broader product/release proof stays owned.

## D271–D278 — retained causes, controls and scope corrections
[Whole0417 retirement](../history/stitchcad-devnotes-part567.md) preserves prior reports and proofs.

## D279/D280 — newline measurement and exact Git storage
D279 P3: native awk counts441 for440 content bytes plus CRLF; adopted contract excludes terminal CR.
D280 P2: native git hash-object with core.autocrlf=true changes CRLF bytes before object storage, so
working-tree authority can disagree after commit. Owner G1-SLICE.6.5.book.3 fixes both in0418.
Reproduce: newline_contract.py --baseline gives6 native subcase failures rc=1; corrected3 tests pass0.
- [x] ROOT CAUSE: transition_receipts.py --record docs/history/stitchcad-devnotes-part567.md:4 exact
  Git spans rc=0; whole0417 scopes/retirement preserved; native awk/Git localizes both defects.
- [x] ADDRESSED: newline_contract.py:3 tests OK rc=0; raw bytes preserved and effective policy checked.
- [x] NO REGRESSION: reader23/authoring8/probes7/0/book/publication/retention rc=0; product proof stays owned.
