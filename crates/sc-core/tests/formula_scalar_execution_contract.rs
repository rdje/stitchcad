//! Real checked AST execution, selected-only source reads, exact kernels and original failures.
#![allow(clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    recipe::{
        FormulaApproximationSources, FormulaBindingKind, FormulaBuiltin as B,
        FormulaCheckedExpression, FormulaCheckedNodeForm as Form, FormulaDeclaration as D,
        FormulaExecutedScalarExpression, FormulaExecutionLimits, FormulaExpression,
        FormulaInitialDeclaration, FormulaInputOrigin, FormulaKind as K, FormulaNamespace,
        FormulaNormalizedExpression, FormulaNumericError, FormulaRational as R,
        FormulaRationalError, FormulaScalarExecutionContext as Context,
        FormulaScalarExecutionError as Error, FormulaScalarExecutionErrorCause as Cause,
        FormulaScalarExecutionFailure as Failure, FormulaScalarExecutionReadKind as Read,
        FormulaScalarInputOrigin, FormulaSizeContext, FormulaToleranceComparisonRefusal,
        FormulaToleranceContext, FormulaToleranceContextDefinition, FormulaToleranceName,
        FormulaToleranceProvider, FormulaToleranceReadError, FormulaToleranceValueSource,
    },
    value::{
        CanonicalInputs, LengthDeclaration, LengthDeclarationDefinition, LengthState,
        LengthValueError, ScalarInputDeclaration, ScalarInputDeclarationDefinition,
        ScalarInputKind, ScalarInputState, ScalarInputValue,
    },
};
use sc_units::{Length, Tolerance, ToleranceClass};
use std::error::Error as _;

#[path = "support/formula_scalar_execution/inputs.rs"]
mod general_inputs;

#[path = "support/formula_scalar_execution/oracle.rs"]
mod independent_oracle;

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn normalized(source: &str) -> FormulaNormalizedExpression<'_> {
    FormulaExpression::parse(source)
        .expect("syntax")
        .normalize_literals()
        .expect("inputs")
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
    context: Context<'a>,
) -> Result<FormulaExecutedScalarExpression<'a>, Error<'a>> {
    proof
        .admit_execution(FormulaExecutionLimits::new(256, 768))
        .expect("explicit envelope")
        .prepare_scalar(inputs)
        .expect("actual prepared scope")
        .evaluate(context)
}
fn failure<'a, 'r>(error: &'r Error<'a>) -> &'r Failure<'a> {
    let Cause::Evaluation(failure) = error.cause() else {
        unreachable!("value refusal fixture");
    };
    failure
}
fn names(result: &FormulaExecutedScalarExpression<'_>) -> Vec<String> {
    result
        .reads()
        .map(|read| {
            let Form::Name { name, .. } = read.node().form() else {
                unreachable!("actual read name");
            };
            name.to_owned()
        })
        .collect()
}
fn inputs(flag: bool) -> CanonicalInputs {
    let lengths = [
        (
            1,
            LengthState::Unknown {
                observation: id(30),
            },
        ),
        (2, LengthState::Derived { formula: id(31) }),
        (
            4,
            LengthState::Known {
                value: Length::ZERO,
                evidence: vec![id(32)],
            },
        ),
        (
            5,
            LengthState::Assumed {
                value: Length::from_micrometres(1000).expect("Length"),
                assumption: id(33),
            },
        ),
        (
            6,
            LengthState::Preference {
                value: Length::from_micrometres(-1000).expect("Length"),
                provenance: id(34),
            },
        ),
    ]
    .map(|(identity, state)| {
        LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(identity),
            source: id(20),
            state,
        })
        .expect("canonical length")
    })
    .into();
    CanonicalInputs::new(
        lengths,
        vec![
            ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                id: id(3),
                source: id(21),
                kind: ScalarInputKind::Boolean,
                state: ScalarInputState::Preference {
                    value: ScalarInputValue::from_boolean(flag),
                    provenance: id(35),
                },
            })
            .expect("actual Boolean preference"),
        ],
    )
    .expect("actual canonical inventory")
}
fn namespace<'a>(
    names: &'a [MachineToken; 3],
    inputs: &'a CanonicalInputs,
) -> FormulaNamespace<'a> {
    let [width, derived, flag] = names;
    FormulaNamespace::new(
        [
            D::length_input(
                width,
                FormulaInputOrigin::Measurement,
                id(101),
                inputs.length(id(1)).expect("actual width"),
            ),
            D::length_input(
                derived,
                FormulaInputOrigin::Parameter,
                id(102),
                inputs.length(id(2)).expect("actual derived"),
            ),
            D::input(
                flag,
                FormulaScalarInputOrigin::Parameter,
                id(103),
                id(3),
                FormulaBindingKind::Boolean,
            ),
        ]
        .map(|metadata| FormulaInitialDeclaration::try_from(metadata).expect("initial")),
    )
    .expect("namespace")
}

#[test]
fn exact_ast_operators_and_all_available_scalar_calls_compute_actual_values_and_sources() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let namespace = FormulaNamespace::new([]).expect("namespace");
    for (source, kind, numerator, denominator, sources) in [
        ("1 mm/3", K::Length, 1000_i128, 3_u128, &[][..]),
        ("0.0004 mm+0.0004 mm", K::Length, 0, 1, &[][..]),
        ("0.0005 mm+0.0005 mm", K::Length, 2, 1, &[][..]),
        ("-((2 mm)^2)", K::Area, -4_000_000, 1, &[][..]),
        ("3 mm-1 mm", K::Length, 2000, 1, &[][..]),
        ("2 mm*3 mm", K::Area, 6_000_000, 1, &[][..]),
        ("sqrt((3 mm)^2)", K::Length, 3000, 1, &["sqrt"][..]),
        ("hypot(3 mm,4 mm)", K::Length, 5000, 1, &["hypot"][..]),
        ("abs(-3 mm)", K::Length, 3000, 1, &[][..]),
        ("min(2 mm,1 mm)", K::Length, 1000, 1, &[][..]),
        ("max(2 mm,1 mm)", K::Length, 2000, 1, &[][..]),
        ("clamp(3 mm,0 mm,2 mm)", K::Length, 2000, 1, &[][..]),
        ("round_to(3 um,2 um)", K::Length, 4, 1, &["round_to"][..]),
        ("sin(90 deg)", K::Ratio, 1_000_000, 1, &["sin"][..]),
        ("cos(180 deg)", K::Ratio, -1_000_000, 1, &["cos"][..]),
        ("atan(100 pct)", K::Angle, 45_000_000, 1, &["atan"][..]),
        ("atan2(1 mm,1 mm)", K::Angle, 45_000_000, 1, &["atan2"][..]),
        (
            "arc_length(360 deg,1 mm)",
            K::Length,
            6283,
            1,
            &["arc_length"][..],
        ),
    ] {
        let expression = normalized(source);
        let proof = expression.check_kinds(&namespace).expect("complete proof");
        let result = execute(&proof, &inputs, context()).expect("actual AST result");
        assert_eq!(result.value().kind(), kind, "{source}");
        assert_eq!(
            result.value().numeric().expect("numeric").rational(),
            R::new(
                numerator.unsigned_abs(),
                denominator,
                numerator.is_negative()
            )
            .expect("expected exact value"),
            "{source}"
        );
        assert_eq!(
            result
                .value()
                .sources()
                .iter()
                .map(B::token)
                .collect::<Vec<_>>(),
            sources,
            "{source}"
        );
        assert!(std::ptr::eq(result.root().expression(), &expression));
        assert!(std::ptr::eq(result.inputs(), &inputs));
        assert_eq!(result.reads().count(), 0);
    }
    for (source, expected) in [
        ("1 mm<2 mm", true),
        ("1 mm==2 mm", false),
        ("1 mm!=2 mm", true),
        ("2 mm<=2 mm", true),
        ("2 mm>=1 mm", true),
        ("1 mm>2 mm", false),
    ] {
        let expression = normalized(source);
        let proof = expression.check_kinds(&namespace).expect("proof");
        assert_eq!(
            execute(&proof, &inputs, context())
                .expect("actual comparison")
                .value()
                .boolean(),
            Some(expected)
        );
    }
}

#[test]
fn actual_boolean_condition_selects_only_its_branch_and_keeps_canonical_source_state() {
    let tokens = ["private_width", "derived", "use_fixed"]
        .map(|name| MachineToken::new(name).expect("name"));
    for flag in [false, true] {
        let inputs = inputs(flag);
        let before = inputs.clone();
        let namespace = namespace(&tokens, &inputs);
        let expression = normalized("if(use_fixed,1 mm,private_width)");
        let proof = expression
            .check_kinds(&namespace)
            .expect("both branches passed static checks");
        if flag {
            let result = execute(&proof, &inputs, context()).expect("unknown branch is untaken");
            assert_eq!(
                result.value().numeric().expect("Length").rational(),
                R::from_signed_integer(1000)
            );
            assert_eq!(names(&result), ["use_fixed"]);
            assert_eq!(result.plan().sources().len(), 2);
            let Read::Scalar(read) = result
                .reads()
                .next()
                .expect("actual condition read")
                .source()
            else {
                unreachable!("actual general Boolean input");
            };
            assert!(std::ptr::eq(
                read.input().canonical_declaration(),
                inputs.scalar(id(3)).expect("owned")
            ));
            assert!(
                matches!(read.input().state(), ScalarInputState::Preference { value, provenance }
                if value.boolean() == Some(true) && *provenance == id(35))
            );
            assert!(!format!("{result:?}").contains("private_width"));
        } else {
            let error = execute(&proof, &inputs, context()).expect_err("actual chosen unknown");
            let Failure::LengthInput(cause) = failure(&error) else {
                unreachable!("canonical unavailable read");
            };
            assert_eq!(
                cause.reason(),
                LengthValueError::RequiresObservation {
                    declaration: id(1),
                    observation: id(30)
                }
            );
            assert!(std::ptr::eq(
                cause.input().canonical_declaration(),
                inputs.length(id(1)).expect("actual record")
            ));
            assert!(matches!(
                error.node().form(),
                Form::Name {
                    name: "private_width",
                    ..
                }
            ));
        }
        let expression = normalized("use_fixed");
        let proof = expression
            .check_kinds(&namespace)
            .expect("actual Boolean source proof");
        assert_eq!(
            execute(&proof, &inputs, context())
                .expect("present true or false")
                .value()
                .boolean(),
            Some(flag)
        );
        assert_eq!(inputs, before);
    }
}

#[test]
fn only_taken_optional_contexts_are_read_and_condition_call_sources_reach_the_result() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let namespace = FormulaNamespace::new([]).expect("namespace");
    for source in [
        "if(1 mm<2 mm,1 mm,eps_phys)",
        "if(1 mm<2 mm,1 mm,if(is_base_size,2 mm,3 mm))",
    ] {
        let expression = normalized(source);
        let proof = expression.check_kinds(&namespace).expect("complete proof");
        let result = execute(&proof, &inputs, context()).expect("absent optional source untaken");
        assert_eq!(result.reads().count(), 0);
        assert_eq!(
            result.value().numeric().expect("Length").rational(),
            R::from_signed_integer(1000)
        );
    }
    let expression = normalized("if(1 mm>2 mm,1 mm,eps_phys)");
    let proof = expression.check_kinds(&namespace).expect("proof");
    let error = execute(&proof, &inputs, context()).expect_err("actual selected absent provider");
    assert!(matches!(
        failure(&error),
        Failure::ToleranceRead(FormulaToleranceReadError::MissingContext {
            name: FormulaToleranceName::Physical,
            context_label: None
        })
    ));
    assert!(matches!(
        error.node().form(),
        Form::Name {
            name: "eps_phys",
            ..
        }
    ));
    let expression = normalized("is_base_size");
    let proof = expression.check_kinds(&namespace).expect("proof");
    assert!(matches!(
        failure(&execute(&proof, &inputs, context()).expect_err("no default false")),
        Failure::SizeRead(_)
    ));
    let expression = normalized("if(sin(0 deg)==0 pct,1 mm,2 mm)");
    let proof = expression
        .check_kinds(&namespace)
        .expect("actual condition proof");
    let result = execute(&proof, &inputs, context()).expect("actual conditional");
    assert_eq!(
        result.value().sources().iter().collect::<Vec<_>>(),
        [B::Sin]
    );
    assert_eq!(result.reads().count(), 0);
}

#[test]
fn actual_child_order_and_derived_requirements_retain_the_first_original_failure_node() {
    let tokens = ["private_width", "derived", "use_fixed"]
        .map(|name| MachineToken::new(name).expect("name"));
    let inputs = inputs(true);
    let before = inputs.clone();
    let namespace = namespace(&tokens, &inputs);
    for source in [
        "private_width+1 mm/(0 pct)",
        "1 mm/(0 pct)+private_width",
        "min(private_width,1 mm/(0 pct))",
        "min(1 mm/(0 pct),private_width)",
    ] {
        let expression = normalized(source);
        let proof = expression
            .check_kinds(&namespace)
            .expect("all kinds checked before values");
        let error = execute(&proof, &inputs, context()).expect_err("actual first selected failure");
        let span = error.node().span();
        if source.starts_with("private_width") || source.starts_with("min(private_width") {
            assert_eq!(source.get(span.start()..span.end()), Some("private_width"));
            assert!(matches!(failure(&error), Failure::LengthInput(_)));
        } else {
            assert_eq!(source.get(span.start()..span.end()), Some("1 mm/(0 pct)"));
            assert!(matches!(
                failure(&error),
                Failure::Numeric(FormulaNumericError::Rational(
                    FormulaRationalError::ZeroDivisor
                ))
            ));
            assert_eq!(
                error
                    .source()
                    .and_then(|cause| cause.downcast_ref::<FormulaNumericError>()),
                Some(&FormulaNumericError::Rational(
                    FormulaRationalError::ZeroDivisor
                ))
            );
        }
        assert!(std::ptr::eq(error.node().expression(), &expression));
        assert!(!format!("{error:?}").contains("private_width"));
    }
    let expression = normalized("derived");
    let proof = expression
        .check_kinds(&namespace)
        .expect("derived metadata is valid");
    let error =
        execute(&proof, &inputs, context()).expect_err("sole dependency still needs execution");
    let Failure::LengthInput(cause) = failure(&error) else {
        unreachable!("original canonical cause");
    };
    assert_eq!(
        cause.reason(),
        LengthValueError::RequiresEvaluation {
            declaration: id(2),
            formula: id(31)
        }
    );
    assert_eq!(inputs, before);
}

#[test]
fn every_evaluated_call_operand_and_actual_tolerance_provider_survives_selection_and_comparison() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let namespace = FormulaNamespace::new([]).expect("namespace");
    for source in [
        "min(0 mm,round_to(1 mm,1 um))",
        "max(2 mm,round_to(1 mm,1 um))",
        "clamp(0 mm,0 mm,round_to(1 mm,1 um))",
    ] {
        let expression = normalized(source);
        let proof = expression.check_kinds(&namespace).expect("proof");
        let result = execute(&proof, &inputs, context()).expect("all actual operands run");
        assert_eq!(
            result.value().sources().iter().collect::<Vec<_>>(),
            [B::RoundTo]
        );
    }
    let physical = Tolerance::new(
        ToleranceClass::PhysicalAcceptance,
        Length::from_micrometres(10).expect("Length"),
        "explicit fixture factory limit",
    )
    .expect("quantitative provider");
    let tolerances = FormulaToleranceContext::new(FormulaToleranceContextDefinition {
        label: Some("actual_fixture_factory"),
        format: None,
        importer: None,
        physical: Some(FormulaToleranceProvider {
            source: id(90),
            tolerance: &physical,
        }),
    })
    .expect("actual class validated");
    let expression = normalized("within(1 mm,1.01 mm,eps_phys)");
    let proof = expression
        .check_kinds(&namespace)
        .expect("actual direct class proof");
    let actual_context = Context::new(tolerances, FormulaSizeContext::empty());
    assert!(!format!("{actual_context:?}").contains("actual_fixture_factory"));
    let result = execute(&proof, &inputs, actual_context).expect("inclusive real quantum");
    assert_eq!(result.value().boolean(), Some(true));
    assert_eq!(names(&result), ["eps_phys"]);
    let Read::Tolerance(read) = result
        .reads()
        .next()
        .expect("actual once-read class")
        .source()
    else {
        unreachable!("class");
    };
    assert!(
        matches!(read.source(), FormulaToleranceValueSource::Provider(provider)
        if provider.source == id(90) && std::ptr::eq(provider.tolerance, &physical))
    );
    assert_eq!(read.context_label(), Some("actual_fixture_factory"));

    let expression = normalized("within(sqrt((2 mm)^2),2 mm,eps_num)");
    let proof = expression
        .check_kinds(&namespace)
        .expect("static direct T1 is legal");
    let error =
        execute(&proof, &inputs, context()).expect_err("actual approximation contribution at T1");
    let Failure::Comparison(cause) = failure(&error) else {
        unreachable!("original class comparison");
    };
    assert!(matches!(
        cause.refusal(),
        FormulaToleranceComparisonRefusal::Approximation(_)
    ));
    assert!(cause.left().sources().contains(B::Sqrt));
    assert_eq!(cause.tolerance().name(), FormulaToleranceName::Numerical);
    assert_eq!(
        cause.left().numeric().expect("actual value").rational(),
        R::from_signed_integer(2000)
    );
    assert_eq!(error.node().span(), expression.root().span());
}

#[test]
fn executed_canonical_claims_keep_zero_assumption_preference_and_original_record_borrows() {
    let token = MachineToken::new("value").expect("name");
    let inputs = inputs(false);
    let before = inputs.clone();
    for (identity, expected) in [(4, 0), (5, 1000), (6, -1000)] {
        let record = inputs.length(id(identity)).expect("actual declaration");
        let metadata = D::length_input(&token, FormulaInputOrigin::Parameter, id(101), record);
        let namespace =
            FormulaNamespace::new(
                [FormulaInitialDeclaration::try_from(metadata).expect("initial")],
            )
            .expect("namespace");
        let expression = normalized("value+value");
        let proof = expression.check_kinds(&namespace).expect("actual proof");
        let result = execute(&proof, &inputs, context()).expect("actual authored claims");
        assert_eq!(
            result.value().numeric().expect("Length").rational(),
            R::from_signed_integer(expected * 2)
        );
        assert_eq!(names(&result), ["value", "value"]);
        for read in result.reads() {
            let Read::Length(source) = read.source() else {
                unreachable!("actual dedicated source");
            };
            assert!(std::ptr::eq(source.input().canonical_declaration(), record));
            assert!(std::ptr::eq(source.input().state(), record.state()));
        }
        assert_eq!(result.value().sources(), FormulaApproximationSources::EMPTY);
    }
    assert_eq!(inputs, before);
}

#[test]
fn full_task_and_source_reservations_execute_without_recursive_frames_or_deduplicating_reads() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let namespace = FormulaNamespace::new([]).expect("namespace");
    let source = format!("min({})", vec!["eps_num"; 255].join(","));
    let expression = normalized(&source);
    let proof = expression.check_kinds(&namespace).expect("256-node proof");
    let plan = proof
        .admit_execution(FormulaExecutionLimits::new(256, 767))
        .expect("exact logical bound")
        .prepare_scalar(&inputs)
        .expect("actual complete source plan");
    let result = plan
        .evaluate(context())
        .expect("maximum root fanout fits N tasks");
    assert_eq!(
        result.value().numeric().expect("Length").rational(),
        R::from_signed_integer(1)
    );
    assert_eq!(result.reads().count(), 255);
    for (read, dependency) in result.reads().zip(proof.dependencies()) {
        assert_eq!(read.node().span(), dependency.span());
        assert!(
            matches!(read.source(), Read::Tolerance(value) if value.name() == FormulaToleranceName::Numerical)
        );
    }
    let source = format!("-({})", vec!["1 um"; 128].join("+"));
    let expression = normalized(&source);
    let proof = expression
        .check_kinds(&namespace)
        .expect("256-node left-deep proof");
    let result = proof
        .admit_execution(FormulaExecutionLimits::new(256, 512))
        .expect("exact N tasks+N values")
        .prepare_scalar(&inputs)
        .expect("scalar scope")
        .evaluate(context())
        .expect("nonrecursive complete traversal");
    assert_eq!(
        result.value().numeric().expect("Length").rational(),
        R::from_signed_integer(-128)
    );
    assert_eq!(result.reads().count(), 0);
}
