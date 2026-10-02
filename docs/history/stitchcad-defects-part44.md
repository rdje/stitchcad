# Sealed archive — D116/D117/D118/D120 static namespace and headers

Immutable historical segment, sealed by leaf `G1-SLICE.5b.1b.1` on `2026-10-02`.

- **Sealed identity:** 26 lines, 2103 bytes, `sha256:7499c60076ac7058074aeeccd872dae8c329ee18d8d5fdb8d93202bdf8673689`
- **Coverage:** D116, D117, D118, D120; original report bodies retained unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D116** — reference lets rebind reserved formula names.
  - Reproduce: actual table-loaded statement("let eps_num: length = 1 cm", {}) ACCEPTs let;
    baseline 2026-10-02 exit0. Contract3.1 forbids every origin rebinding any reserved name.
  - Root: statement checks existing recipe origin only, never reserved membership before evaluation.
  - Impact: false static acceptance changes identity/context meanings; numeric reserved lookup then
    silently wins over the newly authored let. Owner: G1-SLICE.5b.1b.1, current blocking repair.

- **D117** — reference lets silently shadow a different input origin.
  - Reproduce: measurement waist declaration plus let waist:length=1 cm ACCEPTs; baseline exit0.
  - Root: the same guard rejects only origin recipe, not any other visible binding; fixture bind
    also writes a dict directly and can discard a previous declaration before checking it.
  - Impact: flat namespace/history loses input provenance. Owner: G1-SLICE.5b.1b.1, current.
    New namespace builder consumes declaration pairs, preserves both-origin collision evidence.

- **D118** — reference assertion headers accept size names as tolerances.
  - Reproduce: size_index reserved fixture bound count1; assert c:size_index=1 cm==1 cm ACCEPTs.
  - Root: assertion header checks all reserved names rather than grammar's five TOLERANCE names;
    eps_chord also gets runtime tolerance-unbound although it is outside that closed grammar role.
  - Impact: static class evidence is false. Owner: G1-SLICE.5b.1b.1, current blocking repair;
    named but unavailable valid context tolerances keep their separate runtime behavior.

- **D120** — reference assertions admit non-arithmetic operands.
  - Reproduce: boolean parameter flag=False; assert c:eps_num=flag==flag ACCEPTs true, exit0.
  - Root: assertion only tests equal kinds, unlike grammar5's comparison requiring arithmetic kinds.
  - Impact: accidental Boolean/opaque subtraction can certify invalid closure forms or crash.
  - Owner: G1-SLICE.5b.1b.1, current; reuse the actual numeric comparison kind rule before execution.
