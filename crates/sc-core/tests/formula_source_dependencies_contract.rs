//! Complete actual prospective canonical Derived relation, without cycle/value execution claims.
#![allow(clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    recipe::{
        FormulaBindingKind, FormulaCanonicalInputAdapter as Adapter,
        FormulaDeclaration as Declaration, FormulaExpressionDocument, FormulaInitialDeclaration,
        FormulaInputOrigin, FormulaNamespace, FormulaNormalizedSources, FormulaScalarInputOrigin,
        FormulaSource, FormulaSourceCheckLimits, FormulaSourceDependencyLimits as Limits,
        FormulaSourceDependencyRefusal as Refusal, FormulaSourceInputs, FormulaSourceLimits,
        FormulaSourceNormalizationLimits, FormulaSources,
    },
    value::{
        CanonicalInputs, LengthDeclaration, LengthDeclarationDefinition, LengthState,
        LengthValueError, ScalarInputDeclaration, ScalarInputDeclarationDefinition,
        ScalarInputKind, ScalarInputState,
    },
};
use sc_units::Length;
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn length(identity: u128, state: LengthState) -> LengthDeclaration {
    LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(identity),
        source: id(9001),
        state,
    })
    .expect("actual original canonical declaration")
}
fn scalar(identity: u128, state: ScalarInputState) -> ScalarInputDeclaration {
    ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
        id: id(identity),
        source: id(9001),
        kind: ScalarInputKind::Count,
        state,
    })
    .expect("actual original general scalar")
}
fn sources(rows: &[(u128, &str)]) -> FormulaSources {
    let sources: Vec<_> = rows
        .iter()
        .map(|(identity, source)| {
            FormulaSource::new(
                id(*identity),
                FormulaExpressionDocument::new((*source).into()).expect("actual original syntax"),
            )
        })
        .collect();
    let limits = FormulaSourceLimits::new(
        sources.len(),
        sources.iter().map(|s| s.expression().source().len()).sum(),
        sources
            .iter()
            .map(|s| s.expression().canonical().as_str().len())
            .sum(),
    );
    FormulaSources::new(sources, limits).expect("unique actual sources")
}
fn normalize<'a>(
    canonical: &'a CanonicalInputs,
    sources: &'a FormulaSources,
) -> FormulaNormalizedSources<'a> {
    let count = sources.sources().len();
    let nodes: usize = sources
        .sources()
        .iter()
        .map(|s| s.expression().node_count())
        .sum();
    FormulaSourceInputs::new(canonical, sources)
        .expect("actual complete sources/targets")
        .normalize_sources(FormulaSourceNormalizationLimits::new(
            count,
            nodes,
            count + 2 * nodes,
        ))
        .expect("complete retained source allowance")
}
fn check_limits(normalized: &FormulaNormalizedSources<'_>) -> FormulaSourceCheckLimits {
    FormulaSourceCheckLimits::new(
        normalized.sources().len(),
        normalized.node_count(),
        normalized.inputs().derived_inputs().count(),
        normalized.sources().len() + 2 * normalized.node_count(),
    )
}
fn names(rows: &[(u128, &str)]) -> Vec<(EntityId, MachineToken)> {
    rows.iter()
        .map(|(identity, name)| {
            (
                id(*identity),
                MachineToken::new(*name).expect("actual name"),
            )
        })
        .collect()
}
fn namespace<'a>(
    canonical: &'a CanonicalInputs,
    lengths: &'a [(EntityId, MachineToken)],
    scalars: &'a [(EntityId, MachineToken)],
) -> FormulaNamespace<'a> {
    let lengths = lengths.iter().map(|(identity, name)| {
        Declaration::length_input(
            name,
            FormulaInputOrigin::Parameter,
            id(5000 + identity.as_bits()),
            canonical.length(*identity).expect("actual owner"),
        )
    });
    let scalars = scalars.iter().map(|(identity, name)| {
        Declaration::input(
            name,
            FormulaScalarInputOrigin::Parameter,
            id(5000 + identity.as_bits()),
            *identity,
            FormulaBindingKind::from(canonical.scalar(*identity).expect("actual owner").kind()),
        )
    });
    FormulaNamespace::new(lengths.chain(scalars).map(|declaration| {
        FormulaInitialDeclaration::try_from(declaration).expect("actual initial metadata")
    }))
    .expect("unique actual names")
}

#[test]
fn actual_derived_edges_keep_both_families_repeated_untaken_uses_endpoints_and_original_spans() {
    let canonical = CanonicalInputs::new(
        vec![
            length(10, LengthState::Derived { formula: id(9002) }),
            length(
                11,
                LengthState::Unknown {
                    observation: id(999),
                },
            ),
            length(
                12,
                LengthState::Preference {
                    value: Length::ZERO,
                    provenance: id(999),
                },
            ),
            length(13, LengthState::Derived { formula: id(9001) }),
            length(
                14,
                LengthState::Known {
                    value: Length::ZERO,
                    evidence: vec![id(999)],
                },
            ),
            length(
                15,
                LengthState::Assumed {
                    value: Length::ZERO,
                    assumption: id(999),
                },
            ),
        ],
        vec![
            scalar(20, ScalarInputState::Derived { formula: id(9003) }),
            scalar(
                21,
                ScalarInputState::Unknown {
                    observation: id(999),
                },
            ),
            scalar(22, ScalarInputState::Derived { formula: id(9004) }),
        ],
    )
    .expect("actual canonical owners");
    let before = canonical.clone();
    let originals = sources(&[
        (9003, "1"),
        (9001, "if(is_base_size,a+a,eps_phys)+u+p+k+h"),
        (9002, "1 mm"),
        (9004, "n+n+uc"),
        (9009, "tan(1 deg)"),
    ]);
    let normalized = normalize(&canonical, &originals);
    let length_names = names(&[(10, "a"), (11, "u"), (12, "p"), (14, "k"), (15, "h")]);
    let scalar_names = names(&[(20, "n"), (21, "uc")]);
    let namespace = namespace(&canonical, &length_names, &scalar_names);
    let checked = normalized
        .check_sources(&namespace, check_limits(&normalized))
        .expect("actual kind/source proofs");
    let allowance = Limits::new(5, 4, 10);
    assert_eq!(
        (
            allowance.sources(),
            allowance.edges(),
            allowance.retained_records()
        ),
        (5, 4, 10)
    );
    let deps = checked
        .prepare_dependencies(allowance)
        .expect("complete prospective relation");
    assert!(std::ptr::eq(deps.checked(), &checked));
    assert_eq!(deps.limits(), allowance);
    assert_eq!(deps.retained_records(), 10);
    let edges: Vec<_> = deps.edges().collect();
    let actual: Vec<_> = edges
        .iter()
        .map(|edge| {
            (
                edge.consumer().normalized().source().id(),
                edge.supplier().normalized().source().id(),
                edge.name_use().declaration().name(),
            )
        })
        .collect();
    assert_eq!(
        actual,
        vec![
            (id(9001), id(9002), "a"),
            (id(9001), id(9002), "a"),
            (id(9004), id(9003), "n"),
            (id(9004), id(9003), "n")
        ]
    );
    for edge in &edges {
        let span = edge.name_use().span();
        assert_eq!(
            edge.consumer()
                .normalized()
                .source()
                .expression()
                .source()
                .get(span.start()..span.end()),
            Some(edge.name_use().declaration().name())
        );
        match edge.canonical_input().adapter() {
            Adapter::Length(input) => assert!(std::ptr::eq(
                input.canonical_declaration(),
                canonical.length(id(10)).expect("actual Length")
            )),
            Adapter::Scalar(input) => assert!(std::ptr::eq(
                input.canonical_declaration(),
                canonical.scalar(id(20)).expect("actual Count")
            )),
        }
    }
    let mut first = deps.outgoing(id(9001)).expect("actual expression entity");
    assert_ne!(
        first.next().expect("first actual use").name_use().span(),
        first.next().expect("second actual use").name_use().span()
    );
    assert_eq!(deps.outgoing(id(9004)).expect("Count consumer").len(), 2);
    assert_eq!(
        deps.outgoing(id(9009))
            .expect("unused constant vertex retained")
            .len(),
        0
    );
    assert_eq!(
        deps.outgoing(id(9003))
            .expect("constant Count supplier")
            .len(),
        0
    );
    assert!(deps.outgoing(id(10)).is_none()); // A canonical declaration is not a source entity.
    assert_eq!(
        deps.edges()
            .next_back()
            .expect("last original edge")
            .name_use()
            .declaration()
            .name(),
        "n"
    );
    assert!(!format!("{deps:?}").contains("eps_phys"));
    assert_eq!(canonical, before);
    assert_eq!(
        canonical
            .length(id(10))
            .expect("same record")
            .authored_value(),
        Err(LengthValueError::RequiresEvaluation {
            declaration: id(10),
            formula: id(9002)
        })
    );
}

#[test]
fn cyclic_and_unused_self_relations_are_preserved_for_the_separate_required_cycle_phase() {
    let canonical = CanonicalInputs::new(
        vec![
            length(10, LengthState::Derived { formula: id(9001) }),
            length(11, LengthState::Derived { formula: id(9002) }),
            length(12, LengthState::Derived { formula: id(9003) }),
        ],
        vec![],
    )
    .expect("actual canonical owners");
    let originals = sources(&[(9001, "b"), (9002, "a"), (9003, "r")]);
    let normalized = normalize(&canonical, &originals);
    let names = names(&[(10, "a"), (11, "b"), (12, "r")]);
    let namespace = namespace(&canonical, &names, &[]);
    let checked = normalized
        .check_sources(&namespace, check_limits(&normalized))
        .expect("kinds do not prove acyclicity");
    let deps = checked
        .prepare_dependencies(Limits::new(3, 3, 7))
        .expect("complete original relation only");
    let endpoints: Vec<_> = deps
        .edges()
        .map(|edge| {
            (
                edge.consumer().normalized().source().id(),
                edge.supplier().normalized().source().id(),
            )
        })
        .collect();
    assert_eq!(
        endpoints,
        vec![
            (id(9001), id(9002)),
            (id(9002), id(9001)),
            (id(9003), id(9003))
        ]
    );
    for (allowance, category) in [
        (Limits::new(2, 0, 0), "sources"),
        (Limits::new(3, 2, 0), "edges"),
        (Limits::new(3, 3, 6), "retained"),
    ] {
        let error = checked
            .prepare_dependencies(allowance)
            .expect_err("complete refusal before allocation");
        assert!(std::ptr::eq(error.checked(), &checked));
        assert_eq!(error.limits(), allowance);
        assert!(matches!(
            (category, error.refusal()),
            (
                "sources",
                Refusal::Sources {
                    allowed: 2,
                    required: 3
                }
            ) | (
                "edges",
                Refusal::Edges {
                    allowed: 2,
                    required: 3
                }
            ) | (
                "retained",
                Refusal::RetainedRecords {
                    allowed: 6,
                    required: 7
                }
            )
        ));
        assert!(std::error::Error::source(&error).is_none());
    }
}

#[test]
fn empty_relation_charges_its_actual_terminal_offset_and_invents_no_vertex_or_edge() {
    let canonical = CanonicalInputs::new(vec![], vec![]).expect("empty actual owner");
    let originals = sources(&[]);
    let normalized = normalize(&canonical, &originals);
    let checked = normalized
        .check_sources(
            &FormulaNamespace::new([]).expect("initial scope"),
            FormulaSourceCheckLimits::new(0, 0, 0, 0),
        )
        .expect("empty complete source proofs");
    assert!(matches!(
        checked
            .prepare_dependencies(Limits::new(0, 0, 0))
            .expect_err("terminal offset costs one record")
            .refusal(),
        Refusal::RetainedRecords {
            allowed: 0,
            required: 1
        }
    ));
    let deps = checked
        .prepare_dependencies(Limits::new(0, 0, 1))
        .expect("explicit empty offset allowance");
    assert_eq!(deps.checked().sources().len(), 0);
    assert_eq!(deps.retained_records(), 1);
    assert_eq!(deps.edges().len(), 0);
    assert!(deps.outgoing(id(0)).is_none());
}
