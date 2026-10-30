//! Actual policy-aware expression execution,without caller-seeded AST values or dependencies.
#![allow(clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    recipe::{
        FormulaBindingKind, FormulaBuiltin as B, FormulaCheckedExpression, FormulaDeclaration as D,
        FormulaEvaluatedTangentError, FormulaExecutedScalarExpression, FormulaExecutionLimits,
        FormulaExpression, FormulaInitialDeclaration, FormulaKind as K, FormulaNamespace,
        FormulaNormalizedExpression, FormulaRational as R,
        FormulaScalarExecutionContext as Context, FormulaScalarExecutionError as Error,
        FormulaScalarExecutionErrorCause as Cause, FormulaScalarExecutionFailure as Failure,
        FormulaScalarExecutionPlanRefusal as Refusal, FormulaScalarInputOrigin, FormulaSizeContext,
        FormulaTangentError as TangentError, FormulaTangentLimits as Limits,
        FormulaTangentResource as Unit, FormulaToleranceComparisonRefusal, FormulaToleranceContext,
    },
    value::CanonicalInputs,
};
use std::error::Error as _;

fn normalized(source: &str) -> FormulaNormalizedExpression<'_> {
    FormulaExpression::parse(source)
        .expect("actual syntax")
        .normalize_literals()
        .expect("actual normalization")
}
fn context<'a>() -> Context<'a> {
    Context::new(
        FormulaToleranceContext::empty(),
        FormulaSizeContext::empty(),
    )
}
fn execute<'a, 'e: 'a, 's: 'a, 'd: 'a>(
    proof: &'a FormulaCheckedExpression<'e, 's, 'd>,
    inputs: &'a CanonicalInputs,
    limits: Limits,
) -> Result<FormulaExecutedScalarExpression<'a>, Error<'a>> {
    proof
        .admit_execution(FormulaExecutionLimits::new(256, 768))
        .expect("complete workspace")
        .prepare_scalar_with_tangent_limits(inputs, limits)
        .expect("complete actual plan")
        .evaluate(context())
}
fn failure<'a, 'r>(error: &'r Error<'a>) -> &'r Failure<'a> {
    let Cause::Evaluation(failure) = error.cause() else {
        unreachable!("actual evaluation fixture")
    };
    failure
}
fn tangent<'a, 'r>(error: &'r Error<'a>) -> &'r FormulaEvaluatedTangentError {
    let Failure::Tangent(error) = failure(error) else {
        unreachable!("actual original tangent failure")
    };
    error
}
fn angle_literal(microdegrees: u128) -> String {
    format!(
        "{}.{:06} deg",
        microdegrees / 1_000_000,
        microdegrees % 1_000_000
    )
}

#[test]
fn complete_missing_policy_precedes_joins_even_in_an_untaken_call() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("actual immutable inventory");
    let missing = MachineToken::new("missing").expect("name");
    let namespace = FormulaNamespace::new([FormulaInitialDeclaration::try_from(D::input(
        &missing,
        FormulaScalarInputOrigin::Parameter,
        EntityId::from_bits(1),
        EntityId::from_bits(99),
        FormulaBindingKind::Length,
    ))
    .expect("actual declaration")])
    .expect("namespace");
    let source = "if(1 mm<2 mm,missing,tan(0 deg)*1 mm)";
    let expression = normalized(source);
    let proof = expression
        .check_kinds(&namespace)
        .expect("complete actual kinds");
    let error = proof
        .admit_execution(FormulaExecutionLimits::new(256, 768))
        .expect("workspace")
        .prepare_scalar(&inputs)
        .expect_err("missing policy before canonical join");
    assert!(matches!(error.refusal(), Refusal::MissingTangentPolicy));
    assert_eq!(error.tangent_limits(), None);
    assert!(core::ptr::eq(error.inputs(), &inputs));
    assert_eq!(
        source.get(error.node().span().start()..error.node().span().end()),
        Some("tan(0 deg)")
    );
    let limits = Limits::new(0, 0);
    let error = proof
        .admit_execution(FormulaExecutionLimits::new(256, 768))
        .expect("workspace")
        .prepare_scalar_with_tangent_limits(&inputs, limits)
        .expect_err("actual missing canonical owner next");
    assert!(matches!(error.refusal(), Refusal::CanonicalInput(_)));
    assert_eq!(error.tangent_limits(), Some(limits));
    assert!(core::ptr::eq(error.inputs(), &inputs));
}

#[test]
fn actual_native_calls_keep_original_proof_inventory_policy_and_input_contributions() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let namespace = FormulaNamespace::new([]).expect("namespace");
    for (source, expected, calls) in [
        ("tan(0 deg)", 0, vec![B::Tan]),
        ("tan(45 deg)", 1_000_000, vec![B::Tan]),
        ("tan(-60 deg)", -1_732_051, vec![B::Tan]),
        ("tan(180 deg)", 0, vec![B::Tan]),
        ("tan(30 deg*cos(0 deg))", 577350, vec![B::Cos, B::Tan]),
        ("tan(30 deg)+tan(30 deg)", 1_154_700, vec![B::Tan]),
    ] {
        let expression = normalized(source);
        let proof = expression.check_kinds(&namespace).expect("actual kinds");
        let limits = Limits::new(2073, 512);
        let result = execute(&proof, &inputs, limits).expect("actual complete expression");
        assert_eq!(result.value().kind(), K::Ratio);
        assert_eq!(
            result.value().numeric().expect("Ratio").rational(),
            R::from_signed_integer(expected)
        );
        assert_eq!(result.value().sources().iter().collect::<Vec<_>>(), calls);
        assert_eq!(result.plan().tangent_limits(), Some(limits));
        assert!(core::ptr::eq(result.inputs(), &inputs));
        assert!(core::ptr::eq(result.root().expression(), &expression));
        assert_eq!(result.reads().count(), 0);
    }
}

#[test]
fn selected_policy_is_static_but_untaken_calls_consume_no_value_or_source_resources() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let namespace = FormulaNamespace::new([]).expect("namespace");
    for source in [
        "if(1==1,1 mm,tan(90 deg)*1 mm)",
        "if(1!=1,tan(30 deg)*1 mm,1 mm)",
    ] {
        let expression = normalized(source);
        let proof = expression
            .check_kinds(&namespace)
            .expect("both actual branches");
        let limits = Limits::new(0, 0);
        let result = execute(&proof, &inputs, limits).expect("no actual tangent call");
        assert_eq!(
            result.value().numeric().expect("Length").rational(),
            R::from_signed_integer(1000)
        );
        assert!(result.value().sources().is_empty());
        assert_eq!(result.plan().tangent_limits(), Some(limits));
    }
    let expression = normalized("if(tan(45 deg)!=1.0,1 mm,2 mm)");
    let proof = expression
        .check_kinds(&namespace)
        .expect("actual false condition");
    let result = execute(&proof, &inputs, Limits::new(156, 0))
        .expect("exact Tan call and selected false branch");
    assert_eq!(
        result.value().numeric().expect("Length").rational(),
        R::from_signed_integer(2000)
    );
    assert_eq!(
        result.value().sources().iter().collect::<Vec<_>>(),
        [B::Tan]
    );
}

#[test]
fn actual_tan_contribution_reaches_original_tolerance_class_failure() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let namespace = FormulaNamespace::new([]).expect("namespace");
    let expression = normalized("within(tan(0 deg),0.0,eps_num)");
    let proof = expression.check_kinds(&namespace).expect("actual kinds");
    let limits = Limits::new(156, 0);
    let error = execute(&proof, &inputs, limits).expect_err("T1 source guard");
    assert_eq!(error.tangent_limits(), Some(limits));
    let Failure::Comparison(original) = failure(&error) else {
        unreachable!("actual class cause")
    };
    assert!(matches!(
        original.refusal(),
        FormulaToleranceComparisonRefusal::Approximation(_)
    ));
    let expression = normalized("within(tan(0 deg),0.0,eps_geo)");
    let proof = expression.check_kinds(&namespace).expect("actual kinds");
    let result = execute(&proof, &inputs, limits).expect("actual T2 comparison");
    assert_eq!(result.value().boolean(), Some(true));
    assert_eq!(
        result.value().sources().iter().collect::<Vec<_>>(),
        [B::Tan]
    );
}

#[test]
fn exact_pole_and_first_reservations_retain_original_nodes_policy_and_typed_causes() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let namespace = FormulaNamespace::new([]).expect("namespace");
    for (source, limits, request) in [
        (
            "tan(0 deg)",
            Limits::new(155, 0),
            Some((Unit::IntegerBits, 156)),
        ),
        (
            "tan(30 deg)",
            Limits::new(2072, 512),
            Some((Unit::IntegerBits, 2073)),
        ),
        (
            "tan(30 deg)",
            Limits::new(2073, 511),
            Some((Unit::SeriesTerms, 512)),
        ),
        ("1.0+tan(-270 deg)", Limits::new(156, 0), None),
    ] {
        let expression = normalized(source);
        let proof = expression.check_kinds(&namespace).expect("actual kinds");
        let error = execute(&proof, &inputs, limits).expect_err("actual Tan refusal");
        let original = tangent(&error);
        assert_eq!(error.tangent_limits(), Some(limits));
        assert_eq!(original.limits(), limits);
        assert!(core::ptr::eq(error.node().expression(), &expression));
        assert!(core::ptr::eq(
            error
                .source()
                .expect("typed original chain")
                .downcast_ref()
                .expect("original wrapper"),
            original
        ));
        if let Some((unit, requested)) = request {
            assert_eq!(original.diagnostic_code(), None);
            let Some(TangentError::Resources(resource)) = original.preparation_error() else {
                unreachable!("actual quota cause")
            };
            assert_eq!(resource.unit(), unit);
            assert_eq!(resource.requested(), requested);
        } else {
            assert_eq!(
                original.preparation_error(),
                Some(TangentError::Pole {
                    input: R::from_signed_integer(-270_000_000)
                })
            );
            assert_eq!(
                source.get(error.node().span().start()..error.node().span().end()),
                Some("tan(-270 deg)")
            );
        }
    }
}

#[test]
fn actual_full_width_literal_arithmetic_retains_later_tangent_resources_and_certified_result() {
    let n = 247385123034593398372138492992328993860_u128;
    let d = 8635369834810561370044731507908276509_u128;
    let source = format!("tan((({}/{d})*cos(0 deg)))", angle_literal(n));
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let namespace = FormulaNamespace::new([]).expect("namespace");
    let expression = normalized(&source);
    let proof = expression
        .check_kinds(&namespace)
        .expect("actual full-width intermediate Angle");
    let limits = Limits::new(4122, 3584);
    let error = execute(&proof, &inputs, limits).expect_err("next actual iteration");
    assert_eq!(error.tangent_limits(), Some(limits));
    let original = tangent(&error);
    assert_eq!(
        original
            .input()
            .numeric()
            .expect("actual completed Angle")
            .rational(),
        R::new(n, d, false).expect("independent input")
    );
    assert_eq!(
        original.input().sources().iter().collect::<Vec<_>>(),
        [B::Cos]
    );
    let native = original
        .execution_error()
        .expect("original actual native progress");
    assert_eq!(native.precision(), Some(512));
    assert_eq!(native.charged_series_terms(), 1536);
    let TangentError::Resources(resource) = native.cause() else {
        unreachable!("actual resource refusal")
    };
    assert_eq!(resource.unit(), Unit::IntegerBits);
    assert_eq!(resource.requested(), 8219);
    let limits = Limits::new(8219, 3584);
    let result = execute(&proof, &inputs, limits).expect("actual certified zero");
    assert_eq!(result.value().numeric().expect("Ratio").rational(), R::ZERO);
    assert_eq!(
        result.value().sources().iter().collect::<Vec<_>>(),
        [B::Cos, B::Tan]
    );
    assert_eq!(result.plan().tangent_limits(), Some(limits));
}

#[test]
fn earlier_non_tangent_failure_retains_selected_policy_without_inventing_a_call() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let namespace = FormulaNamespace::new([]).expect("namespace");
    let expression = normalized("1 mm/0");
    let proof = expression.check_kinds(&namespace).expect("actual kinds");
    let limits = Limits::new(2073, 512);
    let error = execute(&proof, &inputs, limits).expect_err("actual division cause");
    assert_eq!(error.tangent_limits(), Some(limits));
    assert!(matches!(failure(&error), Failure::Numeric(_)));
    let error = proof
        .admit_execution(FormulaExecutionLimits::new(256, 768))
        .expect("workspace")
        .prepare_scalar(&inputs)
        .expect("legacy available scope")
        .evaluate(context())
        .expect_err("same division cause");
    assert_eq!(error.tangent_limits(), None);
    assert!(matches!(failure(&error), Failure::Numeric(_)));
}

#[test]
fn actual_width_failure_keeps_original_sourced_angle_and_exact_native_bit_count() {
    let d = u128::MAX / 90_000_000;
    let n = 90_000_000 * d - 1;
    let source = format!("tan(-({}/{d})*cos(0 deg))", angle_literal(n));
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let namespace = FormulaNamespace::new([]).expect("namespace");
    let expression = normalized(&source);
    let proof = expression.check_kinds(&namespace).expect("actual Angle");
    let limits = Limits::new(4122, 1536);
    let error = execute(&proof, &inputs, limits).expect_err("actual width");
    assert_eq!(error.tangent_limits(), Some(limits));
    let original = tangent(&error);
    assert_eq!(
        original.input().numeric().expect("Angle").rational(),
        R::new(n, d, true).expect("original rational")
    );
    assert_eq!(
        original.input().sources().iter().collect::<Vec<_>>(),
        [B::Cos]
    );
    let native = original.execution_error().expect("actual native failure");
    assert_eq!(native.precision(), Some(256));
    assert_eq!(native.charged_series_terms(), 1536);
    assert!(matches!(
        native.cause(),
        TangentError::Numeric(sc_core::recipe::FormulaNumericError::Rational(
            sc_core::recipe::FormulaRationalError::RationalWidth {
                component: sc_core::recipe::FormulaRationalComponent::Numerator,
                measured_bits: 148,
            }
        ))
    ));
}
