# Sealed archive — whole0348 and historical recipe-input proofs

Immutable historical segment, sealed by leaf `G1-SLICE.6.4.2.d.1` on `2026-10-27` (UTC).

- **Sealed identity:** 139 lines, 10682 bytes, sha256:3da40349a7fbee8f0dfe0cf4e1222cb2e9fe78f55874d59dcc16e2f830c74980
- **Coverage:** complete committed recipe-document task/report/lesson/ledger and old book proof sections from cf74a710f5fadb4856b289442100e9377b55fa97.
- Whole source interval: docs/tasks/G1-SLICE.md bytes 46910–49034, 2124 bytes, sha256:80c560da5e0569aa9b5ba8f887ca1fdf83ae6fc5cb9dc96b9e8a0f7680d372ec
- Whole source interval: docs/tasks/PLANNING.md bytes 86534–87326, 792 bytes, sha256:9e58a4f95f405fae1212de7aaf6653d1b69ab2aa9c1e5c43719e05fdc8ee5c71
- Whole source interval: DEV_NOTES.md bytes 193–1448, 1255 bytes, sha256:094bcef87b0ae0644a81e62a4ae739eb06ca18d556af91a68ab28610dbb255f2
- Whole source interval: CHANGELOG.md bytes 207–829, 622 bytes, sha256:73082231471efc7fbe6db375c8e174d4ea626e44cd473ff7d8283a6d74df74dc
- Whole source interval: docs/book/src/annexes/formula-recipe-inputs.md bytes 10510–13342, 2832 bytes, sha256:0364b6558f9634070aad84be1d48e07a12ebfbc8217d66168239c04f375b0ade
- Whole source interval: docs/book/src/annexes/formula-recipe-inputs.md bytes 14894–17888, 2994 bytes, sha256:80d350cbcc2bd868223ae2363fe024f0a5af0496df666cc54aa5e0cf9c21752f
- **Retrieval:** bytes below the first rule and blank line; original source terminal newlines retained.
- **Write policy:** none; corrections belong in a later record.

---

- ID: `G1-SLICE.6.4.2.c`
  Status: `done`
  Goal: owned immutable authored recipe source and actual normalized/canonical syntax views without
  self-referential storage, invented source spans, execution or static-check proof. Reuse parser and
  literal normalization contracts; retain typed refusal and private replacement boundaries.
  FormulaRecipeDocument owns exact source and typed canonical identity; normalized() reuses actual
  parser/converter with source-borrowed views and typed syntax/literal refusals, no self-reference.
  Own focused native/privacy/lint/book/API checks, D253 stale phase prose and whole0347 continuity499.
  - [x] REPRODUCE / ISSUE: existing normalized arenas borrow original source. New document lifetime
    compile-fail checks refuse detached views2/0, rc=0, target/recipe-document-privacy-0348.log.
  - [x] ROOT CAUSE (WHY + WHERE): canonical recipe bytes retain identity but not source spans/spelling;
    formula_recipe_document_contract compares distinct source/equal identity and original source spans,
    native7 passed/0 failed, rc=0, target/recipe-document-native-0348.log.
  - [x] FIX: immutable owned exact source/typed canonical identity; normalized() uses actual parser/
    converter, retaining source borrowing and original typed syntax/literal refusals, without self-reference.
  - [x] ADDRESSED: python3 -I -B scripts/local_environment.py -- cargo test -p sc-core --test
    formula_recipe_document_contract gives7/0, rc=0; source/order/errors/empty/detached edits/Debug exact.
  - [x] NO REGRESSION: privacy2/strict selected Clippy/fmt/book/publication77/107/1444/2359 pass0;
    target/recipe-document-*-0348.log. D250132 baseline rows exactly equal, delta0; full census rc=1.
  - [x] LOCKSTEP: API vocabulary/phase map/D253/status/index/pointer aligned; continuity499 compares3
    Git spans0; caps unchanged. Test conditional syntax/panic-lint setup corrected/excluded. G1still6/18;
    promotion: declined (canonical identity/source contracts). Commit0348; source registries .2.d next.
    No whole Design/static-name/numeric execution/readiness/integration/CI claim.
## D253 — recipe-input phase footer still defers available static/numeric APIs

Reproduce: rg "arithmetic .5c follows|Numerical binding/evaluation|numeric binding still require" in
annexes/formula-recipe-inputs.md; compare core checked_recipe/bound/numeric public APIs and native
contract receipts0346/0347. Header D252 is corrected, but footer/route prose still calls implemented
binding/static/numeric foundations future work. Impact: readers receive contradictory capability
status. Owner G1-SLICE.6.4.2.c, P2 fixed0348: phase map separates implemented scoped APIs from pending
whole execution/platform acceptance; original input-review proof boundaries retained. Book/publication
77chapters/107API rows/1444source/2359rendered links pass0, target/recipe-document-publication-0348.log.

## _(2026-10-26 UTC)_ — owned original recipe source

0348 FormulaRecipeDocument owns exact source String and typed canonical recipe identity after existing
parser/literal acceptance. Borrowed normalized arenas cannot live inside that object without a
self-reference; normalized() instead reuses the actual parser/converter and returns source-borrowed
views with original spans. It propagates typed errors without panic or invented source positions.
Formatting/unit aliases can share canonical identity while document equality preserves exact text.
Empty drafts accept; unknown calls, duplicate names and incompatible kinds remain syntax content,
not static or execution proof. No canonical identity is used as a source reader or numeric certificate.
Native7/privacy2/strict selected lint/fmt/book/publication pass0. D253 footer/phase map now distinguishes
available static/numeric foundations from pending execution/platform acceptance; historical proof scope
stays intact. D250132 glossary rows remain baseline, delta0. Whole0347 continuity499 matches3 exact Git
spans. Promotion: existing canonical identity/source contracts. G1still6/18; current source registries
.6.4.2.d follow. Full Design/readiness/execution/integration/CI remains unverified.

## STITCHCAD-G1-0348 — own exact authored recipe source and identity

- Add immutable FormulaRecipeDocument, retaining source/spans and typed canonical identity without self-reference.
- Actual normalized views borrow owned text; original syntax/literal refusals propagate. No name/kind/execution proof.
- Native7/privacy2/strict selected lint/fmt/book/publication pass0; D253 stale phase prose corrected.
- D250132 glossary rows remain exactly baseline. Whole0347 continuity499 matches3 Git spans; caps unchanged.
- G1still6/18; source registries .6.4.2.d next; full Design/readiness/execution/integration/CI pending.

Ten public identity contracts check sixteen authored statements/nine complete sources, all55
independently authored expression rows through all three operand roles (165 controls), and100
independent Decimal/Fraction rows through those roles (300 controls). The actual four normative
book/decision examples, alias equality/order distinctions, two actual cross-type text collisions,
ownership/privacy/extraction and large sources are covered. On64KiB stack, the full4096-statement,
two256-node/16-if-level operands per assertion serialize to exact independently composed bytes;
100000-character names,50000 grouping levels and deepest/widest calls also retain exact identity.
These are flat-shape/byte contracts, not a browser, performance or production certification.

Five compile-fail docs check private construction and all three cross-domain equality pairings;
two runnable API docs check source-independent usage. Twenty-one actual serializer faults compile
and fail public assertions, covering headers, operands, envelope/order/count/spacing, empty identity,
terminal newline, opaque Debug and exact extraction. The direct entry plans eight inputs before
effects: implementation/current test/four imported TSVs/book grammar/byte decision. Ordinary mode
declares twenty-two diagnostic files and one mutable source; classifier-only declares zero files
or mutable sources and invokes no native child. Both modes preserve prepared local stores.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/canonical_recipe_mutations.py --classifier-only
```

A failed-body thread panic must name a current assertion macro path/line/byte column. Custom messages
pass; nonthread mentions, passing names and labelled extraction panics refuse. Two independent
complete captures/eighteen physical refusals and twenty-four mode omissions preserve all21 original
cases/full-target arguments/restoration; an independent legacy-regex fault distinguishes nonthread
noise. Copied classifier-only mode accepts linked readonly input/unused log aliases without source/
output writes. Every actual failed body reaches a current macro, including small-stack workers;
independently authored failed-test sets remain exact. Thirty-one frozen inputs/eighteen unchanged
Rust files/six compile-time inputs/twenty-two local single-link logs and the current full ten-test
executable restore exactly. Dispatch captures and real native behavior are verified separately.
Existing fixtures retain their independent recursive-reference checks; no independent whole-recipe
parser or numerical evaluation proof is claimed. [Coupled review .3f.2](#coupled-input-and-identity-review)
is complete below.

```bash
cargo test -p sc-core --test formula_canonical_recipe_contract
python3 -I -B docs/tasks/artifacts/formula_structure/canonical_recipe_mutations.py
```

## Verification scope

Eight public contracts check sixteen independently authored statement/header/operand-byte rows,
nine whole sources, original spans/source borrowing/privacy/Clone and exact later-statement failures.
One hundred independent Decimal/Fraction literal rows pass through all three operand roles, supplying
300 whole-recipe input controls. Both width and rounded scalar refusals retain their typed families.
Earliest refusal/source chaining, unknown calls, duplicate/forward names, raw turns/wide magnitudes,
incompatible annotations and unevaluated operators retain the stated input-only scope.

On a64KiB thread stack,4096 assertions each hold two256-node operands with16 if levels. Parsing,
conversion, syntax drop, normalized Clone and drop preserve all2097152 nodes. This checks the declared
flat shape and separate bounds for that maximum; it is not a memory-performance or browser certificate.
Six compile-fail docs check private fields/source/view lifetimes, and two runnable API docs check usage.

Seventeen actual production faults compile and fail public assertions: name/header metadata,
declared kind/tolerance, operand coverage/error role, order/count/ordinal, spans, privacy and both
error causes. The direct entry plans seven inputs before effects: three implementations, current
public test and its three imported TSVs. Ordinary mode declares eighteen logs and three mutable
sources; classifier-only declares zero files or mutable sources and invokes no native child. Both
modes retain local store/directory preparation.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/normalized_recipe_mutations.py --classifier-only
```

A qualifying failed-body thread panic must name a current assertion macro path/line/byte column.
Custom messages pass; passing names, labelled extraction panics and nonthread location mentions
refuse. Two independent complete captures, sixteen physical refusals and actual input/file/mutable/
mode/stores/restoration/classifier faults calibrate the plans, including each of the three mutable
declarations. A copied classifier-only entry accepts linked read-only input and ignores unused log
aliases without source/output writes. Seventeen original full-target faults reach current macros
in every failed body, including the small-stack worker; independently authored failed-test sets and
source bindings remain exact. Twenty-seven frozen inputs/seventeen unchanged Rust files/three
compile-time TSVs/eighteen local single-link logs and the current full eight-test executable restore
exactly. Dispatch capture and real native behavior are verified separately.
Authored fixtures are checked by the independent recursive reference, with semantic execution trapped.
Test-side header/operand byte composition is inspection, not a product statement/recipe serializer.

```bash
cargo test -p sc-core --test formula_normalized_recipe_contract
python3 -I -B docs/tasks/artifacts/formula_structure/normalized_recipe_mutations.py
```

End of whole0348 and historical recipe-input proof continuity.
