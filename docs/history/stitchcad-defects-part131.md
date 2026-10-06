# Sealed archive — D209 receipt width repair

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.a18.r` on `2026-10-06` (UTC).

- **Sealed identity:** 9 lines, 1176 bytes, sha256:d09e72babf2dd3535bf3213515d16342383dee48ae32dd17e0825fc54fd6c523
- **Coverage:** whole authored D209 report; absent at 5d24d9e0da39d01425eda76718e3cfc631776c9a.
- **Retrieval:** content below the first rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later live record.

---

### Defect D209 — committed receipt cannot enter the bounded archive

- Owner: G1-SLICE.5b.4c.h2.b.p.o.a18.r; P1, fixed before signed-angle work.
- Reproduce: complete .a17 receipt from5d24d9e has414-byte LOCKSTEP line; public read of initial part286 refuses1 with logical maxline bound exceeded. Source was kept intact; refused uncommitted candidate removed.
- Cause: live task line allowance exceeds the archive400-byte bound; staged evidence check judged signatures without byte width.
- Impact: rollover stalls; retaining a trimmed receipt would lose information.
- Repair: fold whitespace only; exact original whole receipt reconstructs with sha256:a8ab93cac9047b0d8a8eb6140597da8cffa060a5f5f887cbfd5abcc1915ce1e4. Shared pending acceptance expands exactly; prior scope records stay unchanged.
- Verified: receipt_retention_contract.py on Python3.14: eight actual ASCII/UTF-8 archive/gate verdicts, four original admissions, multiline positive/two evidence refusals; rc=0. Stage guard rejects new task lines over400 bytes; archive limits/reader unchanged.
- Status: fixed; standing control watches prevention and whole text, normal doctrine gate watches staged widths.
