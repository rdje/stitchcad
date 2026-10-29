//! Actual one-shot scalar recipes: earned stored prefix, named assertions and no partial result.
#![allow(clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    recipe::{
        FormulaApproximationSources as Sources, FormulaBindingValueRule as Storage, FormulaBuiltin,
        FormulaDeclaration as D, FormulaExecutedScalarRecipe, FormulaInitialDeclaration,
        FormulaInputOrigin, FormulaKind, FormulaNamespace, FormulaRational as R, FormulaRecipe,
        FormulaRecipeDependencyRole as Role, FormulaRecipeExecutionLimits as Limits,
        FormulaScalarExecutionContext as Context, FormulaScalarExecutionErrorCause as ScalarCause,
        FormulaScalarExecutionFailure as ScalarFailure, FormulaScalarRecipeError,
        FormulaScalarRecipeErrorCause as Cause, FormulaScalarRecipeFailure as Failure,
        FormulaScalarRecipeOutcome as Outcome, FormulaScalarRecipeReadKind as ReadKind,
        FormulaSizeContext, FormulaStatementExpression as Part, FormulaToleranceContext,
    },
    value::{
        CanonicalInputs, LengthDeclaration, LengthDeclarationDefinition, LengthState,
        LengthValueError,
    },
};
const LIMITS: Limits = Limits::new(64, 4096, 16384, 64);
fn context() -> Context<'static> {
    Context::new(
        FormulaToleranceContext::empty(),
        FormulaSizeContext::empty(),
    )
}
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn failure<'a, 'e>(error: &'e FormulaScalarRecipeError<'a>) -> &'e Failure<'a> {
    let Cause::Evaluation(failure) = error.cause() else {
        unreachable!("actual semantic fixture");
    };
    failure
}
fn names<'a>(result: &FormulaExecutedScalarRecipe<'a>) -> Vec<&'a str> {
    result
        .reads()
        .map(|read| read.dependency().declaration().name())
        .collect()
}

#[test]
fn actual_fraction_binding_zero_false_and_interleaved_assertions_supply_only_stored_values() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let text = "let first:length=1 um/3\nlet half:ratio=0.000003/2\nlet false_flag:boolean=half<0.0\nassert prior:eps_num=first==0 um\nlet second:length=if(false_flag,eps_phys,first+1 um)\nassert final:eps_num=second==1 um";
    let recipe = FormulaRecipe::parse(text)
        .expect("syntax")
        .normalize_literals()
        .expect("inputs");
    let proof = recipe
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("whole proof");
    let result = proof
        .admit_execution(LIMITS)
        .expect("explicit")
        .prepare_scalar(&inputs)
        .expect("sources")
        .evaluate(context())
        .expect("real ordered evaluation");
    assert_eq!(result.statements().len(), 6);
    assert!(std::ptr::eq(result.plan().admission().proof(), &proof));
    assert!(std::ptr::eq(result.plan().inputs(), &inputs));
    let Outcome::Binding {
        expression,
        binding,
    } = result.statements().first().expect("actual first").outcome()
    else {
        unreachable!("let");
    };
    assert_eq!(
        expression.numeric().expect("Length").rational(),
        R::new(1, 3, false).expect("exact")
    );
    assert_eq!(binding.value().numeric_integer(), Some(0));
    let Outcome::Binding {
        expression,
        binding,
    } = result.statements().get(1).expect("half").outcome()
    else {
        unreachable!("let");
    };
    assert_eq!(
        expression.numeric().expect("Ratio").rational(),
        R::new(3, 2, false).expect("exact")
    );
    assert_eq!(binding.value().numeric_integer(), Some(2));
    let Outcome::Binding { binding, .. } = result.statements().get(2).expect("flag").outcome()
    else {
        unreachable!("let");
    };
    assert_eq!(binding.value().boolean(), Some(false));
    assert_eq!(binding.value().numeric_integer(), None);
    assert_eq!(
        names(&result),
        [
            "half",
            "first",
            "eps_num",
            "false_flag",
            "first",
            "second",
            "eps_num"
        ]
    );
    for (position, completed) in result.statements().iter().enumerate() {
        assert!(std::ptr::eq(
            completed.statement(),
            proof.statements().get(position).expect("actual proof")
        ));
        assert_eq!(completed.statement().statement_index(), position + 1);
    }
    for read in result.reads() {
        if read.dependency().role() == Role::AssertionTolerance {
            assert!(read.node().is_none());
        } else {
            assert_eq!(
                read.node().expect("real name node").span(),
                read.dependency().span()
            );
        }
        if let ReadKind::PriorLet { statement, binding } = read.source() {
            assert!(statement.statement_index() < read.dependency().statement_index());
            assert!(std::ptr::eq(
                statement,
                proof
                    .statements()
                    .get(statement.statement_index() - 1)
                    .expect("real supplier")
            ));
            assert_eq!(binding.read().expect("actual stored read"), read.value());
        }
    }
    assert!(!format!("{result:?}").contains("false_flag"));
}

#[test]
fn prior_boolean_condition_and_cancelled_stored_results_keep_actual_approximation_sources() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let recipe = FormulaRecipe::parse("let a:ratio=sin(90 deg)\nlet flag:boolean=a>0.0\nlet output:length=if(flag,1 um,eps_phys)\nlet zero:length=output-output\nassert check:eps_geo=zero==0 um")
        .expect("syntax").normalize_literals().expect("inputs");
    let proof = recipe
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("whole proof");
    let result = proof
        .admit_execution(LIMITS)
        .expect("explicit")
        .prepare_scalar(&inputs)
        .expect("sources")
        .evaluate(context())
        .expect("real ordered evaluation");
    assert_eq!(
        names(&result),
        ["a", "flag", "output", "output", "zero", "eps_geo"]
    );
    for completed in result.statements() {
        match completed.outcome() {
            Outcome::Binding {
                expression,
                binding,
            } => {
                assert_eq!(expression.sources(), Sources::for_call(FormulaBuiltin::Sin));
                assert_eq!(binding.sources(), expression.sources());
            }
            Outcome::Assertion(comparison) => {
                assert!(comparison.holds());
                assert_eq!(
                    comparison.left().numeric().expect("zero").rational(),
                    R::from_signed_integer(0)
                );
                assert_eq!(
                    comparison.left().sources(),
                    Sources::for_call(FormulaBuiltin::Sin)
                );
            }
        }
    }
}

#[test]
fn actual_unknown_and_derived_requirements_keep_original_record_and_first_failing_statement() {
    for (state, wanted) in [
        (
            LengthState::Unknown { observation: id(3) },
            LengthValueError::RequiresObservation {
                declaration: id(1),
                observation: id(3),
            },
        ),
        (
            LengthState::Derived { formula: id(4) },
            LengthValueError::RequiresEvaluation {
                declaration: id(1),
                formula: id(4),
            },
        ),
    ] {
        let inputs = CanonicalInputs::new(
            vec![LengthDeclaration::new(LengthDeclarationDefinition {
                id: id(1),
                source: id(2),
                state,
            })
            .expect("actual record")],
            vec![],
        )
        .expect("inventory");
        let before = inputs.clone();
        let name = MachineToken::new("private_width").expect("name");
        let record = inputs.length(id(1)).expect("actual");
        let metadata = D::length_input(&name, FormulaInputOrigin::Measurement, id(5), record);
        let namespace =
            FormulaNamespace::new(
                [FormulaInitialDeclaration::try_from(metadata).expect("initial")],
            )
            .expect("namespace");
        let recipe = FormulaRecipe::parse("let prior:length=1 um\nlet selected:length=if(1==1,prior,private_width)\nlet fails:length=private_width\nlet later:length=1 um/0")
            .expect("syntax").normalize_literals().expect("inputs");
        let proof = recipe.check_kinds(namespace).expect("whole proof");
        let error = proof
            .admit_execution(LIMITS)
            .expect("explicit")
            .prepare_scalar(&inputs)
            .expect("sources")
            .evaluate(context())
            .expect_err("no accepted prior prefix or invented value");
        assert_eq!(error.statement().expect("actual").statement_index(), 3);
        assert!(std::ptr::eq(error.proof(), &proof));
        let Failure::Expression {
            part: Part::Binding,
            error: inner,
        } = failure(&error)
        else {
            unreachable!("real binding RHS failure");
        };
        let ScalarCause::Evaluation(cause) = inner.cause() else {
            unreachable!("actual read failure");
        };
        let ScalarFailure::LengthInput(cause) = cause.as_ref() else {
            unreachable!("actual dedicated input");
        };
        assert_eq!(cause.reason(), wanted);
        assert!(std::ptr::eq(cause.input().canonical_declaration(), record));
        assert_eq!(inputs, before);
        assert!(!format!("{error:?}").contains("private_width"));
    }
}

#[test]
fn assertion_left_then_right_then_header_keep_original_failure_context_and_false_values() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    for (text, expected) in [
        (
            "let prior:length=1 mm\nassert named:eps_phys=prior/0==2 mm",
            "left",
        ),
        (
            "let prior:length=1 mm\nassert named:eps_phys=prior==2 mm/0",
            "right",
        ),
        (
            "let prior:length=1 mm\nassert named:eps_phys=prior==2 mm",
            "header",
        ),
        (
            "let prior:length=1 um\nassert named:eps_num=prior==3 um",
            "false",
        ),
        (
            "let prior:ratio=sin(90 deg)\nassert named:eps_num=prior==1.0",
            "class",
        ),
    ] {
        let recipe = FormulaRecipe::parse(text)
            .expect("syntax")
            .normalize_literals()
            .expect("inputs");
        let proof = recipe
            .check_kinds(FormulaNamespace::new([]).expect("namespace"))
            .expect("whole proof");
        let error = proof
            .admit_execution(LIMITS)
            .expect("explicit")
            .prepare_scalar(&inputs)
            .expect("sources")
            .evaluate(context())
            .expect_err("no accepted partial result");
        assert_eq!(error.statement().expect("actual").statement_index(), 2);
        assert_eq!(error.statement().expect("name").statement().name(), "named");
        match (expected, failure(&error)) {
            (
                "left",
                Failure::Expression {
                    part: Part::AssertionLeft,
                    ..
                },
            )
            | (
                "right",
                Failure::Expression {
                    part: Part::AssertionRight,
                    ..
                },
            )
            | ("header", Failure::ToleranceRead(_))
            | ("class", Failure::Comparison(_)) => {}
            ("false", Failure::Assertion(comparison)) => {
                assert!(!comparison.holds());
                assert_eq!(
                    comparison.left().numeric().expect("actual").rational(),
                    R::from_signed_integer(1)
                );
                assert_eq!(
                    comparison.right().numeric().expect("actual").rational(),
                    R::from_signed_integer(3)
                );
                assert_eq!(
                    comparison.tolerance().value().rational(),
                    R::from_signed_integer(1)
                );
            }
            _ => unreachable!("actual named assertion refusal has its original phase"),
        }
    }
}

#[test]
fn completed_wide_count_refuses_only_at_actual_binding_with_full_storage_witness() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let text =
        "let prior:length=1 um\nlet huge:count=9223372036854775807*2\nlet later:length=eps_phys";
    let recipe = FormulaRecipe::parse(text)
        .expect("syntax")
        .normalize_literals()
        .expect("inputs");
    let proof = recipe
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("whole proof");
    let error = proof
        .admit_execution(LIMITS)
        .expect("explicit")
        .prepare_scalar(&inputs)
        .expect("sources")
        .evaluate(context())
        .expect_err("unbound wide Count is valid;actual storage refuses");
    assert_eq!(error.statement().expect("actual").statement_index(), 2);
    assert_eq!(
        error.statement().expect("actual").statement().name(),
        "huge"
    );
    let Failure::Binding {
        kind: FormulaKind::Count,
        rule,
    } = failure(&error)
    else {
        unreachable!("actual storage");
    };
    assert_eq!(
        *rule,
        Storage::Storage {
            minimum: 0,
            maximum: i64::MAX,
            measured: R::from_signed_integer(i128::from(i64::MAX) * 2)
        }
    );
}

#[test]
fn empty_and_maximum_assertion_recipes_return_only_complete_actual_outcomes_and_header_reads() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    for (text, count, nodes, workspace) in [
        (String::new(), 0, 0, 0),
        (
            (0..4096)
                .map(|n| format!("assert check_{n}:eps_num=1==1"))
                .collect::<Vec<_>>()
                .join("\n"),
            4096,
            8192,
            20480,
        ),
    ] {
        let recipe = FormulaRecipe::parse(&text)
            .expect("syntax")
            .normalize_literals()
            .expect("inputs");
        let proof = recipe
            .check_kinds(FormulaNamespace::new([]).expect("namespace"))
            .expect("whole proof");
        let result = proof
            .admit_execution(Limits::new(count, nodes, workspace, count))
            .expect("exact")
            .prepare_scalar(&inputs)
            .expect("sources")
            .evaluate(context())
            .expect("complete actual evaluation");
        assert_eq!(result.statements().len(), count);
        assert_eq!(result.reads().count(), count);
        for (position, read) in result.reads().enumerate() {
            assert_eq!(read.dependency().statement_index(), position + 1);
            assert_eq!(read.dependency().role(), Role::AssertionTolerance);
            assert!(read.node().is_none());
        }
    }
}

#[test]
fn actual_within_and_assertion_reads_keep_original_physical_provider_and_lazy_other_context() {
    use sc_core::recipe::{
        FormulaScalarExecutionReadKind, FormulaToleranceContextDefinition,
        FormulaToleranceProvider, FormulaToleranceValueSource,
    };
    use sc_units::{Length, Tolerance, ToleranceClass};
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let physical = Tolerance::new(
        ToleranceClass::PhysicalAcceptance,
        Length::from_micrometres(10).expect("Length"),
        "actual fixture factory limit",
    )
    .expect("quantitative");
    let context = Context::new(
        FormulaToleranceContext::new(FormulaToleranceContextDefinition {
            label: Some("actual_fixture_factory"),
            format: None,
            importer: None,
            physical: Some(FormulaToleranceProvider {
                source: id(90),
                tolerance: &physical,
            }),
        })
        .expect("class checked"),
        FormulaSizeContext::empty(),
    );
    let recipe = FormulaRecipe::parse("let a:length=1 mm\nlet inside:boolean=within(a,1.01 mm,eps_phys)\nlet selected:length=if(inside,a,eps_fmt)\nassert check:eps_phys=selected==1.01 mm")
        .expect("syntax").normalize_literals().expect("inputs");
    let proof = recipe
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("whole proof");
    let result = proof
        .admit_execution(LIMITS)
        .expect("explicit")
        .prepare_scalar(&inputs)
        .expect("sources")
        .evaluate(context)
        .expect("actual inclusive physical tolerance");
    assert_eq!(
        names(&result),
        ["a", "eps_phys", "inside", "a", "selected", "eps_phys"]
    );
    let mut provider_occurrences = 0;
    for read in result.reads() {
        if let ReadKind::Scalar(FormulaScalarExecutionReadKind::Tolerance(value)) = read.source() {
            provider_occurrences += 1;
            assert!(
                matches!(value.source(), FormulaToleranceValueSource::Provider(provider)
                if provider.source == id(90) && std::ptr::eq(provider.tolerance, &physical))
            );
            assert_eq!(value.context_label(), Some("actual_fixture_factory"));
            assert_eq!(value.value().rational(), R::from_signed_integer(10));
        }
    }
    assert_eq!(
        provider_occurrences, 2,
        "one real within name and one actual assertion header"
    );
}
