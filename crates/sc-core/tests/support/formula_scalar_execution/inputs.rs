//! Actual general-scalar payloads and unavailable states cross the worker's selected read boundary.
use super::{
    context, execute, failure, id, normalized, Cause, Failure, FormulaBindingKind,
    FormulaInitialDeclaration, FormulaNamespace, FormulaScalarInputOrigin, D, K, R,
};
use sc_core::{
    name::MachineToken,
    recipe::FormulaScalarExecutionReadKind as Read,
    value::{
        CanonicalInputs, ScalarInputDeclaration, ScalarInputDeclarationDefinition,
        ScalarInputKind as Kind, ScalarInputReadError, ScalarInputState as State,
        ScalarInputValue as Value,
    },
};
use sc_units::Length;

#[test]
fn actual_general_numeric_reads_keep_their_family_signed_turns_and_unbound_count_range() {
    let name = MachineToken::new("general").expect("name");
    for (value, kind, expected) in [
        (Value::from(Length::ZERO), Kind::Length, 0),
        (
            Value::from_angle_microdegrees(-720_000_000),
            Kind::Angle,
            -1_440_000_000,
        ),
        (Value::from_ratio_ppm(-1_000_000), Kind::Ratio, -2_000_000),
        (
            Value::from_count(i64::MAX).expect("stored Count"),
            Kind::Count,
            i128::from(i64::MAX) * 2,
        ),
    ] {
        let inputs = CanonicalInputs::new(
            vec![],
            vec![
                ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                    id: id(7),
                    source: id(20),
                    kind,
                    state: State::Known {
                        value,
                        evidence: vec![id(32)],
                    },
                })
                .expect("actual canonical typed record"),
            ],
        )
        .expect("inventory");
        let before = inputs.clone();
        let canonical = inputs.scalar(id(7)).expect("actual general record");
        let metadata = D::input(
            &name,
            FormulaScalarInputOrigin::Material,
            id(100),
            id(7),
            FormulaBindingKind::from(kind),
        );
        let namespace =
            FormulaNamespace::new(
                [FormulaInitialDeclaration::try_from(metadata).expect("initial")],
            )
            .expect("namespace");
        let expression = normalized("general+general");
        let proof = expression
            .check_kinds(&namespace)
            .expect("actual typed proof");
        let result = execute(&proof, &inputs, context()).expect("actual unbound scalar execution");
        assert_eq!(
            result.value().kind(),
            K::from(FormulaBindingKind::from(kind))
        );
        assert_eq!(
            result.value().numeric().expect("numeric").rational(),
            R::from_signed_integer(expected)
        );
        assert_eq!(result.reads().count(), 2);
        for read in result.reads() {
            let Read::Scalar(source) = read.source() else {
                unreachable!("general family stays general");
            };
            assert!(std::ptr::eq(
                source.input().canonical_declaration(),
                canonical
            ));
            assert!(std::ptr::eq(source.input().state(), canonical.state()));
            assert_eq!(source.value(), value);
        }
        assert_eq!(inputs, before);
    }
}

#[test]
fn general_derived_and_unknown_boolean_reads_keep_original_requirements_without_shortcuts() {
    let name = MachineToken::new("general").expect("name");
    for (kind, state, source, expected) in [
        (
            Kind::Area,
            State::Derived { formula: id(31) },
            "general",
            ScalarInputReadError::RequiresEvaluation {
                declaration: id(7),
                formula: id(31),
            },
        ),
        (
            Kind::Boolean,
            State::Unknown {
                observation: id(30),
            },
            "if(general,1 mm,1 mm)",
            ScalarInputReadError::RequiresObservation {
                declaration: id(7),
                observation: id(30),
            },
        ),
    ] {
        let inputs = CanonicalInputs::new(
            vec![],
            vec![
                ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                    id: id(7),
                    source: id(20),
                    kind,
                    state,
                })
                .expect("actual canonical record"),
            ],
        )
        .expect("inventory");
        let metadata = D::input(
            &name,
            FormulaScalarInputOrigin::Profile,
            id(100),
            id(7),
            FormulaBindingKind::from(kind),
        );
        let namespace =
            FormulaNamespace::new(
                [FormulaInitialDeclaration::try_from(metadata).expect("initial")],
            )
            .expect("namespace");
        let expression = normalized(source);
        let proof = expression
            .check_kinds(&namespace)
            .expect("complete static proof");
        let error =
            execute(&proof, &inputs, context()).expect_err("actual dependency/observation needed");
        assert!(matches!(error.cause(), Cause::Evaluation(_)));
        let Failure::ScalarInput(cause) = failure(&error) else {
            unreachable!("original general read refusal");
        };
        assert_eq!(cause.reason(), expected);
        assert!(std::ptr::eq(
            cause.input().canonical_declaration(),
            inputs.scalar(id(7)).expect("actual record")
        ));
        assert_eq!(cause.input().origin(), FormulaScalarInputOrigin::Profile);
    }
}
