# Sealed archive — whole early archive chronology

Immutable historical segment, sealed by leaf `G1-SLICE.4c.4.3.2` on `2026-10-26` (UTC).

- **Sealed identity:** 36 lines, 2713 bytes, sha256:fb9f86606b1d27c6f0c642d5ad728f742dce64a0d84a16ce02de40b65264cba3
- **Coverage:** whole committed source interval from 8f471138bbd75873dc386038fa57b0d738a75984.
- Whole source interval: docs/book/src/governance.md bytes 25512–28225, 2713 bytes, sha256:fb9f86606b1d27c6f0c642d5ad728f742dce64a0d84a16ce02de40b65264cba3
- **Retrieval:** bytes below the first rule and blank line; original terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

For the first archive implementation head ebed2c5, the
[doctrine run](https://github.com/rdje/stitchcad/actions/runs/36931196049) (job 110600555955) and
[Rust run](https://github.com/rdje/stitchcad/actions/runs/36931196050) (job 110600556738) were observed
completed successfully with every step successful, including the Python prerequisite, archive
enforcer and native/WASM checks. Post-commit refusal probes also reject an edit to a committed
window. This is tooling/structural evidence; it grants no domain or physical production approval.


The second transition captures63 immutable raw records from372033f into window2. Before removing
working copies, the installed reader/source comparison reconstructed all63 full files exactly;
an isolated fixture read/materialized all127 logical records, including the unchanged first window.
Maintained links now land on catalog headings, while reader identities remain the original paths.
`bash scripts/history_archive.sh prove-source window2` repeats its source comparison when that
Git object is available. The bounded reader and aggregate ceilings are unchanged.

Watched CLI controls verify logical reads/materialized bytes by full-file identity,
plus newest-window digest/member/catalog and cross-window collision refusals. In local window10's copy,
catalog prose keeps inventory green but triggers the committed guard; root bytes stay exact.
Title checks reject misleading labels.

At pushed f876913, the [doctrine job](https://github.com/rdje/stitchcad/actions/runs/36989497775/job/110782133989)
and [Rust job](https://github.com/rdje/stitchcad/actions/runs/36989497872/job/110782134441)
completed successfully with all seven/nine steps successful. The archive prerequisite/enforcer and
fmt/clippy/tests/real WASM passed;140 post-commit local CLI controls also verify newest immutable
catalog refusal. G1-SLICE.5a.3b.3b.3c.1v records the exact head/job observations.
These are retention/instrument proofs, not garment signoff.

The third transition captures62 raw sealed records from af98fff before the next namespace review
would exceed the64-file working limit. A fresh local fixture reconstructs all189 logical records
exactly, including both previous windows. The installed reader proves each of the62 original full
files before their working copies retire; maintained pointers land on window3's member headings.
`bash scripts/history_archive.sh prove-source window3` repeats the capture/source comparison.
No reader, checker, logical address, decoded record or limit changes. At exact pushed b595a37, both
CI jobs and every reported step completed successfully. G1-SLICE.5b.1b.0v records their receipts
and independent newest committed-catalog refusal.

