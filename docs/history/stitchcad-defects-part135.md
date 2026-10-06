# Sealed archive — whole authored ledger red-classifier defect report and verified repair

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.a23.2` on `2026-10-06` (UTC).

- **Sealed identity:** 20 lines, 2128 bytes, sha256:7783424017815de2433cf8a90db6716a053911ef9677176f20bd45c5876f5f76
- **Coverage:** whole authored D216 report and verified closure; not present at 39861d1.
- **Retrieval:** content below the first rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later live record.

---

- **D216** — ledger mutation classifier admits unrelated assertions as pointer reds.
  Owner: G1-SLICE.5b.4c.h2.b.p.o.a23.2; priority P1, fix in current0226.
  Reproduce: python_ledger_mutation_contract.py --before executes actual39861d1 mutation body in a guarded mirror.
  Nine controlled child responses include four unrelated assertions with only the contract filename token;
  original code reports four actual contract reds, rc=0, target/ledger-mutation-before.log. All source bytes restore.
  Root cause: returncode1 plus any AssertionError/filename token lacks the actual assertion site/case/verdict.
  Impact: broken guards or unrelated assertions can certify a fault detector. Require the expected named case,
  original verdict assertion and structurally inconsistent tuple; independent wrong-site/case/shape controls first.
  Verified repair0226: complete seven sources/all physical archive inputs, both explicit mutable sources,
  one workspace/four logs before effects; fresh source writes, children, logs, restoration and restored reads.
  D216 requires the expected case/polarity, exact original verdict assertion and inconsistent five-field tuple.
  Independent controls pass on3.9/3.14:27 complete plans/15 effect calls/9 prepared children; two interrupted
  restoration cases, six wrong-site/case/polarity/shape/verdict/family refusals, ten actual omission reds,
  profile-alias refusal and thirteen physical CLI refusals with exact mirrors/project bytes, rc=0,
  target/ledger-mutation-controls{,39}.log. The protected-restoration omission is independently detected;
  its owned copied sources are restored by the control after proof. Complete original bodies/edits/argv exact.
  Exclusive native runs pass four current expected case/site/verdict reds and restored13 baseline on both
  caller versions; all seven sources/backing archive bytes exact and zero contract residue, rc=0,
  target/ledger-mutation-real{,39}.log. First filename-text red is catalog target/short label: raw links still match.
  Closed; publication producer remains next, with shell fixture/platform guards separately owned by .b.s.
