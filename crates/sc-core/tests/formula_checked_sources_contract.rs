//! Complete actual initial-scope proofs, canonical dependency joins and Derived consumer kinds.
#![allow(clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    recipe::{
        FormulaBindingKind, FormulaCanonicalInputRefusal, FormulaDeclaration,
        FormulaExpressionDocument, FormulaInitialDeclaration, FormulaInputOrigin,
        FormulaKind as Kind, FormulaNamespace, FormulaNormalizedSources, FormulaSource,
        FormulaSourceCheckLimits as Limits, FormulaSourceCheckRefusal as Refusal,
        FormulaSourceInputs, FormulaSourceLimits, FormulaSourceNormalizationLimits, FormulaSources,
    },
    value::{
        CanonicalInputPosition, CanonicalInputTarget, CanonicalInputs, LengthDeclaration,
        LengthDeclarationDefinition, LengthState, LengthValueError, ScalarInputDeclaration,
        ScalarInputDeclarationDefinition, ScalarInputKind, ScalarInputState,
    },
};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn length(identity: u128, state: LengthState) -> LengthDeclaration {
    LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(identity),
        source: id(4000),
        state,
    })
    .expect("actual original length")
}
fn sources(rows: &[(u128, &str)]) -> FormulaSources {
    let sources: Vec<_> = rows
        .iter()
        .map(|(identity, source)| {
            FormulaSource::new(
                id(*identity),
                FormulaExpressionDocument::new((*source).into()).expect("actual syntax"),
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
    FormulaSources::new(sources, limits).expect("actual source identities")
}
fn normalize<'a>(
    inputs: &'a CanonicalInputs,
    sources: &'a FormulaSources,
) -> FormulaNormalizedSources<'a> {
    let nodes: usize = sources
        .sources()
        .iter()
        .map(|s| s.expression().node_count())
        .sum();
    FormulaSourceInputs::new(inputs, sources)
        .expect("actual complete owners")
        .normalize_sources(FormulaSourceNormalizationLimits::new(
            sources.sources().len(),
            nodes,
            sources.sources().len() + 2 * nodes,
        ))
        .expect("complete normalization")
}
fn limits(normalized: &FormulaNormalizedSources<'_>) -> Limits {
    Limits::new(
        normalized.sources().len(),
        normalized.node_count(),
        normalized.inputs().derived_inputs().count(),
        normalized.sources().len() + 2 * normalized.node_count(),
    )
}

#[test]
fn every_real_scalar_target_kind_uses_the_actual_source_index_and_original_proof_owner() {
    let rows = [
        (9005, "1==1"),
        (9004, "2"),
        (9003, "0.5"),
        (9002, "(1 mm)^2"),
        (9001, "30 deg"),
        (9000, "1 mm"),
    ];
    let originals = sources(&rows);
    let canonical = CanonicalInputs::new(
        vec![length(10, LengthState::Derived { formula: id(9000) })],
        ScalarInputKind::ALL
            .iter()
            .enumerate()
            .map(|(index, kind)| {
                ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                    id: id(100 + index as u128),
                    source: id(4000),
                    kind: *kind,
                    state: ScalarInputState::Derived {
                        formula: id(9000 + index as u128),
                    },
                })
                .expect("actual original declared kind")
            })
            .collect(),
    )
    .expect("actual inventory");
    let before = canonical.clone();
    let normalized = normalize(&canonical, &originals);
    let allowance = limits(&normalized);
    let checked = normalized
        .check_sources(
            &FormulaNamespace::new([]).expect("initial scope"),
            allowance,
        )
        .expect("all six actual source/target kinds");
    assert_eq!(checked.target_count(), 7);
    assert_eq!(checked.limits(), allowance);
    assert_eq!(checked.retained_records(), allowance.retained_records());
    assert!(std::ptr::eq(checked.normalized(), &normalized));
    for (actual, original) in checked.sources().iter().zip(normalized.sources()) {
        assert!(std::ptr::eq(actual.normalized(), original));
        assert!(std::ptr::eq(
            actual.proof().expression(),
            original.expression()
        ));
        assert!(std::ptr::eq(
            checked.source(original.source().id()).expect("exact id"),
            actual
        ));
    }
    for (index, kind) in ScalarInputKind::ALL.iter().enumerate() {
        assert_eq!(
            checked
                .source(id(9000 + index as u128))
                .expect("actual id")
                .proof()
                .kind(),
            Kind::from(FormulaBindingKind::from(*kind))
        );
    }
    assert!(checked.source(id(9999)).is_none());
    assert_eq!(canonical, before);
}

#[test]
fn actual_unknown_and_reserved_metadata_pass_without_values_or_retaining_namespace_allocation() {
    let canonical = CanonicalInputs::new(
        vec![
            length(
                1,
                LengthState::Unknown {
                    observation: id(80),
                },
            ),
            length(10, LengthState::Derived { formula: id(9000) }),
        ],
        vec![],
    )
    .expect("actual inventory");
    let name = MachineToken::new("actual_width").expect("actual name");
    let actual = canonical.length(id(1)).expect("actual canonical owner");
    let originals = sources(&[
        (9000, "if(is_base_size,actual_width,actual_width)"),
        (9001, "eps_phys"),
    ]);
    let normalized = normalize(&canonical, &originals);
    let checked = {
        let namespace = FormulaNamespace::new([FormulaInitialDeclaration::try_from(
            FormulaDeclaration::length_input(
                &name,
                FormulaInputOrigin::Parameter,
                id(5001),
                actual,
            ),
        )
        .expect("initial metadata")])
        .expect("actual supplied namespace");
        normalized
            .check_sources(&namespace, limits(&normalized))
            .expect("complete static sources")
    }; // The namespace map may drop; actual declaration/name owners remain borrowed.
    assert_eq!(
        checked
            .source(id(9000))
            .expect("source")
            .proof()
            .dependencies()
            .len(),
        3
    );
    assert_eq!(
        checked
            .source(id(9001))
            .expect("optional context source")
            .proof()
            .kind(),
        Kind::Length
    );
    assert_eq!(
        actual.authored_value(),
        Err(LengthValueError::RequiresObservation {
            declaration: id(1),
            observation: id(80),
        })
    );
    assert!(!format!("{checked:?}").contains("actual_width"));
}

#[test]
fn all_original_expression_checks_precede_consumer_mismatch_and_late_unused_branch_refuses() {
    let canonical = CanonicalInputs::new(
        vec![],
        vec![
            ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                id: id(10),
                source: id(4000),
                kind: ScalarInputKind::Area,
                state: ScalarInputState::Derived { formula: id(9000) },
            })
            .expect("actual Area consumer"),
        ],
    )
    .expect("inventory");
    let originals = sources(&[(9000, "1 mm"), (9001, "if(1==1,2 mm,missing)")]);
    let normalized = normalize(&canonical, &originals);
    let error = normalized
        .check_sources(
            &FormulaNamespace::new([]).expect("namespace"),
            limits(&normalized),
        )
        .expect_err("unused late source still receives whole static checking");
    let Refusal::Expression {
        source_index,
        error: cause,
    } = error.refusal()
    else {
        unreachable!("actual static source refusal");
    };
    assert_eq!(*source_index, 1);
    let source = normalized
        .sources()
        .get(1)
        .expect("actual failing original source");
    assert!(std::ptr::eq(cause.expression(), source.expression()));
    assert_eq!(cause.token(), "formula_unbound_name");
    assert_eq!(
        source
            .source()
            .expression()
            .source()
            .get(cause.span().start()..cause.span().end()),
        Some("missing")
    );
}

#[test]
fn actual_canonical_dependency_join_rejects_equal_external_owners_even_in_untaken_branches() {
    let canonical = CanonicalInputs::new(
        vec![
            length(
                1,
                LengthState::Unknown {
                    observation: id(80),
                },
            ),
            length(10, LengthState::Derived { formula: id(9000) }),
        ],
        vec![],
    )
    .expect("actual owner");
    let external = canonical.length(id(1)).expect("record").clone();
    let name = MachineToken::new("width").expect("name");
    let namespace = FormulaNamespace::new([FormulaInitialDeclaration::try_from(
        FormulaDeclaration::length_input(&name, FormulaInputOrigin::Parameter, id(5001), &external),
    )
    .expect("initial metadata")])
    .expect("namespace");
    let originals = sources(&[(9000, "if(1==1,eps_num,width)")]);
    let normalized = normalize(&canonical, &originals);
    let error = normalized
        .check_sources(&namespace, limits(&normalized))
        .expect_err("real owner required");
    let Refusal::CanonicalInput {
        source_index,
        span,
        error: cause,
    } = error.refusal()
    else {
        unreachable!("actual original canonical cause");
    };
    assert_eq!(*source_index, 0);
    assert!(std::ptr::eq(cause.inputs(), &canonical));
    assert!(
        matches!(cause.refusal(), FormulaCanonicalInputRefusal::LengthOwner { declaration } if declaration == id(1))
    );
    assert_eq!(
        originals
            .source(id(9000))
            .expect("original source")
            .expression()
            .source()
            .get(span.start()..span.end()),
        Some("width")
    );
    assert!(std::ptr::eq(error.normalized(), &normalized));
}

#[test]
fn shared_source_consumer_mismatch_retains_actual_declared_target_and_actual_root_kind() {
    let canonical = CanonicalInputs::new(
        vec![length(10, LengthState::Derived { formula: id(9000) })],
        vec![
            ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                id: id(11),
                source: id(4000),
                kind: ScalarInputKind::Count,
                state: ScalarInputState::Derived { formula: id(9000) },
            })
            .expect("actual Count consumer"),
        ],
    )
    .expect("actual inventory");
    let originals = sources(&[(9000, "1 mm")]);
    let normalized = normalize(&canonical, &originals);
    let error = normalized
        .check_sources(
            &FormulaNamespace::new([]).expect("initial scope"),
            limits(&normalized),
        )
        .expect_err("Length source cannot satisfy actual Count consumer");
    let Refusal::TargetKind {
        source_index,
        target,
        expected,
        actual,
    } = error.refusal()
    else {
        unreachable!("actual consumer kind refusal");
    };
    assert_eq!(
        (*source_index, *expected, *actual),
        (0, Kind::Count, Kind::Length)
    );
    assert_eq!(
        target.position(),
        CanonicalInputPosition::Scalar { index: 0 }
    );
    assert_eq!(
        target.target(),
        CanonicalInputTarget::Scalar {
            declaration: id(11),
            kind: ScalarInputKind::Count
        }
    );
    assert!(std::ptr::eq(
        target.scalar().expect("actual family"),
        canonical.scalar(id(11)).expect("actual owner")
    ));
    assert!(std::ptr::eq(
        target.source(),
        originals.source(id(9000)).expect("actual source")
    ));
}

#[test]
fn complete_source_node_target_and_retained_quotas_precede_static_work_and_empty_drafts_fit_zero() {
    let canonical = CanonicalInputs::new(
        vec![length(10, LengthState::Derived { formula: id(9000) })],
        vec![],
    )
    .expect("actual target");
    let originals = sources(&[(9000, "missing")]);
    let normalized = normalize(&canonical, &originals);
    let namespace = FormulaNamespace::new([]).expect("initial namespace");
    for (allowance, population) in [
        (Limits::new(0, 0, 0, 0), "sources"),
        (Limits::new(1, 0, 0, 0), "nodes"),
        (Limits::new(1, 1, 0, 0), "targets"),
        (Limits::new(1, 1, 1, 2), "retained"),
    ] {
        let error = normalized
            .check_sources(&namespace, allowance)
            .expect_err("complete quota first");
        assert_eq!(error.limits(), allowance);
        assert!(std::ptr::eq(error.normalized(), &normalized));
        assert!(matches!(
            (population, error.refusal()),
            (
                "sources",
                Refusal::Sources {
                    allowed: 0,
                    required: 1
                }
            ) | (
                "nodes",
                Refusal::Nodes {
                    allowed: 0,
                    required: 1
                }
            ) | (
                "targets",
                Refusal::Targets {
                    allowed: 0,
                    required: 1
                }
            ) | (
                "retained",
                Refusal::RetainedRecords {
                    allowed: 2,
                    required: 3
                }
            )
        ));
    }
    assert!(matches!(
        normalized
            .check_sources(&namespace, Limits::new(1, 1, 1, 3))
            .expect_err("actual language failure only after admission")
            .refusal(),
        Refusal::Expression { .. }
    ));
    let empty_inputs = CanonicalInputs::new(vec![], vec![]).expect("actual empty owner");
    let empty_sources = sources(&[]);
    let empty = normalize(&empty_inputs, &empty_sources);
    let checked = empty
        .check_sources(&namespace, Limits::new(0, 0, 0, 0))
        .expect("zero draft");
    assert!(checked.sources().is_empty());
    assert_eq!(checked.target_count(), 0);
    assert_eq!(checked.retained_records(), 0);
}
