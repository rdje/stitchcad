# Sealed archive — D111 current statement identity status

Immutable historical segment, sealed by leaf `G1-SLICE.5a.4` on `2026-10-02`.

- **Sealed identity:** 7 lines, 653 bytes, `sha256:437f35b7ec3aabfd46de0fcda74792086855c5b9d38490ae4c84ed59d852e47b`
- **Coverage:** D111; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

- **D111** — formula contract §2 still says statement identity is pending after .3f.1c implements it.
  - Reproduce: search "Statement identity, bindings and evaluation remain pending" in
    docs/book/src/spec/formula-language.md and compare canonical_recipe.rs public canonical_form
    plus formula_canonical_recipe_contract.rs actual exact-byte controls.
  - Impact: students/expert readers receive conflicting implementation status inside one chapter.
  - Owner/schedule: G1-SLICE.5a.4, P2 now; correct current status and verify source/rendered links
    and all existing serializer contracts. No execution, persistence or approval claim added.
