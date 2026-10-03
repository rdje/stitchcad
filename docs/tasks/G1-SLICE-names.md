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
