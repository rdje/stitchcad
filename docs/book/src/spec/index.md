# The G0 contract

This part of the book holds StitchCAD's **specification**: the normative contract that implementation
must satisfy. It is written during gate G0 (the product-and-semantic-contract gate) from
`ROADMAP.md`, and each chapter names the gate that implements it.

## How to read these chapters

- **Normative language.** "SHALL" and "MUST" state a requirement on the implementation; "SHOULD"
  states a default that a recorded decision may override; "MAY" states an option. A sentence in the
  indicative describes what the model *is*, not what a build does today.
- **Status is elsewhere on purpose.** These chapters do not carry status flags or dates — a hand-kept
  "last updated" is right the day it is typed and false the day after. What has actually been built is
  in the repository's `LIVE_STATUS.md`; why a rule looks the way it does is in `docs/decisions/`;
  which leaf owes which chapter is in `docs/tasks/G0-CONTRACT.md`.
- **Claims carry their verification.** Where a chapter states a fact about an external standard or a
  third-party product, it also states how that fact was established — read in this repository, cited
  from the roadmap's own review, or still unverified with a named owner. An unverified claim is
  labelled rather than repeated until it sounds true.

## The chapters, and what each one settles

| Chapter | Settles |
| --- | --- |
| [Glossary](glossary.md) | One meaning per construction term, its canonical object, the synonyms factories and other CADs use, and the machine token that must never be rendered raw |
| [Units and tolerances](units-and-tolerances.md) | The internal unit, the five tolerance classes and how each is derived, the curve set, the offset error budget |
| [Garment ontology](ontology.md) | Every first-class object: identity, fields, invariants, and how it carries uncertainty |
| [Formula language](formula-language.md) | The recipe's expression language, in three parts: the contract (kinds, names, evaluation, errors, exclusions), the [grammar and its tables](formula-language/grammar.md), and the [worked examples](formula-language/examples.md) over the reference skirt |
| [Supported envelope](feature-matrix.md) | What v1 supports, what it rejects with a diagnostic, and what is deferred — the boundary of the release claim |
| [Instantiation paths](instantiation-paths.md) | Measurement-driven regeneration and grade-rule instantiation, where they diverge, and the declared equivalence tolerance |
| [Size sets](size-sets.md) | Size labels versus order, base size, multi-dimensional charts, and which object owns a size set |
| Command layer | The typed command set, atomic groups, preview/commit, revision preconditions, undo/redo granularity, and agent authority levels |
| Release and approval | The manifest, what approval binds to, how it stale-ifies, and the graduated acceptance states |
| [Interchange dialects](interchange-dialects.md) | The six axes an export target is made of, the target registry, the layer table in both naming modes, cut-as-1 against sew-as-1, blocks and metadata, the R12/R13 entity policy, tessellation, the three grading carriages |
| Internationalization | The one message system, externalization, termbases, pseudolocalization, locale-independent files, and the RTL geometry rule |
| [Measurement standards](standards.md) | Which external standards the model draws on, what is adopted from each, and the verification status of every claim |
| [Reference skirt](reference-skirt.md) | The one garment specified with real numbers, which every conformance suite, golden file and agent gate is built around |

Each chapter is added to this book by the task-tree leaf that writes it, so the list above grows into
the table of contents rather than preceding it.
