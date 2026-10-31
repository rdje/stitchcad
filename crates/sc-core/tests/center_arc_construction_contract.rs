//! Actual center-variant scalar roles, source ownership and complete admission, without geometry.
#![allow(clippy::expect_used)]
use sc_core::{
    construction::{
        CenterArcArgument as Arg, CenterArcCheckRefusal as Check, CenterArcConstruction as Arc,
        CenterArcNormalizationLimits as Limits, CenterArcNormalizationRefusal as Normal,
    },
    name::MachineToken,
    ontology::{EdgeRef, EntityId, LocalTag, PointRef},
    recipe::{
        FormulaCanonicalInput, FormulaDeclaration, FormulaExpressionCheckRefusal,
        FormulaExpressionDocument as Doc, FormulaInitialDeclaration, FormulaInputOrigin,
        FormulaKind, FormulaNamespace,
    },
    value::{
        CanonicalInputs, LengthDeclaration, LengthDeclarationDefinition, LengthState,
        LengthValueError,
    },
};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn arc(radius: &str, start: &str, sweep: &str) -> Arc {
    Arc::new(
        id(10),
        PointRef::new(id(99), LocalTag::new(4)),
        Doc::new(radius.into()).expect("original radius input"),
        Doc::new(start.into()).expect("original start input"),
        Doc::new(sweep.into()).expect("original sweep input"),
    )
    .expect("not a local self-supply")
}

#[test]
fn actual_original_argument_proofs_preserve_raw_turns_and_authentic_center_without_values() {
    let arc = arc("1 cm/0", " (450 deg) ", "-720 deg");
    let center = arc.center_dependency();
    assert!(std::ptr::eq(center.arc(), &arc));
    assert_eq!(center.point(), PointRef::new(id(99), LocalTag::new(4)));
    assert_eq!(arc.output(), EdgeRef::new(id(10), LocalTag::FIRST));
    let normalized = arc
        .normalized(Limits::new(6, 15))
        .expect("all three actual shapes");
    let checked = normalized
        .check_arguments(&FormulaNamespace::new([]).expect("namespace"))
        .expect("Length/Angle/Angle without division or curve construction");
    assert!(std::ptr::eq(checked.normalized(), &normalized));
    assert!(std::ptr::eq(checked.normalized().arc(), &arc));
    for argument in [Arg::Radius, Arg::StartAngle, Arg::SweepAngle] {
        let proof = checked.argument(argument);
        assert_eq!(proof.kind(), argument.required_kind());
        assert!(std::ptr::eq(
            proof.expression(),
            normalized.argument(argument)
        ));
        assert_eq!(
            &proof.canonical_expression(),
            arc.argument(argument).canonical()
        );
    }
    assert_eq!(
        checked
            .argument(Arg::StartAngle)
            .canonical_expression()
            .as_str(),
        "angle:450000000"
    );
    assert_eq!(
        checked
            .argument(Arg::SweepAngle)
            .canonical_expression()
            .as_str(),
        "(- angle:720000000)"
    );
    let span = checked.argument(Arg::StartAngle).root().span();
    assert_eq!(
        arc.argument(Arg::StartAngle)
            .source()
            .get(span.start()..span.end()),
        Some("(450 deg)")
    );
}

#[test]
fn later_wrong_sweep_kind_refuses_before_earlier_radius_runtime_failure() {
    let arc = arc("1 mm/0", "0 deg", " (1) ");
    let normalized = arc
        .normalized(Limits::new(5, 13))
        .expect("complete actual arguments");
    let error = normalized
        .check_arguments(&FormulaNamespace::new([]).expect("namespace"))
        .expect_err("sweep Count is not Angle");
    assert!(std::ptr::eq(error.normalized(), &normalized));
    assert_eq!(error.argument(), Arg::SweepAngle);
    assert_eq!(error.required_kind(), FormulaKind::Angle);
    assert_eq!(error.token(), "formula_dimension");
    assert!(matches!(
        error.refusal(),
        Check::Kind {
            actual: FormulaKind::Count
        }
    ));
    let span = error.span();
    assert_eq!(
        arc.argument(Arg::SweepAngle)
            .source()
            .get(span.start()..span.end()),
        Some("(1)")
    );
}

#[test]
fn actual_role_order_and_original_name_or_untaken_branch_causes_survive_shared_checking() {
    for (radius, start, wanted, token) in [
        ("1", "missing_angle", Arg::Radius, "formula_dimension"),
        (
            "1 mm",
            " private_secret ",
            Arg::StartAngle,
            "formula_unbound_name",
        ),
        (
            "if(1==1,1 mm,1 deg)",
            "0 deg",
            Arg::Radius,
            "formula_dimension",
        ),
    ] {
        let arc = arc(radius, start, "0 deg");
        let normalized = arc
            .normalized(Limits::new(32, 67))
            .expect("whole arguments admitted");
        let error = normalized
            .check_arguments(&FormulaNamespace::new([]).expect("namespace"))
            .expect_err("original static refusal");
        assert_eq!(error.argument(), wanted);
        assert_eq!(error.token(), token);
        match error.refusal() {
            Check::Kind { actual } => assert_eq!(*actual, FormulaKind::Count),
            Check::Expression(original) => {
                assert!(std::ptr::eq(
                    original.expression(),
                    normalized.argument(wanted)
                ));
                assert_eq!(original.span(), error.span());
                if token == "formula_unbound_name" {
                    assert!(matches!(
                        original.refusal(),
                        FormulaExpressionCheckRefusal::UnboundName(_)
                    ));
                    assert!(!format!("{error:?}").contains("private_secret"));
                } else {
                    assert!(matches!(
                        original.refusal(),
                        FormulaExpressionCheckRefusal::Dimension(_)
                    ));
                }
            }
        }
    }
}

#[test]
fn complete_quota_measures_three_owners_and_all_nodes_before_normalization() {
    let arc = arc("1 mm/0", "90 deg", "450 deg");
    let before = arc.clone();
    let limits = Limits::new(4, 0);
    let error = arc
        .normalized(limits)
        .expect_err("all five nodes precede retained quota");
    assert!(std::ptr::eq(error.arc(), &arc));
    assert_eq!(error.limits(), limits);
    assert!(matches!(
        error.refusal(),
        Normal::Nodes {
            allowed: 4,
            required: 5
        }
    ));
    let error = arc
        .normalized(Limits::new(5, 12))
        .expect_err("three owners plus twice five nodes");
    assert!(matches!(
        error.refusal(),
        Normal::RetainedRecords {
            allowed: 12,
            required: 13
        }
    ));
    let normalized = arc
        .normalized(Limits::new(5, 13))
        .expect("exact complete allowance");
    assert_eq!(normalized.node_count(), 5);
    assert_eq!(normalized.retained_records(), 13);
    assert_eq!(arc, before);
}

#[test]
fn center_self_supply_keeps_original_reference_and_static_proofs_grant_no_geometric_domain() {
    let center = PointRef::new(id(10), LocalTag::new(9));
    let error = Arc::new(
        id(10),
        center,
        Doc::new("1".into()).expect("input"),
        Doc::new("1".into()).expect("input"),
        Doc::new("1".into()).expect("input"),
    )
    .expect_err("own creating operation cannot supply a prior center");
    assert_eq!(error.operation(), id(10));
    assert_eq!(error.center(), center);
    let declaration = arc("-1 mm", "0 deg", "0 deg");
    let normalized = declaration
        .normalized(Limits::new(4, 11))
        .expect("complete retained shape");
    let checked = normalized
        .check_arguments(&FormulaNamespace::new([]).expect("namespace"))
        .expect("kinds alone do not validate radius/sweep geometry");
    assert_eq!(checked.argument(Arg::Radius).kind(), FormulaKind::Length);
    assert_eq!(checked.argument(Arg::SweepAngle).kind(), FormulaKind::Angle);
}

#[test]
fn real_unknown_radius_keeps_original_canonical_supplier_and_stays_unknown() {
    let inputs = CanonicalInputs::new(
        vec![LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(40),
            source: id(30),
            state: LengthState::Unknown {
                observation: id(99),
            },
        })
        .expect("real unknown length")],
        vec![],
    )
    .expect("actual canonical inventory");
    let before = inputs.clone();
    let record = inputs.length(id(40)).expect("original record");
    let name = MachineToken::new("unknown_radius").expect("name");
    let namespace = FormulaNamespace::new([FormulaInitialDeclaration::try_from(
        FormulaDeclaration::length_input(&name, FormulaInputOrigin::Measurement, id(20), record),
    )
    .expect("actual metadata")])
    .expect("actual namespace");
    let arc = arc("unknown_radius", "90 deg", "180 deg");
    let normalized = arc.normalized(Limits::new(3, 9)).expect("whole shape");
    let checked = normalized
        .check_arguments(&namespace)
        .expect("Unknown remains statically Length");
    let dependency = checked
        .argument(Arg::Radius)
        .dependencies()
        .first()
        .expect("original name use");
    let joined = FormulaCanonicalInput::resolve(dependency.declaration(), &inputs)
        .expect("same actual canonical supplier");
    assert!(std::ptr::eq(joined.inputs(), &inputs));
    assert_eq!(inputs, before);
    assert_eq!(
        record.authored_value(),
        Err(LengthValueError::RequiresObservation {
            declaration: id(40),
            observation: id(99)
        })
    );
}
