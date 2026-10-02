# Sealed archive — table input and archive CI

Immutable historical segment, sealed by leaf `G1-SLICE.5a.3b.3b.3a` on `2026-10-02`.

- **Sealed identity:** 22 lines, 1861 bytes, `sha256:62f1e78acee77e5d136ffda174f74cd9661c6e9e3d06e0c9145fb6da3c1a316d`
- **Coverage:** G1-0028 and SPINE-0019c; original payload unchanged.
- **Retrieval:** the content below the first `---` rule and following blank line; exactly one terminal newline.
- **Write policy:** none; corrections belong in the live pointer, never in this immutable segment.

---

## STITCHCAD-G1-0028 - named current measurement tables (leaf `G1-SLICE.4a.3`)

sc-measure adds immutable MeasurementTable identity/name and ordered unique measurement-id/token
bindings. Each captures expected metadata id/token/body-POM kind/canonical declaration; current
queries borrow records, refuse missing peers or reassignment, and retain actionable underlying
metadata errors. Same-id source/state/document edits stay visible without numeric caching. Table
names preserve authored content; empty drafts and shared declarations are legal, token uniqueness
is per table. Private representation prevents unchecked mutation. Design revision, source truth,
physical repeatability, formula/evidence policy and release certification remain separate proofs.

Sixteen table contracts plus privacy pass. Eight independent real guard mutations fail their intended
regressions; restored strict checks execute 342 tests, with WASM/book/censuses green. Milestone probes report 23 green suites; the staged doctrine gate passes. All ontology .2.1 length-input fields and table bindings
are mapped to executable APIs; .4a closes structurally, .4 stays active for Ease/SizeSet/signoff. G1
remains 5/18; next .4b per-POM Ease. Completed metadata review moves unchanged to its sibling; oldest
live records seal unchanged as changelog-part25 and devnotes-part27. README/package/book/decision and live pointers agree.

## STITCHCAD-SPINE-0019c - observed archive CI (leaf `SPINE.19.2v`)

For ebed2c5, doctrine run36931196049/job110600555955 and Rust run36931196050/job110600556738
completed success; every step successful, including Python prerequisite/enforcer and Rust/WASM.
Post-commit archive probes 27/0 include committed-window mutation. Doc-only verdict; book/censuses/
staged gate pass. Archive transition closes; next product G1 .4a.3. No new seal or status change.
