# Sealed archive — verified map-reference and handoff-documentation defects

Immutable historical segment, sealed by leaf `G1-SLICE.5b.4c.h2.b.p.o.a25.2` on `2026-10-25` (UTC).

- **Sealed identity:** 16 lines, 1472 bytes, sha256:299b23000c002e157b678f5ab03bb78f867d4b77caa9b209de3373476d54153a
- **Coverage:** whole completed source interval from 1b8fc23c7aab412d6a4602290dfcc26faceaeeb0.
- Whole source interval: docs/tasks/PLANNING.md bytes 84570–86042, 1472 bytes, sha256:299b23000c002e157b678f5ab03bb78f867d4b77caa9b209de3373476d54153a
- **Retrieval:** bytes below the first rule and blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in a later record.

---

- **D220** — map's current-source reference assumes an HTML header despite accepting five header forms.
  Reproduce: python_map_handoff_fixture_contract.py --headerless removes only the reference header in
  memory; the actual unchanged generator preserves its body, but the old reference raises IndexError
  at content.split('-->', 1)[1]. Impact: valid header removal breaks the watched map contract.
  Owner: G1-SLICE.5b.4c.h2.b.p.o.a24, fix now; independent body/header controls replace the assumption.
  Fixed: current-body reference handles all five shapes; headerless metamorphic control passes without
  editing the curated file. Native generator retains its original five cases/two actual body reds.

- **D221** — governance still says D217 blocks current live handoff proof after its verified repair.
  Reproduce: governance.md's handoff section says blocked; committed0237 DEV_NOTES records native
  calibration and real clear0/held1/restored0. Impact: the director's book reports an obsolete blocker.
  Owner: G1-SLICE.5b.4c.h2.b.p.o.a24. Fixed wording names verified inheritance repair and current quiet-
  workspace precondition. Fresh native calibration passes10 lifecycles/two reds; census evidence is
  available, with exit1 for DiskWatch's directory handle rather than D217's old unavailable evidence.
  After the handle closes, fresh native clear0/held1/restored0 passes with a terminal0 holder;
  historical FD10 stays unknown. D221 closed by .a24.
