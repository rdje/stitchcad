# Sealed archive — whole0218 task and lesson

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.a18.s` on `2026-10-06` (UTC).

- **Sealed identity:** 20 lines, 1605 bytes, sha256:2b57018e0f7635bc8b642b4ec7726e7bcd050fe56262608abef1d6775ed1c19e
- **Coverage:** complete intervals from e45c21d383dcd281a7a25864e1b4e77d5898f03f.
- Whole source interval: docs/tasks/G1-SLICE-producers.md bytes 22275–23448, 1173 bytes, sha256:4a85b2cbd4f55a9ed1a116bc3addace9a7041806175e547c7b72f843ec2370e3
- Whole source interval: DEV_NOTES.md bytes 193–626, 433 bytes, sha256:8a663ee0184ed69cae2ed9ecb7ee3272edd26606232b15f0fe727047e487e1d3
- **Retrieval:** content below the first rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later live record.

---

- ID: `G1-SLICE.5b.4c.h2.b.p.o.a18.r`
  Status: `done`
  Goal: STITCHCAD-G1-0218 repairs D209 receipt archival width; preserves full text/shared scopes and guards new staged lines before parent0219.
  Acceptance: exact original receipt reconstruction, actual400/401-byte reader/gate controls, fixed limits and focused lockstep.

- [x] REPRODUCE / ISSUE: original414-byte receipt refuses archive1; old gate admits it. receipt_retention_contract.py, rc=0.
- [x] ROOT CAUSE: receipt_retention_contract.py exposes missing width/four old admissions/eight boundary verdicts, rc=0.
- [x] FIX: receipt_retention_contract.py proves exact fold/shared scopes; staged400-byte guard; rc=0.
- [x] ADDRESSED: receipt_retention_contract.py on3.9/3.14 passes ASCII/UTF-8/doc-only bounds, rc=0; D209 sealed131.
- [x] NO REGRESSION: census248/publication45/coverage9/archive28+635/ledger11+13 pass, rc=0, target/receipt-retention-*.log.
- [x] LOCKSTEP: receipt_retention_contract.py:12open196sealed/no overlap, rc=0; book/doctrine/live/runner/ledgers/CH197 aligned. Promotion: declined; limits fixed. Commit0218.

  Final staged gate13 green, rc=0, target/receipt-retention-staged-gate.log.

## _(2026-10-06 UTC)_ — D209 receipt width

A 414-byte committed receipt cannot enter the 400-byte archive. Whitespace-only folding reconstructs
the complete original exactly. The fresh acceptance gate now checks added task lines in UTF-8 bytes;
ASCII/Unicode boundary and multiline controls pass. D209 is sealed in part131; limits stay fixed.
Promotion declined: existing bounded whole-retention rule, now checked before commit.
