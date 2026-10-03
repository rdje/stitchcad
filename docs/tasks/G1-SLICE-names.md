# G1-SLICE — formula namespace and signature evidence

Evidence sibling of [G1-SLICE](G1-SLICE.md), owned by [recipe lanes](G1-SLICE-recipes.md).
Completed protocols and receipts retain original bytes; current implementation work stays there.

## Completed initial namespace protocol — preserved from 4e0d0bf

- ID: `G1-SLICE.5b.2c.2`
  Status: `done`
  Goal: implement checked initial namespace after .2c.1b, including ordered-pair collisions,
  immutable reserved metadata and source-preserving typed refusals. Acceptance: no value/state or
  geometry reads, no silently overwritten duplicate or recipe-order bypass. Finalize API protocol
  after diagnostic ruling. Work unit: `STITCHCAD-G1-0087`; predecessor1972f57 clean/no jobs.
  Pre-code protocol: contract2/3/3.1/5.2.1/4.1 and metadata/declaration/normalized/name/ref APIs
  reviewed. MachineToken already enforces the three keywords; borrow its exact validated names.
  New opaque FormulaInitialDeclaration accepts only Input/LengthInput/Point/Edge via TryFrom;
  rejected Recipe/Reserved returns the original declaration, preventing future-let/context seeding.
  Build immutable FormulaNamespace from ordered initial declarations; seed all eight fixed reserved
  metadata entries irrespective of provider availability. BTreeMap entry checks precede insertion;
  first collision refuses without a partial namespace. Preserve input order in first-error sources.
  Typed FormulaNamespaceError distinguishes reserved attempts (formula_rebinding) and authored
  collisions (formula_ambiguous_name), including equal origins; explicit name/source views retain
  original IDs/records/refs. Opaque Debug and token-only Display expose no authored payload/state.
  Exact-size declaration iteration is lexical for metadata inspection; no recipe/evaluation order
  authority. Checked name reads/prior recipe binding remain .2d; no state/value/geometry query.
  Verify six initial source domains, all five LengthStates/three scalar domains/six kinds, exact
  creator/tag/borrow identities, all8 reserved refusals, every ordered collision pair and privacy/
  lifetimes. Actual compiled index/admission/context/collision/source/privacy faults must fail
  public assertions and restore bytes. Strict native/WASM, focused reference/book/ledger/gate;
  exact completed .1b/oldest ledger/lesson retention keeps existing bounds. Per-leaf commit.
  D132/D133 product-publication defects owned here: raw generic was hidden as an HTML tag while
  publication topology probes passed. Fix markup and reject actual mdBook warning output before
  claiming publication; copied-book unclosed-generic failure and repaired baseline must prove the
  guard. Keep all existing source/rendered-link/status controls; no scope/cap/diagnostic waiver.
  Verification: ten public contracts/17 actual body reds, strict621 native tests/51 result groups,
  WASM three libraries and reference/language16/publication10/ledger9+13 controls pass, rc=0.
  D132/D133 fixed; complete receipts below. Commit: `STITCHCAD-G1-0087`.

## Completed initial namespace receipts — preserved from 4e0d0bf

## Initial namespace receipts — .5b.2c.2,2026-10-03 (UTC)

Focused public contract:10 tests plus three privacy/lifetime compile-fail examples pass, rc=0.
namespace_mutations.py:17 actual compiled body assertion reds, exact original bytes restored,
rc=0; classifier refuses compiler/unwrap-only noise. Strict make check:621 passed/51 result groups,
fmt/clippy -D warnings pass, rc=0; final make wasm builds all three libraries, rc=0. Full reference
structural, language16 and ledger9/13 pointer controls pass, rc=0. Publication10 controls pass,
rc=0;55 chapters/36 scoped API rows/1143 source links/1775 rendered links. D132/D133 actual
malformed generic warning refuses BOOK_BUILD_WARNING; quoted repair renders exact generic.
Predecessor protocol23lines2027B SHA36198552…, receipts28lines2537B SHAbbe37e98…, checklist
21lines1879B SHAf5736386… retained byte-identically from1972f57. Oldest ledger13lines1176B
SHAce5bfa8a… and lessons11lines903B SHAa1c4d338…/12lines1018B SHA92241e4a… sealed exactly.
Original D132/D133 report15lines1304B SHA2c4850a6… sealed; no prior archive changed.
Retention:230 logical records/44 working Markdown/10412 decoded lines/785417 decoded bytes/
373931 resident bytes, rc=0. Fresh materialization:10open/122unique sealed, zero duplicates/overlap;
D18 intentionally unassigned per canonical PLANNING census. Tree census:10lanes/13trees/10siblings/
zero unowned/orphans/dead links, rc=0. G1 remains5/18; ordered reads/bindings .2d next.

## Completed initial namespace checklist — preserved from 4e0d0bf

### G1-SLICE.5b.2c.2 — checked initial product namespace

- [x] **REPRODUCE / ISSUE** — sourced declarations can describe recipe/reserved metadata but lack
  initial admission/collision authority. Public namespace contract →10 tests pass, rc=0; actual
  admission/order/source/privacy faults establish the missing boundary before acceptance.
- [x] **ROOT CAUSE (WHY + WHERE)** — direct unfiltered declarations could bypass recipe order or
  overwrite names. namespace_mutations.py →17 actual compiled body assertion reds/exact restore,
  rc=0; recipe/reserved admission, context hiding, first-error order and source loss falsified.
  Initial projection, ordered entry checks and fixed reserved population are the owned guards.
- [x] **FIX** — opaque initial-source projection, immutable checked namespace and source-bearing
  typed errors. Strict lint rejects the initial128B inline collision error; one boxed metadata pair
  keeps failures compact without copying canonical records. No lint allowance or numeric query.
- [x] **ADDRESSED (verified)** —10 public contracts/17 actual compiled body assertion reds,
  rc=0: six initial domains, three scalar domains/six kinds, five states, exact refs/borrows,
  36 ordered origin collisions/equal origins and200 reserved attempts. Byte-exact source restore.
  Initial fault classifier refused unwrap-only red; explicit body refusal assertions repair tests.
- [x] **NO REGRESSION** — make check →621 tests/51 result groups, strict lint/fmt green; make wasm
  →three libraries build; structural/reference/language16/publication10/ledger9+13 pass, rc=0.
  D132/D133 actual renderer warning exposed topology-only acceptance; repaired exact generic and
  copied-book warning refusal/repaired baseline verify guard, rc=0. Grammar/values unchanged.
- [x] **LOCKSTEP** — declaration chapter/examples/API/status/README/live scope aligned; checked
  ordered reads/types/whole graphs remain .2d–.4. Exact prior task/ledger/lesson retained within
  existing bounds. promotion: declined (existing source ownership/metadata/order/diagnostic rules).

## Completed exact initial reads protocol — preserved from 19900d0

- ID: `G1-SLICE.5b.2d.1`
  Status: `done`
  Goal: checked exact-name initial metadata reads without reading values or optional providers.
  Work unit: `STITCHCAD-G1-0088`; predecessor4e0d0bf clean, message empty/untracked, no pending jobs.
  Pre-code: contract3/3.1/4.1/5.2, declarations/reserved/namespace/MachineToken and normalized
  name-node views reviewed; public declarations and canonical records stay sole metadata sources.
  Public FormulaNamespace::resolve borrows a validated MachineToken and returns the existing
  FormulaDeclaration, retaining its own source lifetime. Internal exact-str resolver accepts only
  parser-validated names in later semantic composition; no trimming/alias/default/geometry query.
  Opaque FormulaUnboundName borrows the exact query; private construction, token formula_unbound_name,
  explicit name and complete nine closed searched origins (including empty recipe origin before
  statement1). Unknown supplied records and absent optional contexts still have declared kinds.
  Default Debug/Display omit authored payload; no fabricated statement/canonical-expression context.
  Verify all scalar/length/geometry/reserved sources, exact records/identities, distinct names with
  equal kinds, exact query versus declaration lifetimes, missing/near names and immutable failures.
  Actual compiled lookup/fallback/source/origin/token/privacy faults must fail public assertions;
  compiler/unwrap noise refuses and source restores. Strict native/WASM, focused book/reference/
  language/ledger/retention/census/gate. Exact completed namespace evidence/oldest live records move
  to bounded sibling/segments; Knowledge Map orientation trimmed with every route preserved.
  No prior archive or ceiling changes. Per-leaf commit.
  Verification: seven public contracts/two negative doctests/nine actual body reds and prior17 faults,
  strict630 native tests/52 groups, WASM three libraries and focused controls pass, rc=0.
  Full receipts below. Commit: `STITCHCAD-G1-0088`.

## Completed exact initial reads receipts — preserved from 19900d0

## Exact initial name-read receipts — .5b.2d.1,2026-10-03 (UTC)

Public read contracts7/two private-lifetime doctests and nine actual compiled body assertion reds,
exact source restore, rc=0. Prior17 namespace faults rerun against composed code: body reds, rc=0;
Display anchor scopes its actual error impl; both classifiers refuse name/expect/compiler noise.
make check:630 passed/52 result groups and strict fmt/clippy green, rc=0; make wasm:three libraries,
rc=0. Full structural/reference, language16, publication10 and ledger9/13 pointer controls pass,
rc=0. Book55 chapters/37 scoped APIs/1143 source/1777 rendered links; actual warning/refixed generic
still checked. Prior protocol30lines2728B SHAa67c4b56…, receipts17lines1483B SHA426d31f4…,
checklist23lines2105B SHAa30b863a… retained byte-exactly from4e0d0bf in bounded names sibling.
Oldest ledger14lines1261B SHAa370b662…/lesson17lines1539B SHA06d7a8cd… sealed exactly.
Retention232 logical records/46 working Markdown/10465 decoded lines/789301 decoded bytes/
377815 resident bytes, rc=0; materialized defects10open/122unique sealed/zero duplicates or overlap.
Tree census10lanes/13trees/11siblings/zero unowned/orphans/dead links, rc=0; current examples
updated from actual census. G1 remains5/18; .2d.2 ordered actual binding scope next.

## Completed exact initial reads checklist — preserved from 19900d0

### G1-SLICE.5b.2d.1 — exact initial metadata reads

- [x] **REPRODUCE / ISSUE** — the checked namespace has declarations but no exact typed read or
  absent-name arguments. Public formula_name_read_contract →seven tests pass, rc=0; this boundary
  returns borrowed original metadata or exact-query diagnostics without execution authority.
- [x] **ROOT CAUSE (WHY + WHERE)** — ad hoc caller iteration could repair/default a missing name
  or consult source values. name_read_mutations.py →nine actual compiled body assertion reds,
  rc=0; wrong source/alias/fallback/value read/domain/token/name/privacy faults are detected.
- [x] **FIX** — validated public resolve, internal exact parser-name lookup and opaque unbound
  error with exact query/closed searched origins. Source/query lifetimes remain independent.
- [x] **ADDRESSED (verified)** — seven public contracts/two negative doctests/nine actual faults,
  rc=0: all scalar/length states/geometry/reserved sources, exact identities, near names, immutable
  failures and private borrow/format contracts. Exact source restored; no value/context query.
- [x] **NO REGRESSION** — make check →630 passed/52 groups, fmt/clippy strict green; make wasm
  →three libraries; prior17 actual collision faults/structural/reference/language16/publication10/
  ledger9+13 controls pass, rc=0. Flat namespace and grammar stay unchanged.
- [x] **LOCKSTEP** — README/book/examples/API/live/frontier align; prior recipe binding scope .2d.2
  and types/whole graphs .3/.4 remain owned. Exact prior evidence and oldest live records retained.
  promotion: declined (existing exact spelling/canonical borrows/privacy/no-default contracts).

## Completed static review protocol — preserved from 19900d0

- ID: `G1-SLICE.5b.1c`
  Status: `done`
  Goal: close full static obligation map against contract2/3/5/6/9 and grammar5/6/7 after .1a/.1b;
  named exclusions/envelope precedence and prior syntax limits in both directions. Independently
  review all actual worked/refusal examples; any remaining contract ambiguity settled before code.
  Acceptance: complete map, actual static/no-execution evidence and precise remaining numeric/
  geometry/runtime/production proof boundaries; safe implementation protocols for .5b.2–.4.
  Pre-code review: inspect normative contract2/3/4.1/4.3/5/6/9 and grammar1/5/6/7 against actual
  parse/infer/namespace/preflight and all worked/refusal rows. Reuse independent4032/1139/196
  matrices, map every obligation/remaining proof owner explicitly, and exercise exclusion spellings
  through actual public reference entry points before accepting any diagnostic contract. Record
  any ambiguity with concrete source/result evidence; do not silently redefine exclusions or
  claim typed product errors, persistence, numerical execution or physical geometry from reference
  token checks. Detailed review evidence/proposals live in G1-SLICE-evidence.md.
  Children .1c.1 (complete map and D124 diagnostic proposal), .1c.2 (ruling and review closure),
  owned in G1-SLICE-evidence.md. D124 preserves current grammar/keywords and closes recognition.
  Verification: full signature4032/namespace1139/recipe196/review100 controls and actual reds verified.
  Commit: `STITCHCAD-G1-0082`.

## Completed ordered scopes protocol — preserved from 63c0c7d

- ID: `G1-SLICE.5b.2d.2`
  Status: `done`
  Goal: derive prior-binding scope from actual normalized recipe/order, truthful binding refusals.
  Work unit: `STITCHCAD-G1-0089`; predecessor19900d0 clean/message empty/untracked/no jobs.
  Pre-code contract3/3.1/4.1/5.2.1 and normalized recipe/declaration/namespace APIs reviewed.
  Add opaque FormulaNameCursor consuming a checked initial namespace and borrowing one actual
  normalized recipe. Public current scope exposes actual statement/index, initial plus earlier let
  metadata; current/future names remain absent. No caller supplies a position/declaration/recipe
  binding, no final namespace escapes, no mutation while a borrowed scope remains live.
  current and advance_metadata check binding header collision before scope/insertion; assertion
  labels never declare scalar values. Failed advance retains unchanged position/prefix and cannot
  skip past a refusal. End is fused, including empty and4096-statement recipes. Initial namespace
  remains a distinct construction boundary; cursor grants no expression/type/value acceptance.
  Metadata advance records only actual current let annotation/location. Type/whole validators .3/.4
  must check all operands before using it; missing/invalid RHS is still unvalidated syntax here.
  Extend FormulaNamespaceError with RecipeRebinding carrying boxed actual prior/attempted sources;
  reserved attempts retain fixed metadata and real current ordinal/spans, input collisions remain
  ambiguity even with same kinds. Whole diagnostics carry actual sources; no invented initial index.
  Verify all kinds/domains, assertion gaps, exact spans/owner refs, self/forward no-reordering,
  repeated/reserved/input collisions/first refusal, no value/provider/geometry access, private
  fields/query lifetimes/scopes, immutable failed advance, deterministic4096 on64KiB stack.
  Actual compiled cursor/source/order/assertion/rebinding/privacy faults must fail body assertions;
  classifier refuses compiler/unwrap noise and restores all source bytes. Strict native/WASM,
  reference/language/book/ledger/retention/census/gate and exact prior evidence/oldest records.
  Keep bounded book/task/live surfaces; no grammar/cap/diagnostic waiver. Per-leaf commit.
  Verification: eight public contracts/five negative examples/18 actual faults plus prior17/9,
  strict643 native tests/53 groups/WASM and focused controls pass, rc=0; receipts below.
  Commit: `STITCHCAD-G1-0089`.

## Completed ordered scopes receipts — preserved from 63c0c7d

## Actual ordered name-scope receipts — .5b.2d.2,2026-10-03 (UTC)

Public8/five negative examples/18 actual compiled body assertion reds pass, both sources exact,
rc=0; first compiler-only fault refused, repaired owned-error fault yields actual body red.
Prior namespace17/read9 actual faults rerun and restored; all classifier anchors/noise controls
pass, rc=0. make check:643 passed/53 result groups and strict fmt/clippy green; make wasm:three
libraries, rc=0. Full structural/reference/language16/publication10/ledger9+13 controls pass,
rc=0;56 chapters/39 APIs/1156 source/1798 rendered links. Original predecessor protocol23lines/
2008B SHAe601e894…, receipts15lines1315B SHA1d0897eb…, checklist19lines1695B SHAeb355b94…
retained exactly from19900d0. Older complete review protocol also remains byte-exact in names
sibling. Oldest ledger9lines795B SHA480c7c1b…/lesson14lines1220B SHA11170a8b… sealed exactly.
Retention234 logical records/48 working Markdown/10510 decoded lines/792394 decoded bytes/
380908 resident bytes, rc=0; fresh defects10open/122unique sealed/zero duplicates or overlap.
Tree census10lanes/13trees/11siblings/zero unowned/orphans/dead links, rc=0. Namespace foundation
.5b.2/.2d closes; G1 remains5/18. Product expression type/signature validation .5b.3 next.

## Completed ordered scopes checklist — preserved from 63c0c7d

### G1-SLICE.5b.2d.2 — actual ordered declaration metadata

- [x] **REPRODUCE / ISSUE** — detached let metadata has no active ordered scope. Public
  formula_ordered_names_contract →eight contracts pass, rc=0; actual owner/order/refusal sources
  and both4096 traversal/drop boundaries verify metadata staging before expression acceptance.
- [x] **ROOT CAUSE (WHY + WHERE)** — arbitrary declaration/ordinal injection could reveal future
  bindings or lose prior refusal locations. ordered_name_mutations.py →18 actual compiled body
  assertion reds/exact two-source restore, rc=0. Initial compiler-only fault correctly refused.
- [x] **FIX** — opaque actual-recipe cursor and borrowed current scope; header refusal before
  scope/insertion, assertion positions retained without value binding, failed advance unchanged.
  RecipeRebinding retains both actual sources/indices/spans; no final initial namespace escapes.
- [x] **ADDRESSED (verified)** — eight public contracts/five private/lifetime examples/18 body reds,
  rc=0: self/forward/labels, all six annotations/48 reserved attempts, exact refs/ordinals/spans,
  collision kinds, unchanged failure/owner views/privacy and64KiB max recipes. No values queried.
- [x] **NO REGRESSION** — make check →643 passed/53 groups, strict fmt/clippy green; make wasm
  →three libraries; prior17/9 actual faults/structural/reference/language16/publication10/ledger9+13
  controls pass, rc=0. Source byte-exact; metadata grants no expression/whole/runtime acceptance.
- [x] **LOCKSTEP** — bounded book annex/examples/API/live/frontier and original task/oldest records
  align; type/signatures .3 and whole graph .4 retain owners. promotion: declined (existing order,
  typed borrowed sources and no-partial-proof contracts).

## Completed reference signature protocol — preserved from 63c0c7d

- ID: `G1-SLICE.5b.1a`
  Status: `done`
  Goal: independent complete operator/function/selector signature review of the actual reference;
  enumerate all eight operand kinds, positional product/quotient rules, generic arithmetic kinds,
  closed arities, conditional kind rules and all eight reserved candidates for tolerance roles.
  Pre-code contract: grammar5/5.1/6/6.1/7 and contract2/3.1/5.3/6 govern. Author expected tables
  independently, compare table populations in both directions, then exercise actual parse/infer
  with numeric reads and geometry resolution trapped. Every arithmetic kind/generic signature,
  commutative product versus directed quotient, wrong arity/kind and both static branches tested.
  Prove controls fail on actual in-memory reference guard faults; preserve producer bytes.
  Diagnose/repair observed signature defects here, with exact before/after refusals. No namespace,
  typed diagnostic payload, runtime domain, whole-recipe atomic preflight or product validator claim.
  Acceptance: full expected/actual population equality, exhaustive bounded matrix and named defects
  fixed; focused current reference/syntax/book controls green, live/book/task lockstep and commit.
  Verification:4032 actual parse/infer cases/22 closed names; twelve compiled body assertion
  faults, no numeric/environmental/geometry reads; full reference and language16 green, rc=0.
  Commit: `STITCHCAD-G1-0073`.

## Completed reference namespace protocol — preserved from 63c0c7d

- ID: `G1-SLICE.5b.1b`
  Status: `done`
  Goal: all nine origins/eight reserved names, spelling/context, collision/rebinding/forward names
  and static whole-recipe preflight in the reference, with independent declaration fixtures.
  Acceptance: both-origin collision evidence, whole-recipe no-value-access proof, ordered namespace
  and header checks; resolve discovered defects before trusting the reference for .5b.2/.4.
  Typed production diagnostic arguments retain their .5b.2/.4 owners; no runtime/MCP claim.
  Children .1b.1 (namespace/header static phase), .1b.2 (whole-recipe preflight);
  detailed pre-code protocols/receipts in G1-SLICE-evidence.md.
  Prerequisite .1b.0/.0v:64-file history capacity, exact retained window and observed CI;
  owned in G1-SLICE-evidence.md, no domain-scope pivot or limit increase.
  Verification:1139 namespace/196 whole-source cases;13 namespace/14 whole-preflight actual
  assertion reds, worked replay and per-recipe measurement; no execution/value/geometry reads.
  Children .1b.1/.1b.2 done; full .1c review closes at .1c.2. Commit: `STITCHCAD-G1-0076`/`STITCHCAD-G1-0077`.

## Completed operator signatures protocol — preserved from dc346b4

Exact payload: 23lines/1976B, SHA2567ea7b24e149ed358afc7cbfbd33157fb450bf7e978e5920a320baf3d4f820b54.

- ID: `G1-SLICE.5b.3a`
  Status: `done`
  Goal: closed pure operator kind signatures from normative grammar5/5.1 before expression checking.
  Work unit: `STITCHCAD-G1-0090`; predecessor63c0c7d clean/message empty/untracked/no jobs.
  Pre-code full grammar1/2/5/5.1/6/7, contract2/4.1/5.2, FormulaBinaryOperator/FormulaKind and
  normalized unary/binary views reviewed. Preserve existing syntax/operator tokens and type worlds.
  Add FormulaUnaryOperator Negate/Square with canonical node tokens and result_kind metadata.
  Existing FormulaBinaryOperator gets result_kind for all ten variants plus arc_length_hint only
  for refused angle×length in either order. Return Option<FormulaKind>; absence is a signature
  refusal, not numeric division/domain or accepted expression. Typed contextual errors remain .3c.
  Arithmetic T is length/angle/area/ratio/count; negation excludes count, square permits length/
  ratio/count. Addition/subtraction/comparisons require equal arithmetic kinds. Multiplication
  follows commutative table; quotient preserves authored direction, including count/ratio versus
  ratio/count and integer count/count→ratio. No implicit promotion or value/geometry query.
  Independently enumerate every8-kind unary and64 ordered binary pairs for every operator, compare
  full closed normative rows in both directions and exact hint population. Named dimensions/data
  and static error integration remain .3c; no numeric or graph acceptance claimed from metadata.
  Actual compiled operator/result/commutativity/direction/count/Boolean/hint faults must fail body
  assertions and restore source. Strict native/WASM, focused reference/book/ledger/retention/gate;
  exact predecessor evidence/oldest records retained within bounded surfaces. Per-leaf commit.
  Verification: four public contracts/656 kind cases/14 actual body reds, strict647 native tests/54
  groups/WASM and focused controls pass, rc=0; receipts below. Commit: `STITCHCAD-G1-0090`.


## Completed operator signatures receipts — preserved from dc346b4

Exact payload: 16lines/1393B, SHA25618db956df6381c807e0a5a078e28af685960d2069b071f017f9566a5f2e381d3.

## Operator signature receipts — .5b.3a,2026-10-03 (UTC)

Public4 contracts cover16 unary/640 ordered binary cases and full actual normative rows;7 unary/
71 binary acceptances,17 products/14 quotients/2 hints, all12 symbols match unchanged independent
canonical serialization. Fourteen actual compiled body assertion reds restore source byte-exact,
rc=0; watched anchors/classifier reject compiler/unwrap/name noise. make check:647 passed/54
result groups, strict fmt/clippy green; make wasm:three libraries, rc=0. Full structural/reference/
language16/publication10/ledger9+13 controls pass, rc=0;57 chapters/40 APIs/1161 source/1811 rendered
links. Prior protocol28lines2466B SHA78912bdc…, receipts15lines1301B SHA69044ab9…, checklist
20lines1786B SHA09862a29… retained byte-identically from63c0c7d. Older reference protocols17lines/
1427B SHA8c644d12… and14lines1124B SHA8890c07e… retained exactly. Oldest ledger9lines773B
SHAdf9a2d7c…/lesson15lines1296B SHAffe6d2c5… sealed exactly, no previous archive changes.
Retention236 logical records/50 working Markdown/10556 decoded lines/795551 decoded bytes/
384065 resident bytes, rc=0. Fresh defects10open/122unique sealed/zero duplicate/overlap;
tree census10lanes/13trees/11siblings/zero unowned/orphans/dead links, rc=0. G1 remains5/18;
built-in/selector signatures .5b.3b next, then bounded expression/whole graph .3c/.4.

## Completed operator signatures checklist — preserved from dc346b4

Exact payload: 18lines/1453B, SHA25673cc1a1e3a0239ef31a6a0e5688b4084eab2ef5577733d6689e514b7ad4fdce7.

### G1-SLICE.5b.3a — closed operator kind signatures

- [x] **REPRODUCE / ISSUE** — normalized operator syntax lacks kind-signature metadata. Public
  formula_operator_signature_contract →four contracts/656 kind cases pass, rc=0; canonical bytes
  and complete actual normative rows independently verify the closed operator vocabulary.
- [x] **ROOT CAUSE (WHY + WHERE)** — inferred promotion or symmetric division can erase declared
  kinds/direction. operator_signature_mutations.py →14 compiled body assertion reds/exact restore,
  rc=0; count/Boolean/result/order/hint faults demonstrate the needed explicit matrix.
- [x] **FIX** — pure unary/binary result-kind metadata, exact tokens and angle×length hint;
  no operand/source value or numeric-domain query. Contextual errors remain .3c.
- [x] **ADDRESSED (verified)** —16 unary/640 binary cases/actual rows/12 canonical symbols and14
  actual body reds pass, rc=0;7/71 accepted,17 products/14 quotients/2 hints, exact source restore.
- [x] **NO REGRESSION** — make check →647 passed/54 groups, strict fmt/clippy green; make wasm
  →three libraries; reference/language16/publication10/ledger9+13 controls pass, rc=0.
- [x] **LOCKSTEP** — bounded book/API/live/task scope and exact prior/oldest records align;
  full function/expression/whole proofs remain .3b–.4. promotion: declined (existing closed kind
  signatures, operand order and no-implicit-conversion contracts).


## D134/D135 guidance checklist — .5b.3a.1

- [x] **REPRODUCE / ISSUE** — actual parse/infer on both angle/length quotients emitted arc_length
  advice; multiplication did too. Read-only reproduction rc=0, before repair. Review chapter still
  named namespaces pending after0087–0089 and operator signatures pending after0090.
- [x] **ROOT CAUSE (WHY + WHERE)** — reference absent-pair guard ignored operator; independent
  matrix required the same wrong quotient hint. static_signature_contract.py --mutations →4032
  actual cases/14 compiled assertion reds/unchanged producer bytes, rc=0, including restored
  quotient-regression and suppressed-product controls. Guidance predicate matches the actual advice
  phrase, distinguishing an arc_length call's name from advice about a product.
- [x] **FIX** — multiplication-only reference hint; all binary operator/pair cases assert advice
  presence and absence. Review links exact implemented metadata scopes and remaining .3b–.4.
- [x] **ADDRESSED (verified)** —4032 parse/infer cases/14 actual body reds pass, rc=0. Grammar bytes
  equal dc346b4; signatures/tokens/numeric behavior unchanged. D134/D135 original reports retained
  exactly:15lines1306B SHAe4113b96c8a5f3d29e8fe2cc7fe2de5dfe5a9ef0c80f73086f17d22ec8664ddb.
- [x] **NO REGRESSION** — complete structural/reference, language16, publication10, ledger9+13,
  coverage10lanes/13trees/11siblings/zero gaps and retention239records/53workingMD pass, rc=0.
  Publication57chapters/40API/1163source/1813render links; no Rust changes or Cargo mutation.
- [x] **LOCKSTEP** — frontier resumes .3b; live10open/124sealed, G1 remains5/18. First scratch
  census omitted the heading-form D131; complete three-form marker census proves IDs1–135
  except intentionally unassigned D18, zero duplicates/overlap. Prior protocol23lines1976B
  SHA7ea7b24e…/receipts16lines1393B SHA18db956d…/checklist18lines1453B SHA73cc1a1e… retained
  byte-exact. Oldest ledger10lines878B SHA32b87b67…/lesson13lines1106B SHAb0a427bf… sealed.
  promotion: declined (existing dimensional signature and precise proof-boundary contracts).

## Completed guidance repair protocol — preserved from c5d4579

Exact payload:16lines/1347B SHA2562f5f7d166c0e8720349d8b27bf6fe5fd2bb401cdb99f4440cffa832324e34de2.

- ID: `G1-SLICE.5b.3a.1`
  Status: `done`
  Goal: D134 reference guidance and D135 review status match the implemented contract.
  Work unit: `STITCHCAD-G1-0091`; predecessor dc346b4 clean/message empty/untracked.
  Tools-first actual parse/infer reproduced arc_length guidance on both angle/length quotients,
  although grammar5.1 names an angle-times-length product and product operator metadata restricts
  the hint to Multiply. Existing static matrix incorrectly requires that same quotient guidance.
  Own the small blocking repair before .3b: restrict reference hint to multiplication; verify its
  presence AND absence independently over every binary pair/operator, actual compiled in-memory
  regression and missing-hint faults, unchanged source on disk after controls. No grammar change,
  accepted signatures, tokens or arithmetic behavior change. Retain exact .3a protocol/receipts/
  checklist and oldest live records before bounded-document growth; focused structural/language/
  publication/ledger/retention/coverage/gates and per-leaf commit. No Rust mutation. D135 touched-annex metadata status is corrected with precise public API links.
  Verification: static4032 cases/14 actual reds, focused structure/language16/publication10/
  ledger9+13/retention/coverage pass, rc=0; exact grammar unchanged. Commit: `STITCHCAD-G1-0091`.

## Built-in signatures checklist — .5b.3b

- [x] **REPRODUCE / ISSUE** — normalized call/if syntax lacked product signature metadata.
  formula_builtin_signature_contract →four tests/680702 cases pass, rc=0; all actual24 normative
  rows and22 unchanged canonical names independently match the closed vocabulary.
- [x] **ROOT CAUSE (WHY + WHERE)** — ordinary length cannot retain within's symbolic class role,
  and generic T must remain homogeneous arithmetic. builtin_signature_mutations.py →21 actual
  compiled body assertion reds/exact restore, rc=0; value/class/kind/arity/branch/order/selector
  faults prove both role and kind boundaries. Provider/value/geometry reads are absent by API type.
- [x] **FIX** — closed22-name registry/category/arity metadata and explicit Value/Tolerance
  descriptors, pure result_kind with shared arithmetic predicate. No new keyword/grammar/envelope
  classification or accepted-expression proof; contextual diagnostic and dependencies remain .3c.
- [x] **ADDRESSED (verified)** —every13-member kind/class tuple at arities0–4, all24 actual rows,
  exact tokens/categories/arity samples and wide variadics pass;21 actual body reds/exact source
  restore plus eight restored public operator/built-in tests pass, rc=0. All14 existing actual
  operator faults rerun successfully after sharing the predicate; restored bytes confirmed.
- [x] **NO REGRESSION** — make check:651 passed/55 groups, strict fmt/clippy; make wasm:three
  libraries; structural/reference/language16/publication10/ledger9+13/coverage pass, rc=0.
  Book58chapters/44API/1169source/1829render links; complete normative grammar byte-identical.
  Draft normalizer getter corrected by actual API census; oracle indexing/panic lint corrected
  with checked access, no lint waiver. Faults rerun against that final oracle before restoration.
- [x] **LOCKSTEP** — new bounded book/examples/API/live scope align; G1 stays5/18,10open/124sealed.
  Exact predecessor protocol16lines1347B SHA2f5f7d16… retained. Two oldest ledger payloads combined
  without edits:16lines1250B SHA2d8487c5…; lesson10lines838B SHAa83b9679… sealed. No previous archive
  changed. Retention241records/55workingMD/10675decodedlines/803679decodedB/392193residentB,
  rc=0; fresh three-form marker census proves all IDs1–135 except unassignedD18, no duplicates/
  overlap. Tree10lanes/13trees/11siblings/zero gaps; next .3c typed expression interface/checker.
  promotion: declined (existing closed function/selector signatures and symbolic tolerance roles).

## Completed built-in signatures protocol — preserved from c98dc54

Exact payload:25lines/2141B SHA256ad4577309c6ca0115b80877668da72767b217ba884864189925744b47738f38e.

- ID: `G1-SLICE.5b.3b`
  Status: `done`
  Goal: closed built-in/selector signature vocabulary, ordered arities and symbolic tolerance role.
  Work unit: `STITCHCAD-G1-0092`; predecessor c5d4579 clean/message empty/untracked/no jobs.
  Pre-code roadmap ADR-0003/G1, full grammar6/6.1/7, contract2/3.1/4.1/5.2 and normalized call/if,
  FormulaKind/ReservedName/ToleranceName APIs, reference closed22 names/signature matrix reviewed.
  Add closed FormulaBuiltin ALL/token/from_token (exact spelling), category Function/Selector/
  Conditional/ToleranceComparison and Fixed/OneOrMore arity metadata. If remains special syntax;
  lookup metadata neither adds reservations nor overrides named envelope precedence.
  FormulaBuiltinOperand is Value(kind) or Tolerance(existing symbolic class); either has its
  ordinary kind, but only the latter fills within's third role. Ordinary length, arithmetic on a
  class and size names cannot masquerade as class metadata. Role descriptors grant no syntax or
  accepted-expression proof; .3c derives the symbolic role from actual resolved reserved-name nodes.
  result_kind uses closed signatures with same arithmetic T, both conditional branch kinds,
  one-or-more homogeneous min/max, ordered atan2/arc_length/selectors, exactly five tolerances.
  Signature metadata has no structural255 bound or numeric/provider/geometry execution authority.
  Independent eight-kind and five-class argument population at arities0–4, wide variadic samples,
  normative rows both directions and canonical22 name/token agreement must verify closure.
  Actual compiled vocabulary/arity/generic/branch/order/selector/tolerance-role faults must fail
  body assertions and restore source exactly; no Cargo job or Rust edit overlaps them. Strict
  native/WASM, focused reference/language/book/ledger/retention/coverage/gate, exact previous/oldest
  record retention and per-leaf commit. Typed contextual mismatch/dependencies stay .3c/.4.
  Verification: four public contracts/680702 kind-class cases/21 actual compiled reds, strict
  native651/55 groups/WASM/reference/book/ledger pass, rc=0. Commit: `STITCHCAD-G1-0092`.


## Completed reference review parent — preserved from c98dc54

Exact payload:11lines/766B SHA256a3eb3ea30dd1d6d4bece14696e0749b6414271e33e02bd04949be64381527ce1.

- ID: `G1-SLICE.5b.1`
  Status: `done`
  Goal: enumerate every static obligation from contract2/3/5/6/9 and grammar5/6/7; exercise the
  actual reference with independently authored namespace/kind/refusal cases before trusting it.
  Acceptance: all nine origins/eight reserved names, single assignment/ambiguous collisions,
  forward names, all signatures and dimensional/exclusion/envelope rules mapped both directions;
  both branches inspected without value access. Log/repair discovered reference defects first.
  No curated example-only or runtime oracle claim. D124 current-grammar ruling resolves recognition before .2/.3.
  Verification: full signature4032/namespace1139/recipe196/review100 controls and actual reds verified.
  Commit: `STITCHCAD-G1-0082`.


## Completed namespace foundation parent — preserved from c98dc54

Exact payload:11lines/776B SHA256d7fcb0cb9ba829ea3ff7a87f1e3b654a383dc6ce40a36904d2ae517f763124b1.

- ID: `G1-SLICE.5b.2`
  Status: `done`
  Goal: immutable typed origin declarations and namespace/context resolution; numeric availability
  remains separate from declared kind. Geometry names identify prior operation outputs only.
  Acceptance: all origins/reserved contexts, spelling, collision/rebinding/forward refusal
  arguments and searched origins preserved; no implicit shadow/default or value computation.
  Core input boundary does not introduce sc-core→sc-measure dependency cycle; existing canonical
  declarations remain the sole authored values. D70 axes remain outside this namespace foundation.
  Verification: .2a/.2b/.2c/.2d verified below; metadata does not accept a whole recipe.
  Commit: `STITCHCAD-G1-0083`/`0084`/`0085`/`0086`/`0087`/`0088`/`0089`.

## Wanted-kind catalog checklist — .5b.3c.1

- [x] **REPRODUCE / ISSUE** — result-kind queries alone expose no typed wanted operands/results.
  Existing operator/built-in public suites now ten tests/681358 admission cases, exact wanted
  descriptors/result positions and nonoverlapping alternatives pass, rc=0. Actual normative rows
  and prior independent matrices supply separate catalog oracles.
- [x] **ROOT CAUSE (WHY + WHERE)** — prose-only or duplicated guessed requirements can drift from
  actual accepted signatures. wanted_signature_mutations.py →19 compiled body reds/exact restore,
  rc=0; exact/generic/negatable/tolerance/arity/population/direction/result/position faults prove
  both acceptance and diagnostic descriptors. Conditional branch-position fault preserves the
  accepted kind yet fails the independent descriptor assertion; admission alone is insufficient.
- [x] **FIX** — private immutable catalog constructors, typed operand/result requirements, static
  closed rows and pure matching; no arbitrary result index, source context or accepted expression.
  Explicit const storage fixes the draft temporary-reference refusal; no lifetime or lint waiver.
- [x] **ADDRESSED (verified)** —656 operator/680702 built-in tuples, complete24 actual built-in
  rows,17 products/14 quotients, exact arities/variables/result indices and wide min/max samples;
  two negative construction/mutation examples plus19 actual body reds pass, rc=0. Original product
  query implementations/fault anchors and normative grammar remain unchanged.
- [x] **NO REGRESSION** — make check:656 passed/55 groups, strict fmt/clippy; make wasm:three
  libraries; complete reference/structural, language16, publication10, ledger9+13, coverage pass,
  rc=0. Book59chapters/47API/1178source/1847render links. No execution/provider/state/geometry API.
- [x] **LOCKSTEP** — bounded book/API/examples/live/task scopes align; G1 stays5/18,11open/124sealed; D136 owned for immediate .2a repair.
  Prior .3b protocol25lines2141B SHAad457730…, review parent11lines766B SHAa3eb3ea3… and namespace
  parent11lines776B SHAd7fcb0cb… retained exactly fromc98dc54. Oldest ledger8lines648B SHAfa18d0d1…/
  lesson9lines753B SHAa62696a7… sealed. Retention243records/57workingMD/10714decodedlines/
  806158decodedB/394672residentB, rc=0. Fresh three-form defect census/complete IDs exceptD18 and
  tree10lanes/13trees/11siblings prove no duplicates/overlap/gaps; previous archives untouched.
  D136 actual unknown-call empty arguments reproduce with three queries, rc=0; .3c.2a owns
  truthful call lookup/payload repair next, then .2b checker; ordinal-context integration .3.
  promotion: declined (existing diagnostic wanted-kind, closed signatures and no invented context).

## Completed initial namespace parent — preserved from c98dc54

Exact payload:11lines/664B SHA2565f2f865ea0a01a2bdecb715623f9c814a046e8bcca21363183d5c8d3b1f8e787.

- ID: `G1-SLICE.5b.2c`
  Status: `done`
  Goal: checked initial namespace from ordered declaration pairs plus reserved metadata contexts.
  Acceptance: reject collisions before indexing, retain both origins/source identities, all reserved
  rebinding refusals; absent context does not hide declared kind; no value or geometry resolution.
  Verification: .1a/.1b decision/reference sources and .2 checked initial namespace verified below.
  Commit: `STITCHCAD-G1-0085`/`0086`/`0087`.

  Children: .2c.1a (reproduce/document D131 diagnostic conflict), .2c.1b (delegated decision,
  canonical/reference diagnostic repair), .2c.2 (checked product initial namespace).


## Completed ordered lookup parent — preserved from c98dc54

Exact payload:9lines/521B SHA256a80b45da26d2f3ea960379e6d1cff7eeee52510f5bc15089cdde7b90de11fff9.

- ID: `G1-SLICE.5b.2d`
  Status: `done`
  Goal: checked name reads/prior recipe bindings with typed searched origins and rebinding indices;
  forward/self references never reorder. Whole-expression/recipe integration remains .5b.3/.4.
  Acceptance: actual ordinal/span/source data only; no numeric/default/shadow behavior.
  Children: .1 exact initial metadata reads, .2 actual ordered recipe binding scope.
  Verification: .1/.2 source/query/order/refusal controls verified below.
  Commit: `STITCHCAD-G1-0088`/`0089`.

## Completed wanted-kind protocol — preserved from 74e8648

Exact payload:24lines/2066B SHA256c0074a5c2e5aebc81f83224d1be49bc8367e610dd5c805a83a136bbfae204137.

- ID: `G1-SLICE.5b.3c.1`
  Status: `done`
  Goal: closed typed operand/result requirements for dimension diagnostics before checking code.
  Work unit: `STITCHCAD-G1-0093`; predecessor c98dc54 clean/message empty/untracked/no jobs.
  Pre-code contract2/4.1/5.2, full grammar5–7, all public operator/built-in kind APIs and normalized
  arenas/scopes reviewed. formula_dimension must retain actual operands AND wanted kind rules;
  the wanted rule cannot be a prose-only guess or numeric/value query.
  Add immutable FormulaKindSignature catalogs accessible from existing unary/binary/built-in enums.
  Requirements are Exact(kind), shared Arithmetic T, Negatable N and symbolic ToleranceName.
  Result is Exact(kind) or an actual operand-kind position; variadic repeats its last requirement.
  Catalog construction stays private; public read-only views/result_kind are metadata only.
  All known fixed/variadic rules preserve positional and generic consistency; within's exact class
  role is retained. No type-error constructor/accepted expression or arbitrary source/ordinal API.
  Catalog admission must independently equal existing tested result_kind APIs over all656 operator
  cases and680702 built-in kind/class tuples, with actual normative rows in both directions and
  independently authored wanted descriptors. Actual compiled requirement/result/arity/repetition/
  generic/tolerance/product/order/population faults must fail body assertions and restore bytes.
  Keep standing older producers/anchors unchanged, strict native/WASM, focused reference/book/
  ledger/retention/coverage/gate; retain exact prior/oldest records before surface growth, commit.
  Diagnostic/checked-owner source lifetimes and truthful lookup domains finalized at .2 before code;
  statement indices derive only from actual scope-bound operands at .3, atomic whole graph at .4.
  Verification: ten public contracts/681358 admission cases/exact descriptors,19 actual compiled
  reds, strict656 native/55 groups/WASM and focused controls pass, rc=0. Commit: `STITCHCAD-G1-0093`.


## Completed coupled input protocol — preserved from 74e8648

Exact payload:32lines/2743B SHA2566437548e7bc37233457d2de023b119564240a61969666fb08fd8c6eaae459643.

- ID: `G1-SLICE.5a.3f.2`
  Status: `done`
  Clean7c81533; no jobs/user edits. Prior turn completed .1c; cleanup due18:56UTC, not yet due.
  Read roadmap4/7.8/G1, formula contract4.1/4.2/4.3/5/9, grammar1/4/4.1 and actual examples;
  prior normalization/identity/syntax APIs, fixtures, literal guards and all scoped proof maps.
  Close .3f only for syntax/input/identity, never numeric execution, geometry, storage or approval.
  Actual book population:17 bindings/four assertions (21 statements/25 operands),13 refusals.
  Author exact complete statement bytes independently from existing authored expression fixtures;
  independent recursive reference must check actual source/header/operand population and whole
  ordered token coverage with infer/evaluate trapped. No independent whole-recipe parser claim.
  Public tests exercise actual whole worked source/header/global spans and metadata, complete
  normalized identity/source independence/Clone/privacy, aliases/order and all refusal examples:
  distinguish three syntax refusals from ten deferred semantic checks, with explicit stage outcomes.
  Later input refusals in either untaken branch/call arguments preserve original rule/span/known
  ordinal; no partial normalized whole escapes. Coupled maximum4096/256/16 and first excess
  preserve typed domain/unsupported rules and context. Preserve every product source byte.
  Exclusive actual production faults for metadata, input coverage/ordinal, identity order/operand
  and combined bound must compile and fail new public assertions; classifier refuses compiler/
  expect-only noise and exact restoration precedes any build/probe. Watch anchors and authored
  reference producer through standing structural suite. Strict native/release/WASM plus scoped
  reference/language/publication and all alignment/ledger/archive/census records need actual results.
  Add book obligation map with exact implemented proof and concrete remaining .5/.6/.7/G2 owners;
  .5a.4 full milestone follows, evaluator/DAG/operations still need pre-code decomposition.
  Before growth, move complete whole recipe evidence and last closure exactly to this bounded
  sibling, retain old anchor routes, and shorten map orientation only as needed under unchanged cap.
  Oldest ledger payloads stay exact; no policy ceiling or human approval changes.
  Verification: final receipts below; all observed terminal0 after exact restoration.
  Commit: `STITCHCAD-G1-0071`.
Initial new public build refused E0716: the excess-source temporary was dropped while the parse
Result could still borrow it. Bind the owned test source before parsing and keep it alive through
refusal inspection. This is test development; no product implementation change or assertion red.


## Call lookup checklist — .5b.3c.2a

- [x] **REPRODUCE / ISSUE** — D136's three unknown calls and D137's six envelope aliases
  return correct tokens with empty required arguments; actual loader/infer reproductions, rc=0.
- [x] **ROOT CAUSE (WHY + WHERE)** — actual infer_call/call raised FErr without arguments;
  earlier token/kind matrices certify no payload schema. call_lookup_contract.py executes the
  repaired actual producer with exact independently authored fields/argument/value traps, rc=0.
- [x] **FIX** — documented contract5.2.2/envelope10 before code; actual callee check retains
  name/scope/real searched domains/request/alternatives. Product private query-borrowing error
  and FormulaBuiltin::resolve_call provide metadata only; grammar/keywords/population unchanged.
- [x] **ADDRESSED** — five public contracts/two negative privacy/lifetime examples pass;
  166 reference cases/12 compiled body reds/three loaded normative-set reds, and18 actual compiled
  product faults fail body assertions with exact byte restoration. Final focused test passes, rc=0.
- [x] **NO REGRESSION** — strict make check663 tests/56 result groups, fmt/clippy -D warnings;
  WASM three libraries; full reference/recognition matrices and language16 pass, rc=0. Native
  verification completed before the final exclusive fault run; restored public five tests pass0.
- [x] **LOCKSTEP / RETENTION** — publication10/60 chapters/51 API rows/1191 source/1870 rendered
  links, ledger9/pointer13/tree coverage/retention pass, rc=0. Prior protocols24lines2066B and
  32lines2743B retained exact from74e8648; oldest ledger5lines339B/lesson8lines649B/report19lines
  1685B sealed without changing prior windows. Census11open/126sealed, no duplicates/overlap.
  G1 stays5/18; D136/D137 fixed, D138 dimensions .2b.1 next; independent approval unclaimed.
  Promotion: decision_call-lookup.md records actual call sources and truthful request scope.
  Staged make gate returned terminal rc=0; commit hook repeats final checks. Unit STITCHCAD-G1-0094.

## Call lookup final receipts — .5b.3c.2a

All named commands returned terminal rc=0: make check, make wasm, the focused restored public
contract, call_lookup_mutations.py (18 actual compiled assertion reds/exact restore),
call_lookup_contract.py --mutations (166 actual cases/12 compiled/three normative reds), full
run_formula_structure_probes.sh, run_formula_language_probes.sh (16), publication (10), ledger
(9)/pointer (13), tree coverage and archive retention. Strict development initially caught
expect_err in a non-test helper; an explicit assertion precedes error destructuring, no lint waiver.
The first fault-anchor draft refused a rustfmt-wrapped match arm before mutation; exact formatted
source anchors then proved all18 failures. Prior bytes independently compared, no history rewritten.
Retention246 logical records/60 working Markdown/10779 decoded lines/810458 decoded bytes/
398972 resident bytes. Fresh materialization proves11 unique open/126 unique sealed; no overlap.
Production approval, whole-expression/recipe acceptance and execution remain unclaimed.

## Completed call lookup protocol — preserved from 7edc635

Exact payload:34lines/2966B SHA25607ec08f9a715df06de3a6c415130c9b08ad2572008b29c57086354d4a5cebf5f.

- ID: `G1-SLICE.5b.3c.2a`
  Status: `done`
  Goal: D136/D137 complete call refusal payloads and truthful typed lookup before checker.
  Work unit: `STITCHCAD-G1-0094`; predecessor74e8648 clean/message empty/untracked/no jobs.
  Pre-code contract3/5.2/5.3, grammar6/6.1/7, envelope10, units4 and existing MachineToken,
  namespace/catalog/normalized-call views and reference/probe APIs reviewed. Grammar unchanged.
  Data lookup searches nine declared origins; call lookup first searches six envelope aliases,
  then the closed built-in catalog. These are distinct typed lookup domains, not new data origins.
  Document call-scoped names/searches and real requested constructs/declared alternatives before
  code. Preserve unknown-call/envelope-before-argument precedence; no imagined constraint kind,
  recipe index, geometry reference, canonical expression or value provider in a name-only query.
  Product resolver takes a validated MachineToken; internal parser-validated str projection only.
  Opaque immutable query-borrowing refusals retain exact name, typed reason, actual searched sources
  and declared alternatives for envelope requests. Debug omits query; Display is token only.
  Reference unknown/envelope calls must carry required arguments from actual call name and source
  catalogs. Extend actual reference body-fault anchors when repair changes them; no inert faults.
  Independently verify every21 ordinary built-in calls, six envelope aliases, unknown/scalar/reserved
  names, exact schema/source order/alternative populations and no namespace/value/argument reads.
  Actual compiled reference and product source/name/source-order/precedence/privacy faults must
  fail body assertions and restore bytes. Strict native/WASM, focused reference/book/ledger/
  retention/coverage/gate; exact completed/oldest records retained and per-leaf commit.
  Interface finalized before code: FormulaBuiltin::resolve_call(&MachineToken) returns the existing
  builtin or private-field FormulaCallRefusal borrowing the exact query; internal lookup_call_name
  takes parser-validated str. RefusalKind is Unbound/Nurbs/SketchConstraints; LookupSource is
  Envelope/BuiltinCatalog in real search order. Alternatives are typed LineSegment/CircularArc/
  CubicBezier or OrderedConstructionRecipe; fixed tags documented in contract5.2.2/envelope10.
  Call-scoped exact name is the requested kind, never fictional geometric parameters; If is still
  the keyword special form. Private construction, token-only Display and payload-omitting Debug.
  D138 missing dimension arguments owned at .2b.1 before checker .2b.2; does not block this lookup.
  Verification: five public contracts/two negative examples/18 compiled body reds;166 reference
  cases/12 compiled and three loaded-set reds; strict663/56 groups/WASM/focused gates pass, rc=0.
  [Receipts/checklist](G1-SLICE-names.md#call-lookup-checklist--5b3c2a); decision_call-lookup.md.
  Commit: `STITCHCAD-G1-0094`.


## Completed syntax milestone protocol — preserved from 7edc635

Exact payload:26lines/2216B SHA256eecb46c725de5738fd5433a01328f5628077061aae139f811cd7a2eb50253a7c.

- ID: `G1-SLICE.5a.4`
  Status: `done`
  Work unit: `STITCHCAD-G1-0072`; clean predecessor ee42f5d, no initial jobs/user edits.
  Scope: complete syntax/input/canonical review and safe remaining evaluator decomposition.
  Read: bootstrap/README/memory/task doctrine, full formula contract/grammar/examples, units,
  ontology3/5, ADR-0003/D84/D95/D103/D109, current immutable APIs and ten public test families.
  No product source edit, new syntax, inference/evaluation, geometry or independent approval.
  Obligations: ASCII/token spelling/keywords, all units/literals/input rounding/width/domains,
  precedence and ordered calls/conditionals, let/assert/recipe boundaries and all four limits,
  exact canonical roles/domains/order/empty/aliases, original spans/ordinal/context and privacy.
  Display stays presentation; syntax parser admits only machine ASCII, no locale/display reader.
  Deferred: all nine origins/reserved collisions, every kind signature/exclusion, static whole
  acceptance, exact result arithmetic/storage, true irrational rounding, state/lazy execution,
  tolerances/atomic replay/cycle loading and complete construction operation dependencies.
  Each .5b–.5g child below owns a safe contract before code; refine further if a unit grows.
  Full milestone: make check, make wasm, make probes, final staged make gate. Rerun existing
  coupled seven actual assertion faults alone and verify exact restoration before other checks.
  Existing earlier fault receipts stay historical; this review does not claim re-running them all.
  D111: one current contract paragraph still calls completed statement identity pending; repair
  current prose plus ADR implementation status, retaining evaluation/storage/approval boundaries.
  D34 recurrence: TASK_TREE census example still says six siblings against actual ten; correct
  example here, mechanical frontier/count derivation stays PLANNING.5, without pivoting to it.
  Containment: preserve complete prior closure/node graph exactly in this bounded sibling; seal
  oldest complete changelog/dev-note records before growth crosses their health targets.
  Verification: completed receipts below; final commit hook repeats the staged doctrine gate.

## Dimension payload checklist — .5b.3c.2b.1

- [x] **REPRODUCE / ISSUE** — D138's five actual parse/infer examples returned dimension tokens
  with empty required arguments, rc=0; original report retained in defects-part58. Header/geometry
  census separately reproduces D139/D140, owned .3a/.5f.3a; P0 geometry repair follows this commit.
- [x] **ROOT CAUSE (WHY + WHERE)** — actual infer/infer_call emitted prose-only FErr and refused
  if/within before resolving every kind. dimension_payload_contract.py verifies actual constructor/
  call sites; source census and before/after examples pinpoint missing fields/early checks, rc=0.
- [x] **FIX** — canonical5.2.3 documented first; one expression dimension constructor retains
  operation/all actual kinds/tolerance roles/full wanted rows. Actual loaded operator/call tables
  supply signatures; children resolve left-to-right before dimension checks, callee priority intact.
- [x] **ADDRESSED** —4023 cases/3814 complete refusals across nine declared groups,15 actual
  compiled body reds; fields/row population/direction/variadics/roles/error selection verified, rc=0.
- [x] **NO REGRESSION** — old4032/14 signature, call166/12 plus normative3 controls and full
  structural/reference suite pass, rc=0; language16/publication10 pass0. Focused product15 tests
  (operator/builtin/call) pass0; Rust bytes unchanged. Shared older probes load actual catalogs,
  retaining independent limit checks; no empty or guessed wanted-rule fallback was introduced.
- [x] **LOCKSTEP / RETENTION** — canonical/annex/decision/index/live/task/tool records align;
  ledger9/pointer13/tree coverage/retention/gate pass, rc=0. Exact prior protocols34lines2966B/
  26lines2216B from7edc635 and oldest ledger8lines648B/lesson7lines500B/report7lines684B retained.
  Census12open/127sealed, no duplicates/overlap; G1 remains5/18, independent approval unclaimed.
  Promotion: decision_dim.md. D139 headers and P0 D140 geometry stay explicit remaining owners.
  No expression/whole-graph/product evaluation proof or grammar/token/identity change claimed.
  README objective/layout/commands reviewed unchanged. Staged make gate passes rc=0; hook repeats it.

## Dimension payload final receipts — .5b.3c.2b.1

All named verification commands returned terminal rc=0: dimension_payload_contract.py --mutations
(4023/3814/15), old static_signature_contract.py --mutations (4032/14), static_review_contract.py
--mutations, full run_formula_structure_probes.sh, language16, publication10, focused product15,
ledger9/pointer13, tree coverage and retention. No Rust file changed; no full-CI/push claim.
The constructor's new required catalogs exposed a TypeError in the old standalone probe loader;
an initial empty-catalog update still failed negative-count inference with KeyError. That dependency
now uses the actual fully loaded table prefix with independent limits still asserted. One attempted
edit named a nonexistent boundary and refused before writing; its corrected patch passed. Changed
within/conditional/unknown-call fault anchors were updated and proved actual body reds; no stale
anchor, compiler failure or wrong exception was accepted as evidence. No background job remains.
Retention249 logical records/63 working Markdown/10834 decoded lines/813906 decoded bytes/
402420 resident bytes, rc=0. The next archived payload would consume the last working-file slot;
before a multi-record next leaf, own a blocking history rollover under unchanged64-file bound.
