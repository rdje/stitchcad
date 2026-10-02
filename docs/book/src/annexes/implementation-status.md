# Annex: implementation status and requirement owners

This is the technical companion to [availability](../availability.md). It maps the current foundation
families to roadmap requirements, public code and task-owned verification. These are structural/local
integration claims. None establishes generated geometry, physical fit, factory acceptance or release.
The implementation rows are cross-checked by the book publication instrument; semantic signoff still
requires the named contracts and later proof owners.

| Family | Roadmap | Public source and verification | Book contract |
| --- | --- | --- | --- |
| Units/tolerances | §4.2; G0/G1 | sc-units; G1-SLICE.2/.5a.3c.1 | [Numerical contract](../spec/units-and-tolerances.md), [wide rounding](numeric-rounding.md) |
| Identity/references | §4.1; G1 | sc-core ontology; G1-SLICE.3a/.3b | [Model and current repairs](../spec/ontology-implementation.md) |
| Pieces/copies, sewing, marks/allowances | §3.1/§4.1; G1 | sc-core ontology; G1-SLICE.3c.1–.3 | [Executable ontology](../spec/ontology-implementation.md) |
| Garment constructions/closures | §3.1; G1 | sc-core ontology; G1-SLICE.3c.4 | [Constructions](../spec/ontology-constructions.md), [closures](../spec/ontology-closures.md), [review](../spec/ontology-review.md) |
| Canonical length/state/source | §2/§8; G1 | sc-core value; G1-SLICE.4a.1 | [Length inputs](../spec/measurement-inputs.md) |
| Measurement metadata/tables | §3.1/§3.4; G1 | sc-measure; G1-SLICE.4a.2/.3 | [Measurements](../spec/measurement-metadata.md) |
| Ease mappings/sets | §3.1/§3.3; G1 | sc-measure; G1-SLICE.4b | [Ease](../spec/ease-inputs.md) |
| Size membership | §3.4; G1 | sc-measure; G1-SLICE.4c.1 | [Membership](../spec/size-membership.md) |
| Garment chart observations/collections | §3.3/§3.4; G1 | sc-measure; G1-SLICE.4c.3a/.3b | [Observations](../spec/size-chart-observations.md), [collections](../spec/size-chart-collections.md) |
| MTM inputs | §3.3/§3.4; G1 | sc-measure; G1-SLICE.4c.3c | [MTM introduction](../spec/mtm-input-charts.md), [API](mtm-input-contract.md) |

| Formula literal inputs/normalized arenas | §4.1; G1 | sc-core recipe; G1-SLICE.5a.3c.2/.3 | [Literal normalization](formula-literals.md) |
| Formula lexing/expression syntax | §4.1; G1 | sc-core recipe; G1-SLICE.5a.1/.2b.2 | [Syntax API](formula-syntax.md) |

## Remaining proofs

Full SizeSet axes/breaks/composition and resolved profile intent remain G1-SLICE.4c work. D70's axes
cardinality ruling is pending. Recipe evaluation is .5, bus/undo .6, storage .7, constraints .8,
API/MCP .9 and CLI .10; browser/runtime/canvas/license proofs and full G1 exit remain later children.
The current ontology supports authored operation identities; that is not an executed recipe.

G2 validates geometry and the reference-skirt CLI/export proof. G3 executes garment constructions,
grading and the declared envelope. G4 validates profile/evidence/policy behavior. G5 proves UI/API/MCP
workflow parity and real UI/i18n behavior. G6 validates external receivers and reliability, and G7 owns
independent review and a scoped production declaration. V1 assembly and V2 simulation retain separate
tracks; neither is provided by the current libraries.

`make check` verifies strict library tests. `make wasm` verifies cross-compilation only. Book/topic
navigation, source/rendered local links, chapter/index coverage and the scoped status map are checked
by `docs/tasks/artifacts/book_publication/run_book_publication_probes.sh` from the repository root.
The map does not prove all semantics by matching API names: canonical tests and explicit deferred
requirements remain necessary. Test counts in historical review chapters are dated review snapshots;
current counts are derived from current test execution rather than inferred from those chapters.
