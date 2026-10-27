# Sealed engineering continuity — part529

- **Identity:** 80 lines, 5602 bytes, sha256:b12602912e2f0183d841f372c55847b5f4b588ec43478311da6f13f6a66cf228
- **Coverage:** complete Closure cause task/lesson/ledger, D260 report and historical geometry-provider evidence from dcce0677103469809edb0aade802243039782b04.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 51322–52696, 1374 bytes, sha256:a1ba69ff3305d97c216aba171a3af907d093091a919cfe4d282efe04eee0a7bb
- Whole source interval: DEV_NOTES.md bytes 193–948, 755 bytes, sha256:1be01d82ebcbb195ec6c87a32268eb1af489ec7cc760a67fe165c6b0ce73107a
- Whole source interval: CHANGELOG.md bytes 207–584, 377 bytes, sha256:5fd5ae72b40438abb24550a2dc742f5790555bcc871c45bc33baec17fc24550f
- Whole source interval: docs/tasks/PLANNING.md bytes 87761–88728, 967 bytes, sha256:deb1eb0344f4ff0ef4e6542272efab66459ebbfd12c42bbba9efe457951f9319
- Whole source interval: docs/book/src/annexes/formula-static-validation.md bytes 5250–7314, 2064 bytes, sha256:99b300682672607e55ead99a8457e87ce0f11f2a6bfaa97e111d83c755f1f04f
- **Retrieval:** bytes below the first rule and blank line; original source terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.6.4.5.2.a`
  Status: `done`
  Goal: D260 preserve original envelope/placement causes through ClosureError; exact red/green,
  unchanged validation/schema, current book/defect records and whole528 continuity.
  - [x] REPRODUCE / ISSUE: cargo test -p sc-core --test closure_contract
    closure_envelope_error_keeps_original_cause -- --exact:0/1 rc=101; placement equivalent0/1 rc=101.
  - [x] ROOT CAUSE (WHY + WHERE): both exact native controls returned rc=101 at source() None;
    empty ClosureError Error impl hid original envelope and boxed placement causes.
  - [x] FIX: cargo clippy -p sc-core --test closure_contract -- -D warnings:rc=0;
    borrow concrete original errors, not Box type; no validation/schema changes.
  - [x] ADDRESSED: both same exact cargo test controls give1/0 rc=0 each; concrete downcast/
    pointer identity and placement-to-AnchorError chain retained.
  - [x] NO REGRESSION: python3 -I -B Git-source comparison: all non-Error-impl production
    bytes identical rc=0; fmt0, glossary132 failures rc=1, exact D250 rows compare rc=0.
  - [x] LOCKSTEP: bash scripts/run_make.sh book:rc=0; scoped publication79/139/1454/2402 rc=0;
    transition_receipts.py --record docs/history/stitchcad-devnotes-part528.md:4 exact spans rc=0.
    promotion: declined (local standard Error repair, canonical D260). Completed STITCHCAD-G1-0377.
## _(2026-10-27 UTC)_ — original Closure error causes

0377 fixes D260: ClosureError's empty Error impl hid its original envelope/boxed placement errors.
Actual Fly/deleted-anchor controls both red0/1 rc101, then green1/0 rc0 after direct cause borrowing.
Concrete type/pointer and placement-to-AnchorError chain stay original; no dynamic Box erasure.
All production bytes outside ClosureError Error impl matchfb122a9. Validation/schema/data unchanged.
Strict affected lint/fmt/book/publication79/139 pass0; D250132 exact prior rows.528 retains closed
Pocket scope/0376/report retirement in4 exact Git spans0. Promotion declined: local standard Error
repair with canonical D260 report. Actual notion-placement inventory next; full CI remains deferred.

## STITCHCAD-G1-0377 — preserve original Closure error causes

- Fix D260 envelope/placement standard source chains; original concrete objects and Anchor chain retained.
- Both exact controls red101/green0; other production bytes identical. Lint/fmt/book/publication pass0.
- D250132 unchanged; closed Pocket/0376/retirement continuity528. Notion-placement collection next.

## D260 — Closure errors hid original envelope/placement causes (fixed0377)

ClosureError's empty Error impl hid stored envelope/placement causes.
Exact closure_contract tests closure_envelope_error_keeps_original_cause and
closure_placement_error_keeps_original_cause: red0/1 rc=101 each, green1/0 rc=0 each through
local_environment.py; target/closure-causes-*-0377.log. Concrete type and same original pointer retained,
including placement-to-AnchorError chaining. All production bytes outside Error impl identical.
P2; owner G1-SLICE.6.4.5.2.a. Closed focused; full CI integration/pre-push.
- [x] ROOT CAUSE: both exact cargo test controls red0/1 rc=101 at missing source(); empty Error impl.
- [x] ADDRESSED: same controls green1/0 rc=0 each after original cause borrowing/as_ref.
- [x] NO REGRESSION: cargo clippy -p sc-core --test closure_contract -- -D warnings rc=0;
  production non-Error-impl Git-source bytes identical rc=0, fmt/book/publication pass0.
## Geometry provider argument checking

The reference point provider takes x and y formula sources; its edge provider takes a len source.
These are operation arguments, not formula constructors. All sources parse before any kind is
resolved, then all kinds resolve in that order. Each argument must be length before the first value
is computed. The scoped payload is defined in [contract5.2.4](../spec/formula-language.md#524-geometry-argument-refusals).

| Provider | Argument formulas | Result |
| --- | --- | --- |
| point | x:1 mm; y:2 mm | exact coordinates1000/2000µm |
| point | x:1.0; y:2 deg | formula_dimension; actual ratio/angle, wanted length/length |
| point | x:1 um / 0; y:1 deg | formula_dimension before the division executes |
| point | x:1.0; y:missing | formula_unbound_name; all kinds are unavailable |
| edge | len:1 deg | formula_dimension; actual angle, wanted length |
| point | x:hypot(3 um,4 um); y:7 um | x contribution hypot; y has no approximation source |

No failed static check changes the provider's result, lazy-cache flag or contribution metadata.
An invalid or unbound child preserves its own error; a provider mismatch reports every resolved
argument kind, its real x/y/len position, and required length. An untaken conditional branch still
has its names checked. Parsing retains literal-input normalization; execution traps cover expression
evaluation, value reads and geometry callbacks.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/geometry_argument_contract.py --mutations
```

Complete97-case/75-payload/12-fault local geometry-reference receipts remain in
`docs/history/stitchcad-devnotes-part491.md`.

D140 is repaired in this local reference adapter. These controls do not validate arbitrary provider
metadata, persisted graphs, whole-recipe runtime rollback, product operation identity or physical
geometry. Product selectors/operation-argument execution remains .5f.3b/G2; complete product static
recipe acceptance is implemented below. The grammar and stable diagnostic token set are unchanged.

End of whole0377/D260 and historical geometry-provider evidence.
