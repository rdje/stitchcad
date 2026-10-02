# Specification and reference

This part of the book holds StitchCAD's **specification**: the normative contract that implementation
must satisfy. Its foundation was written during gate G0 (the product-and-semantic-contract gate) from
`ROADMAP.md`, and each chapter names the gate that implements it.

The [learning path](../learn/design-to-pattern.md) introduces the concepts without requiring API or
format knowledge. These annexes carry their exact contracts. The [topic index](../topic-index.md)
gives experts direct entry points, and [availability](../availability.md) distinguishes implemented
libraries from future workflows. Existing chapter URLs and numbered clause anchors are preserved.

## How to read these chapters

- **Normative language.** "SHALL" and "MUST" state a requirement on the implementation; "SHOULD"
  states a default that a recorded decision may override; "MAY" states an option. A sentence in the
  indicative describes what the model *is*, not what a build does today.
- **Requirements and implementation are distinct.** The normative chapters specify intended
  behavior. Linked executable chapters describe built library contracts and their deferred proofs.
  [Availability](../availability.md) and the [implementation-status annex](../annexes/implementation-status.md)
  summarize that boundary. Detailed progress is in the repository's LIVE_STATUS.md; decisions and
  task-trees retain their canonical ownership. Git records currency; no hand-kept "last updated" date
  certifies a chapter.
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
| [Command layer](command-layer.md) | The five command classes and their commands, the shape of a command, atomic groups and undo granularity, preview/commit, revision preconditions and idempotency, the five authority levels, and the generated workflow-parity table |
| [Release and approval](release-contract.md) | The manifest field by field with its source, package identity and stale-ification, completeness against the declared construction, the six graduated acceptance states, scope narrowing, human-only approval, and the artifact policy matrix |
| [Interchange dialects](interchange-dialects.md) | The six axes an export target is made of, the target registry, the layer table in both naming modes, cut-as-1 against sew-as-1, blocks and metadata, the R12/R13 entity policy, tessellation, the three grading carriages |
| [Internationalization](i18n-architecture.md) | The one message system and why, message identity, the termbase per language, the externalization lint and its closed exemptions, locale-independent canonical files, pseudolocalization, the RTL geometry rule, the review tiers, and the derived message inventory |
| [Measurement standards](standards.md) | Which external standards the model draws on, what is adopted from each, and the verification status of every claim |
| [Reference skirt](reference-skirt.md) | The one garment specified with real numbers, which every conformance suite, golden file and agent gate is built around |

Each change updates its task-owned implementation and affected book contracts in the same commit.
The [topic index](../topic-index.md) also includes executable contracts and all other registered chapters.
