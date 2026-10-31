//! Actual source-bound point argument proofs; no coordinate or geometry execution.
#![allow(clippy::expect_used)]
use sc_core::{
    construction::{
        PointConstruction as Point, PointConstructionCheckRefusal as Check,
        PointConstructionNormalizationLimits as Limits,
        PointConstructionNormalizationRefusal as Normal, PointCoordinate as Axis,
    },
    name::MachineToken,
    ontology::{EntityId, LocalTag, PointRef},
    recipe::{
        FormulaCanonicalInput, FormulaDeclaration, FormulaExpressionCheckRefusal,
        FormulaExpressionDocument as Document, FormulaInitialDeclaration, FormulaInputOrigin,
        FormulaKind, FormulaNamespace,
    },
    value::{
        CanonicalInputs, LengthDeclaration, LengthDeclarationDefinition, LengthState,
        LengthValueError,
    },
};
fn point(x: &str, y: &str) -> Point {
    Point::new(
        EntityId::from_bits(10),
        Document::new(x.into()).expect("original x input"),
        Document::new(y.into()).expect("original y input"),
    )
}

#[test]
fn actual_original_sources_and_real_length_proofs_keep_persistent_output_without_values() {
    let alias = point("25 mm+eps_num", "1 um/0");
    let point = point(" (2.5 cm + eps_num) ", "1 um/0");
    let normalized = point
        .normalized(Limits::new(6, 14))
        .expect("complete exact allowance");
    let proof = normalized
        .check_arguments(&FormulaNamespace::new([]).expect("namespace"))
        .expect("both real Length arguments");
    assert!(std::ptr::eq(proof.normalized(), &normalized));
    assert!(std::ptr::eq(proof.normalized().point(), &point));
    assert_eq!(
        point.output(),
        PointRef::new(EntityId::from_bits(10), LocalTag::FIRST)
    );
    for axis in [Axis::X, Axis::Y] {
        let coordinate = proof.coordinate(axis);
        assert_eq!(coordinate.kind(), FormulaKind::Length);
        assert!(std::ptr::eq(
            coordinate.expression(),
            normalized.coordinate(axis)
        ));
        assert_eq!(
            &coordinate.canonical_expression(),
            point.coordinate(axis).canonical()
        );
    }
    let span = proof.coordinate(Axis::X).root().span();
    assert_eq!(
        point
            .coordinate(Axis::X)
            .source()
            .get(span.start()..span.end()),
        Some("(2.5 cm + eps_num)")
    );
    assert_eq!(proof.coordinate(Axis::Y).node_count(), 3);
    // Successful static proof of 1 um/0 demonstrates that no division/provider/point was executed.
    assert_ne!(point, alias);
    assert_eq!(point.output(), alias.output());
    assert_eq!(
        point.coordinate(Axis::X).canonical(),
        alias.coordinate(Axis::X).canonical()
    );
    assert!(!format!("{point:?}").contains("eps_num"));
}

#[test]
fn actual_unknown_canonical_input_remains_unread_and_original_supplier_is_retained() {
    let record = LengthDeclaration::new(LengthDeclarationDefinition {
        id: EntityId::from_bits(40),
        source: EntityId::from_bits(30),
        state: LengthState::Unknown {
            observation: EntityId::from_bits(99),
        },
    })
    .expect("actual unknown record");
    let inputs = CanonicalInputs::new(vec![record], vec![]).expect("actual inventory");
    let before = inputs.clone();
    let record = inputs
        .length(EntityId::from_bits(40))
        .expect("original record");
    let name = MachineToken::new("private_width").expect("name");
    let metadata = FormulaDeclaration::length_input(
        &name,
        FormulaInputOrigin::Measurement,
        EntityId::from_bits(20),
        record,
    );
    let namespace = FormulaNamespace::new([
        FormulaInitialDeclaration::try_from(metadata).expect("real initial metadata")
    ])
    .expect("actual namespace");
    let point = point("private_width/4", "0 mm");
    let normalized = point
        .normalized(Limits::new(4, 10))
        .expect("actual complete shapes");
    let proof = normalized
        .check_arguments(&namespace)
        .expect("unknown is a real static Length");
    let dependency = proof
        .coordinate(Axis::X)
        .dependencies()
        .first()
        .expect("original name use");
    assert_eq!(dependency.declaration().name(), "private_width");
    let joined = FormulaCanonicalInput::resolve(dependency.declaration(), &inputs)
        .expect("original actual canonical join");
    assert!(std::ptr::eq(joined.inputs(), &inputs));
    assert_eq!(inputs, before);
    assert_eq!(
        record.authored_value(),
        Err(LengthValueError::RequiresObservation {
            declaration: EntityId::from_bits(40),
            observation: EntityId::from_bits(99),
        })
    );
}

#[test]
fn later_kind_refusal_precedes_earlier_runtime_fault_and_both_arguments_keep_actual_context() {
    let namespace = FormulaNamespace::new([]).expect("namespace");
    let early = point("1", "later_missing");
    let point = point("1 um/0", " (1 deg) ");
    let normalized = point
        .normalized(Limits::new(4, 10))
        .expect("complete original arguments");
    let error = normalized
        .check_arguments(&namespace)
        .expect_err("real y is Angle");
    assert!(std::ptr::eq(error.normalized(), &normalized));
    assert_eq!(error.coordinate(), Axis::Y);
    assert_eq!(error.required_kind(), FormulaKind::Length);
    assert_eq!(error.token(), "formula_dimension");
    assert!(matches!(
        error.refusal(),
        Check::Kind {
            actual: FormulaKind::Angle
        }
    ));
    let span = error.span();
    assert_eq!(
        point
            .coordinate(Axis::Y)
            .source()
            .get(span.start()..span.end()),
        Some("(1 deg)")
    );
    let normalized = early
        .normalized(Limits::new(2, 6))
        .expect("complete original arguments");
    let error = normalized
        .check_arguments(&namespace)
        .expect_err("x Count precedes later unbound y");
    assert_eq!(error.coordinate(), Axis::X);
    assert!(matches!(
        error.refusal(),
        Check::Kind {
            actual: FormulaKind::Count
        }
    ));
}

#[test]
fn original_expression_name_and_untaken_branch_failures_are_not_replaced_by_root_kind_errors() {
    let namespace = FormulaNamespace::new([]).expect("namespace");
    for source in [" private_secret ", "if(1==1,1 um,1 deg)"] {
        let point = point(source, "0 mm");
        let normalized = point
            .normalized(Limits::new(16, 34))
            .expect("complete shape allowance");
        let error = normalized
            .check_arguments(&namespace)
            .expect_err("original expression must refuse");
        let Check::Expression(original) = error.refusal() else {
            unreachable!("actual expression refusal");
        };
        assert!(std::ptr::eq(
            original.expression(),
            normalized.coordinate(Axis::X)
        ));
        assert_eq!(error.coordinate(), Axis::X);
        assert_eq!(error.span(), original.span());
        if source.contains("private_secret") {
            assert!(matches!(
                original.refusal(),
                FormulaExpressionCheckRefusal::UnboundName(_)
            ));
            assert_eq!(error.token(), "formula_unbound_name");
            assert!(!format!("{error:?}").contains("private_secret"));
        } else {
            assert!(matches!(
                original.refusal(),
                FormulaExpressionCheckRefusal::Dimension(_)
            ));
            assert_eq!(error.token(), "formula_dimension");
        }
    }
}

#[test]
fn complete_normalization_quota_counts_both_arguments_and_preserves_original_documents() {
    let point = point("1 mm+2 mm", "3 mm/0");
    let before = point.clone();
    let limits = Limits::new(5, 0);
    let error = point
        .normalized(limits)
        .expect_err("complete nodes before retained quota");
    assert!(std::ptr::eq(error.point(), &point));
    assert_eq!(error.limits(), limits);
    assert!(matches!(
        error.refusal(),
        Normal::Nodes {
            allowed: 5,
            required: 6
        }
    ));
    let error = point
        .normalized(Limits::new(6, 13))
        .expect_err("complete retained records");
    assert!(matches!(
        error.refusal(),
        Normal::RetainedRecords {
            allowed: 13,
            required: 14
        }
    ));
    let limits = Limits::new(6, 14);
    let normalized = point
        .normalized(limits)
        .expect("exact complete populations");
    assert_eq!(normalized.node_count(), 6);
    assert_eq!(normalized.retained_records(), 14);
    assert_eq!(normalized.limits(), limits);
    assert_eq!(point, before);
}
