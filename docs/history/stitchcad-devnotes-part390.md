# Sealed archive — whole numerical certificate protection scope

Immutable historical segment, sealed by leaf `G1-SLICE.5d.1.2.2.2.b.2.b.2.b` on `2026-10-26` (UTC).

- **Sealed identity:** 22 lines, 2497 bytes, sha256:80a79057229eb0872d41814d10d051ea20111df5854f7ec535cb8a63bef5d148
- **Coverage:** whole committed source interval from c9e6da4cee52831a7fad6e34d2e9f1c7bdf1ce18.
- Whole source interval: docs/tasks/G1-SLICE-recipes.md bytes 29559–32056, 2497 bytes, sha256:80a79057229eb0872d41814d10d051ea20111df5854f7ec535cb8a63bef5d148
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.5d.1.2.2.2.b.2.b.2.h`
  Status: `done`
  Goal: P1 protect retained numerical certificates from generic artifact cleanup; next after .a commit, before .b or cleanup.
  Defect0284: cleanup.py inspect accepts actual forward0278 chunk-000.bin (2080000B) and
  inverse0284 chunk-000.bin (28800000B) as deletable strays, rc=0. No deletion performed.
  Reproduce: runpy cleanup.py; inspect each path with tracked(). Generic .bin suffix has no
  certificate-store exclusion; deletion would break retained readers/checkpoint continuity.
  Paths: docs/tasks/artifacts/artifact_cleanup/cleanup.py;docs/ARTIFACT_CLEANUP.md
  Verification paths: docs/tasks/artifacts/artifact_cleanup/certificate_store_controls.py;docs/tasks/artifacts/ci_environment/python_cleanup_entry_contract.py
  Existing AST baseline locks PROTECTED; admit exactly two new stores while preserving all old
  constants/bodies. New read-only actual-census/guard-omission control is watched by that entry suite.
  Fix/verify exact store protection and ordinary stray control; own until verified/closed.
  Work unit0285 also owns389 whole0284 controller task and257 ledger before acceptance growth.
  Focused read-only protected-store/ordinary-stray census; no cleanup run or cadence reset here.
  Acceptance0285:
  - [x] REPRODUCE:0284 actual cleanup inspect admitted2 retained .bin proof chunks; read-only/no deletion.
  - [x] ROOT CAUSE0285: certificate_store_controls.py actual census and compiled omission reproduce admission when both new prefixes are absent, rc=0; generic suffix rules lacked proof-store identity.
  - [x] FIX: protect/prune both exact stores; old-policy comparator admits precisely those2 additions and watches the new census control.
  - [x] ADDRESSED0285: python3 -I -B docs/tasks/artifacts/artifact_cleanup/certificate_store_controls.py protected2/ordinary-stray1/omission-red1/restored1 passes, rc=0;2 actual proof files refuse unchanged.
  - [x] NO REGRESSION0285: runpy.run_path('docs/tasks/artifacts/ci_environment/python_cleanup_entry_contract.py')['original_body_controls']() old bodies/constants exact except admitted2 stores and new control pass, rc=0. Ordinary scratch/log selection remains; fixture local Git TMPDIR prepared/no warning. Broader .5d.4/pre-push.
  - [x] LOCKSTEP:389/257 exact Git/hash/public bytes, rc=0; policy/book/live/decision close owned defect. No project cleanup or cadence reset. G1still5/18; full inverse .b next.
  Verification: rc=0; Commit: STITCHCAD-G1-0285.
