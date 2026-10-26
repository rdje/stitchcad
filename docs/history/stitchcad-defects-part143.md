# Sealed archive — whole closed D238 report and evidence

Immutable historical segment, sealed by leaf `G1-SLICE.6.2.2.c.2.5.1` on `2026-10-26` (UTC).

- **Sealed identity:** 12 lines, 1030 bytes, sha256:402d8065f57cb2ec4c92f750fe852f72877394c721da6e45b583666c7d69dc63
- **Coverage:** whole committed intervals from 22ab07432d1cedd4a5e5e02f69d9debad03966d4.
- Whole source interval: docs/tasks/PLANNING.md bytes 83859–84889, 1030 bytes, sha256:402d8065f57cb2ec4c92f750fe852f72877394c721da6e45b583666c7d69dc63
- **Retrieval:** bytes below the first rule and blank line; original terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

## D238 — membership frontier leaves profile intent/application ambiguous

At516895d, size-membership's final paragraph says profile transformations remain later, without
separating implemented immutable intent from future application. Current SizeSetResolutionIntent and
companion chapter implement that intent. Reproduce the unqualified frontier against those actual APIs.
Impact: primary book view can underreport structural availability. Owner G1-SLICE.6.2.2.c.2.4.3,
P2 fix0327 while updating artifact inputs. State implemented intent and deferred actual profile application;
retain geometry/global currentness/evidence/release boundaries. No measurement-model code changes.
- [x] ROOT CAUSE0327: existing intent exports/source contradict unqualified pending wording; source review rc=0.
- [x] ADDRESSED0327: membership distinguishes implemented intent from application; book/publication76/93 rc=0.
- [x] NO REGRESSION0327: native command11/0, measure sources unchanged; physical/global/release boundaries retained, rc=0.

