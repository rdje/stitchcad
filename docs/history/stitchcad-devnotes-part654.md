# Sealed engineering continuity — part654

- **Identity:** 25 lines, 4697 bytes, sha256:6145dfdb5c3e9030ed0fa92aeba686a4bfe5229ff10cf57aa28ed460f5802ef3
- **Coverage:** whole0498 task scope, lesson and changelog from 15827be5e037fb0eb2fbf7975790baa8b13e5828.
- Whole source interval: docs/tasks/G1-SLICE-execution.md bytes 11935–14884, 2949 bytes, sha256:457f48fcf3b7ec0e2ee458a6fb4347e82924d13053831c98adc29943e7bb214a
- Whole source interval: DEV_NOTES.md bytes 193–1201, 1008 bytes, sha256:cc976487d9607239461aa414b0ff523636f3b0f695f58633966b8c792b5640ea
- Whole source interval: CHANGELOG.md bytes 207–947, 740 bytes, sha256:566e9baf5845f46c8f875c25533173d5bb85f24ba7c07c7790eabd36e3bd2805
- **Retrieval:** exact bytes after the first rule and blank line.
- **Write policy:** immutable; corrections belong in a new record.

---

- ID: `G1-SLICE.6.5.3.2.3.2.4.2.1`
  Status: `done`
  Goal:0498 shared exact floor/half-comparison magnitude rounding implemented without a second rounding policy or dependency;old ratio behavior/operation names retained.
  - [x] REPRODUCE / ISSUE: read actual round.rs/public contract/tests/probe selectors and run nonuniform_ease_oracle.py --check →160 rows,188-bit denominator,rc=0. Wide valid source ratios exceed fixed inputs;shared policy must accept an exact decomposition without narrowing or copying its half-away decision.
  - [x] ROOT CAUSE: python3 -I -B docs/tasks/artifacts/seam_partition/nonuniform_ease_oracle.py --check →Fraction/common-denominator agreement/188-bit denominator,rc=0;actual old ratio signatures accept only128-bit fields. Floor plus exact fractional-half ordering suffices for the shared decision;caller retains division validity and workspace ownership.
  - [x] FIX: new round_magnitude_from_half_comparison delegates to one private checked increment decision used by both old ratio entries. Preserve old signed/unsigned error context;new overflow names the new operation. Document mathematical input responsibilities and limits. Adapt only old remainder/tie fault anchors;producer dispatch/refusal/restoration/classifier rules unchanged.
  - [x] ADDRESSED: python3 -I -B scripts/local_environment.py -- cargo test -p sc-units --test magnitude_round_contract --test round_contract --test unsigned_round_contract →11 passed,rc=0. New2 include125 nonzero independent Decimal rows and explicit half/full-width/overflow cases;old4+5 keep signed endpoints,zero,width and operation context. Source doctest
    filter round::round_magnitude_from_half_comparison →1 passed/2 filtered,rc=0.
  - [x] NO REGRESSION: python3 -I -B scripts/local_environment.py -- cargo clippy -p sc-units --lib --test magnitude_round_contract --test round_contract --test unsigned_round_contract -- -D warnings and cargo fmt --all -- --check →pass,rc=0. unsigned_round_reference.py →138 Decimal rows pass,rc=0;read-only AST/source census →5+9 unique actual fault
    anchors,rc=0. Initial new fixture count134 failed101;independent census proves125 nonzero/13 zero and corrects expectation.
    transition_receipts.py --record docs/history/stitchcad-devnotes-part653.md →3 exact Git spans,rc=0. mdbook build/publication check →88/174/1561/2664;glossary census →0 failures;verify-retention →1107 logical/74 working/2549112 residentB,all rc=0. Compiled mutation runs remain full paired .6.5.3.2.acceptance or required pre-push probes;weighted source
    acceptance remains .4.2.acceptance. No unobserved verdict implied.
  - [x] LOCKSTEP: units spec/rounding annex/indexed numerical decision/live journals/task/index synchronized;README/LIVE reviewed unchanged,G1 6/18. Whole0497 retained653. Next .4.2.2 explicit workspace admission,then .4.2.3 weighted quantities;actual contexts/full paired effects remain later owners.
## _(2026-10-28 UTC)_ — shared rounding accepts exact wide half comparisons

0498 adds dependency-free magnitude floor/half-comparison rounding;all3 entry points share one increment decision. Consumers own exact division/nonzero denominator,workspace,sign and scalar checks. New u128 increment overflow keeps its operation;old ratio errors/values unchanged.
Focused native11/source doctest1/Decimal138/scoped Clippy/fmt pass0. Two new contracts cover125 nonzero Decimal rows and explicit floor/half/overflow limits. Initial count assertion134 failed101;independent fixture census established125 and corrected the authored expectation.
Old5+9 fault anchors remain unique after exact remainder/tie relocation;compiled mutation verdict deferred to full paired acceptance. No weighted runtime/physical claim. Promotion:indexed numerical decision;LIVE/README unchanged,G1 6/18.
[Whole0497 recovery lesson](docs/history/stitchcad-devnotes-part653.md) retains exact arithmetic/context scope;3 Git spans verified.
## STITCHCAD-G1-0498 — share half-away rounding with exact wide workspaces

- New unsigned floor/half-comparison primitive delegates to the same private increment decision as signed/unsigned ratios. Preserve ratio values/error names;new required increment beyond u128 returns named Overflow. No dependency or float.
- Native11/source doctest1/Decimal138/scoped Clippy/fmt pass. Two new contracts include125 nonzero independent rows and explicit half/full-width limits. Old5+9 fault anchors adapted and unique;compiled mutation verdict deferred to full paired acceptance/pre-push probes.
- Whole0497 retained653;book/indexed numerical decision/task/live pointers synchronized. LIVE G1 6/18 unchanged;weighted workspace admission follows.

