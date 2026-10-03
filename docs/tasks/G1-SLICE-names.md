# G1-SLICE — formula name resolution evidence

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
