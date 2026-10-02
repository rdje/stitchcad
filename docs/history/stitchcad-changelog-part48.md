# Sealed archive — current per-POM Ease sets

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3b.3b.3c.1v` on `2026-10-02`.

- **Sealed identity:** 14 lines, 1137 bytes, `sha256:0f58d786f68478aedd484bae01625d158ee32b6572b74e080b0aacfc1993d4d1`
- **Coverage:** STITCHCAD-G1-0030; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## STITCHCAD-G1-0030 - current per-POM Ease sets (leaf `G1-SLICE.4b.2`)

Immutable ordered sets bind unique mapping identities, machine tokens and POM identities to named
current body/garment tables. Borrowed canonical mappings retain current fit, provenance/permission and
amount state/source; retargeting or missing identities refuse. Both selected table memberships and
current Ease validate before lookup returns. Shared body/amount sources, one mixed table and empty
drafts are legal; missing POMs never acquire default mappings or zero ease.

Fourteen contracts plus privacy pass. Ten production guard removals each produce an actual assertion
failure and restore exact source. Strict Rust passes 371 tests, with WASM/book/glossary and focused
tracking/ledger/staged gates green. D69 fixes stale package discovery, verified by cargo metadata;
it seals in defects-part13. Existing D34's stale sibling example is corrected; derivation remains
owned. Prior individual Ease contract/checklist retained unchanged; oldest live entries seal unchanged
as changelog-part27/devnotes-part29. Next .4b.3 structural Ease review; G1 stays 5/18.
