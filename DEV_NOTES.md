# DEV_NOTES.md

Detailed technical notes — root cause, implementation, validation — per slice. The
engineering-continuity surface (not the public docs; that's `docs/book/`). Newest first.

## _(2026-10-02)_ — domain diagnostics need context without an invented cause

- D90's public DomainExceeded variant lacked operation; forwarded Length construction reused the
  base constructor, losing the caller. D92's generic display claimed every invalid value exceeds an
  upper limit and proves a conversion bug, including a zero positive-range width and negative length.
- Add a static operation field and one private checked-construction path per quantity; forward actual
  caller labels without duplicating guards or changing numeric data. Core resolve/range bridges name
  their public operation. Render neutral outside-domain wording; do not infer a cause from magnitude.
- Actual predecessor public calls produce two assertion failures. Five public tests verify typed and
  rendered direct/forwarded signed refusals, inclusive endpoints and unchanged non-domain behavior;
  three private core tests verify unreachable-invalid totality guards without a geometry certificate.
- Fourteen actual context/rendering mutations compile and fail assertions, restoring every source
  byte. D89's six compiled operator reds still discriminate. Strict native488 including docs,
  release5 and three-crate WASM pass. Division/area domain failures cannot occur for valid Length
  operands; those caller labels are wired without a fabricated public error reproduction.
- Book migration/examples match code. Earlier task evidence and oldest ledgers preserve exact bytes.
  D91 context classifier remains next;
  D83/D84 remain owned. No formula evaluator, command bus, MCP or production-release claim.
- promotion: declined (routine completion of the existing typed-error and truthful-diagnostic contract).

## _(2026-10-02)_ — public operators must close the constructor invariant

- D89 review found Length’s Add/Sub directly construct Self from raw integer sums/differences. Valid
  ±1 km operands produce ±2 km lengths without typed refusal; constructors and checked methods
  reject those results. The private-field domain guarantee was therefore false for public operators.
- Trait Output now returns Result<Length,UnitError>, delegating to checked_add/checked_sub. Ordinary
  values, inclusive endpoints, cancellation and signed crossings preserve the same exact numeric
  contract. Callers migrate to `(left + right)?` / `(left - right)?`; crate docs demonstrate handling.
- Public predecessor tests fail three assertions, not compilation. Four current contracts include an
  explicit Result type and a nine-by-nine i128 pair oracle; six actual production bypass/operation/
  saturation mutations compile and fail assertions, then restore source bytes exactly.
- D90 also surfaced: DomainExceeded lacks the failing operation. .3b.1b owns its public error/call-site
  repair immediately next. This slice certifies operator domain closure, not complete diagnostic
  context, formula normalization/evaluation, geometry, MCP or production release. D83/D84 retain owners.
- D91: book L6b mistook valid inline Rust question-mark handling for formula syntax (13 pass/2 fail).
  Explicit Rust fences unblock publication; .3b.1c owns context-aware census proof after D90.
- Completed rational protocol/checklist/journal and oldest live payloads preserve predecessor bytes.
- promotion: declined (routine enforcement of the existing numeric domain and typed-refusal contract).

## _(2026-10-02)_ — rational limits bound reduced values, not hidden temporaries

- D83 see measured width but returned success; L8's final census verdict was not runtime refusal.
  Canonical input rounded away oversized sub-quantum fractions before any width observation.
- see now raises formula_domain with operation, published max_rational_bits and measured width.
  Check converted exact input before rounding and every completed numeric node in result-kind
  internal units. Remove true-unit temporary observations; their scale can inflate a valid fraction.
  Fraction reduction precedes width checks; lexical digit count and raw cross-products are not values.
- Sixty-one independently authored Fraction controls cover 127/128/129-bit edges, exact input,
  signs, cancellation, scale, selectors and lazy branches; twelve actual guard mutations turn red.
  Existing literal/arith/angle six/nine/seven mutations retain exact restoration. Literal controls
  now number361: the oversized 100-zero fraction refuses; a reducible long-zero spelling stays valid.
- D88 surfaced because token-only tan tests accepted an unrelated rational refusal after pole guard
  removal. The mutation runner caught this masked failure. Require the exact mathematical-domain
  reason, distinct from atan2-zero refusal; all seven actual reds then discriminate again.
- D83 scalar domains/i64 and D84 signed-angle verification remain owned next; no new Rust evaluator,
  arbitrary-input transcendental, release or MCP proof. Predecessor evidence/history retain exact bytes.
- promotion: declined (routine enforcement of existing rational/domain and verification contracts).

## _(2026-10-02)_ — angle storage units must reach the irrational call unchanged

- The angle audit found a foundational scale error, not merely a missing binding modulo: to_true
  rescales ratio only, while trig/arc_length treated internal microdegrees as degrees. Full-turn arc
  around radius1 um returned6283185 rather than6; sin90 returned0 and cos90 returned1000000.
  Source history 3704b8a introduced both unscaled radian paths, dir truncation and no tangent pole guard.
- One shared direct conversion divides microdegrees by180000000 before multiplying by pi; it keeps
  signed/multi-turn/fractional sweeps. dir rounds before normalization, agreeing with atan2. Exact
  tangent poles use rational modulo180deg and raise formula_domain; representable neighbors remain finite.
- Forty-two explicit rows/72 controls and an independent standard-library math oracle agree on defined
  curated arguments. Seven actual source mutations discriminate scale, sweep, fractional precision,
  dir rounding, pole/period/token. Decimal60 scope remains curated, not an arbitrary-input certificate.
- The director resolved D84: signed/multi-turn formula values persist; entity direction fields
  normalize. Specifications and the durable decision align; .3c still owns binding/equality and
  signed inverse-trig verification. D83 rational/scalar guards continue independently.
- Expert annex/grammar and live task pointers match the repair; prior arithmetic and history payloads
  preserve exact text. Older task decisions/journals are retained rather than expanding live caps.
- promotion: promoted by `decision_angles.md` (director’s storage ruling).

## _(2026-10-02)_ — canonical literal display must not conceal a different value

- Actual reference diagnostic reproduces fractional unit and bare-decimal nodes: two canonical-zero
  literals accumulate 4/5 internal quantum and bind 1. L1 previously rounded only the displayed node,
  masking the disagreement with canonical kind:integer identity. Scoped source history identifies
  3704b8a G0-CONTRACT.9 as the introducing parser; later D75/D76 repairs did not alter literal values.
- Convert/round each literal once at input, before expression arithmetic. Counts retain kind and
  bare decimals/pct retain ratio scaling; no angle modulo or arithmetic-node folding is introduced.
  Sixty explicit rows/360 controls use an independent Decimal rounding oracle and kind-preserving
  respellings; six actual guards discriminate quantum, ties, scale, kind and direct unit factors.
- D80/D81 repair duplicate units numbering and a stale live next pointer. Formula contract now
  explains the existing canonical-input boundary; details and honest proof gaps stay in the annex.
- The wider diagnostic exposes D82 early operator rounding and D83 unenforced numeric domains;
  .5a.3b.2/.3 own immediate repairs before product normalization. Published example agreement is
  still curated scope, not complete exact-arithmetic or arbitrary-input production verification.
- Completed rounding evidence and oldest live payloads preserve committed predecessor text.
- promotion: declined (routine reference repair; literal conversion/canonical identity policy unchanged).

# Sealed archive — earlier lessons

| Segment | Coverage | Sealed identity |
| --- | --- | --- |
| [`devnotes-part1.md`](docs/history/window1.md#stitchcad-devnotes-part1md) | the `2026-09-29` and `2026-09-04` lessons, plus the bootstrap entry | 66 lines, 5589 bytes, `sha256:d3b94e9a…` |
| [`devnotes-part2.md`](docs/history/window1.md#stitchcad-devnotes-part2md) | the two oldest `2026-09-30` lessons (enumeration, and the vocabulary census) | 62 lines, 5915 bytes, `sha256:edcd0808…` |
| [`devnotes-part3.md`](docs/history/window1.md#stitchcad-devnotes-part3md) | two `2026-09-30` lessons (two tables, one garment; a fixture internally right) | 50 lines, 4723 bytes, `sha256:fcca661d…` |
| [`devnotes-part4.md`](docs/history/window1.md#stitchcad-devnotes-part4md) | two `2026-09-30` lessons (a blocked leaf splits; permission is no criterion) | 42 lines, 3706 bytes, `sha256:c2ac5791…` |
| [`devnotes-part5.md`](docs/history/window1.md#stitchcad-devnotes-part5md) | two `2026-09-30` lessons (a rule whose only path is "don't"; a digest is about bytes) | 43 lines, 3977 bytes, `sha256:859ce981…` |
| [`devnotes-part6.md`](docs/history/window1.md#stitchcad-devnotes-part6md) | two `2026-09-30` lessons (a spec's tables are its test suite; settle it with the artifact) | 55 lines, 5359 bytes, `sha256:129d50d8…` |
| [`devnotes-part7.md`](docs/history/window1.md#stitchcad-devnotes-part7md) | three `2026-09-30` lessons (an arm that removes the rule; a synthetic input is a fixture; a RED arm asserts the refusal) | 57 lines, 5120 bytes, `sha256:13fd6c73…` |
| [`devnotes-part8.md`](docs/history/window1.md#stitchcad-devnotes-part8md) | two `2026-09-30` lessons (source layout; i18n population) | 35 lines, 3196 bytes, `sha256:04ab285c…` |
| [`devnotes-part9.md`](docs/history/window1.md#stitchcad-devnotes-part9md) | the `2026-09-30` certifying-artifact lesson | 15 lines, 1343 bytes, `sha256:bc7fae65…` |
| [`devnotes-part10.md`](docs/history/window1.md#stitchcad-devnotes-part10md) | the `2026-09-30` shipped-work reconciliation lesson | 15 lines, 1380 bytes, `sha256:701d33f2…` |
| [`devnotes-part11.md`](docs/history/window1.md#stitchcad-devnotes-part11md) | the `2026-09-30` property-test framework lesson | 15 lines, 1384 bytes, `sha256:ae04eadf…` |
| [`devnotes-part12.md`](docs/history/window1.md#stitchcad-devnotes-part12md) | ontology slice decomposition | 16 lines, 1570 bytes, `sha256:a2f04e3d…` |
| [`devnotes-part13.md`](docs/history/window1.md#stitchcad-devnotes-part13md) | injected identity lesson | 18 lines, 1612 bytes, `sha256:38e83349…` |
| [`devnotes-part14.md`](docs/history/window1.md#stitchcad-devnotes-part14md) | persistent-identity lesson | 24 lines, 2230 bytes, `sha256:2b6aebd3…` |
| [`devnotes-part15.md`](docs/history/window1.md#stitchcad-devnotes-part15md) | structural-piece lesson | 15 lines, 1334 bytes, `sha256:1b362d26…` |
| [`devnotes-part16.md`](docs/history/window1.md#stitchcad-devnotes-part16md) | interval-coverage lesson | 13 lines, 1183 bytes, `sha256:fcf7c475…` |
| [`devnotes-part17.md`](docs/history/window1.md#stitchcad-devnotes-part17md) | separate-pair-member lesson | 12 lines, 1049 bytes, `sha256:140c4c41…` |
| [`devnotes-part18.md`](docs/history/window1.md#stitchcad-devnotes-part18md) | semantic-anchor/profile-binding lesson | 12 lines, 1102 bytes, `sha256:61a13500…` |
| [`devnotes-part19.md`](docs/history/window1.md#stitchcad-devnotes-part19md) | physical-copy identity lesson | 18 lines, 1663 bytes, `sha256:c0e3c442…` |
| [`devnotes-part20.md`](docs/history/window1.md#stitchcad-devnotes-part20md) | physical sewing-interval lesson | 15 lines, 1375 bytes, `sha256:34867dc9…` |

| [`devnotes-part21.md`](docs/history/window1.md#stitchcad-devnotes-part21md) | directed-grainline lesson | 13 lines, 1212 bytes, `sha256:c31c3298…` |

| [`devnotes-part22.md`](docs/history/window1.md#stitchcad-devnotes-part22md) | per-edge allowance lesson | 13 lines, 1192 bytes, `sha256:1807ae98…` |

The live window below holds the most recent lessons. When it passes its health target (200 lines /
16 384 bytes) again, the oldest entries are sealed the same way, and the `DESCRIPTOR` rule of
`run_changelog_ledger_probes.sh` proves the digest afterwards.

| [`devnotes-part23.md`](docs/history/window1.md#stitchcad-devnotes-part23md) | tuck/pleat and dart lessons | 25 lines, 2172 bytes, `sha256:ba5ee2a7…` |

| [`devnotes-part24.md`](docs/history/window1.md#stitchcad-devnotes-part24md) | canonical gather lesson | 13 lines, 1160 bytes, `sha256:2a10a04e…` |

| [`devnotes-part25.md`](docs/history/window1.md#stitchcad-devnotes-part25md) | Hem and served-layer lessons | 28 lines, 2416 bytes, `sha256:a95d8c77…` |

| [`devnotes-part26.md`](docs/history/stitchcad-devnotes-part26.md) | closure and notion-placement lessons | 27 lines, 2385 bytes, `sha256:b4b58e1b…` |

| [`devnotes-part27.md`](docs/history/stitchcad-devnotes-part27.md) | canonical buttonhole derivation lesson | 13 lines, 1158 bytes, `sha256:319f11b3…` |

| [`stitchcad-devnotes-part28.md`](docs/history/stitchcad-devnotes-part28.md) | Pocket composition lesson | 14 lines, 1301 bytes, `sha256:764116ee…` |

| [`stitchcad-devnotes-part29.md`](docs/history/stitchcad-devnotes-part29.md) | coverage probe calibration lesson | 15 lines, 1383 bytes, `sha256:d6d266b6…` |

| [`stitchcad-devnotes-part30.md`](docs/history/stitchcad-devnotes-part30.md) | numeric availability/source-truth lesson | 15 lines, 1356 bytes, `sha256:7fa4db87…` |

| [`stitchcad-devnotes-part31.md`](docs/history/stitchcad-devnotes-part31.md) | identifier grammar/binding lesson | 14 lines, 1283 bytes, `sha256:317f385a…` |

| [`stitchcad-devnotes-part32.md`](docs/history/stitchcad-devnotes-part32.md) | canonical procedure metadata lesson | 20 lines, 1878 bytes, `sha256:d5d201dc…` |

| [`stitchcad-devnotes-part33.md`](docs/history/stitchcad-devnotes-part33.md) | archive capacity/retrieval lesson | 18 lines, 1591 bytes, `sha256:aad7494a…` |

| [`stitchcad-devnotes-part34.md`](docs/history/stitchcad-devnotes-part34.md) | measurement table binding lesson | 18 lines, 1699 bytes, `sha256:3dc0b619…` |

| [`devnotes-part35.md`](docs/history/stitchcad-devnotes-part35.md) | individual Ease mapping lesson | 13 lines, 1190 bytes, `sha256:de68d50d…` |

| [`devnotes-part36.md`](docs/history/stitchcad-devnotes-part36.md) | per-POM Ease query lesson | 13 lines, 1196 bytes, `sha256:f0b78fd7…` |

| [`devnotes-part37.md`](docs/history/stitchcad-devnotes-part37.md) | membership/Ease review lessons | 21 lines, 1781 bytes, `sha256:6a76a906…` |
| [`devnotes-part38.md`](docs/history/stitchcad-devnotes-part38.md) | chart correspondence lesson | 14 lines, 1281 bytes, `sha256:453f9677…` |
| [`devnotes-part39.md`](docs/history/stitchcad-devnotes-part39.md) | MTM/coverage lessons | 31 lines, 2736 bytes, `sha256:c7d16877…` |
| [`devnotes-part40.md`](docs/history/stitchcad-devnotes-part40.md) | progressive book lesson | 18 lines, 1677 bytes, `sha256:89bc77bc…` |

| [`devnotes-part41.md`](docs/history/stitchcad-devnotes-part41.md) | borrowed lexical source lesson | 19 lines, 1811 bytes, `sha256:92361240…` |

| [`devnotes-part42.md`](docs/history/stitchcad-devnotes-part42.md) | complete argument traversal lesson | 19 lines, 1781 bytes, `sha256:f77b3cf9…` |

| [`devnotes-part43.md`](docs/history/stitchcad-devnotes-part43.md) | source gaps and keyword roles | 15 lines, 1397 bytes, `sha256:3d7763a5…` |

| [`devnotes-part44.md`](docs/history/stitchcad-devnotes-part44.md) | exact arithmetic results | 17 lines, 1562 bytes, `sha256:a378e4ad…` |

| [`devnotes-part45.md`](docs/history/stitchcad-devnotes-part45.md) | semantic bounds and delimiter nesting | 19 lines, 1803 bytes, `sha256:c9686523…` |

| [`devnotes-part46.md`](docs/history/stitchcad-devnotes-part46.md) | signed reconstruction lesson | 17 lines, 1562 bytes, `sha256:d00c7340…` |
