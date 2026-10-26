# Sealed archive — whole closed D229-D232 reports

Immutable historical segment, sealed by leaf `G1-SLICE.4c.4.5` on `2026-10-26` (UTC).

- **Sealed identity:** 41 lines, 3382 bytes, sha256:4bc0f407e2e7a6e2ac3189e0793ed2ad151903cb8495a6922be483a808f20e99
- **Coverage:** whole committed source interval from fb53a444edc41c0b9039039f92b6791f205932d0.
- Whole source interval: docs/tasks/PLANNING.md bytes 83168–86550, 3382 bytes, sha256:4bc0f407e2e7a6e2ac3189e0793ed2ad151903cb8495a6922be483a808f20e99
- **Retrieval:** bytes below the first rule and blank line; original terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

## D229 — current book retention label lagged the actual archive

At sourceba41e9a, the master ends in window14 while governance's Current retention window section
still names13. Reproduce: git show ba41e9a for the master and governance; 6662298 governance delta0.
Impact: the reviewer's book presents an old current archive, though logical records are intact.
Owner/schedule: G1-SLICE.4c.3d.3.h15, P2 fixed0305 during required packing. Preserve the complete
window13 narrative under its historical heading; publish current15/source/retrieval boundaries.
Focused source comparison0/book0 and byte-exact retention proofs verify the correction; no data loss.

## D230 — companion sizing pages retained an obsolete implementation frontier

At source7618d20, observation/collection pages still place axes/breaks in later slices; membership's
footer similarly defers complete coverage. Reproduce: rg axes/breaks/coverage in those three book pages.
Impact: the main SizeSet chapter and companion pages disagree about current implemented inputs.
Owner/schedule: G1-SLICE.4c.3e.1, P2 fixed0309; link actual axes/break/composite behavior while retaining
physical/global-currentness/profile/approval boundaries. Source review0/book0 verify the correction.

## D231 — moved MTM Rust fixture omitted from the mutation entry's source plan

After0307, the contract imports support/mtm_fixture.rs but the entry plans only implementation/root test.
Reproduce: existing measure_refusals with an independent explicit fixture oracle; missing copied fixture
reaches activation/effect (controlled AssertionError), rather than early exit2. Actual source bytes unchanged.
Owner/schedule: G1-SLICE.4c.3e.1, P1 fixed0309 before scope closure. Register the read-only fixture and
independently require it in producer controls. Complete13-child/file capture,5 physical refusals and a
compiled actual fixture-plan omission red pass0; no full native mutation campaign or kernel rerun.
- [x] ROOT CAUSE0309: independent missing-fixture control reaches activation/effect on the pre-repair entry; actual source bytes unchanged, rc=0.
- [x] ADDRESSED0309: existing complete13-child/file capture and5 physical refusals pass; compiled actual fixture-plan omission red detected, rc=0.
- [x] NO REGRESSION0309: whole D225-D227 source7618d20 bytes/hash/public exact in138, actual implementation/entry/test/fixture hashes stable and book rc=0.

## D232 — cleanup cannot forward explicit idle-CUA attestation

Reproduce: cleanup apply calls handoff without --idle-cua; actual default blocks4 metadata-only CUA
processes with0 repo handles, while canonical --idle-cua passes0. All1047 selected paths remain.
Owner: G1-SLICE.4c.4.cleanup5.a, P1 fixed0315 before fresh cleanup; explicit apply-only CLI passthrough and
controlled intact/default/attested/refusal checks. First lsof unavailable-type refusal is separately
observed before deletion; initial raw snapshot absent, later retries retained under SPINE.23r; no race claim. Fresh plan required after code repair.

- [x] ROOT CAUSE0315: actual default4 blocks/attested0/1047 selected intact; cleanup fixed argv lacks flag, rc=0.
- [x] ADDRESSED0315: explicit apply-only forwarding; actual captures/old refusal/2 reds/5 invalid arguments rc=0.
- [x] NO REGRESSION0315: original policies/certificates/blocked-clear effects and49 handoff fixtures/17 reds/book rc=0.

