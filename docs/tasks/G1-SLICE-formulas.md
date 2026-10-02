# G1-SLICE — completed formula syntax contracts and evidence

Bounded semantic sibling of [G1-SLICE](G1-SLICE.md). Completed syntax contracts/checklists retain
exact committed text; the current frontier and verification/commit journal remain in the parent.

## Lexical contract and evidence — preserved from 60c7305

- ID: `G1-SLICE.5a.1`
  Status: `done`
  Goal: borrowed ASCII lexical stream for the normative machine-form formula syntax; explicit
  lexical kind, original text and byte span, with typed first-error refusal and fused termination.
  Pre-code protocol: sc_core::recipe owns the syntax front-end. Immutable FormulaLexeme and
  FormulaSourceSpan retain exact source text/positions; FormulaLexer scans borrowed input without
  cloning names/numbers or interpreting values. Classify three keywords using one shared private
  name-module classifier; preserve all existing MachineToken public constructor/error behavior.
  ASCII preflight reports the first full offending Unicode scalar before producing tokens. General
  ASCII whitespace is ignored between tokens but span gaps remain; the later parser enforces the
  literal's exact single-space unit separator. Identifier spelling uses the same allocation-free
  lower-snake predicate as MachineToken. Number tokens retain digits/optional nonempty fraction,
  with no numeric conversion/rounding/implicit unit. Punctuation/operators include longest paired
  comparisons; malformed identifiers, decimal fraction, lone ! and unsupported characters refuse.
  After first error/end, iterator remains ended. Debug of lexer reveals scope/position, not source.
  Lexical success certifies only tokens: adjacent atoms, comments assembled from slash operators,
  unsupported calls/exponents/units, type/name errors and recipe bounds retain parser/checker owners.
  Own native/WASM tests, borrowing/privacy docs, every current worked example's machine-token scan,
  actual guard mutations, book/public-status map/index, canonical ADR clause-link fix D73, bounded
  live/evidence seals and commit. No evaluation, canonical formula identity or app/MCP claimed.
  Verification: 13 lexical contracts/two privacy-lifetime docs; nine actual assertion reds/exact restore;
  454 strict native tests, WASM/book; publication nine, formula 15 and ledger nine probes green.
  Commit: `STITCHCAD-G1-0037` (this recording commit).

### `G1-SLICE.5a.1` — borrowed lexical syntax with explicit parser boundaries

- [x] **REPRODUCE / ISSUE** — complete formula contract/grammar/examples and ADR read before
  lexical protocol/subleaves. No recipe module existed at predecessor 9ef9602. D73 ADR names absent
  examples/exclusions clauses. First gap contract finds D74: 12 pass/1 fail, rc=101, byte 11 refused.
- [x] **ROOT CAUSE (WHY + WHERE)** — `cargo test -p sc-core --test formula_lex_contract` →
  UnsupportedCharacter at bytes 11..12, 12 passed/1 failed, rc=101; lexical whitespace helper omits
  vertical tab. `rg -n '§10|§11' docs/decisions/decision_adr-0003-construction-recipe-and-formula-language.md`
  found two obsolete clause pointers, rc=0; main headings are §1–§9 and actual exclusions §6.
  Missing lexical front-end is a planned feature; it must borrow source and carry no evaluation authority.
- [x] **FIX** — immutable token/kind/span and typed source-private first-error/fused scanner; shared
  spelling/three-keyword classifiers retain MachineToken behavior. Full ASCII preflight precedes tokens;
  number spelling and whitespace gaps retain later parser authority. Added vertical tab, corrected ADR
  clause links, contract/privacy tests, isolated actual source mutations and indexed syntax annex.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test formula_lex_contract` → 13 passed,
  0 failed, rc=0; all 17 bindings/four assertions/13 refusal forms consumed with explicit lexical scope.
  `bash docs/tasks/artifacts/formula_lex/run_formula_lex_mutations.sh` → nine actual assertion reds,
  rc=101 each, runner rc=0; both sources restored byte-identically. Two privacy/lifetime doctests pass.
  D73 targets exist and D74 gap regression remains green; exact logged descriptions seal to part15.
- [x] **NO REGRESSION** — `CARGO_HOME="$PWD/target/cargo-home" TMPDIR="$PWD/target/scratch" make check`
  → strict lint and 454 tests including docs green, rc=0; `make wasm` → three-library cross-build, rc=0.
  Formula-language probes 15 pass/0 fail, ledger nine pass/0 fail, publication nine pass/0 fail, rc=0.
  Feature/tree/glossary/uncertainty/fixture censuses green; source/rendered publication links verified.
  Staged `make gate` → all doctrines green, rc=0. Focused checks are appropriate for this syntax
  subleaf; full milestone belongs .5a.4. No remote-CI claim.
- [x] **LOCKSTEP** — roadmap language unchanged; .5a.2/.3/.4 own parser/canonical/bounds completion.
  Learning/availability/module/package/README/status map/index and detailed API annex state actual
  lexical scope. Forty-eight chapters/15 API rows, 991 source/1506 rendered links checked. Exact
  predecessor publication contract/checklist and oldest ledger payloads preserved; no cap raised.
  D70 required axes ruling stays pending. G1 remains 5/18 top-level leaves with .4/.5 structurally partial.

Further syntax work remains owned by the parent frontier.
